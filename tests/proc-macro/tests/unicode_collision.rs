use shipstern_proc_macro::include_shipstern_parser;

include_shipstern_parser!("../idls/unicode_collision.json");

///
/// `ä` and `Ä` are distinct instructions, with distinct `Instruction` variants
/// and distinct `parse_*` helpers, so this IDL compiles. But
/// `to_snake_case().to_uppercase()` folds both to `Ä`, because `to_snake_case`
/// lowercases only ASCII while `to_uppercase` is Unicode-aware.
///
/// Neither instruction gets a constant: an ambiguous name is worse than none,
/// and emitting both would make this feature the sole reason a previously valid
/// IDL fails to compile. Referencing `Instructions::Ä_DISCRIMINATOR` here would
/// fail to compile.
///
/// Parsing is unaffected, which is what this test pins.
///
#[test]
fn colliding_const_names_are_skipped_without_breaking_parsing() {
    let path = shipstern_core::instruction::Path::new_single(0);

    let first =
        unicode_collision::resolve_instruction_default(&[], &[0x11, 0x22, 0x33, 0x44], &path)
            .expect("first instruction should still resolve");

    let second =
        unicode_collision::resolve_instruction_default(&[], &[0x55, 0x66, 0x77, 0x88], &path)
            .expect("second instruction should still resolve");

    assert_ne!(
        std::mem::discriminant(&first.instruction),
        std::mem::discriminant(&second.instruction),
        "the two instructions must remain distinguishable"
    );
}
