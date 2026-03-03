use yellowstone_vixen_proc_macro::vixen;

use crate::PublicKey;

#[vixen]
#[derive(Clone, PartialEq)]
pub struct StakePoolProgram {
    #[hint(
        oneof = "stake_pool_program::Instruction",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, \
                23, 24, 25, 26"
    )]
    pub instruction: Option<stake_pool_program::Instruction>,
}

pub mod stake_pool_program {
    use super::vixen;

    #[vixen(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Instruction {
        Initialize(super::InitializeInstruction),
        AddValidatorToPool(super::AddValidatorToPoolInstruction),
        RemoveValidatorFromPool(super::RemoveValidatorFromPoolInstruction),
        DecreaseValidatorStake(super::DecreaseValidatorStakeInstruction),
        IncreaseValidatorStake(super::IncreaseValidatorStakeInstruction),
        SetPreferredValidator(super::SetPreferredValidatorInstruction),
        UpdateValidatorListBalance(super::UpdateValidatorListBalanceInstruction),
        UpdateStakePoolBalance(super::UpdateStakePoolBalanceInstruction),
        CleanupRemovedValidatorEntries(super::CleanupRemovedValidatorEntriesInstruction),
        DepositStake(super::DepositStakeInstruction),
        WithdrawStake(super::WithdrawStakeInstruction),
        SetManager(super::SetManagerInstruction),
        SetFee(super::SetFeeInstruction),
        SetStaker(super::SetStakerInstruction),
        DepositSol(super::DepositSolInstruction),
        SetFundingAuthority(super::SetFundingAuthorityInstruction),
        WithdrawSol(super::WithdrawSolInstruction),
        CreateTokenMetadata(super::CreateTokenMetadataInstruction),
        UpdateTokenMetadata(super::UpdateTokenMetadataInstruction),
        IncreaseAdditionalValidatorStake(super::IncreaseAdditionalValidatorStakeInstruction),
        DecreaseAdditionalValidatorStake(super::DecreaseAdditionalValidatorStakeInstruction),
        DecreaseValidatorStakeWithReserve(super::DecreaseValidatorStakeWithReserveInstruction),
        DepositStakeWithSlippage(super::DepositStakeWithSlippageInstruction),
        WithdrawStakeWithSlippage(super::WithdrawStakeWithSlippageInstruction),
        DepositSolWithSlippage(super::DepositSolWithSlippageInstruction),
        WithdrawSolWithSlippage(super::WithdrawSolWithSlippageInstruction),
    }
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct Fee {
    pub numerator: u64,
    pub denominator: u64,
}

#[vixen(enumeration)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum PreferredValidatorType {
    Deposit = 0,
    Withdraw = 1,
}

#[vixen(enumeration)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum FundingType {
    SolDeposit = 0,
    StakeDeposit = 1,
    SolWithdraw = 2,
}

#[vixen(enumeration)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum FeeKind {
    SolReferral = 0,
    StakeReferral = 1,
    Epoch = 2,
    StakeWithdrawal = 3,
    SolDeposit = 4,
    StakeDeposit = 5,
    SolWithdrawal = 6,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct FeeType {
    #[hint(enumeration = "FeeKind")]
    pub kind: i32,

    #[hint(oneof = "fee_type::Value", tags = "2, 3")]
    pub value: Option<fee_type::Value>,
}

pub mod fee_type {
    use super::vixen;

    #[vixen(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Value {
        Fee(super::Fee),

        ReferralBps(u32),
    }
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeAccounts {
    pub stake_pool: PublicKey,
    pub manager: PublicKey,
    pub staker: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub validator_list: PublicKey,
    pub reserve_stake: PublicKey,
    pub pool_mint: PublicKey,
    pub manager_pool_account: PublicKey,
    pub token_program: PublicKey,
    pub deposit_authority: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeArgs {
    pub fee: Option<Fee>,
    pub withdrawal_fee: Option<Fee>,
    pub deposit_fee: Option<Fee>,
    pub referral_fee: u32,
    pub max_validators: u32,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeInstruction {
    pub accounts: Option<InitializeAccounts>,
    pub args: Option<InitializeArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct AddValidatorToPoolAccounts {
    pub stake_pool: PublicKey,
    pub staker: PublicKey,
    pub funder: PublicKey,
    pub stake_pool_withdraw: PublicKey,
    pub validator_list: PublicKey,
    pub stake: PublicKey,
    pub validator: PublicKey,
    pub rent: PublicKey,
    pub clock: PublicKey,
    pub sysvar_stake_history: PublicKey,
    pub stake_config: PublicKey,
    pub system_program: PublicKey,
    pub stake_program: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct AddValidatorToPoolArgs {
    pub raw_validator_seed: u32,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct AddValidatorToPoolInstruction {
    pub accounts: Option<AddValidatorToPoolAccounts>,
    pub args: Option<AddValidatorToPoolArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct RemoveValidatorFromPoolAccounts {
    pub stake_pool: PublicKey,
    pub staker: PublicKey,
    pub stake_pool_withdraw: PublicKey,
    pub validator_list: PublicKey,
    pub stake_account: PublicKey,
    pub transient_stake_account: PublicKey,
    pub clock: PublicKey,
    pub stake_program: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct RemoveValidatorFromPoolInstruction {
    pub accounts: Option<RemoveValidatorFromPoolAccounts>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeAccounts {
    pub stake_pool: PublicKey,
    pub staker: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub validator_list: PublicKey,
    pub validator_stake: PublicKey,
    pub transient_stake: PublicKey,
    pub clock: PublicKey,
    pub rent: PublicKey,
    pub system_program: PublicKey,
    pub stake_program: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeArgs {
    pub lamports: u64,
    pub transient_stake_seed: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeInstruction {
    pub accounts: Option<DecreaseValidatorStakeAccounts>,
    pub args: Option<DecreaseValidatorStakeArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct IncreaseValidatorStakeAccounts {
    pub stake_pool: PublicKey,
    pub staker: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub validator_list: PublicKey,
    pub reserve_stake: PublicKey,
    pub transient_stake: PublicKey,
    pub validator_stake: PublicKey,
    pub validator: PublicKey,
    pub clock: PublicKey,
    pub rent: PublicKey,
    pub sysvar_stake_history: PublicKey,
    pub stake_config: PublicKey,
    pub system_program: PublicKey,
    pub stake_program: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct IncreaseValidatorStakeArgs {
    pub lamports: u64,
    pub transient_stake_seed: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct IncreaseValidatorStakeInstruction {
    pub accounts: Option<IncreaseValidatorStakeAccounts>,
    pub args: Option<IncreaseValidatorStakeArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetPreferredValidatorAccounts {
    pub stake_pool_address: PublicKey,
    pub staker: PublicKey,
    pub validator_list_address: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetPreferredValidatorArgs {
    #[hint(enumeration = "PreferredValidatorType")]
    pub validator_type: i32,
    pub validator_vote_address: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetPreferredValidatorInstruction {
    pub accounts: Option<SetPreferredValidatorAccounts>,
    pub args: Option<SetPreferredValidatorArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateValidatorListBalanceAccounts {
    pub stake_pool: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub validator_list_address: PublicKey,
    pub reserve_stake: PublicKey,
    pub clock: PublicKey,
    pub sysvar_stake_history: PublicKey,
    pub stake_program: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateValidatorListBalanceArgs {
    pub start_index: u32,
    pub no_merge: bool,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateValidatorListBalanceInstruction {
    pub accounts: Option<UpdateValidatorListBalanceAccounts>,
    pub args: Option<UpdateValidatorListBalanceArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateStakePoolBalanceAccounts {
    pub stake_pool: PublicKey,
    pub withdraw_authority: PublicKey,
    pub validator_list_storage: PublicKey,
    pub reserve_stake: PublicKey,
    pub manager_fee_account: PublicKey,
    pub stake_pool_mint: PublicKey,
    pub token_program: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateStakePoolBalanceInstruction {
    pub accounts: Option<UpdateStakePoolBalanceAccounts>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct CleanupRemovedValidatorEntriesAccounts {
    pub stake_pool: PublicKey,
    pub validator_list_storage: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct CleanupRemovedValidatorEntriesInstruction {
    pub accounts: Option<CleanupRemovedValidatorEntriesAccounts>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DepositStakeAccounts {
    pub stake_pool: PublicKey,
    pub validator_list_storage: PublicKey,
    pub stake_pool_deposit_authority: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub deposit_stake_address: PublicKey,
    pub validator_stake_account: PublicKey,
    pub reserve_stake_account: PublicKey,
    pub pool_tokens_to: PublicKey,
    pub manager_fee_account: PublicKey,
    pub referrer_pool_tokens_account: PublicKey,
    pub pool_mint: PublicKey,
    pub clock: PublicKey,
    pub sysvar_stake_history: PublicKey,
    pub token_program: PublicKey,
    pub stake_program: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DepositStakeInstruction {
    pub accounts: Option<DepositStakeAccounts>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeAccounts {
    pub stake_pool: PublicKey,
    pub validator_list_storage: PublicKey,
    pub stake_pool_withdraw: PublicKey,
    pub stake_to_split: PublicKey,
    pub stake_to_receive: PublicKey,
    pub user_stake_authority: PublicKey,
    pub user_transfer_authority: PublicKey,
    pub user_pool_token_account: PublicKey,
    pub manager_fee_account: PublicKey,
    pub pool_mint: PublicKey,
    pub clock: PublicKey,
    pub token_program: PublicKey,
    pub stake_program: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeArgs {
    pub amount: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeInstruction {
    pub accounts: Option<WithdrawStakeAccounts>,
    pub args: Option<WithdrawStakeArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetManagerAccounts {
    pub stake_pool: PublicKey,
    pub manager: PublicKey,
    pub new_manager: PublicKey,
    pub new_fee_receiver: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetManagerInstruction {
    pub accounts: Option<SetManagerAccounts>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetFeeAccounts {
    pub stake_pool: PublicKey,
    pub manager: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetFeeArgs {
    pub fee: Option<FeeType>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetFeeInstruction {
    pub accounts: Option<SetFeeAccounts>,
    pub args: Option<SetFeeArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetStakerAccounts {
    pub stake_pool: PublicKey,
    pub set_staker_authority: PublicKey,
    pub new_staker: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetStakerInstruction {
    pub accounts: Option<SetStakerAccounts>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DepositSolAccounts {
    pub stake_pool: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub reserve_stake_account: PublicKey,
    pub lamports_from: PublicKey,
    pub pool_tokens_to: PublicKey,
    pub manager_fee_account: PublicKey,
    pub referrer_pool_tokens_account: PublicKey,
    pub pool_mint: PublicKey,
    pub system_program: PublicKey,
    pub token_program: PublicKey,
    pub deposit_authority: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DepositSolArgs {
    pub amount: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DepositSolInstruction {
    pub accounts: Option<DepositSolAccounts>,
    pub args: Option<DepositSolArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetFundingAuthorityAccounts {
    pub stake_pool: PublicKey,
    pub manager: PublicKey,
    pub auth: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetFundingAuthorityArgs {
    #[hint(enumeration = "FundingType")]
    pub funding_type: i32,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetFundingAuthorityInstruction {
    pub accounts: Option<SetFundingAuthorityAccounts>,
    pub args: Option<SetFundingAuthorityArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolAccounts {
    pub stake_pool: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub user_transfer_authority: PublicKey,
    pub pool_tokens_from: PublicKey,
    pub reserve_stake_account: PublicKey,
    pub lamports_to: PublicKey,
    pub manager_fee_account: PublicKey,
    pub pool_mint: PublicKey,
    pub clock: PublicKey,
    pub sysvar_stake_history: PublicKey,
    pub stake_program: PublicKey,
    pub token_program: PublicKey,
    pub sol_withdraw_authority: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolArgs {
    pub amount: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolInstruction {
    pub accounts: Option<WithdrawSolAccounts>,
    pub args: Option<WithdrawSolArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct CreateTokenMetadataAccounts {
    pub stake_pool: PublicKey,
    pub manager: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub pool_mint: PublicKey,
    pub payer: PublicKey,
    pub token_metadata: PublicKey,
    pub mpl_token_metadata: PublicKey,
    pub system_program: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct CreateTokenMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct CreateTokenMetadataInstruction {
    pub accounts: Option<CreateTokenMetadataAccounts>,
    pub args: Option<CreateTokenMetadataArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateTokenMetadataAccounts {
    pub stake_pool: PublicKey,
    pub manager: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub token_metadata: PublicKey,
    pub mpl_token_metadata: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateTokenMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateTokenMetadataInstruction {
    pub accounts: Option<UpdateTokenMetadataAccounts>,
    pub args: Option<UpdateTokenMetadataArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct IncreaseAdditionalValidatorStakeAccounts {
    pub stake_pool: PublicKey,
    pub staker: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub validator_list: PublicKey,
    pub reserve_stake: PublicKey,
    pub ephemeral_stake: PublicKey,
    pub transient_stake: PublicKey,
    pub validator_stake: PublicKey,
    pub validator: PublicKey,
    pub clock: PublicKey,
    pub stake_history: PublicKey,
    pub stake_config: PublicKey,
    pub system_program: PublicKey,
    pub stake_program: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct IncreaseAdditionalValidatorStakeArgs {
    pub lamports: u64,
    pub transient_stake_seed: u64,
    pub ephemeral_stake_seed: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct IncreaseAdditionalValidatorStakeInstruction {
    pub accounts: Option<IncreaseAdditionalValidatorStakeAccounts>,
    pub args: Option<IncreaseAdditionalValidatorStakeArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DecreaseAdditionalValidatorStakeAccounts {
    pub stake_pool: PublicKey,
    pub staker: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub validator_list: PublicKey,
    pub reserve_stake: PublicKey,
    pub validator_stake: PublicKey,
    pub ephemeral_stake: PublicKey,
    pub transient_stake: PublicKey,
    pub clock: PublicKey,
    pub stake_history: PublicKey,
    pub system_program: PublicKey,
    pub stake_program: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DecreaseAdditionalValidatorStakeArgs {
    pub lamports: u64,
    pub transient_stake_seed: u64,
    pub ephemeral_stake_seed: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DecreaseAdditionalValidatorStakeInstruction {
    pub accounts: Option<DecreaseAdditionalValidatorStakeAccounts>,
    pub args: Option<DecreaseAdditionalValidatorStakeArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeWithReserveAccounts {
    pub stake_pool: PublicKey,
    pub staker: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub validator_list: PublicKey,
    pub reserve_stake: PublicKey,
    pub validator_stake: PublicKey,
    pub transient_stake: PublicKey,
    pub clock: PublicKey,
    pub stake_history: PublicKey,
    pub system_program: PublicKey,
    pub stake_program: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeWithReserveArgs {
    pub lamports: u64,
    pub transient_stake_seed: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeWithReserveInstruction {
    pub accounts: Option<DecreaseValidatorStakeWithReserveAccounts>,
    pub args: Option<DecreaseValidatorStakeWithReserveArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DepositStakeWithSlippageAccounts {
    pub stake_pool: PublicKey,
    pub validator_list_storage: PublicKey,
    pub stake_pool_deposit_authority: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub deposit_stake_address: PublicKey,
    pub validator_stake_account: PublicKey,
    pub reserve_stake_account: PublicKey,
    pub pool_tokens_to: PublicKey,
    pub manager_fee_account: PublicKey,
    pub referrer_pool_tokens_account: PublicKey,
    pub pool_mint: PublicKey,
    pub clock: PublicKey,
    pub sysvar_stake_history: PublicKey,
    pub token_program: PublicKey,
    pub stake_program: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DepositStakeWithSlippageArgs {
    pub minimum_pool_tokens_out: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DepositStakeWithSlippageInstruction {
    pub accounts: Option<DepositStakeWithSlippageAccounts>,
    pub args: Option<DepositStakeWithSlippageArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeWithSlippageAccounts {
    pub stake_pool: PublicKey,
    pub validator_list_storage: PublicKey,
    pub stake_pool_withdraw: PublicKey,
    pub stake_to_split: PublicKey,
    pub stake_to_receive: PublicKey,
    pub user_stake_authority: PublicKey,
    pub user_transfer_authority: PublicKey,
    pub user_pool_token_account: PublicKey,
    pub manager_fee_account: PublicKey,
    pub pool_mint: PublicKey,
    pub clock: PublicKey,
    pub token_program: PublicKey,
    pub stake_program: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeWithSlippageArgs {
    pub pool_tokens_in: u64,
    pub minimum_lamports_out: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeWithSlippageInstruction {
    pub accounts: Option<WithdrawStakeWithSlippageAccounts>,
    pub args: Option<WithdrawStakeWithSlippageArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DepositSolWithSlippageAccounts {
    pub stake_pool: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub reserve_stake_account: PublicKey,
    pub lamports_from: PublicKey,
    pub pool_tokens_to: PublicKey,
    pub manager_fee_account: PublicKey,
    pub referrer_pool_tokens_account: PublicKey,
    pub pool_mint: PublicKey,
    pub system_program: PublicKey,
    pub token_program: PublicKey,
    pub deposit_authority: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DepositSolWithSlippageArgs {
    pub lamports_in: u64,
    pub minimum_pool_tokens_out: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DepositSolWithSlippageInstruction {
    pub accounts: Option<DepositSolWithSlippageAccounts>,
    pub args: Option<DepositSolWithSlippageArgs>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolWithSlippageAccounts {
    pub stake_pool: PublicKey,
    pub stake_pool_withdraw_authority: PublicKey,
    pub user_transfer_authority: PublicKey,
    pub pool_tokens_from: PublicKey,
    pub reserve_stake_account: PublicKey,
    pub lamports_to: PublicKey,
    pub manager_fee_account: PublicKey,
    pub pool_mint: PublicKey,
    pub clock: PublicKey,
    pub sysvar_stake_history: PublicKey,
    pub stake_program: PublicKey,
    pub token_program: PublicKey,
    pub sol_withdraw_authority: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolWithSlippageArgs {
    pub pool_tokens_in: u64,
    pub minimum_lamports_out: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolWithSlippageInstruction {
    pub accounts: Option<WithdrawSolWithSlippageAccounts>,
    pub args: Option<WithdrawSolWithSlippageArgs>,
}

// ── Conversion helpers ──────────────────────────────────────────────

pub(crate) fn fee_to_proto(fee: spl_stake_pool::state::Fee) -> Fee {
    Fee {
        numerator: fee.numerator,
        denominator: fee.denominator,
    }
}

pub(crate) fn preferred_validator_type_to_proto(
    v: spl_stake_pool::instruction::PreferredValidatorType,
) -> PreferredValidatorType {
    use spl_stake_pool::instruction::PreferredValidatorType as PVT;

    match v {
        PVT::Deposit => PreferredValidatorType::Deposit,
        PVT::Withdraw => PreferredValidatorType::Withdraw,
    }
}

pub(crate) fn funding_type_to_proto(v: spl_stake_pool::instruction::FundingType) -> FundingType {
    use spl_stake_pool::instruction::FundingType as FT;

    match v {
        FT::SolDeposit => FundingType::SolDeposit,
        FT::StakeDeposit => FundingType::StakeDeposit,
        FT::SolWithdraw => FundingType::SolWithdraw,
    }
}

/// StakePool::SetFee carries a `FeeType` (not a `Fee`).
/// `FeeType` is an enum whose variants carry the new `Fee` value.
/// We normalize it into `{ kind, fee }`.
pub(crate) fn fee_type_to_proto(v: spl_stake_pool::state::FeeType) -> FeeType {
    use fee_type::Value;
    use spl_stake_pool::state::FeeType as FT;

    match v {
        FT::SolDeposit(fee) => FeeType {
            kind: FeeKind::SolDeposit as i32,
            value: Some(Value::Fee(fee_to_proto(fee))),
        },
        FT::StakeDeposit(fee) => FeeType {
            kind: FeeKind::StakeDeposit as i32,
            value: Some(Value::Fee(fee_to_proto(fee))),
        },
        FT::SolWithdrawal(fee) => FeeType {
            kind: FeeKind::SolWithdrawal as i32,
            value: Some(Value::Fee(fee_to_proto(fee))),
        },
        FT::StakeWithdrawal(fee) => FeeType {
            kind: FeeKind::StakeWithdrawal as i32,
            value: Some(Value::Fee(fee_to_proto(fee))),
        },
        FT::Epoch(fee) => FeeType {
            kind: FeeKind::Epoch as i32,
            value: Some(Value::Fee(fee_to_proto(fee))),
        },
        FT::SolReferral(bps) => FeeType {
            kind: FeeKind::SolReferral as i32,
            value: Some(Value::ReferralBps(bps as u32)),
        },
        FT::StakeReferral(bps) => FeeType {
            kind: FeeKind::StakeReferral as i32,
            value: Some(Value::ReferralBps(bps as u32)),
        },
    }
}
