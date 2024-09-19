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

//! Vixen provides a simple API for requesting, parsing, and consuming data
//! from Yellowstone.

use builder::RuntimeBuilder;
use config::{BufferConfig, YellowstoneConfig};
use futures_util::future::OptionFuture;
use metrics::{Counters, Exporter, MetricsFactory, NullMetrics};
use stop::{StopCode, StopTx};
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
pub mod instruction;
pub mod metrics;
#[cfg(feature = "stream")]
pub mod stream;
mod util;
mod yellowstone;

pub use handler::{Handler, HandlerResult, Pipeline};
pub use util::*;

/// An error thrown by the Vixen runtime.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A system I/O error.
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    /// An error returned by a Yellowstone server.
    #[error("Yellowstone gRPC error")]
    Yellowstone(#[from] yellowstone::Error),
    /// An error occurring when the Yellowstone client stops early.
    #[error("Yellowstone client crashed")]
    ClientHangup,
    /// An error occurring when the Yellowstone server closes the connection.
    #[error("Yellowstone stream hung up unexpectedly")]
    ServerHangup,
    /// A gRPC error returned by the Yellowstone server.
    #[error("Yellowstone stream returned an error")]
    YellowstoneStatus(#[from] yellowstone_grpc_proto::tonic::Status),
    /// An error caused by the metrics exporter.
    #[error("Error exporting metrics")]
    MetricsExporter(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}

/// The main runtime for Vixen.
#[derive(Debug)]
pub struct Runtime<M: MetricsFactory> {
    yellowstone_cfg: YellowstoneConfig,
    buffer_cfg: BufferConfig,
    pipelines: handler::PipelineSets,
    counters: Counters<M::Instrumenter>,
    exporter: Option<M::Exporter>,
}

impl Runtime<NullMetrics> {
    /// Create a new runtime builder.
    pub fn builder() -> RuntimeBuilder { RuntimeBuilder::default() }
}

impl<M: MetricsFactory> Runtime<M> {
    /// Create a new Tokio runtime and run the Vixen runtime within it,
    /// terminating the current process if the runtime crashes.
    #[inline]
    pub fn run(self) { util::handle_fatal(self.try_run()); }

    /// Create a new Tokio runtime and run the Vixen runtime within it.
    ///
    /// # Errors
    /// This function returns an error if the runtime crashes.
    #[inline]
    pub fn try_run(self) -> Result<(), Error> {
        util::tokio_runtime()?.block_on(self.try_run_async())
    }

    /// Create a new [`LocalSet`] and run the Vixen runtime within it,
    /// terminating the current process if the runtime crashes.
    ///
    /// **NOTE:** This function **must** be called from within a Tokio runtime.
    #[inline]
    pub async fn run_async(self) { util::handle_fatal(self.try_run_async().await); }

    /// Create a new [`LocalSet`] and run the Vixen runtime within it.
    ///
    /// **NOTE:** This function **must** be called from within a Tokio runtime.
    ///
    /// # Errors
    /// This function returns an error if the runtime crashes.
    #[inline]
    pub async fn try_run_async(self) -> Result<(), Error> {
        LocalSet::new().run_until(self.try_run_local()).await
    }

    /// Run the Vixen runtime.
    ///
    /// **NOTE:** This function **must** be called from within a Tokio
    /// [`LocalSet`].
    ///
    /// # Errors
    /// This function returns an error if the runtime crashes.
    #[tracing::instrument("Runtime::run", skip(self))]
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

        let mut buffer = buffer::Buffer::run_yellowstone(buffer_cfg, client, pipelines, counters);

        let stop_ty = tokio::select! {
            s = signal => StopType::Signal(s),
            b = buffer.wait_for_stop() => StopType::Buffer(b),
            Some(x) = &mut exporter => StopType::Exporter(x),
        };

        let should_stop_buffer = !matches!(stop_ty, StopType::Buffer(..));
        let should_stop_exporter = !matches!(stop_ty, StopType::Exporter(..));

        match stop_ty {
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
        }?;

        if should_stop_buffer {
            Self::stop_buffer(buffer).await;
        }

        if should_stop_exporter {
            Self::stop_exporter(exporter, stop_exporter).await;
        }

        Ok(())
    }

    async fn stop_buffer(buffer: buffer::Buffer) {
        match buffer.join().await {
            Err(e) => tracing::warn!(err = %Chain(&e), "Error stopping runtime buffer"),
            Ok(c) => c.as_unit(),
        }
    }

    async fn stop_exporter(
        exporter: OptionFuture<
            tokio::task::JoinHandle<Result<StopCode, <M::Exporter as Exporter>::Error>>,
        >,
        tx: StopTx,
    ) {
        tx.maybe_send();

        let res = tokio::select! {
            e = exporter => Some(e),
            () = tokio::time::sleep(std::time::Duration::from_secs(5)) => None,
        };

        'unpack: {
            let Some(res) = res else {
                tracing::warn!("Metrics exporter took too long to stop");
                break 'unpack;
            };

            let Some(res) = res else { break 'unpack };

            match res {
                Err(e) => {
                    tracing::warn!(
                        err = %Chain(&e),
                        "Metrics exporter panicked after stop requested",
                    );
                },
                Ok(Err(e)) => {
                    tracing::warn!(
                        err = %Chain(&e),
                        "Metrics exporter returned an error after stop requested",
                    );
                },
                Ok(Ok(c)) => c.as_unit(),
            };
        }
    }
}
