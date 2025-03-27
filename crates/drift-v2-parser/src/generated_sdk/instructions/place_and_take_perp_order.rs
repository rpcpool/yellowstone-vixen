//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

use crate::generated::types::OrderParams;

/// Accounts.
#[derive(Debug)]
pub struct PlaceAndTakePerpOrder {
    pub state: solana_program::pubkey::Pubkey,

    pub user: solana_program::pubkey::Pubkey,

    pub user_stats: solana_program::pubkey::Pubkey,

    pub authority: solana_program::pubkey::Pubkey,
}

impl PlaceAndTakePerpOrder {
    pub fn instruction(
        &self,
        args: PlaceAndTakePerpOrderInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: PlaceAndTakePerpOrderInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&PlaceAndTakePerpOrderInstructionData::new()).unwrap();
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
pub struct PlaceAndTakePerpOrderInstructionData {
    discriminator: [u8; 8],
}

impl PlaceAndTakePerpOrderInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [213, 51, 1, 187, 108, 220, 230, 224],
        }
    }
}

impl Default for PlaceAndTakePerpOrderInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceAndTakePerpOrderInstructionArgs {
    pub params: OrderParams,
    pub success_condition: Option<u32>,
}

/// Instruction builder for `PlaceAndTakePerpOrder`.
///
/// ### Accounts:
///
///   0. `[]` state
///   1. `[writable]` user
///   2. `[writable]` user_stats
///   3. `[signer]` authority
#[derive(Clone, Debug, Default)]
pub struct PlaceAndTakePerpOrderBuilder {
    state: Option<solana_program::pubkey::Pubkey>,
    user: Option<solana_program::pubkey::Pubkey>,
    user_stats: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    params: Option<OrderParams>,
    success_condition: Option<u32>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl PlaceAndTakePerpOrderBuilder {
    pub fn new() -> Self { Self::default() }

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

    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }

    #[inline(always)]
    pub fn params(&mut self, params: OrderParams) -> &mut Self {
        self.params = Some(params);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn success_condition(&mut self, success_condition: u32) -> &mut Self {
        self.success_condition = Some(success_condition);
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
        let accounts = PlaceAndTakePerpOrder {
            state: self.state.expect("state is not set"),
            user: self.user.expect("user is not set"),
            user_stats: self.user_stats.expect("user_stats is not set"),
            authority: self.authority.expect("authority is not set"),
        };
        let args = PlaceAndTakePerpOrderInstructionArgs {
            params: self.params.clone().expect("params is not set"),
            success_condition: self.success_condition.clone(),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `place_and_take_perp_order` CPI accounts.
pub struct PlaceAndTakePerpOrderCpiAccounts<'a, 'b> {
    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_stats: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `place_and_take_perp_order` CPI instruction.
pub struct PlaceAndTakePerpOrderCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_stats: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: PlaceAndTakePerpOrderInstructionArgs,
}

impl<'a, 'b> PlaceAndTakePerpOrderCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: PlaceAndTakePerpOrderCpiAccounts<'a, 'b>,
        args: PlaceAndTakePerpOrderInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            state: accounts.state,
            user: accounts.user,
            user_stats: accounts.user_stats,
            authority: accounts.authority,
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&PlaceAndTakePerpOrderInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::DRIFT_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.state.clone());
        account_infos.push(self.user.clone());
        account_infos.push(self.user_stats.clone());
        account_infos.push(self.authority.clone());
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

/// Instruction builder for `PlaceAndTakePerpOrder` via CPI.
///
/// ### Accounts:
///
///   0. `[]` state
///   1. `[writable]` user
///   2. `[writable]` user_stats
///   3. `[signer]` authority
#[derive(Clone, Debug)]
pub struct PlaceAndTakePerpOrderCpiBuilder<'a, 'b> {
    instruction: Box<PlaceAndTakePerpOrderCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> PlaceAndTakePerpOrderCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(PlaceAndTakePerpOrderCpiBuilderInstruction {
            __program: program,
            state: None,
            user: None,
            user_stats: None,
            authority: None,
            params: None,
            success_condition: None,
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

    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }

    #[inline(always)]
    pub fn params(&mut self, params: OrderParams) -> &mut Self {
        self.instruction.params = Some(params);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn success_condition(&mut self, success_condition: u32) -> &mut Self {
        self.instruction.success_condition = Some(success_condition);
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
        let args = PlaceAndTakePerpOrderInstructionArgs {
            params: self.instruction.params.clone().expect("params is not set"),
            success_condition: self.instruction.success_condition.clone(),
        };
        let instruction = PlaceAndTakePerpOrderCpi {
            __program: self.instruction.__program,

            state: self.instruction.state.expect("state is not set"),

            user: self.instruction.user.expect("user is not set"),

            user_stats: self.instruction.user_stats.expect("user_stats is not set"),

            authority: self.instruction.authority.expect("authority is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct PlaceAndTakePerpOrderCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_stats: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    params: Option<OrderParams>,
    success_condition: Option<u32>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
