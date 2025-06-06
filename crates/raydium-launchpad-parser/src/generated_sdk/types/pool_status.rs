//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};
use num_derive::FromPrimitive;

/// Represents the different states a pool can be in
/// * Fund - Initial state where pool is accepting funds
/// * Migrate - Pool funding has ended and waiting for migration
/// * Trade - Pool migration is complete and amm trading is enabled
#[derive(
    BorshSerialize,
    BorshDeserialize,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Copy,
    PartialOrd,
    Hash,
    FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PoolStatus {
    Fund,
    Migrate,
    Trade,
}
