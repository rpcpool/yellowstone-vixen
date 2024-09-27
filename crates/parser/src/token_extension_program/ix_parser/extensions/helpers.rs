use yellowstone_vixen_core::instruction::InstructionUpdate;

pub fn decode_extension_ix_type<T: TryFrom<u8>>(ix_data: &[u8]) -> Result<T, String> {
    T::try_from(ix_data[0]).map_err(|_| "Error decoding extension ix data".to_owned())
}

pub trait ExtensionIxParser {
    fn try_parse_extension_ix(ix: &InstructionUpdate) -> Result<Self, String>
    where Self: Sized;
}
