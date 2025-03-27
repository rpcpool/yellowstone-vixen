//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlobalConfig {
    pub discriminator: [u8; 8],
    pub emergency_mode: u8,
    pub flash_take_order_blocked: u8,
    pub new_orders_blocked: u8,
    pub orders_taking_blocked: u8,
    pub host_fee_bps: u16,
    pub is_order_taking_permissionless: u8,
    pub padding0: [u8; 1],
    /// The number of seconds after an order has been updated before it can be closed
    pub order_close_delay_seconds: u64,
    pub padding1: [u64; 9],
    /// The total amount of lamports that were present in the pda_authority last
    /// time a program instructions which alters the pda_authority account was
    /// executed
    pub pda_authority_previous_lamports_balance: u64,
    /// The total amount of tips that have been paid out - should be at least
    /// as much as the total lamports present in the pda_authority account
    pub total_tip_amount: u64,
    /// The amount of tips the host is due to receive -
    /// in lamports, stored in the pda_authority account
    pub host_tip_amount: u64,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub pda_authority: Pubkey,
    pub pda_authority_bump: u64,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub admin_authority: Pubkey,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub admin_authority_cached: Pubkey,
    pub txn_fee_cost: u64,
    pub ata_creation_cost: u64,
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub padding2: [u64; 241],
}

impl GlobalConfig {
    pub const LEN: usize = 2168;

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for GlobalConfig {
    type Error = std::io::Error;

    fn try_from(
        account_info: &solana_program::account_info::AccountInfo<'a>,
    ) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "fetch")]
pub fn fetch_global_config(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &solana_program::pubkey::Pubkey,
) -> Result<crate::shared::DecodedAccount<GlobalConfig>, std::io::Error> {
    let accounts = fetch_all_global_config(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_global_config(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: &[solana_program::pubkey::Pubkey],
) -> Result<Vec<crate::shared::DecodedAccount<GlobalConfig>>, std::io::Error> {
    let accounts = rpc
        .get_multiple_accounts(addresses)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::DecodedAccount<GlobalConfig>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        let account = accounts[i].as_ref().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Account not found: {}", address),
        ))?;
        let data = GlobalConfig::from_bytes(&account.data)?;
        decoded_accounts.push(crate::shared::DecodedAccount {
            address,
            account: account.clone(),
            data,
        });
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "fetch")]
pub fn fetch_maybe_global_config(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &solana_program::pubkey::Pubkey,
) -> Result<crate::shared::MaybeAccount<GlobalConfig>, std::io::Error> {
    let accounts = fetch_all_maybe_global_config(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_maybe_global_config(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: &[solana_program::pubkey::Pubkey],
) -> Result<Vec<crate::shared::MaybeAccount<GlobalConfig>>, std::io::Error> {
    let accounts = rpc
        .get_multiple_accounts(addresses)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::MaybeAccount<GlobalConfig>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        if let Some(account) = accounts[i].as_ref() {
            let data = GlobalConfig::from_bytes(&account.data)?;
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
impl anchor_lang::AccountDeserialize for GlobalConfig {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for GlobalConfig {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for GlobalConfig {
    fn owner() -> Pubkey { crate::LIMO_ID }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for GlobalConfig {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for GlobalConfig {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
