//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct UpdateGlobalConfigAdmin {
    pub pending_global_admin: solana_program::pubkey::Pubkey,

    pub global_config: solana_program::pubkey::Pubkey,
}

impl UpdateGlobalConfigAdmin {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.pending_global_admin,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.global_config,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&UpdateGlobalConfigAdminInstructionData::new()).unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::FARMS_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateGlobalConfigAdminInstructionData {
    discriminator: [u8; 8],
}

impl UpdateGlobalConfigAdminInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [184, 87, 23, 193, 156, 238, 175, 119],
        }
    }
}

impl Default for UpdateGlobalConfigAdminInstructionData {
    fn default() -> Self { Self::new() }
}

/// Instruction builder for `UpdateGlobalConfigAdmin`.
///
/// ### Accounts:
///
///   0. `[signer]` pending_global_admin
///   1. `[writable]` global_config
#[derive(Clone, Debug, Default)]
pub struct UpdateGlobalConfigAdminBuilder {
    pending_global_admin: Option<solana_program::pubkey::Pubkey>,
    global_config: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UpdateGlobalConfigAdminBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn pending_global_admin(
        &mut self,
        pending_global_admin: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.pending_global_admin = Some(pending_global_admin);
        self
    }

    #[inline(always)]
    pub fn global_config(&mut self, global_config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.global_config = Some(global_config);
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
        let accounts = UpdateGlobalConfigAdmin {
            pending_global_admin: self
                .pending_global_admin
                .expect("pending_global_admin is not set"),
            global_config: self.global_config.expect("global_config is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `update_global_config_admin` CPI accounts.
pub struct UpdateGlobalConfigAdminCpiAccounts<'a, 'b> {
    pub pending_global_admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub global_config: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `update_global_config_admin` CPI instruction.
pub struct UpdateGlobalConfigAdminCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub pending_global_admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub global_config: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> UpdateGlobalConfigAdminCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UpdateGlobalConfigAdminCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            pending_global_admin: accounts.pending_global_admin,
            global_config: accounts.global_config,
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
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.pending_global_admin.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.global_config.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = borsh::to_vec(&UpdateGlobalConfigAdminInstructionData::new()).unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::FARMS_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.pending_global_admin.clone());
        account_infos.push(self.global_config.clone());
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

/// Instruction builder for `UpdateGlobalConfigAdmin` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` pending_global_admin
///   1. `[writable]` global_config
#[derive(Clone, Debug)]
pub struct UpdateGlobalConfigAdminCpiBuilder<'a, 'b> {
    instruction: Box<UpdateGlobalConfigAdminCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdateGlobalConfigAdminCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UpdateGlobalConfigAdminCpiBuilderInstruction {
            __program: program,
            pending_global_admin: None,
            global_config: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn pending_global_admin(
        &mut self,
        pending_global_admin: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.pending_global_admin = Some(pending_global_admin);
        self
    }

    #[inline(always)]
    pub fn global_config(
        &mut self,
        global_config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.global_config = Some(global_config);
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
        let instruction = UpdateGlobalConfigAdminCpi {
            __program: self.instruction.__program,

            pending_global_admin: self
                .instruction
                .pending_global_admin
                .expect("pending_global_admin is not set"),

            global_config: self
                .instruction
                .global_config
                .expect("global_config is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct UpdateGlobalConfigAdminCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    pending_global_admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    global_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
