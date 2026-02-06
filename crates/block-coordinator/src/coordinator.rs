use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use solana_clock::Slot;
use tokio::sync::mpsc;
use yellowstone_block_machine::state_machine::{
    BlockReplayEvent, BlockStateMachineOutput, BlocksStateMachine, ConsensusUpdate, FrozenBlock,
};

use crate::{
    buffer::SlotRecordBuffer,
    types::{ConfirmedSlot, CoordinatorInput, CoordinatorMessage},
};

/// Wraps a slot number with a deterministic ANSI color for log readability.
/// Consecutive slots get different colors so interleaved events are easy to follow.
struct ColorSlot(Slot);

impl fmt::Display for ColorSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const COLORS: [&str; 6] = [
            "\x1b[91m", // red
            "\x1b[92m", // green
            "\x1b[93m", // yellow
            "\x1b[94m", // blue
            "\x1b[95m", // magenta
            "\x1b[96m", // cyan
        ];
        let color = COLORS[(self.0 % COLORS.len() as u64) as usize];
        write!(f, "{}{}\x1b[0m", color, self.0)
    }
}

/// Core orchestrator that owns a `BlocksStateMachine` and buffers parsed records per slot.
///
/// Receives events from two channels:
/// - `input_rx`: BlockSM events extracted from the geyser stream (via CoordinatorSource tap)
/// - `parsed_rx`: Parsed records from handlers (via CoordinatorHandle)
///
/// Flushes confirmed slots in strict order to `output_tx` for downstream consumption.
///
/// ## Two-Gate Flush
///
/// A slot flushes only when BOTH conditions are met:
/// 1. **fully_parsed** — all transactions have been parsed by handlers
///    (parsed_tx_count >= expected from FrozenBlock)
/// 2. **confirmed** — BlockSM confirmed the slot via cluster consensus
///
/// ## Sequential Ordering
///
/// `try_flush_sequential()` iterates the BTreeMap from lowest slot and stops
/// at the first non-ready slot. Dead/forked slots are removed to unblock
/// subsequent slots.
pub struct BlockMachineCoordinator<R> {
    block_sm: BlocksStateMachine,
    buffer: BTreeMap<Slot, SlotRecordBuffer<R>>,
    /// Slots whose BlockSummary was rejected by BlockSM (untracked).
    /// Parsed records for these slots are dropped to prevent re-creating
    /// a buffer entry that can never satisfy the two-gate flush.
    discarded_slots: BTreeSet<Slot>,
    /// Highest slot number that has been successfully flushed.
    /// Used to detect two-gate invariant violations (late parsed messages).
    last_flushed_slot: Option<Slot>,
    input_rx: mpsc::Receiver<CoordinatorInput>,
    parsed_rx: mpsc::Receiver<CoordinatorMessage<R>>,
    output_tx: mpsc::Sender<ConfirmedSlot<R>>,
}

impl<R: Send + 'static> BlockMachineCoordinator<R> {
    pub fn new(
        input_rx: mpsc::Receiver<CoordinatorInput>,
        parsed_rx: mpsc::Receiver<CoordinatorMessage<R>>,
        output_tx: mpsc::Sender<ConfirmedSlot<R>>,
    ) -> Self {
        Self {
            block_sm: BlocksStateMachine::default(),
            buffer: BTreeMap::new(),
            discarded_slots: BTreeSet::new(),
            last_flushed_slot: None,
            input_rx,
            parsed_rx,
            output_tx,
        }
    }

    /// Main event loop. Runs until both input channels close.
    pub async fn run(mut self) {
        loop {
            tokio::select! {
                Some(input) = self.input_rx.recv() => {
                    self.handle_input(input);
                }
                Some(msg) = self.parsed_rx.recv() => {
                    self.handle_parsed_message(msg);
                }
                else => {
                    tracing::warn!("Coordinator channels closed, shutting down");
                    break;
                }
            }
        }
    }

    /// Feed a BlockSM input event and drain any resulting outputs.
    fn handle_input(&mut self, input: CoordinatorInput) {
        match input {
            CoordinatorInput::Replay(event) => {
                let (event_slot, lifecycle_stage, is_block_summary) = match &event {
                    BlockReplayEvent::Entry(e) => (e.slot, None, false),
                    BlockReplayEvent::SlotLifecycleStatus(s) => (s.slot, Some(s.stage), false),
                    BlockReplayEvent::BlockSummary(b) => (b.slot, None, true),
                };

                match self.block_sm.process_replay_event(event) {
                    Ok(()) => {
                        if let Some(stage) = lifecycle_stage {
                            tracing::info!(slot = %ColorSlot(event_slot), stage = ?stage, "Slot lifecycle");
                        }
                    },
                    Err(_) => {
                        // Entry rejections are noisy (hundreds per partial slot).
                        if lifecycle_stage.is_some() || is_block_summary {
                            tracing::warn!(
                                slot = %ColorSlot(event_slot),
                                is_block_summary,
                                stage = ?lifecycle_stage,
                                "Untracked slot — rejected"
                            );
                        }
                        // BlockSummary is the final replay event for a slot.
                        // If rejected, the slot can never complete — discard it
                        // to unblock sequential flush.
                        if is_block_summary {
                            self.discard_slot(event_slot, "untracked");
                        }
                    },
                }
                self.drain_block_sm_outputs();
            },
            CoordinatorInput::Consensus(event) => {
                let ConsensusUpdate::SlotCommitmentStatus(ref status) = event;
                tracing::debug!(slot = status.slot, commitment = ?status.commitment, "Consensus event");
                self.block_sm.process_consensus_event(event);
                self.drain_block_sm_outputs();
            },
            CoordinatorInput::BlockExtra {
                slot,
                block_time,
                block_height,
            } => {
                if !self.discarded_slots.contains(&slot) {
                    let buf = self.buffer.entry(slot).or_default();
                    buf.block_time = block_time;
                    buf.block_height = block_height;
                }
            },
        }
    }

    /// Drain all pending outputs from the BlockSM after feeding it an event.
    fn drain_block_sm_outputs(&mut self) {
        while let Some(output) = self.block_sm.pop_next_unprocess_blockstore_update() {
            self.handle_block_sm_output(output);
        }
    }

    /// React to a single BlockSM output.
    ///
    /// - FrozenBlock → store expected tx count + metadata, try flush
    /// - SlotStatus(Confirmed) → mark confirmed, try flush
    /// - DeadSlotDetected → discard buffered records
    /// - ForksDetected → discard buffered records
    fn handle_block_sm_output(&mut self, output: BlockStateMachineOutput) {
        match output {
            BlockStateMachineOutput::FrozenBlock(frozen) => {
                self.on_frozen_block(frozen);
            },
            BlockStateMachineOutput::SlotStatus(status) => {
                if status.commitment == solana_commitment_config::CommitmentLevel::Confirmed {
                    let slot = status.slot;
                    let buf = self.buffer.entry(slot).or_default();
                    buf.mark_confirmed();
                    tracing::info!(
                        slot = %ColorSlot(slot),
                        parsed_tx = buf.parsed_tx_count(),
                        expected_tx = ?buf.expected_tx_count,
                        fully_parsed = buf.parsed_tx_count() >= buf.expected_tx_count.unwrap_or(u64::MAX),
                        records = buf.record_count(),
                        "Slot confirmed"
                    );
                    self.try_flush_sequential();
                }
            },
            BlockStateMachineOutput::DeadSlotDetected(dead) => {
                self.discard_slot(dead.slot, "dead");
            },
            BlockStateMachineOutput::ForksDetected(fork) => {
                self.discard_slot(fork.slot, "forked");
            },
        }
    }

    /// Handle a FrozenBlock: store metadata and expected transaction count.
    fn on_frozen_block(&mut self, frozen: FrozenBlock) {
        let slot = frozen.slot;
        let expected_tx_count: u64 = frozen.entries.iter().map(|e| e.executed_txn_count).sum();

        let buf = self.buffer.entry(slot).or_default();
        buf.parent_slot = Some(frozen.parent_slot);
        buf.blockhash = Some(frozen.blockhash);
        buf.set_expected_tx_count(expected_tx_count);

        tracing::info!(
            slot = %ColorSlot(slot),
            expected_tx_count,
            entry_count = frozen.entries.len(),
            parent_slot = frozen.parent_slot,
            "Block frozen"
        );

        // Transactions may have arrived before FrozenBlock.
        self.try_flush_sequential();
    }

    /// Handle a parsed record or transaction-parsed signal from a handler.
    fn handle_parsed_message(&mut self, msg: CoordinatorMessage<R>) {
        let slot = match &msg {
            CoordinatorMessage::Parsed { slot, .. }
            | CoordinatorMessage::TransactionParsed { slot } => *slot,
        };

        // Drop messages for discarded slots (dead/forked/untracked).
        if self.discarded_slots.contains(&slot) {
            return;
        }

        // Late messages for flushed slots indicate a critical bug — the two-gate
        // system guarantees all transactions are parsed before flush. If this
        // happens, something is fundamentally broken and we must crash to
        // prevent silent data corruption.
        if self.last_flushed_slot.is_some_and(|last| slot <= last) {
            panic!(
                "TWO-GATE INVARIANT VIOLATED: Received parsed message for slot {} \
                 but last_flushed_slot is {:?}. This means a slot was flushed before \
                 all its transactions were parsed. Check handler/coordinator timing.",
                slot, self.last_flushed_slot
            );
        }

        let buf = self.buffer.entry(slot).or_default();
        match msg {
            CoordinatorMessage::Parsed {
                tx_index,
                ix_path,
                record,
                ..
            } => {
                buf.insert_record(tx_index, ix_path, record);
            },
            CoordinatorMessage::TransactionParsed { .. } => {
                buf.increment_parsed_tx_count();
                self.try_flush_sequential();
            },
        }
    }

    /// Flush ready slots in strict ascending slot order.
    ///
    /// Iterates the BTreeMap from lowest slot. Stops at the first non-ready slot
    /// to preserve cross-slot ordering. Dead/forked slots are removed by
    /// `discard_slot`, which then calls this method to unblock subsequent slots.
    ///
    /// ## Gap Detection
    ///
    /// A slot can only flush if its parent has been flushed (or discarded).
    /// This prevents out-of-order flushing when slots arrive non-sequentially.
    fn try_flush_sequential(&mut self) {
        while let Some((slot, buf)) = self.buffer.first_key_value() {
            if !buf.is_ready() {
                break;
            }

            // Gap check: parent must be flushed or discarded.
            let parent_slot = buf.parent_slot.unwrap_or(0);
            let parent_ok = self
                .last_flushed_slot
                .is_some_and(|last| parent_slot <= last)
                || self.discarded_slots.contains(&parent_slot);

            // Allow flush if this is the first slot we've ever seen.
            let is_first = self.last_flushed_slot.is_none();

            if !is_first && !parent_ok {
                tracing::debug!(
                    slot = %ColorSlot(*slot),
                    parent_slot,
                    last_flushed = ?self.last_flushed_slot,
                    "Slot ready but parent not flushed — waiting"
                );
                break;
            }

            let (slot, mut buf) = self.buffer.pop_first().unwrap();
            let records = buf.drain_sorted_records();

            let confirmed = ConfirmedSlot {
                slot,
                parent_slot: buf.parent_slot.unwrap_or(0),
                blockhash: buf.blockhash.unwrap_or_default(),
                block_time: buf.block_time,
                block_height: buf.block_height,
                executed_transaction_count: buf.expected_tx_count.unwrap_or(0),
                records,
            };

            tracing::info!(
                slot = %ColorSlot(slot),
                tx_count = confirmed.executed_transaction_count,
                record_count = confirmed.records.len(),
                block_height = ?confirmed.block_height,
                parent_slot = confirmed.parent_slot,
                "Flushing slot"
            );

            // Track this as the last flushed slot.
            self.last_flushed_slot = Some(slot);

            if self.output_tx.try_send(confirmed).is_err() {
                tracing::error!(slot = %ColorSlot(slot), "Failed to send confirmed slot to output");
            }
        }
    }

    /// Remove a slot from the buffer (dead or forked).
    /// Then try flushing — discarding may unblock the next slot in sequence.
    fn discard_slot(&mut self, slot: Slot, reason: &str) {
        self.discarded_slots.insert(slot);
        // Keep only the 100 most recent discarded slots.
        while self.discarded_slots.len() > 100 {
            self.discarded_slots.pop_first();
        }
        if let Some(buf) = self.buffer.remove(&slot) {
            tracing::warn!(
                slot = %ColorSlot(slot),
                reason,
                discarded_records = buf.record_count(),
                "Discarding slot"
            );
        }
        self.try_flush_sequential();
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    use super::*;

    #[test]
    fn buffer_and_flush_manually() {
        // Unit test the buffer/flush logic directly (no BlockSM, no async).
        let mut coordinator = {
            let (_itx, input_rx) = mpsc::channel(1);
            let (_ptx, parsed_rx) = mpsc::channel(1);
            let (output_tx, _orx) = mpsc::channel(1);
            BlockMachineCoordinator::<String>::new(input_rx, parsed_rx, output_tx)
        };

        let slot = 100;
        let buf = coordinator.buffer.entry(slot).or_default();
        buf.insert_record(0, vec![0], "record-a".into());
        buf.set_expected_tx_count(1);
        buf.increment_parsed_tx_count();
        buf.parent_slot = Some(99);
        buf.blockhash = Some(solana_hash::Hash::default());

        // Not ready yet — not confirmed.
        assert!(!coordinator.buffer[&slot].is_ready());

        coordinator.buffer.get_mut(&slot).unwrap().mark_confirmed();
        assert!(coordinator.buffer[&slot].is_ready());
    }

    #[test]
    fn sequential_flush_stops_at_non_ready() {
        let (_itx, input_rx) = mpsc::channel(1);
        let (_ptx, parsed_rx) = mpsc::channel(1);
        let (output_tx, mut output_rx) = mpsc::channel(64);
        let mut coord = BlockMachineCoordinator::<String>::new(input_rx, parsed_rx, output_tx);

        // Slot 100: ready
        let buf100 = coord.buffer.entry(100).or_default();
        buf100.set_expected_tx_count(0);
        buf100.mark_confirmed();
        buf100.parent_slot = Some(99);
        buf100.blockhash = Some(solana_hash::Hash::default());

        // Slot 101: NOT ready (not confirmed)
        let buf101 = coord.buffer.entry(101).or_default();
        buf101.set_expected_tx_count(0);
        buf101.parent_slot = Some(100);
        buf101.blockhash = Some(solana_hash::Hash::default());

        // Slot 102: ready
        let buf102 = coord.buffer.entry(102).or_default();
        buf102.set_expected_tx_count(0);
        buf102.mark_confirmed();
        buf102.parent_slot = Some(101);
        buf102.blockhash = Some(solana_hash::Hash::default());

        coord.try_flush_sequential();

        // Only slot 100 should have flushed (101 blocks 102).
        let flushed = output_rx.try_recv().unwrap();
        assert_eq!(flushed.slot, 100);
        assert!(output_rx.try_recv().is_err());

        // Slot 101 and 102 still in buffer.
        assert!(coord.buffer.contains_key(&101));
        assert!(coord.buffer.contains_key(&102));
    }

    #[test]
    fn discard_unblocks_next_slot() {
        let (_itx, input_rx) = mpsc::channel(1);
        let (_ptx, parsed_rx) = mpsc::channel(1);
        let (output_tx, mut output_rx) = mpsc::channel(64);
        let mut coord = BlockMachineCoordinator::<String>::new(input_rx, parsed_rx, output_tx);

        // Slot 100: blocking (not ready)
        let buf100 = coord.buffer.entry(100).or_default();
        buf100.insert_record(0, vec![0], "will-be-discarded".into());

        // Slot 101: ready
        let buf101 = coord.buffer.entry(101).or_default();
        buf101.set_expected_tx_count(1);
        buf101.increment_parsed_tx_count();
        buf101.mark_confirmed();
        buf101.parent_slot = Some(100);
        buf101.blockhash = Some(solana_hash::Hash::default());
        buf101.insert_record(0, vec![0], "keeper".into());

        // Discard slot 100 → should unblock and flush 101.
        coord.discard_slot(100, "dead");

        let flushed = output_rx.try_recv().unwrap();
        assert_eq!(flushed.slot, 101);
        assert_eq!(flushed.records, vec!["keeper".to_string()]);
    }
}
