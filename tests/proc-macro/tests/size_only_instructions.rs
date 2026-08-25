use shipstern_proc_macro::include_shipstern_parser;

include_shipstern_parser!("../idls/size_only_instructions.json");

///
/// Every instruction is size-discriminated, so no constant is emitted and the
/// `impl Instructions` block is skipped entirely. Referencing any
/// `*_DISCRIMINATOR` on it would fail to compile; the parser still works.
///
#[test]
fn size_only_instructions_emit_no_consts() {
    let path = shipstern_core::instruction::Path::new_single(0);

    let parsed =
        size_only_instructions::resolve_instruction_default(&[], &7_u32.to_le_bytes(), &path)
            .expect("size discriminator should match");

    let size_only_instructions::instruction::Instruction::OnlySized { args, .. } =
        parsed.instruction;

    assert_eq!(args.amount, 7);
}
