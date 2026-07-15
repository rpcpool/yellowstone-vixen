// Regression test for inline Codama structs.
//
// Pinocchio-style fixed strings can be represented as an inline struct such
// as `[u8 length][N-byte UTF-8 buffer]`. The proc macro must materialize these
// structs as protobuf messages, including when they are wrapped in Option.

use vixen_test_utils::check_protobuf_format;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/inline_struct.json");

#[test]
fn check_protobuf_schema() {
    check_protobuf_format(inline_struct::PROTOBUF_SCHEMA);

    assert!(inline_struct::PROTOBUF_SCHEMA.contains("message SetMetadataArgsMetadataName"));
    assert!(inline_struct::PROTOBUF_SCHEMA.contains("message SetMetadataArgsOptionalMetadata"));
}

#[test]
fn inline_structs_are_generated_as_messages() {
    let args = inline_struct::instruction::SetMetadataArgs::default();

    assert_eq!(args.metadata_name.len, 0);
    assert!(args.optional_metadata.is_none());
}
