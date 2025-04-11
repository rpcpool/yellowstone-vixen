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
    use sdk::types::LastWithdraw;

    use crate as sdk;
    use crate::proto_def;
    impl IntoProto<proto_def::LastWithdraw> for LastWithdraw {
        fn into_proto(self) -> proto_def::LastWithdraw {
            proto_def::LastWithdraw {
                last_withdraw_timestamp: self.last_withdraw_timestamp,
            }
        }
    }
    use sdk::types::CreateEvent;
    impl IntoProto<proto_def::CreateEvent> for CreateEvent {
        fn into_proto(self) -> proto_def::CreateEvent {
            proto_def::CreateEvent {
                name: self.name,
                symbol: self.symbol,
                uri: self.uri,
                mint: self.mint.to_string(),
                bonding_curve: self.bonding_curve.to_string(),
                user: self.user.to_string(),
            }
        }
    }
    use sdk::types::TradeEvent;
    impl IntoProto<proto_def::TradeEvent> for TradeEvent {
        fn into_proto(self) -> proto_def::TradeEvent {
            proto_def::TradeEvent {
                mint: self.mint.to_string(),
                sol_amount: self.sol_amount,
                token_amount: self.token_amount,
                is_buy: self.is_buy,
                user: self.user.to_string(),
                timestamp: self.timestamp,
                virtual_sol_reserves: self.virtual_sol_reserves,
                virtual_token_reserves: self.virtual_token_reserves,
                real_sol_reserves: self.real_sol_reserves,
                real_token_reserves: self.real_token_reserves,
            }
        }
    }
    use sdk::types::CompleteEvent;
    impl IntoProto<proto_def::CompleteEvent> for CompleteEvent {
        fn into_proto(self) -> proto_def::CompleteEvent {
            proto_def::CompleteEvent {
                user: self.user.to_string(),
                mint: self.mint.to_string(),
                bonding_curve: self.bonding_curve.to_string(),
                timestamp: self.timestamp,
            }
        }
    }
    use sdk::types::SetParamsEvent;
    impl IntoProto<proto_def::SetParamsEvent> for SetParamsEvent {
        fn into_proto(self) -> proto_def::SetParamsEvent {
            proto_def::SetParamsEvent {
                fee_recipient: self.fee_recipient.to_string(),
                initial_virtual_token_reserves: self.initial_virtual_token_reserves,
                initial_virtual_sol_reserves: self.initial_virtual_sol_reserves,
                initial_real_token_reserves: self.initial_real_token_reserves,
                token_total_supply: self.token_total_supply,
                fee_basis_points: self.fee_basis_points,
            }
        }
    }
}
