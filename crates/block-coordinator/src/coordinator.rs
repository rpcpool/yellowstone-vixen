use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use solana_clock::Slot;
use tokio::sync::mpsc;
use yellowstone_block_machine::{
    dragonsmouth::wrapper::BlocksStateMachineWrapper,
    state_machine::{BlockStateMachineOutput, FrozenBlock},
};
use yellowstone_grpc_proto::geyser::SubscribeUpdate;

use crate::{
    buffer::SlotRecordBuffer,
    types::{ConfirmedSlot, CoordinatorMessage},
};

/// Maximum number of discarded slots to retain in memory.
/// Used to prevent re-buffering records for slots that were already discarded.
const MAX_DISCARDED_SLOTS: usize = 100;

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

/// Core orchestrator that owns a `BlocksStateMachineWrapper` and buffers parsed records per slot.
///
/// Receives events from two channels:
/// - `input_rx`: Raw geyser SubscribeUpdate events (via CoordinatorSource tap)
/// - `parsed_rx`: Parsed records from handlers (via CoordinatorHandle)
///
/// Flushes confirmed slots in strict order to `output_tx` for downstream consumption.
///
/// ## Two-Gate Flush
///
/// A slot flushes only when BOTH conditions are met:
/// 1. **is_fully_parsed** — all transactions have been parsed by handlers
///    (parsed_tx_count >= expected from FrozenBlock)
/// 2. **confirmed** — BlockSM confirmed the slot via cluster consensus
///
/// ## Sequential Ordering
///
/// `try_flush_sequential()` iterates the BTreeMap from lowest slot and stops
/// at the first non-ready slot. Dead/forked slots are removed to unblock
/// subsequent slots.
///
/// ## Flush Consolidation
///
/// `try_flush_sequential()` is called exactly once per event in `run()`,
/// after all state mutations are applied. Internal methods only mutate state.
pub struct BlockMachineCoordinator<R> {
    wrapper: BlocksStateMachineWrapper,
    buffer: BTreeMap<Slot, SlotRecordBuffer<R>>,
    /// Slots whose BlockSummary was rejected by BlockSM (untracked).
    /// Parsed records for these slots are dropped to prevent re-creating
    /// a buffer entry that can never satisfy the two-gate flush.
    discarded_slots: BTreeSet<Slot>,
    /// Highest slot number that has been successfully flushed.
    /// Used to detect two-gate invariant violations (late parsed messages).
    last_flushed_slot: Option<Slot>,
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
            buffer: BTreeMap::new(),
            discarded_slots: BTreeSet::new(),
            last_flushed_slot: None,
            input_rx,
            parsed_rx,
            output_tx,
        }
    }

    /// Main event loop. Runs until all input channels close.
    ///
    /// Each event follows: apply state mutations → try flush.
    /// This is the single site where `try_flush_sequential` is called.
    pub async fn run(mut self) {
        loop {
            tokio::select! {
                Some(update) = self.input_rx.recv() => {
                    self.apply_update(&update);
                }
                Some(msg) = self.parsed_rx.recv() => {
                    self.apply_parsed_message(msg);
                }
                else => {
                    tracing::warn!("Coordinator channels closed, shutting down");
                    break;
                }
            }
            self.try_flush_sequential();
        }
    }

    /// Feed a raw geyser SubscribeUpdate to the wrapper and drain any resulting outputs.
    fn apply_update(&mut self, update: &SubscribeUpdate) {
        if self.wrapper.handle_new_geyser_event(update).is_err() {
            // Untracked slot - check if this was a BlockMeta (BlockSummary) rejection
            if let Some(yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof::BlockMeta(
                meta,
            )) = &update.update_oneof
            {
                tracing::warn!(
                    slot = %ColorSlot(meta.slot),
                    "BlockMeta for untracked slot — discarding"
                );
                self.discard_slot(meta.slot, "untracked");
            }
        }
        self.drain_and_apply_block_sm_outputs();
    }

    /// Drain all pending outputs from the BlockSM after feeding it an event.
    fn drain_and_apply_block_sm_outputs(&mut self) {
        while let Some(output) = self.wrapper.pop_next_state_machine_output() {
            self.apply_block_sm_output(output);
        }
    }

    /// Apply a single BlockSM output to buffer state. No flush calls.
    fn apply_block_sm_output(&mut self, output: BlockStateMachineOutput) {
        match output {
            BlockStateMachineOutput::FrozenBlock(frozen) => {
                self.apply_frozen_block(frozen);
            },
            BlockStateMachineOutput::SlotStatus(status) => {
                if status.commitment == solana_commitment_config::CommitmentLevel::Confirmed {
                    let slot = status.slot;
                    let buf = self.buffer.entry(slot).or_default();
                    buf.mark_as_confirmed();
                    tracing::info!(
                        slot = %ColorSlot(slot),
                        parsed_tx = buf.parsed_tx_count(),
                        expected_tx = ?buf.expected_tx_count(),
                        fully_parsed = buf.is_fully_parsed(),
                        records = buf.record_count(),
                        "Slot confirmed"
                    );
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

    /// Apply a FrozenBlock: store metadata and expected transaction count.
    fn apply_frozen_block(&mut self, frozen: FrozenBlock) {
        let slot = frozen.slot;
        let expected_tx_count: u64 = frozen.entries.iter().map(|e| e.executed_txn_count).sum();

        let buf = self.buffer.entry(slot).or_default();
        buf.set_block_metadata(frozen.parent_slot, frozen.blockhash, expected_tx_count);

        tracing::info!(
            slot = %ColorSlot(slot),
            expected_tx_count,
            entry_count = frozen.entries.len(),
            parent_slot = frozen.parent_slot,
            "Block frozen"
        );
    }

    /// Apply a parsed record or transaction-parsed signal to buffer state.
    fn apply_parsed_message(&mut self, msg: CoordinatorMessage<R>) {
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
            CoordinatorMessage::Parsed { key, record, .. } => {
                buf.insert_record(key, record);
            },
            CoordinatorMessage::TransactionParsed { .. } => {
                buf.increment_parsed_tx_count();
            },
        }
    }

    /// Flush ready slots in strict ascending slot order.
    ///
    /// Iterates the BTreeMap from lowest slot. Stops at the first non-ready slot
    /// to preserve cross-slot ordering. Dead/forked slots are removed by
    /// `discard_slot`, so they never block subsequent slots.
    ///
    /// ## Gap Detection
    ///
    /// A slot can only flush if its parent has been flushed (or discarded).
    /// This prevents out-of-order flushing when slots arrive non-sequentially.
    fn try_flush_sequential(&mut self) {
        while let Some((&slot, buf)) = self.buffer.first_key_value() {
            if !buf.is_ready() {
                break;
            }

            // Gap check: parent must be flushed or discarded.
            let parent_slot = buf
                .parent_slot()
                .expect("ready slot must have parent_slot from FrozenBlock");
            let parent_ok = self
                .last_flushed_slot
                .is_some_and(|last| parent_slot <= last)
                || self.discarded_slots.contains(&parent_slot);

            // Allow flush if this is the first slot we've ever seen.
            let is_first = self.last_flushed_slot.is_none();

            if !is_first && !parent_ok {
                tracing::debug!(
                    slot = %ColorSlot(slot),
                    parent_slot,
                    last_flushed = ?self.last_flushed_slot,
                    "Slot ready but parent not flushed — waiting"
                );
                break;
            }

            let (slot, buf) = self.buffer.pop_first().unwrap();
            let confirmed = buf.into_confirmed_slot(slot);

            tracing::info!(
                slot = %ColorSlot(slot),
                tx_count = confirmed.executed_transaction_count,
                record_count = confirmed.records.len(),
                parent_slot = confirmed.parent_slot,
                "Flushing slot"
            );

            self.last_flushed_slot = Some(slot);

            if self.output_tx.try_send(confirmed).is_err() {
                tracing::error!(slot = %ColorSlot(slot), "Failed to send confirmed slot to output");
            }
        }
    }

    /// Remove a slot from the buffer (dead or forked).
    /// Discarding may unblock subsequent slots — the caller (`run`) will
    /// call `try_flush_sequential` after this event completes.
    fn discard_slot(&mut self, slot: Slot, reason: &str) {
        self.discarded_slots.insert(slot);
        self.trim_discarded_slots();

        if let Some(buf) = self.buffer.remove(&slot) {
            tracing::warn!(
                slot = %ColorSlot(slot),
                reason,
                discarded_records = buf.record_count(),
                "Discarding slot"
            );
        }
    }

    fn trim_discarded_slots(&mut self) {
        while self.discarded_slots.len() > MAX_DISCARDED_SLOTS {
            self.discarded_slots.pop_first();
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    use super::*;
    use crate::types::RecordSortKey;

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
        buf.insert_record(
            RecordSortKey { tx_index: 0, ix_path: vec![0] },
            "record-a".into(),
        );
        buf.set_block_metadata(99, solana_hash::Hash::default(), 1);
        buf.increment_parsed_tx_count();

        // Not ready yet — not confirmed.
        assert!(!coordinator.buffer[&slot].is_ready());

        coordinator.buffer.get_mut(&slot).unwrap().mark_as_confirmed();
        assert!(coordinator.buffer[&slot].is_ready());
    }

    #[test]
    fn sequential_flush_stops_at_non_ready() {
        let (_itx, input_rx) = mpsc::channel(1);
        let (_ptx, parsed_rx) = mpsc::channel(1);
        let (output_tx, mut output_rx) = mpsc::channel(64);
        let mut coord =
            BlockMachineCoordinator::<String>::new(input_rx, parsed_rx, output_tx);

        // Slot 100: ready
        let buf100 = coord.buffer.entry(100).or_default();
        buf100.set_block_metadata(99, solana_hash::Hash::default(), 0);
        buf100.mark_as_confirmed();

        // Slot 101: NOT ready (not confirmed)
        let buf101 = coord.buffer.entry(101).or_default();
        buf101.set_block_metadata(100, solana_hash::Hash::default(), 0);

        // Slot 102: ready
        let buf102 = coord.buffer.entry(102).or_default();
        buf102.set_block_metadata(101, solana_hash::Hash::default(), 0);
        buf102.mark_as_confirmed();

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
        let mut coord =
            BlockMachineCoordinator::<String>::new(input_rx, parsed_rx, output_tx);

        // Slot 100: blocking (not ready)
        let buf100 = coord.buffer.entry(100).or_default();
        buf100.insert_record(
            RecordSortKey { tx_index: 0, ix_path: vec![0] },
            "will-be-discarded".into(),
        );

        // Slot 101: ready
        let buf101 = coord.buffer.entry(101).or_default();
        buf101.set_block_metadata(100, solana_hash::Hash::default(), 1);
        buf101.increment_parsed_tx_count();
        buf101.mark_as_confirmed();
        buf101.insert_record(
            RecordSortKey { tx_index: 0, ix_path: vec![0] },
            "keeper".into(),
        );

        // Discard slot 100 → then flush should unblock 101.
        coord.discard_slot(100, "dead");
        coord.try_flush_sequential();

        let flushed = output_rx.try_recv().unwrap();
        assert_eq!(flushed.slot, 101);
        assert_eq!(flushed.records, vec!["keeper".to_string()]);
    }
}
