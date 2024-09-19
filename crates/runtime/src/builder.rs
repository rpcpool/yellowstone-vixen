//! Builder types for the Vixen runtime and stream server.

use tracing::error;
use vixen_core::{instruction::InstructionUpdate, AccountUpdate, TransactionUpdate};

use crate::{
    config::{MaybeDefault, VixenConfig},
    handler::{BoxPipeline, DynPipeline, PipelineSets},
    instruction::InstructionPipeline,
    metrics::{Counters, Metrics, MetricsFactory, NullMetrics},
    util, Runtime,
};

/// Helper trait for defining the intended use for a builder.
pub trait BuilderKind: Default {
    /// The type of error returned by the builder.
    type Error: std::error::Error;
}

/// An error thrown by the Vixen runtime builder.
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    /// Two account pipelines were registered with the same parser ID.
    #[error("ID collision detected among account pipelines")]
    AccountPipelineCollision,
    /// Two transaction pipelines were registered with the same parser ID.
    #[error("ID collision detected among transaction pipelines")]
    TransactionPipelineCollision,
    /// A required field was missing from the builder.
    #[error("Missing field {0:?}")]
    MissingField(&'static str),
    /// A required field or section was missing from the provided configuration.
    #[error("Missing config section {0:?}")]
    MissingConfig(&'static str),
    /// An error occurred while instantiating the metrics backend.
    #[error("Error instantiating metrics backend")]
    Metrics(#[source] Box<dyn std::error::Error>),
}

/// A builder used by both the [`Runtime`] and
/// [`stream::Server`](crate::stream::Server) types.
#[derive(Debug)]
#[must_use = "Consider calling .build() on this builder"]
pub struct Builder<K: BuilderKind, M> {
    pub(crate) err: Result<(), K::Error>,
    pub(crate) account: Vec<BoxPipeline<'static, AccountUpdate>>,
    pub(crate) transaction: Vec<BoxPipeline<'static, TransactionUpdate>>,
    pub(crate) instruction: Vec<BoxPipeline<'static, InstructionUpdate>>,
    pub(crate) metrics: M,
    pub(crate) extra: K,
}

impl<K: BuilderKind> Default for Builder<K, NullMetrics> {
    fn default() -> Self {
        Self {
            err: Ok(()),
            account: vec![],
            transaction: vec![],
            instruction: vec![],
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

    /// Replace the metrics backend currently configured with a new one,
    /// updating the type of the builder in the process.
    pub fn metrics<N>(self, metrics: N) -> Builder<K, N> {
        let Self {
            err,
            account,
            transaction,
            instruction,
            metrics: _,
            extra,
        } = self;

        Builder {
            err,
            account,
            transaction,
            instruction,
            metrics,
            extra,
        }
    }
}

/// Marker type used for the [`RuntimeBuilder`] type.
#[derive(Debug, Default, Clone, Copy)]
pub struct RuntimeKind;
/// A builder for the [`Runtime`] type.
pub type RuntimeBuilder<M = NullMetrics> = Builder<RuntimeKind, M>;

impl BuilderKind for RuntimeKind {
    type Error = BuilderError;
}

impl<M: MetricsFactory> RuntimeBuilder<M> {
    /// Add a new account pipeline to the builder.
    pub fn account<A: DynPipeline<AccountUpdate> + Send + Sync + 'static>(
        self,
        account: A,
    ) -> Self {
        self.mutate(|s| s.account.push(Box::new(account)))
    }

    /// Add a new transaction pipeline to the builder.
    pub fn transaction<T: DynPipeline<TransactionUpdate> + Send + Sync + 'static>(
        self,
        transaction: T,
    ) -> Self {
        self.mutate(|s| s.transaction.push(Box::new(transaction)))
    }

    /// Add a new instruction pipeline to the builder.
    ///
    /// **NOTE:** All registered instruction pipelines will be bundled into a
    /// single [`InstructionPipeline`] instance.
    pub fn instruction<I: DynPipeline<InstructionUpdate> + Send + Sync + 'static>(
        self,
        instruction: I,
    ) -> Self {
        self.mutate(|s| s.instruction.push(Box::new(instruction)))
    }

    /// Attempt to build a new [`Runtime`] instance from the current builder
    /// state and the provided configuration.
    ///
    /// # Errors
    /// This function returns an error if the builder or configuration are
    /// invalid.
    pub fn try_build(self, config: VixenConfig<M::Config>) -> Result<Runtime<M>, BuilderError> {
        let Self {
            err,
            account,
            mut transaction,
            instruction,
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

        if let Some(i) = InstructionPipeline::new(instruction, &instrumenter) {
            transaction.push(Box::new(i));
        }

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

    /// Build a new [`Runtime`] instance from the current builder state and the
    /// provided configuration, terminating the current process if an error
    /// occurs.
    #[inline]
    pub fn build(self, config: VixenConfig<M::Config>) -> Runtime<M> {
        util::handle_fatal_msg(self.try_build(config), "Error building Vixen runtime")
    }
}
