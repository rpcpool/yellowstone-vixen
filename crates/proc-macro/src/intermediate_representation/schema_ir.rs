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
    /// Fixed-count array (no length prefix on-chain). The value is the element count.
    /// Rendered as `Vec<T>` / `repeated` in proto for compatibility, but borsh
    /// reads/writes exactly N elements without a length prefix.
    FixedArray(usize),
}

#[derive(Debug, Clone)]
pub enum FieldTypeIr {
    Scalar(ScalarIr),
    Message(String),
}

#[derive(Debug, Clone)]
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

    Float,
    Double,
    String,
    Bytes,

    /// Fixed-size byte array with known size (no length prefix on-chain).
    /// Stored as `Vec<u8>` in Rust for prost compatibility, but borsh
    /// reads/writes exactly N bytes. Used for u128/i128 (16), pubkeys (32),
    /// fixed byte arrays, etc.
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
    pub fn has_type(&self, name: &str) -> bool { self.types.iter().any(|m| m.name == name) }

    pub fn push_unique_type(&mut self, msg: TypeIr) {
        if self.has_type(&msg.name) {
            return;
        }

        self.types.push(msg);
    }
}
