// TODO
#![allow(dead_code, unused)]

use buffer::BufferOpts;
use tokio::task::LocalSet;
use vixen_core::{AccountUpdate, TransactionUpdate};
use yellowstone::YellowstoneOpts;

pub extern crate yellowstone_vixen_core as vixen_core;

mod buffer;
// mod parser;
// mod parser_manager;
mod handler;
mod yellowstone;

pub use handler::{DynHandlerPack, Handler, HandlerManager, HandlerManagers};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Yellowstone gRPC error")]
    Yellowstone(#[from] yellowstone::Error),
}

#[derive(Debug, clap::Args, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IndexerOpts {
    #[command(flatten)]
    yellowstone: YellowstoneOpts,

    #[command(flatten)]
    #[serde(default)]
    buffer: BufferOpts,
}

pub fn run<
    A: DynHandlerPack<AccountUpdate> + Send + Sync + 'static,
    X: DynHandlerPack<TransactionUpdate> + Send + Sync + 'static,
>(
    opts: IndexerOpts,
    manager: HandlerManagers<A, X>,
) {
    match try_run(opts, manager) {
        Ok(()) => (),
        Err(e) => {
            tracing::error!(err = ?anyhow::Error::new(e), "Fatal error encountered");
            std::process::exit(1);
        },
    }
}

pub fn try_run<
    A: DynHandlerPack<AccountUpdate> + Send + Sync + 'static,
    X: DynHandlerPack<TransactionUpdate> + Send + Sync + 'static,
>(
    opts: IndexerOpts,
    manager: HandlerManagers<A, X>,
) -> Result<(), Error> {
    let IndexerOpts {
        yellowstone,
        buffer,
    } = opts;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let client = yellowstone::connect(yellowstone, manager.filters()).await?;
            let locals = LocalSet::new();

            locals
                .run_until(async move {
                    let buf = buffer::run_yellowstone(buffer, client, manager);

                    // TODO
                    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                })
                .await;

            Ok(())
        })
}
