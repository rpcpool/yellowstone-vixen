# Block Coordinator Tests

## Test Files

| File | Purpose |
|------|---------|
| `integration.rs` | Synthetic tests for all coordinator behaviors |
| `replay_fixtures.rs` | Real data validation using captured geyser data |

## Test Coverage

### Two-Gate System
| Test | Verifies |
|------|----------|
| `two_gate_flush_end_to_end` | Both gates required, records sorted |
| `empty_slot_flushes` | Gate 1 satisfied with 0 transactions |
| `incomplete_slot_blocks_subsequent` | Incomplete slot blocks flush |

### Sequential Ordering
| Test | Verifies |
|------|----------|
| `sequential_flush_order` | Earlier slot blocks later ones |
| `gap_in_sequence_blocks_flush` | Missing parent blocks child |

### Discard Handling
| Test | Verifies |
|------|----------|
| `dead_slot_discarded` | Dead slot removed, no output |
| `dead_slot_unblocks_next` | Discard unblocks subsequent slot |
| `untracked_slot_discarded` | Rejected BlockSummary causes discard |
| `discarded_slot_ignores_parsed_messages` | Messages for discarded slot dropped |

### Fork Handling
| Test | Verifies |
|------|----------|
| `sibling_fork_via_finalized` | Finalizing one sibling forks other |

### Edge Cases
| Test | Verifies |
|------|----------|
| `parsed_messages_before_lifecycle_are_buffered` | Early messages preserved |
| `double_confirmation_is_idempotent` | Confirming twice is safe |

### Invariant Violation (panic tests)
| Test | Verifies |
|------|----------|
| `late_message_for_flushed_slot_panics` | Panics on two-gate violation |

## Fixture File

`fixtures/sample.bin` — Captured 05-feb-2026 from live Richat geyser stream.

**Contents:**
- 50 slots (398202773 - 398202820)
- 40,186 total messages
- 39,840 entries
- 295 slot lifecycle events
- 50 BlockMeta events

## Running Tests

```bash
cargo test -p yellowstone-vixen-block-coordinator
cargo test -p yellowstone-vixen-block-coordinator -- --nocapture
```

## Custom Fixture Path

```bash
FIXTURE_PATH=/path/to/fixture.bin cargo test -p yellowstone-vixen-block-coordinator
```
