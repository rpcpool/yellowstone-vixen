/// Generate a unique Kafka key for transaction/instruction record deduplication.
/// Format: `{slot}:{signature}:{ix_index}`
pub fn make_instruction_record_key(slot: u64, signature: &str, ix_index: &str) -> String {
    format!("{slot}:{signature}:{ix_index}")
}

/// Generate a unique Kafka key for account record deduplication.
/// Format: `{slot}:{pubkey}:{write_version}`
pub fn make_account_record_key(slot: u64, pubkey: &str, write_version: u64) -> String {
    format!("{slot}:{pubkey}:{write_version}")
}
