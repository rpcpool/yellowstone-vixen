mod common;

use common::p;
use yellowstone_vixen_anchor_event::{AnchorEventInstructionParser, EVENT_IX_TAG};
use yellowstone_vixen_core::Parser;
use yellowstone_vixen_mock::tx_fixture;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("idls/perpetuals.json");
include_vixen_parser!("idls/perpetuals.events.json");

fn make_parser(
) -> AnchorEventInstructionParser<perpetuals::InstructionParser, perpetuals_events::InstructionParser>
{
    AnchorEventInstructionParser::new(
        perpetuals::InstructionParser,
        perpetuals_events::InstructionParser,
        perpetuals::PROGRAM_ID,
    )
}

const BORROW_FROM_CUSTODY_TX: &str =
    "5mYEUYXCZisS8CChCG8mL8N3NEWHUA81Rr7kLA28P5upSzDStLq1f4QKhFLY7R8GsRNB27gM6YzvKerxejtLQxCj";

/// Load the CPI event payload (discriminator + borsh data) from the fixture.
///
/// Reads the fixture JSON, finds the instruction whose data starts with
/// `EVENT_IX_TAG`, and returns the bytes after the tag (disc + payload).
fn load_cpi_event_payload_from_fixture() -> Vec<u8> {
    let path = yellowstone_vixen_mock::fixture_path(BORROW_FROM_CUSTODY_TX).unwrap();
    let data = std::fs::read(&path).unwrap();
    let fixture = yellowstone_vixen_mock::read_instructions_fixture(&data).unwrap();
    let fixture = match fixture {
        yellowstone_vixen_mock::FixtureData::Instructions(f) => f,
        _ => panic!("expected instructions fixture"),
    };
    fixture
        .instructions
        .iter()
        .find(|ix| ix.data.len() >= 8 && ix.data[..8] == EVENT_IX_TAG)
        .map(|ix| ix.data[8..].to_vec())
        .expect("no CPI event instruction found in fixture")
}

fn assert_borrow_from_custody_event(
    args: &perpetuals_events::instruction::BorrowFromCustodyEventArgs,
) {
    assert_eq!(
        args.owner,
        p("E2Z5ggFhABjC5tSZYouMgfgUpgNsvDpWrR6YTFt7D4YC")
    );
    assert_eq!(args.pool, p("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq"));
    assert_eq!(
        args.position_key,
        p("iUzDVme5Mc21GdULKK2JFuvjNWY4TaULF2kNGTcoXf9")
    );
    assert_eq!(
        args.position_mint,
        p("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
    );
    assert_eq!(
        args.position_custody,
        p("G18jKKXQwBbrHeiK3C9MRXhkHsLHf7XgCSisykV46EZa")
    );
    assert_eq!(args.size_custody_token, 8_463_144_037);
    assert_eq!(args.collateral_amount, 3_000_000_000_000);
    assert_eq!(args.collateral_amount_usd, 11_148_464_949_892);
    assert_eq!(args.margin_usd, 10_033_618_454_902);
    assert_eq!(args.update_time, 1_772_150_417);
}

// ---------------------------------------------------------------------------
// Proto schema
// ---------------------------------------------------------------------------

#[test]
fn check_events_protobuf_schema() {
    common::check_protobuf_format(perpetuals_events::PROTOBUF_SCHEMA);
    insta::assert_snapshot!(perpetuals_events::PROTOBUF_SCHEMA);
}

// ---------------------------------------------------------------------------
// CPI event parsing with real transaction data
// ---------------------------------------------------------------------------

/// Parse the real BorrowFromCustody transaction using `AnchorEventInstructionParser`.
///
/// The fixture contains two perpetuals instructions:
/// 1. BorrowFromCustody (regular instruction)
/// 2. BorrowFromCustodyEvent (CPI self-invocation event)
///
/// The composable parser routes each to the correct inner parser.
#[tokio::test]
async fn parse_cpi_event_from_real_tx() {
    let parser = make_parser();

    let ixs = tx_fixture!(
        "5mYEUYXCZisS8CChCG8mL8N3NEWHUA81Rr7kLA28P5upSzDStLq1f4QKhFLY7R8GsRNB27gM6YzvKerxejtLQxCj",
        &parser
    );

    // Find the regular instruction output (BorrowFromCustody).
    let ix_output = ixs
        .iter()
        .find_map(|out| {
            let out = out.as_ref()?;
            out.instruction.as_ref()?;
            Some(out)
        })
        .expect("no regular instruction output found");

    assert!(ix_output.instruction.is_some());

    let (borrow_accounts, borrow_args) = match &ix_output.instruction.as_ref().unwrap().instruction
    {
        perpetuals::instruction::Instruction::BorrowFromCustody { accounts, args } => {
            (accounts, args)
        },
        other => panic!("Expected BorrowFromCustody, got {other:?}"),
    };

    assert_eq!(
        borrow_accounts.owner,
        p("E2Z5ggFhABjC5tSZYouMgfgUpgNsvDpWrR6YTFt7D4YC")
    );
    assert_eq!(
        borrow_accounts.pool,
        p("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq")
    );
    assert_eq!(borrow_args.amount, 8_463_144_037);

    // Find the CPI event output (BorrowFromCustodyEvent).
    let evt_output = ixs
        .iter()
        .find_map(|out| {
            let out = out.as_ref()?;
            if out.instruction.is_none() && !out.events.is_empty() {
                Some(out)
            } else {
                None
            }
        })
        .expect("no CPI event output found");

    assert!(evt_output.instruction.is_none());
    assert_eq!(evt_output.events.len(), 1);

    let event_args = match &evt_output.events[0].instruction {
        perpetuals_events::instruction::Instruction::BorrowFromCustodyEvent {
            accounts: _,
            args,
        } => args,
        other => panic!("Expected BorrowFromCustodyEvent, got {other:?}"),
    };

    assert_borrow_from_custody_event(event_args);
}

// ---------------------------------------------------------------------------
// Log event parsing with real event data
// ---------------------------------------------------------------------------

/// Test `resolve_events_from_logs` using real event payload data.
///
/// Takes the actual BorrowFromCustodyEvent borsh payload from the CPI event
/// in the 5mYE... transaction, formats it as a "Program data:" log line
/// (base64 encoded, without EVENT_IX_TAG prefix), and verifies parsing.
#[test]
fn resolve_events_from_logs_with_real_event_data() {
    use base64::Engine;

    let disc_and_payload = load_cpi_event_payload_from_fixture();
    let encoded = base64::engine::general_purpose::STANDARD.encode(&disc_and_payload);
    let logs = vec![
        "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu invoke [1]".to_string(),
        format!("Program data: {encoded}"),
        "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu success".to_string(),
    ];

    let events = perpetuals_events::resolve_events_from_logs(&logs);
    assert_eq!(events.len(), 1, "should parse exactly one event from logs");

    let event_args = match &events[0].instruction {
        perpetuals_events::instruction::Instruction::BorrowFromCustodyEvent {
            accounts: _,
            args,
        } => args,
        other => panic!("Expected BorrowFromCustodyEvent, got {other:?}"),
    };

    assert_borrow_from_custody_event(event_args);
}

/// Verify that the fixture's real logs (which have no "Program data:" lines)
/// produce no false-positive log events. Perpetuals emits events via CPI,
/// not via "Program data:" logs.
#[test]
fn resolve_events_from_fixture_logs_returns_empty() {
    let path = yellowstone_vixen_mock::fixture_path(BORROW_FROM_CUSTODY_TX).unwrap();
    let data = std::fs::read(&path).unwrap();
    let fixture = yellowstone_vixen_mock::read_instructions_fixture(&data).unwrap();
    let fixture = match fixture {
        yellowstone_vixen_mock::FixtureData::Instructions(f) => f,
        _ => panic!("expected instructions fixture"),
    };

    // The real transaction logs contain no "Program data:" lines.
    let events = perpetuals_events::resolve_events_from_logs(&fixture.log_messages);
    assert!(
        events.is_empty(),
        "perpetuals uses CPI events, not log events"
    );
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

#[test]
fn resolve_events_from_logs_returns_empty_for_no_matches() {
    let logs = vec![
        "Program ABC invoke [1]".to_string(),
        "Program log: hello".to_string(),
        "Program ABC success".to_string(),
    ];
    let events = perpetuals_events::resolve_events_from_logs(&logs);
    assert!(events.is_empty());
}

#[test]
fn resolve_events_from_logs_skips_invalid_base64() {
    let logs = vec!["Program data: !!!invalid-base64!!!".to_string()];
    let events = perpetuals_events::resolve_events_from_logs(&logs);
    assert!(events.is_empty());
}

#[test]
fn resolve_events_from_logs_skips_non_matching_discriminator() {
    use base64::Engine;
    let fake_data = vec![0xde, 0xad, 0xbe, 0xef, 0x00, 0x00, 0x00, 0x00];
    let encoded = base64::engine::general_purpose::STANDARD.encode(&fake_data);
    let logs = vec![format!("Program data: {encoded}")];
    let events = perpetuals_events::resolve_events_from_logs(&logs);
    assert!(events.is_empty());
}

// ---------------------------------------------------------------------------
// Constants and trait impls
// ---------------------------------------------------------------------------

#[test]
fn event_ix_tag_constant_is_correct() {
    let expected = 0x1d9a_cb51_2ea5_45e4_u64.to_le_bytes();
    assert_eq!(EVENT_IX_TAG, expected);
}

#[test]
fn anchor_event_parser_implements_prefilter() {
    let parser = make_parser();
    let _pf = parser.prefilter();
}

#[test]
fn anchor_event_parser_implements_id() {
    use std::borrow::Cow;
    let parser = make_parser();
    let id: Cow<'static, str> = parser.id();
    assert!(!id.is_empty());
}
