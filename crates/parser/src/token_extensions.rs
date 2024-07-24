use spl_token_2022::{
    solana_program::{program_error::ProgramError, program_pack::Pack},
    state::{Account, Mint, Multisig},
};
use yellowstone_vixen_core::{AccountUpdate, Parser, ParseResult, Prefilter};

#[derive(Debug)]
pub enum TokenExtensionState {
    Account(Account),
    Mint(Mint),
    Multisig(Multisig),
}

impl TokenExtensionState {
    fn try_unpack(input: &[u8]) -> ParseResult<Self> {
        match input.len() {
            Mint::LEN => Mint::unpack(input)
                .map(|mint| Self::Mint(mint))
                .map_err(Into::into),
            Multisig::LEN => Multisig::unpack(input)
                .map(|multisig| Self::Multisig(multisig))
                .map_err(Into::into),
            _ => {
                let data = input
                    .get(..Account::LEN)
                    .ok_or(ProgramError::InvalidAccountData)?;

                Account::unpack(data)
                    .map(|account| Self::Account(account))
                    .map_err(Into::into)
            }
        }
    }
}

pub struct TokenExtensionParser;

impl Parser for TokenExtensionParser {
    type Input = AccountUpdate;
    type Output = TokenExtensionState;

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([spl_token_2022::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acct.account.as_ref().ok_or(ProgramError::InvalidArgument)?;

        TokenExtensionState::try_unpack(&inner.data)
    }
}