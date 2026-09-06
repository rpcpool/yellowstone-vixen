//! A handle for talking to a [`Runtime`](crate::Runtime) after it has started.

use std::sync::{Arc, Mutex, PoisonError};

use shipstern_core::Filters;
use tokio::sync::watch;

/// Why a filter update never reached the source.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FilterUpdateError {
    /// The source behind the runtime cannot change its subscription
    /// mid-stream, so no update could ever take effect.
    #[error("the source does not support filter updates")]
    Unsupported,
    /// The runtime has stopped, or was dropped without being run, so nothing
    /// is left to apply the set.
    #[error("the runtime is no longer running")]
    Closed,
    /// The set names a parser no pipeline on this runtime is registered for.
    /// The server would stream that data and the runtime would discard all of
    /// it, so the update is refused before anything is sent.
    #[error("no pipeline registered for parser `{0}`")]
    UnknownParser(String),
}

/// Filter state shared by every handle onto one runtime.
///
/// The `watch` slot is both the transport to the source and the record of the
/// last set handed off, so the two can never disagree.
#[derive(Debug)]
pub(crate) struct FilterState {
    /// The set the pipelines were built with. Also the source of truth for
    /// which parser IDs an update may name.
    initial: Filters,
    /// Latest set for the source. A newer set replaces one the source has not
    /// read yet, which is what the servers do with successive requests anyway.
    filter_updates_tx: watch::Sender<Filters>,
    /// Serialises read-modify-write sequences across handles, so two
    /// concurrent edits cannot interleave and lose one another. Never held
    /// across an await.
    update_lock: Mutex<()>,
}

impl FilterState {
    pub(crate) fn new(filter_updates_tx: watch::Sender<Filters>) -> Self {
        let initial = filter_updates_tx.borrow().clone();

        Self {
            initial,
            filter_updates_tx,
            update_lock: Mutex::new(()),
        }
    }

    /// The set the pipelines were built with.
    pub(crate) fn initial(&self) -> &Filters { &self.initial }

    fn snapshot(&self) -> Filters { self.filter_updates_tx.borrow().clone() }

    /// Refuse a set naming a parser that has no registered pipeline.
    fn validate(&self, filters: &Filters) -> Result<(), FilterUpdateError> {
        let unknown = filters
            .parser_ids()
            .find(|id| !self.initial.parsers_filters.contains_key(*id));

        match unknown {
            Some(id) => Err(FilterUpdateError::UnknownParser(id.to_owned())),
            None => Ok(()),
        }
    }
}

/// A handle onto a [`Runtime`](crate::Runtime) for changing its subscription
/// while it runs.
///
/// [`Runtime::run`](crate::Runtime::run) and its variants take the runtime by
/// value, so take the handle with [`Runtime::handle`](crate::Runtime::handle)
/// first. Handles are cheap to clone, share one view of the filters, and work
/// from async and plain threads alike since nothing here awaits.
///
/// ```rust, ignore
/// let runtime = Runtime::<YellowstoneGrpcSource>::builder()
///     .account(Pipeline::new(TokenProgramAccParser, [Handler]))
///     .try_build(config)?;
///
/// let handle = runtime.handle();
/// tokio::spawn(runtime.run_async());
///
/// // Widen the account subscription with one more owner.
/// let extra = Prefilter::builder().account_owners([new_mint]).build()?;
/// handle.update_filters(|filters| filters.merge(TokenProgramAccParser.id(), extra))?;
///
/// // Back to what the pipelines were built with.
/// handle.reset_filters()?;
/// ```
///
/// # Semantics
///
/// - Every update sends the complete set and replaces the live subscription,
///   which is what gRPC servers do with a mid-stream request.
/// - Keys are parser IDs, and only IDs with a registered pipeline are
///   accepted. Take them from [`Parser::id`](shipstern_core::Parser::id).
/// - `Ok(())` means the set was handed to the source, not that the server
///   applied it. Handlers keep seeing updates matching the old set until the
///   already-queued backlog drains, roughly however far behind the pipeline
///   was at the time.
/// - Only the newest set matters. Two updates in quick succession may reach
///   the source as the second alone, and a set rejected while the source is
///   between connections is retried once the stream recovers.
/// - A set the server refuses, by exceeding its filter limits for example,
///   ends the run. Under `run` and `run_async` that exits the process.
#[derive(Debug, Clone)]
pub struct RuntimeHandle {
    /// Whether the source behind the runtime applies filter updates, so a
    /// send fails at the call site instead of vanishing into a slot nobody
    /// reads.
    supported: bool,
    state: Arc<FilterState>,
}

impl RuntimeHandle {
    pub(crate) fn new(supported: bool, state: Arc<FilterState>) -> Self {
        Self { supported, state }
    }

    /// The last filter set handed to the source, seeded from the registered
    /// pipelines.
    ///
    /// ```rust, ignore
    /// let owners = handle
    ///     .filters()
    ///     .get(&TokenProgramAccParser.id())
    ///     .and_then(|prefilter| prefilter.account.as_ref())
    ///     .map(|account| account.owners.clone());
    /// ```
    #[must_use]
    pub fn filters(&self) -> Filters { self.state.snapshot() }

    /// Edit the live filter set in place and send the result.
    ///
    /// `edit` sees a copy of the current set, and the result replaces the
    /// subscription. Concurrent calls from any handle are applied one after
    /// another, so no edit is lost.
    ///
    /// ```rust, ignore
    /// handle.update_filters(|filters| {
    ///     filters.merge(TokenProgramAccParser.id(), extra_owner);
    ///     filters.remove(&TokenProgramIxParser.id());
    /// })?;
    /// ```
    ///
    /// # Errors
    ///
    /// [`FilterUpdateError::UnknownParser`] leaves the set untouched, and
    /// nothing is sent. See [`FilterUpdateError`] for the rest.
    ///
    pub fn update_filters<F>(&self, edit: F) -> Result<(), FilterUpdateError>
    where F: FnOnce(&mut Filters) {
        if !self.supported {
            return Err(FilterUpdateError::Unsupported);
        }

        let _serialised = self
            .state
            .update_lock
            .lock()
            .unwrap_or_else(PoisonError::into_inner);

        let mut next = self.state.snapshot();
        edit(&mut next);
        self.state.validate(&next)?;

        // A failed send leaves the slot untouched, so `filters()` never
        // reports a set that went nowhere.
        self.state
            .filter_updates_tx
            .send(next)
            .map_err(|_| FilterUpdateError::Closed)
    }

    /// Replace the whole subscription with `filters`.
    ///
    /// ```rust, ignore
    /// let mut filters = handle.filters();
    /// filters.insert(TokenProgramAccParser.id(), narrower);
    /// handle.send_filter_update(filters)?;
    /// ```
    ///
    /// # Errors
    ///
    /// See [`FilterUpdateError`].
    ///
    pub fn send_filter_update(&self, filters: Filters) -> Result<(), FilterUpdateError> {
        self.update_filters(|current| *current = filters)
    }

    /// Restore the set the pipelines were built with.
    ///
    /// ```rust, ignore
    /// handle.reset_filters()?;
    /// ```
    ///
    /// # Errors
    ///
    /// See [`FilterUpdateError`].
    ///
    pub fn reset_filters(&self) -> Result<(), FilterUpdateError> {
        self.send_filter_update(self.state.initial.clone())
    }
}
