use borsh::BorshDeserialize;
use spl_pod::solana_program;
use spl_stake_pool::state::{StakePool, ValidatorList, ValidatorStakeInfo};

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
        let data_len = data_bytes.len();
        const VALIDATOR_STAKE_INFO_LEN: usize = std::mem::size_of::<ValidatorStakeInfo>();
        const STAKEPOOL_LEN: usize = std::mem::size_of::<StakePool>();
        const VALIDATORLIST_LEN: usize = std::mem::size_of::<ValidatorList>();
        match data_len {
            STAKEPOOL_LEN => Ok(SplStakePoolProgramState::StakePool(
                StakePool::try_from_slice(data_bytes)?,
            )),
            VALIDATOR_STAKE_INFO_LEN => Ok(SplStakePoolProgramState::ValidatorStakeInfo(
                ValidatorStakeInfo::try_from_slice(data_bytes)?,
            )),
            VALIDATORLIST_LEN => Ok(SplStakePoolProgramState::ValidatorList(
                ValidatorList::try_from_slice(data_bytes)?,
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
