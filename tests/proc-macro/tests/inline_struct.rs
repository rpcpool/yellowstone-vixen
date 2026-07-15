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
    assert!(inline_struct::PROTOBUF_SCHEMA.contains("message SetMetadataArgsArrayMetadata"));
    assert!(inline_struct::PROTOBUF_SCHEMA.contains("message SetMetadataArgsTupleMetadataTuple"));
    assert!(inline_struct::PROTOBUF_SCHEMA
        .contains("message SetMetadataArgsTupleMetadataTupleItem1Inner"));
    assert!(inline_struct::PROTOBUF_SCHEMA
        .contains("message SetMetadataArgsOptionalArrayMetadataOption"));
}

#[test]
fn inline_structs_are_generated_as_messages() {
    let args = inline_struct::instruction::SetMetadataArgs::default();

    assert_eq!(args.metadata_name.len, 0);
    assert!(args.optional_metadata.is_none());
    assert!(args.array_metadata.is_empty());
    assert!(args.tuple_metadata.is_none());
    assert!(args.optional_array_metadata.is_none());
}

#[test]
fn nested_inline_structs_round_trip_borsh() {
    use borsh::BorshDeserialize;

    let args = inline_struct::instruction::SetMetadataArgs {
        metadata_name: inline_struct::SetMetadataArgsMetadataName {
            len: 3,
            value: vec![b'a'; 25],
        },
        optional_metadata: Some(inline_struct::SetMetadataArgsOptionalMetadata {
            len: 2,
            value: vec![b'b'; 5],
        }),
        array_metadata: vec![
            inline_struct::SetMetadataArgsArrayMetadata {
                code: 1,
                value: vec![b'c'; 3],
            },
            inline_struct::SetMetadataArgsArrayMetadata {
                code: 2,
                value: vec![b'd'; 3],
            },
        ],
        tuple_metadata: Some(inline_struct::SetMetadataArgsTupleMetadataTuple {
            item_0: inline_struct::SetMetadataArgsTupleMetadataTupleItem0 { code: 7 },
            item_1: vec![
                inline_struct::SetMetadataArgsTupleMetadataTupleItem1Inner {
                    items: Some(inline_struct::SetMetadataArgsTupleMetadataTupleItem1Item {
                        value: 8,
                    }),
                },
                inline_struct::SetMetadataArgsTupleMetadataTupleItem1Inner { items: None },
            ],
        }),
        optional_array_metadata: Some(inline_struct::SetMetadataArgsOptionalArrayMetadataOption {
            value: vec![
                inline_struct::SetMetadataArgsOptionalArrayMetadataValue { value: 9 },
                inline_struct::SetMetadataArgsOptionalArrayMetadataValue { value: 10 },
            ],
        }),
    };

    let encoded = borsh::to_vec(&args).unwrap();
    let decoded = inline_struct::instruction::SetMetadataArgs::try_from_slice(&encoded).unwrap();

    assert_eq!(decoded, args);
}
