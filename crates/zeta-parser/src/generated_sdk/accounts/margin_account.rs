//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::generated::types::{AnchorDecimal, Asset, MarginAccountType, ProductLedger};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarginAccount {
    pub discriminator: [u8; 8],
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub authority: Pubkey,
    pub nonce: u8,
    pub balance: u64,
    pub force_cancel_flag: bool,
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::Bytes>"))]
    pub open_orders_nonce: [u8; 138],
    pub series_expiry: [u64; 5],
    pub series_expiry_padding: u64,
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub product_ledgers: [ProductLedger; 46],
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub product_ledgers_padding: [ProductLedger; 91],
    pub perp_product_ledger: ProductLedger,
    pub rebalance_amount: i64,
    pub asset: Asset,
    pub account_type: MarginAccountType,
    pub last_funding_delta: AnchorDecimal,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub delegated_pubkey: Pubkey,
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::Bytes>"))]
    pub padding: [u8; 338],
}

impl MarginAccount {
    pub const LEN: usize = 6152;

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for MarginAccount {
    type Error = std::io::Error;

    fn try_from(
        account_info: &solana_program::account_info::AccountInfo<'a>,
    ) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "fetch")]
pub fn fetch_margin_account(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &solana_program::pubkey::Pubkey,
) -> Result<crate::shared::DecodedAccount<MarginAccount>, std::io::Error> {
    let accounts = fetch_all_margin_account(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_margin_account(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: &[solana_program::pubkey::Pubkey],
) -> Result<Vec<crate::shared::DecodedAccount<MarginAccount>>, std::io::Error> {
    let accounts = rpc
        .get_multiple_accounts(addresses)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::DecodedAccount<MarginAccount>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        let account = accounts[i].as_ref().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Account not found: {}", address),
        ))?;
        let data = MarginAccount::from_bytes(&account.data)?;
        decoded_accounts.push(crate::shared::DecodedAccount {
            address,
            account: account.clone(),
            data,
        });
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "fetch")]
pub fn fetch_maybe_margin_account(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &solana_program::pubkey::Pubkey,
) -> Result<crate::shared::MaybeAccount<MarginAccount>, std::io::Error> {
    let accounts = fetch_all_maybe_margin_account(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_maybe_margin_account(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: &[solana_program::pubkey::Pubkey],
) -> Result<Vec<crate::shared::MaybeAccount<MarginAccount>>, std::io::Error> {
    let accounts = rpc
        .get_multiple_accounts(addresses)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::MaybeAccount<MarginAccount>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        if let Some(account) = accounts[i].as_ref() {
            let data = MarginAccount::from_bytes(&account.data)?;
            decoded_accounts.push(crate::shared::MaybeAccount::Exists(
                crate::shared::DecodedAccount {
                    address,
                    account: account.clone(),
                    data,
                },
            ));
        } else {
            decoded_accounts.push(crate::shared::MaybeAccount::NotFound(address));
        }
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountDeserialize for MarginAccount {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for MarginAccount {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for MarginAccount {
    fn owner() -> Pubkey { crate::ZETA_ID }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for MarginAccount {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for MarginAccount {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
