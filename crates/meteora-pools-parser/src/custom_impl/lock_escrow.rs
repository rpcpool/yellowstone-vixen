impl borsh::de::BorshDeserialize for crate::accounts::LockEscrow {
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        let discriminator = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let pool = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let owner = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let escrow_vault = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let bump = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let total_locked_amount = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let lp_per_token = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let unclaimed_fee_pending = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let a_fee = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let b_fee = borsh::BorshDeserialize::deserialize_reader(reader)?;

        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        if !buf.is_empty() {
            if buf.iter().any(|b| *b != 0) {
                return Err(borsh::maybestd::io::Error::new(
                    borsh::maybestd::io::ErrorKind::InvalidData,
                    "Not zeroed end padding",
                ));
            } else {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    name: "zeroed_end_padding",
                    name = "zeroed_end_padding",
                    length = buf.len(),
                    program = crate::ID.to_string(),
                    type_kind = "account",
                    type_name = "LockEscrow"
                );
            }
        }

        Ok(Self {
            discriminator,
            pool,
            owner,
            escrow_vault,
            bump,
            total_locked_amount,
            lp_per_token,
            unclaimed_fee_pending,
            a_fee,
            b_fee,
        })
    }
}
