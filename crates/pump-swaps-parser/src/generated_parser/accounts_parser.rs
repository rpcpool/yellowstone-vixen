//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::{
    accounts::{GlobalConfig, Pool},
    ID,
};

/// PumpSwap Program State
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
#[cfg_attr(feature = "tracing", derive(strum_macros::Display))]
pub enum PumpSwapProgramState {
    GlobalConfig(GlobalConfig),
    Pool(Pool),
}

impl PumpSwapProgramState {
    pub fn try_unpack(data_bytes: &[u8]) -> yellowstone_vixen_core::ParseResult<Self> {
        let acc_discriminator: [u8; 8] = data_bytes[0..8].try_into()?;
        let acc = match acc_discriminator {
            [149, 8, 156, 202, 160, 252, 176, 217] => Ok(PumpSwapProgramState::GlobalConfig(
                GlobalConfig::from_bytes(data_bytes)?,
            )),
            [241, 154, 109, 4, 17, 177, 109, 188] => {
                Ok(PumpSwapProgramState::Pool(Pool::from_bytes(data_bytes)?))
            },
            _ => Err(yellowstone_vixen_core::ParseError::from(
                "Invalid Account discriminator".to_owned(),
            )),
        };

        #[cfg(feature = "tracing")]
        match &acc {
            Ok(acc) => {
                tracing::info!(
                    name: "correctly_parsed_account",
                    name = "account_update",
                    program = ID.to_string(),
                    account = acc.to_string()
                );
            },
            Err(e) => {
                tracing::info!(
                    name: "incorrectly_parsed_account",
                    name = "account_update",
                    program = ID.to_string(),
                    account = "error",
                    discriminator = ?acc_discriminator,
                    error = ?e
                );
            },
        }

        acc
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AccountParser;

impl yellowstone_vixen_core::Parser for AccountParser {
    type Input = yellowstone_vixen_core::AccountUpdate;
    type Output = PumpSwapProgramState;

    fn id(&self) -> std::borrow::Cow<str> { "pump_swap::AccountParser".into() }

    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
        yellowstone_vixen_core::Prefilter::builder()
            .account_owners([ID])
            .build()
            .unwrap()
    }

    async fn parse(
        &self,
        acct: &yellowstone_vixen_core::AccountUpdate,
    ) -> yellowstone_vixen_core::ParseResult<Self::Output> {
        let inner = acct
            .account
            .as_ref()
            .ok_or(solana_program::program_error::ProgramError::InvalidArgument)?;
        PumpSwapProgramState::try_unpack(&inner.data)
    }
}

impl yellowstone_vixen_core::ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { ID.to_bytes().into() }
}

// #[cfg(feature = "proto")]
mod proto_parser {
    use yellowstone_vixen_core::proto::ParseProto;

    use super::{AccountParser, GlobalConfig, PumpSwapProgramState};
    use crate::{proto_def, proto_helpers::proto_types_parsers::IntoProto};
    impl IntoProto<proto_def::GlobalConfig> for GlobalConfig {
        fn into_proto(self) -> proto_def::GlobalConfig {
            proto_def::GlobalConfig {
                admin: self.admin.to_string(),
                lp_fee_basis_points: self.lp_fee_basis_points,
                protocol_fee_basis_points: self.protocol_fee_basis_points,
                disable_flags: self.disable_flags.into(),
                protocol_fee_recipients: self
                    .protocol_fee_recipients
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
            }
        }
    }
    use super::Pool;
    impl IntoProto<proto_def::Pool> for Pool {
        fn into_proto(self) -> proto_def::Pool {
            proto_def::Pool {
                pool_bump: self.pool_bump.into(),
                index: self.index.into(),
                creator: self.creator.to_string(),
                base_mint: self.base_mint.to_string(),
                quote_mint: self.quote_mint.to_string(),
                lp_mint: self.lp_mint.to_string(),
                pool_base_token_account: self.pool_base_token_account.to_string(),
                pool_quote_token_account: self.pool_quote_token_account.to_string(),
                lp_supply: self.lp_supply,
            }
        }
    }

    impl IntoProto<proto_def::ProgramState> for PumpSwapProgramState {
        fn into_proto(self) -> proto_def::ProgramState {
            let state_oneof = match self {
                PumpSwapProgramState::GlobalConfig(data) => {
                    proto_def::program_state::StateOneof::GlobalConfig(data.into_proto())
                },
                PumpSwapProgramState::Pool(data) => {
                    proto_def::program_state::StateOneof::Pool(data.into_proto())
                },
            };

            proto_def::ProgramState {
                state_oneof: Some(state_oneof),
            }
        }
    }

    impl ParseProto for AccountParser {
        type Message = proto_def::ProgramState;

        fn output_into_message(value: Self::Output) -> Self::Message { value.into_proto() }
    }
}
