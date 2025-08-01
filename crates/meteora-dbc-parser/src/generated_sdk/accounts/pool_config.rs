//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use crate::generated::types::{LiquidityDistributionConfig, LockedVestingConfig, PoolFeesConfig};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PoolConfig {
    pub discriminator: [u8; 8],
    /// quote mint
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub quote_mint: Pubkey,
    /// Address to get the fee
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub fee_claimer: Pubkey,
    /// Address to receive extra base token after migration, in case token is fixed supply
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub leftover_receiver: Pubkey,
    /// Pool fee
    pub pool_fees: PoolFeesConfig,
    /// Collect fee mode
    pub collect_fee_mode: u8,
    /// migration option
    pub migration_option: u8,
    /// whether mode slot or timestamp
    pub activation_type: u8,
    /// token decimals
    pub token_decimal: u8,
    /// version
    pub version: u8,
    /// token type of base token
    pub token_type: u8,
    /// quote token flag
    pub quote_token_flag: u8,
    /// partner locked lp percentage
    pub partner_locked_lp_percentage: u8,
    /// partner lp percentage
    pub partner_lp_percentage: u8,
    /// creator post migration fee percentage
    pub creator_locked_lp_percentage: u8,
    /// creator lp percentage
    pub creator_lp_percentage: u8,
    /// migration fee option
    pub migration_fee_option: u8,
    /// flag to indicate whether token is dynamic supply (0) or fixed supply (1)
    pub fixed_token_supply_flag: u8,
    /// creator trading fee percentage
    pub creator_trading_fee_percentage: u8,
    /// padding 0
    pub padding0: [u8; 2],
    /// padding 1
    pub padding1: [u8; 8],
    /// swap base amount
    pub swap_base_amount: u64,
    /// migration quote threshold (in quote token)
    pub migration_quote_threshold: u64,
    /// migration base threshold (in base token)
    pub migration_base_threshold: u64,
    /// migration sqrt price
    pub migration_sqrt_price: u128,
    /// locked vesting config
    pub locked_vesting_config: LockedVestingConfig,
    /// pre migration token supply
    pub pre_migration_token_supply: u64,
    /// post migration token supply
    pub post_migration_token_supply: u64,
    /// padding 2
    pub padding2: [u128; 2],
    /// minimum price
    pub sqrt_start_price: u128,
    /// curve, only use 20 point firstly, we can extend that latter
    pub curve: [LiquidityDistributionConfig; 20],
}

impl PoolConfig {
    pub const LEN: usize = 1048;

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_account_info::AccountInfo<'a>> for PoolConfig {
    type Error = std::io::Error;

    fn try_from(account_info: &solana_account_info::AccountInfo<'a>) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "fetch")]
pub fn fetch_pool_config(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &solana_pubkey::Pubkey,
) -> Result<crate::shared::DecodedAccount<PoolConfig>, std::io::Error> {
    let accounts = fetch_all_pool_config(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_pool_config(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: &[solana_pubkey::Pubkey],
) -> Result<Vec<crate::shared::DecodedAccount<PoolConfig>>, std::io::Error> {
    let accounts = rpc
        .get_multiple_accounts(addresses)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::DecodedAccount<PoolConfig>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        let account = accounts[i].as_ref().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Account not found: {}", address),
        ))?;
        let data = PoolConfig::from_bytes(&account.data)?;
        decoded_accounts.push(crate::shared::DecodedAccount {
            address,
            account: account.clone(),
            data,
        });
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "fetch")]
pub fn fetch_maybe_pool_config(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &solana_pubkey::Pubkey,
) -> Result<crate::shared::MaybeAccount<PoolConfig>, std::io::Error> {
    let accounts = fetch_all_maybe_pool_config(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_maybe_pool_config(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: &[solana_pubkey::Pubkey],
) -> Result<Vec<crate::shared::MaybeAccount<PoolConfig>>, std::io::Error> {
    let accounts = rpc
        .get_multiple_accounts(addresses)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::MaybeAccount<PoolConfig>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        if let Some(account) = accounts[i].as_ref() {
            let data = PoolConfig::from_bytes(&account.data)?;
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
impl anchor_lang::AccountDeserialize for PoolConfig {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for PoolConfig {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for PoolConfig {
    fn owner() -> Pubkey { crate::DYNAMIC_BONDING_CURVE_ID }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for PoolConfig {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for PoolConfig {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
