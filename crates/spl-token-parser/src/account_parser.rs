use std::borrow::Cow;

use spl_token::{
    solana_program::{program_error::ProgramError, program_option::COption, program_pack::Pack},
    state::{Account as SplAccount, Mint as SplMint, Multisig as SplMultisig},
};
use yellowstone_vixen_core::{
    AccountUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};
use yellowstone_vixen_proc_macro::vixen;

use crate::PublicKey;

/// SPL Token account state, proto-compatible
#[vixen]
#[derive(Clone, PartialEq)]
pub struct Mint {
    pub mint_authority: Option<PublicKey>,
    pub supply: u64,
    pub decimals: u32,
    pub is_initialized: bool,
    pub freeze_authority: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct TokenAccount {
    pub mint: PublicKey,
    pub owner: PublicKey,
    pub amount: u64,

    pub delegate: Option<PublicKey>,
    pub state: u32,
    pub delegated_amount: u64,

    /// If present, native rent-exempt reserve (lamports).
    pub is_native: ::core::option::Option<u64>,

    pub close_authority: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct Multisig {
    pub m: u32,
    pub n: u32,
    pub is_initialized: bool,
    pub signers: Vec<PublicKey>,
}

/// One-of wrapper for SPL Token program account state.
#[vixen]
#[derive(Clone, PartialEq)]
pub struct TokenProgramState {
    #[hint(oneof = "account::Account", tags = "1, 2, 3")]
    pub account: ::core::option::Option<account::Account>,
}

pub mod account {
    use super::vixen;

    #[vixen(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Account {
        TokenAccount(super::TokenAccount),
        Mint(super::Mint),
        Multisig(super::Multisig),
    }
}

impl From<SplMint> for Mint {
    fn from(m: SplMint) -> Self {
        Self {
            mint_authority: match m.mint_authority {
                COption::Some(pk) => Some(PublicKey::new(pk.to_bytes())),
                COption::None => None,
            },
            supply: m.supply,
            decimals: m.decimals as u32,
            is_initialized: m.is_initialized,
            freeze_authority: match m.freeze_authority {
                COption::Some(pk) => Some(PublicKey::new(pk.to_bytes())),
                COption::None => None,
            },
        }
    }
}

impl From<SplAccount> for TokenAccount {
    fn from(a: SplAccount) -> Self {
        Self {
            mint: PublicKey::new(a.mint.to_bytes()),
            owner: PublicKey::new(a.owner.to_bytes()),
            amount: a.amount,

            delegate: match a.delegate {
                COption::Some(pk) => Some(PublicKey::new(pk.to_bytes())),
                COption::None => None,
            },
            state: a.state as u32,
            delegated_amount: a.delegated_amount,

            is_native: match a.is_native {
                COption::Some(x) => Some(x),
                COption::None => None,
            },

            close_authority: match a.close_authority {
                COption::Some(pk) => Some(PublicKey::new(pk.to_bytes())),
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
            signers: m
                .signers
                .iter()
                .map(|pk| PublicKey::new(pk.to_bytes()))
                .collect(),
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
                account::Account::Mint(Mint::from(m))
            },
            SplAccount::LEN => {
                let a = SplAccount::unpack_from_slice(data).map_err(ParseError::from)?;
                account::Account::TokenAccount(TokenAccount::from(a))
            },
            SplMultisig::LEN => {
                let ms = SplMultisig::unpack_from_slice(data).map_err(ParseError::from)?;
                account::Account::Multisig(Multisig::from(ms))
            },
            _ => return Err(ParseError::Filtered),
        };

        Ok(TokenProgramState {
            account: Some(state),
        })
    }
}

impl ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { spl_token::ID.to_bytes().into() }
}

#[cfg(test)]
mod tests {
    use yellowstone_vixen_mock::account_fixture;

    use super::{account, AccountParser, Parser, TokenProgramState};

    #[tokio::test]
    async fn test_mint_account_parsing() {
        let parser = AccountParser;

        let account = account_fixture!("3SmPYPvZfEmroktLiJsgaNENuPEud3Z52zSfLQ1zJdkK", &parser);

        let TokenProgramState {
            account: Some(account::Account::Mint(mint)),
        } = account
        else {
            panic!("Invalid Account");
        };

        assert_eq!(mint.decimals, 10);
    }
}
