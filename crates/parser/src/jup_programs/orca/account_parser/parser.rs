use std::borrow::Cow;

use spl_pod::solana_program::program_error::ProgramError;
// use whirlpools::state::{FeeTier, Position, TickArray, Whirlpool, WhirlpoolsConfig};
use orca_whirlpools_client::accounts::{FeeTier, Position, TickArray, Whirlpool, WhirlpoolsConfig};
use yellowstone_vixen_core::{
    AccountUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};

#[derive(Debug, Clone)]
pub enum OrcaProgramState {
    Whirlpool(Whirlpool),
    WhirlpoolsConfig(WhirlpoolsConfig),
    FeeTier(FeeTier),
    Position(Position),
    TickArray(TickArray),
}

impl OrcaProgramState {
    pub fn try_unpack(data_bytes: &[u8]) -> Result<Self, ParseError> {
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
pub struct OrcaProgramAccParser;

impl Parser for OrcaProgramAccParser {
    type Input = AccountUpdate;
    type Output = OrcaProgramState;

    fn id(&self) -> Cow<str> {
        "yellowstone_vixen_parser::token_program::OrcaProgramAccParser".into()
    }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([spl_token::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acct.account.as_ref().ok_or(ProgramError::InvalidArgument)?;
        OrcaProgramState::try_unpack(&inner.data)
    }
}

impl ProgramParser for OrcaProgramAccParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        spl_token::ID.to_bytes().into()
    }
}
