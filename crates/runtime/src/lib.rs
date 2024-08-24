#![deny(
    clippy::disallowed_methods,
    clippy::suspicious,
    clippy::style,
    clippy::clone_on_ref_ptr,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::module_name_repetitions)]
// TODO: document everything
#![allow(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc)]

use builder::RuntimeBuilder;
use config::{BufferConfig, YellowstoneConfig};
use futures_util::future::OptionFuture;
use metrics::{Counters, Exporter, MetricsFactory, NullMetrics};
use tokio::task::LocalSet;

#[cfg(feature = "opentelemetry")]
pub extern crate opentelemetry;
#[cfg(feature = "prometheus")]
pub extern crate prometheus;
pub extern crate thiserror;
pub extern crate yellowstone_vixen_core as vixen_core;
pub use vixen_core::bs58;
#[cfg(feature = "stream")]
pub extern crate yellowstone_vixen_proto as proto;

mod buffer;
pub mod builder;
pub mod config;
pub mod handler;
pub mod metrics;
#[cfg(feature = "stream")]
pub mod stream;
mod util;
mod yellowstone;

pub use handler::{DynPipeline, Handler, HandlerResult, Pipeline, PipelineSet, PipelineSets};
pub use util::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Yellowstone gRPC error")]
    Yellowstone(#[from] yellowstone::Error),
    #[error("Yellowstone client crashed")]
    ClientHangup,
    #[error("Yellowstone stream hung up unexpectedly")]
    ServerHangup,
    #[error("Yellowstone stream returned an error")]
    YellowstoneStatus(#[from] yellowstone_grpc_proto::tonic::Status),
    #[error("Error exporting metrics")]
    MetricsExporter(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}

#[derive(Debug)]
pub struct Runtime<M: MetricsFactory> {
    yellowstone_cfg: YellowstoneConfig,
    buffer_cfg: BufferConfig,
    pipelines: PipelineSets,
    counters: Counters<M::Instrumenter>,
    exporter: Option<M::Exporter>,
}

impl Runtime<NullMetrics> {
    pub fn builder() -> RuntimeBuilder<NullMetrics> { RuntimeBuilder::default() }
}

impl<M: MetricsFactory> Runtime<M> {
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

    pub async fn try_run_local(self) -> Result<(), Error> {
        enum StopType<S, X> {
            Signal(S),
            Buffer(Result<std::convert::Infallible, Error>),
            Exporter(Result<Result<stop::StopCode, X>, tokio::task::JoinError>),
        }

        let Self {
            yellowstone_cfg,
            buffer_cfg,
            pipelines,
            counters,
            exporter,
        } = self;

        let (stop_exporter, rx) = stop::channel();
        let mut exporter = OptionFuture::from(exporter.map(|e| tokio::spawn(e.run(rx))));

        let client = yellowstone::connect(yellowstone_cfg, pipelines.filters()).await?;
        let signal;

        #[cfg(unix)]
        {
            use futures_util::stream::{FuturesUnordered, StreamExt};
            use tokio::signal::unix::SignalKind;

            let mut stream = [
                SignalKind::hangup(),
                SignalKind::interrupt(),
                SignalKind::quit(),
                SignalKind::terminate(),
            ]
            .into_iter()
            .map(|k| {
                tokio::signal::unix::signal(k).map(|mut s| async move {
                    s.recv().await;
                    Ok(k)
                })
            })
            .collect::<Result<FuturesUnordered<_>, _>>()?;

            signal = async move { stream.next().await.transpose() }
        }

        #[cfg(not(unix))]
        {
            use std::fmt;

            use futures_util::TryFutureExt;

            struct CtrlC;

            impl fmt::Debug for CtrlC {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("^C") }
            }

            signal = tokio::signal::ctrl_c()
                .map_ok(|()| Some(CtrlC))
                .map_err(Into::into);
        }

        let buffer =
            buffer::run_yellowstone(buffer_cfg, client, pipelines, counters).wait_for_stop();

        let ret = tokio::select! {
            s = signal => StopType::Signal(s),
            b = buffer => StopType::Buffer(b),
            Some(x) = &mut exporter => StopType::Exporter(x),
        };

        if !matches!(ret, StopType::Exporter(_)) {
            stop_exporter.maybe_send();

            match exporter.await {
                Some(Ok(Err(err))) => {
                    tracing::warn!(%err, "Metrics exporter returned an error after stop requested");
                },
                Some(Err(err)) => {
                    tracing::warn!(%err, "Metrics exporter crashed after stop requested");
                },
                Some(Ok(Ok(..))) | None => (),
            }
        }

        match ret {
            StopType::Signal(Ok(Some(s))) => {
                tracing::warn!("{s:?} received, shutting down...");
                Ok(())
            },
            StopType::Signal(Ok(None)) => Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "Signal handler returned None",
            )
            .into()),
            // Not sure why the compiler couldn't figure this one out
            StopType::Buffer(Ok(o)) => match o {},
            StopType::Signal(Err(e)) | StopType::Buffer(Err(e)) => Err(e),
            StopType::Exporter(Ok(Ok(..))) => {
                Err(Error::MetricsExporter("Exporter stopped early".into()))
            },
            StopType::Exporter(Ok(Err(e))) => Err(Error::MetricsExporter(e.into())),
            StopType::Exporter(Err(e)) => Err(Error::MetricsExporter(e.into())),
        }
    }
}
