//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct RefreshUserState {
    pub user_state: solana_program::pubkey::Pubkey,

    pub farm_state: solana_program::pubkey::Pubkey,

    pub scope_prices: Option<solana_program::pubkey::Pubkey>,
}

impl RefreshUserState {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_state,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.farm_state,
            false,
        ));
        if let Some(scope_prices) = self.scope_prices {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                scope_prices,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::FARMS_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&RefreshUserStateInstructionData::new()).unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::FARMS_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RefreshUserStateInstructionData {
    discriminator: [u8; 8],
}

impl RefreshUserStateInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [1, 135, 12, 62, 243, 140, 77, 108],
        }
    }
}

impl Default for RefreshUserStateInstructionData {
    fn default() -> Self { Self::new() }
}

/// Instruction builder for `RefreshUserState`.
///
/// ### Accounts:
///
///   0. `[writable]` user_state
///   1. `[writable]` farm_state
///   2. `[optional]` scope_prices
#[derive(Clone, Debug, Default)]
pub struct RefreshUserStateBuilder {
    user_state: Option<solana_program::pubkey::Pubkey>,
    farm_state: Option<solana_program::pubkey::Pubkey>,
    scope_prices: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl RefreshUserStateBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn user_state(&mut self, user_state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user_state = Some(user_state);
        self
    }

    #[inline(always)]
    pub fn farm_state(&mut self, farm_state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.farm_state = Some(farm_state);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn scope_prices(
        &mut self,
        scope_prices: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.scope_prices = scope_prices;
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
        let accounts = RefreshUserState {
            user_state: self.user_state.expect("user_state is not set"),
            farm_state: self.farm_state.expect("farm_state is not set"),
            scope_prices: self.scope_prices,
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `refresh_user_state` CPI accounts.
pub struct RefreshUserStateCpiAccounts<'a, 'b> {
    pub user_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub farm_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub scope_prices: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `refresh_user_state` CPI instruction.
pub struct RefreshUserStateCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub farm_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub scope_prices: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

impl<'a, 'b> RefreshUserStateCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: RefreshUserStateCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            user_state: accounts.user_state,
            farm_state: accounts.farm_state,
            scope_prices: accounts.scope_prices,
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
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.farm_state.key,
            false,
        ));
        if let Some(scope_prices) = self.scope_prices {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *scope_prices.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::FARMS_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = borsh::to_vec(&RefreshUserStateInstructionData::new()).unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::FARMS_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(4 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.user_state.clone());
        account_infos.push(self.farm_state.clone());
        if let Some(scope_prices) = self.scope_prices {
            account_infos.push(scope_prices.clone());
        }
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

/// Instruction builder for `RefreshUserState` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` user_state
///   1. `[writable]` farm_state
///   2. `[optional]` scope_prices
#[derive(Clone, Debug)]
pub struct RefreshUserStateCpiBuilder<'a, 'b> {
    instruction: Box<RefreshUserStateCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> RefreshUserStateCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(RefreshUserStateCpiBuilderInstruction {
            __program: program,
            user_state: None,
            farm_state: None,
            scope_prices: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn user_state(
        &mut self,
        user_state: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_state = Some(user_state);
        self
    }

    #[inline(always)]
    pub fn farm_state(
        &mut self,
        farm_state: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.farm_state = Some(farm_state);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn scope_prices(
        &mut self,
        scope_prices: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.scope_prices = scope_prices;
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
        let instruction = RefreshUserStateCpi {
            __program: self.instruction.__program,

            user_state: self.instruction.user_state.expect("user_state is not set"),

            farm_state: self.instruction.farm_state.expect("farm_state is not set"),

            scope_prices: self.instruction.scope_prices,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct RefreshUserStateCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    user_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    farm_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    scope_prices: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
