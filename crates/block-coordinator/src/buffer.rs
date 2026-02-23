use std::collections::BTreeMap;

use solana_clock::Slot;

use crate::types::{BlockMetadata, ConfirmedSlot, ParseStatsKind, RecordSortKey};

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
    /// Instruction records sorted by (tx_index, ix_path) for ordered flush.
    instruction_records: BTreeMap<RecordSortKey, R>,
    /// Account records (no ordering needed — different topics).
    account_records: Vec<R>,
    /// Block metadata from FrozenBlock.
    metadata: Option<BlockMetadata>,
    /// Gate 1: fully parsed.
    parsed_tx_count: u64,
    /// Gate 2: confirmed by cluster consensus.
    confirmed: bool,
    /// Parse stats counters.
    filtered_instruction_count: u64,
    failed_instruction_count: u64,
    filtered_account_count: u64,
    failed_account_count: u64,
}

impl<R> Default for SlotRecordBuffer<R> {
    fn default() -> Self {
        Self {
            instruction_records: BTreeMap::new(),
            account_records: Vec::new(),
            metadata: None,
            parsed_tx_count: 0,
            confirmed: false,
            filtered_instruction_count: 0,
            failed_instruction_count: 0,
            filtered_account_count: 0,
            failed_account_count: 0,
        }
    }
}

impl<R> SlotRecordBuffer<R> {
    pub fn insert_record(&mut self, key: RecordSortKey, record: R) {
        if self.instruction_records.contains_key(&key) {
            tracing::warn!(
                ?key,
                "Duplicate RecordSortKey — previous record overwritten"
            );
        }
        self.instruction_records.insert(key, record);
    }

    pub fn insert_account_record(&mut self, record: R) {
        self.account_records.push(record);
    }

    /// Set all block metadata from a FrozenBlock in one atomic operation.
    pub fn set_block_metadata(&mut self, metadata: BlockMetadata) {
        if self.parsed_tx_count > metadata.expected_tx_count {
            tracing::error!(
                parsed = self.parsed_tx_count,
                expected = metadata.expected_tx_count,
                "parsed_tx_count exceeds expected — possible handler bug"
            );
        }
        self.metadata = Some(metadata);
    }

    pub fn increment_parsed_tx_count(&mut self) {
        self.parsed_tx_count += 1;
        // INVARIANT: parsed_tx_count should never exceed expected_tx_count.
        // If this fires, a handler is sending duplicate TransactionParsed signals
        // or expected_tx_count from FrozenBlock entries is wrong. Investigate
        // immediately — the slot will still flush (>= not ==) to avoid stalling
        // the pipeline, but records may be incomplete or misordered.
        if let Some(meta) = &self.metadata
            && self.parsed_tx_count > meta.expected_tx_count
        {
            tracing::error!(
                parsed = self.parsed_tx_count,
                expected = meta.expected_tx_count,
                "parsed_tx_count exceeds expected — investigate immediately"
            );
        }
    }

    pub fn increment_parse_stat(&mut self, kind: ParseStatsKind) {
        match kind {
            ParseStatsKind::InstructionFiltered => self.filtered_instruction_count += 1,
            ParseStatsKind::InstructionError => self.failed_instruction_count += 1,
            ParseStatsKind::AccountFiltered => self.filtered_account_count += 1,
            ParseStatsKind::AccountError => self.failed_account_count += 1,
        }
    }

    pub fn mark_as_confirmed(&mut self) { self.confirmed = true; }

    pub fn is_fully_parsed(&self) -> bool {
        self.metadata
            .as_ref()
            .is_some_and(|meta| self.parsed_tx_count >= meta.expected_tx_count)
    }

    /// Both gates must be satisfied for flush.
    pub fn is_ready(&self) -> bool { self.is_fully_parsed() && self.confirmed }

    pub fn parent_slot(&self) -> Option<Slot> {
        self.metadata.as_ref().map(|meta| meta.parent_slot)
    }

    pub fn parsed_tx_count(&self) -> u64 { self.parsed_tx_count }

    pub fn is_confirmed(&self) -> bool { self.confirmed }

    pub fn record_count(&self) -> usize {
        self.instruction_records.len() + self.account_records.len()
    }

    /// Consume this buffer and produce a ConfirmedSlot.
    /// Returns None if metadata is missing.
    pub fn into_confirmed_slot(mut self, slot: Slot) -> Option<ConfirmedSlot<R>> {
        let metadata = self.metadata.take()?;
        Some(ConfirmedSlot {
            slot,
            parent_slot: metadata.parent_slot,
            blockhash: metadata.blockhash,
            executed_transaction_count: metadata.expected_tx_count,
            records: self.drain_all_records(),
            filtered_instruction_count: self.filtered_instruction_count,
            failed_instruction_count: self.failed_instruction_count,
            filtered_account_count: self.filtered_account_count,
            failed_account_count: self.failed_account_count,
        })
    }

    /// Drain all records: instruction records in sorted order first, then account records appended.
    fn drain_all_records(&mut self) -> Vec<R> {
        let mut records: Vec<R> =
            std::mem::take(&mut self.instruction_records).into_values().collect();
        records.append(&mut self.account_records);
        records
    }
}

#[cfg(test)]
mod tests {
    use solana_hash::Hash;

    use super::*;

    #[test]
    fn insert_and_sorted_drain() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_block_metadata(BlockMetadata {
            parent_slot: 0,
            blockhash: Hash::default(),
            expected_tx_count: 0,
        });
        // Insert out of order
        buf.insert_record(RecordSortKey::new(1, vec![0]), "tx1-ix0".into());
        buf.insert_record(RecordSortKey::new(0, vec![0, 1]), "tx0-ix0.1".into());
        buf.insert_record(RecordSortKey::new(0, vec![0]), "tx0-ix0".into());

        let confirmed = buf.into_confirmed_slot(42).expect("confirmed slot");
        assert_eq!(confirmed.records, vec![
            "tx0-ix0".to_string(),
            "tx0-ix0.1".to_string(),
            "tx1-ix0".to_string(),
        ]);
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
        buf.set_block_metadata(BlockMetadata {
            parent_slot: 0,
            blockhash: Hash::default(),
            expected_tx_count: 1,
        });
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
        buf.set_block_metadata(BlockMetadata {
            parent_slot: 0,
            blockhash: Hash::default(),
            expected_tx_count: 2,
        });
        buf.increment_parsed_tx_count();
        buf.increment_parsed_tx_count();
        buf.mark_as_confirmed();
        assert!(buf.is_ready());
    }

    #[test]
    fn ix_path_depth_first_ordering() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_block_metadata(BlockMetadata {
            parent_slot: 0,
            blockhash: Hash::default(),
            expected_tx_count: 0,
        });
        // Simulate: tx0 has main ix [0] with two CPIs [0,0] and [0,1]
        // And [0,0] has a nested CPI [0,0,0]
        buf.insert_record(RecordSortKey::new(0, vec![0, 1]), "cpi-1".into());
        buf.insert_record(RecordSortKey::new(0, vec![0, 0, 0]), "nested-cpi".into());
        buf.insert_record(RecordSortKey::new(0, vec![0]), "main".into());
        buf.insert_record(RecordSortKey::new(0, vec![0, 0]), "cpi-0".into());

        let confirmed = buf.into_confirmed_slot(42).expect("confirmed slot");
        assert_eq!(confirmed.records, vec![
            "main".to_string(),
            "cpi-0".to_string(),
            "nested-cpi".to_string(),
            "cpi-1".to_string(),
        ]);
    }

    #[test]
    fn drain_empties_buffer() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.insert_record(RecordSortKey::new(0, vec![0]), "record".into());
        assert_eq!(buf.record_count(), 1);

        buf.set_block_metadata(BlockMetadata {
            parent_slot: 0,
            blockhash: Hash::default(),
            expected_tx_count: 0,
        });
        let confirmed = buf.into_confirmed_slot(42).expect("confirmed slot");
        assert_eq!(confirmed.records.len(), 1);
    }

    #[test]
    fn parsed_tx_overshoot_still_ready() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_block_metadata(BlockMetadata {
            parent_slot: 0,
            blockhash: Hash::default(),
            expected_tx_count: 1,
        });
        buf.increment_parsed_tx_count();
        buf.increment_parsed_tx_count(); // overshoot
        buf.mark_as_confirmed();

        // Overshoot doesn't prevent readiness — the slot still flushes.
        // (The tracing::error fires at runtime to flag the handler bug.)
        assert!(buf.is_ready());
        assert_eq!(buf.parsed_tx_count(), 2);
    }

    #[test]
    fn set_block_metadata_overwrites() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_block_metadata(BlockMetadata {
            parent_slot: 10,
            blockhash: Hash::default(),
            expected_tx_count: 5,
        });
        assert_eq!(buf.parent_slot(), Some(10));

        // Second call overwrites — last metadata wins.
        let new_hash = Hash::new_unique();
        buf.set_block_metadata(BlockMetadata {
            parent_slot: 20,
            blockhash: new_hash,
            expected_tx_count: 3,
        });
        assert_eq!(buf.parent_slot(), Some(20));

        buf.increment_parsed_tx_count();
        buf.increment_parsed_tx_count();
        buf.increment_parsed_tx_count();
        buf.mark_as_confirmed();
        assert!(buf.is_ready());

        let confirmed = buf.into_confirmed_slot(42).expect("confirmed slot");
        assert_eq!(confirmed.parent_slot, 20);
        assert_eq!(confirmed.blockhash, new_hash);
        assert_eq!(confirmed.executed_transaction_count, 3);
    }
}
