use std::{collections::HashSet, fmt::Write};

use crate::intermediate_representation::{
    FieldTypeIr, LabelIr, OneofIr, ScalarIr, SchemaIr, TypeIr,
};

/// Render the IR schema as a .proto string.
pub fn proto_schema_string(schema: &SchemaIr, package: &str) -> String {
    let mut out = String::new();

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

    // Regular types (excluding oneof parents)
    for t in &schema.types {
        if oneof_parents.contains(t.name.as_str()) {
            continue;
        }

        render_type(&mut out, t);
    }

    // Oneof parent types
    for oneof in &schema.oneofs {
        render_oneof_parent(&mut out, oneof);
    }

    out
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
        ScalarIr::Uint32 => "uint32",
        ScalarIr::Uint64 => "uint64",
        ScalarIr::Int32 => "int32",
        ScalarIr::Int64 => "int64",
        ScalarIr::Float => "float",
        ScalarIr::Double => "double",
        ScalarIr::String => "string",
        ScalarIr::Bytes => "bytes",
        ScalarIr::PubkeyBytes => "bytes",
    }
}
