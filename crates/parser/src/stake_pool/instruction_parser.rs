use borsh::BorshDeserialize;
use spl_stake_pool::instruction::StakePoolInstruction;
use spl_token_2022::extension::confidential_transfer::instruction::deposit;
use yellowstone_vixen_core::ParseError;

use super::instruction_helpers::{
    AddValidatorToPoolAccounts, AddValidatorToPoolData, InitializeAccounts, InitializeData,
    StakePoolProgramIx,
};

#[derive(Debug, Copy, Clone)]
pub struct InstructionParser;

impl yellowstone_vixen_core::Parser for InstructionParser {
    type Input = yellowstone_vixen_core::instruction::InstructionUpdate;
    type Output = StakePoolProgramIx;

    fn id(&self) -> std::borrow::Cow<str> {
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
        let accounts_len = ix.accounts.len();
        let ix_discriminator: [u8; 8] = ix.data[0..8].try_into()?;
        let mut ix_data = &ix.data[8..];

        let ix_type = StakePoolInstruction::deserialize(&mut ix.data.as_slice()).unwrap();
        let accounts_len = ix.accounts.len();

        match ix_discriminator {
            [175, 175, 109, 31, 13, 152, 155, 237] => {
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

                let de_ix_data: InitializeData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(StakePoolProgramIx::Initialize(ix_accounts, de_ix_data))
            },
            [181, 6, 29, 25, 192, 211, 190, 187] => {
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
                    BorshDeserialize::deserialize(&mut ix_data)?;

                Ok(StakePoolProgramIx::AddValidatorToPool(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            [161, 32, 213, 239, 221, 15, 181, 114] => {
                check_min_accounts_req(accounts_len, 10)?;
                let ix_accounts = RemoveValidatorFromPoolIxAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    staker: ix.accounts[1].0.into(),
                    stake_pool_withdraw: ix.accounts[2].0.into(),
                    new_stake_authority: ix.accounts[3].0.into(),
                    validator_list: ix.accounts[4].0.into(),
                    stake_account: ix.accounts[5].0.into(),
                    transient_stake_account: ix.accounts[6].0.into(),
                    destination_stake_account: ix.accounts[7].0.into(),
                    clock: ix.accounts[8].0.into(),
                    stake_program: ix.accounts[9].0.into(),
                };
                Ok(SplStakePoolProgramIx::RemoveValidatorFromPool(ix_accounts))
            },
            [145, 203, 107, 123, 71, 63, 35, 225] => {
                check_min_accounts_req(accounts_len, 10)?;
                let ix_accounts = DecreaseValidatorStakeIxAccounts {
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
                let de_ix_data: DecreaseValidatorStakeIxData =
                    BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(SplStakePoolProgramIx::DecreaseValidatorStake(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            [5, 121, 50, 243, 14, 159, 97, 6] => {
                check_min_accounts_req(accounts_len, 14)?;
                let ix_accounts = IncreaseValidatorStakeIxAccounts {
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
                let de_ix_data: IncreaseValidatorStakeIxData =
                    BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(SplStakePoolProgramIx::IncreaseValidatorStake(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            [114, 42, 19, 98, 212, 97, 109, 13] => {
                check_min_accounts_req(accounts_len, 3)?;
                let ix_accounts = SetPreferredValidatorIxAccounts {
                    stake_pool_address: ix.accounts[0].0.into(),
                    staker: ix.accounts[1].0.into(),
                    validator_list_address: ix.accounts[2].0.into(),
                };
                let de_ix_data: SetPreferredValidatorIxData =
                    BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(SplStakePoolProgramIx::SetPreferredValidator(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            [98, 93, 78, 124, 109, 4, 165, 194] => {
                check_min_accounts_req(accounts_len, 7)?;
                let ix_accounts = UpdateValidatorListBalanceIxAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[1].0.into(),
                    validator_list_address: ix.accounts[2].0.into(),
                    reserve_stake: ix.accounts[3].0.into(),
                    clock: ix.accounts[4].0.into(),
                    sysvar_stake_history: ix.accounts[5].0.into(),
                    stake_program: ix.accounts[6].0.into(),
                };
                let de_ix_data: UpdateValidatorListBalanceIxData =
                    BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(SplStakePoolProgramIx::UpdateValidatorListBalance(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            [238, 181, 59, 245, 177, 236, 231, 88] => {
                check_min_accounts_req(accounts_len, 7)?;
                let ix_accounts = UpdateStakePoolBalanceIxAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    withdraw_authority: ix.accounts[1].0.into(),
                    validator_list_storage: ix.accounts[2].0.into(),
                    reserve_stake: ix.accounts[3].0.into(),
                    manager_fee_account: ix.accounts[4].0.into(),
                    stake_pool_mint: ix.accounts[5].0.into(),
                    token_program: ix.accounts[6].0.into(),
                };
                Ok(SplStakePoolProgramIx::UpdateStakePoolBalance(ix_accounts))
            },
            [211, 101, 162, 27, 244, 149, 45, 88] => {
                check_min_accounts_req(accounts_len, 2)?;
                let ix_accounts = CleanupRemovedValidatorEntriesIxAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    validator_list_storage: ix.accounts[1].0.into(),
                };
                Ok(SplStakePoolProgramIx::CleanupRemovedValidatorEntries(
                    ix_accounts,
                ))
            },
            [160, 167, 9, 220, 74, 243, 228, 43] => {
                check_min_accounts_req(accounts_len, 15)?;
                let ix_accounts = DepositStakeIxAccounts {
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
                Ok(SplStakePoolProgramIx::DepositStake(ix_accounts))
            },
            [153, 8, 22, 138, 105, 176, 87, 66] => {
                check_min_accounts_req(accounts_len, 13)?;
                let ix_accounts = WithdrawStakeIxAccounts {
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
                let de_ix_data: WithdrawStakeIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(SplStakePoolProgramIx::WithdrawStake(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            [30, 197, 171, 92, 121, 184, 151, 165] => {
                check_min_accounts_req(accounts_len, 4)?;
                let ix_accounts = SetManagerIxAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    manager: ix.accounts[1].0.into(),
                    new_manager: ix.accounts[2].0.into(),
                    new_fee_receiver: ix.accounts[3].0.into(),
                };
                Ok(SplStakePoolProgramIx::SetManager(ix_accounts))
            },
            [18, 154, 24, 18, 237, 214, 19, 80] => {
                check_min_accounts_req(accounts_len, 2)?;
                let ix_accounts = SetFeeIxAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    manager: ix.accounts[1].0.into(),
                };
                let de_ix_data: SetFeeIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(SplStakePoolProgramIx::SetFee(ix_accounts, de_ix_data))
            },
            [149, 203, 114, 28, 80, 138, 17, 131] => {
                check_min_accounts_req(accounts_len, 3)?;
                let ix_accounts = SetStakerIxAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    set_staker_authority: ix.accounts[1].0.into(),
                    new_staker: ix.accounts[2].0.into(),
                };
                Ok(SplStakePoolProgramIx::SetStaker(ix_accounts))
            },
            [108, 81, 78, 117, 125, 155, 56, 200] => {
                check_min_accounts_req(accounts_len, 10)?;
                let ix_accounts = DepositSolIxAccounts {
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
                };
                let de_ix_data: DepositSolIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(SplStakePoolProgramIx::DepositSol(ix_accounts, de_ix_data))
            },
            [48, 2, 114, 83, 165, 222, 71, 233] => {
                check_min_accounts_req(accounts_len, 2)?;
                let ix_accounts = SetFundingAuthorityIxAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    manager: ix.accounts[1].0.into(),
                };
                let de_ix_data: SetFundingAuthorityIxData =
                    BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(SplStakePoolProgramIx::SetFundingAuthority(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            [145, 131, 74, 136, 65, 137, 42, 38] => {
                check_min_accounts_req(accounts_len, 12)?;
                let ix_accounts = WithdrawSolIxAccounts {
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
                };
                let de_ix_data: WithdrawSolIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(SplStakePoolProgramIx::WithdrawSol(ix_accounts, de_ix_data))
            },
            [221, 80, 176, 37, 153, 188, 160, 68] => {
                check_min_accounts_req(accounts_len, 9)?;
                let ix_accounts = CreateTokenMetadataIxAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    manager: ix.accounts[1].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[2].0.into(),
                    pool_mint: ix.accounts[3].0.into(),
                    payer: ix.accounts[4].0.into(),
                    token_metadata: ix.accounts[5].0.into(),
                    mpl_token_metadata: ix.accounts[6].0.into(),
                    system_program: ix.accounts[7].0.into(),
                    rent: ix.accounts[8].0.into(),
                };
                let de_ix_data: CreateTokenMetadataIxData =
                    BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(SplStakePoolProgramIx::CreateTokenMetadata(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            [243, 6, 8, 23, 126, 181, 251, 158] => {
                check_min_accounts_req(accounts_len, 5)?;
                let ix_accounts = UpdateTokenMetadataIxAccounts {
                    stake_pool: ix.accounts[0].0.into(),
                    manager: ix.accounts[1].0.into(),
                    stake_pool_withdraw_authority: ix.accounts[2].0.into(),
                    token_metadata: ix.accounts[3].0.into(),
                    mpl_token_metadata: ix.accounts[4].0.into(),
                };
                let de_ix_data: UpdateTokenMetadataIxData =
                    BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(SplStakePoolProgramIx::UpdateTokenMetadata(
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
