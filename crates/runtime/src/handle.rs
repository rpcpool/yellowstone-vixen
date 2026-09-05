//! A handle for talking to a [`Runtime`](crate::Runtime) after it has started.

use shipstern_core::Filters;
use tokio::sync::mpsc;

/// Why a filter update never reached the source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum FilterUpdateError {
    /// The source behind the runtime cannot change its subscription
    /// mid-stream, so no update could ever take effect.
    #[error("the source does not support filter updates")]
    Unsupported,
    /// The runtime has stopped, or was dropped without being run, so nothing
    /// is left to apply the set.
    #[error("the runtime is no longer running")]
    Closed,
}

/// A handle onto a [`Runtime`](crate::Runtime) that outlives the runtime
/// value itself.
///
/// [`Runtime::run`](crate::Runtime::run) and its variants take the runtime by
/// value, so take a handle with [`Runtime::handle`](crate::Runtime::handle)
/// first and use that from wherever the update originates. Handles are cheap
/// to clone and every clone addresses the same runtime.
///
/// # Example
///
/// ```rust, ignore
/// let runtime = Runtime::<YellowstoneGrpcSource>::builder()
///     .instruction(Pipeline::new(TokenProgramIxParser, [Handler]))
///     .try_build(config)?;
///
/// let handle = runtime.handle();
/// tokio::spawn(runtime.run_async());
///
/// handle.send_filter_update(new_filters).await?;
/// ```
#[derive(Debug, Clone)]
pub struct RuntimeHandle {
    /// `None` when the source behind the runtime ignores filter updates, so a
    /// send fails at the call site instead of vanishing into a channel nobody
    /// reads.
    filter_updates_tx: Option<mpsc::Sender<Filters>>,
}

impl RuntimeHandle {
    pub(crate) fn new(filter_updates_tx: Option<mpsc::Sender<Filters>>) -> Self {
        Self { filter_updates_tx }
    }

    /// Replace the live subscription with `filters`.
    ///
    /// Each [`Filters`] sent replaces the whole subscription rather than
    /// adding to it, because that is what the gRPC servers do with a
    /// mid-stream request, so send the complete set every time.
    ///
    /// Keys are parser IDs. A key matching no registered pipeline still
    /// changes the wire subscription, so the server streams that data and the
    /// runtime then drops every update of it with only a trace-level line, so
    /// take the keys from the parsers actually registered on this runtime.
    ///
    /// The server applies the new set promptly, but a consumer sees it only
    /// once whatever it has already queued drains, so the delay is however far
    /// behind the pipeline already was rather than a property of the update.
    /// Measured against a live endpoint, a consumer running about 15 seconds
    /// behind kept receiving the old set for roughly that long after the send,
    /// and the first updates matching the new set arrived stale by the same
    /// margin before catching up to real time. A pipeline keeping pace sees
    /// the change almost at once.
    ///
    /// Treat `Ok(())` as the request having been handed off, not as the
    /// subscription having changed, and keep handlers able to cope with
    /// updates matching the old set until the backlog clears.
    ///
    /// Delivery is best effort. A set rejected while the source is between
    /// connections is retried once the stream recovers, but nothing reports
    /// back to the sender either way, and a newer set arriving in the meantime
    /// replaces the held one rather than queueing behind it. Every set is
    /// complete rather than a delta, so the server still ends on the newest,
    /// but an intermediate set can be skipped.
    ///
    /// A set the server itself refuses, by exceeding its configured filter
    /// limits for example, is answered on the stream with a code the client
    /// does not treat as recoverable, and the run ends with that error rather
    /// than the previous subscription staying in place. Under
    /// [`Runtime::run`](crate::Runtime::run) and
    /// [`Runtime::run_async`](crate::Runtime::run_async) that error is fatal
    /// and exits the process, so a set the provider will not accept takes the
    /// whole indexer down. Use
    /// [`Runtime::try_run_async`](crate::Runtime::try_run_async) if a caller
    /// needs to survive one.
    ///
    /// A set sent before the runtime starts is queued and applied as soon as
    /// the source connects. A short burst is buffered, and a sender that gets
    /// far ahead of the source waits here until it catches up.
    ///
    /// # Errors
    ///
    /// [`FilterUpdateError::Unsupported`] when the source cannot change its
    /// subscription mid-stream, and [`FilterUpdateError::Closed`] once the
    /// runtime has stopped.
    ///
    pub async fn send_filter_update(&self, filters: Filters) -> Result<(), FilterUpdateError> {
        let Some(tx) = &self.filter_updates_tx else {
            return Err(FilterUpdateError::Unsupported);
        };

        tx.send(filters)
            .await
            .map_err(|_| FilterUpdateError::Closed)
    }

    /// Blocking variant of [`Self::send_filter_update`], for a thread without
    /// a Tokio runtime of its own, such as one sitting next to a
    /// [`Runtime::run`](crate::Runtime::run) call.
    ///
    /// # Errors
    ///
    /// The same as [`Self::send_filter_update`].
    ///
    /// # Panics
    ///
    /// Panics when called from inside an asynchronous context, because
    /// blocking there would stall the executor. Use
    /// [`Self::send_filter_update`] from async code.
    ///
    pub fn blocking_send_filter_update(&self, filters: Filters) -> Result<(), FilterUpdateError> {
        let Some(tx) = &self.filter_updates_tx else {
            return Err(FilterUpdateError::Unsupported);
        };

        tx.blocking_send(filters)
            .map_err(|_| FilterUpdateError::Closed)
    }
}
