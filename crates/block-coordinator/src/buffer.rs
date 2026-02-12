use std::collections::BTreeMap;

use solana_clock::Slot;
use solana_hash::Hash;

use crate::types::{ConfirmedSlot, RecordSortKey};

/// Per-slot buffer that collects parsed records and tracks the two-gate flush condition.
///
/// Gate 1 (is_fully_parsed): All transactions have been parsed by handlers.
///   Determined by comparing `parsed_tx_count` against `expected_tx_count` from FrozenBlock.
///
/// Gate 2 (confirmed): BlockSM confirmed the slot via cluster consensus.
///
/// A slot flushes only when BOTH gates are satisfied.
#[derive(Debug)]
pub struct SlotRecordBuffer<R> {
    /// Records sorted by (tx_index, ix_path) for ordered flush.
    records: BTreeMap<RecordSortKey, R>,
    /// Block metadata from FrozenBlock.
    parent_slot: Option<Slot>,
    blockhash: Option<Hash>,
    /// Gate 1: fully parsed.
    expected_tx_count: Option<u64>,
    parsed_tx_count: u64,
    /// Gate 2: confirmed by cluster consensus.
    confirmed: bool,
}

impl<R> Default for SlotRecordBuffer<R> {
    fn default() -> Self {
        Self {
            records: BTreeMap::new(),
            parent_slot: None,
            blockhash: None,
            expected_tx_count: None,
            parsed_tx_count: 0,
            confirmed: false,
        }
    }
}

impl<R> SlotRecordBuffer<R> {
    pub fn insert_record(&mut self, key: RecordSortKey, record: R) {
        self.records.insert(key, record);
    }

    /// Set all block metadata from a FrozenBlock in one atomic operation.
    pub fn set_block_metadata(
        &mut self,
        parent_slot: Slot,
        blockhash: Hash,
        expected_tx_count: u64,
    ) {
        self.parent_slot = Some(parent_slot);
        self.blockhash = Some(blockhash);
        self.expected_tx_count = Some(expected_tx_count);
    }

    pub fn increment_parsed_tx_count(&mut self) {
        self.parsed_tx_count += 1;
    }

    pub fn mark_as_confirmed(&mut self) {
        self.confirmed = true;
    }

    pub fn is_fully_parsed(&self) -> bool {
        self.expected_tx_count
            .map(|expected| self.parsed_tx_count >= expected)
            .unwrap_or(false)
    }

    /// Both gates must be satisfied for flush.
    pub fn is_ready(&self) -> bool {
        self.is_fully_parsed() && self.confirmed
    }

    pub fn parent_slot(&self) -> Option<Slot> {
        self.parent_slot
    }

    pub fn expected_tx_count(&self) -> Option<u64> {
        self.expected_tx_count
    }

    pub fn parsed_tx_count(&self) -> u64 {
        self.parsed_tx_count
    }

    pub fn is_confirmed(&self) -> bool {
        self.confirmed
    }

    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// Consume this buffer and produce a ConfirmedSlot.
    /// Panics if metadata is missing (caller must verify `is_ready()` first).
    pub fn into_confirmed_slot(mut self, slot: Slot) -> ConfirmedSlot<R> {
        ConfirmedSlot {
            slot,
            parent_slot: self
                .parent_slot
                .expect("ready buffer must have parent_slot from FrozenBlock"),
            blockhash: self
                .blockhash
                .expect("ready buffer must have blockhash from FrozenBlock"),
            executed_transaction_count: self
                .expected_tx_count
                .expect("ready buffer must have expected_tx_count from FrozenBlock"),
            records: self.drain_sorted_records(),
        }
    }

    /// Drain all records in sorted order (by tx_index, then ix_path).
    fn drain_sorted_records(&mut self) -> Vec<R> {
        std::mem::take(&mut self.records).into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_sorted_drain() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_block_metadata(0, Hash::default(), 0);
        // Insert out of order
        buf.insert_record(
            RecordSortKey { tx_index: 1, ix_path: vec![0] },
            "tx1-ix0".into(),
        );
        buf.insert_record(
            RecordSortKey { tx_index: 0, ix_path: vec![0, 1] },
            "tx0-ix0.1".into(),
        );
        buf.insert_record(
            RecordSortKey { tx_index: 0, ix_path: vec![0] },
            "tx0-ix0".into(),
        );

        let confirmed = buf.into_confirmed_slot(42);
        assert_eq!(
            confirmed.records,
            vec![
                "tx0-ix0".to_string(),
                "tx0-ix0.1".to_string(),
                "tx1-ix0".to_string(),
            ]
        );
    }

    #[test]
    fn two_gate_not_ready_by_default() {
        let buf = SlotRecordBuffer::<String>::default();
        assert!(!buf.is_ready());
    }

    #[test]
    fn two_gate_both_required() {
        // Only fully_parsed
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_block_metadata(0, Hash::default(), 1);
        buf.increment_parsed_tx_count();
        assert!(!buf.is_ready());

        // Only confirmed
        let mut buf2 = SlotRecordBuffer::<String>::default();
        buf2.mark_as_confirmed();
        assert!(!buf2.is_ready());
    }

    #[test]
    fn two_gate_ready_when_both() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_block_metadata(0, Hash::default(), 2);
        buf.increment_parsed_tx_count();
        buf.increment_parsed_tx_count();
        buf.mark_as_confirmed();
        assert!(buf.is_ready());
    }

    #[test]
    fn ix_path_depth_first_ordering() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_block_metadata(0, Hash::default(), 0);
        // Simulate: tx0 has main ix [0] with two CPIs [0,0] and [0,1]
        // And [0,0] has a nested CPI [0,0,0]
        buf.insert_record(
            RecordSortKey { tx_index: 0, ix_path: vec![0, 1] },
            "cpi-1".into(),
        );
        buf.insert_record(
            RecordSortKey { tx_index: 0, ix_path: vec![0, 0, 0] },
            "nested-cpi".into(),
        );
        buf.insert_record(
            RecordSortKey { tx_index: 0, ix_path: vec![0] },
            "main".into(),
        );
        buf.insert_record(
            RecordSortKey { tx_index: 0, ix_path: vec![0, 0] },
            "cpi-0".into(),
        );

        let confirmed = buf.into_confirmed_slot(42);
        assert_eq!(
            confirmed.records,
            vec![
                "main".to_string(),
                "cpi-0".to_string(),
                "nested-cpi".to_string(),
                "cpi-1".to_string(),
            ]
        );
    }

    #[test]
    fn drain_empties_buffer() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.insert_record(
            RecordSortKey { tx_index: 0, ix_path: vec![0] },
            "record".into(),
        );
        assert_eq!(buf.record_count(), 1);

        buf.set_block_metadata(0, Hash::default(), 0);
        let confirmed = buf.into_confirmed_slot(42);
        assert_eq!(confirmed.records.len(), 1);
    }
}
