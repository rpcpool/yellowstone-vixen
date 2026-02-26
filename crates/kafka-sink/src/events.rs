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

/// Event published for unparsed/unknown accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawAccountEvent {
    pub slot: u64,
    /// Account pubkey (base58).
    pub pubkey: String,
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

/// Event published when an account slot is committed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSlotCommitEvent {
    pub slot: u64,
    /// Number of successfully decoded accounts.
    pub decoded_account_count: u64,
    /// Number of accounts that were not decoded because no parser matched.
    pub decode_filtered_account_count: u64,
    /// Number of accounts that were not decoded because parser execution failed.
    pub decode_error_account_count: u64,
    /// Number of undecoded accounts published to fallback topics.
    pub fallback_account_count: u64,
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
    /// Instructions: `{slot}:{signature}:{ix_index}`, Accounts: `{slot}:{pubkey}`.
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
