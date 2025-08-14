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
use tracing::info;

use crate::{util, Runtime};

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
pub struct Server<'a> {
    grpc_cfg: GrpcConfig,
    desc_sets: Vec<&'a [u8]>,
    channels: Channels,
    runtime: Runtime,
}

impl fmt::Debug for Server<'_> {
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

impl Server<'_> {
    /// Create a new stream server builder.
    pub fn builder() -> StreamBuilder<'static> { StreamBuilder::default() }
}

impl Server<'_> {
    /// Create a new Tokio runtime and run the Vixen stream server within it,
    /// terminating the current process if the runtime or gRPC server crash.
    ///
    /// For error handling, use the recoverable variant [`Self::try_run`].
    ///
    /// If you want to provide your own tokio Runtime because you need to run
    /// async code outside of the Vixen stream server, use the [`Self::run_async`]
    /// method.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use yellowstone_vixen::stream;
    /// use yellowstone_vixen_meteora_parser::{
    ///     accounts_parser::AccountParser as MeteoraAccParser,
    ///     instructions_parser::InstructionParser as MeteoraIxParser,
    ///     proto_def::DESCRIPTOR_SET as METEORA_DESCRIPTOR_SET,
    /// };
    /// use yellowstone_vixen_pumpfun_parser::{
    ///     accounts_parser::AccountParser as PumpfunAccParser,
    ///     instructions_parser::InstructionParser as PumpfunIxParser,
    ///     proto_def::DESCRIPTOR_SET as PUMP_DESCRIPTOR_SET,
    /// };
    ///
    /// // NOTE: The main function is not async
    /// fn main() {
    ///     stream::Server::builder()
    ///         .descriptor_set(METEORA_DESCRIPTOR_SET)
    ///         .descriptor_set(PUMP_DESCRIPTOR_SET)
    ///         .account(Proto::new(MeteoraAccParser))
    ///         .instruction(Proto::new(MeteoraIxParser))
    ///         .instruction(Proto::new(PumpfunIxParser))
    ///         .build(config)
    ///         .run(); // Process will exit if an error occurs
    /// }
    /// ```
    #[inline]
    pub fn run(self) { util::handle_fatal(self.try_run()); }

    /// Error returning variant of [`Self::run`].
    ///
    /// # Errors
    /// This function returns an error if the runtime or gRPC server crash.
    #[inline]
    pub fn try_run(self) -> Result<(), Box<Error>> {
        tokio::runtime::Runtime::new()
            .map_err(|e| Box::new(e.into()))?
            .block_on(self.try_run_async())
    }

    /// Run the Vixen stream server asynchronously, terminating the current process
    /// if the runtime or gRPC server crash.
    ///
    /// For error handling, use the recoverable variant [`Self::try_run_async`].
    ///
    /// If you don't need to run any async code outside the Vixen stream server, you
    /// can use the [`Self::run`] method instead, which takes care of creating
    /// a tokio Runtime for you.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use yellowstone_vixen::stream;
    /// use yellowstone_vixen_meteora_parser::{
    ///     accounts_parser::AccountParser as MeteoraAccParser,
    ///     instructions_parser::InstructionParser as MeteoraIxParser,
    ///     proto_def::DESCRIPTOR_SET as METEORA_DESCRIPTOR_SET,
    /// };
    /// use yellowstone_vixen_pumpfun_parser::{
    ///     accounts_parser::AccountParser as PumpfunAccParser,
    ///     instructions_parser::InstructionParser as PumpfunIxParser,
    ///     proto_def::DESCRIPTOR_SET as PUMP_DESCRIPTOR_SET,
    /// };
    ///
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     stream::Server::builder()
    ///         .descriptor_set(METEORA_DESCRIPTOR_SET)
    ///         .descriptor_set(PUMP_DESCRIPTOR_SET)
    ///         .account(Proto::new(MeteoraAccParser))
    ///         .instruction(Proto::new(MeteoraIxParser))
    ///         .instruction(Proto::new(PumpfunIxParser))
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
    /// This function returns an error if the runtime or gRPC server crash.
    #[tracing::instrument("stream::Server::run", skip(self), err)]
    pub async fn try_run_async(self) -> Result<(), Box<Error>> {
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

        runtime
            .try_run_async()
            .await
            .map_err(|e| Box::new(Error::Runtime(*e)))?;
        grpc.stop().await.map_err(|e| Box::new(e.into()))?;
        Ok(())
    }
}
