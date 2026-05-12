// Regression test for proc_macro_panic: metaplex token_metadata IDL contains
// fields with MapTypeNode (e.g. a string→PayloadType map in rule sets). Without
// the `T::Map(_) => Bytes` arm in `map_type()`, the proc-macro panicked with
// "map_type not implemented for Map(...)".
//
// This IDL has no instructions — only accounts — so only AccountParser is
// exercised here.

use vixen_test_utils::check_protobuf_format;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/metaplex.json");

#[test]
fn check_protobuf_schema() {
    check_protobuf_format(token_metadata::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(token_metadata::PROTOBUF_SCHEMA);
}

#[test]
fn account_dispatch_index_is_some() {
    assert!(
        token_metadata::ACCOUNT_DISPATCH_MESSAGE_INDEX.is_some(),
        "expected AccountDispatch message index for an accounts-only IDL"
    );
}

#[test]
fn check_json_serialization() {
    // account
    let json_str = serde_json::to_string(&token_metadata::AssetData::default());
    assert!(json_str.is_ok(), "failed to json serialize");
    // instruction
    let json_str = serde_json::to_string(&token_metadata::CreateMasterEditionArgs::default());
    assert!(json_str.is_ok(), "failed to json serialize");
}
