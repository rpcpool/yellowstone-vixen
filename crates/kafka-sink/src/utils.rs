/// Generate a unique Kafka key for deduplication.
/// Format: `{slot}:{signature}:{ix_index}`
pub fn make_record_key(slot: u64, signature: &str, ix_index: &str) -> String {
    format!("{slot}:{signature}:{ix_index}")
}
