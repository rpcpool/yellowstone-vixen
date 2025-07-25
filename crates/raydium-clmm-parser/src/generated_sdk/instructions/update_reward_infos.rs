//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct UpdateRewardInfos {
    pub pool_state: solana_pubkey::Pubkey,
}

impl UpdateRewardInfos {
    pub fn instruction(&self) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(1 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(self.pool_state, false));
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&UpdateRewardInfosInstructionData::new()).unwrap();

        solana_instruction::Instruction {
            program_id: crate::AMM_V3_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateRewardInfosInstructionData {
    discriminator: [u8; 8],
}

impl UpdateRewardInfosInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [163, 172, 224, 52, 11, 154, 106, 223],
        }
    }
}

impl Default for UpdateRewardInfosInstructionData {
    fn default() -> Self { Self::new() }
}

/// Instruction builder for `UpdateRewardInfos`.
///
/// ### Accounts:
///
///   0. `[writable]` pool_state
#[derive(Clone, Debug, Default)]
pub struct UpdateRewardInfosBuilder {
    pool_state: Option<solana_pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl UpdateRewardInfosBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn pool_state(&mut self, pool_state: solana_pubkey::Pubkey) -> &mut Self {
        self.pool_state = Some(pool_state);
        self
    }

    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(&mut self, account: solana_instruction::AccountMeta) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }

    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }

    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_instruction::Instruction {
        let accounts = UpdateRewardInfos {
            pool_state: self.pool_state.expect("pool_state is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `update_reward_infos` CPI accounts.
pub struct UpdateRewardInfosCpiAccounts<'a, 'b> {
    pub pool_state: &'b solana_account_info::AccountInfo<'a>,
}

/// `update_reward_infos` CPI instruction.
pub struct UpdateRewardInfosCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_account_info::AccountInfo<'a>,

    pub pool_state: &'b solana_account_info::AccountInfo<'a>,
}

impl<'a, 'b> UpdateRewardInfosCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: UpdateRewardInfosCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            pool_state: accounts.pool_state,
        }
    }

    #[inline(always)]
    pub fn invoke(&self) -> solana_program_entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }

    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(&'b solana_account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program_entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }

    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program_entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(&'b solana_account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program_entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(1 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(
            *self.pool_state.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = borsh::to_vec(&UpdateRewardInfosInstructionData::new()).unwrap();

        let instruction = solana_instruction::Instruction {
            program_id: crate::AMM_V3_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(2 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.pool_state.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_cpi::invoke(&instruction, &account_infos)
        } else {
            solana_cpi::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `UpdateRewardInfos` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` pool_state
#[derive(Clone, Debug)]
pub struct UpdateRewardInfosCpiBuilder<'a, 'b> {
    instruction: Box<UpdateRewardInfosCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdateRewardInfosCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UpdateRewardInfosCpiBuilderInstruction {
            __program: program,
            pool_state: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn pool_state(
        &mut self,
        pool_state: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.pool_state = Some(pool_state);
        self
    }

    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }

    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(&'b solana_account_info::AccountInfo<'a>, bool, bool)],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }

    #[inline(always)]
    pub fn invoke(&self) -> solana_program_entrypoint::ProgramResult { self.invoke_signed(&[]) }

    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program_entrypoint::ProgramResult {
        let instruction = UpdateRewardInfosCpi {
            __program: self.instruction.__program,

            pool_state: self.instruction.pool_state.expect("pool_state is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct UpdateRewardInfosCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    pool_state: Option<&'b solana_account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
