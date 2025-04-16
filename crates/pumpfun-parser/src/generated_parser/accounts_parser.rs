//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::{
    accounts::{BondingCurve, Global},
    ID,
};

/// Pump Program State
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum PumpProgramState {
    BondingCurve(BondingCurve),
    Global(Global),
}

impl PumpProgramState {
    pub fn try_unpack(data_bytes: &[u8]) -> yellowstone_vixen_core::ParseResult<Self> {
        let acc_discriminator: [u8; 8] = data_bytes[0..8].try_into()?;
        match acc_discriminator {
            [23, 183, 248, 55, 96, 216, 172, 96] => Ok(PumpProgramState::BondingCurve(
                BondingCurve::from_bytes(data_bytes)?,
            )),
            [167, 232, 232, 177, 200, 108, 114, 127] => {
                Ok(PumpProgramState::Global(Global::from_bytes(data_bytes)?))
            },
            _ => Err(yellowstone_vixen_core::ParseError::from(
                "Invalid Account discriminator".to_owned(),
            )),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AccountParser;

impl yellowstone_vixen_core::Parser for AccountParser {
    type Input = yellowstone_vixen_core::AccountUpdate;
    type Output = PumpProgramState;

    fn id(&self) -> std::borrow::Cow<str> { "pump::AccountParser".into() }

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
        PumpProgramState::try_unpack(&inner.data)
    }
}

impl yellowstone_vixen_core::ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { ID.to_bytes().into() }
}

// #[cfg(feature = "proto")]
mod proto_parser {
    use yellowstone_vixen_core::proto::ParseProto;

    use super::{AccountParser, BondingCurve, PumpProgramState};
    use crate::{proto_def, proto_helpers::proto_types_parsers::IntoProto};
    impl IntoProto<proto_def::BondingCurve> for BondingCurve {
        fn into_proto(self) -> proto_def::BondingCurve {
            proto_def::BondingCurve {
                virtual_token_reserves: self.virtual_token_reserves,
                virtual_sol_reserves: self.virtual_sol_reserves,
                real_token_reserves: self.real_token_reserves,
                real_sol_reserves: self.real_sol_reserves,
                token_total_supply: self.token_total_supply,
                complete: self.complete,
            }
        }
    }
    use super::Global;
    impl IntoProto<proto_def::Global> for Global {
        fn into_proto(self) -> proto_def::Global {
            proto_def::Global {
                initialized: self.initialized,
                authority: self.authority.to_string(),
                fee_recipient: self.fee_recipient.to_string(),
                initial_virtual_token_reserves: self.initial_virtual_token_reserves,
                initial_virtual_sol_reserves: self.initial_virtual_sol_reserves,
                initial_real_token_reserves: self.initial_real_token_reserves,
                token_total_supply: self.token_total_supply,
                fee_basis_points: self.fee_basis_points,
            }
        }
    }

    impl IntoProto<proto_def::ProgramState> for PumpProgramState {
        fn into_proto(self) -> proto_def::ProgramState {
            let state_oneof = match self {
                PumpProgramState::BondingCurve(data) => {
                    proto_def::program_state::StateOneof::BondingCurve(data.into_proto())
                },
                PumpProgramState::Global(data) => {
                    proto_def::program_state::StateOneof::Global(data.into_proto())
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
