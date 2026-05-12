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
    let fill = order_engine::instruction::Fill::default();
    let json_str = serde_json::to_string(&fill).expect("failed to json serialize");
    let _: order_engine::instruction::Fill =
        serde_json::from_str(&json_str).expect("failed to json deserialize");
}
