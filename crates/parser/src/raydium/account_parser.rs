use borsh::BorshDeserialize;
use spl_pod::solana_program::program_error::ProgramError;
use yellowstone_vixen_core::{ParseError, ParseResult, Parser, Prefilter, ProgramParser};

use super::account_helpers::{
    AmmConfig, ObservationState, OperationState, PersonalPositionState, PoolState,
    ProtocolPositionState, TickArrayBitmapExtension, TickArrayState,
};
use crate::{helpers::ACC_DISCRIMINATOR_SIZE, raydium::RADIUM_V3_PROGRAM_ID};

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum RaydiumProgramState {
    AmmConfig(AmmConfig),
    OperationState(OperationState),
    ObservationState(ObservationState),
    PersonalPositionState(PersonalPositionState),
    PoolState(PoolState),
    ProtocolPositionState(ProtocolPositionState),
    TickArrayState(TickArrayState),
    TickArrayBitmapExtension(TickArrayBitmapExtension),
}

impl RaydiumProgramState {
    pub fn try_unpack(data_bytes: &[u8]) -> ParseResult<Self> {
        let data_len = data_bytes.len();
        let data_bytes = &data_bytes[ACC_DISCRIMINATOR_SIZE..];

        match data_len {
            AmmConfig::LEN => Ok(RaydiumProgramState::AmmConfig(AmmConfig::try_from_slice(
                data_bytes,
            )?)),
            OperationState::LEN => Ok(RaydiumProgramState::OperationState(
                OperationState::try_from_slice(data_bytes)?,
            )),
            ObservationState::LEN => Ok(RaydiumProgramState::ObservationState(
                ObservationState::try_from_slice(data_bytes)?,
            )),
            PersonalPositionState::LEN => Ok(RaydiumProgramState::PersonalPositionState(
                PersonalPositionState::try_from_slice(data_bytes)?,
            )),
            PoolState::LEN => Ok(RaydiumProgramState::PoolState(PoolState::try_from_slice(
                data_bytes,
            )?)),
            ProtocolPositionState::LEN => Ok(RaydiumProgramState::ProtocolPositionState(
                ProtocolPositionState::try_from_slice(data_bytes)?,
            )),
            TickArrayState::LEN => Ok(RaydiumProgramState::TickArrayState(
                TickArrayState::try_from_slice(data_bytes)?,
            )),
            TickArrayBitmapExtension::LEN => Ok(RaydiumProgramState::TickArrayBitmapExtension(
                TickArrayBitmapExtension::try_from_slice(data_bytes)?,
            )),
            _ => Err(ParseError::from("Invalid Account data length".to_owned())),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AccountParser;

impl Parser for AccountParser {
    type Input = yellowstone_vixen_core::AccountUpdate;
    type Output = RaydiumProgramState;

    fn id(&self) -> std::borrow::Cow<str> { "raydium::AccountParser".into() }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([RADIUM_V3_PROGRAM_ID])
            .build()
            .unwrap()
    }

    async fn parse(
        &self,
        acct: &yellowstone_vixen_core::AccountUpdate,
    ) -> ParseResult<Self::Output> {
        let inner = acct.account.as_ref().ok_or(ProgramError::InvalidArgument)?;
        RaydiumProgramState::try_unpack(&inner.data)
    }
}

impl ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        RADIUM_V3_PROGRAM_ID.to_bytes().into()
    }
}

#[cfg(feature = "proto")]
mod proto_parser {
    use yellowstone_vixen_core::proto::ParseProto;
    use yellowstone_vixen_proto::parser::raydium::{
        program_state as raydium_program_state_proto, AmmConfigProto, ObservationProto,
        ObservationStateProto, OperationStateProto, PersonalPositionStateProto, PoolStateProto,
        ProgramState as RaydiumProgramStateProto, ProtocolPositionStateProto,
        RaydiumPositionRewardInfoProto, RaydiumTickArrayStateProto, RaydiumTickStateProto,
        RewardInfoProto, TickArrayBitmapExtensionProto, TickArrayBitmapProto,
    };

    use super::{AccountParser, RaydiumProgramState};
    use crate::{
        helpers::{proto::FromVecPubkeyToVecString, IntoProto},
        raydium::{
            AmmConfig, Observation, ObservationState, OperationState, PersonalPositionState,
            PoolState, PositionRewardInfo, ProtocolPositionState, RewardInfo,
            TickArrayBitmapExtension, TickArrayState, TickState,
        },
    };

    impl IntoProto<AmmConfigProto> for AmmConfig {
        fn into_proto(self) -> AmmConfigProto {
            AmmConfigProto {
                bump: self.bump.into(),
                index: self.index.into(),
                owner: self.owner.to_string(),
                protocol_fee_rate: self.protocol_fee_rate,
                trade_fee_rate: self.trade_fee_rate,
                padding: self.padding.to_vec(),
                padding_u32: self.padding_u32,
                fund_owner: self.fund_owner.to_string(),
                tick_spacing: self.tick_spacing.into(),
                fund_fee_rate: self.fund_fee_rate,
            }
        }
    }

    impl IntoProto<OperationStateProto> for OperationState {
        fn into_proto(self) -> OperationStateProto {
            OperationStateProto {
                bump: self.bump.into(),
                operation_owners: self.operation_owners.to_vec().to_string_vec(),
                whitelist_mints: self.whitelist_mints.to_vec().to_string_vec(),
            }
        }
    }

    impl IntoProto<ObservationProto> for Observation {
        fn into_proto(self) -> ObservationProto {
            ObservationProto {
                block_timestamp: self.block_timestamp,
                tick_cumulative: self.tick_cumulative,
                padding: self.padding.to_vec(),
            }
        }
    }

    impl IntoProto<ObservationStateProto> for ObservationState {
        fn into_proto(self) -> ObservationStateProto {
            ObservationStateProto {
                initialized: self.initialized,
                recent_epoch: self.recent_epoch,
                observation_index: self.observation_index.into(),
                pool_id: self.pool_id.to_string(),
                observations: self
                    .observations
                    .into_iter()
                    .map(IntoProto::into_proto)
                    .collect(),
                padding: self.padding.to_vec(),
            }
        }
    }

    impl IntoProto<RaydiumPositionRewardInfoProto> for PositionRewardInfo {
        fn into_proto(self) -> RaydiumPositionRewardInfoProto {
            RaydiumPositionRewardInfoProto {
                growth_inside_last_x64: self.growth_inside_last_x64.to_string(),
                reward_amount_owed: self.reward_amount_owed,
            }
        }
    }

    impl IntoProto<PersonalPositionStateProto> for PersonalPositionState {
        fn into_proto(self) -> PersonalPositionStateProto {
            PersonalPositionStateProto {
                bump: self.bump.into(),
                nft_mint: self.nft_mint.to_string(),
                pool_id: self.pool_id.to_string(),
                tick_lower_index: self.tick_lower_index,
                tick_upper_index: self.tick_upper_index,
                liquidity: self.liquidity.to_string(),
                fee_growth_inside_0_last_x64: self.fee_growth_inside_0_last_x64.to_string(),
                fee_growth_inside_1_last_x64: self.fee_growth_inside_1_last_x64.to_string(),
                token_fees_owed_0: self.token_fees_owed_0,
                token_fees_owed_1: self.token_fees_owed_1,
                recent_epoch: self.recent_epoch,
                reward_infos: self
                    .reward_infos
                    .into_iter()
                    .map(IntoProto::into_proto)
                    .collect(),
                padding: self.padding.to_vec(),
            }
        }
    }

    impl IntoProto<RewardInfoProto> for RewardInfo {
        fn into_proto(self) -> RewardInfoProto {
            RewardInfoProto {
                reward_state: self.reward_state.into(),
                open_time: self.open_time,
                end_time: self.end_time,
                last_update_time: self.last_update_time,
                emissions_per_second_x64: self.emissions_per_second_x64.to_string(),
                reward_total_emissioned: self.reward_total_emissioned,
                reward_claimed: self.reward_claimed,
                token_mint: self.token_mint.to_string(),
                token_vault: self.token_vault.to_string(),
                authority: self.authority.to_string(),
                reward_growth_global_x64: self.reward_growth_global_x64.to_string(),
            }
        }
    }

    impl IntoProto<PoolStateProto> for PoolState {
        fn into_proto(self) -> PoolStateProto {
            PoolStateProto {
                bump: self.bump[0].into(),
                amm_config: self.amm_config.to_string(),
                owner: self.owner.to_string(),
                token_mint_0: self.token_mint_0.to_string(),
                token_mint_1: self.token_mint_1.to_string(),
                token_vault_0: self.token_vault_0.to_string(),
                token_vault_1: self.token_vault_1.to_string(),
                observation_key: self.observation_key.to_string(),
                mint_decimals_0: self.mint_decimals_0.into(),
                mint_decimals_1: self.mint_decimals_1.into(),
                tick_spacing: self.tick_spacing.into(),
                liquidity: self.liquidity.to_string(),
                sqrt_price_x64: self.sqrt_price_x64.to_string(),
                tick_current: self.tick_current,
                padding3: self.padding3.into(),
                padding4: self.padding4.into(),
                fee_growth_global_0_x64: self.fee_growth_global_0_x64.to_string(),
                fee_growth_global_1_x64: self.fee_growth_global_1_x64.to_string(),
                protocol_fees_token_0: self.protocol_fees_token_0,
                protocol_fees_token_1: self.protocol_fees_token_1,
                swap_in_amount_token_0: self.swap_in_amount_token_0.to_string(),
                swap_in_amount_token_1: self.swap_in_amount_token_1.to_string(),
                swap_out_amount_token_0: self.swap_out_amount_token_0.to_string(),
                swap_out_amount_token_1: self.swap_out_amount_token_1.to_string(),
                status: self.status.into(),
                padding: self.padding.to_vec(),
                reward_infos: self
                    .reward_infos
                    .into_iter()
                    .map(IntoProto::into_proto)
                    .collect(),

                tick_array_bitmap: self.tick_array_bitmap.to_vec(),
                total_fees_token_0: self.total_fees_token_0,
                total_fees_token_1: self.total_fees_token_1,
                total_fees_claimed_token_0: self.total_fees_claimed_token_0,
                total_fees_claimed_token_1: self.total_fees_claimed_token_1,
                fund_fees_token_0: self.fund_fees_token_0,
                fund_fees_token_1: self.fund_fees_token_1,
                open_time: self.open_time,
                recent_epoch: self.recent_epoch,
                padding1: self.padding1.to_vec(),
                padding2: self.padding2.to_vec(),
            }
        }
    }
    impl IntoProto<ProtocolPositionStateProto> for ProtocolPositionState {
        fn into_proto(self) -> ProtocolPositionStateProto {
            ProtocolPositionStateProto {
                bump: self.bump.into(),
                pool_id: self.pool_id.to_string(),
                tick_lower_index: self.tick_lower_index,
                tick_upper_index: self.tick_upper_index,
                liquidity: self.liquidity.to_string(),
                fee_growth_inside_0_last_x64: self.fee_growth_inside_0_last_x64.to_string(),
                fee_growth_inside_1_last_x64: self.fee_growth_inside_1_last_x64.to_string(),
                token_fees_owed_0: self.token_fees_owed_0,
                token_fees_owed_1: self.token_fees_owed_1,
                reward_growth_inside: self
                    .reward_growth_inside
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
                recent_epoch: self.recent_epoch,
                padding: self.padding.to_vec(),
            }
        }
    }

    impl IntoProto<RaydiumTickStateProto> for TickState {
        fn into_proto(self) -> RaydiumTickStateProto {
            RaydiumTickStateProto {
                tick: self.tick,
                liquidity_gross: self.liquidity_gross.to_string(),
                liquidity_net: self.liquidity_net.to_string(),
                fee_growth_outside_0_x64: self.fee_growth_outside_0_x64.to_string(),
                fee_growth_outside_1_x64: self.fee_growth_outside_1_x64.to_string(),
                reward_growths_outside_x64: self
                    .reward_growths_outside_x64
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
                padding: self.padding.to_vec(),
            }
        }
    }

    impl IntoProto<RaydiumTickArrayStateProto> for TickArrayState {
        fn into_proto(self) -> RaydiumTickArrayStateProto {
            RaydiumTickArrayStateProto {
                pool_id: self.pool_id.to_string(),
                start_tick_index: self.start_tick_index,
                ticks: self.ticks.into_iter().map(IntoProto::into_proto).collect(),
                initialized_tick_count: self.initialized_tick_count.into(),
                recent_epoch: self.recent_epoch,
                padding: self.padding.to_vec(),
            }
        }
    }

    impl IntoProto<TickArrayBitmapExtensionProto> for TickArrayBitmapExtension {
        fn into_proto(self) -> TickArrayBitmapExtensionProto {
            TickArrayBitmapExtensionProto {
                pool_id: self.pool_id.to_string(),
                positive_tick_array_bitmap: self
                    .positive_tick_array_bitmap
                    .into_iter()
                    .map(|d| TickArrayBitmapProto { data: d.to_vec() })
                    .collect(),
                negative_tick_array_bitmap: self
                    .negative_tick_array_bitmap
                    .into_iter()
                    .map(|d| TickArrayBitmapProto { data: d.to_vec() })
                    .collect(),
            }
        }
    }

    impl ParseProto for AccountParser {
        type Message = RaydiumProgramStateProto;

        fn output_into_message(value: Self::Output) -> Self::Message {
            let state_oneof = match value {
                RaydiumProgramState::AmmConfig(data) => Some(
                    raydium_program_state_proto::StateOneof::AmmConfig(data.into_proto()),
                ),
                RaydiumProgramState::OperationState(data) => Some(
                    raydium_program_state_proto::StateOneof::OperationState(data.into_proto()),
                ),
                RaydiumProgramState::ObservationState(data) => Some(
                    raydium_program_state_proto::StateOneof::ObservationState(data.into_proto()),
                ),
                RaydiumProgramState::PersonalPositionState(data) => Some(
                    raydium_program_state_proto::StateOneof::PersonalPositionState(
                        data.into_proto(),
                    ),
                ),
                RaydiumProgramState::PoolState(data) => Some(
                    raydium_program_state_proto::StateOneof::PoolState(data.into_proto()),
                ),
                RaydiumProgramState::ProtocolPositionState(data) => Some(
                    raydium_program_state_proto::StateOneof::ProtocolPositionState(
                        data.into_proto(),
                    ),
                ),
                RaydiumProgramState::TickArrayState(data) => Some(
                    raydium_program_state_proto::StateOneof::TickArrayState(data.into_proto()),
                ),
                RaydiumProgramState::TickArrayBitmapExtension(data) => Some(
                    raydium_program_state_proto::StateOneof::TickArrayBitmapExtension(
                        data.into_proto(),
                    ),
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
    async fn test_amm_config_account_parsing() {
        let parser = AccountParser;

        let account = account_fixture!("A1BBtTYJd4i3xU8D6Tc2FzU6ZN4oXZWXKZnCxwbHXr8x", &parser);

        if let RaydiumProgramState::AmmConfig(amm_config) = account {
            assert_eq!(
                amm_config.owner.to_string(),
                "projjosVCPQH49d5em7VYS7fJZzaqKixqKtus7yk416".to_string()
            );
            assert_eq!(
                amm_config.fund_owner.to_string(),
                "FundHfY8oo8J9KYGyfXFFuQCHe7Z1VBNmsj84eMcdYs4".to_string()
            );
            assert_eq!(amm_config.tick_spacing, 120);
        } else {
            panic!("Invalid parsed data");
        }
    }
}
