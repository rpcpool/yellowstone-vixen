use borsh::BorshDeserialize;
use spl_pod::solana_program::{self};
use spl_stake_pool::state::{AccountType, StakePool, ValidatorList, ValidatorStakeInfo};

/// SplStakePool Program State
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum SplStakePoolProgramState {
    StakePool(StakePool),
    ValidatorStakeInfo(ValidatorStakeInfo),
    ValidatorList(ValidatorList),
}

impl SplStakePoolProgramState {
    pub fn try_unpack(data_bytes: &[u8]) -> yellowstone_vixen_core::ParseResult<Self> {
        let first_slice = &data_bytes[0..3];
        let account_type = AccountType::try_from_slice(first_slice)?;

        match account_type {
            AccountType::StakePool => {
                let stake_pool = StakePool::try_from_slice(data_bytes)?;
                return Ok(SplStakePoolProgramState::StakePool(stake_pool));
            },
            AccountType::ValidatorList => {
                let validator_list = ValidatorList::try_from_slice(data_bytes)?;
                return Ok(SplStakePoolProgramState::ValidatorList(validator_list));
            },
            _ => Err(yellowstone_vixen_core::ParseError::from(
                "Invalid Account".to_owned(),
            )),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AccountParser;

impl yellowstone_vixen_core::Parser for AccountParser {
    type Input = yellowstone_vixen_core::AccountUpdate;
    type Output = SplStakePoolProgramState;

    fn id(&self) -> std::borrow::Cow<str> {
        "spl_stake_pool::AccountParser".into()
    }

    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
        yellowstone_vixen_core::Prefilter::builder()
            .account_owners([spl_stake_pool::ID])
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
        SplStakePoolProgramState::try_unpack(&inner.data)
    }
}

impl yellowstone_vixen_core::ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        spl_stake_pool::ID.to_bytes().into()
    }
}
