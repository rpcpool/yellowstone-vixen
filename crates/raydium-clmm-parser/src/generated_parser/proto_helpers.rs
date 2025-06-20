//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

// #[cfg(feature = "proto")]
pub mod proto_types_parsers {
    use yellowstone_vixen_core::proto_helper_traits;
    proto_helper_traits!();
    use crate::{proto_def, types::Observation};
    impl IntoProto<proto_def::Observation> for Observation {
        fn into_proto(self) -> proto_def::Observation {
            proto_def::Observation {
                block_timestamp: self.block_timestamp,
                tick_cumulative: self.tick_cumulative,
                padding: self.padding.to_vec(),
            }
        }
    }
    use crate::types::PositionRewardInfo;
    impl IntoProto<proto_def::PositionRewardInfo> for PositionRewardInfo {
        fn into_proto(self) -> proto_def::PositionRewardInfo {
            proto_def::PositionRewardInfo {
                growth_inside_last_x64: self.growth_inside_last_x64.to_string(),
                reward_amount_owed: self.reward_amount_owed,
            }
        }
    }
    use crate::types::RewardInfo;
    impl IntoProto<proto_def::RewardInfo> for RewardInfo {
        fn into_proto(self) -> proto_def::RewardInfo {
            proto_def::RewardInfo {
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
    use crate::types::TickState;
    impl IntoProto<proto_def::TickState> for TickState {
        fn into_proto(self) -> proto_def::TickState {
            proto_def::TickState {
                tick: self.tick,
                liquidity_net: self.liquidity_net.to_string(),
                liquidity_gross: self.liquidity_gross.to_string(),
                fee_growth_outside0_x64: self.fee_growth_outside0_x64.to_string(),
                fee_growth_outside1_x64: self.fee_growth_outside1_x64.to_string(),
                reward_growths_outside_x64: self
                    .reward_growths_outside_x64
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
                padding: self.padding.to_vec(),
            }
        }
    }
}
