//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

use crate::generated::types::RemainingAccountsInfo;

/// Accounts.
#[derive(Debug)]
pub struct FundReward {
    pub lb_pair: solana_program::pubkey::Pubkey,

    pub reward_vault: solana_program::pubkey::Pubkey,

    pub reward_mint: solana_program::pubkey::Pubkey,

    pub funder_token_account: solana_program::pubkey::Pubkey,

    pub funder: solana_program::pubkey::Pubkey,

    pub bin_array: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub event_authority: solana_program::pubkey::Pubkey,

    pub program: solana_program::pubkey::Pubkey,
}

impl FundReward {
    pub fn instruction(
        &self,
        args: FundRewardInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: FundRewardInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.lb_pair,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.reward_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reward_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.funder_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.funder,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.bin_array,
            false,
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
        let mut data = borsh::to_vec(&FundRewardInstructionData::new()).unwrap();
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
pub struct FundRewardInstructionData {
    discriminator: [u8; 8],
}

impl FundRewardInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [188, 50, 249, 165, 93, 151, 38, 63],
        }
    }
}

impl Default for FundRewardInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FundRewardInstructionArgs {
    pub reward_index: u64,
    pub amount: u64,
    pub carry_forward: bool,
    pub remaining_accounts_info: RemainingAccountsInfo,
}

/// Instruction builder for `FundReward`.
///
/// ### Accounts:
///
///   0. `[writable]` lb_pair
///   1. `[writable]` reward_vault
///   2. `[]` reward_mint
///   3. `[writable]` funder_token_account
///   4. `[signer]` funder
///   5. `[writable]` bin_array
///   6. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   7. `[]` event_authority
///   8. `[]` program
#[derive(Clone, Debug, Default)]
pub struct FundRewardBuilder {
    lb_pair: Option<solana_program::pubkey::Pubkey>,
    reward_vault: Option<solana_program::pubkey::Pubkey>,
    reward_mint: Option<solana_program::pubkey::Pubkey>,
    funder_token_account: Option<solana_program::pubkey::Pubkey>,
    funder: Option<solana_program::pubkey::Pubkey>,
    bin_array: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    event_authority: Option<solana_program::pubkey::Pubkey>,
    program: Option<solana_program::pubkey::Pubkey>,
    reward_index: Option<u64>,
    amount: Option<u64>,
    carry_forward: Option<bool>,
    remaining_accounts_info: Option<RemainingAccountsInfo>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl FundRewardBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn lb_pair(&mut self, lb_pair: solana_program::pubkey::Pubkey) -> &mut Self {
        self.lb_pair = Some(lb_pair);
        self
    }

    #[inline(always)]
    pub fn reward_vault(&mut self, reward_vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reward_vault = Some(reward_vault);
        self
    }

    #[inline(always)]
    pub fn reward_mint(&mut self, reward_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reward_mint = Some(reward_mint);
        self
    }

    #[inline(always)]
    pub fn funder_token_account(
        &mut self,
        funder_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.funder_token_account = Some(funder_token_account);
        self
    }

    #[inline(always)]
    pub fn funder(&mut self, funder: solana_program::pubkey::Pubkey) -> &mut Self {
        self.funder = Some(funder);
        self
    }

    #[inline(always)]
    pub fn bin_array(&mut self, bin_array: solana_program::pubkey::Pubkey) -> &mut Self {
        self.bin_array = Some(bin_array);
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
    pub fn reward_index(&mut self, reward_index: u64) -> &mut Self {
        self.reward_index = Some(reward_index);
        self
    }

    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.amount = Some(amount);
        self
    }

    #[inline(always)]
    pub fn carry_forward(&mut self, carry_forward: bool) -> &mut Self {
        self.carry_forward = Some(carry_forward);
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
        let accounts = FundReward {
            lb_pair: self.lb_pair.expect("lb_pair is not set"),
            reward_vault: self.reward_vault.expect("reward_vault is not set"),
            reward_mint: self.reward_mint.expect("reward_mint is not set"),
            funder_token_account: self
                .funder_token_account
                .expect("funder_token_account is not set"),
            funder: self.funder.expect("funder is not set"),
            bin_array: self.bin_array.expect("bin_array is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = FundRewardInstructionArgs {
            reward_index: self.reward_index.clone().expect("reward_index is not set"),
            amount: self.amount.clone().expect("amount is not set"),
            carry_forward: self
                .carry_forward
                .clone()
                .expect("carry_forward is not set"),
            remaining_accounts_info: self
                .remaining_accounts_info
                .clone()
                .expect("remaining_accounts_info is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `fund_reward` CPI accounts.
pub struct FundRewardCpiAccounts<'a, 'b> {
    pub lb_pair: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder: &'b solana_program::account_info::AccountInfo<'a>,

    pub bin_array: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `fund_reward` CPI instruction.
pub struct FundRewardCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub lb_pair: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder: &'b solana_program::account_info::AccountInfo<'a>,

    pub bin_array: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: FundRewardInstructionArgs,
}

impl<'a, 'b> FundRewardCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: FundRewardCpiAccounts<'a, 'b>,
        args: FundRewardInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            lb_pair: accounts.lb_pair,
            reward_vault: accounts.reward_vault,
            reward_mint: accounts.reward_mint,
            funder_token_account: accounts.funder_token_account,
            funder: accounts.funder,
            bin_array: accounts.bin_array,
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
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lb_pair.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reward_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.funder_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.funder.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.bin_array.key,
            false,
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
        let mut data = borsh::to_vec(&FundRewardInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(10 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.lb_pair.clone());
        account_infos.push(self.reward_vault.clone());
        account_infos.push(self.reward_mint.clone());
        account_infos.push(self.funder_token_account.clone());
        account_infos.push(self.funder.clone());
        account_infos.push(self.bin_array.clone());
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

/// Instruction builder for `FundReward` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` lb_pair
///   1. `[writable]` reward_vault
///   2. `[]` reward_mint
///   3. `[writable]` funder_token_account
///   4. `[signer]` funder
///   5. `[writable]` bin_array
///   6. `[]` token_program
///   7. `[]` event_authority
///   8. `[]` program
#[derive(Clone, Debug)]
pub struct FundRewardCpiBuilder<'a, 'b> {
    instruction: Box<FundRewardCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> FundRewardCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(FundRewardCpiBuilderInstruction {
            __program: program,
            lb_pair: None,
            reward_vault: None,
            reward_mint: None,
            funder_token_account: None,
            funder: None,
            bin_array: None,
            token_program: None,
            event_authority: None,
            program: None,
            reward_index: None,
            amount: None,
            carry_forward: None,
            remaining_accounts_info: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn lb_pair(
        &mut self,
        lb_pair: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lb_pair = Some(lb_pair);
        self
    }

    #[inline(always)]
    pub fn reward_vault(
        &mut self,
        reward_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_vault = Some(reward_vault);
        self
    }

    #[inline(always)]
    pub fn reward_mint(
        &mut self,
        reward_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_mint = Some(reward_mint);
        self
    }

    #[inline(always)]
    pub fn funder_token_account(
        &mut self,
        funder_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.funder_token_account = Some(funder_token_account);
        self
    }

    #[inline(always)]
    pub fn funder(
        &mut self,
        funder: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.funder = Some(funder);
        self
    }

    #[inline(always)]
    pub fn bin_array(
        &mut self,
        bin_array: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.bin_array = Some(bin_array);
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
    pub fn reward_index(&mut self, reward_index: u64) -> &mut Self {
        self.instruction.reward_index = Some(reward_index);
        self
    }

    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.instruction.amount = Some(amount);
        self
    }

    #[inline(always)]
    pub fn carry_forward(&mut self, carry_forward: bool) -> &mut Self {
        self.instruction.carry_forward = Some(carry_forward);
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
        let args = FundRewardInstructionArgs {
            reward_index: self
                .instruction
                .reward_index
                .clone()
                .expect("reward_index is not set"),
            amount: self.instruction.amount.clone().expect("amount is not set"),
            carry_forward: self
                .instruction
                .carry_forward
                .clone()
                .expect("carry_forward is not set"),
            remaining_accounts_info: self
                .instruction
                .remaining_accounts_info
                .clone()
                .expect("remaining_accounts_info is not set"),
        };
        let instruction = FundRewardCpi {
            __program: self.instruction.__program,

            lb_pair: self.instruction.lb_pair.expect("lb_pair is not set"),

            reward_vault: self
                .instruction
                .reward_vault
                .expect("reward_vault is not set"),

            reward_mint: self
                .instruction
                .reward_mint
                .expect("reward_mint is not set"),

            funder_token_account: self
                .instruction
                .funder_token_account
                .expect("funder_token_account is not set"),

            funder: self.instruction.funder.expect("funder is not set"),

            bin_array: self.instruction.bin_array.expect("bin_array is not set"),

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
struct FundRewardCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    lb_pair: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    funder_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    funder: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bin_array: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_index: Option<u64>,
    amount: Option<u64>,
    carry_forward: Option<bool>,
    remaining_accounts_info: Option<RemainingAccountsInfo>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
