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
///   3) <IxName>Ix        → wrapper combining accounts + args
///
/// At the end, we also create a global:
///
///   ProgramInstruction { oneof ix { ... } }
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
///     { name: "owner",    tag: 1, field_type: PubkeyBytes },
///     { name: "position", tag: 2, field_type: PubkeyBytes },
///     { name: "pool",     tag: 3, field_type: PubkeyBytes },
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
///   name: "OpenPositionIx",
///   fields: [
///     { name: "accounts", tag: 1, field_type: Message(OpenPositionAccounts), optional },
///     { name: "args",     tag: 2, field_type: Message(OpenPositionArgs),     optional },
///   ],
///   kind: Instruction
/// }
/// ```
///
/// ─────────────────────────────────────────────────────────────
/// Final global instruction enum:
///
/// ```rust, ignore
/// TypeIr {
///   name: "ProgramInstruction",
///   fields: [],
///   kind: Instruction
/// }
///
/// OneofIr {
///   parent_message: "ProgramInstruction",
///   field_name: "ix",
///   variants: [
///     { tag: 1, variant_name: "OpenPositionIx", message_type: "OpenPositionIx" },
///     { tag: 2, variant_name: "SetLimitsIx",    message_type: "SetLimitsIx" },
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

/// Build the three messages for a single instruction:
///   - `<IxName>Accounts`  — one PubkeyBytes field per account
///   - `<IxName>Args`      — instruction arguments (delegates to `build_fields_ir`)
///   - `<IxName>Ix`        — wrapper with optional accounts + args
fn build_instruction_messages(ix: &InstructionNode, ir: &mut SchemaIr) {
    let ix_name = crate::utils::to_pascal_case(&ix.name);

    let accounts_name = format!("{ix_name}Accounts");
    let args_name = format!("{ix_name}Args");
    let payload_name = format!("{ix_name}Ix");

    let account_fields: Vec<FieldIr> = ix
        .accounts
        .iter()
        .enumerate()
        .map(|(i, acct)| FieldIr {
            name: crate::utils::to_snake_case(&acct.name),
            tag: (i + 1) as u32,
            label: LabelIr::Singular,
            field_type: FieldTypeIr::Scalar(ScalarIr::PubkeyBytes),
        })
        .collect();

    ir.push_unique_type(TypeIr {
        name: accounts_name.clone(),
        fields: account_fields,
        kind: TypeKindIr::Instruction,
    });

    let arg_fields = build_fields_ir(&args_name, &ix.arguments, ir, TypeKindIr::Helper);

    ir.push_unique_type(TypeIr {
        name: args_name.clone(),
        fields: arg_fields,
        kind: TypeKindIr::Instruction,
    });

    ir.push_unique_type(TypeIr {
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

/// Build the `ProgramInstruction` message with a `oneof ix { ... }` that dispatches
/// to each individual `<IxName>Ix` payload.
fn build_instruction_dispatch_oneof(instructions: &[InstructionNode], ir: &mut SchemaIr) {
    ir.push_unique_type(TypeIr {
        name: "ProgramInstruction".to_string(),
        fields: vec![],
        kind: TypeKindIr::Instruction,
    });

    let variants: Vec<OneofVariantIr> = instructions
        .iter()
        .enumerate()
        .map(|(i, ix)| {
            let payload_name = format!("{}Ix", crate::utils::to_pascal_case(&ix.name));

            OneofVariantIr {
                tag: (i + 1) as u32,
                variant_name: payload_name.clone(),
                message_type: payload_name,
            }
        })
        .collect();

    ir.oneofs.push(OneofIr {
        parent_message: "ProgramInstruction".to_string(),
        field_name: "ix".to_string(),
        variants,
        kind: crate::intermediate_representation::OneofKindIr::InstructionDispatch,
    });
}
