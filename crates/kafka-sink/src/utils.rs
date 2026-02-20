/// Generate a unique Kafka key for transaction/instruction record deduplication.
/// Format: `{slot}:{signature}:{ix_index}`
pub fn make_instruction_record_key(slot: u64, signature: &str, ix_index: &str) -> String {
    format!("{slot}:{signature}:{ix_index}")
}

/// Generate a unique Kafka key for account record compaction.
/// Format: `{slot}:{pubkey}`
pub fn make_account_record_key(slot: u64, pubkey: &str) -> String {
    format!("{slot}:{pubkey}")
}
