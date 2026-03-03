use codama_nodes::InstructionNode;

use crate::intermediate_representation::{
    helpers::build_fields_ir, FieldIr, FieldTypeIr, LabelIr, OneofIr, OneofVariantIr, ScalarIr,
    SchemaIr, TypeIr, TypeKindIr,
};

///
/// Converts Codama `instructions` into IR messages.
///
/// For each instruction in the IDL, we generate three messages:
///
///   1) <IxName>Accounts  → holds all account pubkeys
///   2) <IxName>Args      → holds instruction arguments
///   3) <IxName>          → wrapper combining accounts + args
///
/// At the end, we also create a global:
///
///   <ProgramName> { oneof instruction { ... } }
///
/// This acts as the root enum for all instructions.
///
/// ─────────────────────────────────────────────────────────────
/// Example IDL instruction:
///
/// ```text
/// instruction open_position {
///   accounts {
///     owner: pubkey,
///     position: pubkey,
///     pool: pubkey,
///   }
///
///   args {
///     collateral: u64,
///     leverage: u32,
///   }
/// }
/// ```
///
/// ─────────────────────────────────────────────────────────────
/// IR result:
///
/// 1) Accounts message
///
/// ```rust, ignore
/// TypeIr {
///   name: "OpenPositionAccounts",
///   fields: [
///     { name: "owner",    tag: 1, field_type: PublicKey },
///     { name: "position", tag: 2, field_type: PublicKey },
///     { name: "pool",     tag: 3, field_type: PublicKey },
///   ],
///   kind: Instruction
/// }
/// ```
///
/// 2) Args message
///
/// ```rust, ignore
/// TypeIr {
///   name: "OpenPositionArgs",
///   fields: [
///     { name: "collateral", tag: 1, field_type: Uint64 },
///     { name: "leverage",   tag: 2, field_type: Uint32 },
///   ],
///   kind: Instruction
/// }
/// ```
///
/// 3) Wrapper payload message
///
/// ```rust, ignore
/// TypeIr {
///   name: "OpenPosition",
///   fields: [
///     { name: "accounts", tag: 1, field_type: Message(OpenPositionAccounts), optional },
///     { name: "args",     tag: 2, field_type: Message(OpenPositionArgs),     optional },
///   ],
///   kind: Instruction
/// }
/// ```
///
/// ─────────────────────────────────────────────────────────────
/// Final global instruction enum (uses the program name, e.g. "Perpetuals"):
///
/// ```rust, ignore
/// OneofIr {
///   parent_message: "Perpetuals",
///   field_name: "instruction",
///   variants: [
///     { tag: 1, variant_name: "OpenPosition", message_type: "OpenPosition" },
///     { tag: 2, variant_name: "SetLimits",    message_type: "SetLimits" },
///     ...
///   ]
/// }
/// ```
///
/// This allows a single protobuf message to represent any instruction.
///
pub fn build_instructions_schema(instructions: &[InstructionNode], ir: &mut SchemaIr) {
    for ix in instructions {
        build_instruction_messages(ix, ir);
    }

    build_instruction_dispatch_oneof(instructions, ir);
}

///
/// Build the three messages for a single instruction:
///   - `<IxName>Accounts`  — one PublicKey field per account
///   - `<IxName>Args`      — instruction arguments (delegates to `build_fields_ir`)
///   - `<IxName>`          — wrapper with optional accounts + args
///
fn build_instruction_messages(ix: &InstructionNode, ir: &mut SchemaIr) {
    let ix_name = crate::utils::to_pascal_case(&ix.name);

    let accounts_name = format!("{ix_name}Accounts");
    let args_name = format!("{ix_name}Args");
    let payload_name = ix_name.clone();

    let account_fields: Vec<FieldIr> = ix
        .accounts
        .iter()
        .enumerate()
        .map(|(i, acct)| FieldIr {
            name: crate::utils::to_snake_case(&acct.name),
            tag: (i + 1) as u32,
            label: LabelIr::Singular,
            field_type: FieldTypeIr::Scalar(ScalarIr::PublicKey),
        })
        .collect();

    // Use `types.push()` instead of `push_unique_type()` because instruction
    // wrapper names (after dropping the `Instruction` suffix) can collide with
    // defined types or accounts that were pushed earlier. We always want the
    // Instruction-kind entry in the schema; the renderer routes types to the
    // correct scope based on `kind`, so duplicates with different kinds are fine.
    ir.types.push(TypeIr {
        name: accounts_name.clone(),
        fields: account_fields,
        kind: TypeKindIr::Instruction,
    });

    let arg_fields = build_fields_ir(&args_name, &ix.arguments, ir, TypeKindIr::Helper);

    ir.types.push(TypeIr {
        name: args_name.clone(),
        fields: arg_fields,
        kind: TypeKindIr::Instruction,
    });

    ir.types.push(TypeIr {
        name: payload_name,
        fields: vec![
            FieldIr {
                name: "accounts".to_string(),
                tag: 1,
                label: LabelIr::Optional,
                field_type: FieldTypeIr::Message(accounts_name),
            },
            FieldIr {
                name: "args".to_string(),
                tag: 2,
                label: LabelIr::Optional,
                field_type: FieldTypeIr::Message(args_name),
            },
        ],
        kind: TypeKindIr::Instruction,
    });
}

///
/// Build the `Instructions` message with a `oneof instruction { ... }` that dispatches
/// to each individual `<InstructionName>` payload.
///
fn build_instruction_dispatch_oneof(instructions: &[InstructionNode], ir: &mut SchemaIr) {
    let parent_name = "Instructions".to_string();

    let variants: Vec<OneofVariantIr> = instructions
        .iter()
        .enumerate()
        .map(|(i, ix)| {
            let ix_name = crate::utils::to_pascal_case(&ix.name);

            OneofVariantIr {
                tag: (i + 1) as u32,
                variant_name: ix_name.clone(),
                message_type: ix_name.clone(),
            }
        })
        .collect();

    ir.oneofs.push(OneofIr {
        parent_message: parent_name,
        field_name: "instruction".to_string(),
        variants,
        kind: crate::intermediate_representation::OneofKindIr::InstructionDispatch,
    });
}
