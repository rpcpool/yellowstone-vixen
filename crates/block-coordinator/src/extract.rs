use solana_commitment_config::CommitmentLevel;
use solana_hash::Hash;
use yellowstone_block_machine::state_machine::{
    BlockReplayEvent, BlockSummary, ConsensusUpdate, EntryInfo, SlotCommitmentStatusUpdate,
    SlotLifecycle, SlotLifecycleUpdate,
};
use yellowstone_grpc_proto::geyser::{
    subscribe_update::UpdateOneof, SlotStatus, SubscribeUpdate, SubscribeUpdateBlockMeta,
    SubscribeUpdateEntry, SubscribeUpdateSlot,
};

use crate::types::CoordinatorInput;

// TODO: upgrade version to be allowed to use yellowstone-block-machine directly
/// Extract coordinator inputs from a geyser SubscribeUpdate.
///
/// Mirrors the logic in `yellowstone-block-machine`'s `BlocksStateMachineWrapper`
/// but produces `CoordinatorInput` values instead of calling the state machine directly.
///
/// Returns an empty vec for event types that don't feed BlockSM (Transaction, Account, etc.).
/// Returns multiple inputs for BlockMeta (BlockSummary + BlockExtra metadata).
pub fn extract_coordinator_inputs(update: &SubscribeUpdate) -> Vec<CoordinatorInput> {
    let Some(ref oneof) = update.update_oneof else {
        return vec![];
    };

    match oneof {
        UpdateOneof::Slot(slot_update) => extract_from_slot(slot_update).into_iter().collect(),

        UpdateOneof::Entry(entry) => vec![extract_from_entry(entry)],

        UpdateOneof::BlockMeta(block_meta) => extract_from_block_meta(block_meta),

        // Transaction, Account, Block, Ping, etc. — not BlockSM inputs.
        _ => vec![],
    }
}

/// Map a geyser slot update to a coordinator input.
///
/// Follows the same logic as `BlocksStateMachineWrapper::handle_slot_update`:
/// - Lifecycle statuses (FirstShredReceived, CreatedBank, Completed, Dead) → BlockReplayEvent
/// - Commitment statuses with dead_error → downgraded to Dead lifecycle event
/// - Commitment statuses (Processed, Confirmed, Finalized) → ConsensusUpdate
fn extract_from_slot(s: &SubscribeUpdateSlot) -> Option<CoordinatorInput> {
    let lifecycle = |stage| {
        CoordinatorInput::Replay(BlockReplayEvent::SlotLifecycleStatus(SlotLifecycleUpdate {
            slot: s.slot,
            parent_slot: s.parent,
            stage,
        }))
    };

    // Lifecycle statuses map directly to SlotLifecycle.
    let lifecycle_stage = match s.status() {
        SlotStatus::SlotFirstShredReceived => Some(SlotLifecycle::FirstShredReceived),
        SlotStatus::SlotCreatedBank => Some(SlotLifecycle::CreatedBank),
        SlotStatus::SlotCompleted => Some(SlotLifecycle::Completed),
        SlotStatus::SlotDead => Some(SlotLifecycle::Dead),
        _ => None,
    };

    if let Some(stage) = lifecycle_stage {
        return Some(lifecycle(stage));
    }

    // Commitment status with dead_error → downgrade to Dead lifecycle event.
    // This matches the block-machine wrapper behavior.
    if s.dead_error.is_some() {
        return Some(lifecycle(SlotLifecycle::Dead));
    }

    // Remaining statuses are consensus commitment levels.
    let commitment = match s.status() {
        SlotStatus::SlotProcessed => CommitmentLevel::Processed,
        SlotStatus::SlotConfirmed => CommitmentLevel::Confirmed,
        SlotStatus::SlotFinalized => CommitmentLevel::Finalized,
        _ => {
            tracing::trace!(slot = s.slot, status = ?s.status(), "Unknown slot status, ignoring");
            return None;
        },
    };

    Some(CoordinatorInput::Consensus(
        ConsensusUpdate::SlotCommitmentStatus(SlotCommitmentStatusUpdate {
            parent_slot: s.parent,
            slot: s.slot,
            commitment,
        }),
    ))
}

/// Map a geyser entry update to a coordinator input.
///
/// Converts `SubscribeUpdateEntry` → `EntryInfo` using the same field mapping
/// as `BlocksStateMachineWrapper::handle_block_entry`.
fn extract_from_entry(entry: &SubscribeUpdateEntry) -> CoordinatorInput {
    let entry_hash = Hash::new_from_array(
        entry
            .hash
            .as_slice()
            .try_into()
            .expect("entry hash must be 32 bytes"),
    );

    CoordinatorInput::Replay(BlockReplayEvent::Entry(EntryInfo {
        slot: entry.slot,
        entry_index: entry.index,
        starting_txn_index: entry.starting_transaction_index,
        entry_hash,
        executed_txn_count: entry.executed_transaction_count,
    }))
}

/// Map a geyser block meta update to coordinator inputs.
///
/// Produces two inputs:
/// 1. `BlockSummary` for BlockSM (slot reconstruction)
/// 2. `BlockExtra` for metadata not in FrozenBlock (block_time, block_height)
///
/// Uses bs58 decoding for the blockhash string, matching the block-machine wrapper.
fn extract_from_block_meta(b: &SubscribeUpdateBlockMeta) -> Vec<CoordinatorInput> {
    let blockhash_bytes = match bs58::decode(&b.blockhash).into_vec() {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::warn!(slot = b.slot, ?e, "Invalid blockhash encoding");
            return vec![];
        },
    };

    let blockhash = Hash::new_from_array(
        blockhash_bytes
            .try_into()
            .expect("blockhash must be 32 bytes"),
    );

    vec![
        CoordinatorInput::Replay(BlockReplayEvent::BlockSummary(BlockSummary {
            slot: b.slot,
            entry_count: b.entries_count,
            parent_slot: b.parent_slot,
            executed_transaction_count: b.executed_transaction_count,
            blockhash,
        })),
        CoordinatorInput::BlockExtra {
            slot: b.slot,
            block_time: b.block_time.as_ref().map(|bt| bt.timestamp),
            block_height: b.block_height.as_ref().map(|bh| bh.block_height),
        },
    ]
}
