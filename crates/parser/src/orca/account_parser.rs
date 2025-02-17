use std::borrow::Cow;

use orca_whirlpools_client::accounts::{FeeTier, Position, TickArray, Whirlpool, WhirlpoolsConfig};
use spl_pod::solana_program::program_error::ProgramError;
use yellowstone_vixen_core::{
    AccountUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum OrcaProgramState {
    Whirlpool(Whirlpool),
    WhirlpoolsConfig(WhirlpoolsConfig),
    FeeTier(FeeTier),
    Position(Position),
    TickArray(TickArray),
}

impl OrcaProgramState {
    pub fn try_unpack(data_bytes: &[u8]) -> ParseResult<Self> {
        let data_len = data_bytes.len();
        match data_len {
            Whirlpool::LEN => Ok(OrcaProgramState::Whirlpool(Whirlpool::from_bytes(
                data_bytes,
            )?)),
            WhirlpoolsConfig::LEN => Ok(OrcaProgramState::WhirlpoolsConfig(
                WhirlpoolsConfig::from_bytes(data_bytes)?,
            )),
            FeeTier::LEN => Ok(OrcaProgramState::FeeTier(FeeTier::from_bytes(data_bytes)?)),
            Position::LEN => Ok(OrcaProgramState::Position(Position::from_bytes(
                data_bytes,
            )?)),
            TickArray::LEN => Ok(OrcaProgramState::TickArray(TickArray::from_bytes(
                data_bytes,
            )?)),
            _ => Err(ParseError::from("Invalid Account data length".to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AccountParser;

impl Parser for AccountParser {
    type Input = AccountUpdate;
    type Output = OrcaProgramState;

    fn id(&self) -> Cow<str> { "orca::AccountParser".into() }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([orca_whirlpools_client::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acct.account.as_ref().ok_or(ProgramError::InvalidArgument)?;
        OrcaProgramState::try_unpack(&inner.data)
    }
}

impl ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        orca_whirlpools_client::ID.to_bytes().into()
    }
}

#[cfg(feature = "proto")]
mod proto_parser {

    use orca_whirlpools_client::types::{PositionRewardInfo, Tick, WhirlpoolRewardInfo};
    use yellowstone_vixen_core::proto::ParseProto;
    use yellowstone_vixen_proto::parser::{
        orca_program_state_proto, FeeTierProto, OrcaPositionRewardInfoProto, OrcaProgramStateProto,
        OrcaTickArrayProto, OrcaTickProto, PositionProto, WhirlpoolProto, WhirlpoolRewardInfoProto,
        WhirlpoolsConfigProto,
    };

    use super::{
        AccountParser, FeeTier, OrcaProgramState, Position, TickArray, Whirlpool, WhirlpoolsConfig,
    };
    use crate::helpers::IntoProto;

    impl IntoProto<WhirlpoolRewardInfoProto> for WhirlpoolRewardInfo {
        fn into_proto(self) -> WhirlpoolRewardInfoProto {
            WhirlpoolRewardInfoProto {
                mint: self.mint.to_string(),
                vault: self.vault.to_string(),
                authority: self.authority.to_string(),
                emissions_per_second_x64: self.emissions_per_second_x64.to_string(),
                growth_global_x64: self.growth_global_x64.to_string(),
            }
        }
    }

    impl IntoProto<WhirlpoolProto> for Whirlpool {
        fn into_proto(self) -> WhirlpoolProto {
            WhirlpoolProto {
                discriminator: self.discriminator.to_vec(),
                whirlpools_config: self.whirlpools_config.to_string(),
                whirlpool_bump: self.whirlpool_bump[0].into(),
                tick_spacing: self.tick_spacing.into(),
                tick_spacing_seed: self.tick_spacing_seed.to_vec(),
                fee_rate: self.fee_rate.into(),
                protocol_fee_rate: self.protocol_fee_rate.into(),
                liquidity: self.liquidity.to_string(),
                sqrt_price: self.sqrt_price.to_string(),
                tick_current_index: self.tick_current_index,
                protocol_fee_owed_a: self.protocol_fee_owed_a,
                protocol_fee_owed_b: self.protocol_fee_owed_b,
                token_mint_a: self.token_mint_a.to_string(),
                token_vault_a: self.token_vault_a.to_string(),
                token_mint_b: self.token_mint_b.to_string(),
                token_vault_b: self.token_vault_b.to_string(),
                fee_growth_global_a: self.fee_growth_global_a.to_string(),
                fee_growth_global_b: self.fee_growth_global_b.to_string(),
                reward_last_updated_timestamp: self.reward_last_updated_timestamp,
                reward_infos: self
                    .reward_infos
                    .into_iter()
                    .map(IntoProto::into_proto)
                    .collect(),
            }
        }
    }

    impl IntoProto<WhirlpoolsConfigProto> for WhirlpoolsConfig {
        fn into_proto(self) -> WhirlpoolsConfigProto {
            WhirlpoolsConfigProto {
                discriminator: self.discriminator.to_vec(),
                fee_authority: self.fee_authority.to_string(),
                collect_protocol_fees_authority: self.collect_protocol_fees_authority.to_string(),
                reward_emissions_super_authority: self.reward_emissions_super_authority.to_string(),
                default_protocol_fee_rate: self.default_protocol_fee_rate.into(),
            }
        }
    }

    impl IntoProto<FeeTierProto> for FeeTier {
        fn into_proto(self) -> FeeTierProto {
            FeeTierProto {
                discriminator: self.discriminator.to_vec(),
                whirlpools_config: self.whirlpools_config.to_string(),
                tick_spacing: self.tick_spacing.into(),
                default_fee_rate: self.default_fee_rate.into(),
            }
        }
    }

    impl IntoProto<OrcaPositionRewardInfoProto> for PositionRewardInfo {
        fn into_proto(self) -> OrcaPositionRewardInfoProto {
            OrcaPositionRewardInfoProto {
                growth_inside_checkpoint: self.growth_inside_checkpoint.to_string(),
                amount_owed: self.amount_owed,
            }
        }
    }

    impl IntoProto<PositionProto> for Position {
        fn into_proto(self) -> PositionProto {
            PositionProto {
                discriminator: self.discriminator.to_vec(),
                whirlpool: self.whirlpool.to_string(),
                position_mint: self.position_mint.to_string(),
                liquidity: self.liquidity.to_string(),
                tick_lower_index: self.tick_lower_index,
                tick_upper_index: self.tick_upper_index,
                fee_growth_checkpoint_a: self.fee_growth_checkpoint_a.to_string(),
                fee_growth_checkpoint_b: self.fee_growth_checkpoint_b.to_string(),
                fee_owed_a: self.fee_owed_a,
                fee_owed_b: self.fee_owed_b,
                reward_infos: self
                    .reward_infos
                    .into_iter()
                    .map(IntoProto::into_proto)
                    .collect(),
            }
        }
    }

    impl IntoProto<OrcaTickProto> for Tick {
        fn into_proto(self) -> OrcaTickProto {
            OrcaTickProto {
                initialized: self.initialized,
                liquidity_gross: self.liquidity_gross.to_string(),
                liquidity_net: self.liquidity_net.to_string(),
                fee_growth_outside_a: self.fee_growth_outside_a.to_string(),
                fee_growth_outside_b: self.fee_growth_outside_b.to_string(),
                reward_growths_outside: self
                    .reward_growths_outside
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
            }
        }
    }

    impl IntoProto<OrcaTickArrayProto> for TickArray {
        fn into_proto(self) -> OrcaTickArrayProto {
            OrcaTickArrayProto {
                discriminator: self.discriminator.to_vec(),
                start_tick_index: self.start_tick_index,
                ticks: self.ticks.into_iter().map(IntoProto::into_proto).collect(),
                whirlpool: self.whirlpool.to_string(),
            }
        }
    }

    impl ParseProto for AccountParser {
        type Message = OrcaProgramStateProto;

        fn output_into_message(value: Self::Output) -> Self::Message {
            let state_oneof = match value {
                OrcaProgramState::Whirlpool(data) => Some(
                    orca_program_state_proto::StateOneof::Whirlpool(data.into_proto()),
                ),
                OrcaProgramState::WhirlpoolsConfig(data) => Some(
                    orca_program_state_proto::StateOneof::WhirlpoolsConfig(data.into_proto()),
                ),
                OrcaProgramState::FeeTier(data) => Some(
                    orca_program_state_proto::StateOneof::FeeTier(data.into_proto()),
                ),
                OrcaProgramState::Position(data) => Some(
                    orca_program_state_proto::StateOneof::Position(data.into_proto()),
                ),
                OrcaProgramState::TickArray(data) => Some(
                    orca_program_state_proto::StateOneof::TickArray(data.into_proto()),
                ),
            };
            Self::Message { state_oneof }
        }
    }
}

#[cfg(test)]
mod tests {
    use yellowstone_vixen_mock::{account_fixture, run_account_parse, FixtureData};

    use super::*;

    #[tokio::test]
    async fn test_whirlpool_account_parsing() {
        let parser = AccountParser;

        let account = account_fixture!("56Ekyu6uBpTna3LR2qkjHjNbDNkCLWK2Lt3uWh7J2R8Z", &parser);

        if let OrcaProgramState::Whirlpool(whirlpool) = account {
            assert_eq!(
                whirlpool.whirlpools_config.to_string(),
                "2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ".to_string()
            );
        } else {
            panic!("Invalid parsed data");
        }
    }
}
