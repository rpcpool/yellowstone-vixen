use spl_token_2022::{
    extension::{BaseStateWithExtensions, ExtensionType, StateWithExtensions},
    solana_program::{program_error::ProgramError, program_pack::Pack},
    state::{Account, Mint, Multisig},
};
use yellowstone_vixen_core::{AccountUpdate, ParseResult, Parser, Prefilter};

use crate::token_extension_helpers::{
    get_mint_account_extensions_data_bytes, get_token_account_extensions_data_bytes,
};

pub enum TokenExtensionAccountType {
    TokenAccount,
    Mint,
    Multisig,
}

#[derive(Debug)]
pub struct ExtensionData {
    pub extension: ExtensionType,
    pub extension_data: Vec<u8>,
}

#[derive(Debug)]
pub struct ExtendedMint {
    pub base_account: Mint,
    pub extension_data_vec: Vec<ExtensionData>,
}

impl ExtendedMint {
    fn from_account_data(data_bytes: &[u8]) -> Result<ExtendedMint, ProgramError> {
        let unpacked = StateWithExtensions::<Mint>::unpack(data_bytes)?;
        let extension_types = unpacked.get_extension_types()?;
        let mut extension_data_vec: Vec<ExtensionData> = vec![];

        for extension in extension_types {
            let extension_data = get_mint_account_extensions_data_bytes(&unpacked, extension)?;

            extension_data_vec.push(ExtensionData {
                extension,
                extension_data: extension_data.to_owned(),
            });
        }
        //TODO: WIP
        Ok(ExtendedMint {
            base_account: unpacked.base,
            extension_data_vec,
        })
    }
}
#[derive(Debug)]
pub struct ExtendedTokenAccount {
    pub base_account: Account,
    pub extension_data_vec: Vec<ExtensionData>,
}

impl ExtendedTokenAccount {
    fn from_account_data(data_bytes: &[u8]) -> Result<ExtendedTokenAccount, ProgramError> {
        let unpacked = StateWithExtensions::<Account>::unpack(data_bytes)?;
        let extension_types = unpacked.get_extension_types()?;
        let mut extension_data_vec: Vec<ExtensionData> = vec![];

        for extension in extension_types {
            let extension_data = get_token_account_extensions_data_bytes(&unpacked, extension)?;

            extension_data_vec.push(ExtensionData {
                extension,
                extension_data: extension_data.to_owned(),
            });
        }

        //TODO: WIP
        Ok(ExtendedTokenAccount {
            base_account: unpacked.base,
            extension_data_vec,
        })
    }
}

fn get_extention_account_type(
    data_bytes: &[u8],
) -> Result<TokenExtensionAccountType, ProgramError> {
    if StateWithExtensions::<Mint>::unpack(data_bytes).is_ok() {
        return Ok(TokenExtensionAccountType::Mint);
    }

    if StateWithExtensions::<Account>::unpack(data_bytes).is_ok() {
        return Ok(TokenExtensionAccountType::TokenAccount);
    }

    if Multisig::unpack(data_bytes).is_ok() {
        return Ok(TokenExtensionAccountType::Multisig);
    } else {
        return Err(ProgramError::InvalidAccountData);
    }
}

#[derive(Debug)]
pub enum TokenExtensionState {
    ExtendedTokenAccount(ExtendedTokenAccount),
    ExtendedMint(ExtendedMint),
    Multisig(Multisig),
}

impl TokenExtensionState {
    fn try_unpack(data_bytes: &[u8]) -> ParseResult<Self> {
        let account_type = get_extention_account_type(data_bytes)?;

        match account_type {
            TokenExtensionAccountType::Mint => Ok(TokenExtensionState::ExtendedMint(
                ExtendedMint::from_account_data(data_bytes)?,
            )),
            TokenExtensionAccountType::TokenAccount => {
                Ok(TokenExtensionState::ExtendedTokenAccount(
                    ExtendedTokenAccount::from_account_data(data_bytes)?,
                ))
            }
            TokenExtensionAccountType::Multisig => {
                Ok(TokenExtensionState::Multisig(Multisig::unpack(data_bytes)?))
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

#[cfg(test)]
mod tests {
    use crate::{
        constants::token_program_constants::TOKEN_ACCOUNT_WITH_EXTENSION,
        token_extension_data_parsers::parse_transfer_hook_account_extension,
    };

    use super::*;

    #[test]
    fn test_token_account_parsing() {
        let token_account = TokenExtensionState::try_unpack(TOKEN_ACCOUNT_WITH_EXTENSION);
        assert_eq!(token_account.is_ok(), true);
        let token_account = token_account.unwrap();
        match token_account {
            TokenExtensionState::ExtendedTokenAccount(ext_token_account) => {
                println!("Token Account with Extensions: {:?}", ext_token_account);
                let ext_data = ext_token_account.extension_data_vec;
                assert_eq!(ext_data.len(), 1);
                assert_eq!(ext_data[0].extension, ExtensionType::TransferHookAccount);
                assert_eq!(
                    parse_transfer_hook_account_extension(&ext_data[0].extension_data).is_ok(),
                    true
                );

                let parsed_data =
                    parse_transfer_hook_account_extension(&ext_data[0].extension_data);

                assert_eq!(parsed_data.is_ok(), true);

                let parsed_data = parsed_data.unwrap();

                println!("Parsed Transfer Hook Account Extension: {:?}", parsed_data);
            }
            _ => panic!("Invalid account type"),
        }
    }
}
