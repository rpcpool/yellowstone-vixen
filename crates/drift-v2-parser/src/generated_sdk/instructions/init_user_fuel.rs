//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct InitUserFuel {
    pub admin: solana_program::pubkey::Pubkey,

    pub state: solana_program::pubkey::Pubkey,

    pub user: solana_program::pubkey::Pubkey,

    pub user_stats: solana_program::pubkey::Pubkey,
}

impl InitUserFuel {
    pub fn instruction(
        &self,
        args: InitUserFuelInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitUserFuelInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.admin, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.state, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_stats,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&InitUserFuelInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::DRIFT_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitUserFuelInstructionData {
    discriminator: [u8; 8],
}

impl InitUserFuelInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [132, 191, 228, 141, 201, 138, 60, 48],
        }
    }
}

impl Default for InitUserFuelInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitUserFuelInstructionArgs {
    pub fuel_boost_deposits: Option<i32>,
    pub fuel_boost_borrows: Option<u32>,
    pub fuel_boost_taker: Option<u32>,
    pub fuel_boost_maker: Option<u32>,
    pub fuel_boost_insurance: Option<u32>,
}

/// Instruction builder for `InitUserFuel`.
///
/// ### Accounts:
///
///   0. `[signer]` admin
///   1. `[]` state
///   2. `[writable]` user
///   3. `[writable]` user_stats
#[derive(Clone, Debug, Default)]
pub struct InitUserFuelBuilder {
    admin: Option<solana_program::pubkey::Pubkey>,
    state: Option<solana_program::pubkey::Pubkey>,
    user: Option<solana_program::pubkey::Pubkey>,
    user_stats: Option<solana_program::pubkey::Pubkey>,
    fuel_boost_deposits: Option<i32>,
    fuel_boost_borrows: Option<u32>,
    fuel_boost_taker: Option<u32>,
    fuel_boost_maker: Option<u32>,
    fuel_boost_insurance: Option<u32>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitUserFuelBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn admin(&mut self, admin: solana_program::pubkey::Pubkey) -> &mut Self {
        self.admin = Some(admin);
        self
    }

    #[inline(always)]
    pub fn state(&mut self, state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn user(&mut self, user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user = Some(user);
        self
    }

    #[inline(always)]
    pub fn user_stats(&mut self, user_stats: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user_stats = Some(user_stats);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn fuel_boost_deposits(&mut self, fuel_boost_deposits: i32) -> &mut Self {
        self.fuel_boost_deposits = Some(fuel_boost_deposits);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn fuel_boost_borrows(&mut self, fuel_boost_borrows: u32) -> &mut Self {
        self.fuel_boost_borrows = Some(fuel_boost_borrows);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn fuel_boost_taker(&mut self, fuel_boost_taker: u32) -> &mut Self {
        self.fuel_boost_taker = Some(fuel_boost_taker);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn fuel_boost_maker(&mut self, fuel_boost_maker: u32) -> &mut Self {
        self.fuel_boost_maker = Some(fuel_boost_maker);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn fuel_boost_insurance(&mut self, fuel_boost_insurance: u32) -> &mut Self {
        self.fuel_boost_insurance = Some(fuel_boost_insurance);
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
        let accounts = InitUserFuel {
            admin: self.admin.expect("admin is not set"),
            state: self.state.expect("state is not set"),
            user: self.user.expect("user is not set"),
            user_stats: self.user_stats.expect("user_stats is not set"),
        };
        let args = InitUserFuelInstructionArgs {
            fuel_boost_deposits: self.fuel_boost_deposits.clone(),
            fuel_boost_borrows: self.fuel_boost_borrows.clone(),
            fuel_boost_taker: self.fuel_boost_taker.clone(),
            fuel_boost_maker: self.fuel_boost_maker.clone(),
            fuel_boost_insurance: self.fuel_boost_insurance.clone(),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `init_user_fuel` CPI accounts.
pub struct InitUserFuelCpiAccounts<'a, 'b> {
    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_stats: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `init_user_fuel` CPI instruction.
pub struct InitUserFuelCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_stats: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitUserFuelInstructionArgs,
}

impl<'a, 'b> InitUserFuelCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitUserFuelCpiAccounts<'a, 'b>,
        args: InitUserFuelInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            admin: accounts.admin,
            state: accounts.state,
            user: accounts.user,
            user_stats: accounts.user_stats,
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
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.admin.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_stats.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&InitUserFuelInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::DRIFT_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.admin.clone());
        account_infos.push(self.state.clone());
        account_infos.push(self.user.clone());
        account_infos.push(self.user_stats.clone());
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

/// Instruction builder for `InitUserFuel` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` admin
///   1. `[]` state
///   2. `[writable]` user
///   3. `[writable]` user_stats
#[derive(Clone, Debug)]
pub struct InitUserFuelCpiBuilder<'a, 'b> {
    instruction: Box<InitUserFuelCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitUserFuelCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitUserFuelCpiBuilderInstruction {
            __program: program,
            admin: None,
            state: None,
            user: None,
            user_stats: None,
            fuel_boost_deposits: None,
            fuel_boost_borrows: None,
            fuel_boost_taker: None,
            fuel_boost_maker: None,
            fuel_boost_insurance: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn admin(&mut self, admin: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.admin = Some(admin);
        self
    }

    #[inline(always)]
    pub fn state(&mut self, state: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn user(&mut self, user: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.user = Some(user);
        self
    }

    #[inline(always)]
    pub fn user_stats(
        &mut self,
        user_stats: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_stats = Some(user_stats);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn fuel_boost_deposits(&mut self, fuel_boost_deposits: i32) -> &mut Self {
        self.instruction.fuel_boost_deposits = Some(fuel_boost_deposits);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn fuel_boost_borrows(&mut self, fuel_boost_borrows: u32) -> &mut Self {
        self.instruction.fuel_boost_borrows = Some(fuel_boost_borrows);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn fuel_boost_taker(&mut self, fuel_boost_taker: u32) -> &mut Self {
        self.instruction.fuel_boost_taker = Some(fuel_boost_taker);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn fuel_boost_maker(&mut self, fuel_boost_maker: u32) -> &mut Self {
        self.instruction.fuel_boost_maker = Some(fuel_boost_maker);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn fuel_boost_insurance(&mut self, fuel_boost_insurance: u32) -> &mut Self {
        self.instruction.fuel_boost_insurance = Some(fuel_boost_insurance);
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
        let args = InitUserFuelInstructionArgs {
            fuel_boost_deposits: self.instruction.fuel_boost_deposits.clone(),
            fuel_boost_borrows: self.instruction.fuel_boost_borrows.clone(),
            fuel_boost_taker: self.instruction.fuel_boost_taker.clone(),
            fuel_boost_maker: self.instruction.fuel_boost_maker.clone(),
            fuel_boost_insurance: self.instruction.fuel_boost_insurance.clone(),
        };
        let instruction = InitUserFuelCpi {
            __program: self.instruction.__program,

            admin: self.instruction.admin.expect("admin is not set"),

            state: self.instruction.state.expect("state is not set"),

            user: self.instruction.user.expect("user is not set"),

            user_stats: self.instruction.user_stats.expect("user_stats is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitUserFuelCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_stats: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    fuel_boost_deposits: Option<i32>,
    fuel_boost_borrows: Option<u32>,
    fuel_boost_taker: Option<u32>,
    fuel_boost_maker: Option<u32>,
    fuel_boost_insurance: Option<u32>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
