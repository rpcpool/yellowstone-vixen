//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;

use crate::{
    instructions::{
        AcceptOwner as AcceptOwnerIxAccounts, ApproveStrategy as ApproveStrategyIxAccounts,
        ChangeAmpFactor as ChangeAmpFactorIxAccounts,
        ChangeAmpFactorInstructionArgs as ChangeAmpFactorIxData,
        ChangeMaxSupply as ChangeMaxSupplyIxAccounts,
        ChangeMaxSupplyInstructionArgs as ChangeMaxSupplyIxData,
        ChangeSwapFee as ChangeSwapFeeIxAccounts,
        ChangeSwapFeeInstructionArgs as ChangeSwapFeeIxData,
        CreateStrategy as CreateStrategyIxAccounts,
        CreateStrategyInstructionArgs as CreateStrategyIxData, Deposit as DepositIxAccounts,
        DepositInstructionArgs as DepositIxData, ExecStrategy as ExecStrategyIxAccounts,
        ExecStrategyInstructionArgs as ExecStrategyIxData, Initialize as InitializeIxAccounts,
        InitializeInstructionArgs as InitializeIxData, Pause as PauseIxAccounts,
        RejectOwner as RejectOwnerIxAccounts, Shutdown as ShutdownIxAccounts,
        Swap as SwapIxAccounts, SwapInstructionArgs as SwapIxData, SwapV2 as SwapV2IxAccounts,
        SwapV2InstructionArgs as SwapV2IxData, TransferOwner as TransferOwnerIxAccounts,
        TransferOwnerInstructionArgs as TransferOwnerIxData, Unpause as UnpauseIxAccounts,
        Withdraw as WithdrawIxAccounts, WithdrawInstructionArgs as WithdrawIxData,
    },
    ID,
};

/// StableSwap Instructions
#[derive(Debug)]
pub enum StableSwapProgramIx {
    AcceptOwner(AcceptOwnerIxAccounts),
    ApproveStrategy(ApproveStrategyIxAccounts),
    ChangeAmpFactor(ChangeAmpFactorIxAccounts, ChangeAmpFactorIxData),
    ChangeMaxSupply(ChangeMaxSupplyIxAccounts, ChangeMaxSupplyIxData),
    ChangeSwapFee(ChangeSwapFeeIxAccounts, ChangeSwapFeeIxData),
    CreateStrategy(CreateStrategyIxAccounts, CreateStrategyIxData),
    Deposit(DepositIxAccounts, DepositIxData),
    ExecStrategy(ExecStrategyIxAccounts, ExecStrategyIxData),
    Initialize(InitializeIxAccounts, InitializeIxData),
    Pause(PauseIxAccounts),
    RejectOwner(RejectOwnerIxAccounts),
    Shutdown(ShutdownIxAccounts),
    Swap(SwapIxAccounts, SwapIxData),
    SwapV2(SwapV2IxAccounts, SwapV2IxData),
    TransferOwner(TransferOwnerIxAccounts, TransferOwnerIxData),
    Unpause(UnpauseIxAccounts),
    Withdraw(WithdrawIxAccounts, WithdrawIxData),
}

#[derive(Debug, Copy, Clone)]
pub struct InstructionParser;

impl yellowstone_vixen_core::Parser for InstructionParser {
    type Input = yellowstone_vixen_core::instruction::InstructionUpdate;
    type Output = StableSwapProgramIx;

    fn id(&self) -> std::borrow::Cow<str> { "StableSwap::InstructionParser".into() }

    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
        yellowstone_vixen_core::Prefilter::builder()
            .transaction_accounts([ID])
            .build()
            .unwrap()
    }

    async fn parse(
        &self,
        ix_update: &yellowstone_vixen_core::instruction::InstructionUpdate,
    ) -> yellowstone_vixen_core::ParseResult<Self::Output> {
        if ix_update.program.equals_ref(ID) {
            InstructionParser::parse_impl(ix_update)
        } else {
            Err(yellowstone_vixen_core::ParseError::Filtered)
        }
    }
}

impl yellowstone_vixen_core::ProgramParser for InstructionParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { ID.to_bytes().into() }
}

impl InstructionParser {
    pub(crate) fn parse_impl(
        ix: &yellowstone_vixen_core::instruction::InstructionUpdate,
    ) -> yellowstone_vixen_core::ParseResult<StableSwapProgramIx> {
        let accounts_len = ix.accounts.len();
        let ix_discriminator: [u8; 8] = ix.data[0..8].try_into()?;
        let mut ix_data = &ix.data[8..];
        match ix_discriminator {
            [176, 23, 41, 28, 23, 111, 8, 4] => {
                check_min_accounts_req(accounts_len, 2)?;
                let ix_accounts = AcceptOwnerIxAccounts {
                    pending_owner: ix.accounts[0].0.into(),
                    pool: ix.accounts[1].0.into(),
                };
                Ok(StableSwapProgramIx::AcceptOwner(ix_accounts))
            },
            [7, 141, 162, 60, 71, 115, 26, 146] => {
                check_min_accounts_req(accounts_len, 4)?;
                let ix_accounts = ApproveStrategyIxAccounts {
                    pool: ix.accounts[0].0.into(),
                    vault: ix.accounts[1].0.into(),
                    admin: ix.accounts[2].0.into(),
                    strategy: ix.accounts[3].0.into(),
                };
                Ok(StableSwapProgramIx::ApproveStrategy(ix_accounts))
            },
            [56, 238, 189, 35, 200, 157, 42, 66] => {
                check_min_accounts_req(accounts_len, 3)?;
                let ix_accounts = ChangeAmpFactorIxAccounts {
                    pool: ix.accounts[0].0.into(),
                    vault: ix.accounts[1].0.into(),
                    admin: ix.accounts[2].0.into(),
                };
                let de_ix_data: ChangeAmpFactorIxData =
                    BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(StableSwapProgramIx::ChangeAmpFactor(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            [93, 176, 0, 205, 69, 63, 87, 80] => {
                check_min_accounts_req(accounts_len, 2)?;
                let ix_accounts = ChangeMaxSupplyIxAccounts {
                    owner: ix.accounts[0].0.into(),
                    pool: ix.accounts[1].0.into(),
                };
                let de_ix_data: ChangeMaxSupplyIxData =
                    BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(StableSwapProgramIx::ChangeMaxSupply(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            [231, 15, 132, 51, 132, 165, 64, 170] => {
                check_min_accounts_req(accounts_len, 2)?;
                let ix_accounts = ChangeSwapFeeIxAccounts {
                    owner: ix.accounts[0].0.into(),
                    pool: ix.accounts[1].0.into(),
                };
                let de_ix_data: ChangeSwapFeeIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(StableSwapProgramIx::ChangeSwapFee(ix_accounts, de_ix_data))
            },
            [152, 160, 107, 148, 245, 190, 127, 224] => {
                check_min_accounts_req(accounts_len, 3)?;
                let ix_accounts = CreateStrategyIxAccounts {
                    owner: ix.accounts[0].0.into(),
                    pool: ix.accounts[1].0.into(),
                    strategy: ix.accounts[2].0.into(),
                };
                let de_ix_data: CreateStrategyIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(StableSwapProgramIx::CreateStrategy(ix_accounts, de_ix_data))
            },
            [242, 35, 198, 137, 82, 225, 242, 182] => {
                check_min_accounts_req(accounts_len, 9)?;
                let ix_accounts = DepositIxAccounts {
                    user: ix.accounts[0].0.into(),
                    user_pool_token: ix.accounts[1].0.into(),
                    mint: ix.accounts[2].0.into(),
                    pool: ix.accounts[3].0.into(),
                    pool_authority: ix.accounts[4].0.into(),
                    vault: ix.accounts[5].0.into(),
                    vault_authority: ix.accounts[6].0.into(),
                    token_program: ix.accounts[7].0.into(),
                    token_program2022: ix.accounts[8].0.into(),
                };
                let de_ix_data: DepositIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(StableSwapProgramIx::Deposit(ix_accounts, de_ix_data))
            },
            [249, 46, 55, 57, 31, 38, 61, 27] => {
                check_min_accounts_req(accounts_len, 2)?;
                let ix_accounts = ExecStrategyIxAccounts {
                    strategy: ix.accounts[0].0.into(),
                    pool: ix.accounts[1].0.into(),
                };
                let de_ix_data: ExecStrategyIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(StableSwapProgramIx::ExecStrategy(ix_accounts, de_ix_data))
            },
            [175, 175, 109, 31, 13, 152, 155, 237] => {
                check_min_accounts_req(accounts_len, 6)?;
                let ix_accounts = InitializeIxAccounts {
                    owner: ix.accounts[0].0.into(),
                    mint: ix.accounts[1].0.into(),
                    pool: ix.accounts[2].0.into(),
                    pool_authority: ix.accounts[3].0.into(),
                    withdraw_authority: ix.accounts[4].0.into(),
                    vault: ix.accounts[5].0.into(),
                };
                let de_ix_data: InitializeIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(StableSwapProgramIx::Initialize(ix_accounts, de_ix_data))
            },
            [211, 22, 221, 251, 74, 121, 193, 47] => {
                check_min_accounts_req(accounts_len, 2)?;
                let ix_accounts = PauseIxAccounts {
                    owner: ix.accounts[0].0.into(),
                    pool: ix.accounts[1].0.into(),
                };
                Ok(StableSwapProgramIx::Pause(ix_accounts))
            },
            [238, 206, 198, 215, 51, 178, 133, 228] => {
                check_min_accounts_req(accounts_len, 2)?;
                let ix_accounts = RejectOwnerIxAccounts {
                    pending_owner: ix.accounts[0].0.into(),
                    pool: ix.accounts[1].0.into(),
                };
                Ok(StableSwapProgramIx::RejectOwner(ix_accounts))
            },
            [146, 204, 241, 213, 86, 21, 253, 211] => {
                check_min_accounts_req(accounts_len, 2)?;
                let ix_accounts = ShutdownIxAccounts {
                    owner: ix.accounts[0].0.into(),
                    pool: ix.accounts[1].0.into(),
                };
                Ok(StableSwapProgramIx::Shutdown(ix_accounts))
            },
            [248, 198, 158, 145, 225, 117, 135, 200] => {
                check_min_accounts_req(accounts_len, 12)?;
                let ix_accounts = SwapIxAccounts {
                    user: ix.accounts[0].0.into(),
                    user_token_in: ix.accounts[1].0.into(),
                    user_token_out: ix.accounts[2].0.into(),
                    vault_token_in: ix.accounts[3].0.into(),
                    vault_token_out: ix.accounts[4].0.into(),
                    beneficiary_token_out: ix.accounts[5].0.into(),
                    pool: ix.accounts[6].0.into(),
                    withdraw_authority: ix.accounts[7].0.into(),
                    vault: ix.accounts[8].0.into(),
                    vault_authority: ix.accounts[9].0.into(),
                    vault_program: ix.accounts[10].0.into(),
                    token_program: ix.accounts[11].0.into(),
                };
                let de_ix_data: SwapIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(StableSwapProgramIx::Swap(ix_accounts, de_ix_data))
            },
            [43, 4, 237, 11, 26, 201, 30, 98] => {
                check_min_accounts_req(accounts_len, 15)?;
                let ix_accounts = SwapV2IxAccounts {
                    user: ix.accounts[0].0.into(),
                    mint_in: ix.accounts[1].0.into(),
                    mint_out: ix.accounts[2].0.into(),
                    user_token_in: ix.accounts[3].0.into(),
                    user_token_out: ix.accounts[4].0.into(),
                    vault_token_in: ix.accounts[5].0.into(),
                    vault_token_out: ix.accounts[6].0.into(),
                    beneficiary_token_out: ix.accounts[7].0.into(),
                    pool: ix.accounts[8].0.into(),
                    withdraw_authority: ix.accounts[9].0.into(),
                    vault: ix.accounts[10].0.into(),
                    vault_authority: ix.accounts[11].0.into(),
                    vault_program: ix.accounts[12].0.into(),
                    token_program: ix.accounts[13].0.into(),
                    token2022_program: ix.accounts[14].0.into(),
                };
                let de_ix_data: SwapV2IxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(StableSwapProgramIx::SwapV2(ix_accounts, de_ix_data))
            },
            [245, 25, 221, 175, 106, 229, 225, 45] => {
                check_min_accounts_req(accounts_len, 2)?;
                let ix_accounts = TransferOwnerIxAccounts {
                    owner: ix.accounts[0].0.into(),
                    pool: ix.accounts[1].0.into(),
                };
                let de_ix_data: TransferOwnerIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(StableSwapProgramIx::TransferOwner(ix_accounts, de_ix_data))
            },
            [169, 144, 4, 38, 10, 141, 188, 255] => {
                check_min_accounts_req(accounts_len, 2)?;
                let ix_accounts = UnpauseIxAccounts {
                    owner: ix.accounts[0].0.into(),
                    pool: ix.accounts[1].0.into(),
                };
                Ok(StableSwapProgramIx::Unpause(ix_accounts))
            },
            [183, 18, 70, 156, 148, 109, 161, 34] => {
                check_min_accounts_req(accounts_len, 10)?;
                let ix_accounts = WithdrawIxAccounts {
                    user: ix.accounts[0].0.into(),
                    user_pool_token: ix.accounts[1].0.into(),
                    mint: ix.accounts[2].0.into(),
                    pool: ix.accounts[3].0.into(),
                    withdraw_authority: ix.accounts[4].0.into(),
                    vault: ix.accounts[5].0.into(),
                    vault_authority: ix.accounts[6].0.into(),
                    vault_program: ix.accounts[7].0.into(),
                    token_program: ix.accounts[8].0.into(),
                    token_program2022: ix.accounts[9].0.into(),
                };
                let de_ix_data: WithdrawIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(StableSwapProgramIx::Withdraw(ix_accounts, de_ix_data))
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
