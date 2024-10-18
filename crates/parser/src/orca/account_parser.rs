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

impl ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        orca_whirlpools_client::ID.to_bytes().into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AccountParser;

impl Parser for AccountParser {
    type Input = AccountUpdate;
    type Output = OrcaProgramState;

    fn id(&self) -> Cow<str> { "yellowstone_vixen_parser::token_program::AccountParser".into() }

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
