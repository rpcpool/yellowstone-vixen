use solana_clock::Slot;
use solana_hash::Hash;
use yellowstone_block_machine::state_machine::{BlockReplayEvent, ConsensusUpdate};

/// Input from the source tap to the coordinator.
/// Lightweight extraction from SubscribeUpdate (integers + 32-byte hash, no large allocs).
pub enum CoordinatorInput {
    /// Block replay events: Entry, SlotLifecycle, BlockSummary
    Replay(BlockReplayEvent),
    /// Consensus events: SlotCommitmentStatus (Processed/Confirmed/Finalized)
    Consensus(ConsensusUpdate),
    /// Extra metadata from BlockMeta not captured by BlockSummary.
    /// BlockSummary gives us slot, entry_count, executed_tx_count, blockhash.
    /// This gives us block_time and block_height for the commit event.
    BlockExtra {
        slot: Slot,
        block_time: Option<i64>,
        block_height: Option<u64>,
    },
}

/// Messages from handlers back to the coordinator.
pub enum CoordinatorMessage<R> {
    /// A parsed record ready to buffer.
    Parsed {
        slot: Slot,
        tx_index: u64,
        ix_path: Vec<usize>,
        record: R,
    },
    /// Signal that a transaction has been fully parsed by the handler.
    /// Coordinator counts these to determine when a slot is fully parsed.
    TransactionParsed { slot: Slot },
}

/// Sort key for records within a slot.
/// Ordered by transaction index, then instruction path (depth-first CPI order).
/// Vec<usize> sorts lexicographically: [0] < [0,0] < [0,1] < [1] which matches
/// depth-first execution order.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecordSortKey {
    pub tx_index: u64,
    pub ix_path: Vec<usize>,
}

/// A confirmed slot ready for downstream consumption (e.g., Kafka write).
pub struct ConfirmedSlot<R> {
    pub slot: Slot,
    pub parent_slot: Slot,
    pub blockhash: Hash,
    pub block_time: Option<i64>,
    pub block_height: Option<u64>,
    pub executed_transaction_count: u64,
    pub records: Vec<R>,
}

/// Clonable handle for handlers to send messages to the coordinator.
#[derive(Clone)]
pub struct CoordinatorHandle<R> {
    tx: tokio::sync::mpsc::Sender<CoordinatorMessage<R>>,
}

impl<R: Send> CoordinatorHandle<R> {
    pub fn new(tx: tokio::sync::mpsc::Sender<CoordinatorMessage<R>>) -> Self {
        Self { tx }
    }

    pub async fn send_parsed(
        &self,
        slot: Slot,
        tx_index: u64,
        ix_path: Vec<usize>,
        record: R,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<CoordinatorMessage<R>>> {
        self.tx
            .send(CoordinatorMessage::Parsed {
                slot,
                tx_index,
                ix_path,
                record,
            })
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
}
