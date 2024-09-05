use std::fmt;

use builder::StreamBuilder;
use config::GrpcConfig;
use grpc::Channels;
use tokio::task::LocalSet;
use tracing::info;

use crate::{
    metrics::{MetricsFactory, NullMetrics},
    util, Runtime,
};

pub mod builder;
pub mod config;
mod grpc;

pub use grpc::GrpcHandler;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Stream server error")]
    Grpc(#[from] grpc::Error),
    #[error("Vixen client runtime error")]
    Runtime(#[from] crate::Error),
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(value: std::io::Error) -> Self { Self::Runtime(value.into()) }
}

pub struct Server<M: MetricsFactory> {
    grpc_cfg: GrpcConfig,
    channels: Channels,
    runtime: Runtime<M>,
}

impl<M: MetricsFactory + fmt::Debug> fmt::Debug for Server<M>
where
    M::Instrumenter: fmt::Debug,
    M::Exporter: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            grpc_cfg,
            channels,
            runtime,
        } = self;
        f.debug_struct("Server")
            .field("grpc_cfg", grpc_cfg)
            .field("channels", channels)
            .field("runtime", runtime)
            .finish()
    }
}

impl Server<NullMetrics> {
    pub fn builder() -> StreamBuilder<NullMetrics> { StreamBuilder::default() }
}

impl<M: MetricsFactory> Server<M> {
    #[inline]
    pub fn run(self) { util::handle_fatal(self.try_run()); }

    #[inline]
    pub fn try_run(self) -> Result<(), Error> {
        util::tokio_runtime()?.block_on(self.try_run_async())
    }

    #[inline]
    pub async fn run_async(self) { util::handle_fatal(self.try_run_async().await); }

    #[inline]
    pub async fn try_run_async(self) -> Result<(), Error> {
        LocalSet::new().run_until(self.try_run_local()).await
    }

    #[tracing::instrument("stream::Server::run", skip(self), err)]
    pub async fn try_run_local(self) -> Result<(), Error> {
        let Self {
            grpc_cfg,
            channels,
            runtime,
        } = self;

        let address = grpc_cfg.address;
        let grpc = grpc::Server::run(grpc_cfg, channels);
        // TODO: check for early server shutdowns

        info!(%address, "gRPC server created");

        runtime.try_run_local().await?;
        grpc.stop().await?;
        Ok(())
    }
}
