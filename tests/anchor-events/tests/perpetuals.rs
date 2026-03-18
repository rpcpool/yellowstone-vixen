mod common;

use common::p;
use prost::Message;
use yellowstone_vixen_anchor_event::{
    merge_proto_schemas, AnchorEventInstructionParser, AnchorEventOutput, EVENT_IX_TAG,
};
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

// ---------------------------------------------------------------------------
// Proto schemas
// ---------------------------------------------------------------------------

#[test]
fn check_events_protobuf_schema() {
    common::check_protobuf_format(perpetuals_events::PROTOBUF_SCHEMA);
    insta::assert_snapshot!(perpetuals_events::PROTOBUF_SCHEMA);
}

#[test]
fn check_merged_protobuf_schema() {
    let (schema, message_index) = merge_proto_schemas(
        perpetuals::PROTOBUF_SCHEMA,
        perpetuals_events::PROTOBUF_SCHEMA,
    );

    assert_eq!(message_index, 0);

    common::check_protobuf_format(&schema);
    insta::assert_snapshot!(schema);
}

// ---------------------------------------------------------------------------
// Parsing: instruction + CPI event from real transaction
// ---------------------------------------------------------------------------

///
/// Parse the BorrowFromCustody transaction using `AnchorEventInstructionParser`.
///
/// The fixture contains two perpetuals instructions:
/// 1. BorrowFromCustody — regular instruction
/// 2. BorrowFromCustodyEvent — CPI self-invocation event
///
/// The composable parser routes each to the correct inner parser, producing
/// `AnchorEventOutput { instruction, anchor_events }`.
///
#[tokio::test]
async fn parse_borrow_from_custody_with_cpi_event() {
    let parser = make_parser();

    let ixs = tx_fixture!(BORROW_FROM_CUSTODY_TX, &parser);

    // The fixture has one BorrowFromCustody instruction with a CPI event
    // inner instruction. The parser should produce a single output combining both.
    let output = ixs
        .iter()
        .find_map(|out| out.as_ref())
        .expect("no parsed output found");

    let expected = AnchorEventOutput {
        instruction: Some(perpetuals::Instructions {
            instruction: perpetuals::instruction::Instruction::BorrowFromCustody {
                accounts: perpetuals::instruction::BorrowFromCustodyAccounts {
                    owner: p("E2Z5ggFhABjC5tSZYouMgfgUpgNsvDpWrR6YTFt7D4YC"),
                    perpetuals: p("H4ND9aYttUVLFmNypZqLjZ52FYiGvdEB45GmwNoKEjTj"),
                    pool: p("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq"),
                    custody: p("G18jKKXQwBbrHeiK3C9MRXhkHsLHf7XgCSisykV46EZa"),
                    transfer_authority: p("AVzP2GeRmqGphJsMxWoqjpUifPpCret7LqWhD8NWQK49"),
                    borrow_position: p("iUzDVme5Mc21GdULKK2JFuvjNWY4TaULF2kNGTcoXf9"),
                    custody_token_account: p("WzWUoCmtVv7eqAbU3BfKPU3fhLP6CXR8NCJH78UK9VS"),
                    user_token_account: p("Av9zpU3ZtdfYMHdW9ombpaarS9bQhsK4Rxvt6piUTkmH"),
                    lp_token_mint: p("27G8MtK7VtTcCHkpASjSDdkWWYfoqT6ggEuKidVJidD4"),
                    token_program: p("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                    event_authority: p("37hJBDnntwqhGbK7L6M1bLyvccj4u55CCUiLPdYkiqBN"),
                    program: p("PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu"),
                    remaining_accounts: vec![
                        p("7xS2gz2bTp3fwCC7knJvUWTEU9Tycczu6VhJYKgi1wdz"),
                        p("AQCGyheWPLeo6Qp9WpYS9m3Qj479t7R636N9ey1rEjEn"),
                        p("5Pv3gM9JrFFH883SWAhvJC9RPYmo8UNxuFtv5bMMALkm"),
                        p("G18jKKXQwBbrHeiK3C9MRXhkHsLHf7XgCSisykV46EZa"),
                        p("4vkNeXiYEUizLdrpdPS1eC2mccyM4NUPRtERrk6ZETkk"),
                        p("FYq2BWQ1V5P1WFBqr3qB2Kb5yHVvSv7upzKodgQE5zXh"),
                        p("AFZnHPzy4mvVCffrVwhewHbFc93uTHvDSFrVH7GtfXF1"),
                        p("hUqAT1KQ7eW1i6Csp9CXYtpPfSAvi835V7wKi5fRfmC"),
                        p("6Jp2xZUTWdDD2ZyUPRzeMdc6AFQ5K3pFgZxk2EijfjnM"),
                        p("Fgc93D641F8N2d1xLjQ4jmShuD3GE3BsCXA56KBQbF5u"),
                    ],
                },
                args: perpetuals::instruction::BorrowFromCustodyArgs {
                    amount: 8_463_144_037,
                },
            },
        }),
        anchor_events: vec![perpetuals_events::Instructions {
            instruction: perpetuals_events::instruction::Instruction::BorrowFromCustodyEvent {
                accounts: perpetuals_events::instruction::BorrowFromCustodyEventAccounts {
                    remaining_accounts: vec![p("37hJBDnntwqhGbK7L6M1bLyvccj4u55CCUiLPdYkiqBN")],
                },
                args: perpetuals_events::instruction::BorrowFromCustodyEventArgs {
                    owner: p("E2Z5ggFhABjC5tSZYouMgfgUpgNsvDpWrR6YTFt7D4YC"),
                    pool: p("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq"),
                    position_key: p("iUzDVme5Mc21GdULKK2JFuvjNWY4TaULF2kNGTcoXf9"),
                    position_mint: p("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                    position_custody: p("G18jKKXQwBbrHeiK3C9MRXhkHsLHf7XgCSisykV46EZa"),
                    size_custody_token: 8_463_144_037,
                    collateral_amount: 3_000_000_000_000,
                    collateral_amount_usd: 11_148_464_949_892,
                    margin_usd: 10_033_618_454_902,
                    update_time: 1_772_150_417,
                },
            },
        }],
    };

    assert_eq!(output, &expected);
}

// ---------------------------------------------------------------------------
// Proto encode round-trip for AnchorEventOutput
// ---------------------------------------------------------------------------

///
/// Proto encode round-trip for the combined `AnchorEventOutput` wrapper.
///
/// Verifies that the manual `prost::Message` impl on `AnchorEventOutput`
/// correctly encodes both the instruction (tag 1) and events (tag 2),
/// and that `encoded_len()` matches the actual output size.
///
#[tokio::test]
async fn proto_round_trip_anchor_event_output() {
    let parser = make_parser();

    let ixs = tx_fixture!(BORROW_FROM_CUSTODY_TX, &parser);

    let output = ixs
        .iter()
        .find_map(|out| out.as_ref())
        .expect("no parsed output");

    assert!(output.instruction.is_some());
    assert!(!output.anchor_events.is_empty());

    let mut buf = Vec::new();

    output.encode(&mut buf).expect("proto encode failed");

    assert!(!buf.is_empty());
    assert_eq!(output.encoded_len(), buf.len());
}

// ---------------------------------------------------------------------------
// resolve_events_from_logs
// ---------------------------------------------------------------------------

/// Load the CPI event payload (discriminator + borsh data) from the fixture.
///
/// Reads the fixture JSON, finds the instruction whose data starts with
/// `EVENT_IX_TAG`, and returns the bytes after the tag (disc + payload).
fn load_cpi_event_payload_from_fixture() -> Vec<u8> {
    use yellowstone_vixen_mock::SerializableInstructionUpdate;

    fn find_event(ixs: &[SerializableInstructionUpdate]) -> Option<Vec<u8>> {
        for ix in ixs {
            if ix.data.len() >= 8 && ix.data[..8] == EVENT_IX_TAG {
                return Some(ix.data[8..].to_vec());
            }

            if let Some(found) = find_event(&ix.inner) {
                return Some(found);
            }
        }

        None
    }

    let path = yellowstone_vixen_mock::fixture_path(BORROW_FROM_CUSTODY_TX).unwrap();
    let data = std::fs::read(&path).unwrap();
    let fixture = yellowstone_vixen_mock::read_instructions_fixture(&data).unwrap();
    let fixture = match fixture {
        yellowstone_vixen_mock::FixtureData::Instructions(f) => f,
        _ => panic!("expected instructions fixture"),
    };

    find_event(&fixture.instructions).expect("no CPI event instruction found in fixture")
}

///
/// Test `resolve_events_from_logs` using real event payload data.
///
/// Takes the actual BorrowFromCustodyEvent borsh payload from the CPI event
/// in the 5mYE... transaction, formats it as a "Program data:" log line
/// (base64 encoded, without EVENT_IX_TAG prefix), and verifies parsing.
///
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

    let (event_accounts, event_args) = match &events[0].instruction {
        perpetuals_events::instruction::Instruction::BorrowFromCustodyEvent { accounts, args } => {
            (accounts, args)
        },
        other => panic!("Expected BorrowFromCustodyEvent, got {other:?}"),
    };

    let expected = perpetuals_events::instruction::BorrowFromCustodyEvent {
        accounts: perpetuals_events::instruction::BorrowFromCustodyEventAccounts {
            remaining_accounts: vec![],
        },
        args: perpetuals_events::instruction::BorrowFromCustodyEventArgs {
            owner: p("E2Z5ggFhABjC5tSZYouMgfgUpgNsvDpWrR6YTFt7D4YC"),
            pool: p("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq"),
            position_key: p("iUzDVme5Mc21GdULKK2JFuvjNWY4TaULF2kNGTcoXf9"),
            position_mint: p("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
            position_custody: p("G18jKKXQwBbrHeiK3C9MRXhkHsLHf7XgCSisykV46EZa"),
            size_custody_token: 8_463_144_037,
            collateral_amount: 3_000_000_000_000,
            collateral_amount_usd: 11_148_464_949_892,
            margin_usd: 10_033_618_454_902,
            update_time: 1_772_150_417,
        },
    };

    assert_eq!(event_accounts, &expected.accounts);
    assert_eq!(event_args, &expected.args);
}

/// Verify that the fixture's real logs (which have no "Program data:" lines)
/// produce no false-positive log events.
#[test]
fn resolve_events_from_fixture_logs_returns_empty() {
    let path = yellowstone_vixen_mock::fixture_path(BORROW_FROM_CUSTODY_TX).unwrap();
    let data = std::fs::read(&path).unwrap();
    let fixture = yellowstone_vixen_mock::read_instructions_fixture(&data).unwrap();
    let fixture = match fixture {
        yellowstone_vixen_mock::FixtureData::Instructions(f) => f,
        _ => panic!("expected instructions fixture"),
    };

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
