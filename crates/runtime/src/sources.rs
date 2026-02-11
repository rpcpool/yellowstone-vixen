//! Sources for Vixen.
//!
//! A `SourceTrait` is a trait that defines the behavior for data sources that can be used to connect to it and
//! send updates to a channel. This trait is implemented by various modules, including the `yellowstone_grpc` module.

use async_trait::async_trait;
use tokio::sync::{mpsc::Sender, oneshot};
use vixen_core::Filters;
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
}
