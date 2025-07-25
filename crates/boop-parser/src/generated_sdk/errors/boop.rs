//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum BoopError {
    /// 6000 - Amount in too low
    #[error("Amount in too low")]
    AmountInTooLow = 0x1770,
    /// 6001 - Amount out too low
    #[error("Amount out too low")]
    AmountOutTooLow = 0x1771,
    /// 6002 - Config not initialized
    #[error("Config not initialized")]
    ConfigNotInitialized = 0x1772,
    /// 6003 - Creator is not provided
    #[error("Creator is not provided")]
    CreatorIsNotProvided = 0x1773,
    /// 6004 - Damping term too low
    #[error("Damping term too low")]
    DampingTermTooLow = 0x1774,
    /// 6005 - First buy must be at most 50% of the total supply
    #[error("First buy must be at most 50% of the total supply")]
    FirstBuyMustBeAtMost50PercentOfTotalSupply = 0x1775,
    /// 6006 - Graduation fee relative to graduation target is too high
    #[error("Graduation fee relative to graduation target is too high")]
    GraduationFeeRelativeToTargetIsTooHigh = 0x1776,
    /// 6007 - Invalid bonding curve status
    #[error("Invalid bonding curve status")]
    InvalidBondingCurveStatus = 0x1777,
    /// 6008 - Insufficient tokens to transfer out of the bonding curve
    #[error("Insufficient tokens to transfer out of the bonding curve")]
    InsufficientTokensOut = 0x1778,
    /// 6009 - Invalid mint
    #[error("Invalid mint")]
    InvalidMint = 0x1779,
    /// 6010 - Invalid protocol fee recipient
    #[error("Invalid protocol fee recipient")]
    InvalidProtocolFeeRecipient = 0x177a,
    /// 6011 - Max basis points off graduation price too high
    #[error("Max basis points off graduation price too high")]
    MaxBasisPointsOffGraduationPriceTooHigh = 0x177b,
    /// 6012 - Max graduation price deviation basis points too high
    #[error("Max graduation price deviation basis points too high")]
    MaxGraduationPriceDeviationBasisPointsTooHigh = 0x177c,
    /// 6013 - Max swap amount for pool price correction basis points too high
    #[error("Max swap amount for pool price correction basis points too high")]
    MaxSwapAmountForPoolPriceCorrectionBasisPointsTooHigh = 0x177d,
    /// 6014 - Mint is larger than or equal to native mint
    #[error("Mint is larger than or equal to native mint")]
    MintIsLargerThanOrEqualToNativeMint = 0x177e,
    /// 6015 - No authority transfer in progress
    #[error("No authority transfer in progress")]
    NoAuthorityTransferInProgress = 0x177f,
    /// 6016 - LP Token amount is too low
    #[error("LP Token amount is too low")]
    NothingToDeposit = 0x1780,
    /// 6017 - Nothing to split
    #[error("Nothing to split")]
    NothingToSplit = 0x1781,
    /// 6018 - Nothing to lock
    #[error("Nothing to lock")]
    NothingToLock = 0x1782,
    /// 6019 - Operator already added
    #[error("Operator already added")]
    OperatorAlreadyAdded = 0x1783,
    /// 6020 - Operator does not exist
    #[error("Operator does not exist")]
    OperatorDoesNotExist = 0x1784,
    /// 6021 - Paused
    #[error("Paused")]
    Paused = 0x1785,
    /// 6022 - Pool is already created and has a price out of range when attempting to deposit liquidity
    #[error(
        "Pool is already created and has a price out of range when attempting to deposit liquidity"
    )]
    PoolPriceOutOfRange = 0x1786,
    /// 6023 - Swap fee basis points too high
    #[error("Swap fee basis points too high")]
    SwapFeeBasisPointsTooHigh = 0x1787,
    /// 6024 - Swap amount exceeds the reasonable limit to leave as liquidity
    #[error("Swap amount exceeds the reasonable limit to leave as liquidity")]
    SwapAmountTooHigh = 0x1788,
    /// 6025 - Token amount for Raydium liquidity too high
    #[error("Token amount for Raydium liquidity too high")]
    TokenAmountForRaydiumLiquidityTooHigh = 0x1789,
    /// 6026 - Token for stakers basis points too high
    #[error("Token for stakers basis points too high")]
    TokenForStakersBasisPointsTooHigh = 0x178a,
    /// 6027 - Token graduated
    #[error("Token graduated")]
    TokenGraduated = 0x178b,
    /// 6028 - Token name too long
    #[error("Token name too long")]
    TokenNameTooLong = 0x178c,
    /// 6029 - Token name too short
    #[error("Token name too short")]
    TokenNameTooShort = 0x178d,
    /// 6030 - Token symbol too long
    #[error("Token symbol too long")]
    TokenSymbolTooLong = 0x178e,
    /// 6031 - Token symbol too short
    #[error("Token symbol too short")]
    TokenSymbolTooShort = 0x178f,
    /// 6032 - Unauthorized
    #[error("Unauthorized")]
    Unauthorized = 0x1790,
}

impl solana_program_error::PrintProgramError for BoopError {
    fn print<E>(&self) {
        solana_msg::msg!(&self.to_string());
    }
}

impl<T> solana_decode_error::DecodeError<T> for BoopError {
    fn type_of() -> &'static str { "BoopError" }
}
