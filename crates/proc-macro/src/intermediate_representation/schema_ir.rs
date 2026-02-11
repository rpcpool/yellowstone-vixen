#[derive(Debug, Clone)]
pub struct SchemaIr {
    pub types: Vec<TypeIr>,
    pub oneofs: Vec<OneofIr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKindIr {
    DefinedType,
    Account { len: Option<usize> },
    Instruction,
    Helper,
}

#[derive(Debug, Clone)]
pub struct TypeIr {
    pub name: String,
    pub fields: Vec<FieldIr>,
    // should be used later to determine how to render (e.g. accounts vs instruction args vs helper type)
    #[allow(dead_code)]
    pub kind: TypeKindIr,
}

#[derive(Debug, Clone)]
pub struct FieldIr {
    pub name: String,
    pub tag: u32,
    pub label: LabelIr,
    pub field_type: FieldTypeIr,
}

#[derive(Debug, Clone)]
pub enum LabelIr {
    Singular,
    Optional,
    Repeated,
}

#[derive(Debug, Clone)]
pub enum FieldTypeIr {
    Scalar(ScalarIr),
    Message(String),
}

#[derive(Debug, Clone)]
pub enum ScalarIr {
    Bool,
    Uint32,
    Uint64,
    Int32,
    Int64,
    Float,
    Double,
    String,
    Bytes,

    /// bytes with extra semantic meaning (still `bytes` in proto)
    PubkeyBytes,
}

// Enum we need to know what kind of oneof this is for rendering purposes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OneofKindIr {
    /// The top-level instruction dispatcher (ProgramInstruction { oneof ix { ... } })
    InstructionDispatch,
    /// A user-defined enum (e.g. OrderType { oneof kind { ... } })
    Enum,
}

#[derive(Debug, Clone)]
pub struct OneofIr {
    pub parent_message: String, // e.g. "ProgramInstruction" or "MyEnum"
    pub field_name: String,     // e.g. "ix" or "kind"
    pub variants: Vec<OneofVariantIr>,
    pub kind: OneofKindIr,
}

#[derive(Debug, Clone)]
pub struct OneofVariantIr {
    pub tag: u32,
    pub variant_name: String, // e.g. "CreateIx" or "VariantA"
    pub message_type: String, // e.g. "CreateIx" or "MyEnumVariantA"
}

impl SchemaIr {
    pub fn has_type(&self, name: &str) -> bool { self.types.iter().any(|m| m.name == name) }

    pub fn push_unique_type(&mut self, msg: TypeIr) {
        if self.has_type(&msg.name) {
            return;
        }

        self.types.push(msg);
    }
}
