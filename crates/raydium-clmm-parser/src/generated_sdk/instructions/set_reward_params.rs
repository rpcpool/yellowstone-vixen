//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct SetRewardParams {
    pub authority: solana_pubkey::Pubkey,

    pub amm_config: solana_pubkey::Pubkey,

    pub pool_state: solana_pubkey::Pubkey,

    pub operation_state: solana_pubkey::Pubkey,

    pub token_program: solana_pubkey::Pubkey,

    pub token_program2022: solana_pubkey::Pubkey,
}

impl SetRewardParams {
    pub fn instruction(
        &self,
        args: SetRewardParamsInstructionArgs,
    ) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: SetRewardParamsInstructionArgs,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.amm_config,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(self.pool_state, false));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.operation_state,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_program2022,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&SetRewardParamsInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_instruction::Instruction {
            program_id: crate::AMM_V3_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetRewardParamsInstructionData {
    discriminator: [u8; 8],
}

impl SetRewardParamsInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [112, 52, 167, 75, 32, 201, 211, 137],
        }
    }
}

impl Default for SetRewardParamsInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetRewardParamsInstructionArgs {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
    pub open_time: u64,
    pub end_time: u64,
}

/// Instruction builder for `SetRewardParams`.
///
/// ### Accounts:
///
///   0. `[signer]` authority
///   1. `[]` amm_config
///   2. `[writable]` pool_state
///   3. `[]` operation_state
///   4. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   5. `[]` token_program2022
#[derive(Clone, Debug, Default)]
pub struct SetRewardParamsBuilder {
    authority: Option<solana_pubkey::Pubkey>,
    amm_config: Option<solana_pubkey::Pubkey>,
    pool_state: Option<solana_pubkey::Pubkey>,
    operation_state: Option<solana_pubkey::Pubkey>,
    token_program: Option<solana_pubkey::Pubkey>,
    token_program2022: Option<solana_pubkey::Pubkey>,
    reward_index: Option<u8>,
    emissions_per_second_x64: Option<u128>,
    open_time: Option<u64>,
    end_time: Option<u64>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl SetRewardParamsBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn authority(&mut self, authority: solana_pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }

    #[inline(always)]
    pub fn amm_config(&mut self, amm_config: solana_pubkey::Pubkey) -> &mut Self {
        self.amm_config = Some(amm_config);
        self
    }

    #[inline(always)]
    pub fn pool_state(&mut self, pool_state: solana_pubkey::Pubkey) -> &mut Self {
        self.pool_state = Some(pool_state);
        self
    }

    #[inline(always)]
    pub fn operation_state(&mut self, operation_state: solana_pubkey::Pubkey) -> &mut Self {
        self.operation_state = Some(operation_state);
        self
    }

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn token_program2022(&mut self, token_program2022: solana_pubkey::Pubkey) -> &mut Self {
        self.token_program2022 = Some(token_program2022);
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

    #[inline(always)]
    pub fn open_time(&mut self, open_time: u64) -> &mut Self {
        self.open_time = Some(open_time);
        self
    }

    #[inline(always)]
    pub fn end_time(&mut self, end_time: u64) -> &mut Self {
        self.end_time = Some(end_time);
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
        let accounts = SetRewardParams {
            authority: self.authority.expect("authority is not set"),
            amm_config: self.amm_config.expect("amm_config is not set"),
            pool_state: self.pool_state.expect("pool_state is not set"),
            operation_state: self.operation_state.expect("operation_state is not set"),
            token_program: self.token_program.unwrap_or(solana_pubkey::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            token_program2022: self
                .token_program2022
                .expect("token_program2022 is not set"),
        };
        let args = SetRewardParamsInstructionArgs {
            reward_index: self.reward_index.clone().expect("reward_index is not set"),
            emissions_per_second_x64: self
                .emissions_per_second_x64
                .clone()
                .expect("emissions_per_second_x64 is not set"),
            open_time: self.open_time.clone().expect("open_time is not set"),
            end_time: self.end_time.clone().expect("end_time is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `set_reward_params` CPI accounts.
pub struct SetRewardParamsCpiAccounts<'a, 'b> {
    pub authority: &'b solana_account_info::AccountInfo<'a>,

    pub amm_config: &'b solana_account_info::AccountInfo<'a>,

    pub pool_state: &'b solana_account_info::AccountInfo<'a>,

    pub operation_state: &'b solana_account_info::AccountInfo<'a>,

    pub token_program: &'b solana_account_info::AccountInfo<'a>,

    pub token_program2022: &'b solana_account_info::AccountInfo<'a>,
}

/// `set_reward_params` CPI instruction.
pub struct SetRewardParamsCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_account_info::AccountInfo<'a>,

    pub authority: &'b solana_account_info::AccountInfo<'a>,

    pub amm_config: &'b solana_account_info::AccountInfo<'a>,

    pub pool_state: &'b solana_account_info::AccountInfo<'a>,

    pub operation_state: &'b solana_account_info::AccountInfo<'a>,

    pub token_program: &'b solana_account_info::AccountInfo<'a>,

    pub token_program2022: &'b solana_account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: SetRewardParamsInstructionArgs,
}

impl<'a, 'b> SetRewardParamsCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: SetRewardParamsCpiAccounts<'a, 'b>,
        args: SetRewardParamsInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            authority: accounts.authority,
            amm_config: accounts.amm_config,
            pool_state: accounts.pool_state,
            operation_state: accounts.operation_state,
            token_program: accounts.token_program,
            token_program2022: accounts.token_program2022,
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
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.amm_config.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.pool_state.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.operation_state.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_program2022.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&SetRewardParamsInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_instruction::Instruction {
            program_id: crate::AMM_V3_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.amm_config.clone());
        account_infos.push(self.pool_state.clone());
        account_infos.push(self.operation_state.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.token_program2022.clone());
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

/// Instruction builder for `SetRewardParams` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` authority
///   1. `[]` amm_config
///   2. `[writable]` pool_state
///   3. `[]` operation_state
///   4. `[]` token_program
///   5. `[]` token_program2022
#[derive(Clone, Debug)]
pub struct SetRewardParamsCpiBuilder<'a, 'b> {
    instruction: Box<SetRewardParamsCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SetRewardParamsCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SetRewardParamsCpiBuilderInstruction {
            __program: program,
            authority: None,
            amm_config: None,
            pool_state: None,
            operation_state: None,
            token_program: None,
            token_program2022: None,
            reward_index: None,
            emissions_per_second_x64: None,
            open_time: None,
            end_time: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn authority(&mut self, authority: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }

    #[inline(always)]
    pub fn amm_config(
        &mut self,
        amm_config: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.amm_config = Some(amm_config);
        self
    }

    #[inline(always)]
    pub fn pool_state(
        &mut self,
        pool_state: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.pool_state = Some(pool_state);
        self
    }

    #[inline(always)]
    pub fn operation_state(
        &mut self,
        operation_state: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.operation_state = Some(operation_state);
        self
    }

    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn token_program2022(
        &mut self,
        token_program2022: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program2022 = Some(token_program2022);
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

    #[inline(always)]
    pub fn open_time(&mut self, open_time: u64) -> &mut Self {
        self.instruction.open_time = Some(open_time);
        self
    }

    #[inline(always)]
    pub fn end_time(&mut self, end_time: u64) -> &mut Self {
        self.instruction.end_time = Some(end_time);
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
        let args = SetRewardParamsInstructionArgs {
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
            open_time: self
                .instruction
                .open_time
                .clone()
                .expect("open_time is not set"),
            end_time: self
                .instruction
                .end_time
                .clone()
                .expect("end_time is not set"),
        };
        let instruction = SetRewardParamsCpi {
            __program: self.instruction.__program,

            authority: self.instruction.authority.expect("authority is not set"),

            amm_config: self.instruction.amm_config.expect("amm_config is not set"),

            pool_state: self.instruction.pool_state.expect("pool_state is not set"),

            operation_state: self
                .instruction
                .operation_state
                .expect("operation_state is not set"),

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
struct SetRewardParamsCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    authority: Option<&'b solana_account_info::AccountInfo<'a>>,
    amm_config: Option<&'b solana_account_info::AccountInfo<'a>>,
    pool_state: Option<&'b solana_account_info::AccountInfo<'a>>,
    operation_state: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_program2022: Option<&'b solana_account_info::AccountInfo<'a>>,
    reward_index: Option<u8>,
    emissions_per_second_x64: Option<u128>,
    open_time: Option<u64>,
    end_time: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
