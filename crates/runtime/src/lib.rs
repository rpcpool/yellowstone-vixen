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
pub mod handler;
mod yellowstone;

pub use handler::{
    DynHandlerPack, Handler, HandlerManager, HandlerManagers, HandlerPack, HandlerResult,
};

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
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(LocalSet::new().run_until(run_async(opts, manager)))
}

async fn run_async<
    A: DynHandlerPack<AccountUpdate> + Send + Sync + 'static,
    X: DynHandlerPack<TransactionUpdate> + Send + Sync + 'static,
>(
    opts: IndexerOpts,
    manager: HandlerManagers<A, X>,
) -> Result<(), Error> {
    enum StopType<S> {
        Signal(S),
        Buffer(Result<std::convert::Infallible, Error>),
    }

    let IndexerOpts {
        yellowstone,
        buffer,
    } = opts;

    let client = yellowstone::connect(yellowstone, manager.filters()).await?;
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

    let buffer = buffer::run_yellowstone(buffer, client, manager).wait_for_stop();

    let ret = tokio::select! {
        s = signal => StopType::Signal(s),
        b = buffer => StopType::Buffer(b),
    };

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
        StopType::Signal(Err(e)) => Err(e),
        // Not sure why the compiler couldn't figure this one out
        StopType::Buffer(Ok(o)) => match o {},
        StopType::Buffer(Err(e)) => Err(e),
    }
}
