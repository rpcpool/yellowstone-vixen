use spl_stake_pool::{
    solana_program::{borsh1::try_from_slice_unchecked, program_error},
    state::{StakePool, ValidatorList},
};
use yellowstone_vixen_core::{
    AccountUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};
use yellowstone_vixen_proc_macro::vixen_proto;

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SplStakePoolProgramStateProto {
    #[vixen_proto_hint(oneof = "spl_stake_pool_program_state_proto::State", tags = "1, 2")]
    pub state: Option<spl_stake_pool_program_state_proto::State>,
}

pub mod spl_stake_pool_program_state_proto {
    use super::vixen_proto;

    #[vixen_proto(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum State {
        StakePool(super::StakePoolAccountProto),
        ValidatorList(super::ValidatorListAccountProto),
    }
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct StakePoolAccountProto {
    /// First byte discriminator (1)
    pub account_type: u32,

    /// Raw borsh bytes (including discriminator byte)
    pub data: Vec<u8>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ValidatorListAccountProto {
    /// First byte discriminator (2)
    pub account_type: u32,

    /// Raw borsh bytes (including discriminator byte)
    pub data: Vec<u8>,
}

#[allow(clippy::large_enum_variant)]
pub enum SplStakePoolProgramState {
    StakePool(StakePool),
    ValidatorList(ValidatorList),
}

impl SplStakePoolProgramState {
    pub fn try_unpack(data_bytes: &[u8]) -> ParseResult<Self> {
        let first_byte = *data_bytes
            .first()
            .ok_or_else(|| ParseError::from("Empty account data".to_owned()))?;

        match first_byte {
            0 => Err(ParseError::from("Uninitialized Account".to_owned())),
            1 => {
                let stake_pool = try_from_slice_unchecked::<StakePool>(data_bytes)?;
                Ok(SplStakePoolProgramState::StakePool(stake_pool))
            },
            2 => {
                let validator_list = try_from_slice_unchecked::<ValidatorList>(data_bytes)?;
                Ok(SplStakePoolProgramState::ValidatorList(validator_list))
            },
            _ => Err(ParseError::from("Invalid Account".to_owned())),
        }
    }

    pub fn try_unpack_proto(data_bytes: &[u8]) -> ParseResult<SplStakePoolProgramStateProto> {
        let first_byte = *data_bytes
            .first()
            .ok_or_else(|| ParseError::from("Empty account data".to_owned()))?;

        match first_byte {
            0 => Err(ParseError::from("Uninitialized Account".to_owned())),
            1 => Ok(SplStakePoolProgramStateProto {
                state: Some(spl_stake_pool_program_state_proto::State::StakePool(
                    StakePoolAccountProto {
                        account_type: 1,
                        data: data_bytes.to_vec(),
                    },
                )),
            }),
            2 => Ok(SplStakePoolProgramStateProto {
                state: Some(spl_stake_pool_program_state_proto::State::ValidatorList(
                    ValidatorListAccountProto {
                        account_type: 2,
                        data: data_bytes.to_vec(),
                    },
                )),
            }),
            _ => Err(ParseError::from("Invalid Account".to_owned())),
        }
    }
}

#[derive(Copy, Clone)]
pub struct AccountParser;

impl Parser for AccountParser {
    type Input = AccountUpdate;
    type Output = SplStakePoolProgramStateProto;

    fn id(&self) -> std::borrow::Cow<'static, str> { "spl_stake_pool::AccountParser".into() }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([spl_stake_pool::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acct
            .account
            .as_ref()
            .ok_or(program_error::ProgramError::InvalidArgument)?;
        SplStakePoolProgramState::try_unpack_proto(&inner.data)
    }
}

impl ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { spl_stake_pool::ID.to_bytes().into() }
}
