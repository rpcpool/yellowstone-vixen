use std::borrow::Cow;

use spl_token::{
    solana_program::{program_error::ProgramError, program_option::COption, program_pack::Pack},
    state::{Account as SplAccount, Mint as SplMint, Multisig as SplMultisig},
};
use yellowstone_vixen_core::{
    AccountUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};
use yellowstone_vixen_proc_macro::vixen_proto;

use crate::PubkeyBytes;

/// SPL Token account state, proto-compatible
#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct Mint {
    pub mint_authority: ::core::option::Option<PubkeyBytes>,
    pub supply: u64,
    pub decimals: u32,
    pub is_initialized: bool,
    pub freeze_authority: ::core::option::Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TokenAccount {
    pub mint: PubkeyBytes,
    pub owner: PubkeyBytes,
    pub amount: u64,

    pub delegate: ::core::option::Option<PubkeyBytes>,
    pub state: u32,
    pub delegated_amount: u64,

    /// If present, native rent-exempt reserve (lamports).
    pub is_native: ::core::option::Option<u64>,

    pub close_authority: ::core::option::Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct Multisig {
    pub m: u32,
    pub n: u32,
    pub is_initialized: bool,
    pub signers: Vec<PubkeyBytes>,
}

/// One-of wrapper for SPL Token program account state.
#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TokenProgramState {
    #[vixen_proto_hint(oneof = "token_program_state::State", tags = "1, 2, 3")]
    pub state: ::core::option::Option<token_program_state::State>,
}

pub mod token_program_state {
    use super::vixen_proto;

    #[vixen_proto(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum State {
        TokenAccount(super::TokenAccount),
        Mint(super::Mint),
        Multisig(super::Multisig),
    }
}

impl From<SplMint> for Mint {
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

impl From<SplAccount> for TokenAccount {
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

impl From<SplMultisig> for Multisig {
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
    type Output = TokenProgramState;

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
                token_program_state::State::Mint(Mint::from(m))
            },
            SplAccount::LEN => {
                let a = SplAccount::unpack_from_slice(data).map_err(ParseError::from)?;
                token_program_state::State::TokenAccount(TokenAccount::from(a))
            },
            SplMultisig::LEN => {
                let ms = SplMultisig::unpack_from_slice(data).map_err(ParseError::from)?;
                token_program_state::State::Multisig(Multisig::from(ms))
            },
            _ => return Err(ParseError::Filtered),
        };

        Ok(TokenProgramState { state: Some(state) })
    }
}

impl ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { spl_token::ID.to_bytes().into() }
}

#[cfg(test)]
mod tests {
    use yellowstone_vixen_mock::{account_fixture, run_account_parse, FixtureData};

    use super::{token_program_state, AccountParser, Parser, TokenProgramState};

    #[tokio::test]
    async fn test_mint_account_parsing() {
        let parser = AccountParser;

        let account = account_fixture!("3SmPYPvZfEmroktLiJsgaNENuPEud3Z52zSfLQ1zJdkK", &parser);

        let TokenProgramState {
            state: Some(token_program_state::State::Mint(mint)),
        } = account
        else {
            panic!("Invalid Account");
        };

        assert_eq!(mint.decimals, 10);
    }
}
