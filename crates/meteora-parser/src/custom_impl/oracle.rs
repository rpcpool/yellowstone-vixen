use borsh::BorshDeserialize;

use crate::{accounts::oracle::Oracle, proto_helpers::proto_types_parsers::IntoProto};

impl BorshDeserialize for Oracle {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let discriminator: [u8; 8] = BorshDeserialize::deserialize_reader(reader)?;
        let idx = BorshDeserialize::deserialize_reader(reader)?;
        let active_size = BorshDeserialize::deserialize_reader(reader)?;
        let length = BorshDeserialize::deserialize_reader(reader)?;

        let mut observations = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let observation: [u8; 32] = BorshDeserialize::deserialize_reader(reader)?;
            observations.push(observation);
        }

        Ok(Self {
            discriminator,
            idx,
            active_size,
            length,
            observations,
        })
    }
}

impl IntoProto<crate::proto_def::RepeatedUint32Row> for [u8; 32] {
    fn into_proto(self) -> crate::proto_def::RepeatedUint32Row {
        crate::proto_def::RepeatedUint32Row {
            rows: self.into_iter().map(|el| el as u32).collect(),
        }
    }
}
