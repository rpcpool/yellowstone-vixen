//! Replay tests using captured geyser fixture data.
//!
//! Uses `tests/fixtures/sample.bin` (captured 05-feb-2026).
//!
//! ```bash
//! cargo test -p yellowstone-vixen-block-coordinator
//! FIXTURE_PATH=/custom/path.bin cargo test -p yellowstone-vixen-block-coordinator
//! ```

use std::{collections::HashMap, env, path::PathBuf};

use solana_commitment_config::CommitmentLevel;
use tokio::sync::mpsc;
use yellowstone_block_machine::state_machine::{
    BlockReplayEvent, ConsensusUpdate, SlotLifecycle,
};
use yellowstone_grpc_proto::geyser::{subscribe_update::UpdateOneof, SlotStatus};
use yellowstone_vixen_block_coordinator::{
    BlockMachineCoordinator, ConfirmedSlot, CoordinatorInput, CoordinatorMessage,
    FixtureReader, extract_coordinator_inputs,
};

fn fixture_path() -> PathBuf {
    env::var("FIXTURE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sample.bin"))
}

// =============================================================================
// Extraction Test
// =============================================================================

#[derive(Default)]
struct ExtractionStats {
    total: u64,
    entries: u64,
    slots: u64,
    block_metas: u64,
    skipped: u64,
    lifecycle: LifecycleStats,
}

#[derive(Default)]
struct LifecycleStats {
    first_shred: u64,
    created_bank: u64,
    completed: u64,
    dead: u64,
    processed: u64,
    confirmed: u64,
    finalized: u64,
}

#[test]
fn extraction_values_are_correct() {
    let reader = FixtureReader::new(&fixture_path()).expect("Failed to open fixture");
    let mut stats = ExtractionStats::default();

    for update in reader {
        stats.total += 1;
        let inputs = extract_coordinator_inputs(&update);

        let Some(ref oneof) = update.update_oneof else {
            stats.skipped += 1;
            continue;
        };

        match oneof {
            UpdateOneof::Entry(raw) => {
                stats.entries += 1;
                assert_entry_extraction(&inputs, raw);
            },
            UpdateOneof::Slot(raw) => {
                stats.slots += 1;
                stats.lifecycle.track(raw.status());
                assert_slot_extraction(&inputs, raw);
            },
            UpdateOneof::BlockMeta(raw) => {
                stats.block_metas += 1;
                assert_block_meta_extraction(&inputs, raw);
            },
            _ => {
                stats.skipped += 1;
                assert!(inputs.is_empty(), "Unknown event type should produce no inputs");
            },
        }
    }

    assert_expected_counts(&stats);
}

fn assert_entry_extraction(
    inputs: &[CoordinatorInput],
    raw: &yellowstone_grpc_proto::geyser::SubscribeUpdateEntry,
) {
    assert_eq!(inputs.len(), 1);
    let CoordinatorInput::Replay(BlockReplayEvent::Entry(info)) = &inputs[0] else {
        panic!("Expected Entry input");
    };

    assert_eq!(info.slot, raw.slot);
    assert_eq!(info.entry_index, raw.index);
    assert_eq!(info.executed_txn_count, raw.executed_transaction_count);
    assert_eq!(info.starting_txn_index, raw.starting_transaction_index);
    assert_eq!(info.entry_hash.as_ref(), raw.hash.as_slice());
}

fn assert_slot_extraction(
    inputs: &[CoordinatorInput],
    raw: &yellowstone_grpc_proto::geyser::SubscribeUpdateSlot,
) {
    let is_lifecycle = matches!(
        raw.status(),
        SlotStatus::SlotFirstShredReceived
            | SlotStatus::SlotCreatedBank
            | SlotStatus::SlotCompleted
            | SlotStatus::SlotDead
    );

    if is_lifecycle {
        assert_lifecycle_input(inputs, raw);
    } else if raw.dead_error.is_some() {
        assert_dead_error_input(inputs, raw);
    } else {
        assert_consensus_input(inputs, raw);
    }
}

fn assert_lifecycle_input(
    inputs: &[CoordinatorInput],
    raw: &yellowstone_grpc_proto::geyser::SubscribeUpdateSlot,
) {
    assert_eq!(inputs.len(), 1);
    let CoordinatorInput::Replay(BlockReplayEvent::SlotLifecycleStatus(lifecycle)) = &inputs[0] else {
        panic!("Expected SlotLifecycle input");
    };

    assert_eq!(lifecycle.slot, raw.slot);
    assert_eq!(lifecycle.parent_slot, raw.parent);

    let expected_stage = match raw.status() {
        SlotStatus::SlotFirstShredReceived => SlotLifecycle::FirstShredReceived,
        SlotStatus::SlotCreatedBank => SlotLifecycle::CreatedBank,
        SlotStatus::SlotCompleted => SlotLifecycle::Completed,
        SlotStatus::SlotDead => SlotLifecycle::Dead,
        _ => unreachable!(),
    };
    assert_eq!(lifecycle.stage, expected_stage);
}

fn assert_dead_error_input(
    inputs: &[CoordinatorInput],
    _raw: &yellowstone_grpc_proto::geyser::SubscribeUpdateSlot,
) {
    assert_eq!(inputs.len(), 1);
    let CoordinatorInput::Replay(BlockReplayEvent::SlotLifecycleStatus(lifecycle)) = &inputs[0] else {
        panic!("Expected Dead lifecycle for dead_error slot");
    };
    assert_eq!(lifecycle.stage, SlotLifecycle::Dead);
}

fn assert_consensus_input(
    inputs: &[CoordinatorInput],
    raw: &yellowstone_grpc_proto::geyser::SubscribeUpdateSlot,
) {
    let expected = match raw.status() {
        SlotStatus::SlotProcessed => Some(CommitmentLevel::Processed),
        SlotStatus::SlotConfirmed => Some(CommitmentLevel::Confirmed),
        SlotStatus::SlotFinalized => Some(CommitmentLevel::Finalized),
        _ => None,
    };

    match expected {
        Some(commitment) => {
            assert_eq!(inputs.len(), 1);
            let CoordinatorInput::Consensus(ConsensusUpdate::SlotCommitmentStatus(status)) = &inputs[0] else {
                panic!("Expected Consensus input");
            };
            assert_eq!(status.slot, raw.slot);
            assert_eq!(status.commitment, commitment);
        },
        None => assert!(inputs.is_empty()),
    }
}

fn assert_block_meta_extraction(
    inputs: &[CoordinatorInput],
    raw: &yellowstone_grpc_proto::geyser::SubscribeUpdateBlockMeta,
) {
    assert_eq!(inputs.len(), 2, "BlockMeta should produce BlockSummary + BlockExtra");

    let CoordinatorInput::Replay(BlockReplayEvent::BlockSummary(summary)) = &inputs[0] else {
        panic!("Expected BlockSummary as first input");
    };
    assert_eq!(summary.slot, raw.slot);
    assert_eq!(summary.parent_slot, raw.parent_slot);
    assert_eq!(summary.entry_count, raw.entries_count);
    assert_eq!(summary.executed_transaction_count, raw.executed_transaction_count);

    let expected_hash = bs58::decode(&raw.blockhash).into_vec().unwrap();
    assert_eq!(summary.blockhash.as_ref(), expected_hash.as_slice());

    let CoordinatorInput::BlockExtra { slot, block_time, block_height } = &inputs[1] else {
        panic!("Expected BlockExtra as second input");
    };
    assert_eq!(*slot, raw.slot);
    assert_eq!(*block_time, raw.block_time.as_ref().map(|bt| bt.timestamp));
    assert_eq!(*block_height, raw.block_height.as_ref().map(|bh| bh.block_height));
}

fn assert_expected_counts(stats: &ExtractionStats) {
    assert!(stats.total > 0, "Fixture should not be empty");
    assert_eq!(stats.total, 40186, "Total message count");
    assert_eq!(stats.entries, 39840, "Entry count");
    assert_eq!(stats.slots, 295, "Slot update count");
    assert_eq!(stats.block_metas, 50, "BlockMeta count");
    assert_eq!(stats.skipped, 1, "Skipped count");
}

impl LifecycleStats {
    #[allow(unreachable_patterns)]
    fn track(&mut self, status: SlotStatus) {
        match status {
            SlotStatus::SlotFirstShredReceived => self.first_shred += 1,
            SlotStatus::SlotCreatedBank => self.created_bank += 1,
            SlotStatus::SlotCompleted => self.completed += 1,
            SlotStatus::SlotDead => self.dead += 1,
            SlotStatus::SlotProcessed => self.processed += 1,
            SlotStatus::SlotConfirmed => self.confirmed += 1,
            SlotStatus::SlotFinalized => self.finalized += 1,
            _ => {},
        }
    }
}

// =============================================================================
// Coordinator Replay Test
// =============================================================================

#[tokio::test]
async fn replay_fixture_through_coordinator() {
    let updates: Vec<_> = FixtureReader::new(&fixture_path())
        .expect("Failed to open fixture")
        .collect();

    let expected = ExpectedState::from_updates(&updates);
    let confirmed = run_coordinator_with_updates(&updates).await;

    assert!(!confirmed.is_empty(), "Should produce confirmed slots");
    assert_strictly_ascending(&confirmed);

    for slot in &confirmed {
        assert_slot_metadata(slot, &expected);
    }
}

struct ExpectedState {
    tx_counts: HashMap<u64, u64>,
    parents: HashMap<u64, u64>,
}

impl ExpectedState {
    fn from_updates(updates: &[yellowstone_grpc_proto::geyser::SubscribeUpdate]) -> Self {
        let mut tx_counts: HashMap<u64, u64> = HashMap::new();
        let mut parents: HashMap<u64, u64> = HashMap::new();

        for update in updates {
            let Some(ref oneof) = update.update_oneof else { continue };
            match oneof {
                UpdateOneof::Entry(e) => {
                    *tx_counts.entry(e.slot).or_default() += e.executed_transaction_count;
                },
                UpdateOneof::BlockMeta(m) => {
                    parents.insert(m.slot, m.parent_slot);
                },
                _ => {},
            }
        }

        Self { tx_counts, parents }
    }
}

async fn run_coordinator_with_updates(
    updates: &[yellowstone_grpc_proto::geyser::SubscribeUpdate],
) -> Vec<ConfirmedSlot<()>> {
    let (input_tx, input_rx) = mpsc::channel::<CoordinatorInput>(4096);
    let (parsed_tx, parsed_rx) = mpsc::channel::<CoordinatorMessage<()>>(4096);
    let (output_tx, mut output_rx) = mpsc::channel::<ConfirmedSlot<()>>(256);

    tokio::spawn(BlockMachineCoordinator::new(input_rx, parsed_rx, output_tx).run());

    for update in updates {
        for input in extract_coordinator_inputs(update) {
            let block_summary = match &input {
                CoordinatorInput::Replay(BlockReplayEvent::BlockSummary(s)) => {
                    Some((s.slot, s.executed_transaction_count))
                },
                _ => None,
            };

            input_tx.send(input).await.unwrap();

            if let Some((slot, tx_count)) = block_summary {
                for _ in 0..tx_count {
                    parsed_tx.send(CoordinatorMessage::TransactionParsed { slot }).await.unwrap();
                }
            }
        }
    }

    drop(input_tx);
    drop(parsed_tx);

    let mut confirmed = Vec::new();
    while let Some(slot) = output_rx.recv().await {
        confirmed.push(slot);
    }
    confirmed
}

fn assert_strictly_ascending(slots: &[ConfirmedSlot<()>]) {
    for pair in slots.windows(2) {
        assert!(pair[0].slot < pair[1].slot, "Slots must be ascending: {} >= {}", pair[0].slot, pair[1].slot);
    }
}

fn assert_slot_metadata(slot: &ConfirmedSlot<()>, expected: &ExpectedState) {
    assert_ne!(slot.blockhash, solana_hash::Hash::default(), "Slot {} has zero blockhash", slot.slot);
    assert!(slot.block_time.is_some(), "Slot {} missing block_time", slot.slot);
    assert!(slot.block_height.is_some(), "Slot {} missing block_height", slot.slot);

    if let Some(&expected_tx) = expected.tx_counts.get(&slot.slot) {
        assert_eq!(slot.executed_transaction_count, expected_tx, "Slot {} tx count mismatch", slot.slot);
    }

    if let Some(&expected_parent) = expected.parents.get(&slot.slot) {
        assert_eq!(slot.parent_slot, expected_parent, "Slot {} parent mismatch", slot.slot);
    }
}
