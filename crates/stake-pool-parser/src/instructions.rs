use yellowstone_vixen_proc_macro::vixen_proto;

use crate::PubkeyBytes;

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct StakePoolProgramInstructionProto {
    #[vixen_proto_hint(
        oneof = "stake_pool_program_instruction_proto::Instruction",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, \
                23, 24, 25, 26"
    )]
    pub instruction: Option<stake_pool_program_instruction_proto::Instruction>,
}

pub mod stake_pool_program_instruction_proto {
    use super::vixen_proto;

    #[vixen_proto(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Instruction {
        Initialize(super::InitializeIxProto),
        AddValidatorToPool(super::AddValidatorToPoolIxProto),
        RemoveValidatorFromPool(super::RemoveValidatorFromPoolIxProto),
        DecreaseValidatorStake(super::DecreaseValidatorStakeIxProto),
        IncreaseValidatorStake(super::IncreaseValidatorStakeIxProto),
        SetPreferredValidator(super::SetPreferredValidatorIxProto),
        UpdateValidatorListBalance(super::UpdateValidatorListBalanceIxProto),
        UpdateStakePoolBalance(super::UpdateStakePoolBalanceIxProto),
        CleanupRemovedValidatorEntries(super::CleanupRemovedValidatorEntriesIxProto),
        DepositStake(super::DepositStakeIxProto),
        WithdrawStake(super::WithdrawStakeIxProto),
        SetManager(super::SetManagerIxProto),
        SetFee(super::SetFeeIxProto),
        SetStaker(super::SetStakerIxProto),
        DepositSol(super::DepositSolIxProto),
        SetFundingAuthority(super::SetFundingAuthorityIxProto),
        WithdrawSol(super::WithdrawSolIxProto),
        CreateTokenMetadata(super::CreateTokenMetadataIxProto),
        UpdateTokenMetadata(super::UpdateTokenMetadataIxProto),
        IncreaseAdditionalValidatorStake(super::IncreaseAdditionalValidatorStakeIxProto),
        DecreaseAdditionalValidatorStake(super::DecreaseAdditionalValidatorStakeIxProto),
        DecreaseValidatorStakeWithReserve(super::DecreaseValidatorStakeWithReserveIxProto),
        DepositStakeWithSlippage(super::DepositStakeWithSlippageIxProto),
        WithdrawStakeWithSlippage(super::WithdrawStakeWithSlippageIxProto),
        DepositSolWithSlippage(super::DepositSolWithSlippageIxProto),
        WithdrawSolWithSlippage(super::WithdrawSolWithSlippageIxProto),
    }
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct FeeProto {
    pub numerator: u64,
    pub denominator: u64,
}

#[vixen_proto(enumeration)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum PreferredValidatorTypeProto {
    Deposit = 0,
    Withdraw = 1,
}

#[vixen_proto(enumeration)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum FundingTypeProto {
    SolDeposit = 0,
    StakeDeposit = 1,
    SolWithdraw = 2,
}

#[vixen_proto(enumeration)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum FeeKindProto {
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
pub struct FeeTypeProto {
    #[vixen_proto_hint(enumeration = "FeeKindProto")]
    pub kind: i32,

    #[vixen_proto_hint(oneof = "fee_type_proto::Value", tags = "2, 3")]
    pub value: Option<fee_type_proto::Value>,
}

pub mod fee_type_proto {
    use super::vixen_proto;

    #[vixen_proto(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Value {
        Fee(super::FeeProto),

        ReferralBps(u32),
    }
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeAccountsProto {
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
pub struct InitializeArgsProto {
    pub fee: Option<FeeProto>,
    pub withdrawal_fee: Option<FeeProto>,
    pub deposit_fee: Option<FeeProto>,
    pub referral_fee: u32,
    pub max_validators: u32,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeIxProto {
    pub accounts: Option<InitializeAccountsProto>,
    pub args: Option<InitializeArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct AddValidatorToPoolAccountsProto {
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
pub struct AddValidatorToPoolArgsProto {
    pub raw_validator_seed: u32,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct AddValidatorToPoolIxProto {
    pub accounts: Option<AddValidatorToPoolAccountsProto>,
    pub args: Option<AddValidatorToPoolArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct RemoveValidatorFromPoolAccountsProto {
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
pub struct RemoveValidatorFromPoolIxProto {
    pub accounts: Option<RemoveValidatorFromPoolAccountsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeAccountsProto {
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
pub struct DecreaseValidatorStakeArgsProto {
    pub lamports: u64,
    pub transient_stake_seed: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeIxProto {
    pub accounts: Option<DecreaseValidatorStakeAccountsProto>,
    pub args: Option<DecreaseValidatorStakeArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct IncreaseValidatorStakeAccountsProto {
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
pub struct IncreaseValidatorStakeArgsProto {
    pub lamports: u64,
    pub transient_stake_seed: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct IncreaseValidatorStakeIxProto {
    pub accounts: Option<IncreaseValidatorStakeAccountsProto>,
    pub args: Option<IncreaseValidatorStakeArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetPreferredValidatorAccountsProto {
    pub stake_pool_address: PubkeyBytes,
    pub staker: PubkeyBytes,
    pub validator_list_address: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetPreferredValidatorArgsProto {
    #[vixen_proto_hint(enumeration = "PreferredValidatorTypeProto")]
    pub validator_type: i32,
    pub validator_vote_address: Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetPreferredValidatorIxProto {
    pub accounts: Option<SetPreferredValidatorAccountsProto>,
    pub args: Option<SetPreferredValidatorArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateValidatorListBalanceAccountsProto {
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
pub struct UpdateValidatorListBalanceArgsProto {
    pub start_index: u32,
    pub no_merge: bool,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateValidatorListBalanceIxProto {
    pub accounts: Option<UpdateValidatorListBalanceAccountsProto>,
    pub args: Option<UpdateValidatorListBalanceArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateStakePoolBalanceAccountsProto {
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
pub struct UpdateStakePoolBalanceIxProto {
    pub accounts: Option<UpdateStakePoolBalanceAccountsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct CleanupRemovedValidatorEntriesAccountsProto {
    pub stake_pool: PubkeyBytes,
    pub validator_list_storage: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct CleanupRemovedValidatorEntriesIxProto {
    pub accounts: Option<CleanupRemovedValidatorEntriesAccountsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositStakeAccountsProto {
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
pub struct DepositStakeIxProto {
    pub accounts: Option<DepositStakeAccountsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeAccountsProto {
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
pub struct WithdrawStakeArgsProto {
    pub amount: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeIxProto {
    pub accounts: Option<WithdrawStakeAccountsProto>,
    pub args: Option<WithdrawStakeArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetManagerAccountsProto {
    pub stake_pool: PubkeyBytes,
    pub manager: PubkeyBytes,
    pub new_manager: PubkeyBytes,
    pub new_fee_receiver: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetManagerIxProto {
    pub accounts: Option<SetManagerAccountsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetFeeAccountsProto {
    pub stake_pool: PubkeyBytes,
    pub manager: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetFeeArgsProto {
    pub fee: Option<FeeTypeProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetFeeIxProto {
    pub accounts: Option<SetFeeAccountsProto>,
    pub args: Option<SetFeeArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetStakerAccountsProto {
    pub stake_pool: PubkeyBytes,
    pub set_staker_authority: PubkeyBytes,
    pub new_staker: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetStakerIxProto {
    pub accounts: Option<SetStakerAccountsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositSolAccountsProto {
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
pub struct DepositSolArgsProto {
    pub amount: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositSolIxProto {
    pub accounts: Option<DepositSolAccountsProto>,
    pub args: Option<DepositSolArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetFundingAuthorityAccountsProto {
    pub stake_pool: PubkeyBytes,
    pub manager: PubkeyBytes,
    pub auth: Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetFundingAuthorityArgsProto {
    #[vixen_proto_hint(enumeration = "FundingTypeProto")]
    pub funding_type: i32,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetFundingAuthorityIxProto {
    pub accounts: Option<SetFundingAuthorityAccountsProto>,
    pub args: Option<SetFundingAuthorityArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolAccountsProto {
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
pub struct WithdrawSolArgsProto {
    pub amount: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolIxProto {
    pub accounts: Option<WithdrawSolAccountsProto>,
    pub args: Option<WithdrawSolArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct CreateTokenMetadataAccountsProto {
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
pub struct CreateTokenMetadataArgsProto {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct CreateTokenMetadataIxProto {
    pub accounts: Option<CreateTokenMetadataAccountsProto>,
    pub args: Option<CreateTokenMetadataArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateTokenMetadataAccountsProto {
    pub stake_pool: PubkeyBytes,
    pub manager: PubkeyBytes,
    pub stake_pool_withdraw_authority: PubkeyBytes,
    pub token_metadata: PubkeyBytes,
    pub mpl_token_metadata: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateTokenMetadataArgsProto {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateTokenMetadataIxProto {
    pub accounts: Option<UpdateTokenMetadataAccountsProto>,
    pub args: Option<UpdateTokenMetadataArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct IncreaseAdditionalValidatorStakeAccountsProto {
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
pub struct IncreaseAdditionalValidatorStakeArgsProto {
    pub lamports: u64,
    pub transient_stake_seed: u64,
    pub ephemeral_stake_seed: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct IncreaseAdditionalValidatorStakeIxProto {
    pub accounts: Option<IncreaseAdditionalValidatorStakeAccountsProto>,
    pub args: Option<IncreaseAdditionalValidatorStakeArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseAdditionalValidatorStakeAccountsProto {
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
pub struct DecreaseAdditionalValidatorStakeArgsProto {
    pub lamports: u64,
    pub transient_stake_seed: u64,
    pub ephemeral_stake_seed: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseAdditionalValidatorStakeIxProto {
    pub accounts: Option<DecreaseAdditionalValidatorStakeAccountsProto>,
    pub args: Option<DecreaseAdditionalValidatorStakeArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeWithReserveAccountsProto {
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
pub struct DecreaseValidatorStakeWithReserveArgsProto {
    pub lamports: u64,
    pub transient_stake_seed: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DecreaseValidatorStakeWithReserveIxProto {
    pub accounts: Option<DecreaseValidatorStakeWithReserveAccountsProto>,
    pub args: Option<DecreaseValidatorStakeWithReserveArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositStakeWithSlippageAccountsProto {
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
pub struct DepositStakeWithSlippageArgsProto {
    pub minimum_pool_tokens_out: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositStakeWithSlippageIxProto {
    pub accounts: Option<DepositStakeWithSlippageAccountsProto>,
    pub args: Option<DepositStakeWithSlippageArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeWithSlippageAccountsProto {
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
pub struct WithdrawStakeWithSlippageArgsProto {
    pub pool_tokens_in: u64,
    pub minimum_lamports_out: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawStakeWithSlippageIxProto {
    pub accounts: Option<WithdrawStakeWithSlippageAccountsProto>,
    pub args: Option<WithdrawStakeWithSlippageArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositSolWithSlippageAccountsProto {
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
pub struct DepositSolWithSlippageArgsProto {
    pub lamports_in: u64,
    pub minimum_pool_tokens_out: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DepositSolWithSlippageIxProto {
    pub accounts: Option<DepositSolWithSlippageAccountsProto>,
    pub args: Option<DepositSolWithSlippageArgsProto>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolWithSlippageAccountsProto {
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
pub struct WithdrawSolWithSlippageArgsProto {
    pub pool_tokens_in: u64,
    pub minimum_lamports_out: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawSolWithSlippageIxProto {
    pub accounts: Option<WithdrawSolWithSlippageAccountsProto>,
    pub args: Option<WithdrawSolWithSlippageArgsProto>,
}

// ── Conversion helpers ──────────────────────────────────────────────

pub(crate) fn fee_to_proto(fee: spl_stake_pool::state::Fee) -> FeeProto {
    FeeProto {
        numerator: fee.numerator,
        denominator: fee.denominator,
    }
}

pub(crate) fn preferred_validator_type_to_proto(
    v: spl_stake_pool::instruction::PreferredValidatorType,
) -> PreferredValidatorTypeProto {
    use spl_stake_pool::instruction::PreferredValidatorType as PVT;

    match v {
        PVT::Deposit => PreferredValidatorTypeProto::Deposit,
        PVT::Withdraw => PreferredValidatorTypeProto::Withdraw,
    }
}

pub(crate) fn funding_type_to_proto(
    v: spl_stake_pool::instruction::FundingType,
) -> FundingTypeProto {
    use spl_stake_pool::instruction::FundingType as FT;

    match v {
        FT::SolDeposit => FundingTypeProto::SolDeposit,
        FT::StakeDeposit => FundingTypeProto::StakeDeposit,
        FT::SolWithdraw => FundingTypeProto::SolWithdraw,
    }
}

/// StakePoolInstruction::SetFee carries a `FeeType` (not a `Fee`).
/// `FeeType` is an enum whose variants carry the new `Fee` value.
/// We normalize it into `{ kind, fee }`.
pub(crate) fn fee_type_to_proto(v: spl_stake_pool::state::FeeType) -> FeeTypeProto {
    use fee_type_proto::Value;
    use spl_stake_pool::state::FeeType as FT;

    match v {
        FT::SolDeposit(fee) => FeeTypeProto {
            kind: FeeKindProto::SolDeposit as i32,
            value: Some(Value::Fee(fee_to_proto(fee))),
        },
        FT::StakeDeposit(fee) => FeeTypeProto {
            kind: FeeKindProto::StakeDeposit as i32,
            value: Some(Value::Fee(fee_to_proto(fee))),
        },
        FT::SolWithdrawal(fee) => FeeTypeProto {
            kind: FeeKindProto::SolWithdrawal as i32,
            value: Some(Value::Fee(fee_to_proto(fee))),
        },
        FT::StakeWithdrawal(fee) => FeeTypeProto {
            kind: FeeKindProto::StakeWithdrawal as i32,
            value: Some(Value::Fee(fee_to_proto(fee))),
        },
        FT::Epoch(fee) => FeeTypeProto {
            kind: FeeKindProto::Epoch as i32,
            value: Some(Value::Fee(fee_to_proto(fee))),
        },
        FT::SolReferral(bps) => FeeTypeProto {
            kind: FeeKindProto::SolReferral as i32,
            value: Some(Value::ReferralBps(bps as u32)),
        },
        FT::StakeReferral(bps) => FeeTypeProto {
            kind: FeeKindProto::StakeReferral as i32,
            value: Some(Value::ReferralBps(bps as u32)),
        },
    }
}
