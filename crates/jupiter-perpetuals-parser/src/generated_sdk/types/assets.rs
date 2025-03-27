//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Assets {
    pub fees_reserves: u64,
    pub owned: u64,
    pub locked: u64,
    pub guaranteed_usd: u64,
    pub global_short_sizes: u64,
    pub global_short_average_prices: u64,
}
