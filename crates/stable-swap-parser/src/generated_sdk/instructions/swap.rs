//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct Swap {
    pub user: solana_program::pubkey::Pubkey,

    pub user_token_in: solana_program::pubkey::Pubkey,

    pub user_token_out: solana_program::pubkey::Pubkey,

    pub vault_token_in: solana_program::pubkey::Pubkey,

    pub vault_token_out: solana_program::pubkey::Pubkey,

    pub beneficiary_token_out: solana_program::pubkey::Pubkey,

    pub pool: solana_program::pubkey::Pubkey,

    pub withdraw_authority: solana_program::pubkey::Pubkey,

    pub vault: solana_program::pubkey::Pubkey,

    pub vault_authority: solana_program::pubkey::Pubkey,

    pub vault_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl Swap {
    pub fn instruction(
        &self,
        args: SwapInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: SwapInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.user, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_token_in,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_token_out,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault_token_in,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault_token_out,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.beneficiary_token_out,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.withdraw_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.vault, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.vault_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.vault_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&SwapInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::STABLE_SWAP_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapInstructionData {
    discriminator: [u8; 8],
}

impl SwapInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [248, 198, 158, 145, 225, 117, 135, 200],
        }
    }
}

impl Default for SwapInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapInstructionArgs {
    pub amount_in: Option<u64>,
    pub minimum_amount_out: u64,
}

/// Instruction builder for `Swap`.
///
/// ### Accounts:
///
///   0. `[signer]` user
///   1. `[writable]` user_token_in
///   2. `[writable]` user_token_out
///   3. `[writable]` vault_token_in
///   4. `[writable]` vault_token_out
///   5. `[writable]` beneficiary_token_out
///   6. `[writable]` pool
///   7. `[]` withdraw_authority
///   8. `[]` vault
///   9. `[]` vault_authority
///   10. `[]` vault_program
///   11. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct SwapBuilder {
    user: Option<solana_program::pubkey::Pubkey>,
    user_token_in: Option<solana_program::pubkey::Pubkey>,
    user_token_out: Option<solana_program::pubkey::Pubkey>,
    vault_token_in: Option<solana_program::pubkey::Pubkey>,
    vault_token_out: Option<solana_program::pubkey::Pubkey>,
    beneficiary_token_out: Option<solana_program::pubkey::Pubkey>,
    pool: Option<solana_program::pubkey::Pubkey>,
    withdraw_authority: Option<solana_program::pubkey::Pubkey>,
    vault: Option<solana_program::pubkey::Pubkey>,
    vault_authority: Option<solana_program::pubkey::Pubkey>,
    vault_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    amount_in: Option<u64>,
    minimum_amount_out: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl SwapBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn user(&mut self, user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user = Some(user);
        self
    }

    #[inline(always)]
    pub fn user_token_in(&mut self, user_token_in: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user_token_in = Some(user_token_in);
        self
    }

    #[inline(always)]
    pub fn user_token_out(&mut self, user_token_out: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user_token_out = Some(user_token_out);
        self
    }

    #[inline(always)]
    pub fn vault_token_in(&mut self, vault_token_in: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vault_token_in = Some(vault_token_in);
        self
    }

    #[inline(always)]
    pub fn vault_token_out(
        &mut self,
        vault_token_out: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.vault_token_out = Some(vault_token_out);
        self
    }

    #[inline(always)]
    pub fn beneficiary_token_out(
        &mut self,
        beneficiary_token_out: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.beneficiary_token_out = Some(beneficiary_token_out);
        self
    }

    #[inline(always)]
    pub fn pool(&mut self, pool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.pool = Some(pool);
        self
    }

    #[inline(always)]
    pub fn withdraw_authority(
        &mut self,
        withdraw_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.withdraw_authority = Some(withdraw_authority);
        self
    }

    #[inline(always)]
    pub fn vault(&mut self, vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vault = Some(vault);
        self
    }

    #[inline(always)]
    pub fn vault_authority(
        &mut self,
        vault_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.vault_authority = Some(vault_authority);
        self
    }

    #[inline(always)]
    pub fn vault_program(&mut self, vault_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vault_program = Some(vault_program);
        self
    }

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn amount_in(&mut self, amount_in: u64) -> &mut Self {
        self.amount_in = Some(amount_in);
        self
    }

    #[inline(always)]
    pub fn minimum_amount_out(&mut self, minimum_amount_out: u64) -> &mut Self {
        self.minimum_amount_out = Some(minimum_amount_out);
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
        let accounts = Swap {
            user: self.user.expect("user is not set"),
            user_token_in: self.user_token_in.expect("user_token_in is not set"),
            user_token_out: self.user_token_out.expect("user_token_out is not set"),
            vault_token_in: self.vault_token_in.expect("vault_token_in is not set"),
            vault_token_out: self.vault_token_out.expect("vault_token_out is not set"),
            beneficiary_token_out: self
                .beneficiary_token_out
                .expect("beneficiary_token_out is not set"),
            pool: self.pool.expect("pool is not set"),
            withdraw_authority: self
                .withdraw_authority
                .expect("withdraw_authority is not set"),
            vault: self.vault.expect("vault is not set"),
            vault_authority: self.vault_authority.expect("vault_authority is not set"),
            vault_program: self.vault_program.expect("vault_program is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };
        let args = SwapInstructionArgs {
            amount_in: self.amount_in.clone(),
            minimum_amount_out: self
                .minimum_amount_out
                .clone()
                .expect("minimum_amount_out is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `swap` CPI accounts.
pub struct SwapCpiAccounts<'a, 'b> {
    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_in: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_out: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_token_in: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_token_out: &'b solana_program::account_info::AccountInfo<'a>,

    pub beneficiary_token_out: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub withdraw_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `swap` CPI instruction.
pub struct SwapCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_in: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_out: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_token_in: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_token_out: &'b solana_program::account_info::AccountInfo<'a>,

    pub beneficiary_token_out: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub withdraw_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: SwapInstructionArgs,
}

impl<'a, 'b> SwapCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: SwapCpiAccounts<'a, 'b>,
        args: SwapInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            user: accounts.user,
            user_token_in: accounts.user_token_in,
            user_token_out: accounts.user_token_out,
            vault_token_in: accounts.vault_token_in,
            vault_token_out: accounts.vault_token_out,
            beneficiary_token_out: accounts.beneficiary_token_out,
            pool: accounts.pool,
            withdraw_authority: accounts.withdraw_authority,
            vault: accounts.vault,
            vault_authority: accounts.vault_authority,
            vault_program: accounts.vault_program,
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
        let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.user.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_token_in.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_token_out.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault_token_in.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault_token_out.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.beneficiary_token_out.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.withdraw_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.vault_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.vault_program.key,
            false,
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
        let mut data = borsh::to_vec(&SwapInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::STABLE_SWAP_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(13 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.user.clone());
        account_infos.push(self.user_token_in.clone());
        account_infos.push(self.user_token_out.clone());
        account_infos.push(self.vault_token_in.clone());
        account_infos.push(self.vault_token_out.clone());
        account_infos.push(self.beneficiary_token_out.clone());
        account_infos.push(self.pool.clone());
        account_infos.push(self.withdraw_authority.clone());
        account_infos.push(self.vault.clone());
        account_infos.push(self.vault_authority.clone());
        account_infos.push(self.vault_program.clone());
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

/// Instruction builder for `Swap` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` user
///   1. `[writable]` user_token_in
///   2. `[writable]` user_token_out
///   3. `[writable]` vault_token_in
///   4. `[writable]` vault_token_out
///   5. `[writable]` beneficiary_token_out
///   6. `[writable]` pool
///   7. `[]` withdraw_authority
///   8. `[]` vault
///   9. `[]` vault_authority
///   10. `[]` vault_program
///   11. `[]` token_program
#[derive(Clone, Debug)]
pub struct SwapCpiBuilder<'a, 'b> {
    instruction: Box<SwapCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SwapCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SwapCpiBuilderInstruction {
            __program: program,
            user: None,
            user_token_in: None,
            user_token_out: None,
            vault_token_in: None,
            vault_token_out: None,
            beneficiary_token_out: None,
            pool: None,
            withdraw_authority: None,
            vault: None,
            vault_authority: None,
            vault_program: None,
            token_program: None,
            amount_in: None,
            minimum_amount_out: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn user(&mut self, user: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.user = Some(user);
        self
    }

    #[inline(always)]
    pub fn user_token_in(
        &mut self,
        user_token_in: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_token_in = Some(user_token_in);
        self
    }

    #[inline(always)]
    pub fn user_token_out(
        &mut self,
        user_token_out: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_token_out = Some(user_token_out);
        self
    }

    #[inline(always)]
    pub fn vault_token_in(
        &mut self,
        vault_token_in: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vault_token_in = Some(vault_token_in);
        self
    }

    #[inline(always)]
    pub fn vault_token_out(
        &mut self,
        vault_token_out: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vault_token_out = Some(vault_token_out);
        self
    }

    #[inline(always)]
    pub fn beneficiary_token_out(
        &mut self,
        beneficiary_token_out: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.beneficiary_token_out = Some(beneficiary_token_out);
        self
    }

    #[inline(always)]
    pub fn pool(&mut self, pool: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.pool = Some(pool);
        self
    }

    #[inline(always)]
    pub fn withdraw_authority(
        &mut self,
        withdraw_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.withdraw_authority = Some(withdraw_authority);
        self
    }

    #[inline(always)]
    pub fn vault(&mut self, vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.vault = Some(vault);
        self
    }

    #[inline(always)]
    pub fn vault_authority(
        &mut self,
        vault_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vault_authority = Some(vault_authority);
        self
    }

    #[inline(always)]
    pub fn vault_program(
        &mut self,
        vault_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vault_program = Some(vault_program);
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

    /// `[optional argument]`
    #[inline(always)]
    pub fn amount_in(&mut self, amount_in: u64) -> &mut Self {
        self.instruction.amount_in = Some(amount_in);
        self
    }

    #[inline(always)]
    pub fn minimum_amount_out(&mut self, minimum_amount_out: u64) -> &mut Self {
        self.instruction.minimum_amount_out = Some(minimum_amount_out);
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
        let args = SwapInstructionArgs {
            amount_in: self.instruction.amount_in.clone(),
            minimum_amount_out: self
                .instruction
                .minimum_amount_out
                .clone()
                .expect("minimum_amount_out is not set"),
        };
        let instruction = SwapCpi {
            __program: self.instruction.__program,

            user: self.instruction.user.expect("user is not set"),

            user_token_in: self
                .instruction
                .user_token_in
                .expect("user_token_in is not set"),

            user_token_out: self
                .instruction
                .user_token_out
                .expect("user_token_out is not set"),

            vault_token_in: self
                .instruction
                .vault_token_in
                .expect("vault_token_in is not set"),

            vault_token_out: self
                .instruction
                .vault_token_out
                .expect("vault_token_out is not set"),

            beneficiary_token_out: self
                .instruction
                .beneficiary_token_out
                .expect("beneficiary_token_out is not set"),

            pool: self.instruction.pool.expect("pool is not set"),

            withdraw_authority: self
                .instruction
                .withdraw_authority
                .expect("withdraw_authority is not set"),

            vault: self.instruction.vault.expect("vault is not set"),

            vault_authority: self
                .instruction
                .vault_authority
                .expect("vault_authority is not set"),

            vault_program: self
                .instruction
                .vault_program
                .expect("vault_program is not set"),

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
struct SwapCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_token_in: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_token_out: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_token_in: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_token_out: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    beneficiary_token_out: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    withdraw_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    amount_in: Option<u64>,
    minimum_amount_out: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
