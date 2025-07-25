//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct MigrateMeteoraDammLockLpToken {
    pub virtual_pool: solana_pubkey::Pubkey,
    /// migration_metadata
    pub migration_metadata: solana_pubkey::Pubkey,

    pub pool_authority: solana_pubkey::Pubkey,

    pub pool: solana_pubkey::Pubkey,

    pub lp_mint: solana_pubkey::Pubkey,

    pub lock_escrow: solana_pubkey::Pubkey,

    pub owner: solana_pubkey::Pubkey,

    pub source_tokens: solana_pubkey::Pubkey,

    pub escrow_vault: solana_pubkey::Pubkey,

    pub amm_program: solana_pubkey::Pubkey,

    pub a_vault: solana_pubkey::Pubkey,

    pub b_vault: solana_pubkey::Pubkey,

    pub a_vault_lp: solana_pubkey::Pubkey,

    pub b_vault_lp: solana_pubkey::Pubkey,

    pub a_vault_lp_mint: solana_pubkey::Pubkey,

    pub b_vault_lp_mint: solana_pubkey::Pubkey,
    /// token_program
    pub token_program: solana_pubkey::Pubkey,
}

impl MigrateMeteoraDammLockLpToken {
    pub fn instruction(&self) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(17 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.virtual_pool,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.migration_metadata,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.pool_authority,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(self.pool, false));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.lp_mint,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.lock_escrow,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.owner, false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.source_tokens,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.escrow_vault,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.amm_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.a_vault,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.b_vault,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.a_vault_lp,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.b_vault_lp,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.a_vault_lp_mint,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.b_vault_lp_mint,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&MigrateMeteoraDammLockLpTokenInstructionData::new()).unwrap();

        solana_instruction::Instruction {
            program_id: crate::DYNAMIC_BONDING_CURVE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MigrateMeteoraDammLockLpTokenInstructionData {
    discriminator: [u8; 8],
}

impl MigrateMeteoraDammLockLpTokenInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [177, 55, 238, 157, 251, 88, 165, 42],
        }
    }
}

impl Default for MigrateMeteoraDammLockLpTokenInstructionData {
    fn default() -> Self { Self::new() }
}

/// Instruction builder for `MigrateMeteoraDammLockLpToken`.
///
/// ### Accounts:
///
///   0. `[]` virtual_pool
///   1. `[writable]` migration_metadata
///   2. `[writable, optional]` pool_authority (default to `FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM`)
///   3. `[writable]` pool
///   4. `[]` lp_mint
///   5. `[writable]` lock_escrow
///   6. `[]` owner
///   7. `[writable]` source_tokens
///   8. `[writable]` escrow_vault
///   9. `[optional]` amm_program (default to `Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB`)
///   10. `[]` a_vault
///   11. `[]` b_vault
///   12. `[]` a_vault_lp
///   13. `[]` b_vault_lp
///   14. `[]` a_vault_lp_mint
///   15. `[]` b_vault_lp_mint
///   16. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct MigrateMeteoraDammLockLpTokenBuilder {
    virtual_pool: Option<solana_pubkey::Pubkey>,
    migration_metadata: Option<solana_pubkey::Pubkey>,
    pool_authority: Option<solana_pubkey::Pubkey>,
    pool: Option<solana_pubkey::Pubkey>,
    lp_mint: Option<solana_pubkey::Pubkey>,
    lock_escrow: Option<solana_pubkey::Pubkey>,
    owner: Option<solana_pubkey::Pubkey>,
    source_tokens: Option<solana_pubkey::Pubkey>,
    escrow_vault: Option<solana_pubkey::Pubkey>,
    amm_program: Option<solana_pubkey::Pubkey>,
    a_vault: Option<solana_pubkey::Pubkey>,
    b_vault: Option<solana_pubkey::Pubkey>,
    a_vault_lp: Option<solana_pubkey::Pubkey>,
    b_vault_lp: Option<solana_pubkey::Pubkey>,
    a_vault_lp_mint: Option<solana_pubkey::Pubkey>,
    b_vault_lp_mint: Option<solana_pubkey::Pubkey>,
    token_program: Option<solana_pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl MigrateMeteoraDammLockLpTokenBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn virtual_pool(&mut self, virtual_pool: solana_pubkey::Pubkey) -> &mut Self {
        self.virtual_pool = Some(virtual_pool);
        self
    }

    /// migration_metadata
    #[inline(always)]
    pub fn migration_metadata(&mut self, migration_metadata: solana_pubkey::Pubkey) -> &mut Self {
        self.migration_metadata = Some(migration_metadata);
        self
    }

    /// `[optional account, default to 'FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM']`
    #[inline(always)]
    pub fn pool_authority(&mut self, pool_authority: solana_pubkey::Pubkey) -> &mut Self {
        self.pool_authority = Some(pool_authority);
        self
    }

    #[inline(always)]
    pub fn pool(&mut self, pool: solana_pubkey::Pubkey) -> &mut Self {
        self.pool = Some(pool);
        self
    }

    #[inline(always)]
    pub fn lp_mint(&mut self, lp_mint: solana_pubkey::Pubkey) -> &mut Self {
        self.lp_mint = Some(lp_mint);
        self
    }

    #[inline(always)]
    pub fn lock_escrow(&mut self, lock_escrow: solana_pubkey::Pubkey) -> &mut Self {
        self.lock_escrow = Some(lock_escrow);
        self
    }

    #[inline(always)]
    pub fn owner(&mut self, owner: solana_pubkey::Pubkey) -> &mut Self {
        self.owner = Some(owner);
        self
    }

    #[inline(always)]
    pub fn source_tokens(&mut self, source_tokens: solana_pubkey::Pubkey) -> &mut Self {
        self.source_tokens = Some(source_tokens);
        self
    }

    #[inline(always)]
    pub fn escrow_vault(&mut self, escrow_vault: solana_pubkey::Pubkey) -> &mut Self {
        self.escrow_vault = Some(escrow_vault);
        self
    }

    /// `[optional account, default to 'Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB']`
    #[inline(always)]
    pub fn amm_program(&mut self, amm_program: solana_pubkey::Pubkey) -> &mut Self {
        self.amm_program = Some(amm_program);
        self
    }

    #[inline(always)]
    pub fn a_vault(&mut self, a_vault: solana_pubkey::Pubkey) -> &mut Self {
        self.a_vault = Some(a_vault);
        self
    }

    #[inline(always)]
    pub fn b_vault(&mut self, b_vault: solana_pubkey::Pubkey) -> &mut Self {
        self.b_vault = Some(b_vault);
        self
    }

    #[inline(always)]
    pub fn a_vault_lp(&mut self, a_vault_lp: solana_pubkey::Pubkey) -> &mut Self {
        self.a_vault_lp = Some(a_vault_lp);
        self
    }

    #[inline(always)]
    pub fn b_vault_lp(&mut self, b_vault_lp: solana_pubkey::Pubkey) -> &mut Self {
        self.b_vault_lp = Some(b_vault_lp);
        self
    }

    #[inline(always)]
    pub fn a_vault_lp_mint(&mut self, a_vault_lp_mint: solana_pubkey::Pubkey) -> &mut Self {
        self.a_vault_lp_mint = Some(a_vault_lp_mint);
        self
    }

    #[inline(always)]
    pub fn b_vault_lp_mint(&mut self, b_vault_lp_mint: solana_pubkey::Pubkey) -> &mut Self {
        self.b_vault_lp_mint = Some(b_vault_lp_mint);
        self
    }

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    /// token_program
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
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
        let accounts = MigrateMeteoraDammLockLpToken {
            virtual_pool: self.virtual_pool.expect("virtual_pool is not set"),
            migration_metadata: self
                .migration_metadata
                .expect("migration_metadata is not set"),
            pool_authority: self.pool_authority.unwrap_or(solana_pubkey::pubkey!(
                "FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM"
            )),
            pool: self.pool.expect("pool is not set"),
            lp_mint: self.lp_mint.expect("lp_mint is not set"),
            lock_escrow: self.lock_escrow.expect("lock_escrow is not set"),
            owner: self.owner.expect("owner is not set"),
            source_tokens: self.source_tokens.expect("source_tokens is not set"),
            escrow_vault: self.escrow_vault.expect("escrow_vault is not set"),
            amm_program: self.amm_program.unwrap_or(solana_pubkey::pubkey!(
                "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB"
            )),
            a_vault: self.a_vault.expect("a_vault is not set"),
            b_vault: self.b_vault.expect("b_vault is not set"),
            a_vault_lp: self.a_vault_lp.expect("a_vault_lp is not set"),
            b_vault_lp: self.b_vault_lp.expect("b_vault_lp is not set"),
            a_vault_lp_mint: self.a_vault_lp_mint.expect("a_vault_lp_mint is not set"),
            b_vault_lp_mint: self.b_vault_lp_mint.expect("b_vault_lp_mint is not set"),
            token_program: self.token_program.unwrap_or(solana_pubkey::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `migrate_meteora_damm_lock_lp_token` CPI accounts.
pub struct MigrateMeteoraDammLockLpTokenCpiAccounts<'a, 'b> {
    pub virtual_pool: &'b solana_account_info::AccountInfo<'a>,
    /// migration_metadata
    pub migration_metadata: &'b solana_account_info::AccountInfo<'a>,

    pub pool_authority: &'b solana_account_info::AccountInfo<'a>,

    pub pool: &'b solana_account_info::AccountInfo<'a>,

    pub lp_mint: &'b solana_account_info::AccountInfo<'a>,

    pub lock_escrow: &'b solana_account_info::AccountInfo<'a>,

    pub owner: &'b solana_account_info::AccountInfo<'a>,

    pub source_tokens: &'b solana_account_info::AccountInfo<'a>,

    pub escrow_vault: &'b solana_account_info::AccountInfo<'a>,

    pub amm_program: &'b solana_account_info::AccountInfo<'a>,

    pub a_vault: &'b solana_account_info::AccountInfo<'a>,

    pub b_vault: &'b solana_account_info::AccountInfo<'a>,

    pub a_vault_lp: &'b solana_account_info::AccountInfo<'a>,

    pub b_vault_lp: &'b solana_account_info::AccountInfo<'a>,

    pub a_vault_lp_mint: &'b solana_account_info::AccountInfo<'a>,

    pub b_vault_lp_mint: &'b solana_account_info::AccountInfo<'a>,
    /// token_program
    pub token_program: &'b solana_account_info::AccountInfo<'a>,
}

/// `migrate_meteora_damm_lock_lp_token` CPI instruction.
pub struct MigrateMeteoraDammLockLpTokenCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_account_info::AccountInfo<'a>,

    pub virtual_pool: &'b solana_account_info::AccountInfo<'a>,
    /// migration_metadata
    pub migration_metadata: &'b solana_account_info::AccountInfo<'a>,

    pub pool_authority: &'b solana_account_info::AccountInfo<'a>,

    pub pool: &'b solana_account_info::AccountInfo<'a>,

    pub lp_mint: &'b solana_account_info::AccountInfo<'a>,

    pub lock_escrow: &'b solana_account_info::AccountInfo<'a>,

    pub owner: &'b solana_account_info::AccountInfo<'a>,

    pub source_tokens: &'b solana_account_info::AccountInfo<'a>,

    pub escrow_vault: &'b solana_account_info::AccountInfo<'a>,

    pub amm_program: &'b solana_account_info::AccountInfo<'a>,

    pub a_vault: &'b solana_account_info::AccountInfo<'a>,

    pub b_vault: &'b solana_account_info::AccountInfo<'a>,

    pub a_vault_lp: &'b solana_account_info::AccountInfo<'a>,

    pub b_vault_lp: &'b solana_account_info::AccountInfo<'a>,

    pub a_vault_lp_mint: &'b solana_account_info::AccountInfo<'a>,

    pub b_vault_lp_mint: &'b solana_account_info::AccountInfo<'a>,
    /// token_program
    pub token_program: &'b solana_account_info::AccountInfo<'a>,
}

impl<'a, 'b> MigrateMeteoraDammLockLpTokenCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: MigrateMeteoraDammLockLpTokenCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            virtual_pool: accounts.virtual_pool,
            migration_metadata: accounts.migration_metadata,
            pool_authority: accounts.pool_authority,
            pool: accounts.pool,
            lp_mint: accounts.lp_mint,
            lock_escrow: accounts.lock_escrow,
            owner: accounts.owner,
            source_tokens: accounts.source_tokens,
            escrow_vault: accounts.escrow_vault,
            amm_program: accounts.amm_program,
            a_vault: accounts.a_vault,
            b_vault: accounts.b_vault,
            a_vault_lp: accounts.a_vault_lp,
            b_vault_lp: accounts.b_vault_lp,
            a_vault_lp_mint: accounts.a_vault_lp_mint,
            b_vault_lp_mint: accounts.b_vault_lp_mint,
            token_program: accounts.token_program,
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
        let mut accounts = Vec::with_capacity(17 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.virtual_pool.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.migration_metadata.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.pool_authority.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(*self.pool.key, false));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.lp_mint.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.lock_escrow.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.owner.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.source_tokens.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.escrow_vault.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.amm_program.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.a_vault.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.b_vault.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.a_vault_lp.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.b_vault_lp.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.a_vault_lp_mint.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.b_vault_lp_mint.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = borsh::to_vec(&MigrateMeteoraDammLockLpTokenInstructionData::new()).unwrap();

        let instruction = solana_instruction::Instruction {
            program_id: crate::DYNAMIC_BONDING_CURVE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(18 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.virtual_pool.clone());
        account_infos.push(self.migration_metadata.clone());
        account_infos.push(self.pool_authority.clone());
        account_infos.push(self.pool.clone());
        account_infos.push(self.lp_mint.clone());
        account_infos.push(self.lock_escrow.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.source_tokens.clone());
        account_infos.push(self.escrow_vault.clone());
        account_infos.push(self.amm_program.clone());
        account_infos.push(self.a_vault.clone());
        account_infos.push(self.b_vault.clone());
        account_infos.push(self.a_vault_lp.clone());
        account_infos.push(self.b_vault_lp.clone());
        account_infos.push(self.a_vault_lp_mint.clone());
        account_infos.push(self.b_vault_lp_mint.clone());
        account_infos.push(self.token_program.clone());
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

/// Instruction builder for `MigrateMeteoraDammLockLpToken` via CPI.
///
/// ### Accounts:
///
///   0. `[]` virtual_pool
///   1. `[writable]` migration_metadata
///   2. `[writable]` pool_authority
///   3. `[writable]` pool
///   4. `[]` lp_mint
///   5. `[writable]` lock_escrow
///   6. `[]` owner
///   7. `[writable]` source_tokens
///   8. `[writable]` escrow_vault
///   9. `[]` amm_program
///   10. `[]` a_vault
///   11. `[]` b_vault
///   12. `[]` a_vault_lp
///   13. `[]` b_vault_lp
///   14. `[]` a_vault_lp_mint
///   15. `[]` b_vault_lp_mint
///   16. `[]` token_program
#[derive(Clone, Debug)]
pub struct MigrateMeteoraDammLockLpTokenCpiBuilder<'a, 'b> {
    instruction: Box<MigrateMeteoraDammLockLpTokenCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> MigrateMeteoraDammLockLpTokenCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(MigrateMeteoraDammLockLpTokenCpiBuilderInstruction {
            __program: program,
            virtual_pool: None,
            migration_metadata: None,
            pool_authority: None,
            pool: None,
            lp_mint: None,
            lock_escrow: None,
            owner: None,
            source_tokens: None,
            escrow_vault: None,
            amm_program: None,
            a_vault: None,
            b_vault: None,
            a_vault_lp: None,
            b_vault_lp: None,
            a_vault_lp_mint: None,
            b_vault_lp_mint: None,
            token_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn virtual_pool(
        &mut self,
        virtual_pool: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.virtual_pool = Some(virtual_pool);
        self
    }

    /// migration_metadata
    #[inline(always)]
    pub fn migration_metadata(
        &mut self,
        migration_metadata: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.migration_metadata = Some(migration_metadata);
        self
    }

    #[inline(always)]
    pub fn pool_authority(
        &mut self,
        pool_authority: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.pool_authority = Some(pool_authority);
        self
    }

    #[inline(always)]
    pub fn pool(&mut self, pool: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.pool = Some(pool);
        self
    }

    #[inline(always)]
    pub fn lp_mint(&mut self, lp_mint: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.lp_mint = Some(lp_mint);
        self
    }

    #[inline(always)]
    pub fn lock_escrow(
        &mut self,
        lock_escrow: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lock_escrow = Some(lock_escrow);
        self
    }

    #[inline(always)]
    pub fn owner(&mut self, owner: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.owner = Some(owner);
        self
    }

    #[inline(always)]
    pub fn source_tokens(
        &mut self,
        source_tokens: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.source_tokens = Some(source_tokens);
        self
    }

    #[inline(always)]
    pub fn escrow_vault(
        &mut self,
        escrow_vault: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.escrow_vault = Some(escrow_vault);
        self
    }

    #[inline(always)]
    pub fn amm_program(
        &mut self,
        amm_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.amm_program = Some(amm_program);
        self
    }

    #[inline(always)]
    pub fn a_vault(&mut self, a_vault: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.a_vault = Some(a_vault);
        self
    }

    #[inline(always)]
    pub fn b_vault(&mut self, b_vault: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.b_vault = Some(b_vault);
        self
    }

    #[inline(always)]
    pub fn a_vault_lp(
        &mut self,
        a_vault_lp: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.a_vault_lp = Some(a_vault_lp);
        self
    }

    #[inline(always)]
    pub fn b_vault_lp(
        &mut self,
        b_vault_lp: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.b_vault_lp = Some(b_vault_lp);
        self
    }

    #[inline(always)]
    pub fn a_vault_lp_mint(
        &mut self,
        a_vault_lp_mint: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.a_vault_lp_mint = Some(a_vault_lp_mint);
        self
    }

    #[inline(always)]
    pub fn b_vault_lp_mint(
        &mut self,
        b_vault_lp_mint: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.b_vault_lp_mint = Some(b_vault_lp_mint);
        self
    }

    /// token_program
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
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
        let instruction = MigrateMeteoraDammLockLpTokenCpi {
            __program: self.instruction.__program,

            virtual_pool: self
                .instruction
                .virtual_pool
                .expect("virtual_pool is not set"),

            migration_metadata: self
                .instruction
                .migration_metadata
                .expect("migration_metadata is not set"),

            pool_authority: self
                .instruction
                .pool_authority
                .expect("pool_authority is not set"),

            pool: self.instruction.pool.expect("pool is not set"),

            lp_mint: self.instruction.lp_mint.expect("lp_mint is not set"),

            lock_escrow: self
                .instruction
                .lock_escrow
                .expect("lock_escrow is not set"),

            owner: self.instruction.owner.expect("owner is not set"),

            source_tokens: self
                .instruction
                .source_tokens
                .expect("source_tokens is not set"),

            escrow_vault: self
                .instruction
                .escrow_vault
                .expect("escrow_vault is not set"),

            amm_program: self
                .instruction
                .amm_program
                .expect("amm_program is not set"),

            a_vault: self.instruction.a_vault.expect("a_vault is not set"),

            b_vault: self.instruction.b_vault.expect("b_vault is not set"),

            a_vault_lp: self.instruction.a_vault_lp.expect("a_vault_lp is not set"),

            b_vault_lp: self.instruction.b_vault_lp.expect("b_vault_lp is not set"),

            a_vault_lp_mint: self
                .instruction
                .a_vault_lp_mint
                .expect("a_vault_lp_mint is not set"),

            b_vault_lp_mint: self
                .instruction
                .b_vault_lp_mint
                .expect("b_vault_lp_mint is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct MigrateMeteoraDammLockLpTokenCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    virtual_pool: Option<&'b solana_account_info::AccountInfo<'a>>,
    migration_metadata: Option<&'b solana_account_info::AccountInfo<'a>>,
    pool_authority: Option<&'b solana_account_info::AccountInfo<'a>>,
    pool: Option<&'b solana_account_info::AccountInfo<'a>>,
    lp_mint: Option<&'b solana_account_info::AccountInfo<'a>>,
    lock_escrow: Option<&'b solana_account_info::AccountInfo<'a>>,
    owner: Option<&'b solana_account_info::AccountInfo<'a>>,
    source_tokens: Option<&'b solana_account_info::AccountInfo<'a>>,
    escrow_vault: Option<&'b solana_account_info::AccountInfo<'a>>,
    amm_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    a_vault: Option<&'b solana_account_info::AccountInfo<'a>>,
    b_vault: Option<&'b solana_account_info::AccountInfo<'a>>,
    a_vault_lp: Option<&'b solana_account_info::AccountInfo<'a>>,
    b_vault_lp: Option<&'b solana_account_info::AccountInfo<'a>>,
    a_vault_lp_mint: Option<&'b solana_account_info::AccountInfo<'a>>,
    b_vault_lp_mint: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
