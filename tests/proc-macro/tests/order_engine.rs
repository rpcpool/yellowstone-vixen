use vixen_test_utils::check_protobuf_format;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/order_engine.json");

// This IDL in particular contains no accounts (specific test)

#[test]
fn check_protobuf_schema() {
    check_protobuf_format(order_engine::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(order_engine::PROTOBUF_SCHEMA);
}

#[test]
fn check_json_serialization() {
    // instruction
    let json_str = serde_json::to_string(&order_engine::instruction::Fill::default());
    assert!(json_str.is_ok(), "failed to json serialize");
}
