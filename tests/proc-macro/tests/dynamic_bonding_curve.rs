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
    let config = dynamic_bonding_curve::Config::default();
    let json_str = serde_json::to_string(&config).expect("failed to json serialize");
    let _: dynamic_bonding_curve::Config =
        serde_json::from_str(&json_str).expect("failed to json deserialize");

    // instruction
    let params = dynamic_bonding_curve::InitializePoolParameters::default();
    let json_str = serde_json::to_string(&params).expect("failed to json serialize");
    let _: dynamic_bonding_curve::InitializePoolParameters =
        serde_json::from_str(&json_str).expect("failed to json deserialize");
}
