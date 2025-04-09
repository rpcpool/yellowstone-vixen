//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

use crate::generated::types::{BinLiquidityReduction, RemainingAccountsInfo};

/// Accounts.
#[derive(Debug)]
pub struct RemoveLiquidity2 {
    pub position: solana_program::pubkey::Pubkey,

    pub lb_pair: solana_program::pubkey::Pubkey,

    pub bin_array_bitmap_extension: Option<solana_program::pubkey::Pubkey>,

    pub user_token_x: solana_program::pubkey::Pubkey,

    pub user_token_y: solana_program::pubkey::Pubkey,

    pub reserve_x: solana_program::pubkey::Pubkey,

    pub reserve_y: solana_program::pubkey::Pubkey,

    pub token_x_mint: solana_program::pubkey::Pubkey,

    pub token_y_mint: solana_program::pubkey::Pubkey,

    pub sender: solana_program::pubkey::Pubkey,

    pub token_x_program: solana_program::pubkey::Pubkey,

    pub token_y_program: solana_program::pubkey::Pubkey,

    pub memo_program: solana_program::pubkey::Pubkey,

    pub event_authority: solana_program::pubkey::Pubkey,

    pub program: solana_program::pubkey::Pubkey,
}

impl RemoveLiquidity2 {
    pub fn instruction(
        &self,
        args: RemoveLiquidity2InstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: RemoveLiquidity2InstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(15 + remaining_accounts.len());
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
            self.user_token_x,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_token_y,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.reserve_x,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.reserve_y,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_x_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_y_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.sender,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_x_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_y_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.memo_program,
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
        let mut data = borsh::to_vec(&RemoveLiquidity2InstructionData::new()).unwrap();
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
pub struct RemoveLiquidity2InstructionData {
    discriminator: [u8; 8],
}

impl RemoveLiquidity2InstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [230, 215, 82, 127, 241, 101, 227, 146],
        }
    }
}

impl Default for RemoveLiquidity2InstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveLiquidity2InstructionArgs {
    pub bin_liquidity_removal: Vec<BinLiquidityReduction>,
    pub remaining_accounts_info: RemainingAccountsInfo,
}

/// Instruction builder for `RemoveLiquidity2`.
///
/// ### Accounts:
///
///   0. `[writable]` position
///   1. `[writable]` lb_pair
///   2. `[writable, optional]` bin_array_bitmap_extension
///   3. `[writable]` user_token_x
///   4. `[writable]` user_token_y
///   5. `[writable]` reserve_x
///   6. `[writable]` reserve_y
///   7. `[]` token_x_mint
///   8. `[]` token_y_mint
///   9. `[signer]` sender
///   10. `[]` token_x_program
///   11. `[]` token_y_program
///   12. `[]` memo_program
///   13. `[]` event_authority
///   14. `[]` program
#[derive(Clone, Debug, Default)]
pub struct RemoveLiquidity2Builder {
    position: Option<solana_program::pubkey::Pubkey>,
    lb_pair: Option<solana_program::pubkey::Pubkey>,
    bin_array_bitmap_extension: Option<solana_program::pubkey::Pubkey>,
    user_token_x: Option<solana_program::pubkey::Pubkey>,
    user_token_y: Option<solana_program::pubkey::Pubkey>,
    reserve_x: Option<solana_program::pubkey::Pubkey>,
    reserve_y: Option<solana_program::pubkey::Pubkey>,
    token_x_mint: Option<solana_program::pubkey::Pubkey>,
    token_y_mint: Option<solana_program::pubkey::Pubkey>,
    sender: Option<solana_program::pubkey::Pubkey>,
    token_x_program: Option<solana_program::pubkey::Pubkey>,
    token_y_program: Option<solana_program::pubkey::Pubkey>,
    memo_program: Option<solana_program::pubkey::Pubkey>,
    event_authority: Option<solana_program::pubkey::Pubkey>,
    program: Option<solana_program::pubkey::Pubkey>,
    bin_liquidity_removal: Option<Vec<BinLiquidityReduction>>,
    remaining_accounts_info: Option<RemainingAccountsInfo>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl RemoveLiquidity2Builder {
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
    pub fn user_token_x(&mut self, user_token_x: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user_token_x = Some(user_token_x);
        self
    }

    #[inline(always)]
    pub fn user_token_y(&mut self, user_token_y: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user_token_y = Some(user_token_y);
        self
    }

    #[inline(always)]
    pub fn reserve_x(&mut self, reserve_x: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reserve_x = Some(reserve_x);
        self
    }

    #[inline(always)]
    pub fn reserve_y(&mut self, reserve_y: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reserve_y = Some(reserve_y);
        self
    }

    #[inline(always)]
    pub fn token_x_mint(&mut self, token_x_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_x_mint = Some(token_x_mint);
        self
    }

    #[inline(always)]
    pub fn token_y_mint(&mut self, token_y_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_y_mint = Some(token_y_mint);
        self
    }

    #[inline(always)]
    pub fn sender(&mut self, sender: solana_program::pubkey::Pubkey) -> &mut Self {
        self.sender = Some(sender);
        self
    }

    #[inline(always)]
    pub fn token_x_program(
        &mut self,
        token_x_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_x_program = Some(token_x_program);
        self
    }

    #[inline(always)]
    pub fn token_y_program(
        &mut self,
        token_y_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_y_program = Some(token_y_program);
        self
    }

    #[inline(always)]
    pub fn memo_program(&mut self, memo_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.memo_program = Some(memo_program);
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
    pub fn bin_liquidity_removal(
        &mut self,
        bin_liquidity_removal: Vec<BinLiquidityReduction>,
    ) -> &mut Self {
        self.bin_liquidity_removal = Some(bin_liquidity_removal);
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
        let accounts = RemoveLiquidity2 {
            position: self.position.expect("position is not set"),
            lb_pair: self.lb_pair.expect("lb_pair is not set"),
            bin_array_bitmap_extension: self.bin_array_bitmap_extension,
            user_token_x: self.user_token_x.expect("user_token_x is not set"),
            user_token_y: self.user_token_y.expect("user_token_y is not set"),
            reserve_x: self.reserve_x.expect("reserve_x is not set"),
            reserve_y: self.reserve_y.expect("reserve_y is not set"),
            token_x_mint: self.token_x_mint.expect("token_x_mint is not set"),
            token_y_mint: self.token_y_mint.expect("token_y_mint is not set"),
            sender: self.sender.expect("sender is not set"),
            token_x_program: self.token_x_program.expect("token_x_program is not set"),
            token_y_program: self.token_y_program.expect("token_y_program is not set"),
            memo_program: self.memo_program.expect("memo_program is not set"),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = RemoveLiquidity2InstructionArgs {
            bin_liquidity_removal: self
                .bin_liquidity_removal
                .clone()
                .expect("bin_liquidity_removal is not set"),
            remaining_accounts_info: self
                .remaining_accounts_info
                .clone()
                .expect("remaining_accounts_info is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `remove_liquidity2` CPI accounts.
pub struct RemoveLiquidity2CpiAccounts<'a, 'b> {
    pub position: &'b solana_program::account_info::AccountInfo<'a>,

    pub lb_pair: &'b solana_program::account_info::AccountInfo<'a>,

    pub bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub user_token_x: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_y: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve_x: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve_y: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_x_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_y_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub sender: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_x_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_y_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub memo_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `remove_liquidity2` CPI instruction.
pub struct RemoveLiquidity2Cpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub position: &'b solana_program::account_info::AccountInfo<'a>,

    pub lb_pair: &'b solana_program::account_info::AccountInfo<'a>,

    pub bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub user_token_x: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_y: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve_x: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve_y: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_x_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_y_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub sender: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_x_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_y_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub memo_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: RemoveLiquidity2InstructionArgs,
}

impl<'a, 'b> RemoveLiquidity2Cpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: RemoveLiquidity2CpiAccounts<'a, 'b>,
        args: RemoveLiquidity2InstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            position: accounts.position,
            lb_pair: accounts.lb_pair,
            bin_array_bitmap_extension: accounts.bin_array_bitmap_extension,
            user_token_x: accounts.user_token_x,
            user_token_y: accounts.user_token_y,
            reserve_x: accounts.reserve_x,
            reserve_y: accounts.reserve_y,
            token_x_mint: accounts.token_x_mint,
            token_y_mint: accounts.token_y_mint,
            sender: accounts.sender,
            token_x_program: accounts.token_x_program,
            token_y_program: accounts.token_y_program,
            memo_program: accounts.memo_program,
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
        let mut accounts = Vec::with_capacity(15 + remaining_accounts.len());
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
            *self.user_token_x.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_token_y.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reserve_x.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reserve_y.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_x_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_y_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.sender.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_x_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_y_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.memo_program.key,
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
        let mut data = borsh::to_vec(&RemoveLiquidity2InstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(16 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.position.clone());
        account_infos.push(self.lb_pair.clone());
        if let Some(bin_array_bitmap_extension) = self.bin_array_bitmap_extension {
            account_infos.push(bin_array_bitmap_extension.clone());
        }
        account_infos.push(self.user_token_x.clone());
        account_infos.push(self.user_token_y.clone());
        account_infos.push(self.reserve_x.clone());
        account_infos.push(self.reserve_y.clone());
        account_infos.push(self.token_x_mint.clone());
        account_infos.push(self.token_y_mint.clone());
        account_infos.push(self.sender.clone());
        account_infos.push(self.token_x_program.clone());
        account_infos.push(self.token_y_program.clone());
        account_infos.push(self.memo_program.clone());
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

/// Instruction builder for `RemoveLiquidity2` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` position
///   1. `[writable]` lb_pair
///   2. `[writable, optional]` bin_array_bitmap_extension
///   3. `[writable]` user_token_x
///   4. `[writable]` user_token_y
///   5. `[writable]` reserve_x
///   6. `[writable]` reserve_y
///   7. `[]` token_x_mint
///   8. `[]` token_y_mint
///   9. `[signer]` sender
///   10. `[]` token_x_program
///   11. `[]` token_y_program
///   12. `[]` memo_program
///   13. `[]` event_authority
///   14. `[]` program
#[derive(Clone, Debug)]
pub struct RemoveLiquidity2CpiBuilder<'a, 'b> {
    instruction: Box<RemoveLiquidity2CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> RemoveLiquidity2CpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(RemoveLiquidity2CpiBuilderInstruction {
            __program: program,
            position: None,
            lb_pair: None,
            bin_array_bitmap_extension: None,
            user_token_x: None,
            user_token_y: None,
            reserve_x: None,
            reserve_y: None,
            token_x_mint: None,
            token_y_mint: None,
            sender: None,
            token_x_program: None,
            token_y_program: None,
            memo_program: None,
            event_authority: None,
            program: None,
            bin_liquidity_removal: None,
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
    pub fn user_token_x(
        &mut self,
        user_token_x: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_token_x = Some(user_token_x);
        self
    }

    #[inline(always)]
    pub fn user_token_y(
        &mut self,
        user_token_y: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_token_y = Some(user_token_y);
        self
    }

    #[inline(always)]
    pub fn reserve_x(
        &mut self,
        reserve_x: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reserve_x = Some(reserve_x);
        self
    }

    #[inline(always)]
    pub fn reserve_y(
        &mut self,
        reserve_y: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reserve_y = Some(reserve_y);
        self
    }

    #[inline(always)]
    pub fn token_x_mint(
        &mut self,
        token_x_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_x_mint = Some(token_x_mint);
        self
    }

    #[inline(always)]
    pub fn token_y_mint(
        &mut self,
        token_y_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_y_mint = Some(token_y_mint);
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
    pub fn token_x_program(
        &mut self,
        token_x_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_x_program = Some(token_x_program);
        self
    }

    #[inline(always)]
    pub fn token_y_program(
        &mut self,
        token_y_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_y_program = Some(token_y_program);
        self
    }

    #[inline(always)]
    pub fn memo_program(
        &mut self,
        memo_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.memo_program = Some(memo_program);
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
    pub fn bin_liquidity_removal(
        &mut self,
        bin_liquidity_removal: Vec<BinLiquidityReduction>,
    ) -> &mut Self {
        self.instruction.bin_liquidity_removal = Some(bin_liquidity_removal);
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
        let args = RemoveLiquidity2InstructionArgs {
            bin_liquidity_removal: self
                .instruction
                .bin_liquidity_removal
                .clone()
                .expect("bin_liquidity_removal is not set"),
            remaining_accounts_info: self
                .instruction
                .remaining_accounts_info
                .clone()
                .expect("remaining_accounts_info is not set"),
        };
        let instruction = RemoveLiquidity2Cpi {
            __program: self.instruction.__program,

            position: self.instruction.position.expect("position is not set"),

            lb_pair: self.instruction.lb_pair.expect("lb_pair is not set"),

            bin_array_bitmap_extension: self.instruction.bin_array_bitmap_extension,

            user_token_x: self
                .instruction
                .user_token_x
                .expect("user_token_x is not set"),

            user_token_y: self
                .instruction
                .user_token_y
                .expect("user_token_y is not set"),

            reserve_x: self.instruction.reserve_x.expect("reserve_x is not set"),

            reserve_y: self.instruction.reserve_y.expect("reserve_y is not set"),

            token_x_mint: self
                .instruction
                .token_x_mint
                .expect("token_x_mint is not set"),

            token_y_mint: self
                .instruction
                .token_y_mint
                .expect("token_y_mint is not set"),

            sender: self.instruction.sender.expect("sender is not set"),

            token_x_program: self
                .instruction
                .token_x_program
                .expect("token_x_program is not set"),

            token_y_program: self
                .instruction
                .token_y_program
                .expect("token_y_program is not set"),

            memo_program: self
                .instruction
                .memo_program
                .expect("memo_program is not set"),

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
struct RemoveLiquidity2CpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lb_pair: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_token_x: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_token_y: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reserve_x: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reserve_y: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_x_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_y_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    sender: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_x_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_y_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    memo_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bin_liquidity_removal: Option<Vec<BinLiquidityReduction>>,
    remaining_accounts_info: Option<RemainingAccountsInfo>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
