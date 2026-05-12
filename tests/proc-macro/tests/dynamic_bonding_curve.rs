use vixen_test_utils::check_protobuf_format;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/dynamic_bonding_curve.json");

#[test]
fn check_protobuf_schema() {
    check_protobuf_format(dynamic_bonding_curve::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(dynamic_bonding_curve::PROTOBUF_SCHEMA);
}

// TODO: add test for parsing MeteoraDammMigrationMetadata account which has duplicated field names

#[test]
fn check_json_serialization() {
    // account
    let json_str = serde_json::to_string(&dynamic_bonding_curve::Config::default());
    assert!(json_str.is_ok(), "failed to json serialize");
    // instruction
    let json_str =
        serde_json::to_string(&dynamic_bonding_curve::InitializePoolParameters::default());
    assert!(json_str.is_ok(), "failed to json serialize");
}
