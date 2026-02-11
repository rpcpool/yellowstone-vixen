use std::borrow::Cow;

use spl_token::{
    solana_program::{program_error::ProgramError, program_option::COption, program_pack::Pack},
    state::{Account as SplAccount, Mint as SplMint, Multisig as SplMultisig},
};
use yellowstone_vixen_core::{
    AccountUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};

use crate::PubkeyBytes;

/// SPL Token account state, proto-compatible
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintProto {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub mint_authority: ::core::option::Option<PubkeyBytes>,
    #[prost(uint64, tag = "2")]
    pub supply: u64,
    #[prost(uint32, tag = "3")]
    pub decimals: u32,
    #[prost(bool, tag = "4")]
    pub is_initialized: bool,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub freeze_authority: ::core::option::Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenAccountProto {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub owner: PubkeyBytes,
    #[prost(uint64, tag = "3")]
    pub amount: u64,

    #[prost(bytes = "vec", optional, tag = "4")]
    pub delegate: ::core::option::Option<PubkeyBytes>,
    #[prost(uint32, tag = "5")]
    pub state: u32,
    #[prost(uint64, tag = "6")]
    pub delegated_amount: u64,

    /// If present, native rent-exempt reserve (lamports).
    #[prost(optional, uint64, tag = "7")]
    pub is_native: ::core::option::Option<u64>,

    #[prost(bytes = "vec", optional, tag = "8")]
    pub close_authority: ::core::option::Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultisigProto {
    #[prost(uint32, tag = "1")]
    pub m: u32,
    #[prost(uint32, tag = "2")]
    pub n: u32,
    #[prost(bool, tag = "3")]
    pub is_initialized: bool,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub signers: ::prost::alloc::vec::Vec<PubkeyBytes>,
}

/// One-of wrapper for SPL Token program account state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenProgramStateProto {
    #[prost(oneof = "token_program_state_proto::State", tags = "1, 2, 3")]
    pub state: ::core::option::Option<token_program_state_proto::State>,
}

pub mod token_program_state_proto {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum State {
        #[prost(message, tag = "1")]
        TokenAccount(super::TokenAccountProto),
        #[prost(message, tag = "2")]
        Mint(super::MintProto),
        #[prost(message, tag = "3")]
        Multisig(super::MultisigProto),
    }
}

impl From<SplMint> for MintProto {
    fn from(m: SplMint) -> Self {
        Self {
            mint_authority: match m.mint_authority {
                COption::Some(pk) => Some(pk.to_bytes().to_vec()),
                COption::None => None,
            },
            supply: m.supply,
            decimals: m.decimals as u32,
            is_initialized: m.is_initialized,
            freeze_authority: match m.freeze_authority {
                COption::Some(pk) => Some(pk.to_bytes().to_vec()),
                COption::None => None,
            },
        }
    }
}

impl From<SplAccount> for TokenAccountProto {
    fn from(a: SplAccount) -> Self {
        Self {
            mint: a.mint.to_bytes().to_vec(),
            owner: a.owner.to_bytes().to_vec(),
            amount: a.amount,

            delegate: match a.delegate {
                COption::Some(pk) => Some(pk.to_bytes().to_vec()),
                COption::None => None,
            },
            state: a.state as u32,
            delegated_amount: a.delegated_amount,

            is_native: match a.is_native {
                COption::Some(x) => Some(x),
                COption::None => None,
            },

            close_authority: match a.close_authority {
                COption::Some(pk) => Some(pk.to_bytes().to_vec()),
                COption::None => None,
            },
        }
    }
}

impl From<SplMultisig> for MultisigProto {
    fn from(m: SplMultisig) -> Self {
        Self {
            m: m.m as u32,
            n: m.n as u32,
            is_initialized: m.is_initialized,
            signers: m.signers.iter().map(|pk| pk.to_bytes().to_vec()).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AccountParser;

impl Parser for AccountParser {
    type Input = AccountUpdate;
    type Output = TokenProgramStateProto;

    fn id(&self) -> Cow<'static, str> { "token_program::AccountParser".into() }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([spl_token::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acct.account.as_ref().ok_or(ProgramError::InvalidArgument)?;
        let data = &inner.data;

        let state = match data.len() {
            SplMint::LEN => {
                let m = SplMint::unpack_from_slice(data).map_err(ParseError::from)?;
                token_program_state_proto::State::Mint(MintProto::from(m))
            },
            SplAccount::LEN => {
                let a = SplAccount::unpack_from_slice(data).map_err(ParseError::from)?;
                token_program_state_proto::State::TokenAccount(TokenAccountProto::from(a))
            },
            SplMultisig::LEN => {
                let ms = SplMultisig::unpack_from_slice(data).map_err(ParseError::from)?;
                token_program_state_proto::State::Multisig(MultisigProto::from(ms))
            },
            _ => return Err(ParseError::Filtered),
        };

        Ok(TokenProgramStateProto { state: Some(state) })
    }
}

impl ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { spl_token::ID.to_bytes().into() }
}

#[cfg(test)]
mod tests {
    use yellowstone_vixen_mock::{account_fixture, run_account_parse, FixtureData};

    use super::{token_program_state_proto, AccountParser, Parser, TokenProgramStateProto};

    #[tokio::test]
    async fn test_mint_account_parsing() {
        let parser = AccountParser;

        let account = account_fixture!("3SmPYPvZfEmroktLiJsgaNENuPEud3Z52zSfLQ1zJdkK", &parser);

        let TokenProgramStateProto {
            state: Some(token_program_state_proto::State::Mint(mint)),
        } = account
        else {
            panic!("Invalid Account");
        };

        assert_eq!(mint.decimals, 10);
    }
}
