//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::{
    accounts::{
        FeeTier, LockConfig, Position, PositionBundle, TickArray, TokenBadge, Whirlpool,
        WhirlpoolsConfig, WhirlpoolsConfigExtension,
    },
    ID,
};

/// Whirlpool Program State
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum WhirlpoolProgramState {
    WhirlpoolsConfig(WhirlpoolsConfig),
    WhirlpoolsConfigExtension(WhirlpoolsConfigExtension),
    FeeTier(FeeTier),
    LockConfig(LockConfig),
    Position(Position),
    PositionBundle(PositionBundle),
    TickArray(TickArray),
    TokenBadge(TokenBadge),
    Whirlpool(Whirlpool),
}

impl WhirlpoolProgramState {
    #[allow(unreachable_patterns)]
    pub fn try_unpack(data_bytes: &[u8]) -> yellowstone_vixen_core::ParseResult<Self> {
        let data_len = data_bytes.len();
        match data_len {
            WhirlpoolsConfig::LEN => Ok(WhirlpoolProgramState::WhirlpoolsConfig(
                WhirlpoolsConfig::from_bytes(data_bytes)?,
            )),
            WhirlpoolsConfigExtension::LEN => Ok(WhirlpoolProgramState::WhirlpoolsConfigExtension(
                WhirlpoolsConfigExtension::from_bytes(data_bytes)?,
            )),
            FeeTier::LEN => Ok(WhirlpoolProgramState::FeeTier(FeeTier::from_bytes(
                data_bytes,
            )?)),
            LockConfig::LEN => Ok(WhirlpoolProgramState::LockConfig(LockConfig::from_bytes(
                data_bytes,
            )?)),
            Position::LEN => Ok(WhirlpoolProgramState::Position(Position::from_bytes(
                data_bytes,
            )?)),
            PositionBundle::LEN => Ok(WhirlpoolProgramState::PositionBundle(
                PositionBundle::from_bytes(data_bytes)?,
            )),
            TickArray::LEN => Ok(WhirlpoolProgramState::TickArray(TickArray::from_bytes(
                data_bytes,
            )?)),
            TokenBadge::LEN => Ok(WhirlpoolProgramState::TokenBadge(TokenBadge::from_bytes(
                data_bytes,
            )?)),
            Whirlpool::LEN => Ok(WhirlpoolProgramState::Whirlpool(Whirlpool::from_bytes(
                data_bytes,
            )?)),
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
    type Output = WhirlpoolProgramState;

    fn id(&self) -> std::borrow::Cow<str> { "whirlpool::AccountParser".into() }

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
        WhirlpoolProgramState::try_unpack(&inner.data)
    }
}

impl yellowstone_vixen_core::ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { ID.to_bytes().into() }
}
