//! Helper types for parsing and dispatching instructions from transaction
//! updates.

use std::fmt::{self, Debug};

use vixen_core::{instruction::InstructionUpdate, GetPrefilter, ParserId, TransactionUpdate};

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
        let ixs = InstructionUpdate::build_from_txn(txn).map_err(PipelineErrors::parse)?;
        // TODO: how should sub-pipeline delegation be handled for instruction trees?
        for insn in ixs.iter().flat_map(|i| i.visit_all()) {
            for pipe in &*self.0 {
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
        let mut err = None;
        let ixs = InstructionUpdate::build_from_txn(txn).map_err(PipelineErrors::parse)?;
        let pipe = &self.0;

        for insn in ixs.iter().flat_map(|i| i.visit_all()) {
            let res = pipe.handle(insn).await;

            #[cfg(feature = "prometheus")]
            metrics::increment_processed_updates(&res, metrics::UpdateType::Instruction);

            match res {
                Ok(()) => (),
                Err(PipelineErrors::AlreadyHandled(h)) => h.as_unit(),
                Err(e) => err = Some(e.handle::<InstructionUpdate>(&pipe.id())),
            }
        }

        if let Some(h) = err {
            Err(PipelineErrors::AlreadyHandled(h))
        } else {
            Ok(())
        }
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

#[cfg(test)]
mod tests {
    use std::{
        borrow::Cow,
        sync::atomic::{AtomicUsize, Ordering},
    };

    use vixen_core::{
        instruction::InstructionUpdate, ParseError, ParseResult, Parser, Prefilter,
        TransactionUpdate,
    };
    use yellowstone_grpc_proto::{
        geyser::SubscribeUpdateTransactionInfo,
        solana::storage::confirmed_block::{
            CompiledInstruction, Message, MessageHeader, Transaction, TransactionStatusMeta,
        },
    };

    use super::{SingleInstructionPipeline, PipelineErrors};
    use crate::handler::{BoxPipeline, DynPipeline, Handler, HandlerResult, Pipeline};

    static HANDLED: AtomicUsize = AtomicUsize::new(0);

    #[derive(Debug, Clone, Copy)]
    struct FailFirstIxParser;

    impl Parser for FailFirstIxParser {
        type Input = InstructionUpdate;
        type Output = u8;

        fn id(&self) -> Cow<'static, str> { "test::FailFirstIxParser".into() }

        fn prefilter(&self) -> Prefilter { Prefilter::default() }

        async fn parse(&self, value: &Self::Input) -> ParseResult<Self::Output> {
            if value.data.first() == Some(&0) {
                return Err(ParseError::Other("simulated unpack failure".into()));
            }

            Ok(1)
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct CountingHandler;

    impl Handler<u8, InstructionUpdate> for CountingHandler {
        async fn handle(&self, _value: &u8, _raw: &InstructionUpdate) -> HandlerResult<()> {
            HANDLED.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    fn sample_transaction(instructions: Vec<CompiledInstruction>) -> TransactionUpdate {
        let program = [7u8; 32];
        let payer = [8u8; 32];
        let mint = [9u8; 32];

        TransactionUpdate {
            slot: 1,
            transaction: Some(SubscribeUpdateTransactionInfo {
                signature: vec![1; 64],
                is_vote: false,
                index: 0,
                transaction: Some(Transaction {
                    signatures: vec![vec![1; 64]],
                    message: Some(Message {
                        header: Some(MessageHeader {
                            num_required_signatures: 1,
                            num_readonly_signed_accounts: 0,
                            num_readonly_unsigned_accounts: 0,
                        }),
                        account_keys: vec![program.to_vec(), payer.to_vec(), mint.to_vec()],
                        recent_blockhash: vec![2; 32],
                        instructions,
                        versioned: false,
                        address_table_lookups: vec![],
                    }),
                }),
                meta: Some(TransactionStatusMeta {
                    err: None,
                    fee: 0,
                    pre_balances: vec![0; 3],
                    post_balances: vec![0; 3],
                    inner_instructions: vec![],
                    inner_instructions_none: false,
                    log_messages: vec![],
                    log_messages_none: false,
                    pre_token_balances: vec![],
                    post_token_balances: vec![],
                    rewards: vec![],
                    loaded_writable_addresses: vec![],
                    loaded_readonly_addresses: vec![],
                    return_data: None,
                    return_data_none: true,
                    compute_units_consumed: None,
                    cost_units: None,
                }),
            }),
        }
    }

    #[tokio::test]
    async fn single_instruction_pipeline_keeps_iterating_after_parse_error() {
        HANDLED.store(0, Ordering::Relaxed);

        let txn = sample_transaction(vec![
            CompiledInstruction {
                program_id_index: 0,
                accounts: vec![1, 2],
                data: vec![0],
            },
            CompiledInstruction {
                program_id_index: 0,
                accounts: vec![1, 2],
                data: vec![1],
            },
        ]);

        let pipeline: BoxPipeline<'static, InstructionUpdate> =
            Box::new(Pipeline::new(FailFirstIxParser, [CountingHandler]));
        let single = SingleInstructionPipeline::new(pipeline);

        let result = single.handle(&txn).await;

        assert!(matches!(result, Err(PipelineErrors::AlreadyHandled(_))));
        assert_eq!(
            HANDLED.load(Ordering::Relaxed),
            1,
            "handler should still run for instructions after a parse failure"
        );
    }
}
