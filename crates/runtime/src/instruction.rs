//! Helper types for parsing and dispatching instructions from transaction
//! updates.

use std::fmt;

use vixen_core::{instruction::InstructionUpdate, GetPrefilter, ParserId, TransactionUpdate};

use crate::{
    handler::{BoxPipeline, DynPipeline, PipelineErrors},
    metrics::{InstructionCounters, Instrumenter, JobResult},
};

/// A pipeline for dispatching instruction updates given a transaction update.
pub struct InstructionPipeline<M: Instrumenter>(
    Box<[BoxPipeline<'static, InstructionUpdate>]>,
    InstructionCounters<M>,
);

impl<M: Instrumenter> fmt::Debug for InstructionPipeline<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("InstructionPipeline")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

impl<M: Instrumenter> InstructionPipeline<M> {
    /// Create a new instruction pipeline from a list of sub-pipelines.
    #[must_use]
    pub fn new(
        pipelines: Vec<BoxPipeline<'static, InstructionUpdate>>,
        instrumenter: &M,
    ) -> Option<Self> {
        if pipelines.is_empty() {
            return None;
        }

        Some(Self(
            pipelines.into_boxed_slice(),
            InstructionCounters::new(instrumenter),
        ))
    }

    /// Handle a transaction update by dispatching its instruction updates to
    /// the sub-pipelines.
    ///
    /// # Errors
    /// Returns an error if any of the sub-pipelines return an error.
    pub async fn handle(&self, txn: &TransactionUpdate) -> Result<(), PipelineErrors> {
        let mut err = None;
        let ixs = InstructionUpdate::parse_from_txn(txn).map_err(PipelineErrors::parse)?;
        // TODO: how should sub-pipeline delegation be handled for instruction trees?
        for insn in ixs.iter().flat_map(|i| i.visit_all()) {
            for pipe in &*self.0 {
                // TODO: run these concurrently?
                let res = pipe.handle(insn).await;
                if let Some(r) = JobResult::from_pipeline(&res) {
                    self.1.inc_processed(r);
                }
                match res {
                    Ok(()) => (),
                    Err(PipelineErrors::AlreadyHandled(h)) => h.as_unit(),
                    Err(e) => err = Some(e.handle::<InstructionUpdate>(&pipe.id())),
                }
            }
        }

        if let Some(h) = err {
            Err(PipelineErrors::AlreadyHandled(h))
        } else {
            Ok(())
        }
    }
}

impl<M: Instrumenter> ParserId for InstructionPipeline<M> {
    fn id(&self) -> std::borrow::Cow<str> {
        "yellowstone_vixen::instruction::InstructionPipeline".into()
    }
}

impl<M: Instrumenter> GetPrefilter for InstructionPipeline<M> {
    fn prefilter(&self) -> vixen_core::Prefilter {
        self.0.iter().map(GetPrefilter::prefilter).collect()
    }
}

impl<M: Instrumenter> DynPipeline<TransactionUpdate> for InstructionPipeline<M> {
    fn handle<'h>(
        &'h self,
        value: &'h TransactionUpdate,
    ) -> std::pin::Pin<Box<dyn futures_util::Future<Output = Result<(), PipelineErrors>> + Send + 'h>>
    {
        Box::pin(InstructionPipeline::handle(self, value))
    }
}
