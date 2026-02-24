use std::collections::BTreeMap;

use solana_clock::Slot;

use crate::types::{AccountRecordSortKey, BlockMetadata, ConfirmedSlot, ParseStatsKind, InstructionRecordSortKey};

/// Per-slot buffer that collects parsed records and tracks the three-gate flush condition.
///
/// Gate 1 (is_fully_parsed): All transactions have been parsed by handlers.
///   Determined by comparing `parsed_tx_count` against `expected_tx_count` from FrozenBlock.
///
/// Gate 2 (confirmed): BlockSM confirmed the slot via cluster consensus.
///
/// Gate 3 (is_fully_account_processed): All account updates have been processed.
///   expected_account_count is frozen from the coordinator's account event counter
///   when the slot is confirmed by the block state machine.
///
/// A slot flushes only when ALL THREE gates are satisfied.
#[derive(Debug)]
pub struct SlotRecordBuffer<R> {
    /// Instruction records sorted by (tx_index, ix_path) for ordered flush.
    instruction_records: BTreeMap<InstructionRecordSortKey, R>,
    /// Account records sorted by (ingress_seq, pubkey) for ordered flush.
    account_records: BTreeMap<AccountRecordSortKey, R>,
    /// Block metadata from FrozenBlock.
    metadata: Option<BlockMetadata>,
    /// Gate 1: fully parsed.
    parsed_tx_count: u64,
    /// Gate 2: confirmed by cluster consensus.
    confirmed: bool,
    /// Gate 3: expected account count from source (None = not yet received, blocks flush).
    expected_account_count: Option<u64>,
    /// Gate 3: number of account events fully processed (records + filtered + failed).
    account_processed_count: u64,
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
            account_records: BTreeMap::new(),
            metadata: None,
            parsed_tx_count: 0,
            confirmed: false,
            expected_account_count: None,
            account_processed_count: 0,
            filtered_instruction_count: 0,
            failed_instruction_count: 0,
            filtered_account_count: 0,
            failed_account_count: 0,
        }
    }
}

impl<R> SlotRecordBuffer<R> {
    pub fn insert_instruction_record(&mut self, key: InstructionRecordSortKey, record: R) {
        if self.instruction_records.contains_key(&key) {
            tracing::warn!(
                ?key,
                "Duplicate instruction record sort key — previous record overwritten"
            );
        }
        self.instruction_records.insert(key, record);
    }

    pub fn insert_account_record(&mut self, key: AccountRecordSortKey, record: R) {
        if self.account_records.contains_key(&key) {
            tracing::warn!(
                ?key,
                "Duplicate AccountRecordSortKey — previous record overwritten"
            );
        }
        self.account_records.insert(key, record);
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

    pub fn increment_account_processed_count(&mut self) {
        self.account_processed_count += 1;
        if let Some(expected) = self.expected_account_count
            && self.account_processed_count > expected
        {
            tracing::error!(
                processed = self.account_processed_count,
                expected,
                "account_processed_count exceeds expected — investigate immediately"
            );
        }
    }

    pub fn mark_as_confirmed(&mut self) { self.confirmed = true; }

    /// Set the expected account count for Gate 3. First-write-wins: if already
    /// set, subsequent calls are ignored (prevents duplicate Slot(Confirmed)
    /// from overwriting a non-zero count with 0).
    pub fn set_expected_account_count(&mut self, count: u64) {
        if self.expected_account_count.is_some() {
            tracing::warn!(
                existing = ?self.expected_account_count,
                new = count,
                "expected_account_count already set — ignoring duplicate"
            );
            return;
        }
        if self.account_processed_count() > count {
            tracing::error!(
                processed = self.account_processed_count(),
                expected = count,
                "account_processed_count exceeds expected — possible source/handler mismatch"
            );
        }
        self.expected_account_count = Some(count);
    }

    pub fn account_processed_count(&self) -> u64 {
        self.account_processed_count
    }

    pub fn is_fully_parsed(&self) -> bool {
        self.metadata
            .as_ref()
            .is_some_and(|meta| self.parsed_tx_count >= meta.expected_tx_count)
    }

    /// Gate 3: all account updates processed. None = not yet received, blocks flush.
    pub fn is_fully_account_processed(&self) -> bool {
        self.expected_account_count
            .is_some_and(|n| self.account_processed_count() >= n)
    }

    /// All three gates must be satisfied for flush.
    pub fn is_ready(&self) -> bool {
        self.is_fully_parsed() && self.is_fully_account_processed() && self.confirmed
    }

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

    /// Drain all records: instruction records in sorted order first, then account records
    /// (sorted by ingress_seq:pubkey) appended.
    fn drain_all_records(&mut self) -> Vec<R> {
        let mut records: Vec<R> =
            std::mem::take(&mut self.instruction_records).into_values().collect();
        records.extend(std::mem::take(&mut self.account_records).into_values());
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
        buf.insert_instruction_record(InstructionRecordSortKey::new(1, vec![0]), "tx1-ix0".into());
        buf.insert_instruction_record(InstructionRecordSortKey::new(0, vec![0, 1]), "tx0-ix0.1".into());
        buf.insert_instruction_record(InstructionRecordSortKey::new(0, vec![0]), "tx0-ix0".into());

        let confirmed = buf.into_confirmed_slot(42).expect("confirmed slot");
        assert_eq!(confirmed.records, vec![
            "tx0-ix0".to_string(),
            "tx0-ix0.1".to_string(),
            "tx1-ix0".to_string(),
        ]);
    }

    #[test]
    fn three_gate_not_ready_by_default() {
        let buf = SlotRecordBuffer::<String>::default();
        assert!(!buf.is_ready());
    }

    #[test]
    fn three_gate_each_required() {
        // Only Gate 1 (fully_parsed) + Gate 3 (account count)
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_block_metadata(BlockMetadata {
            parent_slot: 0,
            blockhash: Hash::default(),
            expected_tx_count: 1,
        });
        buf.increment_parsed_tx_count();
        buf.set_expected_account_count(0);
        assert!(!buf.is_ready()); // missing Gate 2 (confirmed)

        // Only Gate 2 (confirmed) + Gate 3
        let mut buf2 = SlotRecordBuffer::<String>::default();
        buf2.mark_as_confirmed();
        buf2.set_expected_account_count(0);
        assert!(!buf2.is_ready()); // missing Gate 1

        // Only Gate 1 + Gate 2
        let mut buf3 = SlotRecordBuffer::<String>::default();
        buf3.set_block_metadata(BlockMetadata {
            parent_slot: 0,
            blockhash: Hash::default(),
            expected_tx_count: 1,
        });
        buf3.increment_parsed_tx_count();
        buf3.mark_as_confirmed();
        assert!(!buf3.is_ready()); // missing Gate 3 (expected_account_count is None)
    }

    #[test]
    fn three_gate_ready_when_all() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_block_metadata(BlockMetadata {
            parent_slot: 0,
            blockhash: Hash::default(),
            expected_tx_count: 2,
        });
        buf.increment_parsed_tx_count();
        buf.increment_parsed_tx_count();
        buf.set_expected_account_count(0);
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
        buf.insert_instruction_record(InstructionRecordSortKey::new(0, vec![0, 1]), "cpi-1".into());
        buf.insert_instruction_record(InstructionRecordSortKey::new(0, vec![0, 0, 0]), "nested-cpi".into());
        buf.insert_instruction_record(InstructionRecordSortKey::new(0, vec![0]), "main".into());
        buf.insert_instruction_record(InstructionRecordSortKey::new(0, vec![0, 0]), "cpi-0".into());

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
        buf.insert_instruction_record(InstructionRecordSortKey::new(0, vec![0]), "record".into());
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
        buf.set_expected_account_count(0);
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
        buf.set_expected_account_count(0);
        buf.mark_as_confirmed();
        assert!(buf.is_ready());

        let confirmed = buf.into_confirmed_slot(42).expect("confirmed slot");
        assert_eq!(confirmed.parent_slot, 20);
        assert_eq!(confirmed.blockhash, new_hash);
        assert_eq!(confirmed.executed_transaction_count, 3);
    }

    #[test]
    fn gate3_none_blocks_flush() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_block_metadata(BlockMetadata {
            parent_slot: 0,
            blockhash: Hash::default(),
            expected_tx_count: 0,
        });
        buf.mark_as_confirmed();
        // Gate 1 + Gate 2 satisfied, but Gate 3 is None → not ready.
        assert!(!buf.is_fully_account_processed());
        assert!(!buf.is_ready());
    }

    #[test]
    fn gate3_some_zero_passes() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_expected_account_count(0);
        // No accounts expected, none processed → Gate 3 satisfied.
        assert!(buf.is_fully_account_processed());
    }

    #[test]
    fn gate3_mixed_account_processing() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_expected_account_count(3);
        assert!(!buf.is_fully_account_processed());

        // 1 successful record
        buf.insert_account_record(
            AccountRecordSortKey::new(100, [1; 32]),
            "acct1".into(),
        );
        buf.increment_account_processed_count();
        assert_eq!(buf.account_processed_count(), 1);
        assert!(!buf.is_fully_account_processed());

        // 1 filtered
        buf.increment_parse_stat(ParseStatsKind::AccountFiltered);
        buf.increment_account_processed_count();
        assert_eq!(buf.account_processed_count(), 2);
        assert!(!buf.is_fully_account_processed());

        // 1 error
        buf.increment_parse_stat(ParseStatsKind::AccountError);
        buf.increment_account_processed_count();
        assert_eq!(buf.account_processed_count(), 3);
        assert!(buf.is_fully_account_processed());
    }

    #[test]
    fn gate3_first_write_wins() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_expected_account_count(5);
        // Second call is ignored (first-write-wins).
        buf.set_expected_account_count(0);
        // Still expects 5, not 0.
        assert!(!buf.is_fully_account_processed());
        for i in 0..5 {
            buf.insert_account_record(
                AccountRecordSortKey::new(i, [i as u8; 32]),
                format!("acct{i}"),
            );
            buf.increment_account_processed_count();
        }
        assert!(buf.is_fully_account_processed());
    }

    #[test]
    fn account_records_drain_in_write_version_order() {
        let mut buf = SlotRecordBuffer::<String>::default();
        buf.set_block_metadata(BlockMetadata {
            parent_slot: 0,
            blockhash: Hash::default(),
            expected_tx_count: 0,
        });
        // Insert out of order
        buf.insert_account_record(
            AccountRecordSortKey::new(300, [3; 32]),
            "wv300".into(),
        );
        buf.insert_account_record(
            AccountRecordSortKey::new(100, [1; 32]),
            "wv100".into(),
        );
        buf.insert_account_record(
            AccountRecordSortKey::new(200, [2; 32]),
            "wv200".into(),
        );

        buf.set_expected_account_count(3);
        let confirmed = buf.into_confirmed_slot(42).expect("confirmed slot");
        assert_eq!(confirmed.records, vec![
            "wv100".to_string(),
            "wv200".to_string(),
            "wv300".to_string(),
        ]);
    }
}
