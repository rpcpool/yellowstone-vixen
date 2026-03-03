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

///
/// Render the IR schema as a .proto string and compute dispatch message indices.
///
/// `program_name` is the PascalCase program name (e.g. "PumpFun").
///
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

    // PublicKey wrapper message
    {
        writeln!(&mut out, "message PublicKey {{").unwrap();
        writeln!(&mut out, "  bytes value = 1;").unwrap();
        writeln!(&mut out, "}}\n    ").unwrap();

        message_count += 1;
    }

    // Oneof parents are rendered separately below — skip them here to avoid duplicates.
    let oneof_parents: HashSet<&str> = schema
        .oneofs
        .iter()
        .map(|o| o.parent_message.as_str())
        .collect();

    // Render all non-oneof-parent messages
    {
        let mut seen_names: HashSet<&str> = HashSet::new();

        for t in &schema.types {
            // Do not render oneof parent messages here, they are rendered separately below and we want to avoid duplicates.
            if oneof_parents.contains(t.name.as_str()) {
                continue;
            }

            // Skip if we've already rendered a message with the same name (can happen with DefinedType/Instruction name collisions in the IR).
            if !seen_names.insert(t.name.as_str()) {
                continue;
            }

            render_type(&mut out, t);

            message_count += 1;
        }
    }

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
            LabelIr::Repeated | LabelIr::FixedArray(_) => "repeated ",
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

    for (i, account) in account_types.iter().enumerate() {
        let tag = i + 1;
        let field_name = crate::utils::to_snake_case(&account.name);

        writeln!(out, "    {} {} = {};", account.name, field_name, tag).unwrap();
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
        ScalarIr::PublicKey => "PublicKey",
    }
}
