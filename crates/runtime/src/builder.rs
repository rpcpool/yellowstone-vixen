use tracing::error;
use vixen_core::{AccountUpdate, TransactionUpdate};

use crate::{
    config::{MaybeDefault, VixenConfig},
    handler::BoxPipeline,
    metrics::{Counters, Metrics, MetricsFactory, NullMetrics},
    util, DynPipeline, PipelineSets, Runtime,
};

pub trait BuilderKind: Default {
    type Error: std::error::Error;
}

#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("ID collision detected among account pipelines")]
    AccountPipelineCollision,
    #[error("ID collision detected among transaction pipelines")]
    TransactionPipelineCollision,
    #[error("Missing field {0:?}")]
    MissingField(&'static str),
    #[error("Missing config section {0:?}")]
    MissingConfig(&'static str),
    #[error("Error instantiating metrics backend")]
    Metrics(#[source] Box<dyn std::error::Error>),
}

#[derive(Debug)]
#[must_use = "Consider calling .build() on this builder"]
pub struct Builder<K: BuilderKind, M> {
    pub(crate) err: Result<(), K::Error>,
    pub(crate) account: Vec<BoxPipeline<'static, AccountUpdate>>,
    pub(crate) transaction: Vec<BoxPipeline<'static, TransactionUpdate>>,
    pub(crate) metrics: M,
    pub(crate) extra: K,
}

impl<K: BuilderKind> Default for Builder<K, NullMetrics> {
    fn default() -> Self {
        Self {
            err: Ok(()),
            account: vec![],
            transaction: vec![],
            metrics: NullMetrics,
            extra: K::default(),
        }
    }
}

// #[inline]
// pub(crate) fn unwrap<T>(name: &'static str, val: Option<T>) -> Result<T, BuilderError> {
//     val.ok_or(BuilderError::MissingField(name))
// }

#[inline]
pub(crate) fn unwrap_cfg<T>(name: &'static str, val: Option<T>) -> Result<T, BuilderError> {
    val.ok_or(BuilderError::MissingConfig(name))
}

impl<K: BuilderKind, M> Builder<K, M> {
    #[inline]
    pub(crate) fn mutate(self, mutate: impl FnOnce(&mut Self)) -> Self {
        self.try_mutate(|s| {
            mutate(s);
            Ok(())
        })
    }

    #[inline]
    pub(crate) fn try_mutate(
        mut self,
        mutate: impl FnOnce(&mut Self) -> Result<(), K::Error>,
    ) -> Self {
        if let Ok(()) = self.err {
            self.err = mutate(&mut self);
        }
        self
    }

    pub fn metrics<N>(self, metrics: N) -> Builder<K, N> {
        let Self {
            err,
            account,
            transaction,
            metrics: _,
            extra,
        } = self;

        Builder {
            err,
            account,
            transaction,
            metrics,
            extra,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RuntimeKind;
pub type RuntimeBuilder<M> = Builder<RuntimeKind, M>;

impl BuilderKind for RuntimeKind {
    type Error = BuilderError;
}

impl<M: MetricsFactory> RuntimeBuilder<M> {
    pub fn account<A: DynPipeline<AccountUpdate> + Send + Sync + 'static>(
        self,
        account: A,
    ) -> Self {
        self.mutate(|s| s.account.push(Box::new(account)))
    }

    pub fn transaction<T: DynPipeline<TransactionUpdate> + Send + Sync + 'static>(
        self,
        transaction: T,
    ) -> Self {
        self.mutate(|s| s.transaction.push(Box::new(transaction)))
    }

    pub fn try_build(self, config: VixenConfig<M::Config>) -> Result<Runtime<M>, BuilderError> {
        let Self {
            err,
            account,
            transaction,
            metrics,
            extra: RuntimeKind,
        } = self;
        let () = err?;

        let VixenConfig {
            yellowstone: yellowstone_cfg,
            buffer: buffer_cfg,
            metrics: metrics_cfg,
        } = config;

        let metrics_cfg = unwrap_cfg(
            "metrics",
            metrics_cfg.opt().or_else(MaybeDefault::default_opt),
        )?;

        let Metrics(instrumenter, exporter) = metrics
            .create(metrics_cfg, "vixen")
            .map_err(|e| BuilderError::Metrics(e.into()))?;

        let account_len = account.len();
        let transaction_len = transaction.len();

        let pipelines = PipelineSets {
            account: account.into_iter().collect(),
            transaction: transaction.into_iter().collect(),
        };

        if pipelines.account.len() != account_len {
            return Err(BuilderError::AccountPipelineCollision);
        }

        if pipelines.transaction.len() != transaction_len {
            return Err(BuilderError::TransactionPipelineCollision);
        }

        Ok(Runtime {
            yellowstone_cfg,
            buffer_cfg,
            pipelines,
            counters: Counters::new(&instrumenter),
            exporter,
        })
    }

    #[inline]
    pub fn build(self, config: VixenConfig<M::Config>) -> Runtime<M> {
        util::handle_fatal_msg(self.try_build(config), "Error building Vixen runtime")
    }
}
