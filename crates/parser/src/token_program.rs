use spl_token::{
    solana_program::{program_error::ProgramError, program_pack::Pack},
    state::{Account as TokenAccount, Mint, Multisig},
};
use yellowstone_vixen_core::{AccountUpdate, ParseResult, Parser, Prefilter};

#[derive(Debug)]
pub enum TokenProgramState {
    Account(TokenAccount),
    Mint(Mint),
    Multisig(Multisig),
}

impl TokenProgramState {
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
                    .get(..TokenAccount::LEN)
                    .ok_or(ProgramError::InvalidAccountData)?;

                TokenAccount::unpack(data)
                    .map(|account| Self::Account(account))
                    .map_err(Into::into)
            }
        }
    }
}

pub struct TokenProgramParser;

impl Parser for TokenProgramParser {
    type Input = AccountUpdate;
    type Output = TokenProgramState;

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([spl_token_2022::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acct.account.as_ref().ok_or(ProgramError::InvalidArgument)?;

        TokenProgramState::try_unpack(&inner.data)
    }
}
