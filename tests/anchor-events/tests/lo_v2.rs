use vixen_test_utils::check_protobuf_format;
use yellowstone_vixen_anchor_event::merge_proto_schemas;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("idls/lo_v2.json");
include_vixen_parser!("idls/lo_v2.events.json");

#[test]
fn check_events_protobuf_schema() {
    check_protobuf_format(limit_order2_events::PROTOBUF_SCHEMA);
    insta::assert_snapshot!(limit_order2_events::PROTOBUF_SCHEMA);
}

#[test]
fn check_merged_protobuf_schema() {
    let (schema, message_index) = merge_proto_schemas(
        limit_order2::PROTOBUF_SCHEMA,
        limit_order2_events::PROTOBUF_SCHEMA,
    );

    let message_count =
        schema.matches("\nmessage ").count() + if schema.starts_with("message ") { 1 } else { 0 };
    assert_eq!(message_index, (message_count - 1) as i32);

    check_protobuf_format(&schema);
    insta::assert_snapshot!(schema);
}
