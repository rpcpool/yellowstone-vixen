//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use solana_program::pubkey::Pubkey;
use borsh::BorshSerialize;
use borsh::BorshDeserialize;

/// Reserve collateral
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReserveCollateral {
/// Reserve collateral mint address
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub mint_pubkey: Pubkey,
/// Reserve collateral mint supply, used for exchange rate
pub mint_total_supply: u64,
/// Reserve collateral supply address
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub supply_vault: Pubkey,
pub padding1: [u128; 32],
pub padding2: [u128; 32],
}


