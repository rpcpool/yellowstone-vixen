use crate::PubkeyBytes;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakePoolProgramInstructionProto {
    #[prost(
        oneof = "stake_pool_program_instruction_proto::Instruction",
        tags = "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26"
    )]
    pub instruction: Option<stake_pool_program_instruction_proto::Instruction>,
}

pub mod stake_pool_program_instruction_proto {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Instruction {
        #[prost(message, tag = "1")]
        Initialize(super::InitializeIxProto),
        #[prost(message, tag = "2")]
        AddValidatorToPool(super::AddValidatorToPoolIxProto),
        #[prost(message, tag = "3")]
        RemoveValidatorFromPool(super::RemoveValidatorFromPoolIxProto),
        #[prost(message, tag = "4")]
        DecreaseValidatorStake(super::DecreaseValidatorStakeIxProto),
        #[prost(message, tag = "5")]
        IncreaseValidatorStake(super::IncreaseValidatorStakeIxProto),
        #[prost(message, tag = "6")]
        SetPreferredValidator(super::SetPreferredValidatorIxProto),
        #[prost(message, tag = "7")]
        UpdateValidatorListBalance(super::UpdateValidatorListBalanceIxProto),
        #[prost(message, tag = "8")]
        UpdateStakePoolBalance(super::UpdateStakePoolBalanceIxProto),
        #[prost(message, tag = "9")]
        CleanupRemovedValidatorEntries(super::CleanupRemovedValidatorEntriesIxProto),
        #[prost(message, tag = "10")]
        DepositStake(super::DepositStakeIxProto),
        #[prost(message, tag = "11")]
        WithdrawStake(super::WithdrawStakeIxProto),
        #[prost(message, tag = "12")]
        SetManager(super::SetManagerIxProto),
        #[prost(message, tag = "13")]
        SetFee(super::SetFeeIxProto),
        #[prost(message, tag = "14")]
        SetStaker(super::SetStakerIxProto),
        #[prost(message, tag = "15")]
        DepositSol(super::DepositSolIxProto),
        #[prost(message, tag = "16")]
        SetFundingAuthority(super::SetFundingAuthorityIxProto),
        #[prost(message, tag = "17")]
        WithdrawSol(super::WithdrawSolIxProto),
        #[prost(message, tag = "18")]
        CreateTokenMetadata(super::CreateTokenMetadataIxProto),
        #[prost(message, tag = "19")]
        UpdateTokenMetadata(super::UpdateTokenMetadataIxProto),
        #[prost(message, tag = "20")]
        IncreaseAdditionalValidatorStake(super::IncreaseAdditionalValidatorStakeIxProto),
        #[prost(message, tag = "21")]
        DecreaseAdditionalValidatorStake(super::DecreaseAdditionalValidatorStakeIxProto),
        #[prost(message, tag = "22")]
        DecreaseValidatorStakeWithReserve(super::DecreaseValidatorStakeWithReserveIxProto),
        #[prost(message, tag = "23")]
        DepositStakeWithSlippage(super::DepositStakeWithSlippageIxProto),
        #[prost(message, tag = "24")]
        WithdrawStakeWithSlippage(super::WithdrawStakeWithSlippageIxProto),
        #[prost(message, tag = "25")]
        DepositSolWithSlippage(super::DepositSolWithSlippageIxProto),
        #[prost(message, tag = "26")]
        WithdrawSolWithSlippage(super::WithdrawSolWithSlippageIxProto),
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeProto {
    #[prost(uint64, tag = "1")]
    pub numerator: u64,
    #[prost(uint64, tag = "2")]
    pub denominator: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PreferredValidatorTypeProto {
    Deposit = 0,
    Withdraw = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FundingTypeProto {
    SolDeposit = 0,
    StakeDeposit = 1,
    SolWithdraw = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeTypeProto {
    #[prost(enumeration = "FeeKindProto", tag = "1")]
    pub kind: i32,

    #[prost(oneof = "fee_type_proto::Value", tags = "2, 3")]
    pub value: Option<fee_type_proto::Value>,
}

pub mod fee_type_proto {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "2")]
        Fee(super::FeeProto),

        #[prost(uint32, tag = "3")]
        ReferralBps(u32),
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub manager: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub staker: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub validator_list: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub reserve_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub pool_mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub manager_pool_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub token_program: PubkeyBytes,
    #[prost(bytes = "vec", optional, tag = "10")]
    pub deposit_authority: Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeArgsProto {
    #[prost(message, tag = "1")]
    pub fee: Option<FeeProto>,
    #[prost(message, tag = "2")]
    pub withdrawal_fee: Option<FeeProto>,
    #[prost(message, tag = "3")]
    pub deposit_fee: Option<FeeProto>,
    #[prost(uint32, tag = "4")]
    pub referral_fee: u32,
    #[prost(uint32, tag = "5")]
    pub max_validators: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<InitializeAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<InitializeArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddValidatorToPoolAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub staker: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub funder: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub stake_pool_withdraw: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub validator_list: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub validator: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub rent: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub clock: PubkeyBytes,
    #[prost(bytes = "vec", tag = "10")]
    pub sysvar_stake_history: PubkeyBytes,
    #[prost(bytes = "vec", tag = "11")]
    pub stake_config: PubkeyBytes,
    #[prost(bytes = "vec", tag = "12")]
    pub system_program: PubkeyBytes,
    #[prost(bytes = "vec", tag = "13")]
    pub stake_program: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddValidatorToPoolArgsProto {
    #[prost(uint32, tag = "1")]
    pub raw_validator_seed: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddValidatorToPoolIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<AddValidatorToPoolAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<AddValidatorToPoolArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveValidatorFromPoolAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub staker: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub stake_pool_withdraw: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub validator_list: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub stake_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub transient_stake_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub clock: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub stake_program: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveValidatorFromPoolIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<RemoveValidatorFromPoolAccountsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecreaseValidatorStakeAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub staker: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub validator_list: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub validator_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub transient_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub clock: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub rent: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub system_program: PubkeyBytes,
    #[prost(bytes = "vec", tag = "10")]
    pub stake_program: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecreaseValidatorStakeArgsProto {
    #[prost(uint64, tag = "1")]
    pub lamports: u64,
    #[prost(uint64, tag = "2")]
    pub transient_stake_seed: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecreaseValidatorStakeIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<DecreaseValidatorStakeAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<DecreaseValidatorStakeArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncreaseValidatorStakeAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub staker: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub validator_list: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub reserve_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub transient_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub validator_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub validator: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub clock: PubkeyBytes,
    #[prost(bytes = "vec", tag = "10")]
    pub rent: PubkeyBytes,
    #[prost(bytes = "vec", tag = "11")]
    pub sysvar_stake_history: PubkeyBytes,
    #[prost(bytes = "vec", tag = "12")]
    pub stake_config: PubkeyBytes,
    #[prost(bytes = "vec", tag = "13")]
    pub system_program: PubkeyBytes,
    #[prost(bytes = "vec", tag = "14")]
    pub stake_program: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncreaseValidatorStakeArgsProto {
    #[prost(uint64, tag = "1")]
    pub lamports: u64,
    #[prost(uint64, tag = "2")]
    pub transient_stake_seed: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncreaseValidatorStakeIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<IncreaseValidatorStakeAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<IncreaseValidatorStakeArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPreferredValidatorAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool_address: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub staker: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub validator_list_address: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPreferredValidatorArgsProto {
    #[prost(enumeration = "PreferredValidatorTypeProto", tag = "1")]
    pub validator_type: i32,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub validator_vote_address: Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPreferredValidatorIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<SetPreferredValidatorAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<SetPreferredValidatorArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateValidatorListBalanceAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub validator_list_address: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub reserve_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub clock: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub sysvar_stake_history: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub stake_program: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateValidatorListBalanceArgsProto {
    #[prost(uint32, tag = "1")]
    pub start_index: u32,
    #[prost(bool, tag = "2")]
    pub no_merge: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateValidatorListBalanceIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<UpdateValidatorListBalanceAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<UpdateValidatorListBalanceArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStakePoolBalanceAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub validator_list_storage: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub reserve_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub manager_fee_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub stake_pool_mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub token_program: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStakePoolBalanceIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<UpdateStakePoolBalanceAccountsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CleanupRemovedValidatorEntriesAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub validator_list_storage: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CleanupRemovedValidatorEntriesIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<CleanupRemovedValidatorEntriesAccountsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositStakeAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub validator_list_storage: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub stake_pool_deposit_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub deposit_stake_address: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub validator_stake_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub reserve_stake_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub pool_tokens_to: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub manager_fee_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "10")]
    pub referrer_pool_tokens_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "11")]
    pub pool_mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "12")]
    pub clock: PubkeyBytes,
    #[prost(bytes = "vec", tag = "13")]
    pub sysvar_stake_history: PubkeyBytes,
    #[prost(bytes = "vec", tag = "14")]
    pub token_program: PubkeyBytes,
    #[prost(bytes = "vec", tag = "15")]
    pub stake_program: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositStakeIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<DepositStakeAccountsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawStakeAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub validator_list_storage: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub stake_pool_withdraw: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub stake_to_split: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub stake_to_receive: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub user_stake_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub user_transfer_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub user_pool_token_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub manager_fee_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "10")]
    pub pool_mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "11")]
    pub clock: PubkeyBytes,
    #[prost(bytes = "vec", tag = "12")]
    pub token_program: PubkeyBytes,
    #[prost(bytes = "vec", tag = "13")]
    pub stake_program: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawStakeArgsProto {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawStakeIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<WithdrawStakeAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<WithdrawStakeArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetManagerAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub manager: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub new_manager: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub new_fee_receiver: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetManagerIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<SetManagerAccountsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFeeAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub manager: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFeeArgsProto {
    #[prost(message, tag = "1")]
    pub fee: Option<FeeTypeProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFeeIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<SetFeeAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<SetFeeArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetStakerAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub set_staker_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub new_staker: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetStakerIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<SetStakerAccountsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositSolAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub reserve_stake_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub lamports_from: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub pool_tokens_to: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub manager_fee_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub referrer_pool_tokens_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub pool_mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub system_program: PubkeyBytes,
    #[prost(bytes = "vec", tag = "10")]
    pub token_program: PubkeyBytes,
    #[prost(bytes = "vec", optional, tag = "11")]
    pub deposit_authority: Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositSolArgsProto {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositSolIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<DepositSolAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<DepositSolArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFundingAuthorityAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub manager: PubkeyBytes,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub auth: Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFundingAuthorityArgsProto {
    #[prost(enumeration = "FundingTypeProto", tag = "1")]
    pub funding_type: i32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFundingAuthorityIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<SetFundingAuthorityAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<SetFundingAuthorityArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawSolAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub user_transfer_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub pool_tokens_from: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub reserve_stake_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub lamports_to: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub manager_fee_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub pool_mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub clock: PubkeyBytes,
    #[prost(bytes = "vec", tag = "10")]
    pub sysvar_stake_history: PubkeyBytes,
    #[prost(bytes = "vec", tag = "11")]
    pub stake_program: PubkeyBytes,
    #[prost(bytes = "vec", tag = "12")]
    pub token_program: PubkeyBytes,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub sol_withdraw_authority: Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawSolArgsProto {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawSolIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<WithdrawSolAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<WithdrawSolArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTokenMetadataAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub manager: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub pool_mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub payer: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub token_metadata: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub mpl_token_metadata: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub system_program: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTokenMetadataArgsProto {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTokenMetadataIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<CreateTokenMetadataAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<CreateTokenMetadataArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTokenMetadataAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub manager: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub token_metadata: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub mpl_token_metadata: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTokenMetadataArgsProto {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTokenMetadataIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<UpdateTokenMetadataAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<UpdateTokenMetadataArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncreaseAdditionalValidatorStakeAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub staker: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub validator_list: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub reserve_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub ephemeral_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub transient_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub validator_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub validator: PubkeyBytes,
    #[prost(bytes = "vec", tag = "10")]
    pub clock: PubkeyBytes,
    #[prost(bytes = "vec", tag = "11")]
    pub stake_history: PubkeyBytes,
    #[prost(bytes = "vec", tag = "12")]
    pub stake_config: PubkeyBytes,
    #[prost(bytes = "vec", tag = "13")]
    pub system_program: PubkeyBytes,
    #[prost(bytes = "vec", tag = "14")]
    pub stake_program: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncreaseAdditionalValidatorStakeArgsProto {
    #[prost(uint64, tag = "1")]
    pub lamports: u64,
    #[prost(uint64, tag = "2")]
    pub transient_stake_seed: u64,
    #[prost(uint64, tag = "3")]
    pub ephemeral_stake_seed: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncreaseAdditionalValidatorStakeIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<IncreaseAdditionalValidatorStakeAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<IncreaseAdditionalValidatorStakeArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecreaseAdditionalValidatorStakeAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub staker: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub validator_list: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub reserve_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub validator_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub ephemeral_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub transient_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub clock: PubkeyBytes,
    #[prost(bytes = "vec", tag = "10")]
    pub stake_history: PubkeyBytes,
    #[prost(bytes = "vec", tag = "11")]
    pub system_program: PubkeyBytes,
    #[prost(bytes = "vec", tag = "12")]
    pub stake_program: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecreaseAdditionalValidatorStakeArgsProto {
    #[prost(uint64, tag = "1")]
    pub lamports: u64,
    #[prost(uint64, tag = "2")]
    pub transient_stake_seed: u64,
    #[prost(uint64, tag = "3")]
    pub ephemeral_stake_seed: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecreaseAdditionalValidatorStakeIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<DecreaseAdditionalValidatorStakeAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<DecreaseAdditionalValidatorStakeArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecreaseValidatorStakeWithReserveAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub staker: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub validator_list: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub reserve_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub validator_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub transient_stake: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub clock: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub stake_history: PubkeyBytes,
    #[prost(bytes = "vec", tag = "10")]
    pub system_program: PubkeyBytes,
    #[prost(bytes = "vec", tag = "11")]
    pub stake_program: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecreaseValidatorStakeWithReserveArgsProto {
    #[prost(uint64, tag = "1")]
    pub lamports: u64,
    #[prost(uint64, tag = "2")]
    pub transient_stake_seed: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecreaseValidatorStakeWithReserveIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<DecreaseValidatorStakeWithReserveAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<DecreaseValidatorStakeWithReserveArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositStakeWithSlippageAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub validator_list_storage: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub stake_pool_deposit_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub deposit_stake_address: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub validator_stake_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub reserve_stake_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub pool_tokens_to: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub manager_fee_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "10")]
    pub referrer_pool_tokens_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "11")]
    pub pool_mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "12")]
    pub clock: PubkeyBytes,
    #[prost(bytes = "vec", tag = "13")]
    pub sysvar_stake_history: PubkeyBytes,
    #[prost(bytes = "vec", tag = "14")]
    pub token_program: PubkeyBytes,
    #[prost(bytes = "vec", tag = "15")]
    pub stake_program: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositStakeWithSlippageArgsProto {
    #[prost(uint64, tag = "1")]
    pub minimum_pool_tokens_out: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositStakeWithSlippageIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<DepositStakeWithSlippageAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<DepositStakeWithSlippageArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawStakeWithSlippageAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub validator_list_storage: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub stake_pool_withdraw: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub stake_to_split: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub stake_to_receive: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub user_stake_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub user_transfer_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub user_pool_token_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub manager_fee_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "10")]
    pub pool_mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "11")]
    pub clock: PubkeyBytes,
    #[prost(bytes = "vec", tag = "12")]
    pub token_program: PubkeyBytes,
    #[prost(bytes = "vec", tag = "13")]
    pub stake_program: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawStakeWithSlippageArgsProto {
    #[prost(uint64, tag = "1")]
    pub pool_tokens_in: u64,
    #[prost(uint64, tag = "2")]
    pub minimum_lamports_out: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawStakeWithSlippageIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<WithdrawStakeWithSlippageAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<WithdrawStakeWithSlippageArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositSolWithSlippageAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub reserve_stake_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub lamports_from: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub pool_tokens_to: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub manager_fee_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub referrer_pool_tokens_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub pool_mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub system_program: PubkeyBytes,
    #[prost(bytes = "vec", tag = "10")]
    pub token_program: PubkeyBytes,
    #[prost(bytes = "vec", optional, tag = "11")]
    pub deposit_authority: Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositSolWithSlippageArgsProto {
    #[prost(uint64, tag = "1")]
    pub lamports_in: u64,
    #[prost(uint64, tag = "2")]
    pub minimum_pool_tokens_out: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositSolWithSlippageIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<DepositSolWithSlippageAccountsProto>,
    #[prost(message, tag = "2")]
    pub args: Option<DepositSolWithSlippageArgsProto>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawSolWithSlippageAccountsProto {
    #[prost(bytes = "vec", tag = "1")]
    pub stake_pool: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub stake_pool_withdraw_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub user_transfer_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub pool_tokens_from: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub reserve_stake_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "6")]
    pub lamports_to: PubkeyBytes,
    #[prost(bytes = "vec", tag = "7")]
    pub manager_fee_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "8")]
    pub pool_mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "9")]
    pub clock: PubkeyBytes,
    #[prost(bytes = "vec", tag = "10")]
    pub sysvar_stake_history: PubkeyBytes,
    #[prost(bytes = "vec", tag = "11")]
    pub stake_program: PubkeyBytes,
    #[prost(bytes = "vec", tag = "12")]
    pub token_program: PubkeyBytes,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub sol_withdraw_authority: Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawSolWithSlippageArgsProto {
    #[prost(uint64, tag = "1")]
    pub pool_tokens_in: u64,
    #[prost(uint64, tag = "2")]
    pub minimum_lamports_out: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawSolWithSlippageIxProto {
    #[prost(message, tag = "1")]
    pub accounts: Option<WithdrawSolWithSlippageAccountsProto>,
    #[prost(message, tag = "2")]
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
