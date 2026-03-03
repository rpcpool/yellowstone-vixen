use codama_nodes::{DefinedTypeNode, NestedTypeNode, TypeNode};

use crate::intermediate_representation::{
    helpers::{build_fields_ir, map_type_with_label},
    FieldIr, OneofIr, OneofVariantIr, SchemaIr, TypeIr, TypeKindIr,
};

/// Converts Codama `defined_types` into our internal Intermediate Representation (IR).
///
/// In the IDL, `defined_types` are user-defined structs, enums, or tuples.
pub fn build_defined_types(defined_types: &[DefinedTypeNode], ir: &mut SchemaIr) {
    for defined_type in defined_types {
        let name = crate::utils::to_pascal_case(&defined_type.name);

        match &defined_type.r#type {
            TypeNode::Struct(struct_type) => build_defined_type_struct(&name, struct_type, ir),
            TypeNode::Tuple(tuple_type) => build_defined_type_tuple(&name, tuple_type, ir),
            TypeNode::Enum(enum_type) => build_defined_type_enum(&name, enum_type, ir),

            other => {
                // alias / scalar defined type: we don't store it
                let _ = other;
            },
        }
    }
}

///
/// IDL example:
///
/// ```text
/// defined type Fees {
///   lp_fee_bps: u64,
///   protocol_fee_bps: u64,
/// }
/// ```
///
/// IR result:
///
/// ```rust, ignore
/// TypeIr {
///   name: "Fees",
///   fields: [
///     { name: "lp_fee_bps",       tag: 1, field_type: Uint64 },
///     { name: "protocol_fee_bps", tag: 2, field_type: Uint64 },
///   ],
///   kind: DefinedType
/// }
/// ```
///
fn build_defined_type_struct(
    name: &str,
    struct_type_node: &codama_nodes::StructTypeNode,
    ir: &mut SchemaIr,
) {
    let fields = build_fields_ir(name, &struct_type_node.fields, ir, TypeKindIr::Helper);

    ir.types.push(TypeIr {
        name: name.to_string(),
        fields,
        kind: TypeKindIr::DefinedType,
    });
}

///
/// IDL example:
///
/// ```text
/// defined type Limits = (u64, string);
/// ```
///
/// IR result:
///
/// ```rust, ignore
/// TypeIr {
///   name: "Limits",
///   fields: [
///     { name: "item_0", tag: 1, field_type: Uint64 },
///     { name: "item_1", tag: 2, field_type: String },
///   ],
///   kind: DefinedType
/// }
/// ```
///
fn build_defined_type_tuple(
    name: &str,
    tuple_type_node: &codama_nodes::TupleTypeNode,
    ir: &mut SchemaIr,
) {
    let mut fields = Vec::with_capacity(tuple_type_node.items.len());

    for (i, item) in tuple_type_node.items.iter().enumerate() {
        let (label, field_type) = map_type_with_label(item);

        fields.push(FieldIr {
            name: format!("item_{}", i),
            tag: (i + 1) as u32,
            label,
            field_type,
        });
    }

    ir.types.push(TypeIr {
        name: name.to_string(),
        fields,
        kind: TypeKindIr::DefinedType,
    });
}

///
/// IDL example:
///
/// ```text
/// enum OrderType {
///   Market,
///   Limit(u64),
///   Stop { price: u64 }
/// }
/// ```
///
/// IR result:
///
/// ```rust, ignore
/// // Helper types:
/// TypeIr { name: "OrderTypeMarket", fields: [] }
/// TypeIr { name: "OrderTypeLimit",  fields: [item_0: Uint64] }
/// TypeIr { name: "OrderTypeStop",   fields: [price: Uint64] }
///
/// // Parent enum type:
/// TypeIr { name: "OrderType", fields: [], kind: DefinedType }
///
/// // Oneof:
/// OneofIr {
///   parent_message: "OrderType",
///   field_name: "kind",
///   variants: [
///     { tag: 1, variant_name: "Market", message_type: "OrderTypeMarket" },
///     { tag: 2, variant_name: "Limit",  message_type: "OrderTypeLimit" },
///     { tag: 3, variant_name: "Stop",   message_type: "OrderTypeStop" },
///   ]
/// }
/// ```
///
fn build_defined_type_enum(name: &str, en: &codama_nodes::EnumTypeNode, ir: &mut SchemaIr) {
    let enum_name = name.to_string();
    let mut variants = Vec::with_capacity(en.variants.len());

    for (i, variant) in en.variants.iter().enumerate() {
        let tag = (i + 1) as u32;

        let (variant_name, payload_fields) = build_enum_variant_payload(&enum_name, variant, ir);

        let payload_name = format!("{}{}", enum_name, variant_name);

        ir.types.push(TypeIr {
            name: payload_name.clone(),
            fields: payload_fields,
            kind: TypeKindIr::Helper,
        });

        variants.push(OneofVariantIr {
            tag,
            variant_name,
            message_type: payload_name,
        });
    }

    // Parent enum type (fields empty; renderer injects the oneof field)
    ir.types.push(TypeIr {
        name: enum_name.clone(),
        fields: vec![],
        kind: TypeKindIr::DefinedType,
    });

    ir.oneofs.push(OneofIr {
        parent_message: enum_name,
        field_name: "kind".to_string(),
        variants,
        kind: crate::intermediate_representation::OneofKindIr::Enum,
    });
}

/// Builds the payload fields for an enum variant (Empty / Tuple / Struct),
/// and returns the variant name (PascalCase) + fields for the payload message.
fn build_enum_variant_payload(
    enum_name: &str,
    variant: &codama_nodes::EnumVariantTypeNode,
    ir: &mut SchemaIr,
) -> (String, Vec<FieldIr>) {
    match variant {
        codama_nodes::EnumVariantTypeNode::Empty(variant) => {
            (crate::utils::to_pascal_case(&variant.name), vec![])
        },

        codama_nodes::EnumVariantTypeNode::Tuple(variant) => {
            let variant_name = crate::utils::to_pascal_case(&variant.name);

            let tuple = match &variant.tuple {
                NestedTypeNode::Value(tuple) => tuple,
                _ => panic!("enum tuple variant payload must be NestedTypeNode::Value"),
            };

            (variant_name, build_tuple_payload_fields(&tuple.items))
        },

        codama_nodes::EnumVariantTypeNode::Struct(variant) => {
            let variant_name = crate::utils::to_pascal_case(&variant.name);

            let struct_type = match &variant.r#struct {
                NestedTypeNode::Value(struct_type) => struct_type,
                _ => panic!("enum struct variant payload must be NestedTypeNode::Value"),
            };

            let payload_name = format!("{}{}", enum_name, variant_name);

            let fields =
                build_fields_ir(&payload_name, &struct_type.fields, ir, TypeKindIr::Helper);

            (variant_name, fields)
        },
    }
}

/// For tuple-variant payloads: generate item_0..item_n fields.
fn build_tuple_payload_fields(items: &[codama_nodes::TypeNode]) -> Vec<FieldIr> {
    let mut fields = Vec::with_capacity(items.len());

    for (j, item) in items.iter().enumerate() {
        let (label, field_type) = map_type_with_label(item);

        fields.push(FieldIr {
            name: format!("item_{}", j),
            tag: (j + 1) as u32,
            label,
            field_type,
        });
    }

    fields
}
