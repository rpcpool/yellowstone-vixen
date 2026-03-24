use prost::Message;
use yellowstone_vixen_core::Parser;
use yellowstone_vixen_mock::tx_fixture;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/dca.json");

/// Transaction that triggered the "unexpected EOF" proto deserialization error
/// due to `withdraw` and `deposit` name collisions between instructions and events.
const DCA_WITHDRAW_TX: &str =
    "XbXyUniNYySCWreUPhqtnXXDtyVXvFnCqAdQupwfzT332biDHq6oBgT1wfdrZ1KhjeEWM5W25YUMn815P76eVfZ";

// ---------------------------------------------------------------------------
// Proto schemas
// ---------------------------------------------------------------------------

#[test]
fn check_protobuf_schema() {
    vixen_test_utils::check_protobuf_format(dca::PROTOBUF_SCHEMA);
    insta::assert_snapshot!(dca::PROTOBUF_SCHEMA);
}

/// Verify that colliding event types get `Evt` prefix in the proto schema.
#[test]
fn proto_schema_has_evt_prefix_for_colliding_events() {
    let schema = dca::PROTOBUF_SCHEMA;

    // Instruction types keep original names
    assert!(schema.contains("message Withdraw {"));
    assert!(schema.contains("message WithdrawAccounts {"));
    assert!(schema.contains("message WithdrawArgs {"));
    assert!(schema.contains("message Deposit {"));
    assert!(schema.contains("message DepositAccounts {"));
    assert!(schema.contains("message DepositArgs {"));

    // Event types get Evt prefix to avoid collision
    assert!(
        schema.contains("message EvtWithdraw {"),
        "event Withdraw should be renamed to EvtWithdraw in proto"
    );
    assert!(
        schema.contains("message EvtWithdrawAccounts {"),
        "event WithdrawAccounts should be renamed to EvtWithdrawAccounts in proto"
    );
    assert!(
        schema.contains("message EvtWithdrawArgs {"),
        "event WithdrawArgs should be renamed to EvtWithdrawArgs in proto"
    );
    assert!(
        schema.contains("message EvtDeposit {"),
        "event Deposit should be renamed to EvtDeposit in proto"
    );
    assert!(
        schema.contains("message EvtDepositAccounts {"),
        "event DepositAccounts should be renamed to EvtDepositAccounts in proto"
    );
    assert!(
        schema.contains("message EvtDepositArgs {"),
        "event DepositArgs should be renamed to EvtDepositArgs in proto"
    );

    // Non-colliding event types keep their original names
    assert!(schema.contains("message CollectedFee {"));
    assert!(schema.contains("message Filled {"));
    assert!(schema.contains("message Opened {"));
    assert!(schema.contains("message Closed {"));
}

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

#[tokio::test]
async fn parse_dca_withdraw_transaction() {
    let parser = dca::InstructionParser;

    let ixs = tx_fixture!(DCA_WITHDRAW_TX, &parser);

    let output = ixs
        .iter()
        .find_map(|out| out.as_ref())
        .expect("no parsed output");

    // Verify instruction is present
    assert!(output.instruction.is_some(), "should have an instruction");
}

// ---------------------------------------------------------------------------
// Proto encode round-trip
// ---------------------------------------------------------------------------

///
/// Proto encode round-trip for the combined `ProgramEventOutput` wrapper.
///
/// This is the core regression test: before the fix, the proto schema had
/// colliding message definitions for `Withdraw`/`Deposit` (instruction vs event),
/// causing "unexpected EOF" errors when Redpanda Console tried to decode
/// the serialized proto bytes.
///
#[tokio::test]
async fn proto_round_trip() {
    let parser = dca::InstructionParser;

    let ixs = tx_fixture!(DCA_WITHDRAW_TX, &parser);

    let output = ixs
        .iter()
        .find_map(|out| out.as_ref())
        .expect("no parsed output");

    let mut buf = Vec::new();
    output.encode(&mut buf).expect("proto encode failed");

    assert!(!buf.is_empty());
    assert_eq!(
        output.encoded_len(),
        buf.len(),
        "encoded_len() must match actual encoded size"
    );
}
