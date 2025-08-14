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
use sources::Source;
use tokio::{
    sync::mpsc::{self, Receiver},
    task::JoinSet,
};
use yellowstone_grpc_proto::{
    // prelude::*,
    tonic::Status,
};

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
pub mod sources;
#[cfg(feature = "stream")]
pub mod stream;
mod util;

pub mod filter_pipeline;

pub use handler::{Handler, HandlerResult, Pipeline};
pub use util::*;
pub use yellowstone_grpc_proto::geyser::CommitmentLevel;
use yellowstone_grpc_proto::geyser::SubscribeUpdate;

/// An error thrown by the Vixen runtime.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A system I/O error.
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    /// An error returned by a Yellowstone server.
    #[error("Yellowstone client builder error")]
    YellowstoneBuilder(#[from] yellowstone_grpc_client::GeyserGrpcBuilderError),
    /// An error returned by a Yellowstone client.
    #[error("Yellowstone client error")]
    YellowstoneClient(#[from] yellowstone_grpc_client::GeyserGrpcClientError),
    /// An error occurring when the Yellowstone client stops early.
    #[error("Yellowstone client crashed")]
    ClientHangup,
    /// An error occurring when the Yellowstone server closes the connection.
    #[error("Yellowstone stream hung up unexpectedly")]
    ServerHangup,
    /// A gRPC error returned by the Yellowstone server.
    #[error("Yellowstone stream returned an error")]
    YellowstoneStatus(#[from] yellowstone_grpc_proto::tonic::Status),
    /// An error occurring when a datasource is not configured correctly.
    #[error("Yellowstone stream config error")]
    ConfigError,
}

/// The main runtime for Vixen.
#[derive(Debug)]
pub struct Runtime {
    yellowstone_cfg: YellowstoneConfig,
    sources: Vec<Box<dyn Source>>,
    commitment_filter: Option<CommitmentLevel>,
    from_slot_filter: Option<u64>,
    buffer_cfg: BufferConfig,
    pipelines: handler::PipelineSets,
    metrics_registry: Option<prometheus::Registry>,
}

impl Runtime {
    /// Create a new runtime builder.
    pub fn builder() -> RuntimeBuilder { RuntimeBuilder::default() }
}

impl Runtime {
    /// Create a new Tokio runtime and run the Vixen runtime within it,
    /// terminating the current process if the runtime crashes.
    ///
    /// For error handling, use the recoverable variant [`Self::try_run`].
    ///
    /// If you want to provide your own tokio Runtime because you need to run
    /// async code outside of the Vixen runtime, use the [`Self::run_async`]
    /// method.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use yellowstone_vixen::Pipeline;
    /// use yellowstone_vixen_parser::{
    ///     token_extension_program::{
    ///         AccountParser as TokenExtensionProgramAccParser,
    ///         InstructionParser as TokenExtensionProgramIxParser,
    ///     },
    ///     token_program::{
    ///         AccountParser as TokenProgramAccParser, InstructionParser as TokenProgramIxParser,
    ///     },
    /// };
    ///
    /// // MyHandler is a handler that implements the Handler trait
    /// // NOTE: The main function is not async
    /// fn main() {
    ///     Runtime::builder()
    ///         .source(YellowstoneGrpcSource::new())
    ///         .account(Pipeline::new(TokenProgramAccParser, [MyHandler]))
    ///         .account(Pipeline::new(TokenExtensionProgramAccParser, [MyHandler]))
    ///         .instruction(Pipeline::new(TokenExtensionProgramIxParser, [MyHandler]))
    ///         .instruction(Pipeline::new(TokenProgramIxParser, [MyHandler]))
    ///         .metrics(yellowstone_vixen::metrics::Prometheus)
    ///         .commitment_level(yellowstone_vixen::CommitmentLevel::Confirmed)
    ///         .build(config)
    ///         .run(); // Process will exit if an error occurs
    /// }
    /// ```
    #[inline]
    pub fn run(self) { util::handle_fatal(self.try_run()); }

    /// Error returning variant of [`Self::run`].
    ///
    /// # Errors
    /// This function returns an error if the runtime crashes.
    #[inline]
    pub fn try_run(self) -> Result<(), Box<Error>> {
        tokio::runtime::Runtime::new()
            .map_err(|e| Box::new(e.into()))?
            .block_on(self.try_run_async())
    }

    /// Run the Vixen runtime asynchronously, terminating the current process
    /// if the runtime crashes.
    ///
    /// For error handling, use the recoverable variant [`Self::try_run_async`].
    ///
    /// If you don't need to run any async code outside the Vixen runtime, you
    /// can use the [`Self::run`] method instead, which takes care of creating
    /// a tokio Runtime for you.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use yellowstone_vixen_parser::{
    ///     token_extension_program::{
    ///         AccountParser as TokenExtensionProgramAccParser,
    ///         InstructionParser as TokenExtensionProgramIxParser,
    ///     },
    ///     token_program::{
    ///         AccountParser as TokenProgramAccParser, InstructionParser as TokenProgramIxParser,
    ///     },
    /// };
    ///
    /// // MyHandler is a handler that implements the Handler trait
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     Runtime::builder()
    ///         .source(YellowstoneGrpcSource::new())
    ///         .account(Pipeline::new(TokenProgramAccParser, [MyHandler]))
    ///         .account(Pipeline::new(TokenExtensionProgramAccParser, [MyHandler]))
    ///         .instruction(Pipeline::new(TokenExtensionProgramIxParser, [MyHandler]))
    ///         .instruction(Pipeline::new(TokenProgramIxParser, [MyHandler]))
    ///         .metrics(yellowstone_vixen::metrics::Prometheus)
    ///         .commitment_level(yellowstone_vixen::CommitmentLevel::Confirmed)
    ///         .build(config)
    ///         .run_async()
    ///         .await;
    /// }
    /// ```
    #[inline]
    pub async fn run_async(self) { util::handle_fatal(self.try_run_async().await); }

    /// Error returning variant of [`Self::run_async`].
    ///
    /// # Errors
    /// This function returns an error if the runtime crashes.
    #[tracing::instrument("Runtime::run", skip(self))]
    pub async fn try_run_async(self) -> Result<(), Box<Error>> {
        enum StopType<S> {
            Signal(S),
            Buffer(Result<(), Error>),
        }

        let (runtime, updates_rx, _set_handles) = self.connect_to_sources();

        let Self {
            yellowstone_cfg: _,
            sources: _,
            commitment_filter: _,
            from_slot_filter: _,
            buffer_cfg,
            pipelines,
            metrics_registry,
        } = runtime;

        if let Some(metrics_registry) = metrics_registry {
            metrics::register_metrics(&metrics_registry);
        }

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
            .collect::<Result<FuturesUnordered<_>, _>>()
            .map_err(|e| Box::new(e.into()))?;

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

        let mut buffer = buffer::Buffer::run_yellowstone(buffer_cfg, updates_rx, pipelines);

        let stop_ty = tokio::select! {
            s = signal => StopType::Signal(s),
            b = buffer.wait_for_stop() => StopType::Buffer(b),
        };

        let should_stop_buffer = !matches!(stop_ty, StopType::Buffer(..));

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
            StopType::Buffer(result) => result,
            StopType::Signal(Err(e)) => Err(e),
        }?;

        if should_stop_buffer {
            Self::stop_buffer(buffer).await;
        }

        Ok(())
    }

    async fn stop_buffer(buffer: buffer::Buffer) {
        match buffer.join().await {
            Err(e) => tracing::warn!(err = %Chain(&e), "Error stopping runtime buffer"),
            Ok(c) => c.as_unit(),
        }
    }

    /// Connect to the sources and return the runtime, the receiver for the updates, and the join set for the sources.
    fn connect_to_sources(
        mut self,
    ) -> (Self, Receiver<Result<SubscribeUpdate, Status>>, JoinSet<()>) {
        let (tx, rx) =
            mpsc::channel::<Result<SubscribeUpdate, Status>>(self.buffer_cfg.sources_channel_size);

        let filters = self
            .pipelines
            .filters()
            .commitment(self.commitment_filter)
            .from_slot(self.from_slot_filter);

        let mut set = JoinSet::new();

        for mut source in self.sources.drain(..) {
            let tx = tx.clone();
            source.config(self.yellowstone_cfg.clone());
            source.filters(filters.clone());

            set.spawn(async move {
                source.connect(tx).await.unwrap_or_else(|_| {
                    panic!("Source connection failed for: {}", source.name());
                });
            });
        }

        (self, rx, set)
    }
}
