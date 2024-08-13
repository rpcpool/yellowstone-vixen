use std::{
    borrow::{Borrow, Cow},
    fmt,
};

use yellowstone_vixen_core::UpdateType;

use crate::handler::HandlerPackErrors;

pub trait MetricsFactory {
    type Output: MetricsBackend;
    type Error: std::error::Error;

    fn create() -> Result<Self::Output, Self::Error>;
}

pub trait MetricsBackend: 'static {
    type Counter: Counter;

    fn make_counter(
        &self,
        name: impl Into<Cow<'static, str>>,
        desc: impl Into<Cow<'static, str>>,
    ) -> Self::Counter;
}

pub trait Counter: Send + Sync {
    #[inline]
    fn inc(&self) { self.inc_by(1); }

    fn inc_by(&self, by: u64);
}

#[derive(Debug, Default, Clone, Copy)]
pub struct NullMetrics;

impl MetricsBackend for NullMetrics {
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

#[cfg(feature = "opentelemetry")]
impl MetricsBackend for opentelemetry::metrics::Meter {
    type Counter = opentelemetry::metrics::Counter<u64>;

    fn make_counter(
        &self,
        name: impl Into<Cow<'static, str>>,
        desc: impl Into<Cow<'static, str>>,
    ) -> Self::Counter {
        self.u64_counter(name).with_description(desc).init()
    }
}

#[cfg(feature = "opentelemetry")]
impl Counter for opentelemetry::metrics::Counter<u64> {
    fn inc_by(&self, by: u64) { self.add(by, &[]); }
}

#[cfg(feature = "prometheus")]
pub struct Prometheus;

#[cfg(feature = "prometheus")]
impl MetricsBackend for Prometheus {
    type Counter = prometheus::IntCounter;

    fn make_counter(
        &self,
        name: impl Into<Cow<'static, str>>,
        desc: impl Into<Cow<'static, str>>,
    ) -> Self::Counter {
        prometheus::IntCounter::with_opts(prometheus::Opts::new(name.into(), desc.into())).unwrap()
    }
}

#[cfg(feature = "prometheus")]
impl Counter for prometheus::IntCounter {
    fn inc_by(&self, by: u64) { prometheus::IntCounter::inc_by(self, by) }
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

pub(crate) struct Metrics<B: MetricsBackend> {
    pub accts_recvd: B::Counter,
    pub txns_recvd: B::Counter,
    pub ixs_recvd: B::Counter,
    pub accts_handled: B::Counter,
    pub txns_handled: B::Counter,
    pub ixs_handled: B::Counter,
    pub accts_ok: B::Counter,
    pub txns_ok: B::Counter,
    pub ixs_ok: B::Counter,
    pub acct_parse_errs: B::Counter,
    pub txn_parse_errs: B::Counter,
    pub ix_parse_errs: B::Counter,
    pub acct_handle_errs: B::Counter,
    pub txn_handle_errs: B::Counter,
    pub ix_handle_errs: B::Counter,
    pub uniq_acct_handle_errs: B::Counter,
    pub uniq_txn_handle_errs: B::Counter,
    pub uniq_ix_handle_errs: B::Counter,
}

impl<B: MetricsBackend> Metrics<B> {
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
            ixs_recvd: metrics.make_counter(
                "instructions_received",
                "Number of instructions received for processing",
            ),
            accts_handled: metrics.make_counter(
                "accounts_processed",
                "Number of (successfully or unsuccessfully) processed accounts",
            ),
            txns_handled: metrics.make_counter(
                "transactions_processed",
                "Number of (successfully or unsuccessfully) processed transactions",
            ),
            ixs_handled: metrics.make_counter(
                "instructions_processed",
                "Number of (successfully or unsuccessfully) processed instructions",
            ),
            accts_ok: metrics.make_counter(
                "successful_accounts",
                "Number of successfully processed accounts",
            ),
            txns_ok: metrics.make_counter(
                "successful_transactions",
                "Number of successfully processed transactions",
            ),
            ixs_ok: metrics.make_counter(
                "successful_instructions",
                "Number of successfully processed instructions",
            ),
            acct_parse_errs: metrics.make_counter(
                "account_parse_errors",
                "Number of accounts that failed to parse",
            ),
            txn_parse_errs: metrics.make_counter(
                "transaction_parse_errors",
                "Number of transactions that failed to parse",
            ),
            ix_parse_errs: metrics.make_counter(
                "instruction_parse_errors",
                "Number of instructions that failed to parse",
            ),
            acct_handle_errs: metrics.make_counter(
                "account_handler_errors",
                "Number of errors thrown by account handlers",
            ),
            txn_handle_errs: metrics.make_counter(
                "transaction_handler_errors",
                "Number of errors thrown by transaction handlers",
            ),
            ix_handle_errs: metrics.make_counter(
                "instruction_handler_errors",
                "Number of errors thrown by instruction handlers",
            ),
            uniq_acct_handle_errs: metrics.make_counter(
                "accounts_with_handler_errors",
                "Number of accounts that threw at least one handler error",
            ),
            uniq_txn_handle_errs: metrics.make_counter(
                "transactions_with_handler_errors",
                "Number of transactions that threw at least one handler error",
            ),
            uniq_ix_handle_errs: metrics.make_counter(
                "instructions_with_handler_errors",
                "Number of instructions that threw at least one handler error",
            ),
        }
    }
}

impl<B: MetricsBackend> fmt::Debug for Metrics<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("Metrics").finish() }
}

impl<B: MetricsBackend> Metrics<B> {
    pub fn inc_received(&self, ty: UpdateType) {
        match ty {
            UpdateType::Account => &self.accts_recvd,
            UpdateType::Transaction => &self.txns_recvd,
            UpdateType::Instruction => &self.ixs_recvd,
        }
        .inc();
    }

    pub fn inc_processed(&self, ty: UpdateType, res: JobResult) {
        match ty {
            UpdateType::Account => &self.accts_handled,
            UpdateType::Transaction => &self.txns_handled,
            UpdateType::Instruction => &self.ixs_handled,
        }
        .inc();

        match res {
            JobResult::Ok => match ty {
                UpdateType::Account => &self.accts_ok,
                UpdateType::Transaction => &self.txns_ok,
                UpdateType::Instruction => &self.ixs_ok,
            }
            .inc(),
            JobResult::ParseErr => match ty {
                UpdateType::Account => &self.acct_parse_errs,
                UpdateType::Transaction => &self.txn_parse_errs,
                UpdateType::Instruction => &self.ix_parse_errs,
            }
            .inc(),
            JobResult::HandleErr(n) => {
                match ty {
                    UpdateType::Account => &self.uniq_acct_handle_errs,
                    UpdateType::Transaction => &self.uniq_txn_handle_errs,
                    UpdateType::Instruction => &self.uniq_ix_handle_errs,
                }
                .inc();

                match ty {
                    UpdateType::Account => &self.acct_handle_errs,
                    UpdateType::Transaction => &self.txn_handle_errs,
                    UpdateType::Instruction => &self.ix_handle_errs,
                }
                .inc_by(n.try_into().unwrap_or_default());
            },
        }
    }
}
