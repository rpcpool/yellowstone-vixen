//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

use crate::generated::types::SwapReduceOnly;

/// Accounts.
#[derive(Debug)]
pub struct EndSwap {
    pub state: solana_program::pubkey::Pubkey,

    pub user: solana_program::pubkey::Pubkey,

    pub user_stats: solana_program::pubkey::Pubkey,

    pub authority: solana_program::pubkey::Pubkey,

    pub out_spot_market_vault: solana_program::pubkey::Pubkey,

    pub in_spot_market_vault: solana_program::pubkey::Pubkey,

    pub out_token_account: solana_program::pubkey::Pubkey,

    pub in_token_account: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub drift_signer: solana_program::pubkey::Pubkey,
    /// Instructions Sysvar for instruction introspection
    pub instructions: solana_program::pubkey::Pubkey,
}

impl EndSwap {
    pub fn instruction(
        &self,
        args: EndSwapInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: EndSwapInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.state, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_stats,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.out_spot_market_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.in_spot_market_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.out_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.in_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.drift_signer,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.instructions,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&EndSwapInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::DRIFT_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndSwapInstructionData {
    discriminator: [u8; 8],
}

impl EndSwapInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [177, 184, 27, 193, 34, 13, 210, 145],
        }
    }
}

impl Default for EndSwapInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndSwapInstructionArgs {
    pub in_market_index: u16,
    pub out_market_index: u16,
    pub limit_price: Option<u64>,
    pub reduce_only: Option<SwapReduceOnly>,
}

/// Instruction builder for `EndSwap`.
///
/// ### Accounts:
///
///   0. `[]` state
///   1. `[writable]` user
///   2. `[writable]` user_stats
///   3. `[signer]` authority
///   4. `[writable]` out_spot_market_vault
///   5. `[writable]` in_spot_market_vault
///   6. `[writable]` out_token_account
///   7. `[writable]` in_token_account
///   8. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   9. `[]` drift_signer
///   10. `[]` instructions
#[derive(Clone, Debug, Default)]
pub struct EndSwapBuilder {
    state: Option<solana_program::pubkey::Pubkey>,
    user: Option<solana_program::pubkey::Pubkey>,
    user_stats: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    out_spot_market_vault: Option<solana_program::pubkey::Pubkey>,
    in_spot_market_vault: Option<solana_program::pubkey::Pubkey>,
    out_token_account: Option<solana_program::pubkey::Pubkey>,
    in_token_account: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    drift_signer: Option<solana_program::pubkey::Pubkey>,
    instructions: Option<solana_program::pubkey::Pubkey>,
    in_market_index: Option<u16>,
    out_market_index: Option<u16>,
    limit_price: Option<u64>,
    reduce_only: Option<SwapReduceOnly>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl EndSwapBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn state(&mut self, state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn user(&mut self, user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user = Some(user);
        self
    }

    #[inline(always)]
    pub fn user_stats(&mut self, user_stats: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user_stats = Some(user_stats);
        self
    }

    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }

    #[inline(always)]
    pub fn out_spot_market_vault(
        &mut self,
        out_spot_market_vault: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.out_spot_market_vault = Some(out_spot_market_vault);
        self
    }

    #[inline(always)]
    pub fn in_spot_market_vault(
        &mut self,
        in_spot_market_vault: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.in_spot_market_vault = Some(in_spot_market_vault);
        self
    }

    #[inline(always)]
    pub fn out_token_account(
        &mut self,
        out_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.out_token_account = Some(out_token_account);
        self
    }

    #[inline(always)]
    pub fn in_token_account(
        &mut self,
        in_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.in_token_account = Some(in_token_account);
        self
    }

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn drift_signer(&mut self, drift_signer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.drift_signer = Some(drift_signer);
        self
    }

    /// Instructions Sysvar for instruction introspection
    #[inline(always)]
    pub fn instructions(&mut self, instructions: solana_program::pubkey::Pubkey) -> &mut Self {
        self.instructions = Some(instructions);
        self
    }

    #[inline(always)]
    pub fn in_market_index(&mut self, in_market_index: u16) -> &mut Self {
        self.in_market_index = Some(in_market_index);
        self
    }

    #[inline(always)]
    pub fn out_market_index(&mut self, out_market_index: u16) -> &mut Self {
        self.out_market_index = Some(out_market_index);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn limit_price(&mut self, limit_price: u64) -> &mut Self {
        self.limit_price = Some(limit_price);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn reduce_only(&mut self, reduce_only: SwapReduceOnly) -> &mut Self {
        self.reduce_only = Some(reduce_only);
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
        let accounts = EndSwap {
            state: self.state.expect("state is not set"),
            user: self.user.expect("user is not set"),
            user_stats: self.user_stats.expect("user_stats is not set"),
            authority: self.authority.expect("authority is not set"),
            out_spot_market_vault: self
                .out_spot_market_vault
                .expect("out_spot_market_vault is not set"),
            in_spot_market_vault: self
                .in_spot_market_vault
                .expect("in_spot_market_vault is not set"),
            out_token_account: self
                .out_token_account
                .expect("out_token_account is not set"),
            in_token_account: self.in_token_account.expect("in_token_account is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            drift_signer: self.drift_signer.expect("drift_signer is not set"),
            instructions: self.instructions.expect("instructions is not set"),
        };
        let args = EndSwapInstructionArgs {
            in_market_index: self
                .in_market_index
                .clone()
                .expect("in_market_index is not set"),
            out_market_index: self
                .out_market_index
                .clone()
                .expect("out_market_index is not set"),
            limit_price: self.limit_price.clone(),
            reduce_only: self.reduce_only.clone(),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `end_swap` CPI accounts.
pub struct EndSwapCpiAccounts<'a, 'b> {
    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_stats: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub out_spot_market_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub in_spot_market_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub out_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub in_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub drift_signer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Instructions Sysvar for instruction introspection
    pub instructions: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `end_swap` CPI instruction.
pub struct EndSwapCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_stats: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub out_spot_market_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub in_spot_market_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub out_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub in_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub drift_signer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Instructions Sysvar for instruction introspection
    pub instructions: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: EndSwapInstructionArgs,
}

impl<'a, 'b> EndSwapCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: EndSwapCpiAccounts<'a, 'b>,
        args: EndSwapInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            state: accounts.state,
            user: accounts.user,
            user_stats: accounts.user_stats,
            authority: accounts.authority,
            out_spot_market_vault: accounts.out_spot_market_vault,
            in_spot_market_vault: accounts.in_spot_market_vault,
            out_token_account: accounts.out_token_account,
            in_token_account: accounts.in_token_account,
            token_program: accounts.token_program,
            drift_signer: accounts.drift_signer,
            instructions: accounts.instructions,
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
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_stats.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.out_spot_market_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.in_spot_market_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.out_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.in_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.drift_signer.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.instructions.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&EndSwapInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::DRIFT_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(12 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.state.clone());
        account_infos.push(self.user.clone());
        account_infos.push(self.user_stats.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.out_spot_market_vault.clone());
        account_infos.push(self.in_spot_market_vault.clone());
        account_infos.push(self.out_token_account.clone());
        account_infos.push(self.in_token_account.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.drift_signer.clone());
        account_infos.push(self.instructions.clone());
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

/// Instruction builder for `EndSwap` via CPI.
///
/// ### Accounts:
///
///   0. `[]` state
///   1. `[writable]` user
///   2. `[writable]` user_stats
///   3. `[signer]` authority
///   4. `[writable]` out_spot_market_vault
///   5. `[writable]` in_spot_market_vault
///   6. `[writable]` out_token_account
///   7. `[writable]` in_token_account
///   8. `[]` token_program
///   9. `[]` drift_signer
///   10. `[]` instructions
#[derive(Clone, Debug)]
pub struct EndSwapCpiBuilder<'a, 'b> {
    instruction: Box<EndSwapCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> EndSwapCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(EndSwapCpiBuilderInstruction {
            __program: program,
            state: None,
            user: None,
            user_stats: None,
            authority: None,
            out_spot_market_vault: None,
            in_spot_market_vault: None,
            out_token_account: None,
            in_token_account: None,
            token_program: None,
            drift_signer: None,
            instructions: None,
            in_market_index: None,
            out_market_index: None,
            limit_price: None,
            reduce_only: None,
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
    pub fn user(&mut self, user: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.user = Some(user);
        self
    }

    #[inline(always)]
    pub fn user_stats(
        &mut self,
        user_stats: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_stats = Some(user_stats);
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
    pub fn out_spot_market_vault(
        &mut self,
        out_spot_market_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.out_spot_market_vault = Some(out_spot_market_vault);
        self
    }

    #[inline(always)]
    pub fn in_spot_market_vault(
        &mut self,
        in_spot_market_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.in_spot_market_vault = Some(in_spot_market_vault);
        self
    }

    #[inline(always)]
    pub fn out_token_account(
        &mut self,
        out_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.out_token_account = Some(out_token_account);
        self
    }

    #[inline(always)]
    pub fn in_token_account(
        &mut self,
        in_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.in_token_account = Some(in_token_account);
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
    pub fn drift_signer(
        &mut self,
        drift_signer: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.drift_signer = Some(drift_signer);
        self
    }

    /// Instructions Sysvar for instruction introspection
    #[inline(always)]
    pub fn instructions(
        &mut self,
        instructions: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.instructions = Some(instructions);
        self
    }

    #[inline(always)]
    pub fn in_market_index(&mut self, in_market_index: u16) -> &mut Self {
        self.instruction.in_market_index = Some(in_market_index);
        self
    }

    #[inline(always)]
    pub fn out_market_index(&mut self, out_market_index: u16) -> &mut Self {
        self.instruction.out_market_index = Some(out_market_index);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn limit_price(&mut self, limit_price: u64) -> &mut Self {
        self.instruction.limit_price = Some(limit_price);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn reduce_only(&mut self, reduce_only: SwapReduceOnly) -> &mut Self {
        self.instruction.reduce_only = Some(reduce_only);
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
        let args = EndSwapInstructionArgs {
            in_market_index: self
                .instruction
                .in_market_index
                .clone()
                .expect("in_market_index is not set"),
            out_market_index: self
                .instruction
                .out_market_index
                .clone()
                .expect("out_market_index is not set"),
            limit_price: self.instruction.limit_price.clone(),
            reduce_only: self.instruction.reduce_only.clone(),
        };
        let instruction = EndSwapCpi {
            __program: self.instruction.__program,

            state: self.instruction.state.expect("state is not set"),

            user: self.instruction.user.expect("user is not set"),

            user_stats: self.instruction.user_stats.expect("user_stats is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            out_spot_market_vault: self
                .instruction
                .out_spot_market_vault
                .expect("out_spot_market_vault is not set"),

            in_spot_market_vault: self
                .instruction
                .in_spot_market_vault
                .expect("in_spot_market_vault is not set"),

            out_token_account: self
                .instruction
                .out_token_account
                .expect("out_token_account is not set"),

            in_token_account: self
                .instruction
                .in_token_account
                .expect("in_token_account is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            drift_signer: self
                .instruction
                .drift_signer
                .expect("drift_signer is not set"),

            instructions: self
                .instruction
                .instructions
                .expect("instructions is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct EndSwapCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_stats: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    out_spot_market_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    in_spot_market_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    out_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    in_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    drift_signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    instructions: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    in_market_index: Option<u16>,
    out_market_index: Option<u16>,
    limit_price: Option<u64>,
    reduce_only: Option<SwapReduceOnly>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
