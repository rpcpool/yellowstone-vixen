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

use builder::RuntimeBuilder;
use config::{BufferConfig, YellowstoneConfig};
use futures_util::future::OptionFuture;
use metrics::{Counters, Exporter, MetricsFactory, NullMetrics};
use tokio::task::LocalSet;
use vixen_core::{AccountUpdate, TransactionUpdate};

#[cfg(feature = "opentelemetry")]
pub extern crate opentelemetry;
#[cfg(feature = "prometheus")]
pub extern crate prometheus;
pub extern crate yellowstone_vixen_core as vixen_core;

mod buffer;
mod builder;
pub mod config;
pub mod handler;
pub mod metrics;
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
    #[error("Error exporting metrics")]
    MetricsExporter(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize)]
#[repr(transparent)]
pub struct PrivateString(pub String);

impl std::fmt::Debug for PrivateString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("<private>") }
}

impl std::ops::Deref for PrivateString {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl std::ops::DerefMut for PrivateString {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<String> for PrivateString {
    #[inline]
    fn from(value: String) -> Self { Self(value) }
}

impl From<PrivateString> for String {
    #[inline]
    fn from(PrivateString(value): PrivateString) -> Self { value }
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

pub mod stop {
    use std::pin::Pin;

    use tokio::sync::oneshot;

    #[derive(Debug)]
    #[allow(missing_copy_implementations)]
    #[repr(transparent)]
    pub struct StopCode(());

    #[derive(Debug)]
    #[repr(transparent)]
    pub struct StopTx(oneshot::Sender<StopCode>);

    impl StopTx {
        #[inline]
        pub fn send_or_else<F: FnOnce()>(self, f: F) {
            self.0.send(StopCode(())).unwrap_or_else(|StopCode(())| f());
        }

        #[inline]
        pub fn maybe_send(self) { self.send_or_else(|| ()); }
    }

    #[derive(Debug)]
    #[repr(transparent)]
    pub struct StopRx(oneshot::Receiver<StopCode>);

    impl std::future::Future for StopRx {
        type Output = StopCode;

        fn poll(
            self: Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Self::Output> {
            Pin::new(&mut self.get_mut().0)
                .poll(cx)
                .map(|r| r.unwrap_or(StopCode(())))
        }
    }

    #[must_use]
    pub fn channel() -> (StopTx, StopRx) {
        let (tx, rx) = oneshot::channel();
        (StopTx(tx), StopRx(rx))
    }
}

#[derive(Debug)]
pub struct Runtime<A, X, M: MetricsFactory> {
    yellowstone_cfg: YellowstoneConfig,
    buffer_cfg: BufferConfig,
    manager: HandlerManagers<A, X>,
    counters: Counters<M::Instrumenter>,
    exporter: Option<M::Exporter>,
}

impl<A, X> Runtime<A, X, NullMetrics> {
    #[must_use]
    pub fn builder() -> RuntimeBuilder<A, X, NullMetrics> { RuntimeBuilder::default() }
}

impl<
    A: DynHandlerPack<AccountUpdate> + Send + Sync + 'static,
    X: DynHandlerPack<TransactionUpdate> + Send + Sync + 'static,
    M: MetricsFactory,
> Runtime<A, X, M>
{
    fn handle_error(res: Result<(), Error>) {
        match res {
            Ok(()) => (),
            Err(e) => {
                tracing::error!(err = %Chain(&e), "Fatal error encountered");
                std::process::exit(1);
            },
        }
    }

    #[inline]
    pub fn run(self) { Self::handle_error(self.try_run()) }

    pub fn try_run(self) -> Result<(), Error> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?
            .block_on(self.try_run_async())
    }

    #[inline]
    pub async fn run_async(self) { Self::handle_error(self.try_run_async().await) }

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
            manager,
            counters,
            exporter,
        } = self;

        let (stop_exporter, rx) = stop::channel();
        let mut exporter = OptionFuture::from(exporter.map(|e| tokio::spawn(e.run(rx))));

        let client = yellowstone::connect(yellowstone_cfg, manager.filters()).await?;
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

        let buffer = buffer::run_yellowstone(buffer_cfg, client, manager, counters).wait_for_stop();

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
