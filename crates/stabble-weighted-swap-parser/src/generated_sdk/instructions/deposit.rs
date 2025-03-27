//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct Deposit {
    pub user: solana_program::pubkey::Pubkey,

    pub user_pool_token: solana_program::pubkey::Pubkey,

    pub mint: solana_program::pubkey::Pubkey,

    pub pool: solana_program::pubkey::Pubkey,

    pub pool_authority: solana_program::pubkey::Pubkey,

    pub vault: solana_program::pubkey::Pubkey,

    pub vault_authority: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub token_program2022: solana_program::pubkey::Pubkey,
}

impl Deposit {
    pub fn instruction(
        &self,
        args: DepositInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: DepositInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.user, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_pool_token,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.mint, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.pool_authority,
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
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program2022,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&DepositInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::WEIGHTED_SWAP_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositInstructionData {
    discriminator: [u8; 8],
}

impl DepositInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [242, 35, 198, 137, 82, 225, 242, 182],
        }
    }
}

impl Default for DepositInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositInstructionArgs {
    pub amounts: Vec<u64>,
    pub minimum_amount_out: u64,
}

/// Instruction builder for `Deposit`.
///
/// ### Accounts:
///
///   0. `[signer]` user
///   1. `[writable]` user_pool_token
///   2. `[writable]` mint
///   3. `[writable]` pool
///   4. `[]` pool_authority
///   5. `[]` vault
///   6. `[]` vault_authority
///   7. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   8. `[]` token_program2022
#[derive(Clone, Debug, Default)]
pub struct DepositBuilder {
    user: Option<solana_program::pubkey::Pubkey>,
    user_pool_token: Option<solana_program::pubkey::Pubkey>,
    mint: Option<solana_program::pubkey::Pubkey>,
    pool: Option<solana_program::pubkey::Pubkey>,
    pool_authority: Option<solana_program::pubkey::Pubkey>,
    vault: Option<solana_program::pubkey::Pubkey>,
    vault_authority: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    token_program2022: Option<solana_program::pubkey::Pubkey>,
    amounts: Option<Vec<u64>>,
    minimum_amount_out: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl DepositBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn user(&mut self, user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user = Some(user);
        self
    }

    #[inline(always)]
    pub fn user_pool_token(
        &mut self,
        user_pool_token: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.user_pool_token = Some(user_pool_token);
        self
    }

    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }

    #[inline(always)]
    pub fn pool(&mut self, pool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.pool = Some(pool);
        self
    }

    #[inline(always)]
    pub fn pool_authority(&mut self, pool_authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.pool_authority = Some(pool_authority);
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

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn token_program2022(
        &mut self,
        token_program2022: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_program2022 = Some(token_program2022);
        self
    }

    #[inline(always)]
    pub fn amounts(&mut self, amounts: Vec<u64>) -> &mut Self {
        self.amounts = Some(amounts);
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
        let accounts = Deposit {
            user: self.user.expect("user is not set"),
            user_pool_token: self.user_pool_token.expect("user_pool_token is not set"),
            mint: self.mint.expect("mint is not set"),
            pool: self.pool.expect("pool is not set"),
            pool_authority: self.pool_authority.expect("pool_authority is not set"),
            vault: self.vault.expect("vault is not set"),
            vault_authority: self.vault_authority.expect("vault_authority is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            token_program2022: self
                .token_program2022
                .expect("token_program2022 is not set"),
        };
        let args = DepositInstructionArgs {
            amounts: self.amounts.clone().expect("amounts is not set"),
            minimum_amount_out: self
                .minimum_amount_out
                .clone()
                .expect("minimum_amount_out is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `deposit` CPI accounts.
pub struct DepositCpiAccounts<'a, 'b> {
    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_pool_token: &'b solana_program::account_info::AccountInfo<'a>,

    pub mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program2022: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `deposit` CPI instruction.
pub struct DepositCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_pool_token: &'b solana_program::account_info::AccountInfo<'a>,

    pub mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program2022: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: DepositInstructionArgs,
}

impl<'a, 'b> DepositCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: DepositCpiAccounts<'a, 'b>,
        args: DepositInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            user: accounts.user,
            user_pool_token: accounts.user_pool_token,
            mint: accounts.mint,
            pool: accounts.pool,
            pool_authority: accounts.pool_authority,
            vault: accounts.vault,
            vault_authority: accounts.vault_authority,
            token_program: accounts.token_program,
            token_program2022: accounts.token_program2022,
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
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.user.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_pool_token.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.pool_authority.key,
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
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program2022.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&DepositInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::WEIGHTED_SWAP_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(10 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.user.clone());
        account_infos.push(self.user_pool_token.clone());
        account_infos.push(self.mint.clone());
        account_infos.push(self.pool.clone());
        account_infos.push(self.pool_authority.clone());
        account_infos.push(self.vault.clone());
        account_infos.push(self.vault_authority.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.token_program2022.clone());
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

/// Instruction builder for `Deposit` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` user
///   1. `[writable]` user_pool_token
///   2. `[writable]` mint
///   3. `[writable]` pool
///   4. `[]` pool_authority
///   5. `[]` vault
///   6. `[]` vault_authority
///   7. `[]` token_program
///   8. `[]` token_program2022
#[derive(Clone, Debug)]
pub struct DepositCpiBuilder<'a, 'b> {
    instruction: Box<DepositCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> DepositCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(DepositCpiBuilderInstruction {
            __program: program,
            user: None,
            user_pool_token: None,
            mint: None,
            pool: None,
            pool_authority: None,
            vault: None,
            vault_authority: None,
            token_program: None,
            token_program2022: None,
            amounts: None,
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
    pub fn user_pool_token(
        &mut self,
        user_pool_token: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_pool_token = Some(user_pool_token);
        self
    }

    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
        self
    }

    #[inline(always)]
    pub fn pool(&mut self, pool: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.pool = Some(pool);
        self
    }

    #[inline(always)]
    pub fn pool_authority(
        &mut self,
        pool_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.pool_authority = Some(pool_authority);
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
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn token_program2022(
        &mut self,
        token_program2022: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program2022 = Some(token_program2022);
        self
    }

    #[inline(always)]
    pub fn amounts(&mut self, amounts: Vec<u64>) -> &mut Self {
        self.instruction.amounts = Some(amounts);
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
        let args = DepositInstructionArgs {
            amounts: self
                .instruction
                .amounts
                .clone()
                .expect("amounts is not set"),
            minimum_amount_out: self
                .instruction
                .minimum_amount_out
                .clone()
                .expect("minimum_amount_out is not set"),
        };
        let instruction = DepositCpi {
            __program: self.instruction.__program,

            user: self.instruction.user.expect("user is not set"),

            user_pool_token: self
                .instruction
                .user_pool_token
                .expect("user_pool_token is not set"),

            mint: self.instruction.mint.expect("mint is not set"),

            pool: self.instruction.pool.expect("pool is not set"),

            pool_authority: self
                .instruction
                .pool_authority
                .expect("pool_authority is not set"),

            vault: self.instruction.vault.expect("vault is not set"),

            vault_authority: self
                .instruction
                .vault_authority
                .expect("vault_authority is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            token_program2022: self
                .instruction
                .token_program2022
                .expect("token_program2022 is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct DepositCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_pool_token: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program2022: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    amounts: Option<Vec<u64>>,
    minimum_amount_out: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
