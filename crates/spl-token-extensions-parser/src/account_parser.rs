use std::borrow::Cow;

use solana_program_error::ProgramError;
use solana_program_pack::Pack;
use spl_token_2022::{
    extension::{BaseStateWithExtensions, StateWithExtensions},
    state::{Account, Mint, Multisig},
};
use yellowstone_vixen_core::{AccountUpdate, ParseResult, Parser, Prefilter, ProgramParser};

use super::accounts::{
    mint_account_extensions_data_bytes, token_account_extensions_data_bytes, ExtensionData,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenExtensionAccountType {
    TokenAccount,
    Mint,
    Multisig,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub struct AccountParser;

impl Parser for AccountParser {
    type Input = AccountUpdate;
    type Output = TokenExtensionState;

    fn id(&self) -> Cow<'static, str> { "token_extensions::AccountParser".into() }

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

impl ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { spl_token_2022::ID.to_bytes().into() }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use yellowstone_vixen_mock::{account_fixture, run_account_parse, FixtureData};

    use super::{AccountParser, ExtensionData, Parser, TokenExtensionState};

    #[tokio::test]
    async fn test_mint_account_parsing() {
        let parser = AccountParser;

        let account = account_fixture!("BtSLwAFDsMX4bhamtyggn2xsdFKQvpaSzw9jEL7BNuyu", &parser);

        let TokenExtensionState::ExtendedMint(ext_mint) = account else {
            panic!("Invalid Account");
        };

        assert_eq!(ext_mint.base_account.decimals, 9);
        assert_eq!(ext_mint.extension_data_vec.len(), 2);

        let extension_data = &ext_mint.extension_data_vec[1];

        let ExtensionData::TokenMetadata(meta) = extension_data else {
            panic!("Invalid Extension Data");
        };

        assert_eq!(
            meta.mint.to_string(),
            "BtSLwAFDsMX4bhamtyggn2xsdFKQvpaSzw9jEL7BNuyu"
        );
        assert_eq!(meta.name, "vixen_test");
        assert_eq!(meta.symbol, "VIX");
    }
}
