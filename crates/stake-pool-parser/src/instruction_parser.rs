use borsh::BorshDeserialize;
use spl_stake_pool::instruction::StakePoolInstruction;

use super::instruction_helpers::{
    AddValidatorToPoolAccounts, AddValidatorToPoolData, CleanupRemovedValidatorEntriesAccounts,
    CreateTokenMetadataAccounts, CreateTokenMetadataData, DecreaseAdditionalValidatorStakeAccounts,
    DecreaseAdditionalValidatorStakeData, DecreaseValidatorStakeAccounts,
    DecreaseValidatorStakeData, DecreaseValidatorStakeWithReserveAccounts,
    DecreaseValidatorStakeWithReserveData, DepositSolAccounts, DepositSolData,
    DepositSolWithSlippageAccounts, DepositSolWithSlippageData, DepositStakeAccounts,
    DepositStakeWithSlippageAccounts, DepositStakeWithSlippageData,
    IncreaseAdditionalValidatorStakeAccounts, IncreaseAdditionalValidatorStakeData,
    IncreaseValidatorStakeAccounts, IncreaseValidatorStakeData, InitializeAccounts, InitializeData,
    RemoveValidatorFromPoolAccounts, SetFeeAccounts, SetFeeData, SetFundingAuthorityAccounts,
    SetFundingAuthorityData, SetManagerAccounts, SetPreferredValidatorAccounts,
    SetPreferredValidatorData, SetStakerAccounts, StakePoolProgramIx,
    UpdateStakePoolBalanceAccounts, UpdateTokenMetadataAccounts, UpdateTokenMetadataData,
    UpdateValidatorListBalanceAccounts, UpdateValidatorListBalanceData, WithdrawSolAccounts,
    WithdrawSolData, WithdrawSolWithSlippageAccounts, WithdrawSolWithSlippageData,
    WithdrawStakeAccounts, WithdrawStakeData, WithdrawStakeWithSlippageAccounts,
    WithdrawStakeWithSlippageData,
};

#[derive(Debug, Copy, Clone)]
pub struct InstructionParser;

impl yellowstone_vixen_core::Parser for InstructionParser {
    type Input = yellowstone_vixen_core::instruction::InstructionUpdate;
    type Output = StakePoolProgramIx;

    fn id(&self) -> std::borrow::Cow<'static, str> {
        "StakePool::InstructionParser".into()
    }

    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
        yellowstone_vixen_core::Prefilter::builder()
            .transaction_accounts([spl_stake_pool::id()])
            .build()
            .unwrap()
    }

    async fn parse(
        &self,
        ix_update: &yellowstone_vixen_core::instruction::InstructionUpdate,
    ) -> yellowstone_vixen_core::ParseResult<Self::Output> {
        if ix_update.program.equals_ref(spl_stake_pool::id()) {
            InstructionParser::parse_impl(ix_update)
        } else {
            Err(yellowstone_vixen_core::ParseError::Filtered)
        }
    }
}

impl yellowstone_vixen_core::ProgramParser for InstructionParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        spl_stake_pool::id().to_bytes().into()
    }
}

impl InstructionParser {
    pub(crate) fn parse_impl(
        ix: &yellowstone_vixen_core::instruction::InstructionUpdate,
    ) -> yellowstone_vixen_core::ParseResult<StakePoolProgramIx> {
        let ix_type = StakePoolInstruction::try_from_slice(ix.data.as_slice())?;
        let accounts_len = ix.accounts.len();

        match ix_type {
            StakePoolInstruction::Initialize {
                fee,
                withdrawal_fee,
                deposit_fee,
                referral_fee,
                max_validators,
            } => {
                check_min_accounts_req(accounts_len, 9)?;

                let mut ix_accounts = InitializeAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    manager: ix.accounts[1].0.into(),
                    staker: ix.accounts[2].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[3].0.into(),
                    validator_list: ix.accounts[4].0.into(),
                    reserve_stake: ix.accounts[5].0.into(),
                    pool_mint: ix.accounts[6].0.into(),
                    manager_pool_account: ix.accounts[7].0.into(),
                    token_program: ix.accounts[8].0.into(),
                    deposit_authority: None,
                };

                if let Some(deposit_authority) = ix.accounts.get(9) {
                    ix_accounts.deposit_authority = Some(deposit_authority.0.into());
                }

                let de_ix_data: InitializeData = InitializeData {
                    fee,
                    withdrawal_fee,
                    deposit_fee,
                    referral_fee,
                    max_validators,
                };

                Ok(StakePoolProgramIx::Initialize(ix_accounts, de_ix_data))
            },
            StakePoolInstruction::AddValidatorToPool(raw_validator_seed) => {
                check_min_accounts_req(accounts_len, 13)?;

                let ix_accounts = AddValidatorToPoolAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    staker: ix.accounts[1].0.into(),
                    funder: ix.accounts[2].0.into(),
                    stake_pool_withdraw: ix.accounts[3].0.into(),
                    validator_list: ix.accounts[4].0.into(),
                    stake: ix.accounts[5].0.into(),
                    validator: ix.accounts[6].0.into(),
                    rent: ix.accounts[7].0.into(),
                    clock: ix.accounts[8].0.into(),
                    sysvar_stake_history: ix.accounts[9].0.into(),
                    stake_config: ix.accounts[10].0.into(),
                    system_program: ix.accounts[11].0.into(),
                    stake_program: ix.accounts[12].0.into(),
                };

                let de_ix_data: AddValidatorToPoolData =
                    AddValidatorToPoolData { raw_validator_seed };

                Ok(StakePoolProgramIx::AddValidatorToPool(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            StakePoolInstruction::RemoveValidatorFromPool => {
                check_min_accounts_req(accounts_len, 8)?;

                let ix_accounts = RemoveValidatorFromPoolAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    staker: ix.accounts[1].0.into(),
                    stake_pool_withdraw: ix.accounts[2].0.into(),
                    validator_list: ix.accounts[3].0.into(),
                    stake_account: ix.accounts[4].0.into(),
                    transient_stake_account: ix.accounts[5].0.into(),
                    clock: ix.accounts[6].0.into(),
                    stake_program: ix.accounts[7].0.into(),
                };

                Ok(StakePoolProgramIx::RemoveValidatorFromPool(ix_accounts))
            },
            StakePoolInstruction::DecreaseValidatorStake {
                lamports,
                transient_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 10)?;

                let ix_accounts = DecreaseValidatorStakeAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    staker: ix.accounts[1].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[2].0.into(),
                    validator_list: ix.accounts[3].0.into(),
                    validator_stake: ix.accounts[4].0.into(),
                    transient_stake: ix.accounts[5].0.into(),
                    clock: ix.accounts[6].0.into(),
                    rent: ix.accounts[7].0.into(),
                    system_program: ix.accounts[8].0.into(),
                    stake_program: ix.accounts[9].0.into(),
                };

                let de_ix_data: DecreaseValidatorStakeData = DecreaseValidatorStakeData {
                    lamports,
                    transient_stake_seed,
                };

                Ok(StakePoolProgramIx::DecreaseValidatorStake(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            StakePoolInstruction::IncreaseValidatorStake {
                lamports,
                transient_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 14)?;

                let ix_accounts = IncreaseValidatorStakeAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    staker: ix.accounts[1].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[2].0.into(),
                    validator_list: ix.accounts[3].0.into(),
                    reserve_stake: ix.accounts[4].0.into(),
                    transient_stake: ix.accounts[5].0.into(),
                    validator_stake: ix.accounts[6].0.into(),
                    validator: ix.accounts[7].0.into(),
                    clock: ix.accounts[8].0.into(),
                    rent: ix.accounts[9].0.into(),
                    sysvar_stake_history: ix.accounts[10].0.into(),
                    stake_config: ix.accounts[11].0.into(),
                    system_program: ix.accounts[12].0.into(),
                    stake_program: ix.accounts[13].0.into(),
                };

                let de_ix_data: IncreaseValidatorStakeData = IncreaseValidatorStakeData {
                    lamports,
                    transient_stake_seed,
                };

                Ok(StakePoolProgramIx::IncreaseValidatorStake(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            StakePoolInstruction::SetPreferredValidator {
                validator_type,
                validator_vote_address,
            } => {
                check_min_accounts_req(accounts_len, 3)?;

                let ix_accounts = SetPreferredValidatorAccounts {
                    stake_pool_address: ix.accounts[0].0.into(),
                    staker: ix.accounts[1].0.into(),
                    validator_list_address: ix.accounts[2].0.into(),
                };

                let de_ix_data: SetPreferredValidatorData = SetPreferredValidatorData {
                    validator_type,
                    validator_vote_address,
                };

                Ok(StakePoolProgramIx::SetPreferredValidator(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            StakePoolInstruction::UpdateValidatorListBalance {
                start_index,
                no_merge,
            } => {
                check_min_accounts_req(accounts_len, 7)?;

                let ix_accounts = UpdateValidatorListBalanceAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[1].0.into(),
                    validator_list_address: ix.accounts[2].0.into(),
                    reserve_stake: ix.accounts[3].0.into(),
                    clock: ix.accounts[4].0.into(),
                    sysvar_stake_history: ix.accounts[5].0.into(),
                    stake_program: ix.accounts[6].0.into(),
                };

                let de_ix_data: UpdateValidatorListBalanceData = UpdateValidatorListBalanceData {
                    start_index,
                    no_merge,
                };

                Ok(StakePoolProgramIx::UpdateValidatorListBalance(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            StakePoolInstruction::UpdateStakePoolBalance => {
                check_min_accounts_req(accounts_len, 7)?;

                let ix_accounts = UpdateStakePoolBalanceAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    withdraw_authority: ix.accounts[1].0.into(),
                    validator_list_storage: ix.accounts[2].0.into(),
                    reserve_stake: ix.accounts[3].0.into(),
                    manager_fee_account: ix.accounts[4].0.into(),
                    stake_pool_mint: ix.accounts[5].0.into(),
                    token_program: ix.accounts[6].0.into(),
                };

                Ok(StakePoolProgramIx::UpdateStakePoolBalance(ix_accounts))
            },
            StakePoolInstruction::CleanupRemovedValidatorEntries => {
                check_min_accounts_req(accounts_len, 2)?;

                let ix_accounts = CleanupRemovedValidatorEntriesAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    validator_list_storage: ix.accounts[1].0.into(),
                };

                Ok(StakePoolProgramIx::CleanupRemovedValidatorEntries(
                    ix_accounts,
                ))
            },
            StakePoolInstruction::DepositStake => {
                check_min_accounts_req(accounts_len, 15)?;

                let ix_accounts = DepositStakeAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    validator_list_storage: ix.accounts[1].0.into(),
                    stake_pool_deposit_authority: ix.accounts[2].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[3].0.into(),
                    deposit_stake_address: ix.accounts[4].0.into(),
                    validator_stake_account: ix.accounts[5].0.into(),
                    reserve_stake_account: ix.accounts[6].0.into(),
                    pool_tokens_to: ix.accounts[7].0.into(),
                    manager_fee_account: ix.accounts[8].0.into(),
                    referrer_pool_tokens_account: ix.accounts[9].0.into(),
                    pool_mint: ix.accounts[10].0.into(),
                    clock: ix.accounts[11].0.into(),
                    sysvar_stake_history: ix.accounts[12].0.into(),
                    token_program: ix.accounts[13].0.into(),
                    stake_program: ix.accounts[14].0.into(),
                };

                Ok(StakePoolProgramIx::DepositStake(ix_accounts))
            },
            StakePoolInstruction::WithdrawStake(amount) => {
                check_min_accounts_req(accounts_len, 13)?;

                let ix_accounts = WithdrawStakeAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    validator_list_storage: ix.accounts[1].0.into(),
                    stake_pool_withdraw: ix.accounts[2].0.into(),
                    stake_to_split: ix.accounts[3].0.into(),
                    stake_to_receive: ix.accounts[4].0.into(),
                    user_stake_authority: ix.accounts[5].0.into(),
                    user_transfer_authority: ix.accounts[6].0.into(),
                    user_pool_token_account: ix.accounts[7].0.into(),
                    manager_fee_account: ix.accounts[8].0.into(),
                    pool_mint: ix.accounts[9].0.into(),
                    clock: ix.accounts[10].0.into(),
                    token_program: ix.accounts[11].0.into(),
                    stake_program: ix.accounts[12].0.into(),
                };

                let de_ix_data: WithdrawStakeData = WithdrawStakeData { arg: amount };

                Ok(StakePoolProgramIx::WithdrawStake(ix_accounts, de_ix_data))
            },
            StakePoolInstruction::SetManager => {
                check_min_accounts_req(accounts_len, 4)?;

                let ix_accounts = SetManagerAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    manager: ix.accounts[1].0.into(),
                    new_manager: ix.accounts[2].0.into(),
                    new_fee_receiver: ix.accounts[3].0.into(),
                };

                Ok(StakePoolProgramIx::SetManager(ix_accounts))
            },
            StakePoolInstruction::SetFee { fee } => {
                check_min_accounts_req(accounts_len, 2)?;

                let ix_accounts = SetFeeAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    manager: ix.accounts[1].0.into(),
                };

                let de_ix_data: SetFeeData = SetFeeData { fee };

                Ok(StakePoolProgramIx::SetFee(ix_accounts, de_ix_data))
            },
            StakePoolInstruction::SetStaker => {
                check_min_accounts_req(accounts_len, 3)?;

                let ix_accounts = SetStakerAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    set_staker_authority: ix.accounts[1].0.into(),
                    new_staker: ix.accounts[2].0.into(),
                };

                Ok(StakePoolProgramIx::SetStaker(ix_accounts))
            },
            StakePoolInstruction::DepositSol(amount) => {
                check_min_accounts_req(accounts_len, 10)?;

                let ix_accounts = DepositSolAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[1].0.into(),
                    reserve_stake_account: ix.accounts[2].0.into(),
                    lamports_from: ix.accounts[3].0.into(),
                    pool_tokens_to: ix.accounts[4].0.into(),
                    manager_fee_account: ix.accounts[5].0.into(),
                    referrer_pool_tokens_account: ix.accounts[6].0.into(),
                    pool_mint: ix.accounts[7].0.into(),
                    system_program: ix.accounts[8].0.into(),
                    token_program: ix.accounts[9].0.into(),
                    deposit_authority: ix
                        .accounts
                        .get(10)
                        .map(|account| Some(account.0.into()))
                        .unwrap_or(None),
                };

                let de_ix_data: DepositSolData = DepositSolData { arg: amount };

                Ok(StakePoolProgramIx::DepositSol(ix_accounts, de_ix_data))
            },
            StakePoolInstruction::SetFundingAuthority(funding_type) => {
                check_min_accounts_req(accounts_len, 2)?;

                let ix_accounts = SetFundingAuthorityAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    manager: ix.accounts[1].0.into(),
                    auth: ix
                        .accounts
                        .get(12)
                        .map(|account| Some(account.0.into()))
                        .unwrap_or(None),
                };

                let de_ix_data: SetFundingAuthorityData =
                    SetFundingAuthorityData { arg: funding_type };

                Ok(StakePoolProgramIx::SetFundingAuthority(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            StakePoolInstruction::WithdrawSol(amount) => {
                check_min_accounts_req(accounts_len, 12)?;

                let ix_accounts = WithdrawSolAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[1].0.into(),
                    user_transfer_authority: ix.accounts[2].0.into(),
                    pool_tokens_from: ix.accounts[3].0.into(),
                    reserve_stake_account: ix.accounts[4].0.into(),
                    lamports_to: ix.accounts[5].0.into(),
                    manager_fee_account: ix.accounts[6].0.into(),
                    pool_mint: ix.accounts[7].0.into(),
                    clock: ix.accounts[8].0.into(),
                    sysvar_stake_history: ix.accounts[9].0.into(),
                    stake_program: ix.accounts[10].0.into(),
                    token_program: ix.accounts[11].0.into(),
                    sol_withdraw_authority: ix
                        .accounts
                        .get(12)
                        .map(|account| Some(account.0.into()))
                        .unwrap_or(None),
                };

                let de_ix_data: WithdrawSolData = WithdrawSolData { arg: amount };
                Ok(StakePoolProgramIx::WithdrawSol(ix_accounts, de_ix_data))
            },
            StakePoolInstruction::CreateTokenMetadata { name, symbol, uri } => {
                check_min_accounts_req(accounts_len, 9)?;

                let ix_accounts = CreateTokenMetadataAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    manager: ix.accounts[1].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[2].0.into(),
                    pool_mint: ix.accounts[3].0.into(),
                    payer: ix.accounts[4].0.into(),
                    token_metadata: ix.accounts[5].0.into(),
                    mpl_token_metadata: ix.accounts[6].0.into(),
                    system_program: ix.accounts[7].0.into(),
                };

                let de_ix_data: CreateTokenMetadataData =
                    CreateTokenMetadataData { name, symbol, uri };
                Ok(StakePoolProgramIx::CreateTokenMetadata(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            StakePoolInstruction::UpdateTokenMetadata { name, symbol, uri } => {
                check_min_accounts_req(accounts_len, 5)?;

                let ix_accounts = UpdateTokenMetadataAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    manager: ix.accounts[1].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[2].0.into(),
                    token_metadata: ix.accounts[3].0.into(),
                    mpl_token_metadata: ix.accounts[4].0.into(),
                };

                let de_ix_data: UpdateTokenMetadataData =
                    UpdateTokenMetadataData { name, symbol, uri };

                Ok(StakePoolProgramIx::UpdateTokenMetadata(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            StakePoolInstruction::IncreaseAdditionalValidatorStake {
                lamports,
                transient_stake_seed,
                ephemeral_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 14)?;

                let ix_accounts = IncreaseAdditionalValidatorStakeAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    staker: ix.accounts[1].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[2].0.into(),
                    validator_list: ix.accounts[3].0.into(),
                    reserve_stake: ix.accounts[4].0.into(),
                    ephemeral_stake: ix.accounts[5].0.into(),
                    transient_stake: ix.accounts[6].0.into(),
                    validator_stake: ix.accounts[7].0.into(),
                    validator: ix.accounts[8].0.into(),
                    clock: ix.accounts[9].0.into(),
                    stake_history: ix.accounts[10].0.into(),
                    stake_config: ix.accounts[11].0.into(),
                    system_program: ix.accounts[12].0.into(),
                    stake_program: ix.accounts[13].0.into(),
                };

                let de_ix_data: IncreaseAdditionalValidatorStakeData =
                    IncreaseAdditionalValidatorStakeData {
                        lamports,
                        transient_stake_seed,
                        ephemeral_stake_seed,
                    };

                Ok(StakePoolProgramIx::IncreaseAdditionalValidatorStake(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            StakePoolInstruction::DecreaseAdditionalValidatorStake {
                lamports,
                transient_stake_seed,
                ephemeral_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 12)?;

                let ix_accounts = DecreaseAdditionalValidatorStakeAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    staker: ix.accounts[1].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[2].0.into(),
                    validator_list: ix.accounts[3].0.into(),
                    reserve_stake: ix.accounts[4].0.into(),
                    validator_stake: ix.accounts[5].0.into(),
                    ephemeral_stake: ix.accounts[6].0.into(),
                    transient_stake: ix.accounts[7].0.into(),
                    clock: ix.accounts[8].0.into(),
                    stake_history: ix.accounts[9].0.into(),
                    system_program: ix.accounts[10].0.into(),
                    stake_program: ix.accounts[11].0.into(),
                };

                let de_ix_data = DecreaseAdditionalValidatorStakeData {
                    lamports,
                    transient_stake_seed,
                    ephemeral_stake_seed,
                };

                Ok(StakePoolProgramIx::DecreaseAdditionalValidatorStake(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            StakePoolInstruction::DecreaseValidatorStakeWithReserve {
                lamports,
                transient_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 11)?;

                let ix_accounts = DecreaseValidatorStakeWithReserveAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    staker: ix.accounts[1].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[2].0.into(),
                    validator_list: ix.accounts[3].0.into(),
                    reserve_stake: ix.accounts[4].0.into(),
                    validator_stake: ix.accounts[5].0.into(),
                    transient_stake: ix.accounts[6].0.into(),
                    clock: ix.accounts[7].0.into(),
                    stake_history: ix.accounts[8].0.into(),
                    system_program: ix.accounts[9].0.into(),
                    stake_program: ix.accounts[10].0.into(),
                };

                let de_ix_data = DecreaseValidatorStakeWithReserveData {
                    lamports,
                    transient_stake_seed,
                };

                Ok(StakePoolProgramIx::DecreaseValidatorStakeWithReserve(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            StakePoolInstruction::DepositStakeWithSlippage {
                minimum_pool_tokens_out,
            } => {
                check_min_accounts_req(accounts_len, 15)?;

                let ix_accounts = DepositStakeWithSlippageAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    validator_list_storage: ix.accounts[1].0.into(),
                    stake_pool_deposit_authority: ix.accounts[2].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[3].0.into(),
                    deposit_stake_address: ix.accounts[4].0.into(),
                    validator_stake_account: ix.accounts[5].0.into(),
                    reserve_stake_account: ix.accounts[6].0.into(),
                    pool_tokens_to: ix.accounts[7].0.into(),
                    manager_fee_account: ix.accounts[8].0.into(),
                    referrer_pool_tokens_account: ix.accounts[9].0.into(),
                    pool_mint: ix.accounts[10].0.into(),
                    clock: ix.accounts[11].0.into(),
                    sysvar_stake_history: ix.accounts[12].0.into(),
                    token_program: ix.accounts[13].0.into(),
                    stake_program: ix.accounts[14].0.into(),
                };

                let de_ix_data = DepositStakeWithSlippageData {
                    minimum_pool_tokens_out,
                };

                Ok(StakePoolProgramIx::DepositStakeWithSlippage(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            StakePoolInstruction::WithdrawStakeWithSlippage {
                pool_tokens_in,
                minimum_lamports_out,
            } => {
                check_min_accounts_req(accounts_len, 13)?;

                let ix_accounts = WithdrawStakeWithSlippageAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    validator_list_storage: ix.accounts[1].0.into(),
                    stake_pool_withdraw: ix.accounts[2].0.into(),
                    stake_to_split: ix.accounts[3].0.into(),
                    stake_to_receive: ix.accounts[4].0.into(),
                    user_stake_authority: ix.accounts[5].0.into(),
                    user_transfer_authority: ix.accounts[6].0.into(),
                    user_pool_token_account: ix.accounts[7].0.into(),
                    manager_fee_account: ix.accounts[8].0.into(),
                    pool_mint: ix.accounts[9].0.into(),
                    clock: ix.accounts[10].0.into(),
                    token_program: ix.accounts[11].0.into(),
                    stake_program: ix.accounts[12].0.into(),
                };

                let de_ix_data = WithdrawStakeWithSlippageData {
                    pool_tokens_in,
                    minimum_lamports_out,
                };

                Ok(StakePoolProgramIx::WithdrawStakeWithSlippage(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            StakePoolInstruction::DepositSolWithSlippage {
                lamports_in,
                minimum_pool_tokens_out,
            } => {
                check_min_accounts_req(accounts_len, 13)?;

                let ix_accounts = DepositSolWithSlippageAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[1].0.into(),
                    reserve_stake_account: ix.accounts[2].0.into(),
                    lamports_from: ix.accounts[3].0.into(),
                    pool_tokens_to: ix.accounts[4].0.into(),
                    manager_fee_account: ix.accounts[5].0.into(),
                    referrer_pool_tokens_account: ix.accounts[6].0.into(),
                    pool_mint: ix.accounts[7].0.into(),
                    system_program: ix.accounts[8].0.into(),
                    token_program: ix.accounts[9].0.into(),
                    deposit_authority: ix
                        .accounts
                        .get(10)
                        .map(|account| Some(account.0.into()))
                        .unwrap_or(None),
                };

                let de_ix_data = DepositSolWithSlippageData {
                    lamports_in,
                    minimum_pool_tokens_out,
                };

                Ok(StakePoolProgramIx::DepositSolWithSlippage(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            StakePoolInstruction::WithdrawSolWithSlippage {
                pool_tokens_in,
                minimum_lamports_out,
            } => {
                check_min_accounts_req(accounts_len, 13)?;

                let ix_accounts = WithdrawSolWithSlippageAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[1].0.into(),
                    user_transfer_authority: ix.accounts[2].0.into(),
                    pool_tokens_from: ix.accounts[3].0.into(),
                    reserve_stake_account: ix.accounts[4].0.into(),
                    lamports_to: ix.accounts[5].0.into(),
                    manager_fee_account: ix.accounts[6].0.into(),
                    pool_mint: ix.accounts[7].0.into(),
                    clock: ix.accounts[8].0.into(),
                    sysvar_stake_history: ix.accounts[9].0.into(),
                    stake_program: ix.accounts[10].0.into(),
                    token_program: ix.accounts[11].0.into(),
                    sol_withdraw_authority: ix
                        .accounts
                        .get(12)
                        .map(|account| Some(account.0.into()))
                        .unwrap_or(None),
                };

                let de_ix_data = WithdrawSolWithSlippageData {
                    pool_tokens_in,
                    minimum_lamports_out,
                };

                Ok(StakePoolProgramIx::WithdrawSolWithSlippage(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            _ => Err(yellowstone_vixen_core::ParseError::from(
                "Invalid Instruction discriminator".to_owned(),
            )),
        }
    }
}

pub fn check_min_accounts_req(
    actual: usize,
    expected: usize,
) -> yellowstone_vixen_core::ParseResult<()> {
    if actual < expected {
        Err(yellowstone_vixen_core::ParseError::from(format!(
            "Too few accounts provided: expected {expected}, got {actual}"
        )))
    } else {
        Ok(())
    }
}
