use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{Result, ResultExt};

pub fn decode_extension_ix_type<T: TryFrom<u8>>(ix_data: &[u8]) -> Result<T>
where T::Error: std::error::Error + Send + Sync + 'static {
    let first_byte: u8 = *ix_data.first().ok_or(yellowstone_vixen_parser::Error::new(
        "Instruction data for token extension is empty",
    ))?;

    T::try_from(first_byte).parse_err("Error decoding instruction data for token extension")
}

pub trait ExtensionInstructionParser: Sized {
    fn try_parse(ix: &InstructionUpdate) -> Result<Self>;
}
