use spl_pod::solana_program::{self, borsh1::try_from_slice_unchecked};
use spl_stake_pool::state::{StakePool, ValidatorList};

/// SplStakePool Program State
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum SplStakePoolProgramState {
    StakePool(StakePool),
    ValidatorList(ValidatorList),
}

impl SplStakePoolProgramState {
    pub fn try_unpack(data_bytes: &[u8]) -> yellowstone_vixen_core::ParseResult<Self> {
        let first_byte = data_bytes[0];

        match first_byte {
            0 => Err(yellowstone_vixen_core::ParseError::from(
                "Uninitialized Account".to_owned(),
            )),
            1 => {
                let stake_pool = try_from_slice_unchecked::<StakePool>(data_bytes)?;
                Ok(SplStakePoolProgramState::StakePool(stake_pool))
            },
            2 => {
                let validator_list = try_from_slice_unchecked::<ValidatorList>(data_bytes)?;
                Ok(SplStakePoolProgramState::ValidatorList(validator_list))
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

    fn id(&self) -> std::borrow::Cow<'static, str> {
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
