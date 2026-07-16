#[derive(Debug, Clone)]
pub struct SchemaIr {
    pub types: Vec<TypeIr>,
    pub oneofs: Vec<OneofIr>,

    /// Defined types that are represented as a field rather than a message
    /// are inlined as their field label and inner type.
    ///
    /// For example, `optionBool` (a tuple with one bool) becomes a direct
    /// `bool` field instead of a wrapper struct with `item_0`. Keeping the
    /// label is important for aliases such as `Option<InlineStruct>` and
    /// `Array<Tuple>`.
    pub type_aliases: std::collections::HashMap<String, TypeAliasIr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKindIr {
    DefinedType,
    Account { len: Option<usize> },
    Instruction,
    Event,
    Helper,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeIr {
    pub name: String,
    pub fields: Vec<FieldIr>,
    // should be used later to determine how to render (e.g. accounts vs instruction args vs helper type)
    #[allow(dead_code)]
    pub kind: TypeKindIr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldIr {
    pub name: String,
    pub tag: u32,
    pub label: LabelIr,
    pub field_type: FieldTypeIr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAliasIr {
    pub label: LabelIr,
    pub field_type: FieldTypeIr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabelIr {
    Singular,
    Optional,
    Repeated,
    /// Fixed-count array (no length prefix on-chain). The value is the element count.
    /// Rendered as `Vec<T>` / `repeated` in proto for compatibility, but borsh
    /// reads/writes exactly N elements without a length prefix.
    FixedArray(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldTypeIr {
    Scalar(ScalarIr),
    Message(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScalarIr {
    Bool,

    /// On-chain u8 (1 byte). Widened to Rust u32 / proto uint32 for prost compatibility.
    U8,
    /// On-chain u16 (2 bytes). Widened to Rust u32 / proto uint32 for prost compatibility.
    U16,
    /// Solana compact u16 (1-3 bytes). Widened to Rust u32 / proto uint32.
    ShortU16,
    /// On-chain u32 (4 bytes). Native match with Rust u32 / proto uint32.
    Uint32,
    /// On-chain u64 (8 bytes). Native match with Rust u64 / proto uint64.
    Uint64,

    /// On-chain i8 (1 byte). Widened to Rust i32 / proto int32 for prost compatibility.
    I8,
    /// On-chain i16 (2 bytes). Widened to Rust i32 / proto int32 for prost compatibility.
    I16,
    /// On-chain i32 (4 bytes). Native match with Rust i32 / proto int32.
    Int32,
    /// On-chain i64 (8 bytes). Native match with Rust i64 / proto int64.
    Int64,

    /// On-chain u128 (16 bytes). Native Rust u128, proto-encoded as `bytes` (16 LE bytes).
    U128,
    /// On-chain i128 (16 bytes). Native Rust i128, proto-encoded as `bytes` (16 LE bytes).
    I128,

    Float,
    Double,
    String,
    Bytes,

    /// Fixed-size byte array with known size (no length prefix on-chain).
    /// Stored as `Vec<u8>` in Rust for prost compatibility, but borsh
    /// reads/writes exactly N bytes. Used for fixed byte arrays, etc.
    FixedBytes(usize),

    /// 32-byte pubkey. Same borsh as FixedBytes(32) but rendered as a
    /// `PublicKey` message wrapper (`message PublicKey { bytes value = 1; }`)
    /// instead of raw bytes.
    PublicKey,
}

// Enum we need to know what kind of oneof this is for rendering purposes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OneofKindIr {
    /// The top-level instruction dispatcher (e.g. PumpFun { oneof instruction { ... } })
    InstructionDispatch,
    /// The top-level event dispatcher (e.g. ProgramEvents { oneof event { ... } })
    EventDispatch,
    /// A user-defined enum (e.g. OrderType { oneof kind { ... } })
    Enum,
}

#[derive(Debug, Clone)]
pub struct OneofIr {
    pub parent_message: String, // e.g. "PumpFun" or "MyEnum"
    pub field_name: String,     // e.g. "instruction" or "kind"
    pub variants: Vec<OneofVariantIr>,
    pub kind: OneofKindIr,
}

#[derive(Debug, Clone)]
pub struct OneofVariantIr {
    pub tag: u32,
    pub variant_name: String, // e.g. "CreateInstruction" or "VariantA"
    pub message_type: String, // e.g. "CreateInstruction" or "MyEnumVariantA"
}

impl SchemaIr {
    /// Insert a generated helper type, allocating a numeric suffix when its
    /// requested name is already used by a different type.
    pub fn push_unique_type(&mut self, mut msg: TypeIr) -> String {
        let base_name = msg.name.clone();
        let mut suffix = 2;

        loop {
            if let Some(existing) = self.types.iter().find(|existing| existing.name == msg.name) {
                if existing == &msg {
                    return msg.name;
                }
            } else if !self.type_aliases.contains_key(&msg.name) {
                let name = msg.name.clone();
                self.types.push(msg);
                return name;
            }

            msg.name = format!("{base_name}{suffix}");
            suffix += 1;
        }
    }

    /// Resolve a direct reference to a defined type alias.
    ///
    /// The alias label must be returned together with its field type. Using
    /// only the inner type loses the wire-level Option/Array wrapper and can
    /// make generated code deserialize the wrong bytes.
    pub fn resolve_field(&self, label: LabelIr, field_type: FieldTypeIr) -> (LabelIr, FieldTypeIr) {
        if !matches!(label, LabelIr::Singular) {
            return (label, field_type);
        }

        let mut current_type = field_type;
        let mut seen = std::collections::HashSet::new();

        loop {
            let FieldTypeIr::Message(name) = &current_type else {
                return (LabelIr::Singular, current_type);
            };

            let Some(alias) = self.type_aliases.get(name) else {
                return (LabelIr::Singular, current_type);
            };

            // Malformed/cyclic aliases should not make macro expansion loop.
            if !seen.insert(name.clone()) {
                return (LabelIr::Singular, current_type);
            }

            if !matches!(alias.label, LabelIr::Singular) {
                return (alias.label.clone(), alias.field_type.clone());
            }

            current_type = alias.field_type.clone();
        }
    }

    /// Collect the set of top-level (defined type / helper / account) names
    /// that collide with instruction or event types.
    ///
    /// When an instruction named "swap" generates `SwapArgs` but a defined
    /// type `swapArgs` also maps to `SwapArgs`, the collision set contains
    /// `"SwapArgs"`. The renderer uses this to always emit `super::SwapArgs`
    /// for such names inside submodules.
    ///
    pub fn colliding_names(&self) -> std::collections::HashSet<String> {
        use std::collections::HashSet;

        let top_level: HashSet<&str> = self
            .types
            .iter()
            .filter(|t| !matches!(t.kind, TypeKindIr::Instruction | TypeKindIr::Event))
            .map(|t| t.name.as_str())
            .collect();

        self.types
            .iter()
            .filter(|t| matches!(t.kind, TypeKindIr::Instruction | TypeKindIr::Event))
            .filter(|t| top_level.contains(t.name.as_str()))
            .map(|t| t.name.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schema() -> SchemaIr {
        SchemaIr {
            types: Vec::new(),
            oneofs: Vec::new(),
            type_aliases: std::collections::HashMap::new(),
        }
    }

    fn message(name: &str, field_name: &str) -> TypeIr {
        TypeIr {
            name: name.to_string(),
            fields: vec![FieldIr {
                name: field_name.to_string(),
                tag: 1,
                label: LabelIr::Singular,
                field_type: FieldTypeIr::Scalar(ScalarIr::U8),
            }],
            kind: TypeKindIr::Helper,
        }
    }

    #[test]
    fn identical_type_definitions_are_deduplicated() {
        let mut ir = schema();
        assert_eq!(ir.push_unique_type(message("Shared", "value")), "Shared");
        assert_eq!(ir.push_unique_type(message("Shared", "value")), "Shared");

        assert_eq!(ir.types.len(), 1);
    }

    #[test]
    fn conflicting_type_definitions_get_unique_names() {
        let mut ir = schema();
        assert_eq!(ir.push_unique_type(message("Shared", "value")), "Shared");
        assert_eq!(
            ir.push_unique_type(message("Shared", "other_value")),
            "Shared2"
        );

        assert_eq!(ir.types.len(), 2);
    }
}
