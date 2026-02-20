use std::{collections::HashSet, fmt::Write};

use crate::intermediate_representation::{
    FieldTypeIr, LabelIr, OneofIr, OneofKindIr, ScalarIr, SchemaIr, TypeIr, TypeKindIr,
};

/// Output of proto schema rendering: the schema string plus dispatch message indices.
pub struct ProtoSchemaOutput {
    pub schema: String,
    /// 0-based index of the `{ProgramName}Account` message in the proto file descriptor.
    /// `None` if the program has no accounts.
    pub account_dispatch_index: Option<usize>,
    /// 0-based index of the `Instructions` message in the proto file descriptor.
    /// `None` if the program has no instructions.
    pub instruction_dispatch_index: Option<usize>,
}

/// Render the IR schema as a .proto string and compute dispatch message indices.
///
/// `program_name` is the PascalCase program name (e.g. "PumpFun").
pub fn proto_schema_string(
    schema: &SchemaIr,
    package: &str,
    program_name: &str,
) -> ProtoSchemaOutput {
    let mut out = String::new();
    let mut message_count: usize = 0;

    // Header
    {
        writeln!(&mut out, r#"syntax = "proto3";"#).unwrap();
        writeln!(&mut out).unwrap();

        if !package.is_empty() {
            writeln!(&mut out, "package {};", package).unwrap();
            writeln!(&mut out).unwrap();
        }
    }

    // Oneof parents are rendered separately below — skip them here to avoid duplicates.
    let oneof_parents: HashSet<&str> = schema
        .oneofs
        .iter()
        .map(|o| o.parent_message.as_str())
        .collect();

    // Regular types (excluding oneof parents, deduped by name to handle
    // DefinedType/Instruction name collisions from the IR).
    let mut seen_names: HashSet<&str> = HashSet::new();

    for t in &schema.types {
        if oneof_parents.contains(t.name.as_str()) {
            continue;
        }

        if !seen_names.insert(t.name.as_str()) {
            continue;
        }

        render_type(&mut out, t);

        message_count += 1;
    }

    // Account dispatch message: {ProgramName}Account { oneof account { ... } }
    // Only rendered when the program has at least one account type.
    let account_types: Vec<&TypeIr> = schema
        .types
        .iter()
        .filter(|t| matches!(t.kind, TypeKindIr::Account { .. }))
        .collect();

    let account_dispatch_index = if account_types.is_empty() {
        None
    } else {
        let idx = message_count;

        render_account_dispatch(&mut out, program_name, &account_types);
        message_count += 1;

        Some(idx)
    };

    // Oneof parent types (enum oneofs + instruction dispatch).
    // Skip oneofs with no variants (empty oneof is invalid proto).
    let mut instruction_dispatch_index = None;

    for oneof in &schema.oneofs {
        if oneof.variants.is_empty() {
            continue;
        }

        if oneof.kind == OneofKindIr::InstructionDispatch {
            instruction_dispatch_index = Some(message_count);
        }

        render_oneof_parent(&mut out, oneof);

        message_count += 1;
    }

    let _ = message_count;

    ProtoSchemaOutput {
        schema: out,
        account_dispatch_index,
        instruction_dispatch_index,
    }
}

fn render_type(out: &mut String, msg: &TypeIr) {
    writeln!(out, "message {} {{", msg.name).unwrap();

    for field in &msg.fields {
        let label = match field.label {
            LabelIr::Singular => "",
            LabelIr::Optional => "optional ",
            LabelIr::Repeated => "repeated ",
        };

        let field_type = match &field.field_type {
            FieldTypeIr::Scalar(s) => scalar_to_proto(s),
            FieldTypeIr::Message(name) => name.as_str(),
        };

        writeln!(
            out,
            "  {}{} {} = {};",
            label, field_type, field.name, field.tag
        )
        .unwrap();
    }

    writeln!(
        out,
        "}}
    "
    )
    .unwrap();
}

///
/// Render the account dispatch message:
///
/// ```protobuf
/// message PumpFunAccount {
///   oneof account {
///     BondingCurve bonding_curve = 1;
///     Global global = 2;
///     ...
///   }
/// }
/// ```
///
fn render_account_dispatch(out: &mut String, program_name: &str, account_types: &[&TypeIr]) {
    let message_name = format!("{program_name}Account");

    writeln!(out, "message {} {{", message_name).unwrap();
    writeln!(out, "  oneof account {{").unwrap();

    for (i, acct) in account_types.iter().enumerate() {
        let tag = i + 1;
        let field_name = crate::utils::to_snake_case(&acct.name);

        writeln!(out, "    {} {} = {};", acct.name, field_name, tag).unwrap();
    }

    writeln!(out, "  }}").unwrap();
    writeln!(out, "}}").unwrap();
}

fn render_oneof_parent(out: &mut String, oneof: &OneofIr) {
    writeln!(out, "message {} {{", oneof.parent_message).unwrap();
    writeln!(out, "  oneof {} {{", oneof.field_name).unwrap();

    for v in &oneof.variants {
        let field_name = crate::utils::to_snake_case(&v.variant_name);

        writeln!(out, "    {} {} = {};", v.message_type, field_name, v.tag).unwrap();
    }

    writeln!(out, "  }}").unwrap();
    writeln!(out, "}}").unwrap();
}

fn scalar_to_proto(s: &ScalarIr) -> &'static str {
    match s {
        ScalarIr::Bool => "bool",
        ScalarIr::U8 | ScalarIr::U16 | ScalarIr::ShortU16 | ScalarIr::Uint32 => "uint32",
        ScalarIr::Uint64 => "uint64",
        ScalarIr::I8 | ScalarIr::I16 | ScalarIr::Int32 => "int32",
        ScalarIr::Int64 => "int64",
        ScalarIr::Float => "float",
        ScalarIr::Double => "double",
        ScalarIr::String => "string",
        ScalarIr::Bytes => "bytes",
        ScalarIr::FixedBytes(_) => "bytes",
        ScalarIr::PubkeyBytes => "bytes",
    }
}
