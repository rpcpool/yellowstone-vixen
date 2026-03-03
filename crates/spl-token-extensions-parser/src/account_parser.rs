use std::borrow::Cow;

use solana_program_error::ProgramError;
use solana_program_pack::Pack;
use spl_token_2022::{
    extension::{BaseStateWithExtensions, StateWithExtensions},
    state::{Account as SplAccount, AccountState, Mint as SplMint, Multisig as SplMultisig},
};
use yellowstone_vixen_core::{AccountUpdate, ParseResult, Parser, Prefilter, ProgramParser};
use yellowstone_vixen_proc_macro::vixen;

use crate::PublicKey;

#[vixen]
#[derive(Clone, PartialEq)]
pub struct TokenExtensionState {
    #[hint(oneof = "account::Account", tags = "1, 2, 3")]
    pub account: Option<account::Account>,
}

pub mod account {
    use super::vixen;

    #[vixen(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Account {
        ExtendedTokenAccount(super::ExtendedTokenAccount),
        ExtendedMint(super::ExtendedMint),
        Multisig(super::Multisig),
    }
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ExtensionData {
    /// `spl_token_2022::extension::ExtensionType` as i32
    pub extension_type: i32,

    /// Raw bytes of the extension payload (exactly what Token-2022 stores)
    pub data: Vec<u8>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ExtendedMint {
    pub base_account: Option<Mint>,
    pub extensions: Vec<ExtensionData>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ExtendedTokenAccount {
    pub base_account: Option<Account>,
    pub extensions: Vec<ExtensionData>,
}

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
pub struct Account {
    pub mint: PublicKey,
    pub owner: PublicKey,
    pub amount: u64,
    pub delegate: Option<PublicKey>,
    /// `spl_token_2022::state::AccountState` as u32
    pub state: u32,
    /// If native: rent-exempt reserve lamports (same semantics as spl-token)
    pub is_native: Option<u64>,
    pub delegated_amount: u64,
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

fn spl_to_mint(m: &SplMint) -> Mint {
    Mint {
        mint_authority: m
            .mint_authority
            .map(|pk| PublicKey::new(pk.to_bytes()))
            .into(),
        supply: m.supply,
        decimals: m.decimals as u32,
        is_initialized: m.is_initialized,
        freeze_authority: m
            .freeze_authority
            .map(|pk| PublicKey::new(pk.to_bytes()))
            .into(),
    }
}

fn account_state_to_u32(s: AccountState) -> u32 {
    match s {
        AccountState::Uninitialized => 0,
        AccountState::Initialized => 1,
        AccountState::Frozen => 2,
    }
}

fn spl_to_account(a: &SplAccount) -> Account {
    Account {
        mint: PublicKey::new(a.mint.to_bytes()),
        owner: PublicKey::new(a.owner.to_bytes()),
        amount: a.amount,
        delegate: a.delegate.map(|pk| PublicKey::new(pk.to_bytes())).into(),
        state: account_state_to_u32(a.state),
        is_native: a.is_native.into(),
        delegated_amount: a.delegated_amount,
        close_authority: a
            .close_authority
            .map(|pk| PublicKey::new(pk.to_bytes()))
            .into(),
    }
}

fn spl_to_multisig(multisig: &SplMultisig) -> Multisig {
    // Multisig has fixed signers array; keep only the first `n` signers
    let n = multisig.n as usize;
    let max = multisig.signers.len().min(n);

    let mut signers = Vec::with_capacity(max);

    for i in 0..max {
        signers.push(PublicKey::new(multisig.signers[i].to_bytes()));
    }

    Multisig {
        m: multisig.m as u32,
        n: multisig.n as u32,
        is_initialized: multisig.is_initialized,
        signers,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenExtensionAccountType {
    TokenAccount,
    Mint,
    Multisig,
}

fn extension_account_type(data_bytes: &[u8]) -> Result<TokenExtensionAccountType, ProgramError> {
    if StateWithExtensions::<SplMint>::unpack(data_bytes).is_ok() {
        return Ok(TokenExtensionAccountType::Mint);
    }

    if StateWithExtensions::<SplAccount>::unpack(data_bytes).is_ok() {
        return Ok(TokenExtensionAccountType::TokenAccount);
    }

    if SplMultisig::unpack(data_bytes).is_ok() {
        return Ok(TokenExtensionAccountType::Multisig);
    }

    Err(ProgramError::InvalidAccountData)
}

fn build_extensions_for_mint(
    unpacked: &StateWithExtensions<SplMint>,
) -> Result<Vec<ExtensionData>, ProgramError> {
    let extension_types = unpacked.get_extension_types()?;

    let mut out = Vec::with_capacity(extension_types.len());

    for ext in extension_types {
        let data = crate::accounts::mint_account_extensions_data_bytes(unpacked, ext)?;

        out.push(ExtensionData {
            extension_type: ext as i32,
            data: data.to_vec(),
        });
    }

    Ok(out)
}

fn build_extensions_for_account(
    unpacked: &StateWithExtensions<SplAccount>,
) -> Result<Vec<ExtensionData>, ProgramError> {
    let extension_types = unpacked.get_extension_types()?;

    let mut out = Vec::with_capacity(extension_types.len());

    for ext in extension_types {
        let data = crate::accounts::token_account_extensions_data_bytes(unpacked, ext)?;

        out.push(ExtensionData {
            extension_type: ext as i32,
            data: data.to_vec(),
        });
    }

    Ok(out)
}

impl TokenExtensionState {
    pub fn try_unpack(data_bytes: &[u8]) -> ParseResult<Self> {
        match extension_account_type(data_bytes)? {
            TokenExtensionAccountType::Mint => {
                let unpacked = StateWithExtensions::<SplMint>::unpack(data_bytes)?;
                let extensions = build_extensions_for_mint(&unpacked)?;

                Ok(TokenExtensionState {
                    account: Some(account::Account::ExtendedMint(ExtendedMint {
                        base_account: Some(spl_to_mint(&unpacked.base)),
                        extensions,
                    })),
                })
            },
            TokenExtensionAccountType::TokenAccount => {
                let unpacked = StateWithExtensions::<SplAccount>::unpack(data_bytes)?;
                let extensions = build_extensions_for_account(&unpacked)?;

                Ok(TokenExtensionState {
                    account: Some(account::Account::ExtendedTokenAccount(
                        ExtendedTokenAccount {
                            base_account: Some(spl_to_account(&unpacked.base)),
                            extensions,
                        },
                    )),
                })
            },
            TokenExtensionAccountType::Multisig => {
                let multisig = SplMultisig::unpack(data_bytes)?;

                Ok(TokenExtensionState {
                    account: Some(account::Account::Multisig(spl_to_multisig(&multisig))),
                })
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
    fn program_id(&self) -> yellowstone_vixen_core::KeyBytes<32> {
        spl_token_2022::ID.to_bytes().into()
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use yellowstone_vixen_core::Parser;
    use yellowstone_vixen_mock::account_fixture;

    use super::{account, AccountParser};

    #[tokio::test]
    async fn test_mint_account_parsing_proto() {
        let parser = AccountParser;

        let account = account_fixture!("BtSLwAFDsMX4bhamtyggn2xsdFKQvpaSzw9jEL7BNuyu", &parser);

        let state = account.account.expect("missing account");

        let account::Account::ExtendedMint(ext_mint) = state else {
            panic!("Invalid Account");
        };

        let base = ext_mint.base_account.expect("missing base mint");

        assert_eq!(base.decimals, 9);

        // Extensions count will depend on the fixture
        assert!(!ext_mint.extensions.is_empty());
    }
}
