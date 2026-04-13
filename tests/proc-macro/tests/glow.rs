// Regression test for type_name_conflict: glow.json declares both an account
// and a defined type named `MarginAccount`. Without the fix in
// `rust_types_from_ir` (skipping DefinedTypes shadowed by an Account of the
// same name), this file would fail to compile with E0428 ("MarginAccount is
// defined multiple times").
//
// This IDL has no instructions — only accounts — so only AccountParser is
// exercised here.

use vixen_test_utils::check_protobuf_format;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/glow.json");

#[test]
fn check_protobuf_schema() {
    check_protobuf_format(margin::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(margin::PROTOBUF_SCHEMA);
}

#[test]
fn account_dispatch_index_is_some() {
    assert!(
        margin::ACCOUNT_DISPATCH_MESSAGE_INDEX.is_some(),
        "expected AccountDispatch message index to be present for an accounts-only IDL"
    );
}
