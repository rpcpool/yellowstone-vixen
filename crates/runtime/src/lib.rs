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

use std::fmt;

use buffer::BufferOpts;
use builder::RuntimeBuilder;
use metrics::{Metrics, MetricsBackend, NullMetrics};
use tokio::task::LocalSet;
use vixen_core::{AccountUpdate, TransactionUpdate};
use yellowstone::YellowstoneOpts;

#[cfg(feature = "opentelemetry")]
pub extern crate opentelemetry;
#[cfg(feature = "prometheus")]
pub extern crate prometheus;
pub extern crate yellowstone_vixen_core as vixen_core;

mod buffer;
mod builder;
pub mod handler;
mod metrics;
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

#[derive(Debug, Clone, Copy)]
struct Chain<'a, E>(&'a E);

impl<'a, E: std::error::Error> fmt::Display for Chain<'a, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Write;

        enum IndentState {
            NumberStart(usize), // Numbered and indented initial line
            PlainStart,         // Plain indented initial line
            HangStart,          // Hanging indent for successive lines
            MidLine,            // No indent, not at line start
        }

        struct Indented<'a, F> {
            f: &'a mut F,
            state: IndentState,
        }

        impl<'a, F: Write> Indented<'a, F> {
            fn write_pad(&mut self) -> fmt::Result {
                match std::mem::replace(&mut self.state, IndentState::MidLine) {
                    IndentState::NumberStart(i) => write!(self.f, "{i: >5}: "),
                    IndentState::PlainStart => write!(self.f, "    "),
                    IndentState::HangStart => write!(self.f, "      "),
                    IndentState::MidLine => Ok(()),
                }
            }
        }

        impl<'a, F: Write> Write for Indented<'a, F> {
            fn write_str(&mut self, mut s: &str) -> fmt::Result {
                while let Some((head, tail)) = s.split_once('\n') {
                    if !head.is_empty() {
                        self.write_pad()?;
                    }
                    self.f.write_str(head)?;
                    self.f.write_char('\n')?;
                    self.state = IndentState::HangStart;
                    s = tail;
                }

                let trail = !s.is_empty();
                if trail {
                    self.write_pad()?;
                }
                self.f.write_str(s)?;
                self.state = if trail {
                    IndentState::MidLine
                } else {
                    IndentState::HangStart
                };
                Ok(())
            }
        }

        let Self(err) = *self;

        write!(f, "{err}")?;

        if let src @ Some(_) = err.source() {
            let mut multi_src = false;

            for (i, src) in std::iter::successors(src, |s| s.source()).enumerate() {
                if i == 0 {
                    write!(f, "\nCaused by:")?;
                    multi_src = src.source().is_some();
                }

                writeln!(f)?;
                write!(
                    Indented {
                        f,
                        state: if multi_src {
                            IndentState::NumberStart(i)
                        } else {
                            IndentState::PlainStart
                        },
                    },
                    "{src}"
                )?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Runtime<A, X, M: MetricsBackend> {
    opts: IndexerOpts,
    manager: HandlerManagers<A, X>,
    metrics: Metrics<M>,
}

impl<A, X> Runtime<A, X, NullMetrics> {
    #[must_use]
    pub fn builder() -> RuntimeBuilder<A, X, NullMetrics> { RuntimeBuilder::default() }
}

impl<
    A: DynHandlerPack<AccountUpdate> + Send + Sync + 'static,
    X: DynHandlerPack<TransactionUpdate> + Send + Sync + 'static,
    M: MetricsBackend,
> Runtime<A, X, M>
{
    pub fn run(self) {
        match self.try_run() {
            Ok(()) => (),
            Err(e) => {
                tracing::error!(err = %Chain(&e), "Fatal error encountered");
                std::process::exit(1);
            },
        }
    }

    pub fn try_run(self) -> Result<(), Error> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?
            .block_on(LocalSet::new().run_until(self.try_run_async()))
    }

    pub async fn try_run_async(self) -> Result<(), Error> {
        enum StopType<S> {
            Signal(S),
            Buffer(Result<std::convert::Infallible, Error>),
        }

        let Self {
            opts: IndexerOpts {
                yellowstone,
                buffer,
            },
            manager,
            metrics,
        } = self;

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

        let buffer = buffer::run_yellowstone(buffer, client, manager, metrics).wait_for_stop();

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
            // Not sure why the compiler couldn't figure this one out
            StopType::Buffer(Ok(o)) => match o {},
            StopType::Signal(Err(e)) | StopType::Buffer(Err(e)) => Err(e),
        }
    }
}
