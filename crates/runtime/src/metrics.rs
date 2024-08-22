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
use yellowstone_vixen_core::UpdateType;

use crate::{
    config::{MaybeDefault, NullConfig},
    handler::HandlerPackErrors,
    stop::{StopCode, StopRx},
};

#[derive(Debug)]
pub struct Metrics<F: MetricsFactory + ?Sized>(pub F::Instrumenter, pub Option<F::Exporter>);

type FactoryResult<F> = Result<Metrics<F>, <F as MetricsFactory>::Error>;

pub trait MetricsFactory {
    type Config: clap::Args + for<'de> serde::Deserialize<'de> + MaybeDefault;
    type Instrumenter: Instrumenter;
    type Exporter: Exporter;
    type Error: Error + Send + Sync + 'static;

    fn create(self, config: Self::Config, id: &'static str) -> FactoryResult<Self>;
}

pub trait Instrumenter: 'static {
    type Counter: Counter;

    fn make_counter(
        &self,
        name: impl Into<Cow<'static, str>>,
        desc: impl Into<Cow<'static, str>>,
    ) -> Self::Counter;
}

pub trait Exporter {
    type Error: Error + Send + Sync + 'static;

    fn run(
        self,
        stop: StopRx,
    ) -> impl Future<Output = Result<StopCode, Self::Error>> + Send + 'static;
}

pub trait Counter: Send + Sync {
    #[inline]
    fn inc(&self) { self.inc_by(1); }

    fn inc_by(&self, by: u64);
}

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
        global,
        metrics::{Counter, Meter},
    };

    use super::{FactoryResult, Metrics};
    use crate::config::NullConfig;

    #[derive(Debug, Clone, Copy)]
    pub struct OpenTelemetry;

    impl super::MetricsFactory for OpenTelemetry {
        type Config = NullConfig;
        type Error = Infallible;
        type Exporter = Infallible;
        type Instrumenter = Meter;

        fn create(self, NullConfig: Self::Config, id: &'static str) -> FactoryResult<Self> {
            Ok(Metrics(global::meter(id), None))
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
    pub fn from_pack<R: Borrow<Result<U, HandlerPackErrors>>, U>(res: R) -> Self {
        match res.borrow() {
            Ok(_) => Self::Ok,
            Err(HandlerPackErrors::Parse(_)) => Self::ParseErr,
            Err(HandlerPackErrors::Handlers(v)) => Self::HandleErr(v.len()),
        }
    }
}

pub(crate) struct Counters<B: Instrumenter> {
    pub accts_recvd: B::Counter,
    pub txns_recvd: B::Counter,
    pub accts_handled: B::Counter,
    pub txns_handled: B::Counter,
    pub accts_ok: B::Counter,
    pub txns_ok: B::Counter,
    pub acct_parse_errs: B::Counter,
    pub txn_parse_errs: B::Counter,
    pub acct_handle_errs: B::Counter,
    pub txn_handle_errs: B::Counter,
    pub uniq_acct_handle_errs: B::Counter,
    pub uniq_txn_handle_errs: B::Counter,
}

impl<B: Instrumenter> Counters<B> {
    pub(crate) fn new(metrics: &B) -> Self {
        Self {
            accts_recvd: metrics.make_counter(
                "accounts_received",
                "Number of accounts received for processing",
            ),
            txns_recvd: metrics.make_counter(
                "transactions_received",
                "Number of transactions received for processing",
            ),
            accts_handled: metrics.make_counter(
                "accounts_processed",
                "Number of (successfully or unsuccessfully) processed accounts",
            ),
            txns_handled: metrics.make_counter(
                "transactions_processed",
                "Number of (successfully or unsuccessfully) processed transactions",
            ),
            accts_ok: metrics.make_counter(
                "successful_accounts",
                "Number of successfully processed accounts",
            ),
            txns_ok: metrics.make_counter(
                "successful_transactions",
                "Number of successfully processed transactions",
            ),
            acct_parse_errs: metrics.make_counter(
                "account_parse_errors",
                "Number of accounts that failed to parse",
            ),
            txn_parse_errs: metrics.make_counter(
                "transaction_parse_errors",
                "Number of transactions that failed to parse",
            ),
            acct_handle_errs: metrics.make_counter(
                "account_handler_errors",
                "Number of errors thrown by account handlers",
            ),
            txn_handle_errs: metrics.make_counter(
                "transaction_handler_errors",
                "Number of errors thrown by transaction handlers",
            ),
            uniq_acct_handle_errs: metrics.make_counter(
                "accounts_with_handler_errors",
                "Number of accounts that threw at least one handler error",
            ),
            uniq_txn_handle_errs: metrics.make_counter(
                "transactions_with_handler_errors",
                "Number of transactions that threw at least one handler error",
            ),
        }
    }
}

impl<B: Instrumenter> fmt::Debug for Counters<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("Counters").finish() }
}

impl<B: Instrumenter> Counters<B> {
    pub fn inc_received(&self, ty: UpdateType) {
        match ty {
            UpdateType::Account => &self.accts_recvd,
            UpdateType::Transaction => &self.txns_recvd,
        }
        .inc();
    }

    pub fn inc_processed(&self, ty: UpdateType, res: JobResult) {
        match ty {
            UpdateType::Account => &self.accts_handled,
            UpdateType::Transaction => &self.txns_handled,
        }
        .inc();

        match res {
            JobResult::Ok => match ty {
                UpdateType::Account => &self.accts_ok,
                UpdateType::Transaction => &self.txns_ok,
            }
            .inc(),
            JobResult::ParseErr => match ty {
                UpdateType::Account => &self.acct_parse_errs,
                UpdateType::Transaction => &self.txn_parse_errs,
            }
            .inc(),
            JobResult::HandleErr(n) => {
                match ty {
                    UpdateType::Account => &self.uniq_acct_handle_errs,
                    UpdateType::Transaction => &self.uniq_txn_handle_errs,
                }
                .inc();

                match ty {
                    UpdateType::Account => &self.acct_handle_errs,
                    UpdateType::Transaction => &self.txn_handle_errs,
                }
                .inc_by(n.try_into().unwrap_or_default());
            },
        }
    }
}
