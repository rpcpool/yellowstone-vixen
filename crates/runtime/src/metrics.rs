//! Metrics interface and backend support for the Vixen runtime.

use std::{
    borrow::{Borrow, Cow},
    convert::Infallible,
    error::Error,
    fmt,
    future::Future,
};

#[cfg(feature = "opentelemetry")]
pub use opentelemetry_impl::*;
#[cfg(feature = "prometheus")]
pub use prometheus_impl::*;
use yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof;

use crate::{
    config::{MaybeDefault, NullConfig},
    handler::PipelineErrors,
    stop::{StopCode, StopRx},
};

/// The constructed metrics backend from a [`MetricsFactory`].
#[derive(Debug)]
pub struct Metrics<F: MetricsFactory + ?Sized>(pub F::Instrumenter, pub Option<F::Exporter>);

/// The result of creating a metrics backend from a [`MetricsFactory`].
pub type FactoryResult<F> = Result<Metrics<F>, <F as MetricsFactory>::Error>;

/// A factory for creating metrics backends.
pub trait MetricsFactory {
    /// Configuration type for the metrics backend.
    type Config: clap::Args + for<'de> serde::Deserialize<'de> + MaybeDefault;
    /// The type of code instrumenter created by this factory.
    type Instrumenter: Instrumenter;
    /// The type of metrics exporter created by this factory.
    type Exporter: Exporter;
    /// Error thrown when creating a metrics backend with this factory.
    type Error: Error + Send + Sync + 'static;

    /// Construct a metrics backend from this factory with the given
    /// configuration and service ID.
    ///
    /// # Errors
    /// This function may return an error if the metrics backend cannot be
    /// created.
    fn create(self, config: Self::Config, id: &'static str) -> FactoryResult<Self>;
}

/// A metrics instrumenter.
pub trait Instrumenter: 'static {
    /// The type of an integer counter for this metrics backend.
    type Counter: Counter;

    /// Create a new integer counter with the given name and description.
    fn make_counter(
        &self,
        name: impl Into<Cow<'static, str>>,
        desc: impl Into<Cow<'static, str>>,
    ) -> Self::Counter;
}

/// A metrics exporter.
pub trait Exporter {
    /// Error thrown when running the exporter.
    type Error: Error + Send + Sync + 'static;

    /// Run the exporter until the given stop signal is received.
    ///
    /// # Errors
    /// This function may yield early if an error occurs while running the
    /// exporter.
    fn run(
        self,
        stop: StopRx,
    ) -> impl Future<Output = Result<StopCode, Self::Error>> + Send + 'static;
}

/// An integer counter for a metrics backend.
pub trait Counter: Send + Sync {
    /// Increment the counter by one.
    #[inline]
    fn inc(&self) { self.inc_by(1); }

    /// Increment the counter by the given amount.
    fn inc_by(&self, by: u64);
}

impl<T: Counter> Counter for &T {
    #[inline]
    fn inc(&self) { T::inc(self); }

    #[inline]
    fn inc_by(&self, by: u64) { T::inc_by(self, by); }
}

/// A no-op metrics backend.
#[derive(Debug, Default, Clone, Copy)]
pub struct NullMetrics;

impl MetricsFactory for NullMetrics {
    type Config = NullConfig;
    type Error = Infallible;
    type Exporter = Infallible;
    type Instrumenter = Self;

    fn create(self, NullConfig: Self::Config, _: &'static str) -> FactoryResult<Self> {
        Ok(Metrics(Self, None))
    }
}

impl Instrumenter for NullMetrics {
    type Counter = NullMetrics;

    #[inline]
    fn make_counter(
        &self,
        _: impl Into<Cow<'static, str>>,
        _: impl Into<Cow<'static, str>>,
    ) -> Self::Counter {
        NullMetrics
    }
}

impl Counter for NullMetrics {
    #[inline]
    fn inc_by(&self, _: u64) {}
}

impl Exporter for Infallible {
    type Error = Infallible;

    async fn run(self, _: StopRx) -> Result<StopCode, Self::Error> { match self {} }
}

#[cfg(feature = "prometheus")]
mod prometheus_impl {
    use std::{borrow::Cow, time::Duration};

    use prometheus::Registry;

    use super::{FactoryResult, Metrics};
    use crate::{
        config::PrometheusConfig,
        stop::{StopCode, StopRx},
    };

    /// The Prometheus metrics backend.
    #[derive(Debug, Clone, Copy)]
    pub struct Prometheus;

    impl super::MetricsFactory for Prometheus {
        type Config = PrometheusConfig;
        type Error = prometheus::Error;
        type Exporter = PrometheusExporter;
        type Instrumenter = Registry;

        fn create(self, config: Self::Config, id: &'static str) -> FactoryResult<Self> {
            Registry::new_custom(Some(id.into()), None)
                .map(|r| Metrics(r.clone(), Some(PrometheusExporter(r, config))))
                .map_err(Into::into)
        }
    }

    /// Exporter for the Prometheus metrics backend.
    #[derive(Debug, Clone)]
    pub struct PrometheusExporter(Registry, PrometheusConfig);

    impl super::Exporter for PrometheusExporter {
        type Error = prometheus::Error;

        async fn run(self, mut stop: StopRx) -> Result<StopCode, Self::Error> {
            loop {
                let ret = tokio::select! {
                    () = tokio::time::sleep(Duration::from_secs(self.1.export_interval)) => None,
                    c = &mut stop => Some(c),
                };

                let me = self.clone();
                // spawn_blocking is required here, see the comment below
                tokio::task::spawn_blocking(move || {
                    // TODO: this spawns a Tokio runtime, which is dumb since we're already in one
                    prometheus::push_metrics(
                        &me.1.job,
                        prometheus::labels! {},
                        &me.1.endpoint,
                        me.0.gather(),
                        Some(prometheus::BasicAuthentication {
                            username: me.1.username.clone(),
                            password: me.1.password.to_string(),
                        }),
                    )
                })
                .await
                .map_err(std::io::Error::from)??;

                if let Some(ret) = ret {
                    return Ok(ret);
                }
            }
        }
    }

    impl super::Instrumenter for Registry {
        type Counter = prometheus::IntCounter;

        fn make_counter(
            &self,
            name: impl Into<Cow<'static, str>>,
            desc: impl Into<Cow<'static, str>>,
        ) -> Self::Counter {
            let counter =
                prometheus::IntCounter::with_opts(prometheus::Opts::new(name.into(), desc.into()))
                    .unwrap();
            self.register(Box::new(counter.clone())).unwrap();
            counter
        }
    }

    impl super::Counter for prometheus::IntCounter {
        fn inc_by(&self, by: u64) { prometheus::IntCounter::inc_by(self, by); }
    }
}

#[cfg(feature = "opentelemetry")]
mod opentelemetry_impl {
    use std::{borrow::Cow, convert::Infallible};

    use opentelemetry::{
        global::{self, GlobalMeterProvider},
        metrics::{Counter, Meter, MeterProvider},
    };

    use super::{FactoryResult, Metrics};
    use crate::{
        config::NullConfig,
        stop::{StopCode, StopRx},
    };

    /// The OpenTelemetry metrics backend.
    #[derive(Debug, Clone, Copy)]
    #[repr(transparent)]
    pub struct OpenTelemetry<M>(M);

    impl OpenTelemetry<GlobalMeterProvider> {
        /// Create a new OpenTelemetry metrics backend from the global meter
        /// provider.
        #[inline]
        #[must_use]
        pub fn global() -> Self { Self(global::meter_provider()) }
    }

    impl<M> OpenTelemetry<M> {
        /// Create a new OpenTelemetry metrics backend from the given meter
        /// provider.
        #[inline]
        #[must_use]
        pub fn new(meter_provider: M) -> Self { Self(meter_provider) }
    }

    impl<M: MeterProvider + Send + 'static> super::MetricsFactory for OpenTelemetry<M> {
        type Config = NullConfig;
        type Error = Infallible;
        type Exporter = OpenTelemetryExporter<M>;
        type Instrumenter = Meter;

        fn create(self, NullConfig: Self::Config, id: &'static str) -> FactoryResult<Self> {
            let Self(meter_provider) = self;
            Ok(Metrics(
                meter_provider.meter(id),
                Some(OpenTelemetryExporter(meter_provider)),
            ))
        }
    }

    /// Exporter for the OpenTelemetry metrics backend.
    #[derive(Debug, Clone, Copy)]
    #[repr(transparent)]
    pub struct OpenTelemetryExporter<M>(M);

    impl<M: Send + 'static> super::Exporter for OpenTelemetryExporter<M> {
        type Error = Infallible;

        async fn run(self, stop: StopRx) -> Result<StopCode, Self::Error> {
            let Self(meter_provider) = self;
            let res = stop.await;
            tokio::task::spawn_blocking(|| drop(meter_provider));
            Ok(res)
        }
    }

    impl super::Instrumenter for Meter {
        type Counter = Counter<u64>;

        fn make_counter(
            &self,
            name: impl Into<Cow<'static, str>>,
            desc: impl Into<Cow<'static, str>>,
        ) -> Self::Counter {
            self.u64_counter(name).with_description(desc).init()
        }
    }

    impl super::Counter for Counter<u64> {
        fn inc_by(&self, by: u64) { self.add(by, &[]); }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum JobResult {
    Ok,
    ParseErr,
    HandleErr(usize),
}

impl JobResult {
    pub fn from_pipeline<R: Borrow<Result<U, PipelineErrors>>, U>(res: R) -> Option<Self> {
        Some(match res.borrow() {
            Ok(_) => Self::Ok,
            Err(PipelineErrors::Parse(_)) => Self::ParseErr,
            Err(PipelineErrors::Handlers(v)) => Self::HandleErr(v.len()),
            Err(PipelineErrors::AlreadyHandled(_)) => return None,
        })
    }
}

pub(crate) trait Update {
    const TYPE: UpdateType;
}

impl Update for vixen_core::AccountUpdate {
    const TYPE: UpdateType = UpdateType::Account;
}

impl Update for vixen_core::TransactionUpdate {
    const TYPE: UpdateType = UpdateType::Transaction;
}

/// Tuple of `(singular, plural)`
#[derive(Clone, Copy)]
struct Noun(&'static str, &'static str);

macro_rules! noun_formatters {
    () => {};

    ($name:ident($s:pat, $p:pat) => $fmt:literal $(, $($tts:tt)*)?) => {
        fn $name(Noun($s, $p): Noun) -> String { format!($fmt) }
        $(noun_formatters!($($tts)*);)?
    };
}

#[derive(Clone, Copy)]
pub(crate) enum UpdateType {
    Account,
    Transaction,
}

impl UpdateType {
    pub fn get(update: &Option<UpdateOneof>) -> Option<Self> {
        match update {
            Some(UpdateOneof::Account(vixen_core::AccountUpdate { .. })) => Some(Self::Account),
            Some(UpdateOneof::Transaction(vixen_core::TransactionUpdate { .. })) => {
                Some(Self::Transaction)
            },
            _ => None,
        }
    }

    const fn noun(self) -> Noun {
        match self {
            UpdateType::Account => Noun("account", "accounts"),
            UpdateType::Transaction => Noun("transaction", "transactions"),
        }
    }
}

struct UpdateCounters<B: Instrumenter> {
    account: B::Counter,
    transaction: B::Counter,
}

impl<B: Instrumenter> UpdateCounters<B> {
    fn new<F: Fn(Noun) -> B::Counter>(f: F) -> Self {
        let f = move |t: UpdateType| f(t.noun());
        Self {
            account: f(UpdateType::Account),
            transaction: f(UpdateType::Transaction),
        }
    }

    fn get(&self, ty: UpdateType) -> &B::Counter {
        match ty {
            UpdateType::Account => &self.account,
            UpdateType::Transaction => &self.transaction,
        }
    }
}

struct ResultCounters<C> {
    handled: C,
    ok: C,
    parse_err: C,
    handle_err: C,
    total_handle_errs: C,
}

impl<C> ResultCounters<C> {
    // `f` receives two function pointers, one for a counter name and another
    // for a counter description
    fn new<F: Fn(fn(Noun) -> String, fn(Noun) -> String) -> C>(f: F) -> Self {
        noun_formatters! {
            handled_name(_, p) => "{p}_processed",
            handled_desc(_, p) => "Number of (successfully or unsuccessfully) processed {p}",
            ok_name(_, p) => "successful_{p}",
            ok_desc(_, p) => "Number of successfully processed {p}",
            parse_err_name(s, _) => "{s}_parse_errors",
            parse_err_desc(_, p) => "Number of {p} that failed to parse",
            handle_err_name(_, p) => "{p}_with_handler_errors",
            handle_err_desc(_, p) => "Number of {p} that threw at least one handler error",
            total_handle_errs_name(s, _) => "{s}_handler_errors",
            total_handle_errs_desc(s, _) => "Number of errors thrown by {s} handlers",
        }

        Self {
            handled: f(handled_name, handled_desc),
            ok: f(ok_name, ok_desc),
            parse_err: f(parse_err_name, parse_err_desc),
            handle_err: f(handle_err_name, handle_err_desc),
            total_handle_errs: f(total_handle_errs_name, total_handle_errs_desc),
        }
    }

    fn inc<'a, F: Fn(&'a C) -> D, D: Counter + 'a>(&'a self, res: JobResult, lens: F) {
        lens(&self.handled).inc();

        match res {
            JobResult::Ok => lens(&self.ok).inc(),
            JobResult::ParseErr => lens(&self.parse_err).inc(),
            JobResult::HandleErr(n) => {
                lens(&self.handle_err).inc();

                lens(&self.total_handle_errs).inc_by(n.try_into().unwrap_or_default());
            },
        }
    }
}

// TODO: this should probably use datapoint attributes rather than weird name formatting
pub(crate) struct Counters<B: Instrumenter> {
    updates_recvd: UpdateCounters<B>,
    update_results: ResultCounters<UpdateCounters<B>>,
}

impl<B: Instrumenter> Counters<B> {
    pub fn new(metrics: &B) -> Self {
        Self {
            updates_recvd: UpdateCounters::new(|Noun(_, p)| {
                metrics.make_counter(
                    format!("{p}_received"),
                    format!("Number of {p} received for processing"),
                )
            }),
            update_results: ResultCounters::new(|c, d| {
                UpdateCounters::new(|n| metrics.make_counter(c(n), d(n)))
            }),
        }
    }
}

impl<B: Instrumenter> fmt::Debug for Counters<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("Counters").finish() }
}

impl<B: Instrumenter> Counters<B> {
    pub fn inc_received(&self, ty: UpdateType) { self.updates_recvd.get(ty).inc(); }

    #[inline]
    pub fn inc_processed(&self, ty: UpdateType, res: JobResult) {
        self.update_results.inc(res, |u| u.get(ty));
    }
}

const INSTRUCTION_NOUN: Noun = Noun("instruction", "instructions");

pub(crate) struct InstructionCounters<B: Instrumenter> {
    result: ResultCounters<B::Counter>,
}

impl<B: Instrumenter> fmt::Debug for InstructionCounters<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InstructionCounters").finish()
    }
}

impl<B: Instrumenter> InstructionCounters<B> {
    pub fn new(metrics: &B) -> Self {
        Self {
            result: ResultCounters::new(|c, d| {
                metrics.make_counter(c(INSTRUCTION_NOUN), d(INSTRUCTION_NOUN))
            }),
        }
    }

    #[inline]
    pub fn inc_processed(&self, res: JobResult) { self.result.inc(res, std::convert::identity); }
}
