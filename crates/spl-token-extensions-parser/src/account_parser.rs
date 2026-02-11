use std::borrow::Cow;

use solana_program_error::ProgramError;
use solana_program_pack::Pack;
use spl_token_2022::{
    extension::{BaseStateWithExtensions, ExtensionType, StateWithExtensions},
    state::{Account, AccountState, Mint, Multisig},
};
use yellowstone_vixen_core::{AccountUpdate, ParseResult, Parser, Prefilter, ProgramParser};

use crate::PubkeyBytes;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenExtensionStateProto {
    #[prost(oneof = "token_extension_state_proto::State", tags = "1, 2, 3")]
    pub state: Option<token_extension_state_proto::State>,
}

pub mod token_extension_state_proto {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum State {
        #[prost(message, tag = "1")]
        ExtendedTokenAccount(super::ExtendedTokenAccountProto),
        #[prost(message, tag = "2")]
        ExtendedMint(super::ExtendedMintProto),
        #[prost(message, tag = "3")]
        Multisig(super::MultisigProto),
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionDataProto {
    /// `spl_token_2022::extension::ExtensionType` as i32
    #[prost(int32, tag = "1")]
    pub extension_type: i32,

    /// Raw bytes of the extension payload (exactly what Token-2022 stores)
    #[prost(bytes = "vec", tag = "2")]
    pub data: Vec<u8>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedMintProto {
    #[prost(message, tag = "1")]
    pub base_account: Option<MintProto>,
    #[prost(message, repeated, tag = "2")]
    pub extensions: Vec<ExtensionDataProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedTokenAccountProto {
    #[prost(message, tag = "1")]
    pub base_account: Option<AccountProto>,
    #[prost(message, repeated, tag = "2")]
    pub extensions: Vec<ExtensionDataProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintProto {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub mint_authority: Option<PubkeyBytes>,
    #[prost(uint64, tag = "2")]
    pub supply: u64,
    #[prost(uint32, tag = "3")]
    pub decimals: u32,
    #[prost(bool, tag = "4")]
    pub is_initialized: bool,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub freeze_authority: Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountProto {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub owner: PubkeyBytes,
    #[prost(uint64, tag = "3")]
    pub amount: u64,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub delegate: Option<PubkeyBytes>,
    /// `spl_token_2022::state::AccountState` as u32
    #[prost(uint32, tag = "5")]
    pub state: u32,
    /// If native: rent-exempt reserve lamports (same semantics as spl-token)
    #[prost(uint64, optional, tag = "6")]
    pub is_native: Option<u64>,
    #[prost(uint64, tag = "7")]
    pub delegated_amount: u64,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub close_authority: Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultisigProto {
    #[prost(uint32, tag = "1")]
    pub m: u32,
    #[prost(uint32, tag = "2")]
    pub n: u32,
    #[prost(bool, tag = "3")]
    pub is_initialized: bool,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub signers: Vec<PubkeyBytes>,
}

fn mint_to_proto(m: &Mint) -> MintProto {
    MintProto {
        mint_authority: m.mint_authority.map(|pk| pk.to_bytes().to_vec()).into(),
        supply: m.supply,
        decimals: m.decimals as u32,
        is_initialized: m.is_initialized,
        freeze_authority: m.freeze_authority.map(|pk| pk.to_bytes().to_vec()).into(),
    }
}

fn account_state_to_u32(s: AccountState) -> u32 {
    match s {
        AccountState::Uninitialized => 0,
        AccountState::Initialized => 1,
        AccountState::Frozen => 2,
    }
}

fn account_to_proto(a: &Account) -> AccountProto {
    AccountProto {
        mint: a.mint.to_bytes().to_vec(),
        owner: a.owner.to_bytes().to_vec(),
        amount: a.amount,
        delegate: a.delegate.map(|pk| pk.to_bytes().to_vec()).into(),
        state: account_state_to_u32(a.state),
        is_native: a.is_native.into(),
        delegated_amount: a.delegated_amount,
        close_authority: a.close_authority.map(|pk| pk.to_bytes().to_vec()).into(),
    }
}

fn multisig_to_proto(multisig: &Multisig) -> MultisigProto {
    // Multisig has fixed signers array; keep only the first `n` signers
    let n = multisig.n as usize;
    let max = multisig.signers.len().min(n);

    let mut signers = Vec::with_capacity(max);

    for i in 0..max {
        signers.push(multisig.signers[i].to_bytes().to_vec());
    }

    MultisigProto {
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

fn extension_type_to_i32(ext: ExtensionType) -> i32 {
    // Keep stable: just cast the enum discriminant
    ext as i32
}

fn build_extensions_for_mint(
    unpacked: &StateWithExtensions<Mint>,
) -> Result<Vec<ExtensionDataProto>, ProgramError> {
    let extension_types = unpacked.get_extension_types()?;

    let mut out = Vec::with_capacity(extension_types.len());

    for ext in extension_types {
        let data = crate::accounts::mint_account_extensions_data_bytes(unpacked, ext)?;

        out.push(ExtensionDataProto {
            extension_type: extension_type_to_i32(ext),
            data: data.to_vec(),
        });
    }

    Ok(out)
}

fn build_extensions_for_account(
    unpacked: &StateWithExtensions<Account>,
) -> Result<Vec<ExtensionDataProto>, ProgramError> {
    let extension_types = unpacked.get_extension_types()?;

    let mut out = Vec::with_capacity(extension_types.len());

    for ext in extension_types {
        let data = crate::accounts::token_account_extensions_data_bytes(unpacked, ext)?;

        out.push(ExtensionDataProto {
            extension_type: extension_type_to_i32(ext),
            data: data.to_vec(),
        });
    }

    Ok(out)
}

impl TokenExtensionStateProto {
    pub fn try_unpack(data_bytes: &[u8]) -> ParseResult<Self> {
        match extension_account_type(data_bytes)? {
            TokenExtensionAccountType::Mint => {
                let unpacked = StateWithExtensions::<Mint>::unpack(data_bytes)?;
                let extensions = build_extensions_for_mint(&unpacked)?;

                Ok(TokenExtensionStateProto {
                    state: Some(token_extension_state_proto::State::ExtendedMint(
                        ExtendedMintProto {
                            base_account: Some(mint_to_proto(&unpacked.base)),
                            extensions,
                        },
                    )),
                })
            },
            TokenExtensionAccountType::TokenAccount => {
                let unpacked = StateWithExtensions::<Account>::unpack(data_bytes)?;
                let extensions = build_extensions_for_account(&unpacked)?;

                Ok(TokenExtensionStateProto {
                    state: Some(token_extension_state_proto::State::ExtendedTokenAccount(
                        ExtendedTokenAccountProto {
                            base_account: Some(account_to_proto(&unpacked.base)),
                            extensions,
                        },
                    )),
                })
            },
            TokenExtensionAccountType::Multisig => {
                let multisig = Multisig::unpack(data_bytes)?;

                Ok(TokenExtensionStateProto {
                    state: Some(token_extension_state_proto::State::Multisig(
                        multisig_to_proto(&multisig),
                    )),
                })
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AccountParser;

impl Parser for AccountParser {
    type Input = AccountUpdate;
    type Output = TokenExtensionStateProto;

    fn id(&self) -> Cow<'static, str> { "token_extensions::AccountParser".into() }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([spl_token_2022::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acct.account.as_ref().ok_or(ProgramError::InvalidArgument)?;
        TokenExtensionStateProto::try_unpack(&inner.data)
    }
}

impl ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { spl_token_2022::ID.to_bytes().into() }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use yellowstone_vixen_core::Parser;
    use yellowstone_vixen_mock::{account_fixture, run_account_parse, FixtureData};

    use super::{token_extension_state_proto, AccountParser};

    #[tokio::test]
    async fn test_mint_account_parsing_proto() {
        let parser = AccountParser;

        let account = account_fixture!("BtSLwAFDsMX4bhamtyggn2xsdFKQvpaSzw9jEL7BNuyu", &parser);

        let state = account.state.expect("missing state");

        let token_extension_state_proto::State::ExtendedMint(ext_mint) = state else {
            panic!("Invalid Account");
        };

        let base = ext_mint.base_account.expect("missing base mint");

        assert_eq!(base.decimals, 9);

        // Extensions count will depend on the fixture
        assert!(!ext_mint.extensions.is_empty());
    }
}
