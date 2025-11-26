use borsh::BorshDeserialize;
use spl_stake_pool::instruction::StakePoolInstruction;
use yellowstone_vixen_parser::check_min_accounts_req;

use crate::instruction_helpers::{
    AddValidatorToPoolAccounts, AddValidatorToPoolArgs, CleanupRemovedValidatorEntriesAccounts,
    CreateTokenMetadataAccounts, CreateTokenMetadataArgs, DecreaseAdditionalValidatorStakeAccounts,
    DecreaseAdditionalValidatorStakeArgs, DecreaseValidatorStakeAccounts,
    DecreaseValidatorStakeArgs, DecreaseValidatorStakeWithReserveAccounts,
    DecreaseValidatorStakeWithReserveArgs, DepositSolAccounts, DepositSolArgs,
    DepositSolWithSlippageAccounts, DepositSolWithSlippageArgs, DepositStakeAccounts,
    DepositStakeWithSlippageAccounts, DepositStakeWithSlippageArgs,
    IncreaseAdditionalValidatorStakeAccounts, IncreaseAdditionalValidatorStakeArgs,
    IncreaseValidatorStakeAccounts, IncreaseValidatorStakeArgs, InitializeAccounts, InitializeArgs,
    RemoveValidatorFromPoolAccounts, SetFeeAccounts, SetFeeArgs, SetFundingAuthorityAccounts,
    SetFundingAuthorityArgs, SetManagerAccounts, SetPreferredValidatorAccounts,
    SetPreferredValidatorArgs, SetStakerAccounts, StakePoolProgramInstruction,
    UpdateStakePoolBalanceAccounts, UpdateTokenMetadataAccounts, UpdateTokenMetadataArgs,
    UpdateValidatorListBalanceAccounts, UpdateValidatorListBalanceArgs, WithdrawSolAccounts,
    WithdrawSolArgs, WithdrawSolWithSlippageAccounts, WithdrawSolWithSlippageArgs,
    WithdrawStakeAccounts, WithdrawStakeArgs, WithdrawStakeWithSlippageAccounts,
    WithdrawStakeWithSlippageArgs,
};

#[derive(Debug, Copy, Clone)]
pub struct InstructionParser;

impl yellowstone_vixen_core::Parser for InstructionParser {
    type Input = yellowstone_vixen_core::instruction::InstructionUpdate;
    type Output = StakePoolProgramInstruction;

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
    ) -> yellowstone_vixen_core::ParseResult<StakePoolProgramInstruction> {
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
                    stake_pool: ix.accounts[0],
                    manager: ix.accounts[1],
                    staker: ix.accounts[2],
                    stake_pool_withdraw_authority: ix.accounts[3],
                    validator_list: ix.accounts[4],
                    reserve_stake: ix.accounts[5],
                    pool_mint: ix.accounts[6],
                    manager_pool_account: ix.accounts[7],
                    token_program: ix.accounts[8],
                    deposit_authority: None,
                };

                if let Some(deposit_authority) = ix.accounts.get(9) {
                    ix_accounts.deposit_authority = Some(deposit_authority.to_owned());
                }

                let args = InitializeArgs {
                    fee,
                    withdrawal_fee,
                    deposit_fee,
                    referral_fee,
                    max_validators,
                };

                Ok(StakePoolProgramInstruction::Initialize {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::AddValidatorToPool(raw_validator_seed) => {
                check_min_accounts_req(accounts_len, 13)?;

                let ix_accounts = AddValidatorToPoolAccounts {
                    stake_pool: ix.accounts[0],
                    staker: ix.accounts[1],
                    funder: ix.accounts[2],
                    stake_pool_withdraw: ix.accounts[3],
                    validator_list: ix.accounts[4],
                    stake: ix.accounts[5],
                    validator: ix.accounts[6],
                    rent: ix.accounts[7],
                    clock: ix.accounts[8],
                    sysvar_stake_history: ix.accounts[9],
                    stake_config: ix.accounts[10],
                    system_program: ix.accounts[11],
                    stake_program: ix.accounts[12],
                };

                let args = AddValidatorToPoolArgs { raw_validator_seed };

                Ok(StakePoolProgramInstruction::AddValidatorToPool {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::RemoveValidatorFromPool => {
                check_min_accounts_req(accounts_len, 8)?;

                let ix_accounts = RemoveValidatorFromPoolAccounts {
                    stake_pool: ix.accounts[0],
                    staker: ix.accounts[1],
                    stake_pool_withdraw: ix.accounts[2],
                    validator_list: ix.accounts[3],
                    stake_account: ix.accounts[4],
                    transient_stake_account: ix.accounts[5],
                    clock: ix.accounts[6],
                    stake_program: ix.accounts[7],
                };

                Ok(StakePoolProgramInstruction::RemoveValidatorFromPool {
                    accounts: ix_accounts,
                })
            },
            StakePoolInstruction::DecreaseValidatorStake {
                lamports,
                transient_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 10)?;

                let ix_accounts = DecreaseValidatorStakeAccounts {
                    stake_pool: ix.accounts[0],
                    staker: ix.accounts[1],
                    stake_pool_withdraw_authority: ix.accounts[2],
                    validator_list: ix.accounts[3],
                    validator_stake: ix.accounts[4],
                    transient_stake: ix.accounts[5],
                    clock: ix.accounts[6],
                    rent: ix.accounts[7],
                    system_program: ix.accounts[8],
                    stake_program: ix.accounts[9],
                };

                let args = DecreaseValidatorStakeArgs {
                    lamports,
                    transient_stake_seed,
                };

                Ok(StakePoolProgramInstruction::DecreaseValidatorStake {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::IncreaseValidatorStake {
                lamports,
                transient_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 14)?;

                let ix_accounts = IncreaseValidatorStakeAccounts {
                    stake_pool: ix.accounts[0],
                    staker: ix.accounts[1],
                    stake_pool_withdraw_authority: ix.accounts[2],
                    validator_list: ix.accounts[3],
                    reserve_stake: ix.accounts[4],
                    transient_stake: ix.accounts[5],
                    validator_stake: ix.accounts[6],
                    validator: ix.accounts[7],
                    clock: ix.accounts[8],
                    rent: ix.accounts[9],
                    sysvar_stake_history: ix.accounts[10],
                    stake_config: ix.accounts[11],
                    system_program: ix.accounts[12],
                    stake_program: ix.accounts[13],
                };

                let args = IncreaseValidatorStakeArgs {
                    lamports,
                    transient_stake_seed,
                };

                Ok(StakePoolProgramInstruction::IncreaseValidatorStake {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::SetPreferredValidator {
                validator_type,
                validator_vote_address,
            } => {
                check_min_accounts_req(accounts_len, 3)?;

                let ix_accounts = SetPreferredValidatorAccounts {
                    stake_pool_address: ix.accounts[0],
                    staker: ix.accounts[1],
                    validator_list_address: ix.accounts[2],
                };

                let args = SetPreferredValidatorArgs {
                    validator_type,
                    validator_vote_address: validator_vote_address.map(|p| p.to_bytes().into()),
                };

                Ok(StakePoolProgramInstruction::SetPreferredValidator {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::UpdateValidatorListBalance {
                start_index,
                no_merge,
            } => {
                check_min_accounts_req(accounts_len, 7)?;

                let ix_accounts = UpdateValidatorListBalanceAccounts {
                    stake_pool: ix.accounts[0],
                    stake_pool_withdraw_authority: ix.accounts[1],
                    validator_list_address: ix.accounts[2],
                    reserve_stake: ix.accounts[3],
                    clock: ix.accounts[4],
                    sysvar_stake_history: ix.accounts[5],
                    stake_program: ix.accounts[6],
                };

                let args = UpdateValidatorListBalanceArgs {
                    start_index,
                    no_merge,
                };

                Ok(StakePoolProgramInstruction::UpdateValidatorListBalance {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::UpdateStakePoolBalance => {
                check_min_accounts_req(accounts_len, 7)?;

                let ix_accounts = UpdateStakePoolBalanceAccounts {
                    stake_pool: ix.accounts[0],
                    withdraw_authority: ix.accounts[1],
                    validator_list_storage: ix.accounts[2],
                    reserve_stake: ix.accounts[3],
                    manager_fee_account: ix.accounts[4],
                    stake_pool_mint: ix.accounts[5],
                    token_program: ix.accounts[6],
                };

                Ok(StakePoolProgramInstruction::UpdateStakePoolBalance {
                    accounts: ix_accounts,
                })
            },
            StakePoolInstruction::CleanupRemovedValidatorEntries => {
                check_min_accounts_req(accounts_len, 2)?;

                let ix_accounts = CleanupRemovedValidatorEntriesAccounts {
                    stake_pool: ix.accounts[0],
                    validator_list_storage: ix.accounts[1],
                };

                Ok(
                    StakePoolProgramInstruction::CleanupRemovedValidatorEntries {
                        accounts: ix_accounts,
                    },
                )
            },
            StakePoolInstruction::DepositStake => {
                check_min_accounts_req(accounts_len, 15)?;

                let ix_accounts = DepositStakeAccounts {
                    stake_pool: ix.accounts[0],
                    validator_list_storage: ix.accounts[1],
                    stake_pool_deposit_authority: ix.accounts[2],
                    stake_pool_withdraw_authority: ix.accounts[3],
                    deposit_stake_address: ix.accounts[4],
                    validator_stake_account: ix.accounts[5],
                    reserve_stake_account: ix.accounts[6],
                    pool_tokens_to: ix.accounts[7],
                    manager_fee_account: ix.accounts[8],
                    referrer_pool_tokens_account: ix.accounts[9],
                    pool_mint: ix.accounts[10],
                    clock: ix.accounts[11],
                    sysvar_stake_history: ix.accounts[12],
                    token_program: ix.accounts[13],
                    stake_program: ix.accounts[14],
                };

                Ok(StakePoolProgramInstruction::DepositStake {
                    accounts: ix_accounts,
                })
            },
            StakePoolInstruction::WithdrawStake(amount) => {
                check_min_accounts_req(accounts_len, 13)?;

                let ix_accounts = WithdrawStakeAccounts {
                    stake_pool: ix.accounts[0],
                    validator_list_storage: ix.accounts[1],
                    stake_pool_withdraw: ix.accounts[2],
                    stake_to_split: ix.accounts[3],
                    stake_to_receive: ix.accounts[4],
                    user_stake_authority: ix.accounts[5],
                    user_transfer_authority: ix.accounts[6],
                    user_pool_token_account: ix.accounts[7],
                    manager_fee_account: ix.accounts[8],
                    pool_mint: ix.accounts[9],
                    clock: ix.accounts[10],
                    token_program: ix.accounts[11],
                    stake_program: ix.accounts[12],
                };

                let args = WithdrawStakeArgs { arg: amount };

                Ok(StakePoolProgramInstruction::WithdrawStake {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::SetManager => {
                check_min_accounts_req(accounts_len, 4)?;

                let ix_accounts = SetManagerAccounts {
                    stake_pool: ix.accounts[0],
                    manager: ix.accounts[1],
                    new_manager: ix.accounts[2],
                    new_fee_receiver: ix.accounts[3],
                };

                Ok(StakePoolProgramInstruction::SetManager {
                    accounts: ix_accounts,
                })
            },
            StakePoolInstruction::SetFee { fee } => {
                check_min_accounts_req(accounts_len, 2)?;

                let ix_accounts = SetFeeAccounts {
                    stake_pool: ix.accounts[0],
                    manager: ix.accounts[1],
                };

                let args = SetFeeArgs { fee };

                Ok(StakePoolProgramInstruction::SetFee {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::SetStaker => {
                check_min_accounts_req(accounts_len, 3)?;

                let ix_accounts = SetStakerAccounts {
                    stake_pool: ix.accounts[0],
                    set_staker_authority: ix.accounts[1],
                    new_staker: ix.accounts[2],
                };

                Ok(StakePoolProgramInstruction::SetStaker {
                    accounts: ix_accounts,
                })
            },
            StakePoolInstruction::DepositSol(amount) => {
                check_min_accounts_req(accounts_len, 10)?;

                let ix_accounts = DepositSolAccounts {
                    stake_pool: ix.accounts[0],
                    stake_pool_withdraw_authority: ix.accounts[1],
                    reserve_stake_account: ix.accounts[2],
                    lamports_from: ix.accounts[3],
                    pool_tokens_to: ix.accounts[4],
                    manager_fee_account: ix.accounts[5],
                    referrer_pool_tokens_account: ix.accounts[6],
                    pool_mint: ix.accounts[7],
                    system_program: ix.accounts[8],
                    token_program: ix.accounts[9],
                    deposit_authority: ix
                        .accounts
                        .get(10)
                        .map(|account| Some(account.to_owned()))
                        .unwrap_or(None),
                };

                let args = DepositSolArgs { arg: amount };

                Ok(StakePoolProgramInstruction::DepositSol {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::SetFundingAuthority(funding_type) => {
                check_min_accounts_req(accounts_len, 2)?;

                let ix_accounts = SetFundingAuthorityAccounts {
                    stake_pool: ix.accounts[0],
                    manager: ix.accounts[1],
                    auth: ix
                        .accounts
                        .get(2)
                        .map(|account| Some(account.to_owned()))
                        .unwrap_or(None),
                };

                let args = SetFundingAuthorityArgs { arg: funding_type };

                Ok(StakePoolProgramInstruction::SetFundingAuthority {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::WithdrawSol(amount) => {
                check_min_accounts_req(accounts_len, 12)?;

                let ix_accounts = WithdrawSolAccounts {
                    stake_pool: ix.accounts[0],
                    stake_pool_withdraw_authority: ix.accounts[1],
                    user_transfer_authority: ix.accounts[2],
                    pool_tokens_from: ix.accounts[3],
                    reserve_stake_account: ix.accounts[4],
                    lamports_to: ix.accounts[5],
                    manager_fee_account: ix.accounts[6],
                    pool_mint: ix.accounts[7],
                    clock: ix.accounts[8],
                    sysvar_stake_history: ix.accounts[9],
                    stake_program: ix.accounts[10],
                    token_program: ix.accounts[11],
                    sol_withdraw_authority: ix
                        .accounts
                        .get(12)
                        .map(|account| Some(account.to_owned()))
                        .unwrap_or(None),
                };

                let args = WithdrawSolArgs { arg: amount };
                Ok(StakePoolProgramInstruction::WithdrawSol {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::CreateTokenMetadata { name, symbol, uri } => {
                check_min_accounts_req(accounts_len, 9)?;

                let ix_accounts = CreateTokenMetadataAccounts {
                    stake_pool: ix.accounts[0],
                    manager: ix.accounts[1],
                    stake_pool_withdraw_authority: ix.accounts[2],
                    pool_mint: ix.accounts[3],
                    payer: ix.accounts[4],
                    token_metadata: ix.accounts[5],
                    mpl_token_metadata: ix.accounts[6],
                    system_program: ix.accounts[7],
                };

                let args = CreateTokenMetadataArgs { name, symbol, uri };
                Ok(StakePoolProgramInstruction::CreateTokenMetadata {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::UpdateTokenMetadata { name, symbol, uri } => {
                check_min_accounts_req(accounts_len, 5)?;

                let ix_accounts = UpdateTokenMetadataAccounts {
                    stake_pool: ix.accounts[0],
                    manager: ix.accounts[1],
                    stake_pool_withdraw_authority: ix.accounts[2],
                    token_metadata: ix.accounts[3],
                    mpl_token_metadata: ix.accounts[4],
                };

                let args = UpdateTokenMetadataArgs { name, symbol, uri };

                Ok(StakePoolProgramInstruction::UpdateTokenMetadata {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::IncreaseAdditionalValidatorStake {
                lamports,
                transient_stake_seed,
                ephemeral_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 14)?;

                let ix_accounts = IncreaseAdditionalValidatorStakeAccounts {
                    stake_pool: ix.accounts[0],
                    staker: ix.accounts[1],
                    stake_pool_withdraw_authority: ix.accounts[2],
                    validator_list: ix.accounts[3],
                    reserve_stake: ix.accounts[4],
                    ephemeral_stake: ix.accounts[5],
                    transient_stake: ix.accounts[6],
                    validator_stake: ix.accounts[7],
                    validator: ix.accounts[8],
                    clock: ix.accounts[9],
                    stake_history: ix.accounts[10],
                    stake_config: ix.accounts[11],
                    system_program: ix.accounts[12],
                    stake_program: ix.accounts[13],
                };

                let args = IncreaseAdditionalValidatorStakeArgs {
                    lamports,
                    transient_stake_seed,
                    ephemeral_stake_seed,
                };

                Ok(
                    StakePoolProgramInstruction::IncreaseAdditionalValidatorStake {
                        accounts: ix_accounts,
                        args,
                    },
                )
            },
            StakePoolInstruction::DecreaseAdditionalValidatorStake {
                lamports,
                transient_stake_seed,
                ephemeral_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 12)?;

                let ix_accounts = DecreaseAdditionalValidatorStakeAccounts {
                    stake_pool: ix.accounts[0],
                    staker: ix.accounts[1],
                    stake_pool_withdraw_authority: ix.accounts[2],
                    validator_list: ix.accounts[3],
                    reserve_stake: ix.accounts[4],
                    validator_stake: ix.accounts[5],
                    ephemeral_stake: ix.accounts[6],
                    transient_stake: ix.accounts[7],
                    clock: ix.accounts[8],
                    stake_history: ix.accounts[9],
                    system_program: ix.accounts[10],
                    stake_program: ix.accounts[11],
                };

                let args = DecreaseAdditionalValidatorStakeArgs {
                    lamports,
                    transient_stake_seed,
                    ephemeral_stake_seed,
                };

                Ok(
                    StakePoolProgramInstruction::DecreaseAdditionalValidatorStake {
                        accounts: ix_accounts,
                        args,
                    },
                )
            },
            StakePoolInstruction::DecreaseValidatorStakeWithReserve {
                lamports,
                transient_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 11)?;

                let ix_accounts = DecreaseValidatorStakeWithReserveAccounts {
                    stake_pool: ix.accounts[0],
                    staker: ix.accounts[1],
                    stake_pool_withdraw_authority: ix.accounts[2],
                    validator_list: ix.accounts[3],
                    reserve_stake: ix.accounts[4],
                    validator_stake: ix.accounts[5],
                    transient_stake: ix.accounts[6],
                    clock: ix.accounts[7],
                    stake_history: ix.accounts[8],
                    system_program: ix.accounts[9],
                    stake_program: ix.accounts[10],
                };

                let args = DecreaseValidatorStakeWithReserveArgs {
                    lamports,
                    transient_stake_seed,
                };

                Ok(
                    StakePoolProgramInstruction::DecreaseValidatorStakeWithReserve {
                        accounts: ix_accounts,
                        args,
                    },
                )
            },
            StakePoolInstruction::DepositStakeWithSlippage {
                minimum_pool_tokens_out,
            } => {
                check_min_accounts_req(accounts_len, 15)?;

                let ix_accounts = DepositStakeWithSlippageAccounts {
                    stake_pool: ix.accounts[0],
                    validator_list_storage: ix.accounts[1],
                    stake_pool_deposit_authority: ix.accounts[2],
                    stake_pool_withdraw_authority: ix.accounts[3],
                    deposit_stake_address: ix.accounts[4],
                    validator_stake_account: ix.accounts[5],
                    reserve_stake_account: ix.accounts[6],
                    pool_tokens_to: ix.accounts[7],
                    manager_fee_account: ix.accounts[8],
                    referrer_pool_tokens_account: ix.accounts[9],
                    pool_mint: ix.accounts[10],
                    clock: ix.accounts[11],
                    sysvar_stake_history: ix.accounts[12],
                    token_program: ix.accounts[13],
                    stake_program: ix.accounts[14],
                };

                let args = DepositStakeWithSlippageArgs {
                    minimum_pool_tokens_out,
                };

                Ok(StakePoolProgramInstruction::DepositStakeWithSlippage {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::WithdrawStakeWithSlippage {
                pool_tokens_in,
                minimum_lamports_out,
            } => {
                check_min_accounts_req(accounts_len, 13)?;

                let ix_accounts = WithdrawStakeWithSlippageAccounts {
                    stake_pool: ix.accounts[0],
                    validator_list_storage: ix.accounts[1],
                    stake_pool_withdraw: ix.accounts[2],
                    stake_to_split: ix.accounts[3],
                    stake_to_receive: ix.accounts[4],
                    user_stake_authority: ix.accounts[5],
                    user_transfer_authority: ix.accounts[6],
                    user_pool_token_account: ix.accounts[7],
                    manager_fee_account: ix.accounts[8],
                    pool_mint: ix.accounts[9],
                    clock: ix.accounts[10],
                    token_program: ix.accounts[11],
                    stake_program: ix.accounts[12],
                };

                let args = WithdrawStakeWithSlippageArgs {
                    pool_tokens_in,
                    minimum_lamports_out,
                };

                Ok(StakePoolProgramInstruction::WithdrawStakeWithSlippage {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::DepositSolWithSlippage {
                lamports_in,
                minimum_pool_tokens_out,
            } => {
                check_min_accounts_req(accounts_len, 13)?;

                let ix_accounts = DepositSolWithSlippageAccounts {
                    stake_pool: ix.accounts[0],
                    stake_pool_withdraw_authority: ix.accounts[1],
                    reserve_stake_account: ix.accounts[2],
                    lamports_from: ix.accounts[3],
                    pool_tokens_to: ix.accounts[4],
                    manager_fee_account: ix.accounts[5],
                    referrer_pool_tokens_account: ix.accounts[6],
                    pool_mint: ix.accounts[7],
                    system_program: ix.accounts[8],
                    token_program: ix.accounts[9],
                    deposit_authority: ix
                        .accounts
                        .get(10)
                        .map(|account| Some(account.to_owned()))
                        .unwrap_or(None),
                };

                let args = DepositSolWithSlippageArgs {
                    lamports_in,
                    minimum_pool_tokens_out,
                };

                Ok(StakePoolProgramInstruction::DepositSolWithSlippage {
                    accounts: ix_accounts,
                    args,
                })
            },
            StakePoolInstruction::WithdrawSolWithSlippage {
                pool_tokens_in,
                minimum_lamports_out,
            } => {
                check_min_accounts_req(accounts_len, 13)?;

                let ix_accounts = WithdrawSolWithSlippageAccounts {
                    stake_pool: ix.accounts[0],
                    stake_pool_withdraw_authority: ix.accounts[1],
                    user_transfer_authority: ix.accounts[2],
                    pool_tokens_from: ix.accounts[3],
                    reserve_stake_account: ix.accounts[4],
                    lamports_to: ix.accounts[5],
                    manager_fee_account: ix.accounts[6],
                    pool_mint: ix.accounts[7],
                    clock: ix.accounts[8],
                    sysvar_stake_history: ix.accounts[9],
                    stake_program: ix.accounts[10],
                    token_program: ix.accounts[11],
                    sol_withdraw_authority: ix
                        .accounts
                        .get(12)
                        .map(|account| Some(account.to_owned()))
                        .unwrap_or(None),
                };

                let args = WithdrawSolWithSlippageArgs {
                    pool_tokens_in,
                    minimum_lamports_out,
                };

                Ok(StakePoolProgramInstruction::WithdrawSolWithSlippage {
                    accounts: ix_accounts,
                    args,
                })
            },
            _ => Err(yellowstone_vixen_core::ParseError::from(
                "Invalid Instruction discriminator".to_owned(),
            )),
        }
    }
}
