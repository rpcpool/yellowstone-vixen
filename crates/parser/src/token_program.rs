use spl_token::{
    solana_program::{program_error::ProgramError, program_pack::Pack},
    state::{Account, Mint, Multisig},
};
use yellowstone_vixen_core::{AccountUpdate, ParseError, ParseResult, Parser, Prefilter};

#[derive(Debug)]
pub enum TokenProgramState {
    TokenAccount(Account),
    Mint(Mint),
    Multisig(Multisig),
}

impl TokenProgramState {
    fn try_unpack(data_bytes: &[u8]) -> ParseResult<Self> {
        match data_bytes.len() {
            Mint::LEN => Mint::unpack(data_bytes)
                .map(|mint| Self::Mint(mint))
                .map_err(Into::into),
            Account::LEN => Account::unpack(data_bytes)
                .map(|token_account| Self::TokenAccount(token_account))
                .map_err(Into::into),
            Multisig::LEN => Multisig::unpack(data_bytes)
                .map(|multisig| Self::Multisig(multisig))
                .map_err(Into::into),
            _ => Err(ParseError::from("Invalid Account data length".to_owned()).into()),
        }
    }
}

pub struct TokenProgramParser;

impl Parser for TokenProgramParser {
    type Input = AccountUpdate;
    type Output = TokenProgramState;

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([spl_token::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acct.account.as_ref().ok_or(ProgramError::InvalidArgument)?;

        TokenProgramState::try_unpack(&inner.data)
    }
}
