//! gRPC server for streaming parsed Yellowstone updates.
//!
//! See [this simple example][ex1] for a basic example of how to use the Vixen
//! stream server, or [a more complex example][ex2] that serves parsed data
//! from Solana programs using pre-packaged Vixen parsers.
//!
//! [ex1]: https://github.com/rpcpool/yellowstone-vixen/blob/main/examples/stream/src/main.rs
//! [ex2]: https://github.com/rpcpool/yellowstone-vixen/blob/main/examples/stream-parser/src/main.rs

use std::fmt;

use config::GrpcConfig;
use grpc::Channels;
use tokio::task::LocalSet;
use tracing::info;

use crate::{
    metrics::{MetricsFactory, NullMetrics},
    util, Runtime,
};

mod builder;
pub mod config;
mod grpc;

pub use builder::*;

/// An error thrown by the Vixen stream server.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A gRPC transport error.
    #[error("Stream server error")]
    Grpc(#[from] grpc::Error),
    /// An error thrown by the Vixen runtime.
    #[error("Vixen client runtime error")]
    Runtime(#[from] crate::Error),
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(value: std::io::Error) -> Self { Self::Runtime(value.into()) }
}

/// A Vixen program stream server.
pub struct Server<'a, M: MetricsFactory> {
    grpc_cfg: GrpcConfig,
    desc_sets: Vec<&'a [u8]>,
    channels: Channels,
    runtime: Runtime<M>,
}

impl<'a, M: MetricsFactory + fmt::Debug> fmt::Debug for Server<'a, M>
where
    M::Instrumenter: fmt::Debug,
    M::Exporter: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            grpc_cfg,
            desc_sets,
            channels,
            runtime,
        } = self;
        f.debug_struct("Server")
            .field("grpc_cfg", grpc_cfg)
            .field("desc_sets", desc_sets)
            .field("channels", channels)
            .field("runtime", runtime)
            .finish()
    }
}

impl<'a> Server<'a, NullMetrics> {
    /// Create a new stream server builder.
    pub fn builder() -> StreamBuilder<'a> { StreamBuilder::default() }
}

impl<'a, M: MetricsFactory> Server<'a, M> {
    /// Create a new Tokio runtime and run the Vixen stream server within it,
    /// terminating the current process if the runtime or gRPC server crash.
    #[inline]
    pub fn run(self) { util::handle_fatal(self.try_run()); }

    /// Create a new Tokio runtime and run the Vixen stream server within it.
    ///
    /// # Errors
    /// This function returns an error if the runtime or gRPC server crash.
    #[inline]
    pub fn try_run(self) -> Result<(), Error> {
        util::tokio_runtime()?.block_on(self.try_run_async())
    }

    /// Create a new [`LocalSet`] and run the Vixen stream server within it,
    /// terminating the current process if the runtime or gRPC server crash.
    ///
    /// **NOTE:** This function **must** be called from within a Tokio runtime.
    #[inline]
    pub async fn run_async(self) { util::handle_fatal(self.try_run_async().await); }

    /// Create a new [`LocalSet`] and run the Vixen stream server within it.
    ///
    /// **NOTE:** This function **must** be called from within a Tokio runtime.
    ///
    /// # Errors
    /// This function returns an error if the runtime or gRPC server crash.
    #[inline]
    pub async fn try_run_async(self) -> Result<(), Error> {
        LocalSet::new().run_until(self.try_run_local()).await
    }

    /// Run the Vixen stream server.
    ///
    /// **NOTE:** This function **must** be called from within a Tokio
    /// [`LocalSet`].
    ///
    /// # Errors
    /// This function returns an error if the runtime or gRPC server crash.
    #[tracing::instrument("stream::Server::run", skip(self), err)]
    pub async fn try_run_local(self) -> Result<(), Error> {
        let Self {
            grpc_cfg,
            desc_sets,
            channels,
            runtime,
        } = self;

        let address = grpc_cfg.address;
        let grpc = grpc::Server::run(grpc_cfg, &desc_sets, channels);
        // TODO: check for early server shutdowns

        info!(%address, "gRPC server created");

        runtime.try_run_local().await?;
        grpc.stop().await?;
        Ok(())
    }
}
