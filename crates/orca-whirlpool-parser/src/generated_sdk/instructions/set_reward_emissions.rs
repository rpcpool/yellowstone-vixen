//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct SetRewardEmissions {
    pub whirlpool: solana_program::pubkey::Pubkey,

    pub reward_authority: solana_program::pubkey::Pubkey,

    pub reward_vault: solana_program::pubkey::Pubkey,
}

impl SetRewardEmissions {
    pub fn instruction(
        &self,
        args: SetRewardEmissionsInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: SetRewardEmissionsInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.whirlpool,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reward_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reward_vault,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&SetRewardEmissionsInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetRewardEmissionsInstructionData {
    discriminator: [u8; 8],
}

impl SetRewardEmissionsInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [13, 197, 86, 168, 109, 176, 27, 244],
        }
    }
}

impl Default for SetRewardEmissionsInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetRewardEmissionsInstructionArgs {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
}

/// Instruction builder for `SetRewardEmissions`.
///
/// ### Accounts:
///
///   0. `[writable]` whirlpool
///   1. `[signer]` reward_authority
///   2. `[]` reward_vault
#[derive(Clone, Debug, Default)]
pub struct SetRewardEmissionsBuilder {
    whirlpool: Option<solana_program::pubkey::Pubkey>,
    reward_authority: Option<solana_program::pubkey::Pubkey>,
    reward_vault: Option<solana_program::pubkey::Pubkey>,
    reward_index: Option<u8>,
    emissions_per_second_x64: Option<u128>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl SetRewardEmissionsBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn whirlpool(&mut self, whirlpool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.whirlpool = Some(whirlpool);
        self
    }

    #[inline(always)]
    pub fn reward_authority(
        &mut self,
        reward_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.reward_authority = Some(reward_authority);
        self
    }

    #[inline(always)]
    pub fn reward_vault(&mut self, reward_vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reward_vault = Some(reward_vault);
        self
    }

    #[inline(always)]
    pub fn reward_index(&mut self, reward_index: u8) -> &mut Self {
        self.reward_index = Some(reward_index);
        self
    }

    #[inline(always)]
    pub fn emissions_per_second_x64(&mut self, emissions_per_second_x64: u128) -> &mut Self {
        self.emissions_per_second_x64 = Some(emissions_per_second_x64);
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
        let accounts = SetRewardEmissions {
            whirlpool: self.whirlpool.expect("whirlpool is not set"),
            reward_authority: self.reward_authority.expect("reward_authority is not set"),
            reward_vault: self.reward_vault.expect("reward_vault is not set"),
        };
        let args = SetRewardEmissionsInstructionArgs {
            reward_index: self.reward_index.clone().expect("reward_index is not set"),
            emissions_per_second_x64: self
                .emissions_per_second_x64
                .clone()
                .expect("emissions_per_second_x64 is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `set_reward_emissions` CPI accounts.
pub struct SetRewardEmissionsCpiAccounts<'a, 'b> {
    pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_vault: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `set_reward_emissions` CPI instruction.
pub struct SetRewardEmissionsCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_vault: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: SetRewardEmissionsInstructionArgs,
}

impl<'a, 'b> SetRewardEmissionsCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: SetRewardEmissionsCpiAccounts<'a, 'b>,
        args: SetRewardEmissionsInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            whirlpool: accounts.whirlpool,
            reward_authority: accounts.reward_authority,
            reward_vault: accounts.reward_vault,
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
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.whirlpool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_vault.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&SetRewardEmissionsInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(4 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.whirlpool.clone());
        account_infos.push(self.reward_authority.clone());
        account_infos.push(self.reward_vault.clone());
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

/// Instruction builder for `SetRewardEmissions` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` whirlpool
///   1. `[signer]` reward_authority
///   2. `[]` reward_vault
#[derive(Clone, Debug)]
pub struct SetRewardEmissionsCpiBuilder<'a, 'b> {
    instruction: Box<SetRewardEmissionsCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SetRewardEmissionsCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SetRewardEmissionsCpiBuilderInstruction {
            __program: program,
            whirlpool: None,
            reward_authority: None,
            reward_vault: None,
            reward_index: None,
            emissions_per_second_x64: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn whirlpool(
        &mut self,
        whirlpool: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.whirlpool = Some(whirlpool);
        self
    }

    #[inline(always)]
    pub fn reward_authority(
        &mut self,
        reward_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_authority = Some(reward_authority);
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
    pub fn reward_index(&mut self, reward_index: u8) -> &mut Self {
        self.instruction.reward_index = Some(reward_index);
        self
    }

    #[inline(always)]
    pub fn emissions_per_second_x64(&mut self, emissions_per_second_x64: u128) -> &mut Self {
        self.instruction.emissions_per_second_x64 = Some(emissions_per_second_x64);
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
        let args = SetRewardEmissionsInstructionArgs {
            reward_index: self
                .instruction
                .reward_index
                .clone()
                .expect("reward_index is not set"),
            emissions_per_second_x64: self
                .instruction
                .emissions_per_second_x64
                .clone()
                .expect("emissions_per_second_x64 is not set"),
        };
        let instruction = SetRewardEmissionsCpi {
            __program: self.instruction.__program,

            whirlpool: self.instruction.whirlpool.expect("whirlpool is not set"),

            reward_authority: self
                .instruction
                .reward_authority
                .expect("reward_authority is not set"),

            reward_vault: self
                .instruction
                .reward_vault
                .expect("reward_vault is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct SetRewardEmissionsCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    whirlpool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_index: Option<u8>,
    emissions_per_second_x64: Option<u128>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
