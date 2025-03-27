//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PerpetualsError {
    /// 6000 - Overflow in arithmetic operation
    #[error("Overflow in arithmetic operation")]
    MathOverflow = 0x1770,
    /// 6001 - Unsupported price oracle
    #[error("Unsupported price oracle")]
    UnsupportedOracle = 0x1771,
    /// 6002 - Invalid oracle account
    #[error("Invalid oracle account")]
    InvalidOracleAccount = 0x1772,
    /// 6003 - Stale oracle price
    #[error("Stale oracle price")]
    StaleOraclePrice = 0x1773,
    /// 6004 - Invalid oracle price
    #[error("Invalid oracle price")]
    InvalidOraclePrice = 0x1774,
    /// 6005 - Instruction is not allowed in production
    #[error("Instruction is not allowed in production")]
    InvalidEnvironment = 0x1775,
    /// 6006 - Invalid collateral account
    #[error("Invalid collateral account")]
    InvalidCollateralAccount = 0x1776,
    /// 6007 - Invalid collateral amount
    #[error("Invalid collateral amount")]
    InvalidCollateralAmount = 0x1777,
    /// 6008 - Collateral slippage
    #[error("Collateral slippage")]
    CollateralSlippage = 0x1778,
    /// 6009 - Invalid position state
    #[error("Invalid position state")]
    InvalidPositionState = 0x1779,
    /// 6010 - Invalid perpetuals config
    #[error("Invalid perpetuals config")]
    InvalidPerpetualsConfig = 0x177a,
    /// 6011 - Invalid pool config
    #[error("Invalid pool config")]
    InvalidPoolConfig = 0x177b,
    /// 6012 - Invalid instruction
    #[error("Invalid instruction")]
    InvalidInstruction = 0x177c,
    /// 6013 - Invalid custody config
    #[error("Invalid custody config")]
    InvalidCustodyConfig = 0x177d,
    /// 6014 - Invalid custody balance
    #[error("Invalid custody balance")]
    InvalidCustodyBalance = 0x177e,
    /// 6015 - Invalid argument
    #[error("Invalid argument")]
    InvalidArgument = 0x177f,
    /// 6016 - Invalid position request
    #[error("Invalid position request")]
    InvalidPositionRequest = 0x1780,
    /// 6017 - Invalid position request input ata
    #[error("Invalid position request input ata")]
    InvalidPositionRequestInputAta = 0x1781,
    /// 6018 - Invalid mint
    #[error("Invalid mint")]
    InvalidMint = 0x1782,
    /// 6019 - Insufficient token amount
    #[error("Insufficient token amount")]
    InsufficientTokenAmount = 0x1783,
    /// 6020 - Insufficient token amount returned
    #[error("Insufficient token amount returned")]
    InsufficientAmountReturned = 0x1784,
    /// 6021 - Price slippage limit exceeded
    #[error("Price slippage limit exceeded")]
    MaxPriceSlippage = 0x1785,
    /// 6022 - Position leverage limit exceeded
    #[error("Position leverage limit exceeded")]
    MaxLeverage = 0x1786,
    /// 6023 - Custody amount limit exceeded
    #[error("Custody amount limit exceeded")]
    CustodyAmountLimit = 0x1787,
    /// 6024 - Pool amount limit exceeded
    #[error("Pool amount limit exceeded")]
    PoolAmountLimit = 0x1788,
    /// 6025 - Personal pool amount limit exceeded
    #[error("Personal pool amount limit exceeded")]
    PersonalPoolAmountLimit = 0x1789,
    /// 6026 - Token is not supported
    #[error("Token is not supported")]
    UnsupportedToken = 0x178a,
    /// 6027 - Instruction is not allowed at this time
    #[error("Instruction is not allowed at this time")]
    InstructionNotAllowed = 0x178b,
    /// 6028 - Jupiter Program ID mismatch
    #[error("Jupiter Program ID mismatch")]
    JupiterProgramMismatch = 0x178c,
    /// 6029 - Program ID mismatch
    #[error("Program ID mismatch")]
    ProgramMismatch = 0x178d,
    /// 6030 - Address mismatch
    #[error("Address mismatch")]
    AddressMismatch = 0x178e,
    /// 6031 - Missing keeper ATA
    #[error("Missing keeper ATA")]
    KeeperATAMissing = 0x178f,
    /// 6032 - Swap amount mismatch
    #[error("Swap amount mismatch")]
    SwapAmountMismatch = 0x1790,
    /// 6033 - CPI not allowed
    #[error("CPI not allowed")]
    CPINotAllowed = 0x1791,
    /// 6034 - Invalid Keeper
    #[error("Invalid Keeper")]
    InvalidKeeper = 0x1792,
    /// 6035 - Exceed execution period
    #[error("Exceed execution period")]
    ExceedExecutionPeriod = 0x1793,
    /// 6036 - Invalid Request Type
    #[error("Invalid Request Type")]
    InvalidRequestType = 0x1794,
    /// 6037 - Invalid Trigger Price
    #[error("Invalid Trigger Price")]
    InvalidTriggerPrice = 0x1795,
    /// 6038 - Trigger Price Slippage
    #[error("Trigger Price Slippage")]
    TriggerPriceSlippage = 0x1796,
    /// 6039 - Missing Trigger Price
    #[error("Missing Trigger Price")]
    MissingTriggerPrice = 0x1797,
    /// 6040 - Missing Price Slippage
    #[error("Missing Price Slippage")]
    MissingPriceSlippage = 0x1798,
    /// 6041 - Invalid Price Calc Mode
    #[error("Invalid Price Calc Mode")]
    InvalidPriceCalcMode = 0x1799,
    /// 6042 - Request Updated Too Recent
    #[error("Request Updated Too Recent")]
    RequestUpdatedTooRecent = 0x179a,
    /// 6043 - Exceed Token Weightage
    #[error("Exceed Token Weightage")]
    ExceedTokenWeightage = 0x179b,
    /// 6044 - Oracle Publish Time Too Early
    #[error("Oracle Publish Time Too Early")]
    OraclePublishTimeTooEarly = 0x179c,
    /// 6045 - Pull Oracle Publish Time Too Early
    #[error("Pull Oracle Publish Time Too Early")]
    PullOraclePublishTimeTooEarly = 0x179d,
    /// 6046 - Stale Pull Oracle Price
    #[error("Stale Pull Oracle Price")]
    StalePullOraclePrice = 0x179e,
    /// 6047 - Invalid Pull Oracle Price
    #[error("Invalid Pull Oracle Price")]
    InvalidPullOraclePrice = 0x179f,
    /// 6048 - Pull Oracle Not Verified
    #[error("Pull Oracle Not Verified")]
    PullOracleNotVerified = 0x17a0,
    /// 6049 - Price Diff Between Pull and Push Oracle is Too Large
    #[error("Price Diff Between Pull and Push Oracle is Too Large")]
    PriceDiffTooLarge = 0x17a1,
    /// 6050 - Invalid Doves Oracle Price
    #[error("Invalid Doves Oracle Price")]
    InvalidDovesOraclePrice = 0x17a2,
    /// 6051 - Invalid Request Time
    #[error("Invalid Request Time")]
    InvalidRequestTime = 0x17a3,
    /// 6052 - Position Updated Too Recent
    #[error("Position Updated Too Recent")]
    PositionUpdatedTooRecent = 0x17a4,
    /// 6053 - Ledger token account does not match
    #[error("Ledger token account does not match")]
    LedgerTokenAccountDoesNotMatch = 0x17a5,
    /// 6054 - Invalid token ledger
    #[error("Invalid token ledger")]
    InvalidTokenLedger = 0x17a6,
    /// 6055 - Oracle Price Difference Too Large
    #[error("Oracle Price Difference Too Large")]
    OraclePriceDifferenceTooLarge = 0x17a7,
}

impl solana_program::program_error::PrintProgramError for PerpetualsError {
    fn print<E>(&self) {
        solana_program::msg!(&self.to_string());
    }
}

impl<T> solana_program::decode_error::DecodeError<T> for PerpetualsError {
    fn type_of() -> &'static str { "PerpetualsError" }
}
