mod common;

use yellowstone_vixen_proc_macro::include_vixen_parser;

// This IDL have non-even length hex strings for discriminators, which caused `hex::decode` to fail before we added padding logic. This test ensures that the padding logic works correctly.

include_vixen_parser!("idls/raydium_amm_v4_with_swapv2.json");

#[tokio::test]
async fn check_protobuf_schema() {
    common::check_protobuf_format(raydium_amm::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(raydium_amm::PROTOBUF_SCHEMA);
}
