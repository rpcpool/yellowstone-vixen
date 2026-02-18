use yellowstone_vixen_proc_macro::vixen_proto;

use crate::PubkeyBytes;

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct StakePoolProgramInstruction {
    #[vixen_proto_hint(
        oneof = "stake_pool_program_instruction::Instruction",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, \
                23, 24, 25, 26"
    )]
    pub instruction: Option<stake_pool_program_instruction::Instruction>,
}

pub mod stake_pool_program_instruction {
    use super::vixen_proto;

    #[vixen_proto(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Instruction {
        Initialize(super::Initialize),
        AddValidatorToPool(super::AddValidatorToPool),
        RemoveValidatorFromPool(super::RemoveValidatorFromPool),
        DecreaseValidatorStake(super::DecreaseValidatorStake),
        IncreaseValidatorStake(super::IncreaseValidatorStake),
        SetPreferredValidator(super::SetPreferredValidator),
        UpdateValidatorListBalance(super::UpdateValidatorListBalance),
        UpdateStakePoolBalance(super::UpdateStakePoolBalance),
        CleanupRemovedValidatorEntries(super::CleanupRemovedValidatorEntries),
        DepositStake(super::DepositStake),
        WithdrawStake(super::WithdrawStake),
        SetManager(super::SetManager),
        SetFee(super::SetFee),
        SetStaker(super::SetStaker),
        DepositSol(super::DepositSol),
        SetFundingAuthority(super::SetFundingAuthority),
        WithdrawSol(super::WithdrawSol),
        CreateTokenMetadata(super::CreateTokenMetadata),
        UpdateTokenMetadata(super::UpdateTokenMetadata),
        IncreaseAdditionalValidatorStake(super::IncreaseAdditionalValidatorStake),
        DecreaseAdditionalValidatorStake(super::DecreaseAdditionalValidatorStake),
        DecreaseValidatorStakeWithReserve(super::DecreaseValidatorStakeWithReserve),
        DepositStakeWithSlippage(super::DepositStakeWithSlippage),
        WithdrawStakeWithSlippage(super::WithdrawStakeWithSlippage),
        DepositSolWithSlippage(super::DepositSolWithSlippage),
        WithdrawSolWithSlippage(super::WithdrawSolWithSlippage),
    }
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct Fee {
    pub numerator: u64,
    pub denominator: u64,
}

#[vixen_proto(enumeration)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum PreferredValidatorType {
    Deposit = 0,
    Withdraw = 1,
}

#[vixen_proto(enumeration)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum FundingType {
    SolDeposit = 0,
    StakeDeposit = 1,
    SolWithdraw = 2,
}

#[vixen_proto(enumeration)]
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

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct FeeType {
    #[vixen_proto_hint(enumeration = "FeeKind")]
    pub kind: i32,

    #[vixen_proto_hint(oneof = "fee_type::Value", tags = "2, 3")]
    pub value: Option<fee_type::Value>,
}

pub mod fee_type {
    use super::vixen_proto;

    #[vixen_proto(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Value {
        Fee(super::Fee),

        ReferralBps(u32),
    }
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeAccounts {
    pub stake_pool: PubkeyBytes,
    pub manager: PubkeyBytes,
    pub staker: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub validator_list: PubkeyBytes,
    pub reserve_stake: PubkeyBytes,
    pub pool_mint: PubkeyBytes,
    pub manager_pool_account: PubkeyBytes,
    pub token_program: PubkeyBytes,
    pub deposit_authority: Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeArgs {
    pub fee: Option<Fee>,
    pub withdrawal_fee: Option<Fee>,
    pub deposit_fee: Option<Fee>,
    pub referral_fee: u32,
    pub max_validators: u32,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct Initialize {
    pub accounts: Option<InitializeAccounts>,
    pub args: Option<InitializeArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct AddValidatorToPoolAccounts {
    pub stake_pool: PubkeyBytes,
    pub staker: PubkeyBytes,
    pub funder: PubkeyBytes,
    pub stake_pool_withdraw: PubkeyBytes,
    pub validator_list: PubkeyBytes,
    pub stake: PubkeyBytes,
    pub validator: PubkeyBytes,
    pub rent: PubkeyBytes,
    pub clock: PubkeyBytes,
    pub sysvar_stake_history: PubkeyBytes,
    pub stake_config: PubkeyBytes,
    pub system_program: PubkeyBytes,
    pub stake_program: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct AddValidatorToPoolArgs {
    pub raw_validator_seed: u32,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct AddValidatorToPool {
    pub accounts: Option<AddValidatorToPoolAccounts>,
    pub args: Option<AddValidatorToPoolArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct RemoveValidatorFromPoolAccounts {
    pub stake_pool: PubkeyBytes,
    pub staker: PubkeyBytes,
    pub stake_pool_withdraw: PubkeyBytes,
    pub validator_list: PubkeyBytes,
    pub stake_account: PubkeyBytes,
    pub transient_stake_account: PubkeyBytes,
    pub clock: PubkeyBytes,
    pub stake_program: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct RemoveValidatorFromPool {
    pub accounts: Option<RemoveValidatorFromPoolAccounts>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeAccounts {
    pub stake_pool: PubkeyBytes,
    pub staker: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub validator_list: PubkeyBytes,
    pub validator_stake: PubkeyBytes,
    pub transient_stake: PubkeyBytes,
    pub clock: PubkeyBytes,
    pub rent: PubkeyBytes,
    pub system_program: PubkeyBytes,
    pub stake_program: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeArgs {
    pub lamports: u64,
    pub transient_stake_seed: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStake {
    pub accounts: Option<DecreaseValidatorStakeAccounts>,
    pub args: Option<DecreaseValidatorStakeArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct IncreaseValidatorStakeAccounts {
    pub stake_pool: PubkeyBytes,
    pub staker: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub validator_list: PubkeyBytes,
    pub reserve_stake: PubkeyBytes,
    pub transient_stake: PubkeyBytes,
    pub validator_stake: PubkeyBytes,
    pub validator: PubkeyBytes,
    pub clock: PubkeyBytes,
    pub rent: PubkeyBytes,
    pub sysvar_stake_history: PubkeyBytes,
    pub stake_config: PubkeyBytes,
    pub system_program: PubkeyBytes,
    pub stake_program: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct IncreaseValidatorStakeArgs {
    pub lamports: u64,
    pub transient_stake_seed: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct IncreaseValidatorStake {
    pub accounts: Option<IncreaseValidatorStakeAccounts>,
    pub args: Option<IncreaseValidatorStakeArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetPreferredValidatorAccounts {
    pub stake_pool_address: PubkeyBytes,
    pub staker: PubkeyBytes,
    pub validator_list_address: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetPreferredValidatorArgs {
    #[vixen_proto_hint(enumeration = "PreferredValidatorType")]
    pub validator_type: i32,
    pub validator_vote_address: Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetPreferredValidator {
    pub accounts: Option<SetPreferredValidatorAccounts>,
    pub args: Option<SetPreferredValidatorArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateValidatorListBalanceAccounts {
    pub stake_pool: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub validator_list_address: PubkeyBytes,
    pub reserve_stake: PubkeyBytes,
    pub clock: PubkeyBytes,
    pub sysvar_stake_history: PubkeyBytes,
    pub stake_program: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateValidatorListBalanceArgs {
    pub start_index: u32,
    pub no_merge: bool,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateValidatorListBalance {
    pub accounts: Option<UpdateValidatorListBalanceAccounts>,
    pub args: Option<UpdateValidatorListBalanceArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateStakePoolBalanceAccounts {
    pub stake_pool: PubkeyBytes,
    pub withdraw_authority: PubkeyBytes,
    pub validator_list_storage: PubkeyBytes,
    pub reserve_stake: PubkeyBytes,
    pub manager_fee_account: PubkeyBytes,
    pub stake_pool_mint: PubkeyBytes,
    pub token_program: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateStakePoolBalance {
    pub accounts: Option<UpdateStakePoolBalanceAccounts>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct CleanupRemovedValidatorEntriesAccounts {
    pub stake_pool: PubkeyBytes,
    pub validator_list_storage: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct CleanupRemovedValidatorEntries {
    pub accounts: Option<CleanupRemovedValidatorEntriesAccounts>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositStakeAccounts {
    pub stake_pool: PubkeyBytes,
    pub validator_list_storage: PubkeyBytes,
    pub stake_pool_deposit_authority: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub deposit_stake_address: PubkeyBytes,
    pub validator_stake_account: PubkeyBytes,
    pub reserve_stake_account: PubkeyBytes,
    pub pool_tokens_to: PubkeyBytes,
    pub manager_fee_account: PubkeyBytes,
    pub referrer_pool_tokens_account: PubkeyBytes,
    pub pool_mint: PubkeyBytes,
    pub clock: PubkeyBytes,
    pub sysvar_stake_history: PubkeyBytes,
    pub token_program: PubkeyBytes,
    pub stake_program: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositStake {
    pub accounts: Option<DepositStakeAccounts>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeAccounts {
    pub stake_pool: PubkeyBytes,
    pub validator_list_storage: PubkeyBytes,
    pub stake_pool_withdraw: PubkeyBytes,
    pub stake_to_split: PubkeyBytes,
    pub stake_to_receive: PubkeyBytes,
    pub user_stake_authority: PubkeyBytes,
    pub user_transfer_authority: PubkeyBytes,
    pub user_pool_token_account: PubkeyBytes,
    pub manager_fee_account: PubkeyBytes,
    pub pool_mint: PubkeyBytes,
    pub clock: PubkeyBytes,
    pub token_program: PubkeyBytes,
    pub stake_program: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeArgs {
    pub amount: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawStake {
    pub accounts: Option<WithdrawStakeAccounts>,
    pub args: Option<WithdrawStakeArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetManagerAccounts {
    pub stake_pool: PubkeyBytes,
    pub manager: PubkeyBytes,
    pub new_manager: PubkeyBytes,
    pub new_fee_receiver: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetManager {
    pub accounts: Option<SetManagerAccounts>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetFeeAccounts {
    pub stake_pool: PubkeyBytes,
    pub manager: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetFeeArgs {
    pub fee: Option<FeeType>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetFee {
    pub accounts: Option<SetFeeAccounts>,
    pub args: Option<SetFeeArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetStakerAccounts {
    pub stake_pool: PubkeyBytes,
    pub set_staker_authority: PubkeyBytes,
    pub new_staker: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetStaker {
    pub accounts: Option<SetStakerAccounts>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositSolAccounts {
    pub stake_pool: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub reserve_stake_account: PubkeyBytes,
    pub lamports_from: PubkeyBytes,
    pub pool_tokens_to: PubkeyBytes,
    pub manager_fee_account: PubkeyBytes,
    pub referrer_pool_tokens_account: PubkeyBytes,
    pub pool_mint: PubkeyBytes,
    pub system_program: PubkeyBytes,
    pub token_program: PubkeyBytes,
    pub deposit_authority: Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositSolArgs {
    pub amount: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositSol {
    pub accounts: Option<DepositSolAccounts>,
    pub args: Option<DepositSolArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetFundingAuthorityAccounts {
    pub stake_pool: PubkeyBytes,
    pub manager: PubkeyBytes,
    pub auth: Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetFundingAuthorityArgs {
    #[vixen_proto_hint(enumeration = "FundingType")]
    pub funding_type: i32,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetFundingAuthority {
    pub accounts: Option<SetFundingAuthorityAccounts>,
    pub args: Option<SetFundingAuthorityArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolAccounts {
    pub stake_pool: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub user_transfer_authority: PubkeyBytes,
    pub pool_tokens_from: PubkeyBytes,
    pub reserve_stake_account: PubkeyBytes,
    pub lamports_to: PubkeyBytes,
    pub manager_fee_account: PubkeyBytes,
    pub pool_mint: PubkeyBytes,
    pub clock: PubkeyBytes,
    pub sysvar_stake_history: PubkeyBytes,
    pub stake_program: PubkeyBytes,
    pub token_program: PubkeyBytes,
    pub sol_withdraw_authority: Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolArgs {
    pub amount: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawSol {
    pub accounts: Option<WithdrawSolAccounts>,
    pub args: Option<WithdrawSolArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct CreateTokenMetadataAccounts {
    pub stake_pool: PubkeyBytes,
    pub manager: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub pool_mint: PubkeyBytes,
    pub payer: PubkeyBytes,
    pub token_metadata: PubkeyBytes,
    pub mpl_token_metadata: PubkeyBytes,
    pub system_program: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct CreateTokenMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct CreateTokenMetadata {
    pub accounts: Option<CreateTokenMetadataAccounts>,
    pub args: Option<CreateTokenMetadataArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateTokenMetadataAccounts {
    pub stake_pool: PubkeyBytes,
    pub manager: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub token_metadata: PubkeyBytes,
    pub mpl_token_metadata: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateTokenMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateTokenMetadata {
    pub accounts: Option<UpdateTokenMetadataAccounts>,
    pub args: Option<UpdateTokenMetadataArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct IncreaseAdditionalValidatorStakeAccounts {
    pub stake_pool: PubkeyBytes,
    pub staker: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub validator_list: PubkeyBytes,
    pub reserve_stake: PubkeyBytes,
    pub ephemeral_stake: PubkeyBytes,
    pub transient_stake: PubkeyBytes,
    pub validator_stake: PubkeyBytes,
    pub validator: PubkeyBytes,
    pub clock: PubkeyBytes,
    pub stake_history: PubkeyBytes,
    pub stake_config: PubkeyBytes,
    pub system_program: PubkeyBytes,
    pub stake_program: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct IncreaseAdditionalValidatorStakeArgs {
    pub lamports: u64,
    pub transient_stake_seed: u64,
    pub ephemeral_stake_seed: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct IncreaseAdditionalValidatorStake {
    pub accounts: Option<IncreaseAdditionalValidatorStakeAccounts>,
    pub args: Option<IncreaseAdditionalValidatorStakeArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseAdditionalValidatorStakeAccounts {
    pub stake_pool: PubkeyBytes,
    pub staker: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub validator_list: PubkeyBytes,
    pub reserve_stake: PubkeyBytes,
    pub validator_stake: PubkeyBytes,
    pub ephemeral_stake: PubkeyBytes,
    pub transient_stake: PubkeyBytes,
    pub clock: PubkeyBytes,
    pub stake_history: PubkeyBytes,
    pub system_program: PubkeyBytes,
    pub stake_program: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseAdditionalValidatorStakeArgs {
    pub lamports: u64,
    pub transient_stake_seed: u64,
    pub ephemeral_stake_seed: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseAdditionalValidatorStake {
    pub accounts: Option<DecreaseAdditionalValidatorStakeAccounts>,
    pub args: Option<DecreaseAdditionalValidatorStakeArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeWithReserveAccounts {
    pub stake_pool: PubkeyBytes,
    pub staker: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub validator_list: PubkeyBytes,
    pub reserve_stake: PubkeyBytes,
    pub validator_stake: PubkeyBytes,
    pub transient_stake: PubkeyBytes,
    pub clock: PubkeyBytes,
    pub stake_history: PubkeyBytes,
    pub system_program: PubkeyBytes,
    pub stake_program: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeWithReserveArgs {
    pub lamports: u64,
    pub transient_stake_seed: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeWithReserve {
    pub accounts: Option<DecreaseValidatorStakeWithReserveAccounts>,
    pub args: Option<DecreaseValidatorStakeWithReserveArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositStakeWithSlippageAccounts {
    pub stake_pool: PubkeyBytes,
    pub validator_list_storage: PubkeyBytes,
    pub stake_pool_deposit_authority: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub deposit_stake_address: PubkeyBytes,
    pub validator_stake_account: PubkeyBytes,
    pub reserve_stake_account: PubkeyBytes,
    pub pool_tokens_to: PubkeyBytes,
    pub manager_fee_account: PubkeyBytes,
    pub referrer_pool_tokens_account: PubkeyBytes,
    pub pool_mint: PubkeyBytes,
    pub clock: PubkeyBytes,
    pub sysvar_stake_history: PubkeyBytes,
    pub token_program: PubkeyBytes,
    pub stake_program: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositStakeWithSlippageArgs {
    pub minimum_pool_tokens_out: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositStakeWithSlippage {
    pub accounts: Option<DepositStakeWithSlippageAccounts>,
    pub args: Option<DepositStakeWithSlippageArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeWithSlippageAccounts {
    pub stake_pool: PubkeyBytes,
    pub validator_list_storage: PubkeyBytes,
    pub stake_pool_withdraw: PubkeyBytes,
    pub stake_to_split: PubkeyBytes,
    pub stake_to_receive: PubkeyBytes,
    pub user_stake_authority: PubkeyBytes,
    pub user_transfer_authority: PubkeyBytes,
    pub user_pool_token_account: PubkeyBytes,
    pub manager_fee_account: PubkeyBytes,
    pub pool_mint: PubkeyBytes,
    pub clock: PubkeyBytes,
    pub token_program: PubkeyBytes,
    pub stake_program: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeWithSlippageArgs {
    pub pool_tokens_in: u64,
    pub minimum_lamports_out: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeWithSlippage {
    pub accounts: Option<WithdrawStakeWithSlippageAccounts>,
    pub args: Option<WithdrawStakeWithSlippageArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositSolWithSlippageAccounts {
    pub stake_pool: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub reserve_stake_account: PubkeyBytes,
    pub lamports_from: PubkeyBytes,
    pub pool_tokens_to: PubkeyBytes,
    pub manager_fee_account: PubkeyBytes,
    pub referrer_pool_tokens_account: PubkeyBytes,
    pub pool_mint: PubkeyBytes,
    pub system_program: PubkeyBytes,
    pub token_program: PubkeyBytes,
    pub deposit_authority: Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositSolWithSlippageArgs {
    pub lamports_in: u64,
    pub minimum_pool_tokens_out: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositSolWithSlippage {
    pub accounts: Option<DepositSolWithSlippageAccounts>,
    pub args: Option<DepositSolWithSlippageArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolWithSlippageAccounts {
    pub stake_pool: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub user_transfer_authority: PubkeyBytes,
    pub pool_tokens_from: PubkeyBytes,
    pub reserve_stake_account: PubkeyBytes,
    pub lamports_to: PubkeyBytes,
    pub manager_fee_account: PubkeyBytes,
    pub pool_mint: PubkeyBytes,
    pub clock: PubkeyBytes,
    pub sysvar_stake_history: PubkeyBytes,
    pub stake_program: PubkeyBytes,
    pub token_program: PubkeyBytes,
    pub sol_withdraw_authority: Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolWithSlippageArgs {
    pub pool_tokens_in: u64,
    pub minimum_lamports_out: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolWithSlippage {
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
