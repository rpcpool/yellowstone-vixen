use prost::Message;
use prost_reflect::{DescriptorPool, DynamicMessage};
use vixen_test_utils::check_protobuf_format;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/inline_struct_collisions.json");

#[test]
fn helpers_do_not_shadow_instruction_or_account_messages() {
    let schema = inline_struct_collisions::PROTOBUF_SCHEMA;

    check_protobuf_format(schema);

    assert!(schema.contains("message FooArgs {\n  uint64 amount = 1;"));
    assert!(schema.contains("message FooArgs2 {\n  uint32 code = 1;"));
    assert!(schema.contains("message Bar {\n  uint32 account_value = 1;"));
    assert!(schema.contains("message Bar2 {\n  uint32 code = 1;"));

    let instructions = inline_struct_collisions::Instructions {
        instruction: inline_struct_collisions::instruction::Instruction::Foo {
            accounts: inline_struct_collisions::instruction::FooAccounts::default(),
            args: inline_struct_collisions::instruction::FooArgs {
                amount: 8_000_000_000,
            },
        },
    };
    let encoded = instructions.encode_to_vec();
    let descriptor = protox_parse::parse("schema.proto", schema).expect("schema parse failed");
    let mut pool = DescriptorPool::new();

    pool.add_file_descriptor_proto(descriptor)
        .expect("descriptor add failed");

    let instructions_descriptor = pool
        .get_message_by_name("inline_struct_collisions.Instructions")
        .expect("Instructions message missing from schema");

    DynamicMessage::decode(instructions_descriptor, encoded.as_slice())
        .expect("schema-driven decode failed for generated instruction bytes");
}
