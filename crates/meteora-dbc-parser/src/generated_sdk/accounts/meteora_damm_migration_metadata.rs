//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MeteoraDammMigrationMetadata {
    pub discriminator: [u8; 8],
    /// pool
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub virtual_pool: Pubkey,
    /// pool creator
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub pool_creator: Pubkey,
    /// partner
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub partner: Pubkey,
    /// lp mint
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub lp_mint: Pubkey,
    /// partner locked lp
    pub partner_locked_lp: u64,
    /// partner lp
    pub partner_lp: u64,
    /// creator locked lp
    pub creator_locked_lp: u64,
    /// creator lp
    pub creator_lp: u64,
    /// padding
    pub padding0: u8,
    /// flag to check whether lp is locked for creator
    pub creator_locked_status: u8,
    /// flag to check whether lp is locked for partner
    pub partner_locked_status: u8,
    /// flag to check whether creator has claimed lp token
    pub creator_claim_status: u8,
    /// flag to check whether partner has claimed lp token
    pub partner_claim_status: u8,
    /// Reserve
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::Bytes>"))]
    pub padding: [u8; 107],
}

impl MeteoraDammMigrationMetadata {
    pub const LEN: usize = 280;

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_account_info::AccountInfo<'a>> for MeteoraDammMigrationMetadata {
    type Error = std::io::Error;

    fn try_from(account_info: &solana_account_info::AccountInfo<'a>) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "fetch")]
pub fn fetch_meteora_damm_migration_metadata(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &solana_pubkey::Pubkey,
) -> Result<crate::shared::DecodedAccount<MeteoraDammMigrationMetadata>, std::io::Error> {
    let accounts = fetch_all_meteora_damm_migration_metadata(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_meteora_damm_migration_metadata(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: &[solana_pubkey::Pubkey],
) -> Result<Vec<crate::shared::DecodedAccount<MeteoraDammMigrationMetadata>>, std::io::Error> {
    let accounts = rpc
        .get_multiple_accounts(addresses)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::DecodedAccount<MeteoraDammMigrationMetadata>> =
        Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        let account = accounts[i].as_ref().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Account not found: {}", address),
        ))?;
        let data = MeteoraDammMigrationMetadata::from_bytes(&account.data)?;
        decoded_accounts.push(crate::shared::DecodedAccount {
            address,
            account: account.clone(),
            data,
        });
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "fetch")]
pub fn fetch_maybe_meteora_damm_migration_metadata(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &solana_pubkey::Pubkey,
) -> Result<crate::shared::MaybeAccount<MeteoraDammMigrationMetadata>, std::io::Error> {
    let accounts = fetch_all_maybe_meteora_damm_migration_metadata(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_maybe_meteora_damm_migration_metadata(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: &[solana_pubkey::Pubkey],
) -> Result<Vec<crate::shared::MaybeAccount<MeteoraDammMigrationMetadata>>, std::io::Error> {
    let accounts = rpc
        .get_multiple_accounts(addresses)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::MaybeAccount<MeteoraDammMigrationMetadata>> =
        Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        if let Some(account) = accounts[i].as_ref() {
            let data = MeteoraDammMigrationMetadata::from_bytes(&account.data)?;
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
impl anchor_lang::AccountDeserialize for MeteoraDammMigrationMetadata {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for MeteoraDammMigrationMetadata {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for MeteoraDammMigrationMetadata {
    fn owner() -> Pubkey { crate::DYNAMIC_BONDING_CURVE_ID }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for MeteoraDammMigrationMetadata {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for MeteoraDammMigrationMetadata {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
