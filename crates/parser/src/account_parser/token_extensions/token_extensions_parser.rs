use spl_token_2022::{
    extension::{BaseStateWithExtensions, StateWithExtensions},
    solana_program::{program_error::ProgramError, program_pack::Pack},
    state::{Account, Mint, Multisig},
};
use yellowstone_vixen_core::{AccountUpdate, ParseResult, Parser, Prefilter};

use super::token_extension_helpers::{
    mint_account_extensions_data_bytes, token_account_extensions_data_bytes, ExtensionData,
};

#[derive(Debug)]
pub enum TokenExtensionAccountType {
    TokenAccount,
    Mint,
    Multisig,
}

#[derive(Debug)]
pub struct ExtendedMint {
    pub base_account: Mint,
    pub extension_data_vec: Vec<ExtensionData>,
}

impl ExtendedMint {
    fn try_from_data(data_bytes: &[u8]) -> Result<ExtendedMint, ProgramError> {
        let unpacked = StateWithExtensions::<Mint>::unpack(data_bytes)?;
        let extension_types = unpacked.get_extension_types()?;
        let mut extension_data_vec: Vec<ExtensionData> = Vec::with_capacity(extension_types.len());

        for extension in extension_types {
            let extension_data = mint_account_extensions_data_bytes(&unpacked, extension)?;
            extension_data_vec.push(ExtensionData::try_from((extension, extension_data))?);
        }

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
    fn try_from_data(data_bytes: &[u8]) -> Result<ExtendedTokenAccount, ProgramError> {
        let unpacked = StateWithExtensions::<Account>::unpack(data_bytes)?;
        let extension_types = unpacked.get_extension_types()?;
        let mut extension_data_vec: Vec<ExtensionData> = Vec::with_capacity(extension_types.len());

        for extension in extension_types {
            let extension_data = token_account_extensions_data_bytes(&unpacked, extension)?;
            extension_data_vec.push(ExtensionData::try_from((extension, extension_data))?);
        }

        Ok(ExtendedTokenAccount {
            base_account: unpacked.base,
            extension_data_vec,
        })
    }
}

fn extension_account_type(data_bytes: &[u8]) -> Result<TokenExtensionAccountType, ProgramError> {
    if StateWithExtensions::<Mint>::unpack(data_bytes).is_ok() {
        return Ok(TokenExtensionAccountType::Mint);
    }

    if StateWithExtensions::<Account>::unpack(data_bytes).is_ok() {
        return Ok(TokenExtensionAccountType::TokenAccount);
    }

    if Multisig::unpack(data_bytes).is_ok() {
        return Ok(TokenExtensionAccountType::Multisig);
    }

    Err(ProgramError::InvalidAccountData)
}

#[derive(Debug)]
pub enum TokenExtensionState {
    ExtendedTokenAccount(ExtendedTokenAccount),
    ExtendedMint(ExtendedMint),
    Multisig(Multisig),
}

impl TokenExtensionState {
    pub fn try_unpack(data_bytes: &[u8]) -> ParseResult<Self> {
        let account_type = extension_account_type(data_bytes)?;

        match account_type {
            TokenExtensionAccountType::Mint => Ok(TokenExtensionState::ExtendedMint(
                ExtendedMint::try_from_data(data_bytes)?,
            )),
            TokenExtensionAccountType::TokenAccount => {
                Ok(TokenExtensionState::ExtendedTokenAccount(
                    ExtendedTokenAccount::try_from_data(data_bytes)?,
                ))
            },
            TokenExtensionAccountType::Multisig => {
                Ok(TokenExtensionState::Multisig(Multisig::unpack(data_bytes)?))
            },
        }
    }
}

pub struct TokenExtensionProgramParser;

impl Parser for TokenExtensionProgramParser {
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
    use core::panic;

    use yellowstone_vixen_mock::{account_fixture, run_parse};

    use super::*;

    #[tokio::test]
    async fn test_mint_parsing() {
        let parser = TokenExtensionProgramParser;

        let account = account_fixture!("BtSLwAFDsMX4bhamtyggn2xsdFKQvpaSzw9jEL7BNuyu");

        let state = run_parse!(parser, account);

        if let TokenExtensionState::ExtendedMint(ext_mint) = state {
            assert_eq!(ext_mint.base_account.decimals as u8, 9);

            assert_eq!(ext_mint.extension_data_vec.len(), 2);

            let extension_data = &ext_mint.extension_data_vec[1];

            if let ExtensionData::TokenMetadata(meta) = extension_data {
                assert_eq!(
                    meta.mint.to_string(),
                    "BtSLwAFDsMX4bhamtyggn2xsdFKQvpaSzw9jEL7BNuyu"
                );

                assert_eq!(meta.name, "vixen_test");

                assert_eq!(meta.symbol, "VIX");
            } else {
                panic!("Invalid Extension Data");
            }
        } else {
            panic!("Invalid Mint Account");
        }
    }
}
