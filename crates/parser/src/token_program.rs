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

#[cfg(test)]
mod tests {
    //Acconts are also printed to std so we can confirmn the parsing
    //use --show-output to see the output
    use std::str::FromStr;

    use spl_pod::solana_program::pubkey::Pubkey;

    use crate::constants::token_program_constants::{
        MINT, MINT_AUTH, MINT_DEIMALS, MINT_SUPPLY, MULTISIG, MULTISIG_M, MULTISIG_N,
        MULTISIG_SIGNERS, TOKEN_ACCOUNT, TOKEN_ACCOUNT_DELEGATE, TOKEN_ACCOUNT_MINT,
        TOKEN_ACCOUNT_OWNER, TOKEN_ACCOUNT_STATE, TOKEN_AMOUNT,
    };

    use super::*;
    #[test]
    fn test_token_account_parsing() {
        let token_account = TokenProgramState::try_unpack(TOKEN_ACCOUNT).unwrap();
        match token_account {
            TokenProgramState::TokenAccount(token_account) => {
                assert_eq!(
                    token_account.mint,
                    Pubkey::from_str(TOKEN_ACCOUNT_MINT).unwrap()
                );
                assert_eq!(
                    token_account.owner,
                    Pubkey::from_str(TOKEN_ACCOUNT_OWNER).unwrap()
                );
                assert_eq!(token_account.amount, TOKEN_AMOUNT);
                assert_eq!(token_account.state, TOKEN_ACCOUNT_STATE);
                assert_eq!(
                    token_account.delegate.unwrap(),
                    Pubkey::from_str(TOKEN_ACCOUNT_DELEGATE).unwrap()
                );
                println!("Token account: {:#?}", token_account);
            }
            _ => panic!("Invalid Token Account"),
        }
    }
    #[test]
    fn test_mint_account_parising() {
        let mint_account = TokenProgramState::try_unpack(MINT).unwrap();
        match mint_account {
            TokenProgramState::Mint(mint) => {
                assert_eq!(mint.supply, MINT_SUPPLY);
                assert_eq!(mint.decimals, MINT_DEIMALS);
                assert_eq!(
                    mint.mint_authority.unwrap(),
                    Pubkey::from_str(MINT_AUTH).unwrap()
                );
                println!("Mint account: {:#?}", mint);
            }
            _ => panic!("Invalid Mint Account"),
        }
    }

    #[test]
    fn test_multisig_parsing() {
        let multisig = TokenProgramState::try_unpack(MULTISIG).unwrap();
        match multisig {
            TokenProgramState::Multisig(multisig) => {
                assert_eq!(multisig.m, MULTISIG_M);
                assert_eq!(multisig.n, MULTISIG_N);
                assert_eq!(multisig.signers.len(), MULTISIG_SIGNERS);
                println!("Multisig account: {:#?}", multisig);
            }
            _ => panic!("Invalid Multisig Account"),
        }
    }
}
