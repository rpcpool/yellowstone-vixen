use yellowstone_vixen_core::instruction::InstructionUpdate;

use crate::{Result, ResultExt};

pub fn decode_extension_ix_type<T: TryFrom<u8>>(ix_data: &[u8]) -> Result<T>
where T::Error: std::error::Error + Send + Sync + 'static {
    T::try_from(ix_data[0]).parse_err("Error decoding instruction data for token extension")
}

pub trait ExtensionIxParser: Sized {
    fn try_parse_extension_ix(ix: &InstructionUpdate) -> Result<Self>;
}
