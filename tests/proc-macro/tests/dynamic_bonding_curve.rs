use vixen_test_utils::check_protobuf_format;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/dynamic_bonding_curve.json");

#[test]
fn check_protobuf_schema() {
    check_protobuf_format(dynamic_bonding_curve::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(dynamic_bonding_curve::PROTOBUF_SCHEMA);
}

// TODO: add test for parsing MeteoraDammMigrationMetadata account which has duplicated field names
