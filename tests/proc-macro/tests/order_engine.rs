mod common;

use insta;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("idls/order_engine.json");

// This IDL in particular contains no accounts (specific test)

#[tokio::test]
async fn check_protobuf_schema() {
    common::check_protobuf_format(order_engine::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(order_engine::PROTOBUF_SCHEMA);
}
