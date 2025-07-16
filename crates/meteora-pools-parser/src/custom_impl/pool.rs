impl borsh::BorshDeserialize for crate::accounts::Pool {
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        let discriminator = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let lp_mint = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let token_a_mint = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let token_b_mint = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let a_vault = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let b_vault = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let a_vault_lp = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let b_vault_lp = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let a_vault_lp_bump = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let enabled = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let protocol_token_a_fee = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let protocol_token_b_fee = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let fee_last_updated_at = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let padding0 = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let fees = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let pool_type = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let stake = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let total_locked_lp = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let bootstrapping = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let partner_info = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let padding = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let curve_type = borsh::BorshDeserialize::deserialize_reader(reader)?;

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
                    type_name = "Pool"
                );
            }
        }

        Ok(Self {
            discriminator,
            lp_mint,
            token_a_mint,
            token_b_mint,
            a_vault,
            b_vault,
            a_vault_lp,
            b_vault_lp,
            a_vault_lp_bump,
            enabled,
            protocol_token_a_fee,
            protocol_token_b_fee,
            fee_last_updated_at,
            padding0,
            fees,
            pool_type,
            stake,
            total_locked_lp,
            bootstrapping,
            partner_info,
            padding,
            curve_type,
        })
    }
}
