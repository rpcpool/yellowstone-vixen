//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct RemoveAllLiquidity {
    pub pool_authority: solana_pubkey::Pubkey,

    pub pool: solana_pubkey::Pubkey,

    pub position: solana_pubkey::Pubkey,
    /// The user token a account
    pub token_a_account: solana_pubkey::Pubkey,
    /// The user token b account
    pub token_b_account: solana_pubkey::Pubkey,
    /// The vault token account for input token
    pub token_a_vault: solana_pubkey::Pubkey,
    /// The vault token account for output token
    pub token_b_vault: solana_pubkey::Pubkey,
    /// The mint of token a
    pub token_a_mint: solana_pubkey::Pubkey,
    /// The mint of token b
    pub token_b_mint: solana_pubkey::Pubkey,
    /// The token account for nft
    pub position_nft_account: solana_pubkey::Pubkey,
    /// owner of position
    pub owner: solana_pubkey::Pubkey,
    /// Token a program
    pub token_a_program: solana_pubkey::Pubkey,
    /// Token b program
    pub token_b_program: solana_pubkey::Pubkey,

    pub event_authority: solana_pubkey::Pubkey,

    pub program: solana_pubkey::Pubkey,
}

impl RemoveAllLiquidity {
    pub fn instruction(
        &self,
        args: RemoveAllLiquidityInstructionArgs,
    ) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: RemoveAllLiquidityInstructionArgs,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(15 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.pool_authority,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(self.pool, false));
        accounts.push(solana_instruction::AccountMeta::new(self.position, false));
        accounts.push(solana_instruction::AccountMeta::new(
            self.token_a_account,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.token_b_account,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.token_a_vault,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.token_b_vault,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_a_mint,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_b_mint,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.position_nft_account,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.owner, true,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_a_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_b_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.event_authority,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&RemoveAllLiquidityInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_instruction::Instruction {
            program_id: crate::CP_AMM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveAllLiquidityInstructionData {
    discriminator: [u8; 8],
}

impl RemoveAllLiquidityInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [10, 51, 61, 35, 112, 105, 24, 85],
        }
    }
}

impl Default for RemoveAllLiquidityInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveAllLiquidityInstructionArgs {
    pub token_a_amount_threshold: u64,
    pub token_b_amount_threshold: u64,
}

/// Instruction builder for `RemoveAllLiquidity`.
///
/// ### Accounts:
///
///   0. `[]` pool_authority
///   1. `[writable]` pool
///   2. `[writable]` position
///   3. `[writable]` token_a_account
///   4. `[writable]` token_b_account
///   5. `[writable]` token_a_vault
///   6. `[writable]` token_b_vault
///   7. `[]` token_a_mint
///   8. `[]` token_b_mint
///   9. `[]` position_nft_account
///   10. `[signer]` owner
///   11. `[]` token_a_program
///   12. `[]` token_b_program
///   13. `[]` event_authority
///   14. `[]` program
#[derive(Clone, Debug, Default)]
pub struct RemoveAllLiquidityBuilder {
    pool_authority: Option<solana_pubkey::Pubkey>,
    pool: Option<solana_pubkey::Pubkey>,
    position: Option<solana_pubkey::Pubkey>,
    token_a_account: Option<solana_pubkey::Pubkey>,
    token_b_account: Option<solana_pubkey::Pubkey>,
    token_a_vault: Option<solana_pubkey::Pubkey>,
    token_b_vault: Option<solana_pubkey::Pubkey>,
    token_a_mint: Option<solana_pubkey::Pubkey>,
    token_b_mint: Option<solana_pubkey::Pubkey>,
    position_nft_account: Option<solana_pubkey::Pubkey>,
    owner: Option<solana_pubkey::Pubkey>,
    token_a_program: Option<solana_pubkey::Pubkey>,
    token_b_program: Option<solana_pubkey::Pubkey>,
    event_authority: Option<solana_pubkey::Pubkey>,
    program: Option<solana_pubkey::Pubkey>,
    token_a_amount_threshold: Option<u64>,
    token_b_amount_threshold: Option<u64>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl RemoveAllLiquidityBuilder {
    pub fn new() -> Self { Self::default() }

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
    pub fn position(&mut self, position: solana_pubkey::Pubkey) -> &mut Self {
        self.position = Some(position);
        self
    }

    /// The user token a account
    #[inline(always)]
    pub fn token_a_account(&mut self, token_a_account: solana_pubkey::Pubkey) -> &mut Self {
        self.token_a_account = Some(token_a_account);
        self
    }

    /// The user token b account
    #[inline(always)]
    pub fn token_b_account(&mut self, token_b_account: solana_pubkey::Pubkey) -> &mut Self {
        self.token_b_account = Some(token_b_account);
        self
    }

    /// The vault token account for input token
    #[inline(always)]
    pub fn token_a_vault(&mut self, token_a_vault: solana_pubkey::Pubkey) -> &mut Self {
        self.token_a_vault = Some(token_a_vault);
        self
    }

    /// The vault token account for output token
    #[inline(always)]
    pub fn token_b_vault(&mut self, token_b_vault: solana_pubkey::Pubkey) -> &mut Self {
        self.token_b_vault = Some(token_b_vault);
        self
    }

    /// The mint of token a
    #[inline(always)]
    pub fn token_a_mint(&mut self, token_a_mint: solana_pubkey::Pubkey) -> &mut Self {
        self.token_a_mint = Some(token_a_mint);
        self
    }

    /// The mint of token b
    #[inline(always)]
    pub fn token_b_mint(&mut self, token_b_mint: solana_pubkey::Pubkey) -> &mut Self {
        self.token_b_mint = Some(token_b_mint);
        self
    }

    /// The token account for nft
    #[inline(always)]
    pub fn position_nft_account(
        &mut self,
        position_nft_account: solana_pubkey::Pubkey,
    ) -> &mut Self {
        self.position_nft_account = Some(position_nft_account);
        self
    }

    /// owner of position
    #[inline(always)]
    pub fn owner(&mut self, owner: solana_pubkey::Pubkey) -> &mut Self {
        self.owner = Some(owner);
        self
    }

    /// Token a program
    #[inline(always)]
    pub fn token_a_program(&mut self, token_a_program: solana_pubkey::Pubkey) -> &mut Self {
        self.token_a_program = Some(token_a_program);
        self
    }

    /// Token b program
    #[inline(always)]
    pub fn token_b_program(&mut self, token_b_program: solana_pubkey::Pubkey) -> &mut Self {
        self.token_b_program = Some(token_b_program);
        self
    }

    #[inline(always)]
    pub fn event_authority(&mut self, event_authority: solana_pubkey::Pubkey) -> &mut Self {
        self.event_authority = Some(event_authority);
        self
    }

    #[inline(always)]
    pub fn program(&mut self, program: solana_pubkey::Pubkey) -> &mut Self {
        self.program = Some(program);
        self
    }

    #[inline(always)]
    pub fn token_a_amount_threshold(&mut self, token_a_amount_threshold: u64) -> &mut Self {
        self.token_a_amount_threshold = Some(token_a_amount_threshold);
        self
    }

    #[inline(always)]
    pub fn token_b_amount_threshold(&mut self, token_b_amount_threshold: u64) -> &mut Self {
        self.token_b_amount_threshold = Some(token_b_amount_threshold);
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
        let accounts = RemoveAllLiquidity {
            pool_authority: self.pool_authority.expect("pool_authority is not set"),
            pool: self.pool.expect("pool is not set"),
            position: self.position.expect("position is not set"),
            token_a_account: self.token_a_account.expect("token_a_account is not set"),
            token_b_account: self.token_b_account.expect("token_b_account is not set"),
            token_a_vault: self.token_a_vault.expect("token_a_vault is not set"),
            token_b_vault: self.token_b_vault.expect("token_b_vault is not set"),
            token_a_mint: self.token_a_mint.expect("token_a_mint is not set"),
            token_b_mint: self.token_b_mint.expect("token_b_mint is not set"),
            position_nft_account: self
                .position_nft_account
                .expect("position_nft_account is not set"),
            owner: self.owner.expect("owner is not set"),
            token_a_program: self.token_a_program.expect("token_a_program is not set"),
            token_b_program: self.token_b_program.expect("token_b_program is not set"),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = RemoveAllLiquidityInstructionArgs {
            token_a_amount_threshold: self
                .token_a_amount_threshold
                .clone()
                .expect("token_a_amount_threshold is not set"),
            token_b_amount_threshold: self
                .token_b_amount_threshold
                .clone()
                .expect("token_b_amount_threshold is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `remove_all_liquidity` CPI accounts.
pub struct RemoveAllLiquidityCpiAccounts<'a, 'b> {
    pub pool_authority: &'b solana_account_info::AccountInfo<'a>,

    pub pool: &'b solana_account_info::AccountInfo<'a>,

    pub position: &'b solana_account_info::AccountInfo<'a>,
    /// The user token a account
    pub token_a_account: &'b solana_account_info::AccountInfo<'a>,
    /// The user token b account
    pub token_b_account: &'b solana_account_info::AccountInfo<'a>,
    /// The vault token account for input token
    pub token_a_vault: &'b solana_account_info::AccountInfo<'a>,
    /// The vault token account for output token
    pub token_b_vault: &'b solana_account_info::AccountInfo<'a>,
    /// The mint of token a
    pub token_a_mint: &'b solana_account_info::AccountInfo<'a>,
    /// The mint of token b
    pub token_b_mint: &'b solana_account_info::AccountInfo<'a>,
    /// The token account for nft
    pub position_nft_account: &'b solana_account_info::AccountInfo<'a>,
    /// owner of position
    pub owner: &'b solana_account_info::AccountInfo<'a>,
    /// Token a program
    pub token_a_program: &'b solana_account_info::AccountInfo<'a>,
    /// Token b program
    pub token_b_program: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
}

/// `remove_all_liquidity` CPI instruction.
pub struct RemoveAllLiquidityCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_account_info::AccountInfo<'a>,

    pub pool_authority: &'b solana_account_info::AccountInfo<'a>,

    pub pool: &'b solana_account_info::AccountInfo<'a>,

    pub position: &'b solana_account_info::AccountInfo<'a>,
    /// The user token a account
    pub token_a_account: &'b solana_account_info::AccountInfo<'a>,
    /// The user token b account
    pub token_b_account: &'b solana_account_info::AccountInfo<'a>,
    /// The vault token account for input token
    pub token_a_vault: &'b solana_account_info::AccountInfo<'a>,
    /// The vault token account for output token
    pub token_b_vault: &'b solana_account_info::AccountInfo<'a>,
    /// The mint of token a
    pub token_a_mint: &'b solana_account_info::AccountInfo<'a>,
    /// The mint of token b
    pub token_b_mint: &'b solana_account_info::AccountInfo<'a>,
    /// The token account for nft
    pub position_nft_account: &'b solana_account_info::AccountInfo<'a>,
    /// owner of position
    pub owner: &'b solana_account_info::AccountInfo<'a>,
    /// Token a program
    pub token_a_program: &'b solana_account_info::AccountInfo<'a>,
    /// Token b program
    pub token_b_program: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: RemoveAllLiquidityInstructionArgs,
}

impl<'a, 'b> RemoveAllLiquidityCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: RemoveAllLiquidityCpiAccounts<'a, 'b>,
        args: RemoveAllLiquidityInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            pool_authority: accounts.pool_authority,
            pool: accounts.pool,
            position: accounts.position,
            token_a_account: accounts.token_a_account,
            token_b_account: accounts.token_b_account,
            token_a_vault: accounts.token_a_vault,
            token_b_vault: accounts.token_b_vault,
            token_a_mint: accounts.token_a_mint,
            token_b_mint: accounts.token_b_mint,
            position_nft_account: accounts.position_nft_account,
            owner: accounts.owner,
            token_a_program: accounts.token_a_program,
            token_b_program: accounts.token_b_program,
            event_authority: accounts.event_authority,
            program: accounts.program,
            __args: args,
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
        let mut accounts = Vec::with_capacity(15 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.pool_authority.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(*self.pool.key, false));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.position.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.token_a_account.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.token_b_account.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.token_a_vault.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.token_b_vault.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_a_mint.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_b_mint.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.position_nft_account.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.owner.key,
            true,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_a_program.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_b_program.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.event_authority.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&RemoveAllLiquidityInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_instruction::Instruction {
            program_id: crate::CP_AMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(16 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.pool_authority.clone());
        account_infos.push(self.pool.clone());
        account_infos.push(self.position.clone());
        account_infos.push(self.token_a_account.clone());
        account_infos.push(self.token_b_account.clone());
        account_infos.push(self.token_a_vault.clone());
        account_infos.push(self.token_b_vault.clone());
        account_infos.push(self.token_a_mint.clone());
        account_infos.push(self.token_b_mint.clone());
        account_infos.push(self.position_nft_account.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.token_a_program.clone());
        account_infos.push(self.token_b_program.clone());
        account_infos.push(self.event_authority.clone());
        account_infos.push(self.program.clone());
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

/// Instruction builder for `RemoveAllLiquidity` via CPI.
///
/// ### Accounts:
///
///   0. `[]` pool_authority
///   1. `[writable]` pool
///   2. `[writable]` position
///   3. `[writable]` token_a_account
///   4. `[writable]` token_b_account
///   5. `[writable]` token_a_vault
///   6. `[writable]` token_b_vault
///   7. `[]` token_a_mint
///   8. `[]` token_b_mint
///   9. `[]` position_nft_account
///   10. `[signer]` owner
///   11. `[]` token_a_program
///   12. `[]` token_b_program
///   13. `[]` event_authority
///   14. `[]` program
#[derive(Clone, Debug)]
pub struct RemoveAllLiquidityCpiBuilder<'a, 'b> {
    instruction: Box<RemoveAllLiquidityCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> RemoveAllLiquidityCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(RemoveAllLiquidityCpiBuilderInstruction {
            __program: program,
            pool_authority: None,
            pool: None,
            position: None,
            token_a_account: None,
            token_b_account: None,
            token_a_vault: None,
            token_b_vault: None,
            token_a_mint: None,
            token_b_mint: None,
            position_nft_account: None,
            owner: None,
            token_a_program: None,
            token_b_program: None,
            event_authority: None,
            program: None,
            token_a_amount_threshold: None,
            token_b_amount_threshold: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
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
    pub fn position(&mut self, position: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.position = Some(position);
        self
    }

    /// The user token a account
    #[inline(always)]
    pub fn token_a_account(
        &mut self,
        token_a_account: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_a_account = Some(token_a_account);
        self
    }

    /// The user token b account
    #[inline(always)]
    pub fn token_b_account(
        &mut self,
        token_b_account: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_b_account = Some(token_b_account);
        self
    }

    /// The vault token account for input token
    #[inline(always)]
    pub fn token_a_vault(
        &mut self,
        token_a_vault: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_a_vault = Some(token_a_vault);
        self
    }

    /// The vault token account for output token
    #[inline(always)]
    pub fn token_b_vault(
        &mut self,
        token_b_vault: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_b_vault = Some(token_b_vault);
        self
    }

    /// The mint of token a
    #[inline(always)]
    pub fn token_a_mint(
        &mut self,
        token_a_mint: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_a_mint = Some(token_a_mint);
        self
    }

    /// The mint of token b
    #[inline(always)]
    pub fn token_b_mint(
        &mut self,
        token_b_mint: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_b_mint = Some(token_b_mint);
        self
    }

    /// The token account for nft
    #[inline(always)]
    pub fn position_nft_account(
        &mut self,
        position_nft_account: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position_nft_account = Some(position_nft_account);
        self
    }

    /// owner of position
    #[inline(always)]
    pub fn owner(&mut self, owner: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.owner = Some(owner);
        self
    }

    /// Token a program
    #[inline(always)]
    pub fn token_a_program(
        &mut self,
        token_a_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_a_program = Some(token_a_program);
        self
    }

    /// Token b program
    #[inline(always)]
    pub fn token_b_program(
        &mut self,
        token_b_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_b_program = Some(token_b_program);
        self
    }

    #[inline(always)]
    pub fn event_authority(
        &mut self,
        event_authority: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.event_authority = Some(event_authority);
        self
    }

    #[inline(always)]
    pub fn program(&mut self, program: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.program = Some(program);
        self
    }

    #[inline(always)]
    pub fn token_a_amount_threshold(&mut self, token_a_amount_threshold: u64) -> &mut Self {
        self.instruction.token_a_amount_threshold = Some(token_a_amount_threshold);
        self
    }

    #[inline(always)]
    pub fn token_b_amount_threshold(&mut self, token_b_amount_threshold: u64) -> &mut Self {
        self.instruction.token_b_amount_threshold = Some(token_b_amount_threshold);
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
        let args = RemoveAllLiquidityInstructionArgs {
            token_a_amount_threshold: self
                .instruction
                .token_a_amount_threshold
                .clone()
                .expect("token_a_amount_threshold is not set"),
            token_b_amount_threshold: self
                .instruction
                .token_b_amount_threshold
                .clone()
                .expect("token_b_amount_threshold is not set"),
        };
        let instruction = RemoveAllLiquidityCpi {
            __program: self.instruction.__program,

            pool_authority: self
                .instruction
                .pool_authority
                .expect("pool_authority is not set"),

            pool: self.instruction.pool.expect("pool is not set"),

            position: self.instruction.position.expect("position is not set"),

            token_a_account: self
                .instruction
                .token_a_account
                .expect("token_a_account is not set"),

            token_b_account: self
                .instruction
                .token_b_account
                .expect("token_b_account is not set"),

            token_a_vault: self
                .instruction
                .token_a_vault
                .expect("token_a_vault is not set"),

            token_b_vault: self
                .instruction
                .token_b_vault
                .expect("token_b_vault is not set"),

            token_a_mint: self
                .instruction
                .token_a_mint
                .expect("token_a_mint is not set"),

            token_b_mint: self
                .instruction
                .token_b_mint
                .expect("token_b_mint is not set"),

            position_nft_account: self
                .instruction
                .position_nft_account
                .expect("position_nft_account is not set"),

            owner: self.instruction.owner.expect("owner is not set"),

            token_a_program: self
                .instruction
                .token_a_program
                .expect("token_a_program is not set"),

            token_b_program: self
                .instruction
                .token_b_program
                .expect("token_b_program is not set"),

            event_authority: self
                .instruction
                .event_authority
                .expect("event_authority is not set"),

            program: self.instruction.program.expect("program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct RemoveAllLiquidityCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    pool_authority: Option<&'b solana_account_info::AccountInfo<'a>>,
    pool: Option<&'b solana_account_info::AccountInfo<'a>>,
    position: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_a_account: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_b_account: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_a_vault: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_b_vault: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_a_mint: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_b_mint: Option<&'b solana_account_info::AccountInfo<'a>>,
    position_nft_account: Option<&'b solana_account_info::AccountInfo<'a>>,
    owner: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_a_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_b_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_account_info::AccountInfo<'a>>,
    program: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_a_amount_threshold: Option<u64>,
    token_b_amount_threshold: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
