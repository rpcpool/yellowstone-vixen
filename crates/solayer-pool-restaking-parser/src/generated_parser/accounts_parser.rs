//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::{accounts::RestakingPool, ID};

/// RestakingProgram Program State
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum RestakingProgramProgramState {
    RestakingPool(RestakingPool),
}

impl RestakingProgramProgramState {
    pub fn try_unpack(data_bytes: &[u8]) -> yellowstone_vixen_core::ParseResult<Self> {
        let data_len = data_bytes.len();
        match data_len {
            RestakingPool::LEN => Ok(RestakingProgramProgramState::RestakingPool(
                RestakingPool::from_bytes(data_bytes)?,
            )),
            _ => Err(yellowstone_vixen_core::ParseError::from(
                "Invalid Account data length".to_owned(),
            )),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AccountParser;

impl yellowstone_vixen_core::Parser for AccountParser {
    type Input = yellowstone_vixen_core::AccountUpdate;
    type Output = RestakingProgramProgramState;

    fn id(&self) -> std::borrow::Cow<str> { "restaking_program::AccountParser".into() }

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
        RestakingProgramProgramState::try_unpack(&inner.data)
    }
}

impl yellowstone_vixen_core::ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { ID.to_bytes().into() }
}
