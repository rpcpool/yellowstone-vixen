//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

use crate::generated::types::{CompressedBinDepositAmount, RemainingAccountsInfo};

/// Accounts.
#[derive(Debug)]
pub struct AddLiquidityOneSidePrecise2 {
    pub position: solana_program::pubkey::Pubkey,

    pub lb_pair: solana_program::pubkey::Pubkey,

    pub bin_array_bitmap_extension: Option<solana_program::pubkey::Pubkey>,

    pub user_token: solana_program::pubkey::Pubkey,

    pub reserve: solana_program::pubkey::Pubkey,

    pub token_mint: solana_program::pubkey::Pubkey,

    pub sender: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub event_authority: solana_program::pubkey::Pubkey,

    pub program: solana_program::pubkey::Pubkey,
}

impl AddLiquidityOneSidePrecise2 {
    pub fn instruction(
        &self,
        args: AddLiquidityOneSidePrecise2InstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: AddLiquidityOneSidePrecise2InstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(10 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.position,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.lb_pair,
            false,
        ));
        if let Some(bin_array_bitmap_extension) = self.bin_array_bitmap_extension {
            accounts.push(solana_program::instruction::AccountMeta::new(
                bin_array_bitmap_extension,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LB_CLMM_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_token,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.reserve,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.sender,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.event_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&AddLiquidityOneSidePrecise2InstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquidityOneSidePrecise2InstructionData {
    discriminator: [u8; 8],
}

impl AddLiquidityOneSidePrecise2InstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [33, 51, 163, 201, 117, 98, 125, 231],
        }
    }
}

impl Default for AddLiquidityOneSidePrecise2InstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquidityOneSidePrecise2InstructionArgs {
    pub bins: Vec<CompressedBinDepositAmount>,
    pub decompress_multiplier: u64,
    pub max_amount: u64,
    pub remaining_accounts_info: RemainingAccountsInfo,
}

/// Instruction builder for `AddLiquidityOneSidePrecise2`.
///
/// ### Accounts:
///
///   0. `[writable]` position
///   1. `[writable]` lb_pair
///   2. `[writable, optional]` bin_array_bitmap_extension
///   3. `[writable]` user_token
///   4. `[writable]` reserve
///   5. `[]` token_mint
///   6. `[signer]` sender
///   7. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   8. `[]` event_authority
///   9. `[]` program
#[derive(Clone, Debug, Default)]
pub struct AddLiquidityOneSidePrecise2Builder {
    position: Option<solana_program::pubkey::Pubkey>,
    lb_pair: Option<solana_program::pubkey::Pubkey>,
    bin_array_bitmap_extension: Option<solana_program::pubkey::Pubkey>,
    user_token: Option<solana_program::pubkey::Pubkey>,
    reserve: Option<solana_program::pubkey::Pubkey>,
    token_mint: Option<solana_program::pubkey::Pubkey>,
    sender: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    event_authority: Option<solana_program::pubkey::Pubkey>,
    program: Option<solana_program::pubkey::Pubkey>,
    bins: Option<Vec<CompressedBinDepositAmount>>,
    decompress_multiplier: Option<u64>,
    max_amount: Option<u64>,
    remaining_accounts_info: Option<RemainingAccountsInfo>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl AddLiquidityOneSidePrecise2Builder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn position(&mut self, position: solana_program::pubkey::Pubkey) -> &mut Self {
        self.position = Some(position);
        self
    }

    #[inline(always)]
    pub fn lb_pair(&mut self, lb_pair: solana_program::pubkey::Pubkey) -> &mut Self {
        self.lb_pair = Some(lb_pair);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn bin_array_bitmap_extension(
        &mut self,
        bin_array_bitmap_extension: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.bin_array_bitmap_extension = bin_array_bitmap_extension;
        self
    }

    #[inline(always)]
    pub fn user_token(&mut self, user_token: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user_token = Some(user_token);
        self
    }

    #[inline(always)]
    pub fn reserve(&mut self, reserve: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reserve = Some(reserve);
        self
    }

    #[inline(always)]
    pub fn token_mint(&mut self, token_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_mint = Some(token_mint);
        self
    }

    #[inline(always)]
    pub fn sender(&mut self, sender: solana_program::pubkey::Pubkey) -> &mut Self {
        self.sender = Some(sender);
        self
    }

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn event_authority(
        &mut self,
        event_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.event_authority = Some(event_authority);
        self
    }

    #[inline(always)]
    pub fn program(&mut self, program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.program = Some(program);
        self
    }

    #[inline(always)]
    pub fn bins(&mut self, bins: Vec<CompressedBinDepositAmount>) -> &mut Self {
        self.bins = Some(bins);
        self
    }

    #[inline(always)]
    pub fn decompress_multiplier(&mut self, decompress_multiplier: u64) -> &mut Self {
        self.decompress_multiplier = Some(decompress_multiplier);
        self
    }

    #[inline(always)]
    pub fn max_amount(&mut self, max_amount: u64) -> &mut Self {
        self.max_amount = Some(max_amount);
        self
    }

    #[inline(always)]
    pub fn remaining_accounts_info(
        &mut self,
        remaining_accounts_info: RemainingAccountsInfo,
    ) -> &mut Self {
        self.remaining_accounts_info = Some(remaining_accounts_info);
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
        let accounts = AddLiquidityOneSidePrecise2 {
            position: self.position.expect("position is not set"),
            lb_pair: self.lb_pair.expect("lb_pair is not set"),
            bin_array_bitmap_extension: self.bin_array_bitmap_extension,
            user_token: self.user_token.expect("user_token is not set"),
            reserve: self.reserve.expect("reserve is not set"),
            token_mint: self.token_mint.expect("token_mint is not set"),
            sender: self.sender.expect("sender is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = AddLiquidityOneSidePrecise2InstructionArgs {
            bins: self.bins.clone().expect("bins is not set"),
            decompress_multiplier: self
                .decompress_multiplier
                .clone()
                .expect("decompress_multiplier is not set"),
            max_amount: self.max_amount.clone().expect("max_amount is not set"),
            remaining_accounts_info: self
                .remaining_accounts_info
                .clone()
                .expect("remaining_accounts_info is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `add_liquidity_one_side_precise2` CPI accounts.
pub struct AddLiquidityOneSidePrecise2CpiAccounts<'a, 'b> {
    pub position: &'b solana_program::account_info::AccountInfo<'a>,

    pub lb_pair: &'b solana_program::account_info::AccountInfo<'a>,

    pub bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub user_token: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub sender: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `add_liquidity_one_side_precise2` CPI instruction.
pub struct AddLiquidityOneSidePrecise2Cpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub position: &'b solana_program::account_info::AccountInfo<'a>,

    pub lb_pair: &'b solana_program::account_info::AccountInfo<'a>,

    pub bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub user_token: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub sender: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: AddLiquidityOneSidePrecise2InstructionArgs,
}

impl<'a, 'b> AddLiquidityOneSidePrecise2Cpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: AddLiquidityOneSidePrecise2CpiAccounts<'a, 'b>,
        args: AddLiquidityOneSidePrecise2InstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            position: accounts.position,
            lb_pair: accounts.lb_pair,
            bin_array_bitmap_extension: accounts.bin_array_bitmap_extension,
            user_token: accounts.user_token,
            reserve: accounts.reserve,
            token_mint: accounts.token_mint,
            sender: accounts.sender,
            token_program: accounts.token_program,
            event_authority: accounts.event_authority,
            program: accounts.program,
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
        let mut accounts = Vec::with_capacity(10 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lb_pair.key,
            false,
        ));
        if let Some(bin_array_bitmap_extension) = self.bin_array_bitmap_extension {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *bin_array_bitmap_extension.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LB_CLMM_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_token.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reserve.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.sender.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.event_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&AddLiquidityOneSidePrecise2InstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(11 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.position.clone());
        account_infos.push(self.lb_pair.clone());
        if let Some(bin_array_bitmap_extension) = self.bin_array_bitmap_extension {
            account_infos.push(bin_array_bitmap_extension.clone());
        }
        account_infos.push(self.user_token.clone());
        account_infos.push(self.reserve.clone());
        account_infos.push(self.token_mint.clone());
        account_infos.push(self.sender.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.event_authority.clone());
        account_infos.push(self.program.clone());
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

/// Instruction builder for `AddLiquidityOneSidePrecise2` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` position
///   1. `[writable]` lb_pair
///   2. `[writable, optional]` bin_array_bitmap_extension
///   3. `[writable]` user_token
///   4. `[writable]` reserve
///   5. `[]` token_mint
///   6. `[signer]` sender
///   7. `[]` token_program
///   8. `[]` event_authority
///   9. `[]` program
#[derive(Clone, Debug)]
pub struct AddLiquidityOneSidePrecise2CpiBuilder<'a, 'b> {
    instruction: Box<AddLiquidityOneSidePrecise2CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> AddLiquidityOneSidePrecise2CpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(AddLiquidityOneSidePrecise2CpiBuilderInstruction {
            __program: program,
            position: None,
            lb_pair: None,
            bin_array_bitmap_extension: None,
            user_token: None,
            reserve: None,
            token_mint: None,
            sender: None,
            token_program: None,
            event_authority: None,
            program: None,
            bins: None,
            decompress_multiplier: None,
            max_amount: None,
            remaining_accounts_info: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn position(
        &mut self,
        position: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position = Some(position);
        self
    }

    #[inline(always)]
    pub fn lb_pair(
        &mut self,
        lb_pair: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lb_pair = Some(lb_pair);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn bin_array_bitmap_extension(
        &mut self,
        bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.bin_array_bitmap_extension = bin_array_bitmap_extension;
        self
    }

    #[inline(always)]
    pub fn user_token(
        &mut self,
        user_token: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_token = Some(user_token);
        self
    }

    #[inline(always)]
    pub fn reserve(
        &mut self,
        reserve: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reserve = Some(reserve);
        self
    }

    #[inline(always)]
    pub fn token_mint(
        &mut self,
        token_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_mint = Some(token_mint);
        self
    }

    #[inline(always)]
    pub fn sender(
        &mut self,
        sender: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.sender = Some(sender);
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
    pub fn event_authority(
        &mut self,
        event_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.event_authority = Some(event_authority);
        self
    }

    #[inline(always)]
    pub fn program(
        &mut self,
        program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program = Some(program);
        self
    }

    #[inline(always)]
    pub fn bins(&mut self, bins: Vec<CompressedBinDepositAmount>) -> &mut Self {
        self.instruction.bins = Some(bins);
        self
    }

    #[inline(always)]
    pub fn decompress_multiplier(&mut self, decompress_multiplier: u64) -> &mut Self {
        self.instruction.decompress_multiplier = Some(decompress_multiplier);
        self
    }

    #[inline(always)]
    pub fn max_amount(&mut self, max_amount: u64) -> &mut Self {
        self.instruction.max_amount = Some(max_amount);
        self
    }

    #[inline(always)]
    pub fn remaining_accounts_info(
        &mut self,
        remaining_accounts_info: RemainingAccountsInfo,
    ) -> &mut Self {
        self.instruction.remaining_accounts_info = Some(remaining_accounts_info);
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
        let args = AddLiquidityOneSidePrecise2InstructionArgs {
            bins: self.instruction.bins.clone().expect("bins is not set"),
            decompress_multiplier: self
                .instruction
                .decompress_multiplier
                .clone()
                .expect("decompress_multiplier is not set"),
            max_amount: self
                .instruction
                .max_amount
                .clone()
                .expect("max_amount is not set"),
            remaining_accounts_info: self
                .instruction
                .remaining_accounts_info
                .clone()
                .expect("remaining_accounts_info is not set"),
        };
        let instruction = AddLiquidityOneSidePrecise2Cpi {
            __program: self.instruction.__program,

            position: self.instruction.position.expect("position is not set"),

            lb_pair: self.instruction.lb_pair.expect("lb_pair is not set"),

            bin_array_bitmap_extension: self.instruction.bin_array_bitmap_extension,

            user_token: self.instruction.user_token.expect("user_token is not set"),

            reserve: self.instruction.reserve.expect("reserve is not set"),

            token_mint: self.instruction.token_mint.expect("token_mint is not set"),

            sender: self.instruction.sender.expect("sender is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

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
struct AddLiquidityOneSidePrecise2CpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lb_pair: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_token: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reserve: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    sender: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bins: Option<Vec<CompressedBinDepositAmount>>,
    decompress_multiplier: Option<u64>,
    max_amount: Option<u64>,
    remaining_accounts_info: Option<RemainingAccountsInfo>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
