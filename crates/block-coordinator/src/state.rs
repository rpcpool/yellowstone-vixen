use std::collections::{BTreeMap, BTreeSet};

use solana_clock::Slot;

use crate::{
    buffer::SlotRecordBuffer,
    types::{BlockMetadata, ConfirmedSlot, CoordinatorError, DiscardReason, RecordSortKey},
};

/// All inputs to the coordinator state machine.
pub enum CoordinatorEvent<R> {
    /// Block metadata received (from FrozenBlock via BlockSM wrapper).
    BlockFrozen { slot: Slot, metadata: BlockMetadata },
    /// Slot confirmed by cluster consensus.
    SlotConfirmed { slot: Slot },
    /// Slot discarded (dead, forked, or untracked).
    SlotDiscarded { slot: Slot, reason: DiscardReason },
    /// A parsed record from a handler.
    RecordParsed {
        slot: Slot,
        key: RecordSortKey,
        record: R,
    },
    /// A handler finished parsing a transaction.
    TransactionParsed { slot: Slot },
}

/// Pure-ish coordinator state (no channels, no wrapper).
pub struct CoordinatorState<R> {
    buffer: BTreeMap<Slot, SlotRecordBuffer<R>>,
    discarded_slots: BTreeSet<Slot>,
    last_flushed_slot: Option<Slot>,
}

impl<R> CoordinatorState<R> {
    /// arbitrary number
    const MAX_DISCARDED_SLOTS: usize = 100;

    pub fn pending_slot_count(&self) -> usize { self.buffer.len() }

    pub fn discarded_slot_count(&self) -> usize { self.discarded_slots.len() }

    pub fn last_flushed_slot(&self) -> Option<Slot> { self.last_flushed_slot }

    pub fn oldest_pending_slot(&self) -> Option<Slot> {
        self.buffer.first_key_value().map(|(&s, _)| s)
    }

    pub fn apply(&mut self, event: CoordinatorEvent<R>) -> Result<(), CoordinatorError> {
        match event {
            CoordinatorEvent::BlockFrozen { slot, metadata } => {
                if self.is_already_flushed(slot) {
                    return Ok(());
                }
                self.buffer
                    .entry(slot)
                    .or_default()
                    .set_block_metadata(metadata);
            },
            CoordinatorEvent::SlotConfirmed { slot } => {
                if self.is_already_flushed(slot) {
                    return Ok(());
                }
                self.buffer.entry(slot).or_default().mark_as_confirmed();
            },
            CoordinatorEvent::SlotDiscarded { slot, reason } => {
                self.discard_slot(slot, reason);
            },
            CoordinatorEvent::RecordParsed { slot, key, record } => {
                if !self.validate_slot(slot)? {
                    return Ok(());
                }
                self.buffer
                    .entry(slot)
                    .or_default()
                    .insert_record(key, record);
            },
            CoordinatorEvent::TransactionParsed { slot } => {
                if !self.validate_slot(slot)? {
                    return Ok(());
                }
                self.buffer
                    .entry(slot)
                    .or_default()
                    .increment_parsed_tx_count();
            },
        }
        Ok(())
    }

    pub fn drain_flushable(&mut self) -> Result<Vec<ConfirmedSlot<R>>, CoordinatorError> {
        let mut flushed = Vec::new();

        while let Some((&slot, buf)) = self.buffer.first_key_value() {
            if !buf.is_ready() {
                break;
            }

            let parent_slot = buf
                .parent_slot()
                .ok_or(CoordinatorError::ReadySlotMissingMetadata { slot })?;

            // First-slot exemption: when no slot has been flushed yet, we have
            // no parent chain to validate against. The very first ready slot
            // flushes unconditionally to bootstrap the pipeline.
            let is_first = self.last_flushed_slot.is_none();
            let parent_ok = self
                .last_flushed_slot
                .is_some_and(|last| parent_slot <= last)
                || self.discarded_slots.contains(&parent_slot);

            if !is_first && !parent_ok {
                break;
            }

            let (slot, buf) = self.buffer.pop_first().unwrap();
            let confirmed = buf
                .into_confirmed_slot(slot)
                .ok_or(CoordinatorError::ReadySlotMissingMetadata { slot })?;
            self.last_flushed_slot = Some(slot);
            flushed.push(confirmed);
        }
        if !flushed.is_empty() {
            self.prune_discarded_slots();
        }
        Ok(flushed)
    }

    /// Guard for BlockFrozen/SlotConfirmed — drop stale lifecycle events
    /// for slots that have already been flushed. Should never happen under
    /// normal operation; if it does, a geyser/validator bug is sending
    /// duplicate events.
    fn is_already_flushed(&self, slot: Slot) -> bool {
        if self.last_flushed_slot.is_some_and(|last| slot <= last) {
            tracing::error!(
                slot,
                last_flushed = ?self.last_flushed_slot,
                "Lifecycle event for already-flushed slot — investigate immediately"
            );
            return true;
        }
        false
    }

    fn validate_slot(&self, slot: Slot) -> Result<bool, CoordinatorError> {
        if self.discarded_slots.contains(&slot) {
            return Ok(false);
        }
        if self.last_flushed_slot.is_some_and(|last| slot <= last) {
            return Err(CoordinatorError::TwoGateInvariantViolation {
                slot,
                last_flushed: self.last_flushed_slot,
            });
        }
        Ok(true)
    }

    fn discard_slot(&mut self, slot: Slot, reason: DiscardReason) {
        self.discarded_slots.insert(slot);
        self.prune_discarded_slots();
        if let Some(buf) = self.buffer.remove(&slot) {
            tracing::warn!(slot, %reason, records = buf.record_count(), "Discarding slot");
        }
    }

    /// Remove discarded-slot entries that are already covered by `last_flushed_slot`.
    ///
    /// Once a slot is flushed past a discarded slot, the `parent_slot <= last`
    /// check in `drain_flushable` already covers it — the entry in
    /// `discarded_slots` is redundant. Pruning only these entries ensures we
    /// never evict an entry still needed for gap resolution.
    ///
    /// A hard cap remains as a safety net: if the set grows beyond
    /// `MAX_DISCARDED_SLOTS` even after pruning (i.e. many discards ahead of
    /// `last_flushed_slot`), we log a warning but do NOT evict — evicting could
    /// permanently stall the pipeline.
    fn prune_discarded_slots(&mut self) {
        if let Some(last) = self.last_flushed_slot {
            self.discarded_slots.retain(|&s| s > last);
        }
        if self.discarded_slots.len() > Self::MAX_DISCARDED_SLOTS {
            tracing::warn!(
                count = self.discarded_slots.len(),
                max = Self::MAX_DISCARDED_SLOTS,
                "Discarded slots exceed cap — not evicting to preserve flush correctness"
            );
        }
    }
}

impl<R> Default for CoordinatorState<R> {
    fn default() -> Self {
        Self {
            buffer: BTreeMap::new(),
            discarded_slots: BTreeSet::new(),
            last_flushed_slot: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use solana_hash::Hash;

    use super::*;

    fn metadata(parent_slot: Slot, expected_tx_count: u64) -> BlockMetadata {
        BlockMetadata {
            parent_slot,
            blockhash: Hash::default(),
            expected_tx_count,
        }
    }

    fn apply_ready_slot(
        state: &mut CoordinatorState<String>,
        slot: Slot,
        parent: Slot,
        expected_tx_count: u64,
    ) {
        state
            .apply(CoordinatorEvent::BlockFrozen {
                slot,
                metadata: metadata(parent, expected_tx_count),
            })
            .unwrap();

        for _ in 0..expected_tx_count {
            state
                .apply(CoordinatorEvent::TransactionParsed { slot })
                .unwrap();
        }

        state
            .apply(CoordinatorEvent::SlotConfirmed { slot })
            .unwrap();
    }

    #[test]
    fn monotonic_flush_order() {
        let mut state = CoordinatorState::<String>::default();

        // Apply out of order to ensure ordering is enforced by flush.
        apply_ready_slot(&mut state, 102, 101, 1);
        apply_ready_slot(&mut state, 100, 99, 1);
        apply_ready_slot(&mut state, 103, 102, 1);
        apply_ready_slot(&mut state, 101, 100, 1);

        let flushed = state.drain_flushable().unwrap();
        let slots: Vec<_> = flushed.iter().map(|slot| slot.slot).collect();
        assert_eq!(slots, vec![100, 101, 102, 103]);
    }

    #[test]
    fn gap_invariant_holds() {
        let mut state = CoordinatorState::<String>::default();

        apply_ready_slot(&mut state, 100, 99, 1);
        apply_ready_slot(&mut state, 102, 101, 1);

        let flushed = state.drain_flushable().unwrap();
        assert_eq!(flushed.len(), 1);
        assert_eq!(flushed[0].slot, 100);

        state
            .apply(CoordinatorEvent::SlotDiscarded {
                slot: 101,
                reason: DiscardReason::Dead,
            })
            .unwrap();

        let flushed = state.drain_flushable().unwrap();
        assert_eq!(flushed.len(), 1);
        assert_eq!(flushed[0].slot, 102);
    }

    #[test]
    fn discard_unblocks_chain() {
        let mut state = CoordinatorState::<String>::default();

        apply_ready_slot(&mut state, 100, 99, 1);
        apply_ready_slot(&mut state, 101, 100, 1);
        apply_ready_slot(&mut state, 102, 101, 1);

        state
            .apply(CoordinatorEvent::SlotDiscarded {
                slot: 100,
                reason: DiscardReason::Dead,
            })
            .unwrap();

        let flushed = state.drain_flushable().unwrap();
        let slots: Vec<_> = flushed.iter().map(|slot| slot.slot).collect();
        assert_eq!(slots, vec![101, 102]);
    }

    #[test]
    fn no_data_loss() {
        let mut state = CoordinatorState::<String>::default();
        let slot = 100;

        state
            .apply(CoordinatorEvent::BlockFrozen {
                slot,
                metadata: metadata(99, 2),
            })
            .unwrap();

        state
            .apply(CoordinatorEvent::RecordParsed {
                slot,
                key: RecordSortKey::new(1, vec![0]),
                record: "b".to_string(),
            })
            .unwrap();
        state
            .apply(CoordinatorEvent::RecordParsed {
                slot,
                key: RecordSortKey::new(0, vec![0]),
                record: "a".to_string(),
            })
            .unwrap();

        state
            .apply(CoordinatorEvent::TransactionParsed { slot })
            .unwrap();
        state
            .apply(CoordinatorEvent::TransactionParsed { slot })
            .unwrap();
        state
            .apply(CoordinatorEvent::SlotConfirmed { slot })
            .unwrap();

        let flushed = state.drain_flushable().unwrap();
        assert_eq!(flushed.len(), 1);
        assert_eq!(flushed[0].records, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn idempotent_confirm() {
        let mut state = CoordinatorState::<String>::default();
        apply_ready_slot(&mut state, 100, 99, 0);

        let flushed = state.drain_flushable().unwrap();
        assert_eq!(flushed.len(), 1);
        assert_eq!(flushed[0].slot, 100);

        // Second confirm for an already-flushed slot is silently dropped.
        state
            .apply(CoordinatorEvent::SlotConfirmed { slot: 100 })
            .unwrap();

        let flushed = state.drain_flushable().unwrap();
        assert!(flushed.is_empty());
        // No zombie buffer entry — the guard prevents re-insertion.
        assert_eq!(state.pending_slot_count(), 0);
    }

    #[test]
    fn two_gate_violation_returns_error() {
        let mut state = CoordinatorState::<String>::default();
        apply_ready_slot(&mut state, 100, 99, 0);

        let _ = state.drain_flushable().unwrap();

        let err = state
            .apply(CoordinatorEvent::RecordParsed {
                slot: 100,
                key: RecordSortKey::new(0, vec![0]),
                record: "late".to_string(),
            })
            .unwrap_err();

        match err {
            CoordinatorError::TwoGateInvariantViolation { slot, .. } => {
                assert_eq!(slot, 100);
            },
            _ => panic!("Unexpected error: {err}"),
        }
    }

    #[test]
    fn eviction_does_not_block_children() {
        let mut state = CoordinatorState::<String>::default();

        // Establish a last_flushed_slot so parent/gap checks are enforced.
        apply_ready_slot(&mut state, 900, 899, 0);
        let flushed = state.drain_flushable().unwrap();
        assert_eq!(flushed.len(), 1);
        assert_eq!(state.last_flushed_slot(), Some(900));

        // Discard more than MAX_DISCARDED_SLOTS slots (200 > 100).
        // Under the old policy, the earliest discards would be evicted,
        // potentially stranding children whose parent was evicted.
        for s in 1000..1200 {
            state
                .apply(CoordinatorEvent::SlotDiscarded {
                    slot: s,
                    reason: DiscardReason::Dead,
                })
                .unwrap();
        }

        // Slot 1200 is a child of discarded slot 1000 (the first discard).
        // It must still flush — the discard entry for 1000 must not be evicted.
        apply_ready_slot(&mut state, 1200, 1000, 0);

        let flushed = state.drain_flushable().unwrap();
        assert_eq!(flushed.len(), 1);
        assert_eq!(flushed[0].slot, 1200);
    }

    #[test]
    fn prune_removes_entries_below_last_flushed() {
        let mut state = CoordinatorState::<String>::default();

        // Discard slots 50..60, then flush slot 100 (which advances last_flushed past them).
        for s in 50..60 {
            state
                .apply(CoordinatorEvent::SlotDiscarded {
                    slot: s,
                    reason: DiscardReason::Dead,
                })
                .unwrap();
        }

        apply_ready_slot(&mut state, 100, 99, 0);
        let flushed = state.drain_flushable().unwrap();
        assert_eq!(flushed.len(), 1);

        // After flushing past slot 100, discards 50..60 should be pruned.
        assert_eq!(state.discarded_slots.len(), 0);
    }

    #[test]
    fn observability_accessors() {
        let mut state = CoordinatorState::<String>::default();

        assert_eq!(state.pending_slot_count(), 0);
        assert_eq!(state.discarded_slot_count(), 0);
        assert_eq!(state.last_flushed_slot(), None);
        assert_eq!(state.oldest_pending_slot(), None);

        // Add two pending slots.
        apply_ready_slot(&mut state, 100, 99, 0);
        apply_ready_slot(&mut state, 102, 101, 0);
        assert_eq!(state.pending_slot_count(), 2);
        assert_eq!(state.oldest_pending_slot(), Some(100));

        // Discard a slot.
        state
            .apply(CoordinatorEvent::SlotDiscarded {
                slot: 50,
                reason: DiscardReason::Dead,
            })
            .unwrap();
        assert_eq!(state.discarded_slot_count(), 1);

        // Flush slot 100.
        let flushed = state.drain_flushable().unwrap();
        assert_eq!(flushed.len(), 1);
        assert_eq!(state.last_flushed_slot(), Some(100));
        assert_eq!(state.pending_slot_count(), 1);
        assert_eq!(state.oldest_pending_slot(), Some(102));
        // Discard for slot 50 was pruned (50 < last_flushed 100).
        assert_eq!(state.discarded_slot_count(), 0);
    }

    // ReadySlotMissingMetadata is defensive and should be unreachable with current invariants.
}
