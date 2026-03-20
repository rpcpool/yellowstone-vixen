use prost::Message;
use vixen_test_utils::{check_protobuf_format, p};
use yellowstone_vixen_core::Parser;
use yellowstone_vixen_mock::tx_fixture;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("idls/invite_escrow.json");

// ---------------------------------------------------------------------------
// Proto schemas
// ---------------------------------------------------------------------------

#[test]
fn check_protobuf_schema() {
    check_protobuf_format(invite_escrow::PROTOBUF_SCHEMA);
    insta::assert_snapshot!(invite_escrow::PROTOBUF_SCHEMA);
}

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

#[tokio::test]
async fn parse_initialize_token_transaction() {
    let parser = invite_escrow::InstructionParser;

    let ixs = tx_fixture!(
        "5esX3MC2s6rQcG1y9npt2jL5HjtUUSSXst1v16vuSng3wAv9uaskiL47kxb5zFnNZVqZVKYdYQVM92hXRKYkYU8x",
        &parser
    );

    let output = ixs
        .iter()
        .find_map(|out| out.as_ref())
        .expect("no parsed output");

    let expected = invite_escrow::ProgramEventOutput {
        instruction: Some(invite_escrow::Instructions {
            instruction: invite_escrow::instruction::Instruction::Initialize {
                accounts: invite_escrow::instruction::InitializeAccounts {
                    sender: p("6fPYzB3LD8XNFFeNs2RuLgyS2YT6zk5zpHE563YvWSGs"),
                    invite_signer: p("EndSeFWW1yEVgSwmehUv4TkqPMsVzyGyKrrxgzsgf74D"),
                    invite_info: p("7ov3L76oTX33PbNQd3T95V7FBEn8J23hdwMAXPV5Cs4K"),
                    system_program: p("11111111111111111111111111111111"),
                    remaining_accounts: vec![],
                },
                args: invite_escrow::instruction::InitializeArgs {
                    expiry: 1_774_449_361,
                    amount: 1_004_273_804,
                },
            },
        }),
        program_events: vec![invite_escrow::Events {
            event: invite_escrow::event::Event::Initialize {
                accounts: invite_escrow::event::InitializeAccounts {
                    remaining_accounts: vec![],
                },
                args: invite_escrow::event::InitializeArgs {
                    sender: p("6fPYzB3LD8XNFFeNs2RuLgyS2YT6zk5zpHE563YvWSGs"),
                    invite_signer: p("EndSeFWW1yEVgSwmehUv4TkqPMsVzyGyKrrxgzsgf74D"),
                    amount: 1_004_273_804,
                    expiration: 1_774_449_361,
                },
            },
        }],
    };

    assert_eq!(output, &expected);
}

// ---------------------------------------------------------------------------
// Proto encode round-trip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn proto_round_trip() {
    let parser = invite_escrow::InstructionParser;

    let ixs = tx_fixture!(
        "5esX3MC2s6rQcG1y9npt2jL5HjtUUSSXst1v16vuSng3wAv9uaskiL47kxb5zFnNZVqZVKYdYQVM92hXRKYkYU8x",
        &parser
    );

    let output = ixs
        .iter()
        .find_map(|out| out.as_ref())
        .expect("no parsed output");

    let mut buf = Vec::new();
    output.encode(&mut buf).expect("proto encode failed");

    assert!(!buf.is_empty());
    assert_eq!(output.encoded_len(), buf.len());
}
