//! Integration tests for BlockMachineCoordinator.
//!
//! ## Complete State Machine
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │                        COORDINATOR STATE MACHINE                            │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │  STATE:                                                                     │
//! │    buffer: BTreeMap<Slot, SlotBuffer>     // pending slots                  │
//! │    discarded_slots: BTreeSet<Slot>        // dead/forked/untracked          │
//! │    last_flushed_slot: Option<Slot>        // for gap detection              │
//! │                                                                             │
//! │  SLOT BUFFER:                                                               │
//! │    expected_tx_count: Option<u64>         // from FrozenBlock               │
//! │    parsed_tx_count: u64                   // from TransactionParsed         │
//! │    confirmed: bool                        // from Confirmed event           │
//! │    records: BTreeMap<RecordSortKey, R>    // from Parsed messages           │
//! │    parent_slot, blockhash                                                   │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │                           INPUT PROCESSING                                  │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │  ┌─────────────────┐                                                        │
//! │  │ Parsed Message  │                                                        │
//! │  └────────┬────────┘                                                        │
//! │           │                                                                 │
//! │           ▼                                                                 │
//! │  ┌────────────────────┐  YES   ┌─────────────────────────────────┐          │
//! │  │ slot in discarded? ├───────►│ DROP (dead/forked/untracked)    │          │
//! │  └────────┬───────────┘        │ [discarded_slot_ignores_parsed] │          │
//! │           │ NO                 └─────────────────────────────────┘          │
//! │           ▼                                                                 │
//! │  ┌────────────────────┐  YES   ┌─────────────────────────────────┐          │
//! │  │ slot <= last_flush?├───────►│ PANIC (two-gate invariant!)     │          │
//! │  └────────┬───────────┘        │ Should never happen if healthy  │          │
//! │           │ NO                 └─────────────────────────────────┘          │
//! │           ▼                                                                 │
//! │  ┌────────────────────┐                                                     │
//! │  │ Buffer the message │        [parsed_before_lifecycle_buffered]           │
//! │  │ (creates slot if   │        [two_gate_flush_end_to_end]                  │
//! │  │  not exists)       │                                                     │
//! │  └────────────────────┘                                                     │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │                           FLUSH DECISION TREE                               │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │  drain_flushable() — called once per event batch in run()                   │
//! │                                                                             │
//! │  FOR each slot in buffer (ascending order):                                 │
//! │                                                                             │
//! │  ┌──────────────────────────────────────────┐                               │
//! │  │ GATE 1: is_fully_parsed?                 │                               │
//! │  │ (parsed_tx_count >= expected_tx_count)   │                               │
//! │  └─────────────────┬────────────────────────┘                               │
//! │                    │                                                        │
//! │         NO ◄───────┴───────► YES                                            │
//! │         │                    │                                              │
//! │         ▼                    ▼                                              │
//! │  ┌─────────────┐    ┌──────────────────────────────────┐                    │
//! │  │ STOP        │    │ GATE 2: confirmed?               │                    │
//! │  │ [incomplete │    └─────────────────┬────────────────┘                    │
//! │  │  _blocks_   │                      │                                     │
//! │  │  subsequent]│           NO ◄───────┴───────► YES                         │
//! │  └─────────────┘           │                    │                           │
//! │                            ▼                    ▼                           │
//! │                     ┌─────────────┐    ┌──────────────────────────────────┐ │
//! │                     │ STOP        │    │ GAP CHECK: parent flushed        │ │
//! │                     │ [sequential │    │ or discarded?                    │ │
//! │                     │  _flush_    │    └─────────────────┬────────────────┘ │
//! │                     │  order]     │                      │                  │
//! │                     └─────────────┘           NO ◄───────┴───────► YES      │
//! │                                              │                    │         │
//! │                                              ▼                    ▼         │
//! │                                       ┌─────────────┐    ┌──────────────┐   │
//! │                                       │ STOP        │    │ FLUSH SLOT   │   │
//! │                                       │ [gap_in_    │    │ Update       │   │
//! │                                       │  sequence_  │    │ last_flushed │   │
//! │                                       │  blocks]    │    │ Continue loop│   │
//! │                                       └─────────────┘    └──────────────┘   │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │                           DISCARD SCENARIOS                                 │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │  1. DEAD SLOT                          [dead_slot_discarded]                │
//! │     SlotLifecycle::Dead ──► ForksDetected ──► discard_slot()                │
//! │                                                                             │
//! │  2. FORKED SLOT                        [sibling_fork_via_finalized]         │
//! │     Sibling finalized ──► ForksDetected ──► discard_slot()                  │
//! │                                                                             │
//! │  3. UNTRACKED SLOT                     [untracked_slot_discarded]           │
//! │     BlockSummary rejected by BlockSM ──► discard_slot()                     │
//! │                                                                             │
//! │  discard_slot() actions:                                                    │
//! │    • Add to discarded_slots set                                             │
//! │    • Remove from buffer                                                     │
//! │    • drain_flushable called in run() after event                            │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │                           TEST COVERAGE MATRIX                              │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │  TWO-GATE SYSTEM:                                                           │
//! │    ✓ two_gate_flush_end_to_end      Gate 1 + Gate 2 both required           │
//! │    ✓ empty_slot_flushes             Gate 1 satisfied with 0 transactions    │
//! │    ✓ incomplete_slot_blocks_subsequent  Gate 1 not satisfied blocks flush   │
//! │                                                                             │
//! │  SEQUENTIAL ORDERING:                                                       │
//! │    ✓ sequential_flush_order         Earlier slot blocks later ones          │
//! │    ✓ gap_in_sequence_blocks_flush   Missing parent blocks child             │
//! │                                                                             │
//! │  DISCARD HANDLING:                                                          │
//! │    ✓ dead_slot_discarded            Dead slot removed, no output            │
//! │    ✓ dead_slot_discards_descendants Killing ancestor discards whole chain  │
//! │    ✓ dead_slot_unblocks_next        Dead sibling unblocks subsequent slot   │
//! │    ✓ untracked_slot_discarded       Rejected BlockSummary causes discard    │
//! │    ✓ discarded_slot_ignores_parsed  Messages for discarded slot dropped     │
//! │                                                                             │
//! │  FORK HANDLING:                                                             │
//! │    ✓ sibling_fork_via_finalized     Finalizing one sibling forks other      │
//! │                                                                             │
//! │  EDGE CASES:                                                                │
//! │    ✓ parsed_before_lifecycle_buffered  Early messages preserved             │
//! │    ✓ double_confirmation_is_idempotent Confirming twice is safe             │
//! │                                                                             │
//! │  INVARIANT VIOLATION (panic tests):                                         │
//! │    ✓ late_message_for_flushed_slot_errors  Detects two-gate bug             │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Fork Detection Details
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │                         Fork Detection Triggers                             │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │  1. Dead Slot                        2. Sibling Finalization                │
//! │  ───────────                         ──────────────────────                 │
//! │                                                                             │
//! │      Parent                              Parent (rooted)                    │
//! │         │                                    │                              │
//! │    ┌────┴────┐                          ┌────┴────┐                         │
//! │    ▼         ▼                          ▼         ▼                         │
//! │  Slot A   Slot B                    Slot A    Slot B                        │
//! │  (dead)   (ok)                   (finalized)  (forked!)                     │
//! │    │                                             │                          │
//! │    ▼                                             ▼                          │
//! │  ForksDetected(A)                          ForksDetected(B)                 │
//! │                                                                             │
//! │  Note: In practice, confirmed slots won't be forked because confirmation    │
//! │  requires supermajority (2/3+) votes — siblings can't both get enough.      │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//! ```

use std::time::Duration;

use solana_hash::Hash;
use tokio::sync::mpsc;
use yellowstone_grpc_proto::{
    geyser::{
        subscribe_update::UpdateOneof, SlotStatus, SubscribeUpdate, SubscribeUpdateBlockMeta,
        SubscribeUpdateEntry, SubscribeUpdateSlot,
    },
    prelude::{BlockHeight, UnixTimestamp},
};
use yellowstone_vixen_block_coordinator::{
    BlockMachineCoordinator, ConfirmedSlot, CoordinatorError, CoordinatorMessage, RecordSortKey,
};

// =============================================================================
// Test Harness
// =============================================================================

struct TestHarness {
    input_tx: mpsc::Sender<SubscribeUpdate>,
    parsed_tx: mpsc::Sender<CoordinatorMessage<String>>,
    output_rx: mpsc::Receiver<ConfirmedSlot<String>>,
}

impl TestHarness {
    fn spawn() -> Self {
        let (input_tx, input_rx) = mpsc::channel(256);
        let (parsed_tx, parsed_rx) = mpsc::channel(256);
        let (output_tx, output_rx) = mpsc::channel(64);

        let coordinator = BlockMachineCoordinator::new(input_rx, parsed_rx, output_tx);
        tokio::spawn(coordinator.run());

        Self {
            input_tx,
            parsed_tx,
            output_rx,
        }
    }

    fn slot(&self, slot: u64) -> SlotBuilder {
        SlotBuilder::new(self.input_tx.clone(), self.parsed_tx.clone(), slot)
    }

    async fn send_orphan_block_summary(&self, slot: u64, parent: u64) {
        let blockhash = Hash::new_unique();
        self.input_tx
            .send(make_block_meta_update(slot, parent, 1, &blockhash))
            .await
            .unwrap();
    }

    async fn expect_flush(&mut self, slot: u64) -> FlushAssertion {
        let confirmed = tokio::time::timeout(Duration::from_secs(2), self.output_rx.recv())
            .await
            .expect("Timed out waiting for flush")
            .expect("Channel closed");

        assert_eq!(confirmed.slot, slot, "Expected slot {slot} to flush");
        FlushAssertion(confirmed)
    }

    async fn expect_no_flush(&mut self) {
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert!(self.output_rx.try_recv().is_err(), "Unexpected flush");
    }
}

// =============================================================================
// SubscribeUpdate Builders
// =============================================================================

fn make_slot_update(slot: u64, parent: u64, status: SlotStatus) -> SubscribeUpdate {
    SubscribeUpdate {
        filters: vec![],
        created_at: None,
        update_oneof: Some(UpdateOneof::Slot(SubscribeUpdateSlot {
            slot,
            parent: Some(parent),
            status: status.into(),
            dead_error: None,
        })),
    }
}

fn make_entry_update(slot: u64, index: u64, tx_count: u64) -> SubscribeUpdate {
    SubscribeUpdate {
        filters: vec![],
        created_at: None,
        update_oneof: Some(UpdateOneof::Entry(SubscribeUpdateEntry {
            slot,
            index,
            num_hashes: 1,
            hash: Hash::new_unique().to_bytes().to_vec(),
            executed_transaction_count: tx_count,
            starting_transaction_index: 0,
        })),
    }
}

fn make_block_meta_update(
    slot: u64,
    parent: u64,
    tx_count: u64,
    blockhash: &Hash,
) -> SubscribeUpdate {
    SubscribeUpdate {
        filters: vec![],
        created_at: None,
        update_oneof: Some(UpdateOneof::BlockMeta(SubscribeUpdateBlockMeta {
            slot,
            blockhash: bs58::encode(blockhash.as_ref()).into_string(),
            rewards: None,
            block_time: Some(UnixTimestamp {
                timestamp: 1700000000,
            }),
            block_height: Some(BlockHeight {
                block_height: slot - 1,
            }),
            parent_slot: parent,
            parent_blockhash: bs58::encode(Hash::default().as_ref()).into_string(),
            executed_transaction_count: tx_count,
            entries_count: 1,
        })),
    }
}

// =============================================================================
// Flush Assertion
// =============================================================================

struct FlushAssertion(ConfirmedSlot<String>);

impl FlushAssertion {
    fn records(self, expected: &[&str]) {
        let expected: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
        assert_eq!(self.0.records, expected, "Records mismatch");
    }

    fn tx_count(self, expected: u64) -> Self {
        assert_eq!(
            self.0.executed_transaction_count, expected,
            "Tx count mismatch"
        );
        self
    }

    fn parent(self, expected: u64) -> Self {
        assert_eq!(self.0.parent_slot, expected, "Parent mismatch");
        self
    }

    fn empty(self) {
        assert!(self.0.records.is_empty(), "Expected no records");
        assert_eq!(self.0.executed_transaction_count, 0, "Tx count mismatch");
    }
}

// =============================================================================
// Slot Builder
// =============================================================================

struct SlotBuilder {
    input_tx: mpsc::Sender<SubscribeUpdate>,
    parsed_tx: mpsc::Sender<CoordinatorMessage<String>>,
    slot: u64,
    parent: u64,
    records: Vec<(RecordSortKey, String)>,
}

impl SlotBuilder {
    fn new(
        input_tx: mpsc::Sender<SubscribeUpdate>,
        parsed_tx: mpsc::Sender<CoordinatorMessage<String>>,
        slot: u64,
    ) -> Self {
        Self {
            input_tx,
            parsed_tx,
            slot,
            parent: slot.saturating_sub(1),
            records: vec![],
        }
    }

    fn parent(mut self, parent: u64) -> Self {
        self.parent = parent;
        self
    }

    fn record(mut self, value: &str) -> Self {
        let tx_index = self.records.len() as u64;
        self.records
            .push((RecordSortKey::new(tx_index, vec![0]), value.to_string()));
        self
    }

    fn record_at(mut self, tx_index: u64, ix_path: Vec<usize>, value: &str) -> Self {
        self.records
            .push((RecordSortKey::new(tx_index, ix_path), value.to_string()));
        self
    }

    async fn empty(self) -> Slot { self.send_lifecycle(0).await }

    async fn parsed(self) -> Slot {
        let tx_count = self.records.len().max(1) as u64;
        let slot = self.send_lifecycle(tx_count).await;
        tokio::time::sleep(Duration::from_millis(5)).await;
        slot
    }

    async fn pending(self, expected_tx: u64) -> Slot {
        self.send_lifecycle_without_parsed(expected_tx).await
    }

    /// Send all block lifecycle events (FirstShredReceived, CreatedBank, Entry, Completed, BlockMeta).
    async fn send_block_events(&self, tx_count: u64) {
        let blockhash = Hash::new_unique();

        for status in [
            SlotStatus::SlotFirstShredReceived,
            SlotStatus::SlotCreatedBank,
        ] {
            self.input_tx
                .send(make_slot_update(self.slot, self.parent, status))
                .await
                .unwrap();
        }

        self.input_tx
            .send(make_entry_update(self.slot, 0, tx_count))
            .await
            .unwrap();

        self.input_tx
            .send(make_slot_update(
                self.slot,
                self.parent,
                SlotStatus::SlotCompleted,
            ))
            .await
            .unwrap();

        self.input_tx
            .send(make_block_meta_update(
                self.slot,
                self.parent,
                tx_count,
                &blockhash,
            ))
            .await
            .unwrap();
    }

    async fn send_lifecycle_without_parsed(self, tx_count: u64) -> Slot {
        self.send_block_events(tx_count).await;
        tokio::time::sleep(Duration::from_millis(5)).await;

        Slot {
            input_tx: self.input_tx,
            parsed_tx: self.parsed_tx,
            slot: self.slot,
            parent: self.parent,
            next_tx_index: std::sync::atomic::AtomicU64::new(0),
        }
    }

    async fn send_lifecycle(self, tx_count: u64) -> Slot {
        self.send_block_events(tx_count).await;

        // Send parsed records
        for (key, record) in &self.records {
            self.parsed_tx
                .send(CoordinatorMessage::Parsed {
                    slot: self.slot,
                    key: key.clone(),
                    record: record.clone(),
                })
                .await
                .unwrap();
        }

        // Send TransactionParsed signals
        for _ in 0..tx_count {
            self.parsed_tx
                .send(CoordinatorMessage::TransactionParsed { slot: self.slot })
                .await
                .unwrap();
        }

        Slot {
            input_tx: self.input_tx,
            parsed_tx: self.parsed_tx,
            slot: self.slot,
            parent: self.parent,
            next_tx_index: std::sync::atomic::AtomicU64::new(tx_count),
        }
    }
}

// =============================================================================
// Slot (post-lifecycle handle)
// =============================================================================

struct Slot {
    input_tx: mpsc::Sender<SubscribeUpdate>,
    parsed_tx: mpsc::Sender<CoordinatorMessage<String>>,
    slot: u64,
    parent: u64,
    next_tx_index: std::sync::atomic::AtomicU64,
}

impl Slot {
    async fn confirm(&self) {
        tokio::time::sleep(Duration::from_millis(5)).await;
        self.send_commitment(SlotStatus::SlotConfirmed).await;
    }

    async fn finalize(&self) {
        tokio::time::sleep(Duration::from_millis(5)).await;
        self.send_commitment(SlotStatus::SlotFinalized).await;
    }

    async fn send_commitment(&self, status: SlotStatus) {
        self.input_tx
            .send(make_slot_update(self.slot, self.parent, status))
            .await
            .unwrap();
    }

    async fn kill(&self) {
        self.input_tx
            .send(make_slot_update(
                self.slot,
                self.parent,
                SlotStatus::SlotDead,
            ))
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(5)).await;
    }

    async fn late_record(&self, value: &str) {
        let tx_index = self
            .next_tx_index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.parsed_tx
            .send(CoordinatorMessage::Parsed {
                slot: self.slot,
                key: RecordSortKey::new(tx_index, vec![0]),
                record: value.to_string(),
            })
            .await
            .unwrap();
        self.parsed_tx
            .send(CoordinatorMessage::TransactionParsed { slot: self.slot })
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
}

// Standalone helper for sending records to arbitrary slots (no Slot handle needed)
async fn send_record_to_slot(
    parsed_tx: &mpsc::Sender<CoordinatorMessage<String>>,
    slot: u64,
    value: &str,
) {
    parsed_tx
        .send(CoordinatorMessage::Parsed {
            slot,
            key: RecordSortKey::new(0, vec![0]),
            record: value.to_string(),
        })
        .await
        .unwrap();
    parsed_tx
        .send(CoordinatorMessage::TransactionParsed { slot })
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(5)).await;
}

// =============================================================================
// Tests
// =============================================================================

#[tokio::test]
async fn two_gate_flush_end_to_end() {
    let mut harness = TestHarness::spawn();

    let slot = harness
        .slot(100)
        .parent(99)
        .record_at(1, vec![0], "tx1-ix0")
        .record_at(0, vec![0, 1], "tx0-cpi")
        .record_at(0, vec![0], "tx0-ix0")
        .send_lifecycle(2)
        .await;

    harness.expect_no_flush().await;
    slot.confirm().await;

    harness
        .expect_flush(100)
        .await
        .parent(99)
        .tx_count(2)
        .records(&["tx0-ix0", "tx0-cpi", "tx1-ix0"]);
}

#[tokio::test]
async fn sequential_flush_order() {
    let mut harness = TestHarness::spawn();

    let slot_101 = harness
        .slot(101)
        .parent(100)
        .record("rec-101")
        .parsed()
        .await;
    let slot_102 = harness
        .slot(102)
        .parent(101)
        .record("rec-102")
        .parsed()
        .await;

    slot_102.confirm().await;
    harness.expect_no_flush().await;

    slot_101.confirm().await;

    harness.expect_flush(101).await.records(&["rec-101"]);
    harness.expect_flush(102).await.records(&["rec-102"]);
}

#[tokio::test]
async fn dead_slot_discarded() {
    let mut harness = TestHarness::spawn();

    let slot = harness
        .slot(100)
        .parent(99)
        .record("will-die")
        .parsed()
        .await;
    slot.kill().await;

    harness.expect_no_flush().await;
}

#[tokio::test]
async fn dead_slot_unblocks_next() {
    let mut harness = TestHarness::spawn();

    // Parent slot — confirmed and flushed first.
    let parent = harness.slot(100).parent(99).empty().await;
    parent.confirm().await;
    harness.expect_flush(100).await;

    // Two siblings (same parent=100). Killing one should NOT propagate to the other.
    let blocker = harness
        .slot(101)
        .parent(100)
        .record("blocker")
        .parsed()
        .await;
    let waiting = harness
        .slot(102)
        .parent(100)
        .record("survives")
        .parsed()
        .await;

    waiting.confirm().await;
    harness.expect_no_flush().await;

    blocker.kill().await;

    harness.expect_flush(102).await.records(&["survives"]);
}

#[tokio::test]
async fn dead_slot_discards_descendants() {
    let mut harness = TestHarness::spawn();

    // Chain: 100 → 101 → 102. Killing 100 should discard 101 and 102 too.
    let ancestor = harness.slot(100).parent(99).record("root").parsed().await;
    let child = harness.slot(101).parent(100).record("child").parsed().await;
    let grandchild = harness
        .slot(102)
        .parent(101)
        .record("grandchild")
        .parsed()
        .await;

    child.confirm().await;
    grandchild.confirm().await;

    // Both descendants are ready, but killing the ancestor should discard the whole chain.
    ancestor.kill().await;

    harness.expect_no_flush().await;
}

#[tokio::test]
async fn empty_slot_flushes() {
    let mut harness = TestHarness::spawn();

    let slot = harness.slot(100).parent(99).empty().await;
    slot.confirm().await;

    harness.expect_flush(100).await.empty();
}

#[tokio::test]
async fn untracked_slot_discarded() {
    let mut harness = TestHarness::spawn();

    harness.send_orphan_block_summary(100, 99).await;
    harness.expect_no_flush().await;

    let next = harness.slot(101).parent(100).empty().await;
    next.confirm().await;

    harness.expect_flush(101).await;
}

#[tokio::test]
async fn discarded_slot_ignores_parsed_messages() {
    let mut harness = TestHarness::spawn();

    let doomed = harness.slot(100).parent(99).record("doomed").parsed().await;
    let survivor = harness.slot(101).parent(99).empty().await;

    doomed.kill().await;
    doomed.late_record("should-be-ignored").await;
    harness.expect_no_flush().await;

    survivor.late_record("valid").await;
    survivor.confirm().await;

    harness.expect_flush(101).await.records(&["valid"]);
}

#[tokio::test]
async fn sibling_fork_via_finalized() {
    let mut harness = TestHarness::spawn();

    let parent = harness.slot(100).parent(99).empty().await;
    parent.confirm().await;

    let winner = harness
        .slot(101)
        .parent(100)
        .record("winner")
        .parsed()
        .await;
    let loser = harness.slot(102).parent(100).record("loser").parsed().await;

    winner.confirm().await;
    harness.expect_flush(100).await;
    harness.expect_flush(101).await.records(&["winner"]);

    winner.finalize().await;
    loser.confirm().await;

    harness.expect_no_flush().await;
}

// =============================================================================
// Edge Case Tests
// =============================================================================

#[tokio::test]
async fn parsed_messages_before_lifecycle_are_buffered() {
    let mut harness = TestHarness::spawn();

    send_record_to_slot(&harness.parsed_tx, 100, "early-bird").await;

    let slot = harness.slot(100).parent(99).empty().await;
    slot.confirm().await;

    harness.expect_flush(100).await.records(&["early-bird"]);
}

#[tokio::test]
async fn double_confirmation_is_idempotent() {
    let mut harness = TestHarness::spawn();

    let slot = harness.slot(100).parent(99).record("data").parsed().await;

    slot.confirm().await;
    harness.expect_flush(100).await.records(&["data"]);

    slot.confirm().await;
    harness.expect_no_flush().await;
}

#[tokio::test]
async fn incomplete_slot_blocks_subsequent() {
    let mut harness = TestHarness::spawn();

    let incomplete = harness.slot(100).parent(99).pending(3).await;
    let complete = harness.slot(101).parent(100).record("ready").parsed().await;

    complete.confirm().await;
    harness.expect_no_flush().await;

    incomplete.late_record("a").await;
    incomplete.late_record("b").await;
    incomplete.late_record("c").await;
    incomplete.confirm().await;

    harness.expect_flush(100).await.records(&["a", "b", "c"]);
    harness.expect_flush(101).await.records(&["ready"]);
}

#[tokio::test]
async fn gap_in_sequence_blocks_flush() {
    let mut harness = TestHarness::spawn();

    let slot_100 = harness.slot(100).parent(99).empty().await;
    let slot_102 = harness
        .slot(102)
        .parent(101)
        .record("waiting")
        .parsed()
        .await;

    slot_100.confirm().await;
    harness.expect_flush(100).await;

    slot_102.confirm().await;
    harness.expect_no_flush().await;

    let slot_101 = harness.slot(101).parent(100).empty().await;
    slot_101.confirm().await;

    harness.expect_flush(101).await;
    harness.expect_flush(102).await.records(&["waiting"]);
}

#[tokio::test]
async fn late_message_for_flushed_slot_errors() {
    let (input_tx, input_rx) = mpsc::channel(256);
    let (parsed_tx, parsed_rx) = mpsc::channel(256);
    let (output_tx, mut output_rx) = mpsc::channel(64);

    let coordinator = BlockMachineCoordinator::new(input_rx, parsed_rx, output_tx);
    let handle = tokio::spawn(coordinator.run());

    // Create and flush slot 100
    let slot = SlotBuilder::new(input_tx.clone(), parsed_tx.clone(), 100)
        .parent(99)
        .empty()
        .await;

    slot.confirm().await;

    let flushed = tokio::time::timeout(Duration::from_secs(2), output_rx.recv())
        .await
        .expect("Timed out")
        .expect("Channel closed");
    assert_eq!(flushed.slot, 100);

    // Send late message - this should return an error
    send_record_to_slot(&parsed_tx, 100, "too-late").await;

    // Wait for coordinator task to complete (it should return an error)
    let result = handle.await.expect("task join");
    assert!(
        matches!(
            result,
            Err(CoordinatorError::TwoGateInvariantViolation { .. })
        ),
        "Expected TwoGateInvariantViolation, got: {result:?}"
    );
}

#[tokio::test]
async fn output_channel_closed_returns_error() {
    let (input_tx, input_rx) = mpsc::channel(256);
    let (parsed_tx, parsed_rx) = mpsc::channel(256);
    let (output_tx, output_rx) = mpsc::channel(64);

    let coordinator = BlockMachineCoordinator::new(input_rx, parsed_rx, output_tx);
    let handle = tokio::spawn(coordinator.run());

    // Drop the output receiver — the coordinator can't send flushed slots.
    drop(output_rx);

    // Create a ready slot that will try to flush.
    let slot = SlotBuilder::new(input_tx.clone(), parsed_tx.clone(), 100)
        .parent(99)
        .empty()
        .await;

    slot.confirm().await;

    let result = handle.await.expect("task join");
    assert!(
        matches!(
            result,
            Err(CoordinatorError::OutputChannelClosed { slot: 100 })
        ),
        "Expected OutputChannelClosed, got: {result:?}"
    );
}
