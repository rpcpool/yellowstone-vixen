use serde::{Deserialize, Serialize};

/// Event published for unparsed/unknown instructions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawInstructionEvent {
    pub slot: u64,
    /// Transaction signature (base58).
    pub signature: String,
    /// Instruction index (e.g., "0.1.2").
    pub ix_index: String,
    /// Program ID (base58).
    pub program_id: String,
    /// Raw instruction data (base58).
    pub data: String,
}

/// Event published for unparsed/unknown accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawAccountEvent {
    pub slot: u64,
    /// Account pubkey (base58).
    pub pubkey: String,
    /// Geyser write version for deterministic ordering.
    pub write_version: u64,
    /// Owner program ID (base58).
    pub owner: String,
    /// Raw account data (base58).
    pub data: String,
}

/// Event published when an instruction slot is fully committed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSlotCommitEvent {
    pub slot: u64,
    pub blockhash: String,
    /// Total transactions in the block.
    pub transaction_count: u64,
    /// Number of successfully decoded instructions.
    pub decoded_instruction_count: u64,
    /// Number of instructions that were not decoded because no parser matched.
    pub decode_filtered_instruction_count: u64,
    /// Number of instructions that were not decoded because parser execution failed.
    pub decode_error_instruction_count: u64,
    /// Number of undecoded instructions published to fallback topics.
    pub fallback_instruction_count: u64,
    /// Number of transactions that failed on-chain execution.
    pub transaction_status_failed_count: u64,
    /// Number of transactions that succeeded on-chain execution.
    pub transaction_status_succeeded_count: u64,
}

/// Marker semantics for account slot events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkerType {
    Completed,
    Watermark,
}

impl std::fmt::Display for MarkerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Completed => f.write_str("completed"),
            Self::Watermark => f.write_str("watermark"),
        }
    }
}

/// Commitment scope for account slot markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommitScope {
    Confirmed,
    Finalized,
    Stream,
}

impl std::fmt::Display for CommitScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Confirmed => f.write_str("confirmed"),
            Self::Finalized => f.write_str("finalized"),
            Self::Stream => f.write_str("stream"),
        }
    }
}

/// Event published when an account slot is committed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSlotCommitEvent {
    pub slot: u64,
    /// See [`MarkerType`].
    pub marker_type: MarkerType,
    /// See [`CommitScope`].
    pub account_commit_at: CommitScope,
    /// Number of successfully decoded accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_account_count: Option<u64>,
    /// Number of accounts that were not decoded because no parser matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decode_filtered_account_count: Option<u64>,
    /// Number of accounts that were not decoded because parser execution failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decode_error_account_count: Option<u64>,
    /// Number of undecoded accounts published to fallback topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_account_count: Option<u64>,
}

/// Distinguishes instruction records from account records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordKind {
    Instruction,
    Account,
}

/// A prepared Kafka record ready for batch publishing.
#[derive(Debug, Clone)]
pub struct PreparedRecord {
    /// Target Kafka topic.
    pub topic: String,
    /// Payload bytes.
    /// Decoded records use protobuf (Confluent wire format when configured).
    /// Fallback records use plain JSON payloads.
    pub payload: Vec<u8>,
    /// Unique key for deduplication.
    /// Instructions: `{slot}:{signature}:{ix_index}`, Accounts: `{slot}:{pubkey}:{write_version}`.
    pub key: String,
    /// Kafka headers for metadata (readable without decoding payload).
    pub headers: Vec<RecordHeader>,
    /// Whether this is a decoded record (true) or fallback/unknown (false).
    pub is_decoded: bool,
    /// Whether this record is an instruction or account record.
    pub kind: RecordKind,
}

/// A Kafka record header (key-value pair).
#[derive(Debug, Clone)]
pub struct RecordHeader {
    pub key: &'static str,
    pub value: String,
}
