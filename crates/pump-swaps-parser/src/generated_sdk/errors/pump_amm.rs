//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PumpAmmError {
    /// 6000 -
    #[error("")]
    FeeBasisPointsExceedsMaximum = 0x1770,
    /// 6001 -
    #[error("")]
    ZeroBaseAmount = 0x1771,
    /// 6002 -
    #[error("")]
    ZeroQuoteAmount = 0x1772,
    /// 6003 -
    #[error("")]
    TooLittlePoolTokenLiquidity = 0x1773,
    /// 6004 -
    #[error("")]
    ExceededSlippage = 0x1774,
    /// 6005 -
    #[error("")]
    InvalidAdmin = 0x1775,
    /// 6006 -
    #[error("")]
    UnsupportedBaseMint = 0x1776,
    /// 6007 -
    #[error("")]
    UnsupportedQuoteMint = 0x1777,
    /// 6008 -
    #[error("")]
    InvalidBaseMint = 0x1778,
    /// 6009 -
    #[error("")]
    InvalidQuoteMint = 0x1779,
    /// 6010 -
    #[error("")]
    InvalidLpMint = 0x177a,
    /// 6011 -
    #[error("")]
    AllProtocolFeeRecipientsShouldBeNonZero = 0x177b,
    /// 6012 -
    #[error("")]
    UnsortedNotUniqueProtocolFeeRecipients = 0x177c,
    /// 6013 -
    #[error("")]
    InvalidProtocolFeeRecipient = 0x177d,
    /// 6014 -
    #[error("")]
    InvalidPoolBaseTokenAccount = 0x177e,
    /// 6015 -
    #[error("")]
    InvalidPoolQuoteTokenAccount = 0x177f,
    /// 6016 -
    #[error("")]
    BuyMoreBaseAmountThanPoolReserves = 0x1780,
    /// 6017 -
    #[error("")]
    DisabledCreatePool = 0x1781,
    /// 6018 -
    #[error("")]
    DisabledDeposit = 0x1782,
    /// 6019 -
    #[error("")]
    DisabledWithdraw = 0x1783,
    /// 6020 -
    #[error("")]
    DisabledBuy = 0x1784,
    /// 6021 -
    #[error("")]
    DisabledSell = 0x1785,
    /// 6022 -
    #[error("")]
    SameMint = 0x1786,
    /// 6023 -
    #[error("")]
    Overflow = 0x1787,
    /// 6024 -
    #[error("")]
    Truncation = 0x1788,
    /// 6025 -
    #[error("")]
    DivisionByZero = 0x1789,
    /// 6026 -
    #[error("")]
    NewSizeLessThanCurrentSize = 0x178a,
    /// 6027 -
    #[error("")]
    AccountTypeNotSupported = 0x178b,
    /// 6028 -
    #[error("")]
    OnlyCanonicalPumpPoolsCanHaveCoinCreator = 0x178c,
}

impl solana_program_error::PrintProgramError for PumpAmmError {
    fn print<E>(&self) {
        solana_msg::msg!(&self.to_string());
    }
}

impl<T> solana_decode_error::DecodeError<T> for PumpAmmError {
    fn type_of() -> &'static str { "PumpAmmError" }
}
