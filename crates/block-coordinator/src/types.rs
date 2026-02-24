use std::fmt;

use smallvec::SmallVec;
use solana_clock::Slot;
use solana_hash::Hash;
use yellowstone_grpc_proto::geyser::SubscribeUpdate;

/// Block metadata that always transitions together.
#[derive(Debug, Clone)]
pub struct BlockMetadata {
    pub parent_slot: Slot,
    pub blockhash: Hash,
    pub expected_tx_count: u64,
}

/// Reason a slot was discarded by the coordinator.
#[derive(Debug, Clone, Copy)]
pub enum DiscardReason {
    Dead,
    Forked,
    Untracked,
}

impl fmt::Display for DiscardReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Dead => "dead",
            Self::Forked => "forked",
            Self::Untracked => "untracked",
        };
        f.write_str(label)
    }
}

/// Coordinator invariants and unrecoverable errors.
#[derive(Debug)]
pub enum CoordinatorError {
    TwoGateInvariantViolation {
        slot: Slot,
        last_flushed: Option<Slot>,
    },
    ReadySlotMissingMetadata {
        slot: Slot,
    },
    OutputChannelClosed {
        slot: Slot,
    },
    AccountAfterConfirmed {
        slot: Slot,
    },
}

impl fmt::Display for CoordinatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TwoGateInvariantViolation { slot, last_flushed } => write!(
                f,
                "Two-gate invariant violated: slot {slot} <= last_flushed {last_flushed:?}"
            ),
            Self::ReadySlotMissingMetadata { slot } => {
                write!(f, "Ready slot missing metadata: slot {slot}")
            },
            Self::OutputChannelClosed { slot } => {
                write!(f, "Output channel closed while sending slot {slot}")
            },
            Self::AccountAfterConfirmed { slot } => {
                write!(
                    f,
                    "Account event after slot {slot} confirmed — geyser contract violation"
                )
            },
        }
    }
}

impl std::error::Error for CoordinatorError {}

/// Categorizes parse outcomes for stats tracking.
#[derive(Debug, Clone, Copy)]
pub enum ParseStatsKind {
    InstructionFiltered,
    InstructionError,
    AccountFiltered,
    AccountError,
    TransactionStatusFailed,
    TransactionStatusSucceeded,
}

/// Messages from handlers back to the coordinator.
pub enum CoordinatorMessage<R> {
    /// A parsed instruction record ready to buffer (sorted by tx_index, ix_path).
    InstructionParsed {
        slot: Slot,
        key: InstructionRecordSortKey,
        record: R,
    },
    /// A parsed account record ready to buffer, sorted by ingress_seq:pubkey.
    AccountParsed {
        slot: Slot,
        key: AccountRecordSortKey,
        record: R,
    },
    /// Signal that a transaction has been fully parsed by the handler.
    /// Coordinator counts these to determine when a slot is fully parsed.
    TransactionParsed { slot: Slot },
    /// A parse stat event (filtered or error) for aggregate tracking.
    ParseStats { slot: Slot, kind: ParseStatsKind },
}

impl<R> CoordinatorMessage<R> {
    pub fn slot(&self) -> Slot {
        match self {
            Self::InstructionParsed { slot, .. }
            | Self::AccountParsed { slot, .. }
            | Self::TransactionParsed { slot }
            | Self::ParseStats { slot, .. } => *slot,
        }
    }
}

/// Sort key for records within a slot.
/// Ordered by transaction index, then instruction path (depth-first CPI order).
/// SmallVec sorts lexicographically: [0] < [0,0] < [0,1] < [1] which matches
/// depth-first execution order. Inline storage for up to 4 elements avoids
/// heap allocation (Solana CPI depth is capped at 4).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstructionRecordSortKey {
    tx_index: u64,
    ix_path: SmallVec<[usize; 4]>,
}

impl InstructionRecordSortKey {
    pub fn new(tx_index: u64, ix_path: Vec<usize>) -> Self {
        Self {
            tx_index,
            ix_path: SmallVec::from_vec(ix_path),
        }
    }
}

/// Sort key for account records within a slot.
/// Ordered by ingress_seq (source-assigned monotonic sequence number for
/// deterministic ordering), then pubkey (discriminate multiple accounts
/// in the same update). Slot is implicit (it's the buffer's key).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountRecordSortKey {
    ingress_seq: u64,
    pubkey: [u8; 32],
}

impl AccountRecordSortKey {
    pub fn new(ingress_seq: u64, pubkey: [u8; 32]) -> Self {
        Self {
            ingress_seq,
            pubkey,
        }
    }
}

/// Input messages from the source to the coordinator.
/// Wraps raw gRPC events plus synthetic messages injected by the source.
pub enum CoordinatorInput {
    /// A raw geyser SubscribeUpdate (Entry, Slot, BlockMeta).
    GeyserUpdate(SubscribeUpdate),
    /// A raw Account event was seen on the geyser stream for this slot.
    /// Lightweight signal — only the slot, no protobuf payload.
    AccountEventSeen { slot: Slot },
}

/// Wraps a slot number with a deterministic ANSI color for log readability.
/// Consecutive slots get different colors so interleaved events are easy to follow.
pub struct ColorSlot(pub Slot);

impl fmt::Display for ColorSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const COLORS: [&str; 6] = [
            "\x1b[91m", // red
            "\x1b[92m", // green
            "\x1b[93m", // yellow
            "\x1b[94m", // blue
            "\x1b[95m", // magenta
            "\x1b[96m", // cyan
        ];
        let color = COLORS[(self.0 % COLORS.len() as u64) as usize];
        write!(f, "{}{}\x1b[0m", color, self.0)
    }
}

/// A confirmed slot ready for downstream consumption (e.g., Kafka write).
pub struct ConfirmedSlot<R> {
    pub slot: Slot,
    pub parent_slot: Slot,
    pub blockhash: Hash,
    pub executed_transaction_count: u64,
    pub records: Vec<R>,
    pub filtered_instruction_count: u64,
    pub failed_instruction_count: u64,
    pub filtered_account_count: u64,
    pub failed_account_count: u64,
    pub transaction_status_failed_count: u64,
    pub transaction_status_succeeded_count: u64,
}

/// Clonable handle for handlers to send messages to the coordinator.
#[derive(Clone)]
pub struct CoordinatorHandle<R> {
    tx: tokio::sync::mpsc::Sender<CoordinatorMessage<R>>,
}

impl<R: Send> CoordinatorHandle<R> {
    pub fn new(tx: tokio::sync::mpsc::Sender<CoordinatorMessage<R>>) -> Self { Self { tx } }

    pub async fn send_instruction_parsed(
        &self,
        slot: Slot,
        key: InstructionRecordSortKey,
        record: R,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<CoordinatorMessage<R>>> {
        self.tx
            .send(CoordinatorMessage::InstructionParsed { slot, key, record })
            .await
    }

    pub async fn send_account_parsed(
        &self,
        slot: Slot,
        key: AccountRecordSortKey,
        record: R,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<CoordinatorMessage<R>>> {
        self.tx
            .send(CoordinatorMessage::AccountParsed { slot, key, record })
            .await
    }

    pub async fn send_transaction_parsed(
        &self,
        slot: Slot,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<CoordinatorMessage<R>>> {
        self.tx
            .send(CoordinatorMessage::TransactionParsed { slot })
            .await
    }

    pub async fn send_parse_stats(
        &self,
        slot: Slot,
        kind: ParseStatsKind,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<CoordinatorMessage<R>>> {
        self.tx
            .send(CoordinatorMessage::ParseStats { slot, kind })
            .await
    }
}
