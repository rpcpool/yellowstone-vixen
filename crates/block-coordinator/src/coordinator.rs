use tokio::sync::mpsc;
use yellowstone_block_machine::{
    dragonsmouth::wrapper::BlocksStateMachineWrapper, state_machine::BlockStateMachineOutput,
};
use yellowstone_grpc_proto::geyser::{subscribe_update::UpdateOneof, SubscribeUpdate};

use crate::{
    state::{CoordinatorEvent, CoordinatorState},
    types::{
        BlockMetadata, ColorSlot, ConfirmedSlot, CoordinatorError, CoordinatorInput,
        CoordinatorMessage, DiscardReason,
    },
};

/// Core orchestrator that owns a `BlocksStateMachineWrapper` and delegates
/// ordering/flush decisions to the pure-ish `CoordinatorState`.
pub struct BlockMachineCoordinator<R> {
    wrapper: BlocksStateMachineWrapper,
    state: CoordinatorState<R>,
    input_rx: mpsc::Receiver<CoordinatorInput>,
    parsed_rx: mpsc::Receiver<CoordinatorMessage<R>>,
    output_tx: mpsc::Sender<ConfirmedSlot<R>>,
    /// When false, Gate 1 is disabled by forcing expected_tx_count to 0.
    require_tx_gate: bool,
}

impl<R: Send + 'static> BlockMachineCoordinator<R> {
    pub fn new(
        input_rx: mpsc::Receiver<CoordinatorInput>,
        parsed_rx: mpsc::Receiver<CoordinatorMessage<R>>,
        output_tx: mpsc::Sender<ConfirmedSlot<R>>,
        require_tx_gate: bool,
    ) -> Self {
        Self {
            wrapper: BlocksStateMachineWrapper::default(),
            state: CoordinatorState::default(),
            input_rx,
            parsed_rx,
            output_tx,
            require_tx_gate,
        }
    }

    /// Main event loop. Any CoordinatorError terminates the coordinator.
    pub async fn run(mut self) -> Result<(), CoordinatorError> {
        if !self.require_tx_gate {
            tracing::info!(
                "require_tx_gate=false: tx gate disabled, transaction status stats will not be reported"
            );
        }
        loop {
            let events: Vec<CoordinatorEvent<R>> = tokio::select! {
                Some(input) = self.input_rx.recv() => {
                    match input {
                        CoordinatorInput::GeyserUpdate(update) => {
                            self.convert_geyser_update(&update)
                        }
                        CoordinatorInput::AccountEventSeen { slot } => {
                            vec![CoordinatorEvent::AccountEventSeen { slot }]
                        }
                    }
                }
                Some(msg) = self.parsed_rx.recv() => {
                    Self::convert_parsed_message(msg)
                }
                else => {
                    tracing::warn!("Coordinator channels closed, shutting down");
                    break;
                }
            };

            for event in events {
                self.state.apply(event)?;
            }

            for confirmed in self.state.drain_flushable()? {
                tracing::info!(
                    slot = %ColorSlot(confirmed.slot),
                    tx_count = confirmed.executed_transaction_count,
                    record_count = confirmed.records.len(),
                    parent_slot = confirmed.parent_slot,
                    "Flushing slot"
                );
                self.output_tx
                    .send(confirmed)
                    .await
                    .map_err(|e| CoordinatorError::OutputChannelClosed { slot: e.0.slot })?;
            }
        }
        Ok(())
    }

    /// Boundary layer: feed updates into the wrapper, convert outputs to events.
    fn convert_geyser_update(&mut self, update: &SubscribeUpdate) -> Vec<CoordinatorEvent<R>> {
        let mut events = Vec::new();

        // Guard: validate BlockMeta.blockhash BEFORE feeding to wrapper.
        if let Some(UpdateOneof::BlockMeta(meta)) = &update.update_oneof
            && bs58::decode(&meta.blockhash).into_vec().is_err()
        {
            tracing::warn!(
                slot = meta.slot,
                "BlockMeta has invalid blockhash — skipping"
            );
            return events;
        }

        if self.wrapper.handle_new_geyser_event(update).is_err() {
            // Untracked slot — only BlockMeta rejections produce a discard event.
            if let Some(UpdateOneof::BlockMeta(meta)) = &update.update_oneof {
                events.push(CoordinatorEvent::SlotDiscarded {
                    slot: meta.slot,
                    reason: DiscardReason::Untracked,
                });
            }
            return events;
        }

        while let Some(output) = self.wrapper.pop_next_state_machine_output() {
            match output {
                BlockStateMachineOutput::FrozenBlock(frozen) => {
                    let expected_tx_count = if self.require_tx_gate {
                        frozen
                            .entries
                            .iter()
                            .map(|e| e.executed_txn_count)
                            .sum()
                    } else {
                        0
                    };
                    let metadata = BlockMetadata {
                        parent_slot: frozen.parent_slot,
                        blockhash: frozen.blockhash,
                        expected_tx_count,
                    };
                    events.push(CoordinatorEvent::BlockFrozen {
                        slot: frozen.slot,
                        metadata,
                    });
                },
                BlockStateMachineOutput::SlotStatus(status)
                    if status.commitment
                        == solana_commitment_config::CommitmentLevel::Confirmed =>
                {
                    events.push(CoordinatorEvent::SlotConfirmed { slot: status.slot });
                },
                BlockStateMachineOutput::SlotStatus(_) => {},
                BlockStateMachineOutput::DeadSlotDetected(dead) => {
                    events.push(CoordinatorEvent::SlotDiscarded {
                        slot: dead.slot,
                        reason: DiscardReason::Dead,
                    });
                },
                BlockStateMachineOutput::ForksDetected(fork) => {
                    events.push(CoordinatorEvent::SlotDiscarded {
                        slot: fork.slot,
                        reason: DiscardReason::Forked,
                    });
                },
            }
        }

        events
    }

    fn convert_parsed_message(msg: CoordinatorMessage<R>) -> Vec<CoordinatorEvent<R>> {
        match msg {
            CoordinatorMessage::InstructionParsed { slot, key, record } => {
                vec![CoordinatorEvent::InstructionRecordParsed { slot, key, record }]
            },
            CoordinatorMessage::AccountParsed { slot, key, record } => {
                vec![CoordinatorEvent::AccountRecordParsed { slot, key, record }]
            },
            CoordinatorMessage::TransactionParsed { slot } => {
                vec![CoordinatorEvent::TransactionParsed { slot }]
            },
            CoordinatorMessage::ParseStats { slot, kind } => {
                vec![CoordinatorEvent::ParseStats { slot, kind }]
            },
        }
    }
}
