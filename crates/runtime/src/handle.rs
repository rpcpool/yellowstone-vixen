//! A handle for talking to a [`Runtime`](crate::Runtime) after it has started.

use std::sync::{Arc, Mutex, PoisonError};

use shipstern_core::Filters;
use tokio::sync::watch;

/// Why a filter update never reached the source.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FilterUpdateError {
    /// The runtime has stopped, or was dropped without being run, so nothing
    /// is left to apply the set.
    #[error("the runtime is no longer running")]
    Closed,
    /// The set names a parser no pipeline on this runtime is registered for.
    /// The server would stream that data and the runtime would discard all of
    /// it, so the update is refused before anything is sent.
    ///
    /// Only the name is checked, not the kind of prefilter carried under it.
    /// A prefilter whose kind does not match the pipeline registered for that
    /// name still reaches the wire, and the runtime discards those updates the
    /// same way, silently.
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

    fn snapshot(&self) -> Filters { self.filter_updates_tx.borrow().clone() }

    /// Refuse a set naming a parser that has no registered pipeline.
    fn validate(&self, filters: &Filters) -> Result<(), FilterUpdateError> {
        let Some(unknown) = filters
            .parser_ids()
            .find(|id| self.initial.get(id).is_none())
        else {
            return Ok(());
        };

        Err(FilterUpdateError::UnknownParser(unknown.to_owned()))
    }
}

/// A handle onto a [`Runtime`](crate::Runtime) for changing its subscription
/// while it runs.
///
/// [`Runtime::run`](crate::Runtime::run) and its variants take the runtime by
/// value, so take the handle with [`Runtime::handle`](crate::Runtime::handle)
/// first. It exists only for sources implementing
/// [`FilterUpdateSource`](crate::sources::FilterUpdateSource). Handles are
/// cheap to clone, share one view of the filters, and work from async and
/// plain threads alike since nothing here awaits.
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
///   accepted. Take them from [`Parser::id`](shipstern_core::Parser::id),
///   except for instruction parsers: the runtime bundles them all behind one
///   [`InstructionPipeline`](crate::instruction::InstructionPipeline), so the
///   set holds a single entry under
///   [`InstructionPipeline::ID`](crate::instruction::InstructionPipeline::ID)
///   whose prefilter is the union of theirs. Naming an individual instruction
///   parser is refused as unknown. Acceptance is judged against the registered
///   set, not the current one, so a key dropped by [`Filters::remove`] stays
///   acceptable and can be merged back even though
///   [`filters`](Self::filters) no longer lists it;
///   [`reset_filters`](Self::reset_filters) restores the full registered set.
/// - `Ok(())` means the set was handed to the source, not that the server
///   applied it. Handlers keep seeing updates matching the old set until the
///   already-queued backlog drains, roughly however far behind the pipeline
///   was at the time. The source awaits that same buffer inside the loop that
///   watches for updates, so while it is full the request has not reached the
///   server either.
/// - Only the newest set matters. Two updates in quick succession may reach
///   the source as the second alone, and a set rejected while the source is
///   between connections is retried once the stream recovers. With
///   auto-reconnect off there is nothing to recover into, so such a set is
///   dropped with a warning while [`filters`](Self::filters) still reports it.
/// - A set the server refuses comes back on the stream, not on the sink. A
///   terminal status code ends the run, and under `run` and `run_async` that
///   exits the process. A recoverable one, `ResourceExhausted` among them,
///   does not: the client resubscribes with the same refused set and repeats,
///   so an unacceptable set can leave the runtime reconnecting rather than
///   stopping.
#[derive(Debug, Clone)]
pub struct RuntimeHandle {
    state: Arc<FilterState>,
}

impl RuntimeHandle {
    pub(crate) fn new(state: Arc<FilterState>) -> Self { Self { state } }

    /// The last filter set published for the source, seeded from the registered
    /// pipelines.
    ///
    /// This leads what the server is serving, and by an unbounded amount when
    /// the sink is between connections: the source may not have read the slot
    /// yet, a rejected set can sit in the retry queue across a reconnect, and
    /// a set still unsent when the connection ends is logged and lost. Judge a
    /// live subscription by what the handlers receive, not by this.
    ///
    /// ```rust, ignore
    /// let owners = handle
    ///     .filters()
    ///     .get(&TokenProgramAccParser.id())
    ///     .and_then(|prefilter| prefilter.account.as_ref())
    ///     .map(|account| account.owners.clone());
    /// ```
    ///
    #[must_use]
    pub fn filters(&self) -> Filters { self.state.snapshot() }

    /// Edit the live filter set in place and send the result.
    ///
    /// `edit` sees a copy of the current set, and the result replaces the
    /// subscription. Concurrent calls from any handle are applied one after
    /// another, so no edit is lost. `edit` runs while that lock is held, so it
    /// must not call back into `update_filters` on this handle or any clone of
    /// it; doing so deadlocks the calling thread.
    ///
    /// ```rust, ignore
    /// handle.update_filters(|filters| {
    ///     filters.merge(TokenProgramAccParser.id(), extra_owner);
    ///     filters.remove(InstructionPipeline::ID);
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
        // The guarded section only copies and republishes a set, so a panic
        // partway through leaves no torn state behind and the next caller can
        // take the lock as if nothing happened.
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
    /// For a set built from scratch. Reading [`Self::filters`] and sending the
    /// result back reads outside the lock, so two callers doing that lose one
    /// of the two edits; use [`Self::update_filters`] for anything that starts
    /// from the current set.
    ///
    /// ```rust, ignore
    /// handle.send_filter_update(Filters::new(rebuilt))?;
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
