//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct ClaimCreatorTradingFee {
    pub pool_authority: solana_pubkey::Pubkey,

    pub pool: solana_pubkey::Pubkey,
    /// The treasury token a account
    pub token_a_account: solana_pubkey::Pubkey,
    /// The treasury token b account
    pub token_b_account: solana_pubkey::Pubkey,
    /// The vault token account for input token
    pub base_vault: solana_pubkey::Pubkey,
    /// The vault token account for output token
    pub quote_vault: solana_pubkey::Pubkey,
    /// The mint of token a
    pub base_mint: solana_pubkey::Pubkey,
    /// The mint of token b
    pub quote_mint: solana_pubkey::Pubkey,

    pub creator: solana_pubkey::Pubkey,
    /// Token a program
    pub token_base_program: solana_pubkey::Pubkey,
    /// Token b program
    pub token_quote_program: solana_pubkey::Pubkey,

    pub event_authority: solana_pubkey::Pubkey,

    pub program: solana_pubkey::Pubkey,
}

impl ClaimCreatorTradingFee {
    pub fn instruction(
        &self,
        args: ClaimCreatorTradingFeeInstructionArgs,
    ) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: ClaimCreatorTradingFeeInstructionArgs,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(13 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.pool_authority,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(self.pool, false));
        accounts.push(solana_instruction::AccountMeta::new(
            self.token_a_account,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.token_b_account,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(self.base_vault, false));
        accounts.push(solana_instruction::AccountMeta::new(
            self.quote_vault,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.base_mint,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.quote_mint,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.creator,
            true,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_base_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_quote_program,
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
        let mut data = borsh::to_vec(&ClaimCreatorTradingFeeInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_instruction::Instruction {
            program_id: crate::DYNAMIC_BONDING_CURVE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimCreatorTradingFeeInstructionData {
    discriminator: [u8; 8],
}

impl ClaimCreatorTradingFeeInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [82, 220, 250, 189, 3, 85, 107, 45],
        }
    }
}

impl Default for ClaimCreatorTradingFeeInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimCreatorTradingFeeInstructionArgs {
    pub max_base_amount: u64,
    pub max_quote_amount: u64,
}

/// Instruction builder for `ClaimCreatorTradingFee`.
///
/// ### Accounts:
///
///   0. `[optional]` pool_authority (default to `FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM`)
///   1. `[writable]` pool
///   2. `[writable]` token_a_account
///   3. `[writable]` token_b_account
///   4. `[writable]` base_vault
///   5. `[writable]` quote_vault
///   6. `[]` base_mint
///   7. `[]` quote_mint
///   8. `[signer]` creator
///   9. `[]` token_base_program
///   10. `[]` token_quote_program
///   11. `[]` event_authority
///   12. `[]` program
#[derive(Clone, Debug, Default)]
pub struct ClaimCreatorTradingFeeBuilder {
    pool_authority: Option<solana_pubkey::Pubkey>,
    pool: Option<solana_pubkey::Pubkey>,
    token_a_account: Option<solana_pubkey::Pubkey>,
    token_b_account: Option<solana_pubkey::Pubkey>,
    base_vault: Option<solana_pubkey::Pubkey>,
    quote_vault: Option<solana_pubkey::Pubkey>,
    base_mint: Option<solana_pubkey::Pubkey>,
    quote_mint: Option<solana_pubkey::Pubkey>,
    creator: Option<solana_pubkey::Pubkey>,
    token_base_program: Option<solana_pubkey::Pubkey>,
    token_quote_program: Option<solana_pubkey::Pubkey>,
    event_authority: Option<solana_pubkey::Pubkey>,
    program: Option<solana_pubkey::Pubkey>,
    max_base_amount: Option<u64>,
    max_quote_amount: Option<u64>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl ClaimCreatorTradingFeeBuilder {
    pub fn new() -> Self { Self::default() }

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

    /// The treasury token a account
    #[inline(always)]
    pub fn token_a_account(&mut self, token_a_account: solana_pubkey::Pubkey) -> &mut Self {
        self.token_a_account = Some(token_a_account);
        self
    }

    /// The treasury token b account
    #[inline(always)]
    pub fn token_b_account(&mut self, token_b_account: solana_pubkey::Pubkey) -> &mut Self {
        self.token_b_account = Some(token_b_account);
        self
    }

    /// The vault token account for input token
    #[inline(always)]
    pub fn base_vault(&mut self, base_vault: solana_pubkey::Pubkey) -> &mut Self {
        self.base_vault = Some(base_vault);
        self
    }

    /// The vault token account for output token
    #[inline(always)]
    pub fn quote_vault(&mut self, quote_vault: solana_pubkey::Pubkey) -> &mut Self {
        self.quote_vault = Some(quote_vault);
        self
    }

    /// The mint of token a
    #[inline(always)]
    pub fn base_mint(&mut self, base_mint: solana_pubkey::Pubkey) -> &mut Self {
        self.base_mint = Some(base_mint);
        self
    }

    /// The mint of token b
    #[inline(always)]
    pub fn quote_mint(&mut self, quote_mint: solana_pubkey::Pubkey) -> &mut Self {
        self.quote_mint = Some(quote_mint);
        self
    }

    #[inline(always)]
    pub fn creator(&mut self, creator: solana_pubkey::Pubkey) -> &mut Self {
        self.creator = Some(creator);
        self
    }

    /// Token a program
    #[inline(always)]
    pub fn token_base_program(&mut self, token_base_program: solana_pubkey::Pubkey) -> &mut Self {
        self.token_base_program = Some(token_base_program);
        self
    }

    /// Token b program
    #[inline(always)]
    pub fn token_quote_program(&mut self, token_quote_program: solana_pubkey::Pubkey) -> &mut Self {
        self.token_quote_program = Some(token_quote_program);
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
    pub fn max_base_amount(&mut self, max_base_amount: u64) -> &mut Self {
        self.max_base_amount = Some(max_base_amount);
        self
    }

    #[inline(always)]
    pub fn max_quote_amount(&mut self, max_quote_amount: u64) -> &mut Self {
        self.max_quote_amount = Some(max_quote_amount);
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
        let accounts = ClaimCreatorTradingFee {
            pool_authority: self.pool_authority.unwrap_or(solana_pubkey::pubkey!(
                "FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM"
            )),
            pool: self.pool.expect("pool is not set"),
            token_a_account: self.token_a_account.expect("token_a_account is not set"),
            token_b_account: self.token_b_account.expect("token_b_account is not set"),
            base_vault: self.base_vault.expect("base_vault is not set"),
            quote_vault: self.quote_vault.expect("quote_vault is not set"),
            base_mint: self.base_mint.expect("base_mint is not set"),
            quote_mint: self.quote_mint.expect("quote_mint is not set"),
            creator: self.creator.expect("creator is not set"),
            token_base_program: self
                .token_base_program
                .expect("token_base_program is not set"),
            token_quote_program: self
                .token_quote_program
                .expect("token_quote_program is not set"),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = ClaimCreatorTradingFeeInstructionArgs {
            max_base_amount: self
                .max_base_amount
                .clone()
                .expect("max_base_amount is not set"),
            max_quote_amount: self
                .max_quote_amount
                .clone()
                .expect("max_quote_amount is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `claim_creator_trading_fee` CPI accounts.
pub struct ClaimCreatorTradingFeeCpiAccounts<'a, 'b> {
    pub pool_authority: &'b solana_account_info::AccountInfo<'a>,

    pub pool: &'b solana_account_info::AccountInfo<'a>,
    /// The treasury token a account
    pub token_a_account: &'b solana_account_info::AccountInfo<'a>,
    /// The treasury token b account
    pub token_b_account: &'b solana_account_info::AccountInfo<'a>,
    /// The vault token account for input token
    pub base_vault: &'b solana_account_info::AccountInfo<'a>,
    /// The vault token account for output token
    pub quote_vault: &'b solana_account_info::AccountInfo<'a>,
    /// The mint of token a
    pub base_mint: &'b solana_account_info::AccountInfo<'a>,
    /// The mint of token b
    pub quote_mint: &'b solana_account_info::AccountInfo<'a>,

    pub creator: &'b solana_account_info::AccountInfo<'a>,
    /// Token a program
    pub token_base_program: &'b solana_account_info::AccountInfo<'a>,
    /// Token b program
    pub token_quote_program: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
}

/// `claim_creator_trading_fee` CPI instruction.
pub struct ClaimCreatorTradingFeeCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_account_info::AccountInfo<'a>,

    pub pool_authority: &'b solana_account_info::AccountInfo<'a>,

    pub pool: &'b solana_account_info::AccountInfo<'a>,
    /// The treasury token a account
    pub token_a_account: &'b solana_account_info::AccountInfo<'a>,
    /// The treasury token b account
    pub token_b_account: &'b solana_account_info::AccountInfo<'a>,
    /// The vault token account for input token
    pub base_vault: &'b solana_account_info::AccountInfo<'a>,
    /// The vault token account for output token
    pub quote_vault: &'b solana_account_info::AccountInfo<'a>,
    /// The mint of token a
    pub base_mint: &'b solana_account_info::AccountInfo<'a>,
    /// The mint of token b
    pub quote_mint: &'b solana_account_info::AccountInfo<'a>,

    pub creator: &'b solana_account_info::AccountInfo<'a>,
    /// Token a program
    pub token_base_program: &'b solana_account_info::AccountInfo<'a>,
    /// Token b program
    pub token_quote_program: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: ClaimCreatorTradingFeeInstructionArgs,
}

impl<'a, 'b> ClaimCreatorTradingFeeCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: ClaimCreatorTradingFeeCpiAccounts<'a, 'b>,
        args: ClaimCreatorTradingFeeInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            pool_authority: accounts.pool_authority,
            pool: accounts.pool,
            token_a_account: accounts.token_a_account,
            token_b_account: accounts.token_b_account,
            base_vault: accounts.base_vault,
            quote_vault: accounts.quote_vault,
            base_mint: accounts.base_mint,
            quote_mint: accounts.quote_mint,
            creator: accounts.creator,
            token_base_program: accounts.token_base_program,
            token_quote_program: accounts.token_quote_program,
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
        let mut accounts = Vec::with_capacity(13 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.pool_authority.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(*self.pool.key, false));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.token_a_account.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.token_b_account.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.base_vault.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.quote_vault.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.base_mint.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.quote_mint.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.creator.key,
            true,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_base_program.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_quote_program.key,
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
        let mut data = borsh::to_vec(&ClaimCreatorTradingFeeInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_instruction::Instruction {
            program_id: crate::DYNAMIC_BONDING_CURVE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(14 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.pool_authority.clone());
        account_infos.push(self.pool.clone());
        account_infos.push(self.token_a_account.clone());
        account_infos.push(self.token_b_account.clone());
        account_infos.push(self.base_vault.clone());
        account_infos.push(self.quote_vault.clone());
        account_infos.push(self.base_mint.clone());
        account_infos.push(self.quote_mint.clone());
        account_infos.push(self.creator.clone());
        account_infos.push(self.token_base_program.clone());
        account_infos.push(self.token_quote_program.clone());
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

/// Instruction builder for `ClaimCreatorTradingFee` via CPI.
///
/// ### Accounts:
///
///   0. `[]` pool_authority
///   1. `[writable]` pool
///   2. `[writable]` token_a_account
///   3. `[writable]` token_b_account
///   4. `[writable]` base_vault
///   5. `[writable]` quote_vault
///   6. `[]` base_mint
///   7. `[]` quote_mint
///   8. `[signer]` creator
///   9. `[]` token_base_program
///   10. `[]` token_quote_program
///   11. `[]` event_authority
///   12. `[]` program
#[derive(Clone, Debug)]
pub struct ClaimCreatorTradingFeeCpiBuilder<'a, 'b> {
    instruction: Box<ClaimCreatorTradingFeeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ClaimCreatorTradingFeeCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ClaimCreatorTradingFeeCpiBuilderInstruction {
            __program: program,
            pool_authority: None,
            pool: None,
            token_a_account: None,
            token_b_account: None,
            base_vault: None,
            quote_vault: None,
            base_mint: None,
            quote_mint: None,
            creator: None,
            token_base_program: None,
            token_quote_program: None,
            event_authority: None,
            program: None,
            max_base_amount: None,
            max_quote_amount: None,
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

    /// The treasury token a account
    #[inline(always)]
    pub fn token_a_account(
        &mut self,
        token_a_account: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_a_account = Some(token_a_account);
        self
    }

    /// The treasury token b account
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
    pub fn base_vault(
        &mut self,
        base_vault: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.base_vault = Some(base_vault);
        self
    }

    /// The vault token account for output token
    #[inline(always)]
    pub fn quote_vault(
        &mut self,
        quote_vault: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.quote_vault = Some(quote_vault);
        self
    }

    /// The mint of token a
    #[inline(always)]
    pub fn base_mint(&mut self, base_mint: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.base_mint = Some(base_mint);
        self
    }

    /// The mint of token b
    #[inline(always)]
    pub fn quote_mint(
        &mut self,
        quote_mint: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.quote_mint = Some(quote_mint);
        self
    }

    #[inline(always)]
    pub fn creator(&mut self, creator: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.creator = Some(creator);
        self
    }

    /// Token a program
    #[inline(always)]
    pub fn token_base_program(
        &mut self,
        token_base_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_base_program = Some(token_base_program);
        self
    }

    /// Token b program
    #[inline(always)]
    pub fn token_quote_program(
        &mut self,
        token_quote_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_quote_program = Some(token_quote_program);
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
    pub fn max_base_amount(&mut self, max_base_amount: u64) -> &mut Self {
        self.instruction.max_base_amount = Some(max_base_amount);
        self
    }

    #[inline(always)]
    pub fn max_quote_amount(&mut self, max_quote_amount: u64) -> &mut Self {
        self.instruction.max_quote_amount = Some(max_quote_amount);
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
        let args = ClaimCreatorTradingFeeInstructionArgs {
            max_base_amount: self
                .instruction
                .max_base_amount
                .clone()
                .expect("max_base_amount is not set"),
            max_quote_amount: self
                .instruction
                .max_quote_amount
                .clone()
                .expect("max_quote_amount is not set"),
        };
        let instruction = ClaimCreatorTradingFeeCpi {
            __program: self.instruction.__program,

            pool_authority: self
                .instruction
                .pool_authority
                .expect("pool_authority is not set"),

            pool: self.instruction.pool.expect("pool is not set"),

            token_a_account: self
                .instruction
                .token_a_account
                .expect("token_a_account is not set"),

            token_b_account: self
                .instruction
                .token_b_account
                .expect("token_b_account is not set"),

            base_vault: self.instruction.base_vault.expect("base_vault is not set"),

            quote_vault: self
                .instruction
                .quote_vault
                .expect("quote_vault is not set"),

            base_mint: self.instruction.base_mint.expect("base_mint is not set"),

            quote_mint: self.instruction.quote_mint.expect("quote_mint is not set"),

            creator: self.instruction.creator.expect("creator is not set"),

            token_base_program: self
                .instruction
                .token_base_program
                .expect("token_base_program is not set"),

            token_quote_program: self
                .instruction
                .token_quote_program
                .expect("token_quote_program is not set"),

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
struct ClaimCreatorTradingFeeCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    pool_authority: Option<&'b solana_account_info::AccountInfo<'a>>,
    pool: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_a_account: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_b_account: Option<&'b solana_account_info::AccountInfo<'a>>,
    base_vault: Option<&'b solana_account_info::AccountInfo<'a>>,
    quote_vault: Option<&'b solana_account_info::AccountInfo<'a>>,
    base_mint: Option<&'b solana_account_info::AccountInfo<'a>>,
    quote_mint: Option<&'b solana_account_info::AccountInfo<'a>>,
    creator: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_base_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_quote_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_account_info::AccountInfo<'a>>,
    program: Option<&'b solana_account_info::AccountInfo<'a>>,
    max_base_amount: Option<u64>,
    max_quote_amount: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
