use crate::intermediate_representation::{
    FieldIr, FieldTypeIr, LabelIr, ScalarIr, SchemaIr, TypeIr, TypeKindIr,
};

/// Common interface for field-like nodes from Codama (both struct fields and instruction arguments).
/// Used to deduplicate logic
pub trait FieldLikeNode {
    fn name(&self) -> &codama_nodes::CamelCaseString;
    fn r#type(&self) -> &codama_nodes::TypeNode;
    fn default_value_strategy(&self) -> Option<codama_nodes::DefaultValueStrategy>;
}

impl FieldLikeNode for codama_nodes::StructFieldTypeNode {
    fn name(&self) -> &codama_nodes::CamelCaseString { &self.name }

    fn r#type(&self) -> &codama_nodes::TypeNode { &self.r#type }

    fn default_value_strategy(&self) -> Option<codama_nodes::DefaultValueStrategy> {
        self.default_value_strategy
    }
}

impl FieldLikeNode for codama_nodes::InstructionArgumentNode {
    fn name(&self) -> &codama_nodes::CamelCaseString { &self.name }

    fn r#type(&self) -> &codama_nodes::TypeNode { &self.r#type }

    fn default_value_strategy(&self) -> Option<codama_nodes::DefaultValueStrategy> {
        self.default_value_strategy
    }
}

///
/// Build IR fields from any field-like nodes, handling inline tuple definitions by materializing new messages for them.
///
/// E.g. for a struct like:
///
/// ```rust, ignore
///   struct MyStruct {
///     field1: u64,
///     field2: (u64, String),
///   }
/// ```
///
/// We materialize:
///
/// ```protobuf
/// message MyStructField2Tuple {
///   uint64 item_0 = 1;
///   string item_1 = 2;
/// }
///
/// message MyStruct {
///   uint64 field1 = 1;
///   MyStructField2Tuple field2 = 2;
/// }
/// ```
///
pub fn build_fields_ir(
    parent_name: &str,
    fields: &[impl FieldLikeNode],
    ir: &mut SchemaIr,
    tuple_msg_kind: TypeKindIr,
) -> Vec<FieldIr> {
    let mut out = Vec::new();
    let mut tag: u32 = 0;

    for field in fields {
        if field.default_value_strategy() == Some(codama_nodes::DefaultValueStrategy::Omitted) {
            continue;
        }

        tag += 1;

        let field_name = crate::utils::to_snake_case(field.name());

        match field.r#type() {
            // Inline tuple => materialize a new message and reference it as Optional
            codama_nodes::TypeNode::Tuple(tuple) => {
                let tuple_msg_name = format!(
                    "{}{}Tuple",
                    parent_name,
                    crate::utils::to_pascal_case(&field_name)
                );

                materialize_tuple_message(&tuple_msg_name, tuple, ir, &tuple_msg_kind);

                out.push(FieldIr {
                    name: field_name,
                    tag,
                    label: LabelIr::Optional,
                    field_type: FieldTypeIr::Message(tuple_msg_name),
                });
            },

            other => {
                let (label, field_type) = map_type_with_label(other);

                out.push(FieldIr {
                    name: field_name,
                    tag,
                    label,
                    field_type,
                });
            },
        }
    }

    out
}

///
/// Recursively materialize a tuple type as a new IR message, registering it in the schema.
///
/// Each tuple item becomes a field named `item_0`, `item_1`, etc.
/// Nested tuples are recursively materialized as their own messages.
///
/// E.g. for a nested tuple `(u64, (bool, String))` with name "MyTuple":
///
/// ```protobuf
/// message MyTupleItem1Tuple {
///   bool item_0 = 1;
///   string item_1 = 2;
/// }
///
/// message MyTuple {
///   uint64 item_0 = 1;
///   MyTupleItem1Tuple item_1 = 2;
/// }
/// ```
///
fn materialize_tuple_message(
    tuple_msg_name: &str,
    tuple: &codama_nodes::TupleTypeNode,
    ir: &mut SchemaIr,
    kind: &TypeKindIr,
) {
    let mut fields = Vec::new();

    for (i, item) in tuple.items.iter().enumerate() {
        let item_name = format!("item_{}", i);
        let tag = (i + 1) as u32;

        match item {
            // Nested tuple => recurse to materialize it first, then reference the message
            codama_nodes::TypeNode::Tuple(inner_tuple) => {
                let inner_msg_name = format!(
                    "{}{}Tuple",
                    tuple_msg_name,
                    crate::utils::to_pascal_case(&item_name)
                );

                materialize_tuple_message(&inner_msg_name, inner_tuple, ir, kind);

                fields.push(FieldIr {
                    name: item_name,
                    tag,
                    label: LabelIr::Optional,
                    field_type: FieldTypeIr::Message(inner_msg_name),
                });
            },

            other => {
                let (label, field_type) = map_type_with_label(other);
                fields.push(FieldIr {
                    name: item_name,
                    tag,
                    label,
                    field_type,
                });
            },
        }
    }

    ir.push_unique_type(TypeIr {
        name: tuple_msg_name.to_string(),
        fields,
        kind: kind.clone(),
    });
}

pub fn map_type_with_label(type_node: &codama_nodes::TypeNode) -> (LabelIr, FieldTypeIr) {
    use codama_nodes::TypeNode as T;

    match type_node {
        T::Option(option) => (LabelIr::Optional, map_type(&option.item)),
        T::Array(array) => (LabelIr::Repeated, map_type(&array.item)),
        _ => (LabelIr::Singular, map_type(type_node)),
    }
}

/// Map a single Codama type node to its IR scalar/message type.
///
/// Does NOT handle wrappers like Option/Array (use `map_type_with_label` for that)
/// or Tuple (handled by `materialize_tuple_message`).
fn map_type(t: &codama_nodes::TypeNode) -> FieldTypeIr {
    use codama_nodes::{NumberFormat as NF, TypeNode as T};

    match t {
        T::String(_) | T::SizePrefix(_) => FieldTypeIr::Scalar(ScalarIr::String),
        T::Bytes(_) => FieldTypeIr::Scalar(ScalarIr::Bytes),
        T::PublicKey(_) => FieldTypeIr::Scalar(ScalarIr::PubkeyBytes),
        T::Boolean(_) => FieldTypeIr::Scalar(ScalarIr::Bool),

        T::Number(n) => match n.format {
            NF::U8 | NF::U16 | NF::U32 | NF::ShortU16 => FieldTypeIr::Scalar(ScalarIr::Uint32),
            NF::U64 => FieldTypeIr::Scalar(ScalarIr::Uint64),
            NF::I8 | NF::I16 | NF::I32 => FieldTypeIr::Scalar(ScalarIr::Int32),
            NF::I64 => FieldTypeIr::Scalar(ScalarIr::Int64),
            NF::F32 => FieldTypeIr::Scalar(ScalarIr::Float),
            NF::F64 => FieldTypeIr::Scalar(ScalarIr::Double),
            NF::U128 | NF::I128 => FieldTypeIr::Scalar(ScalarIr::Bytes),
        },

        T::Link(link) => FieldTypeIr::Message(crate::utils::to_pascal_case(&link.name)),

        // FixedSize bytes/string => bytes
        T::FixedSize(node) => match *node.r#type {
            T::Bytes(_) | T::String(_) => FieldTypeIr::Scalar(ScalarIr::Bytes),
            _ => panic!("map_type not implemented for FixedSize {:?}", node.r#type),
        },

        // Tuple must be handled at call-site via materialize_tuple_message
        T::Tuple(_) => {
            panic!("map_type() called with Tuple; handle tuple via materialize_tuple_message")
        },

        T::Option(o) => map_type(&o.item),
        T::Array(a) => map_type(&a.item),

        other => panic!("map_type not implemented for {:?}", other),
    }
}

/// Unwrap Codama's NestedTypeNode wrappers to get to the underlying StructTypeNode for accounts.
///
/// We expect the account data to always be a struct, but Codama wraps it in various ways (e.g. for fixed-size arrays) so we need to unwrap those.
pub fn unwrap_nested_struct(
    node: &codama_nodes::NestedTypeNode<codama_nodes::StructTypeNode>,
) -> &codama_nodes::StructTypeNode {
    use codama_nodes::NestedTypeNode as N;

    match node {
        N::Value(v) => v,

        N::FixedSize(w) => unwrap_nested_struct(&w.r#type),
        N::HiddenPrefix(w) => unwrap_nested_struct(&w.r#type),
        N::HiddenSuffix(w) => unwrap_nested_struct(&w.r#type),
        N::SizePrefix(w) => unwrap_nested_struct(&w.r#type),

        // If Codama adds more wrappers later, we want to fail loudly.
        other => panic!("Unsupported AccountNode.data wrapper: {:?}", other),
    }
}
