use borsh::BorshDeserialize;
use spl_stake_pool::instruction::StakePoolInstruction;
use yellowstone_vixen_core::{
    instruction::InstructionUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};
use yellowstone_vixen_parser::check_min_accounts_req;

use crate::instructions::*;

#[derive(Copy, Clone)]
pub struct InstructionParser;

impl Parser for InstructionParser {
    type Input = InstructionUpdate;
    type Output = StakePoolProgramInstructionProto;

    fn id(&self) -> std::borrow::Cow<'static, str> { "StakePool::InstructionParser".into() }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .transaction_accounts([spl_stake_pool::id()])
            .build()
            .unwrap()
    }

    async fn parse(&self, ix_update: &InstructionUpdate) -> ParseResult<Self::Output> {
        if ix_update.program.equals_ref(spl_stake_pool::id()) {
            InstructionParser::parse_impl(ix_update)
        } else {
            Err(ParseError::Filtered)
        }
    }
}

impl ProgramParser for InstructionParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        spl_stake_pool::id().to_bytes().into()
    }
}

impl InstructionParser {
    pub(crate) fn parse_impl(
        ix: &InstructionUpdate,
    ) -> ParseResult<StakePoolProgramInstructionProto> {
        let ix_type = StakePoolInstruction::try_from_slice(ix.data.as_slice())?;
        let accounts_len = ix.accounts.len();

        use stake_pool_program_instruction_proto::Instruction as Out;

        let instruction = match ix_type {
            StakePoolInstruction::Initialize {
                fee,
                withdrawal_fee,
                deposit_fee,
                referral_fee,
                max_validators,
            } => {
                check_min_accounts_req(accounts_len, 9)?;

                Out::Initialize(InitializeIxProto {
                    accounts: Some(InitializeAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        manager: ix.accounts[1].into_bytes().to_vec(),
                        staker: ix.accounts[2].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[3].into_bytes().to_vec(),
                        validator_list: ix.accounts[4].into_bytes().to_vec(),
                        reserve_stake: ix.accounts[5].into_bytes().to_vec(),
                        pool_mint: ix.accounts[6].into_bytes().to_vec(),
                        manager_pool_account: ix.accounts[7].into_bytes().to_vec(),
                        token_program: ix.accounts[8].into_bytes().to_vec(),
                        deposit_authority: ix.accounts.get(9).map(|a| a.into_bytes().to_vec()),
                    }),
                    args: Some(InitializeArgsProto {
                        fee: Some(fee_to_proto(fee)),
                        withdrawal_fee: Some(fee_to_proto(withdrawal_fee)),
                        deposit_fee: Some(fee_to_proto(deposit_fee)),
                        referral_fee: referral_fee as u32,
                        max_validators,
                    }),
                })
            },

            StakePoolInstruction::AddValidatorToPool(raw_validator_seed) => {
                check_min_accounts_req(accounts_len, 13)?;

                Out::AddValidatorToPool(AddValidatorToPoolIxProto {
                    accounts: Some(AddValidatorToPoolAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        staker: ix.accounts[1].into_bytes().to_vec(),
                        funder: ix.accounts[2].into_bytes().to_vec(),
                        stake_pool_withdraw: ix.accounts[3].into_bytes().to_vec(),
                        validator_list: ix.accounts[4].into_bytes().to_vec(),
                        stake: ix.accounts[5].into_bytes().to_vec(),
                        validator: ix.accounts[6].into_bytes().to_vec(),
                        rent: ix.accounts[7].into_bytes().to_vec(),
                        clock: ix.accounts[8].into_bytes().to_vec(),
                        sysvar_stake_history: ix.accounts[9].into_bytes().to_vec(),
                        stake_config: ix.accounts[10].into_bytes().to_vec(),
                        system_program: ix.accounts[11].into_bytes().to_vec(),
                        stake_program: ix.accounts[12].into_bytes().to_vec(),
                    }),
                    args: Some(AddValidatorToPoolArgsProto { raw_validator_seed }),
                })
            },

            StakePoolInstruction::RemoveValidatorFromPool => {
                check_min_accounts_req(accounts_len, 8)?;

                Out::RemoveValidatorFromPool(RemoveValidatorFromPoolIxProto {
                    accounts: Some(RemoveValidatorFromPoolAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        staker: ix.accounts[1].into_bytes().to_vec(),
                        stake_pool_withdraw: ix.accounts[2].into_bytes().to_vec(),
                        validator_list: ix.accounts[3].into_bytes().to_vec(),
                        stake_account: ix.accounts[4].into_bytes().to_vec(),
                        transient_stake_account: ix.accounts[5].into_bytes().to_vec(),
                        clock: ix.accounts[6].into_bytes().to_vec(),
                        stake_program: ix.accounts[7].into_bytes().to_vec(),
                    }),
                })
            },

            StakePoolInstruction::DecreaseValidatorStake {
                lamports,
                transient_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 10)?;

                Out::DecreaseValidatorStake(DecreaseValidatorStakeIxProto {
                    accounts: Some(DecreaseValidatorStakeAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        staker: ix.accounts[1].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[2].into_bytes().to_vec(),
                        validator_list: ix.accounts[3].into_bytes().to_vec(),
                        validator_stake: ix.accounts[4].into_bytes().to_vec(),
                        transient_stake: ix.accounts[5].into_bytes().to_vec(),
                        clock: ix.accounts[6].into_bytes().to_vec(),
                        rent: ix.accounts[7].into_bytes().to_vec(),
                        system_program: ix.accounts[8].into_bytes().to_vec(),
                        stake_program: ix.accounts[9].into_bytes().to_vec(),
                    }),
                    args: Some(DecreaseValidatorStakeArgsProto {
                        lamports,
                        transient_stake_seed,
                    }),
                })
            },

            StakePoolInstruction::IncreaseValidatorStake {
                lamports,
                transient_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 14)?;

                Out::IncreaseValidatorStake(IncreaseValidatorStakeIxProto {
                    accounts: Some(IncreaseValidatorStakeAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        staker: ix.accounts[1].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[2].into_bytes().to_vec(),
                        validator_list: ix.accounts[3].into_bytes().to_vec(),
                        reserve_stake: ix.accounts[4].into_bytes().to_vec(),
                        transient_stake: ix.accounts[5].into_bytes().to_vec(),
                        validator_stake: ix.accounts[6].into_bytes().to_vec(),
                        validator: ix.accounts[7].into_bytes().to_vec(),
                        clock: ix.accounts[8].into_bytes().to_vec(),
                        rent: ix.accounts[9].into_bytes().to_vec(),
                        sysvar_stake_history: ix.accounts[10].into_bytes().to_vec(),
                        stake_config: ix.accounts[11].into_bytes().to_vec(),
                        system_program: ix.accounts[12].into_bytes().to_vec(),
                        stake_program: ix.accounts[13].into_bytes().to_vec(),
                    }),
                    args: Some(IncreaseValidatorStakeArgsProto {
                        lamports,
                        transient_stake_seed,
                    }),
                })
            },

            StakePoolInstruction::SetPreferredValidator {
                validator_type,
                validator_vote_address,
            } => {
                check_min_accounts_req(accounts_len, 3)?;

                Out::SetPreferredValidator(SetPreferredValidatorIxProto {
                    accounts: Some(SetPreferredValidatorAccountsProto {
                        stake_pool_address: ix.accounts[0].into_bytes().to_vec(),
                        staker: ix.accounts[1].into_bytes().to_vec(),
                        validator_list_address: ix.accounts[2].into_bytes().to_vec(),
                    }),
                    args: Some(SetPreferredValidatorArgsProto {
                        validator_type: preferred_validator_type_to_proto(validator_type) as i32,
                        // real Pubkey, not an account meta
                        validator_vote_address: validator_vote_address
                            .map(|p| p.to_bytes().to_vec()),
                    }),
                })
            },

            StakePoolInstruction::UpdateValidatorListBalance {
                start_index,
                no_merge,
            } => {
                check_min_accounts_req(accounts_len, 7)?;

                Out::UpdateValidatorListBalance(UpdateValidatorListBalanceIxProto {
                    accounts: Some(UpdateValidatorListBalanceAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[1].into_bytes().to_vec(),
                        validator_list_address: ix.accounts[2].into_bytes().to_vec(),
                        reserve_stake: ix.accounts[3].into_bytes().to_vec(),
                        clock: ix.accounts[4].into_bytes().to_vec(),
                        sysvar_stake_history: ix.accounts[5].into_bytes().to_vec(),
                        stake_program: ix.accounts[6].into_bytes().to_vec(),
                    }),
                    args: Some(UpdateValidatorListBalanceArgsProto {
                        start_index,
                        no_merge,
                    }),
                })
            },

            StakePoolInstruction::UpdateStakePoolBalance => {
                check_min_accounts_req(accounts_len, 7)?;

                Out::UpdateStakePoolBalance(UpdateStakePoolBalanceIxProto {
                    accounts: Some(UpdateStakePoolBalanceAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        withdraw_authority: ix.accounts[1].into_bytes().to_vec(),
                        validator_list_storage: ix.accounts[2].into_bytes().to_vec(),
                        reserve_stake: ix.accounts[3].into_bytes().to_vec(),
                        manager_fee_account: ix.accounts[4].into_bytes().to_vec(),
                        stake_pool_mint: ix.accounts[5].into_bytes().to_vec(),
                        token_program: ix.accounts[6].into_bytes().to_vec(),
                    }),
                })
            },

            StakePoolInstruction::CleanupRemovedValidatorEntries => {
                check_min_accounts_req(accounts_len, 2)?;

                Out::CleanupRemovedValidatorEntries(CleanupRemovedValidatorEntriesIxProto {
                    accounts: Some(CleanupRemovedValidatorEntriesAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        validator_list_storage: ix.accounts[1].into_bytes().to_vec(),
                    }),
                })
            },

            StakePoolInstruction::DepositStake => {
                check_min_accounts_req(accounts_len, 15)?;

                Out::DepositStake(DepositStakeIxProto {
                    accounts: Some(DepositStakeAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        validator_list_storage: ix.accounts[1].into_bytes().to_vec(),
                        stake_pool_deposit_authority: ix.accounts[2].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[3].into_bytes().to_vec(),
                        deposit_stake_address: ix.accounts[4].into_bytes().to_vec(),
                        validator_stake_account: ix.accounts[5].into_bytes().to_vec(),
                        reserve_stake_account: ix.accounts[6].into_bytes().to_vec(),
                        pool_tokens_to: ix.accounts[7].into_bytes().to_vec(),
                        manager_fee_account: ix.accounts[8].into_bytes().to_vec(),
                        referrer_pool_tokens_account: ix.accounts[9].into_bytes().to_vec(),
                        pool_mint: ix.accounts[10].into_bytes().to_vec(),
                        clock: ix.accounts[11].into_bytes().to_vec(),
                        sysvar_stake_history: ix.accounts[12].into_bytes().to_vec(),
                        token_program: ix.accounts[13].into_bytes().to_vec(),
                        stake_program: ix.accounts[14].into_bytes().to_vec(),
                    }),
                })
            },

            StakePoolInstruction::WithdrawStake(amount) => {
                check_min_accounts_req(accounts_len, 13)?;

                Out::WithdrawStake(WithdrawStakeIxProto {
                    accounts: Some(WithdrawStakeAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        validator_list_storage: ix.accounts[1].into_bytes().to_vec(),
                        stake_pool_withdraw: ix.accounts[2].into_bytes().to_vec(),
                        stake_to_split: ix.accounts[3].into_bytes().to_vec(),
                        stake_to_receive: ix.accounts[4].into_bytes().to_vec(),
                        user_stake_authority: ix.accounts[5].into_bytes().to_vec(),
                        user_transfer_authority: ix.accounts[6].into_bytes().to_vec(),
                        user_pool_token_account: ix.accounts[7].into_bytes().to_vec(),
                        manager_fee_account: ix.accounts[8].into_bytes().to_vec(),
                        pool_mint: ix.accounts[9].into_bytes().to_vec(),
                        clock: ix.accounts[10].into_bytes().to_vec(),
                        token_program: ix.accounts[11].into_bytes().to_vec(),
                        stake_program: ix.accounts[12].into_bytes().to_vec(),
                    }),
                    args: Some(WithdrawStakeArgsProto { amount }),
                })
            },

            StakePoolInstruction::SetManager => {
                check_min_accounts_req(accounts_len, 4)?;

                Out::SetManager(SetManagerIxProto {
                    accounts: Some(SetManagerAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        manager: ix.accounts[1].into_bytes().to_vec(),
                        new_manager: ix.accounts[2].into_bytes().to_vec(),
                        new_fee_receiver: ix.accounts[3].into_bytes().to_vec(),
                    }),
                })
            },

            StakePoolInstruction::SetFee { fee } => {
                check_min_accounts_req(accounts_len, 2)?;

                Out::SetFee(SetFeeIxProto {
                    accounts: Some(SetFeeAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        manager: ix.accounts[1].into_bytes().to_vec(),
                    }),
                    args: Some(SetFeeArgsProto {
                        fee: Some(fee_type_to_proto(fee)),
                    }),
                })
            },

            StakePoolInstruction::SetStaker => {
                check_min_accounts_req(accounts_len, 3)?;

                Out::SetStaker(SetStakerIxProto {
                    accounts: Some(SetStakerAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        set_staker_authority: ix.accounts[1].into_bytes().to_vec(),
                        new_staker: ix.accounts[2].into_bytes().to_vec(),
                    }),
                })
            },

            StakePoolInstruction::DepositSol(amount) => {
                check_min_accounts_req(accounts_len, 10)?;

                Out::DepositSol(DepositSolIxProto {
                    accounts: Some(DepositSolAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[1].into_bytes().to_vec(),
                        reserve_stake_account: ix.accounts[2].into_bytes().to_vec(),
                        lamports_from: ix.accounts[3].into_bytes().to_vec(),
                        pool_tokens_to: ix.accounts[4].into_bytes().to_vec(),
                        manager_fee_account: ix.accounts[5].into_bytes().to_vec(),
                        referrer_pool_tokens_account: ix.accounts[6].into_bytes().to_vec(),
                        pool_mint: ix.accounts[7].into_bytes().to_vec(),
                        system_program: ix.accounts[8].into_bytes().to_vec(),
                        token_program: ix.accounts[9].into_bytes().to_vec(),
                        deposit_authority: ix.accounts.get(10).map(|a| a.into_bytes().to_vec()),
                    }),
                    args: Some(DepositSolArgsProto { amount }),
                })
            },

            StakePoolInstruction::SetFundingAuthority(funding_type) => {
                check_min_accounts_req(accounts_len, 2)?;

                Out::SetFundingAuthority(SetFundingAuthorityIxProto {
                    accounts: Some(SetFundingAuthorityAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        manager: ix.accounts[1].into_bytes().to_vec(),
                        auth: ix.accounts.get(2).map(|a| a.into_bytes().to_vec()),
                    }),
                    args: Some(SetFundingAuthorityArgsProto {
                        funding_type: funding_type_to_proto(funding_type) as i32,
                    }),
                })
            },

            StakePoolInstruction::WithdrawSol(amount) => {
                check_min_accounts_req(accounts_len, 12)?;

                Out::WithdrawSol(WithdrawSolIxProto {
                    accounts: Some(WithdrawSolAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[1].into_bytes().to_vec(),
                        user_transfer_authority: ix.accounts[2].into_bytes().to_vec(),
                        pool_tokens_from: ix.accounts[3].into_bytes().to_vec(),
                        reserve_stake_account: ix.accounts[4].into_bytes().to_vec(),
                        lamports_to: ix.accounts[5].into_bytes().to_vec(),
                        manager_fee_account: ix.accounts[6].into_bytes().to_vec(),
                        pool_mint: ix.accounts[7].into_bytes().to_vec(),
                        clock: ix.accounts[8].into_bytes().to_vec(),
                        sysvar_stake_history: ix.accounts[9].into_bytes().to_vec(),
                        stake_program: ix.accounts[10].into_bytes().to_vec(),
                        token_program: ix.accounts[11].into_bytes().to_vec(),
                        sol_withdraw_authority: ix
                            .accounts
                            .get(12)
                            .map(|a| a.into_bytes().to_vec()),
                    }),
                    args: Some(WithdrawSolArgsProto { amount }),
                })
            },

            StakePoolInstruction::CreateTokenMetadata { name, symbol, uri } => {
                check_min_accounts_req(accounts_len, 8)?;

                Out::CreateTokenMetadata(CreateTokenMetadataIxProto {
                    accounts: Some(CreateTokenMetadataAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        manager: ix.accounts[1].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[2].into_bytes().to_vec(),
                        pool_mint: ix.accounts[3].into_bytes().to_vec(),
                        payer: ix.accounts[4].into_bytes().to_vec(),
                        token_metadata: ix.accounts[5].into_bytes().to_vec(),
                        mpl_token_metadata: ix.accounts[6].into_bytes().to_vec(),
                        system_program: ix.accounts[7].into_bytes().to_vec(),
                    }),
                    args: Some(CreateTokenMetadataArgsProto { name, symbol, uri }),
                })
            },

            StakePoolInstruction::UpdateTokenMetadata { name, symbol, uri } => {
                check_min_accounts_req(accounts_len, 5)?;

                Out::UpdateTokenMetadata(UpdateTokenMetadataIxProto {
                    accounts: Some(UpdateTokenMetadataAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        manager: ix.accounts[1].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[2].into_bytes().to_vec(),
                        token_metadata: ix.accounts[3].into_bytes().to_vec(),
                        mpl_token_metadata: ix.accounts[4].into_bytes().to_vec(),
                    }),
                    args: Some(UpdateTokenMetadataArgsProto { name, symbol, uri }),
                })
            },

            StakePoolInstruction::IncreaseAdditionalValidatorStake {
                lamports,
                transient_stake_seed,
                ephemeral_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 14)?;

                Out::IncreaseAdditionalValidatorStake(IncreaseAdditionalValidatorStakeIxProto {
                    accounts: Some(IncreaseAdditionalValidatorStakeAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        staker: ix.accounts[1].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[2].into_bytes().to_vec(),
                        validator_list: ix.accounts[3].into_bytes().to_vec(),
                        reserve_stake: ix.accounts[4].into_bytes().to_vec(),
                        ephemeral_stake: ix.accounts[5].into_bytes().to_vec(),
                        transient_stake: ix.accounts[6].into_bytes().to_vec(),
                        validator_stake: ix.accounts[7].into_bytes().to_vec(),
                        validator: ix.accounts[8].into_bytes().to_vec(),
                        clock: ix.accounts[9].into_bytes().to_vec(),
                        stake_history: ix.accounts[10].into_bytes().to_vec(),
                        stake_config: ix.accounts[11].into_bytes().to_vec(),
                        system_program: ix.accounts[12].into_bytes().to_vec(),
                        stake_program: ix.accounts[13].into_bytes().to_vec(),
                    }),
                    args: Some(IncreaseAdditionalValidatorStakeArgsProto {
                        lamports,
                        transient_stake_seed,
                        ephemeral_stake_seed,
                    }),
                })
            },

            StakePoolInstruction::DecreaseAdditionalValidatorStake {
                lamports,
                transient_stake_seed,
                ephemeral_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 12)?;

                Out::DecreaseAdditionalValidatorStake(DecreaseAdditionalValidatorStakeIxProto {
                    accounts: Some(DecreaseAdditionalValidatorStakeAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        staker: ix.accounts[1].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[2].into_bytes().to_vec(),
                        validator_list: ix.accounts[3].into_bytes().to_vec(),
                        reserve_stake: ix.accounts[4].into_bytes().to_vec(),
                        validator_stake: ix.accounts[5].into_bytes().to_vec(),
                        ephemeral_stake: ix.accounts[6].into_bytes().to_vec(),
                        transient_stake: ix.accounts[7].into_bytes().to_vec(),
                        clock: ix.accounts[8].into_bytes().to_vec(),
                        stake_history: ix.accounts[9].into_bytes().to_vec(),
                        system_program: ix.accounts[10].into_bytes().to_vec(),
                        stake_program: ix.accounts[11].into_bytes().to_vec(),
                    }),
                    args: Some(DecreaseAdditionalValidatorStakeArgsProto {
                        lamports,
                        transient_stake_seed,
                        ephemeral_stake_seed,
                    }),
                })
            },

            StakePoolInstruction::DecreaseValidatorStakeWithReserve {
                lamports,
                transient_stake_seed,
            } => {
                check_min_accounts_req(accounts_len, 11)?;

                Out::DecreaseValidatorStakeWithReserve(DecreaseValidatorStakeWithReserveIxProto {
                    accounts: Some(DecreaseValidatorStakeWithReserveAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        staker: ix.accounts[1].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[2].into_bytes().to_vec(),
                        validator_list: ix.accounts[3].into_bytes().to_vec(),
                        reserve_stake: ix.accounts[4].into_bytes().to_vec(),
                        validator_stake: ix.accounts[5].into_bytes().to_vec(),
                        transient_stake: ix.accounts[6].into_bytes().to_vec(),
                        clock: ix.accounts[7].into_bytes().to_vec(),
                        stake_history: ix.accounts[8].into_bytes().to_vec(),
                        system_program: ix.accounts[9].into_bytes().to_vec(),
                        stake_program: ix.accounts[10].into_bytes().to_vec(),
                    }),
                    args: Some(DecreaseValidatorStakeWithReserveArgsProto {
                        lamports,
                        transient_stake_seed,
                    }),
                })
            },

            StakePoolInstruction::DepositStakeWithSlippage {
                minimum_pool_tokens_out,
            } => {
                check_min_accounts_req(accounts_len, 15)?;

                Out::DepositStakeWithSlippage(DepositStakeWithSlippageIxProto {
                    accounts: Some(DepositStakeWithSlippageAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        validator_list_storage: ix.accounts[1].into_bytes().to_vec(),
                        stake_pool_deposit_authority: ix.accounts[2].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[3].into_bytes().to_vec(),
                        deposit_stake_address: ix.accounts[4].into_bytes().to_vec(),
                        validator_stake_account: ix.accounts[5].into_bytes().to_vec(),
                        reserve_stake_account: ix.accounts[6].into_bytes().to_vec(),
                        pool_tokens_to: ix.accounts[7].into_bytes().to_vec(),
                        manager_fee_account: ix.accounts[8].into_bytes().to_vec(),
                        referrer_pool_tokens_account: ix.accounts[9].into_bytes().to_vec(),
                        pool_mint: ix.accounts[10].into_bytes().to_vec(),
                        clock: ix.accounts[11].into_bytes().to_vec(),
                        sysvar_stake_history: ix.accounts[12].into_bytes().to_vec(),
                        token_program: ix.accounts[13].into_bytes().to_vec(),
                        stake_program: ix.accounts[14].into_bytes().to_vec(),
                    }),
                    args: Some(DepositStakeWithSlippageArgsProto {
                        minimum_pool_tokens_out,
                    }),
                })
            },

            StakePoolInstruction::WithdrawStakeWithSlippage {
                pool_tokens_in,
                minimum_lamports_out,
            } => {
                check_min_accounts_req(accounts_len, 13)?;

                Out::WithdrawStakeWithSlippage(WithdrawStakeWithSlippageIxProto {
                    accounts: Some(WithdrawStakeWithSlippageAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        validator_list_storage: ix.accounts[1].into_bytes().to_vec(),
                        stake_pool_withdraw: ix.accounts[2].into_bytes().to_vec(),
                        stake_to_split: ix.accounts[3].into_bytes().to_vec(),
                        stake_to_receive: ix.accounts[4].into_bytes().to_vec(),
                        user_stake_authority: ix.accounts[5].into_bytes().to_vec(),
                        user_transfer_authority: ix.accounts[6].into_bytes().to_vec(),
                        user_pool_token_account: ix.accounts[7].into_bytes().to_vec(),
                        manager_fee_account: ix.accounts[8].into_bytes().to_vec(),
                        pool_mint: ix.accounts[9].into_bytes().to_vec(),
                        clock: ix.accounts[10].into_bytes().to_vec(),
                        token_program: ix.accounts[11].into_bytes().to_vec(),
                        stake_program: ix.accounts[12].into_bytes().to_vec(),
                    }),
                    args: Some(WithdrawStakeWithSlippageArgsProto {
                        pool_tokens_in,
                        minimum_lamports_out,
                    }),
                })
            },

            StakePoolInstruction::DepositSolWithSlippage {
                lamports_in,
                minimum_pool_tokens_out,
            } => {
                check_min_accounts_req(accounts_len, 10)?;

                Out::DepositSolWithSlippage(DepositSolWithSlippageIxProto {
                    accounts: Some(DepositSolWithSlippageAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[1].into_bytes().to_vec(),
                        reserve_stake_account: ix.accounts[2].into_bytes().to_vec(),
                        lamports_from: ix.accounts[3].into_bytes().to_vec(),
                        pool_tokens_to: ix.accounts[4].into_bytes().to_vec(),
                        manager_fee_account: ix.accounts[5].into_bytes().to_vec(),
                        referrer_pool_tokens_account: ix.accounts[6].into_bytes().to_vec(),
                        pool_mint: ix.accounts[7].into_bytes().to_vec(),
                        system_program: ix.accounts[8].into_bytes().to_vec(),
                        token_program: ix.accounts[9].into_bytes().to_vec(),
                        deposit_authority: ix.accounts.get(10).map(|a| a.into_bytes().to_vec()),
                    }),
                    args: Some(DepositSolWithSlippageArgsProto {
                        lamports_in,
                        minimum_pool_tokens_out,
                    }),
                })
            },

            StakePoolInstruction::WithdrawSolWithSlippage {
                pool_tokens_in,
                minimum_lamports_out,
            } => {
                check_min_accounts_req(accounts_len, 12)?;

                Out::WithdrawSolWithSlippage(WithdrawSolWithSlippageIxProto {
                    accounts: Some(WithdrawSolWithSlippageAccountsProto {
                        stake_pool: ix.accounts[0].into_bytes().to_vec(),
                        stake_pool_withdraw_authority: ix.accounts[1].into_bytes().to_vec(),
                        user_transfer_authority: ix.accounts[2].into_bytes().to_vec(),
                        pool_tokens_from: ix.accounts[3].into_bytes().to_vec(),
                        reserve_stake_account: ix.accounts[4].into_bytes().to_vec(),
                        lamports_to: ix.accounts[5].into_bytes().to_vec(),
                        manager_fee_account: ix.accounts[6].into_bytes().to_vec(),
                        pool_mint: ix.accounts[7].into_bytes().to_vec(),
                        clock: ix.accounts[8].into_bytes().to_vec(),
                        sysvar_stake_history: ix.accounts[9].into_bytes().to_vec(),
                        stake_program: ix.accounts[10].into_bytes().to_vec(),
                        token_program: ix.accounts[11].into_bytes().to_vec(),
                        sol_withdraw_authority: ix
                            .accounts
                            .get(12)
                            .map(|a| a.into_bytes().to_vec()),
                    }),
                    args: Some(WithdrawSolWithSlippageArgsProto {
                        pool_tokens_in,
                        minimum_lamports_out,
                    }),
                })
            },

            _ => {
                return Err(ParseError::from(
                    "Invalid Instruction discriminator".to_owned(),
                ));
            },
        };

        Ok(StakePoolProgramInstructionProto {
            instruction: Some(instruction),
        })
    }
}
