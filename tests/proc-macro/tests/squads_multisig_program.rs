use prost::Message;
use prost_reflect::{DescriptorPool, DynamicMessage};
use vixen_test_utils::check_protobuf_format;
use yellowstone_vixen_core::Parser;
use yellowstone_vixen_mock::tx_fixture;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/squads_multisig_program.json");

#[test]
fn check_protobuf_schema() {
    check_protobuf_format(squads_multisig_program::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(squads_multisig_program::PROTOBUF_SCHEMA);
}

///
/// Regression test for the schema/encoder mismatch on `VaultTransactionCreate`.
///
/// The IDL defines a top-level type `vaultTransactionCreateArgs` and an
/// instruction `vaultTransactionCreate` whose `args` field references it.
/// The instruction-args wrapper struct that vixen synthesizes
/// (`instruction::VaultTransactionCreateArgs`) collides with the top-level
/// type's name. When this collision occurs, vixen used to emit:
///
/// - a *wrapper* Rust struct on the encoder side
///   (`instruction::VaultTransactionCreateArgs { args: super::VaultTransactionCreateArgs }`),
/// - but a *flat* declaration in `PROTOBUF_SCHEMA`
///   (`message VaultTransactionCreateArgs { uint32 vault_index = 1; ... }`).
///
/// The runtime then encodes one extra layer of nesting compared to what the
/// schema declares, and any consumer using the schema (e.g. a Schema
/// Registry-backed decoder) fails with
/// `invalid wire type: LengthDelimited (expected Varint)` on tag=1.
///
/// This test takes a real on-chain `vault_transaction_create` instruction,
/// encodes it through the parser-generated `prost::Message` impl exactly as
/// the kafka sink does, and decodes the result with `prost-reflect` against
/// `PROTOBUF_SCHEMA`. If schema and encoder agree, the round-trip succeeds.
///
#[tokio::test]
async fn schema_matches_encoder_for_vault_transaction_create() {
    let parser = squads_multisig_program::InstructionParser;

    let ixs = tx_fixture!(
        "1JXEeNsaTdwwnTAsQBLvA63cMSh3uKHnCLB8cB23oHdcszMM82k3skn79RmFfi9iUvav1spsUCG3C5QXT6U9Du2",
        &parser
    );

    let instructions = ixs
        .iter()
        .find_map(|ix| ix.as_ref())
        .expect("no instructions in fixture");

    // Sanity: this fixture must contain a vault_transaction_create.
    assert!(
        matches!(
            instructions.instruction,
            squads_multisig_program::instruction::Instruction::VaultTransactionCreate { .. }
        ),
        "fixture should contain a VaultTransactionCreate instruction",
    );

    // Encode through the producer-side prost impl (what the kafka sink emits).
    let mut buf = Vec::new();
    instructions.encode(&mut buf).expect("encode failed");
    assert!(!buf.is_empty());

    // Build a descriptor pool from PROTOBUF_SCHEMA (what consumers see) and
    // try to decode the wire bytes against the schema's `Instructions` type.
    let fdp = protox_parse::parse("schema.proto", squads_multisig_program::PROTOBUF_SCHEMA)
        .expect("schema parse failed");
    let mut pool = DescriptorPool::new();
    pool.add_file_descriptor_proto(fdp).expect("descriptor add failed");

    let inst_desc = pool
        .get_message_by_name("squads_multisig_program.Instructions")
        .expect("Instructions message missing from schema");

    let dynamic = DynamicMessage::decode(inst_desc, buf.as_slice()).expect(
        "schema-driven decode failed — encoder produced bytes that do not match PROTOBUF_SCHEMA",
    );

    // Verify the decoded oneof actually picked vault_transaction_create
    // (tag 17 in `Instructions.instruction`).
    let oneof_field = dynamic
        .fields()
        .find(|(f, _)| f.name() == "vault_transaction_create")
        .expect("vault_transaction_create variant missing in decoded payload");
    assert_eq!(oneof_field.0.number(), 17);
}
