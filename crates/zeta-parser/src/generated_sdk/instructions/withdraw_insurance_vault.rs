//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct WithdrawInsuranceVault {
    pub state: solana_program::pubkey::Pubkey,

    pub insurance_vault: solana_program::pubkey::Pubkey,

    pub insurance_deposit_account: solana_program::pubkey::Pubkey,

    pub user_token_account: solana_program::pubkey::Pubkey,

    pub authority: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl WithdrawInsuranceVault {
    pub fn instruction(
        &self,
        args: WithdrawInsuranceVaultInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: WithdrawInsuranceVaultInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.state, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.insurance_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.insurance_deposit_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&WithdrawInsuranceVaultInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::ZETA_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawInsuranceVaultInstructionData {
    discriminator: [u8; 8],
}

impl WithdrawInsuranceVaultInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [17, 250, 213, 45, 172, 117, 81, 225],
        }
    }
}

impl Default for WithdrawInsuranceVaultInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawInsuranceVaultInstructionArgs {
    pub percentage_amount: u64,
}

/// Instruction builder for `WithdrawInsuranceVault`.
///
/// ### Accounts:
///
///   0. `[writable]` state
///   1. `[writable]` insurance_vault
///   2. `[writable]` insurance_deposit_account
///   3. `[writable]` user_token_account
///   4. `[signer]` authority
///   5. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct WithdrawInsuranceVaultBuilder {
    state: Option<solana_program::pubkey::Pubkey>,
    insurance_vault: Option<solana_program::pubkey::Pubkey>,
    insurance_deposit_account: Option<solana_program::pubkey::Pubkey>,
    user_token_account: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    percentage_amount: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl WithdrawInsuranceVaultBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn state(&mut self, state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn insurance_vault(
        &mut self,
        insurance_vault: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.insurance_vault = Some(insurance_vault);
        self
    }

    #[inline(always)]
    pub fn insurance_deposit_account(
        &mut self,
        insurance_deposit_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.insurance_deposit_account = Some(insurance_deposit_account);
        self
    }

    #[inline(always)]
    pub fn user_token_account(
        &mut self,
        user_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.user_token_account = Some(user_token_account);
        self
    }

    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn percentage_amount(&mut self, percentage_amount: u64) -> &mut Self {
        self.percentage_amount = Some(percentage_amount);
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
        let accounts = WithdrawInsuranceVault {
            state: self.state.expect("state is not set"),
            insurance_vault: self.insurance_vault.expect("insurance_vault is not set"),
            insurance_deposit_account: self
                .insurance_deposit_account
                .expect("insurance_deposit_account is not set"),
            user_token_account: self
                .user_token_account
                .expect("user_token_account is not set"),
            authority: self.authority.expect("authority is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };
        let args = WithdrawInsuranceVaultInstructionArgs {
            percentage_amount: self
                .percentage_amount
                .clone()
                .expect("percentage_amount is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `withdraw_insurance_vault` CPI accounts.
pub struct WithdrawInsuranceVaultCpiAccounts<'a, 'b> {
    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub insurance_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub insurance_deposit_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `withdraw_insurance_vault` CPI instruction.
pub struct WithdrawInsuranceVaultCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub insurance_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub insurance_deposit_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: WithdrawInsuranceVaultInstructionArgs,
}

impl<'a, 'b> WithdrawInsuranceVaultCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: WithdrawInsuranceVaultCpiAccounts<'a, 'b>,
        args: WithdrawInsuranceVaultInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            state: accounts.state,
            insurance_vault: accounts.insurance_vault,
            insurance_deposit_account: accounts.insurance_deposit_account,
            user_token_account: accounts.user_token_account,
            authority: accounts.authority,
            token_program: accounts.token_program,
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
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.insurance_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.insurance_deposit_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&WithdrawInsuranceVaultInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::ZETA_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.state.clone());
        account_infos.push(self.insurance_vault.clone());
        account_infos.push(self.insurance_deposit_account.clone());
        account_infos.push(self.user_token_account.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.token_program.clone());
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

/// Instruction builder for `WithdrawInsuranceVault` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` state
///   1. `[writable]` insurance_vault
///   2. `[writable]` insurance_deposit_account
///   3. `[writable]` user_token_account
///   4. `[signer]` authority
///   5. `[]` token_program
#[derive(Clone, Debug)]
pub struct WithdrawInsuranceVaultCpiBuilder<'a, 'b> {
    instruction: Box<WithdrawInsuranceVaultCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> WithdrawInsuranceVaultCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(WithdrawInsuranceVaultCpiBuilderInstruction {
            __program: program,
            state: None,
            insurance_vault: None,
            insurance_deposit_account: None,
            user_token_account: None,
            authority: None,
            token_program: None,
            percentage_amount: None,
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
    pub fn insurance_vault(
        &mut self,
        insurance_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.insurance_vault = Some(insurance_vault);
        self
    }

    #[inline(always)]
    pub fn insurance_deposit_account(
        &mut self,
        insurance_deposit_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.insurance_deposit_account = Some(insurance_deposit_account);
        self
    }

    #[inline(always)]
    pub fn user_token_account(
        &mut self,
        user_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_token_account = Some(user_token_account);
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
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn percentage_amount(&mut self, percentage_amount: u64) -> &mut Self {
        self.instruction.percentage_amount = Some(percentage_amount);
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
        let args = WithdrawInsuranceVaultInstructionArgs {
            percentage_amount: self
                .instruction
                .percentage_amount
                .clone()
                .expect("percentage_amount is not set"),
        };
        let instruction = WithdrawInsuranceVaultCpi {
            __program: self.instruction.__program,

            state: self.instruction.state.expect("state is not set"),

            insurance_vault: self
                .instruction
                .insurance_vault
                .expect("insurance_vault is not set"),

            insurance_deposit_account: self
                .instruction
                .insurance_deposit_account
                .expect("insurance_deposit_account is not set"),

            user_token_account: self
                .instruction
                .user_token_account
                .expect("user_token_account is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct WithdrawInsuranceVaultCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    insurance_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    insurance_deposit_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    percentage_amount: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
