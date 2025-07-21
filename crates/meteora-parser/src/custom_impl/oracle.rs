use borsh::BorshDeserialize;

impl BorshDeserialize for crate::accounts::Oracle {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let discriminator = BorshDeserialize::deserialize_reader(reader)?;
        let idx = BorshDeserialize::deserialize_reader(reader)?;
        let active_size = BorshDeserialize::deserialize_reader(reader)?;
        let length = BorshDeserialize::deserialize_reader(reader)?;

        let mut observations = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let observation = BorshDeserialize::deserialize_reader(reader)?;
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
