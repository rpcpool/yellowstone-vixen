//! Helper types for parsing and dispatching instructions from transaction
//! updates.

use std::fmt::{self, Debug};

use tracing::trace;
use vixen_core::{
    instruction::{InstructionUpdate, TreeStep},
    GetPrefilter, ParserId, TransactionUpdate,
};

use crate::handler::{BoxPipeline, DynPipeline, PipelineErrors};
#[cfg(feature = "prometheus")]
use crate::metrics;

/// A pipeline for dispatching instruction updates given a transaction update.
pub struct InstructionPipeline(Box<[BoxPipeline<'static, InstructionUpdate>]>);

impl fmt::Debug for InstructionPipeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("InstructionPipeline").field(&self.0).finish()
    }
}

impl InstructionPipeline {
    /// Create a new instruction pipeline from a list of sub-pipelines.
    #[must_use]
    pub fn new(pipelines: Vec<BoxPipeline<'static, InstructionUpdate>>) -> Option<Self> {
        if pipelines.is_empty() {
            return None;
        }

        Some(Self(pipelines.into_boxed_slice()))
    }

    /// Handle a transaction update by dispatching its instruction updates to
    /// the sub-pipelines.
    ///
    /// # Errors
    /// Returns an error if any of the sub-pipelines return an error.
    pub async fn handle(&self, txn: &TransactionUpdate) -> Result<(), PipelineErrors> {
        let mut err = None;
        let ixs = InstructionUpdate::parse_from_txn(txn).map_err(PipelineErrors::parse)?;

        for pipe in &*self.0 {
            pipe.handle_tx_start(txn).await;
        }

        // TODO: how should sub-pipeline delegation be handled for instruction trees?
        for node in ixs.iter().flat_map(|i| i.visit_tree()) {
            for pipe in &*self.0 {
                let insn = match node {
                    TreeStep::EnterCpiCallFromNode {
                        ref caller_cpi_path,
                    } => {
                        pipe.handle_cpi_enter(caller_cpi_path).await;
                        continue;
                    },
                    TreeStep::ReturnFromCpiCallsToNode {
                        ref caller_cpi_path,
                    } => {
                        pipe.handle_cpi_return(caller_cpi_path).await;
                        continue;
                    },
                    TreeStep::PhysicalNode(insn) => insn,
                };

                let res = pipe.handle(insn).await;

                #[cfg(feature = "prometheus")]
                metrics::increment_processed_updates(&res, metrics::UpdateType::Instruction);

                match res {
                    Ok(()) => (),
                    Err(PipelineErrors::AlreadyHandled(h)) => h.as_unit(),
                    Err(e) => err = Some(e.handle::<InstructionUpdate>(&pipe.id())),
                }
            }
        }

        for pipe in &*self.0 {
            pipe.handle_tx_end(txn).await;
        }

        if let Some(h) = err {
            Err(PipelineErrors::AlreadyHandled(h))
        } else {
            Ok(())
        }
    }
}

impl ParserId for InstructionPipeline {
    fn id(&self) -> std::borrow::Cow<'static, str> { "InstructionPipeline".into() }
}

impl GetPrefilter for InstructionPipeline {
    fn prefilter(&self) -> vixen_core::Prefilter {
        self.0.iter().map(GetPrefilter::prefilter).collect()
    }
}

impl DynPipeline<TransactionUpdate> for InstructionPipeline {
    fn handle<'h>(
        &'h self,
        value: &'h TransactionUpdate,
    ) -> std::pin::Pin<Box<dyn futures_util::Future<Output = Result<(), PipelineErrors>> + Send + 'h>>
    {
        Box::pin(InstructionPipeline::handle(self, value))
    }
}

/// A pipeline for dispatching instruction updates for a single parser given a transaction update.
pub struct SingleInstructionPipeline(BoxPipeline<'static, InstructionUpdate>);

impl SingleInstructionPipeline {
    /// Create a new instruction pipeline from a single sub-pipeline.
    #[must_use]
    pub fn new(pipeline: BoxPipeline<'static, InstructionUpdate>) -> Self { Self(pipeline) }

    /// Handle a transaction update by dispatching its instruction updates to
    /// its sub-pipeline.
    ///
    /// # Errors
    /// Returns an error if the inner pipeline fails.
    pub async fn handle(&self, txn: &TransactionUpdate) -> Result<(), PipelineErrors> {
        let ixs = InstructionUpdate::parse_from_txn(txn).map_err(PipelineErrors::parse)?;
        let pipe = &self.0;
        let mut prev_depth: usize = 0;

        pipe.handle_tx_start(txn).await;

        for mode in ixs.iter().flat_map(|i| i.visit_tree()) {
            let insn = match mode {
                TreeStep::EnterCpiCallFromNode {
                    ref caller_cpi_path,
                } => {
                    pipe.handle_cpi_enter(caller_cpi_path).await;
                    continue;
                },
                TreeStep::ReturnFromCpiCallsToNode {
                    ref caller_cpi_path,
                } => {
                    pipe.handle_cpi_return(caller_cpi_path).await;
                    continue;
                },
                TreeStep::PhysicalNode(insn) => insn,
            };

            let depth = insn.path.len();

            if depth < prev_depth {
                trace!(
                    from_depth = prev_depth,
                    to_depth = depth,
                    path = ?insn.path,
                    "Returning from CPI nesting"
                );
            }

            prev_depth = depth;

            let res = pipe.handle(insn).await;

            #[cfg(feature = "prometheus")]
            metrics::increment_processed_updates(&res, metrics::UpdateType::Instruction);

            match res {
                Ok(()) => (),
                Err(PipelineErrors::AlreadyHandled(h)) => h.as_unit(),
                Err(e) => {
                    let handled = e.handle::<InstructionUpdate>(&pipe.id());

                    return Err(PipelineErrors::AlreadyHandled(handled));
                },
            }
        }

        pipe.handle_tx_end(txn).await;

        Ok(())
    }
}

impl ParserId for SingleInstructionPipeline {
    fn id(&self) -> std::borrow::Cow<'static, str> { self.0.id() }
}

impl GetPrefilter for SingleInstructionPipeline {
    fn prefilter(&self) -> vixen_core::Prefilter { self.0.prefilter() }
}

impl Debug for SingleInstructionPipeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SingleInstructionPipeline")
            .field(&self.0)
            .finish()
    }
}

impl DynPipeline<TransactionUpdate> for SingleInstructionPipeline {
    fn handle<'h>(
        &'h self,
        value: &'h TransactionUpdate,
    ) -> std::pin::Pin<Box<dyn futures_util::Future<Output = Result<(), PipelineErrors>> + Send + 'h>>
    {
        Box::pin(SingleInstructionPipeline::handle(self, value))
    }
}
