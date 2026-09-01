//! Sources for Shipstern.
//!
//! A `SourceTrait` is a trait that defines the behavior for data sources that can be used to connect to it and
//! send updates to a channel. This trait is implemented by various modules, including the `yellowstone_grpc` module.

use async_trait::async_trait;
use shipstern_core::Filters;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    oneshot,
};
use yellowstone_grpc_proto::{geyser::SubscribeUpdate, tonic};

/// How a source exited.
#[derive(Debug)]
pub enum SourceExitStatus {
    /// Update channel receiver was dropped.
    ReceiverDropped,
    /// Source finished successfully (finite sources like snapshot/RPC).
    Completed,
    /// Server closed connection unexpectedly (streaming sources).
    StreamEnded,
    /// gRPC error.
    StreamError {
        /// gRPC status code.
        code: tonic::Code,
        /// Server error message.
        message: String,
    },
    /// Other errors.
    Error(String),
}

/// Data source that streams updates to the runtime.
///
/// Implement this trait to create custom sources. See `YellowstoneGrpcSource` for an example.
#[async_trait]
pub trait SourceTrait: std::fmt::Debug + Send + Sync + 'static {
    /// Source-specific configuration.
    type Config: serde::de::DeserializeOwned + clap::Args + std::fmt::Debug;

    /// Create a source from config and filters.
    fn new(config: Self::Config, filters: Filters) -> Self;

    /// Connect and stream updates. Send exit status via `status_tx` before returning.
    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, tonic::Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
    ) -> Result<(), crate::Error>;

    /// Whether this source applies filter updates to a live subscription.
    ///
    /// The runtime checks this before handing a caller the sending half of the
    /// filter update channel, so a caller wiring updates to a source that
    /// ignores them finds out at the call site instead of silently sending
    /// into a void.
    ///
    #[must_use]
    fn supports_filter_updates() -> bool { false }

    /// Connect and stream updates, applying filter sets received on
    /// `filter_updates` to the live subscription.
    ///
    /// The default ignores `filter_updates` and defers to [`Self::connect`],
    /// so a source that cannot change its subscription mid-stream needs no
    /// implementation. Override this together with
    /// [`Self::supports_filter_updates`].
    ///
    async fn connect_with_filter_updates(
        &self,
        tx: Sender<Result<SubscribeUpdate, tonic::Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
        filter_updates: Receiver<Filters>,
    ) -> Result<(), crate::Error> {
        drop(filter_updates);

        self.connect(tx, status_tx).await
    }
}
