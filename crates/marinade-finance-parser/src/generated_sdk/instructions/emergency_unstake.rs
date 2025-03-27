//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct EmergencyUnstake {
    pub state: solana_program::pubkey::Pubkey,

    pub validator_manager_authority: solana_program::pubkey::Pubkey,

    pub validator_list: solana_program::pubkey::Pubkey,

    pub stake_list: solana_program::pubkey::Pubkey,

    pub stake_account: solana_program::pubkey::Pubkey,

    pub stake_deposit_authority: solana_program::pubkey::Pubkey,

    pub clock: solana_program::pubkey::Pubkey,

    pub stake_program: solana_program::pubkey::Pubkey,
}

impl EmergencyUnstake {
    pub fn instruction(
        &self,
        args: EmergencyUnstakeInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: EmergencyUnstakeInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.state, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.validator_manager_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.validator_list,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.stake_list,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.stake_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.stake_deposit_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.clock, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.stake_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&EmergencyUnstakeInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::MARINADE_FINANCE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmergencyUnstakeInstructionData {
    discriminator: [u8; 8],
}

impl EmergencyUnstakeInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [123, 69, 168, 195, 183, 213, 199, 214],
        }
    }
}

impl Default for EmergencyUnstakeInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmergencyUnstakeInstructionArgs {
    pub stake_index: u32,
    pub validator_index: u32,
}

/// Instruction builder for `EmergencyUnstake`.
///
/// ### Accounts:
///
///   0. `[writable]` state
///   1. `[signer]` validator_manager_authority
///   2. `[writable]` validator_list
///   3. `[writable]` stake_list
///   4. `[writable]` stake_account
///   5. `[]` stake_deposit_authority
///   6. `[]` clock
///   7. `[]` stake_program
#[derive(Clone, Debug, Default)]
pub struct EmergencyUnstakeBuilder {
    state: Option<solana_program::pubkey::Pubkey>,
    validator_manager_authority: Option<solana_program::pubkey::Pubkey>,
    validator_list: Option<solana_program::pubkey::Pubkey>,
    stake_list: Option<solana_program::pubkey::Pubkey>,
    stake_account: Option<solana_program::pubkey::Pubkey>,
    stake_deposit_authority: Option<solana_program::pubkey::Pubkey>,
    clock: Option<solana_program::pubkey::Pubkey>,
    stake_program: Option<solana_program::pubkey::Pubkey>,
    stake_index: Option<u32>,
    validator_index: Option<u32>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl EmergencyUnstakeBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn state(&mut self, state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn validator_manager_authority(
        &mut self,
        validator_manager_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.validator_manager_authority = Some(validator_manager_authority);
        self
    }

    #[inline(always)]
    pub fn validator_list(&mut self, validator_list: solana_program::pubkey::Pubkey) -> &mut Self {
        self.validator_list = Some(validator_list);
        self
    }

    #[inline(always)]
    pub fn stake_list(&mut self, stake_list: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake_list = Some(stake_list);
        self
    }

    #[inline(always)]
    pub fn stake_account(&mut self, stake_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake_account = Some(stake_account);
        self
    }

    #[inline(always)]
    pub fn stake_deposit_authority(
        &mut self,
        stake_deposit_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.stake_deposit_authority = Some(stake_deposit_authority);
        self
    }

    #[inline(always)]
    pub fn clock(&mut self, clock: solana_program::pubkey::Pubkey) -> &mut Self {
        self.clock = Some(clock);
        self
    }

    #[inline(always)]
    pub fn stake_program(&mut self, stake_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake_program = Some(stake_program);
        self
    }

    #[inline(always)]
    pub fn stake_index(&mut self, stake_index: u32) -> &mut Self {
        self.stake_index = Some(stake_index);
        self
    }

    #[inline(always)]
    pub fn validator_index(&mut self, validator_index: u32) -> &mut Self {
        self.validator_index = Some(validator_index);
        self
    }

    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }

    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }

    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = EmergencyUnstake {
            state: self.state.expect("state is not set"),
            validator_manager_authority: self
                .validator_manager_authority
                .expect("validator_manager_authority is not set"),
            validator_list: self.validator_list.expect("validator_list is not set"),
            stake_list: self.stake_list.expect("stake_list is not set"),
            stake_account: self.stake_account.expect("stake_account is not set"),
            stake_deposit_authority: self
                .stake_deposit_authority
                .expect("stake_deposit_authority is not set"),
            clock: self.clock.expect("clock is not set"),
            stake_program: self.stake_program.expect("stake_program is not set"),
        };
        let args = EmergencyUnstakeInstructionArgs {
            stake_index: self.stake_index.clone().expect("stake_index is not set"),
            validator_index: self
                .validator_index
                .clone()
                .expect("validator_index is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `emergency_unstake` CPI accounts.
pub struct EmergencyUnstakeCpiAccounts<'a, 'b> {
    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub validator_manager_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub validator_list: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_list: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_deposit_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub clock: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `emergency_unstake` CPI instruction.
pub struct EmergencyUnstakeCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub validator_manager_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub validator_list: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_list: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_deposit_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub clock: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: EmergencyUnstakeInstructionArgs,
}

impl<'a, 'b> EmergencyUnstakeCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: EmergencyUnstakeCpiAccounts<'a, 'b>,
        args: EmergencyUnstakeInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            state: accounts.state,
            validator_manager_authority: accounts.validator_manager_authority,
            validator_list: accounts.validator_list,
            stake_list: accounts.stake_list,
            stake_account: accounts.stake_account,
            stake_deposit_authority: accounts.stake_deposit_authority,
            clock: accounts.clock,
            stake_program: accounts.stake_program,
            __args: args,
        }
    }

    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }

    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }

    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.validator_manager_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.validator_list.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.stake_list.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.stake_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.stake_deposit_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.clock.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.stake_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&EmergencyUnstakeInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MARINADE_FINANCE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.state.clone());
        account_infos.push(self.validator_manager_authority.clone());
        account_infos.push(self.validator_list.clone());
        account_infos.push(self.stake_list.clone());
        account_infos.push(self.stake_account.clone());
        account_infos.push(self.stake_deposit_authority.clone());
        account_infos.push(self.clock.clone());
        account_infos.push(self.stake_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `EmergencyUnstake` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` state
///   1. `[signer]` validator_manager_authority
///   2. `[writable]` validator_list
///   3. `[writable]` stake_list
///   4. `[writable]` stake_account
///   5. `[]` stake_deposit_authority
///   6. `[]` clock
///   7. `[]` stake_program
#[derive(Clone, Debug)]
pub struct EmergencyUnstakeCpiBuilder<'a, 'b> {
    instruction: Box<EmergencyUnstakeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> EmergencyUnstakeCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(EmergencyUnstakeCpiBuilderInstruction {
            __program: program,
            state: None,
            validator_manager_authority: None,
            validator_list: None,
            stake_list: None,
            stake_account: None,
            stake_deposit_authority: None,
            clock: None,
            stake_program: None,
            stake_index: None,
            validator_index: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn state(&mut self, state: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn validator_manager_authority(
        &mut self,
        validator_manager_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.validator_manager_authority = Some(validator_manager_authority);
        self
    }

    #[inline(always)]
    pub fn validator_list(
        &mut self,
        validator_list: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.validator_list = Some(validator_list);
        self
    }

    #[inline(always)]
    pub fn stake_list(
        &mut self,
        stake_list: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_list = Some(stake_list);
        self
    }

    #[inline(always)]
    pub fn stake_account(
        &mut self,
        stake_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_account = Some(stake_account);
        self
    }

    #[inline(always)]
    pub fn stake_deposit_authority(
        &mut self,
        stake_deposit_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_deposit_authority = Some(stake_deposit_authority);
        self
    }

    #[inline(always)]
    pub fn clock(&mut self, clock: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.clock = Some(clock);
        self
    }

    #[inline(always)]
    pub fn stake_program(
        &mut self,
        stake_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_program = Some(stake_program);
        self
    }

    #[inline(always)]
    pub fn stake_index(&mut self, stake_index: u32) -> &mut Self {
        self.instruction.stake_index = Some(stake_index);
        self
    }

    #[inline(always)]
    pub fn validator_index(&mut self, validator_index: u32) -> &mut Self {
        self.instruction.validator_index = Some(validator_index);
        self
    }

    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
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
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }

    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult { self.invoke_signed(&[]) }

    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = EmergencyUnstakeInstructionArgs {
            stake_index: self
                .instruction
                .stake_index
                .clone()
                .expect("stake_index is not set"),
            validator_index: self
                .instruction
                .validator_index
                .clone()
                .expect("validator_index is not set"),
        };
        let instruction = EmergencyUnstakeCpi {
            __program: self.instruction.__program,

            state: self.instruction.state.expect("state is not set"),

            validator_manager_authority: self
                .instruction
                .validator_manager_authority
                .expect("validator_manager_authority is not set"),

            validator_list: self
                .instruction
                .validator_list
                .expect("validator_list is not set"),

            stake_list: self.instruction.stake_list.expect("stake_list is not set"),

            stake_account: self
                .instruction
                .stake_account
                .expect("stake_account is not set"),

            stake_deposit_authority: self
                .instruction
                .stake_deposit_authority
                .expect("stake_deposit_authority is not set"),

            clock: self.instruction.clock.expect("clock is not set"),

            stake_program: self
                .instruction
                .stake_program
                .expect("stake_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct EmergencyUnstakeCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    validator_manager_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    validator_list: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_list: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_deposit_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    clock: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_index: Option<u32>,
    validator_index: Option<u32>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
