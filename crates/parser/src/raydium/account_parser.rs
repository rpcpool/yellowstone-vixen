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

    fn id(&self) -> std::borrow::Cow<str> {
        "yellowstone_vixen_parser::jup_programs::raydium::AccountParser".into()
    }

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
