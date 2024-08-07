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
    pub fn try_unpack(data_bytes: &[u8]) -> ParseResult<Self> {
        match data_bytes.len() {
            Mint::LEN => Mint::unpack(data_bytes).map(Self::Mint).map_err(Into::into),
            Account::LEN => Account::unpack(data_bytes)
                .map(Self::TokenAccount)
                .map_err(Into::into),
            Multisig::LEN => Multisig::unpack(data_bytes)
                .map(Self::Multisig)
                .map_err(Into::into),
            _ => Err(ParseError::from("Invalid Account data length".to_owned())),
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

#[cfg(test)]
mod tests {
    use yellowstone_vixen_mock::{account_fixture, run_parse};

    use super::*;

    #[tokio::test]
    async fn test_mint_parsing() {
        let parser = TokenProgramParser;

        let account = account_fixture!("3SmPYPvZfEmroktLiJsgaNENuPEud3Z52zSfLQ1zJdkK");

        let state = run_parse!(parser, account);

        if let TokenProgramState::Mint(mint) = state {
            assert_eq!(mint.decimals, 10);
        } else {
            panic!("Invalid Mint Account");
        }
    }
}
