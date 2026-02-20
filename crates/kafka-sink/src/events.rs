use serde::{Deserialize, Serialize};

/// Event published for each decoded instruction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedInstructionEvent {
    /// Slot number where this instruction was processed.
    pub slot: u64,
    /// Transaction signature (base58).
    pub signature: String,
    /// Instruction path within the transaction.
    /// Format: "0" for top-level, "0.1" for first inner instruction, "0.1.2" for nested, etc.
    /// Solana supports up to 5 levels of CPI depth.
    pub ix_index: String,
    /// Program name (e.g., "spl-token").
    pub program: String,
    /// Discriminant/variant identifier.
    pub instruction_type: String,
    /// Human-readable instruction name (e.g., "TransferChecked").
    pub instruction_name: String,
    /// Full instruction data (debug format).
    pub data: String,
}

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

/// Event published when a slot is fully committed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotCommitEvent {
    pub slot: u64,
    pub blockhash: String,
    /// Total transactions in the block.
    pub transaction_count: u64,
    /// Number of successfully decoded instructions.
    pub decoded_instruction_count: u64,
    /// Number of successfully decoded accounts.
    pub decoded_account_count: u64,
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
    /// Protobuf-encoded payload (via prost::Message::encode).
    pub payload: Vec<u8>,
    /// Unique key for deduplication.
    /// Instructions: `{slot}:{signature}:{ix_index}`, Accounts: `{slot}:{pubkey}`.
    pub key: String,
    /// Kafka headers for metadata (readable without decoding payload).
    pub headers: Vec<RecordHeader>,
    /// Label for logging (instruction name or program id).
    pub label: String,
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
