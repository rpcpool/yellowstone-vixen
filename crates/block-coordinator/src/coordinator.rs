use tokio::sync::mpsc;
use yellowstone_block_machine::{
    dragonsmouth::wrapper::BlocksStateMachineWrapper, state_machine::BlockStateMachineOutput,
};
use yellowstone_grpc_proto::geyser::{subscribe_update::UpdateOneof, SubscribeUpdate};

use crate::{
    state::{CoordinatorEvent, CoordinatorState},
    types::{
        BlockMetadata, ColorSlot, ConfirmedSlot, CoordinatorError, CoordinatorMessage,
        DiscardReason,
    },
};

/// Core orchestrator that owns a `BlocksStateMachineWrapper` and delegates
/// ordering/flush decisions to the pure-ish `CoordinatorState`.
pub struct BlockMachineCoordinator<R> {
    wrapper: BlocksStateMachineWrapper,
    state: CoordinatorState<R>,
    input_rx: mpsc::Receiver<SubscribeUpdate>,
    parsed_rx: mpsc::Receiver<CoordinatorMessage<R>>,
    output_tx: mpsc::Sender<ConfirmedSlot<R>>,
}

impl<R: Send + 'static> BlockMachineCoordinator<R> {
    pub fn new(
        input_rx: mpsc::Receiver<SubscribeUpdate>,
        parsed_rx: mpsc::Receiver<CoordinatorMessage<R>>,
        output_tx: mpsc::Sender<ConfirmedSlot<R>>,
    ) -> Self {
        Self {
            wrapper: BlocksStateMachineWrapper::default(),
            state: CoordinatorState::default(),
            input_rx,
            parsed_rx,
            output_tx,
        }
    }

    /// Main event loop. Any CoordinatorError terminates the coordinator.
    pub async fn run(mut self) -> Result<(), CoordinatorError> {
        loop {
            let events: Vec<CoordinatorEvent<R>> = tokio::select! {
                Some(update) = self.input_rx.recv() => {
                    self.convert_geyser_update(&update)
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
                    let metadata = BlockMetadata {
                        parent_slot: frozen.parent_slot,
                        blockhash: frozen.blockhash,
                        expected_tx_count: frozen
                            .entries
                            .iter()
                            .map(|e| e.executed_txn_count)
                            .sum(),
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
            CoordinatorMessage::Parsed { slot, key, record } => {
                vec![CoordinatorEvent::RecordParsed { slot, key, record }]
            },
            CoordinatorMessage::TransactionParsed { slot } => {
                vec![CoordinatorEvent::TransactionParsed { slot }]
            },
        }
    }
}
