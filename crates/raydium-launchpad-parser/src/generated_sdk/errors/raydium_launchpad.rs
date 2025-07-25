//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum RaydiumLaunchpadError {
    /// 6000 - Not approved
    #[error("Not approved")]
    NotApproved = 0x1770,
    /// 6001 - Input account owner is not the program address
    #[error("Input account owner is not the program address")]
    InvalidOwner = 0x1771,
    /// 6002 - InvalidInput
    #[error("InvalidInput")]
    InvalidInput = 0x1772,
    /// 6003 - The input params are not match with curve type in config
    #[error("The input params are not match with curve type in config")]
    InputNotMatchCurveConfig = 0x1773,
    /// 6004 - Exceeds desired slippage limit
    #[error("Exceeds desired slippage limit")]
    ExceededSlippage = 0x1774,
    /// 6005 - Pool funding
    #[error("Pool funding")]
    PoolFunding = 0x1775,
    /// 6006 - Pool migrated
    #[error("Pool migrated")]
    PoolMigrated = 0x1776,
    /// 6007 - Migrate type not match
    #[error("Migrate type not match")]
    MigrateTypeNotMatch = 0x1777,
    /// 6008 - Math overflow
    #[error("Math overflow")]
    MathOverflow = 0x1778,
    /// 6009 - No assets to collect
    #[error("No assets to collect")]
    NoAssetsToCollect = 0x1779,
    /// 6010 - Vesting ratio too high
    #[error("Vesting ratio too high")]
    VestingRatioTooHigh = 0x177a,
    /// 6011 - Vesting setting ended
    #[error("Vesting setting ended")]
    VestingSettingEnded = 0x177b,
    /// 6012 - Vesting not started
    #[error("Vesting not started")]
    VestingNotStarted = 0x177c,
    /// 6013 - No vesting schedule
    #[error("No vesting schedule")]
    NoVestingSchedule = 0x177d,
    /// 6014 - The platform info input is invalid
    #[error("The platform info input is invalid")]
    InvalidPlatformInfo = 0x177e,
    /// 6015 - Pool not migrated
    #[error("Pool not migrated")]
    PoolNotMigrated = 0x177f,
}

impl solana_program_error::PrintProgramError for RaydiumLaunchpadError {
    fn print<E>(&self) {
        solana_msg::msg!(&self.to_string());
    }
}

impl<T> solana_decode_error::DecodeError<T> for RaydiumLaunchpadError {
    fn type_of() -> &'static str { "RaydiumLaunchpadError" }
}
