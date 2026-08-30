use shipstern_proc_macro::include_shipstern_parser;
use shipstern_test_utils::check_protobuf_format;

include_shipstern_parser!("../idls/dynamic_bonding_curve.json");

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

///
/// Anchor-style field discriminator: an 8-byte sighash stripped before the
/// account body is deserialized.
///
#[test]
fn exposes_account_discriminators() {
    assert_eq!(
        dynamic_bonding_curve::VirtualPool::DISCRIMINATOR,
        hex::decode("d5e005d16245775c").unwrap().as_slice()
    );
    assert_eq!(dynamic_bonding_curve::VirtualPool::DISCRIMINATOR_OFFSET, 0);

    assert_eq!(
        dynamic_bonding_curve::Config::DISCRIMINATOR,
        hex::decode("9b0caae01efacc82").unwrap().as_slice()
    );
    assert_eq!(dynamic_bonding_curve::Config::DISCRIMINATOR_OFFSET, 0);
}
