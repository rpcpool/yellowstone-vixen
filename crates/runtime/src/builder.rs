//! Builder types for the Vixen runtime and stream server.

use tracing::error;
use vixen_core::{
    instruction::InstructionUpdate, AccountUpdate, BlockMetaUpdate, TransactionUpdate,
};
use yellowstone_grpc_proto::geyser::CommitmentLevel;

use crate::{
    config::VixenConfig,
    handler::{BoxPipeline, DynPipeline, PipelineSet, PipelineSets},
    instruction::SingleInstructionPipeline,
    sources::Source,
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
    /// Two block meta pipelines were registered with the same parser ID.
    #[error("ID collision detected among block meta pipelines")]
    BlockMetaCollision,
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
pub struct Builder<K: BuilderKind> {
    pub(crate) err: Result<(), K::Error>,
    pub(crate) account: Vec<BoxPipeline<'static, AccountUpdate>>,
    pub(crate) transaction: Vec<BoxPipeline<'static, TransactionUpdate>>,
    pub(crate) instruction: Vec<BoxPipeline<'static, InstructionUpdate>>,
    pub(crate) block_meta: Vec<BoxPipeline<'static, BlockMetaUpdate>>,
    pub(crate) commitment_level: Option<CommitmentLevel>,
    pub(crate) from_slot_filter: Option<u64>,
    pub(crate) sources: Vec<Box<dyn Source>>,
    pub(crate) metrics_registry: Option<prometheus::Registry>,
    pub(crate) extra: K,
}

impl<K: BuilderKind> Default for Builder<K> {
    fn default() -> Self {
        Self {
            err: Ok(()),
            account: vec![],
            transaction: vec![],
            instruction: vec![],
            block_meta: vec![],
            commitment_level: None,
            from_slot_filter: None,
            metrics_registry: None,
            sources: vec![],
            extra: K::default(),
        }
    }
}

impl<K: BuilderKind> Builder<K> {
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

    /// Sets the metrics registry for the runtime.
    pub fn metrics(self, metrics_registry: prometheus::Registry) -> Builder<K> {
        let Self {
            err,
            account,
            transaction,
            instruction,
            block_meta,
            commitment_level,
            from_slot_filter,
            metrics_registry: _,
            sources,
            extra,
        } = self;

        Builder {
            err,
            account,
            transaction,
            instruction,
            block_meta,
            commitment_level,
            from_slot_filter,
            metrics_registry: Some(metrics_registry),
            sources,
            extra,
        }
    }
}

/// Marker type used for the [`RuntimeBuilder`] type.
#[derive(Debug, Default, Clone, Copy)]
pub struct RuntimeKind;
/// A builder for the [`Runtime`] type.
pub type RuntimeBuilder = Builder<RuntimeKind>;

impl BuilderKind for RuntimeKind {
    type Error = BuilderError;
}

impl RuntimeBuilder {
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

    /// Add a new block metad pipeline to the builder.
    pub fn block_meta<T: DynPipeline<BlockMetaUpdate> + Send + Sync + 'static>(
        self,
        block_meta: T,
    ) -> Self {
        self.mutate(|s| s.block_meta.push(Box::new(block_meta)))
    }

    /// Set the confirmation level for the Yellowstone client.
    pub fn commitment_level(self, commitment_level: CommitmentLevel) -> Self {
        self.mutate(|s| s.commitment_level = Some(commitment_level))
    }

    /// Add a new data `Source` to which the runtime will subscribe.
    ///
    /// **NOTE:** All added Sources are going to be processed concurrently
    pub fn source<T: Source>(self, source: T) -> Self {
        self.mutate(|s| s.sources.push(Box::new(source)))
    }

    /// Set the from slot filter for the Yellowstone client. The server will attempt to replay
    /// messages from the specified slot onward
    pub fn from_slot(self, from_slot: u64) -> Self {
        self.mutate(|s| s.from_slot_filter = Some(from_slot))
    }

    /// Attempt to build a new [`Runtime`] instance from the current builder
    /// state and the provided configuration.
    ///
    /// # Errors
    /// This function returns an error if the builder or configuration are
    /// invalid.
    pub fn try_build(self, config: VixenConfig) -> Result<Runtime, BuilderError> {
        let Self {
            err,
            account,
            transaction,
            instruction,
            block_meta,
            commitment_level,
            from_slot_filter,
            metrics_registry,
            sources,
            extra: RuntimeKind,
        } = self;
        let () = err?;

        let VixenConfig {
            yellowstone: yellowstone_cfg,
            buffer: buffer_cfg,
        } = config;

        let mut ixs = PipelineSet::new();

        for ix in instruction {
            let id = ix.id().into_owned();
            let pre_existent_parser = ixs.insert(
                id.clone(),
                Box::new(SingleInstructionPipeline::new(ix))
                    as BoxPipeline<'static, TransactionUpdate>,
            );

            if pre_existent_parser.is_some() {
                tracing::warn!("Duplicate parser ID detected: {}", id);
            }
        }

        let account_len = account.len();
        let transaction_len = transaction.len();
        let block_meta_len = block_meta.len();

        let pipelines = PipelineSets {
            account: account.into_iter().collect(),
            transaction: transaction.into_iter().collect(),
            instruction: ixs,
            block_meta: block_meta.into_iter().collect(),
        };

        if pipelines.account.len() != account_len {
            return Err(BuilderError::AccountPipelineCollision);
        }

        if pipelines.transaction.len() != transaction_len {
            return Err(BuilderError::TransactionPipelineCollision);
        }

        if pipelines.block_meta.len() != block_meta_len {
            return Err(BuilderError::MissingField("block_meta"));
        }

        if sources.is_empty() {
            return Err(BuilderError::MissingField("sources"));
        }

        Ok(Runtime {
            yellowstone_cfg,
            sources,
            buffer_cfg,
            pipelines,
            commitment_filter: commitment_level,
            from_slot_filter,
            metrics_registry,
        })
    }

    /// Build a new [`Runtime`] instance from the current builder state and the
    /// provided configuration, terminating the current process if an error
    /// occurs.
    #[inline]
    #[must_use]
    pub fn build(self, config: VixenConfig) -> Runtime {
        util::handle_fatal_msg(self.try_build(config), "Error building Vixen runtime")
    }
}
