//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

use crate::generated::types::PrelaunchOracleParams;

/// Accounts.
#[derive(Debug)]
pub struct UpdatePrelaunchOracleParams {
    pub admin: solana_program::pubkey::Pubkey,

    pub prelaunch_oracle: solana_program::pubkey::Pubkey,

    pub perp_market: solana_program::pubkey::Pubkey,

    pub state: solana_program::pubkey::Pubkey,
}

impl UpdatePrelaunchOracleParams {
    pub fn instruction(
        &self,
        args: UpdatePrelaunchOracleParamsInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: UpdatePrelaunchOracleParamsInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.admin, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.prelaunch_oracle,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.perp_market,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.state, false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&UpdatePrelaunchOracleParamsInstructionData::new()).unwrap();
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
pub struct UpdatePrelaunchOracleParamsInstructionData {
    discriminator: [u8; 8],
}

impl UpdatePrelaunchOracleParamsInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [98, 205, 147, 243, 18, 75, 83, 207],
        }
    }
}

impl Default for UpdatePrelaunchOracleParamsInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePrelaunchOracleParamsInstructionArgs {
    pub params: PrelaunchOracleParams,
}

/// Instruction builder for `UpdatePrelaunchOracleParams`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` admin
///   1. `[writable]` prelaunch_oracle
///   2. `[writable]` perp_market
///   3. `[]` state
#[derive(Clone, Debug, Default)]
pub struct UpdatePrelaunchOracleParamsBuilder {
    admin: Option<solana_program::pubkey::Pubkey>,
    prelaunch_oracle: Option<solana_program::pubkey::Pubkey>,
    perp_market: Option<solana_program::pubkey::Pubkey>,
    state: Option<solana_program::pubkey::Pubkey>,
    params: Option<PrelaunchOracleParams>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UpdatePrelaunchOracleParamsBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn admin(&mut self, admin: solana_program::pubkey::Pubkey) -> &mut Self {
        self.admin = Some(admin);
        self
    }

    #[inline(always)]
    pub fn prelaunch_oracle(
        &mut self,
        prelaunch_oracle: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.prelaunch_oracle = Some(prelaunch_oracle);
        self
    }

    #[inline(always)]
    pub fn perp_market(&mut self, perp_market: solana_program::pubkey::Pubkey) -> &mut Self {
        self.perp_market = Some(perp_market);
        self
    }

    #[inline(always)]
    pub fn state(&mut self, state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn params(&mut self, params: PrelaunchOracleParams) -> &mut Self {
        self.params = Some(params);
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
        let accounts = UpdatePrelaunchOracleParams {
            admin: self.admin.expect("admin is not set"),
            prelaunch_oracle: self.prelaunch_oracle.expect("prelaunch_oracle is not set"),
            perp_market: self.perp_market.expect("perp_market is not set"),
            state: self.state.expect("state is not set"),
        };
        let args = UpdatePrelaunchOracleParamsInstructionArgs {
            params: self.params.clone().expect("params is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `update_prelaunch_oracle_params` CPI accounts.
pub struct UpdatePrelaunchOracleParamsCpiAccounts<'a, 'b> {
    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub prelaunch_oracle: &'b solana_program::account_info::AccountInfo<'a>,

    pub perp_market: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `update_prelaunch_oracle_params` CPI instruction.
pub struct UpdatePrelaunchOracleParamsCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub prelaunch_oracle: &'b solana_program::account_info::AccountInfo<'a>,

    pub perp_market: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: UpdatePrelaunchOracleParamsInstructionArgs,
}

impl<'a, 'b> UpdatePrelaunchOracleParamsCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UpdatePrelaunchOracleParamsCpiAccounts<'a, 'b>,
        args: UpdatePrelaunchOracleParamsInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            admin: accounts.admin,
            prelaunch_oracle: accounts.prelaunch_oracle,
            perp_market: accounts.perp_market,
            state: accounts.state,
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
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.admin.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.prelaunch_oracle.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.perp_market.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.state.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&UpdatePrelaunchOracleParamsInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::DRIFT_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.admin.clone());
        account_infos.push(self.prelaunch_oracle.clone());
        account_infos.push(self.perp_market.clone());
        account_infos.push(self.state.clone());
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

/// Instruction builder for `UpdatePrelaunchOracleParams` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` admin
///   1. `[writable]` prelaunch_oracle
///   2. `[writable]` perp_market
///   3. `[]` state
#[derive(Clone, Debug)]
pub struct UpdatePrelaunchOracleParamsCpiBuilder<'a, 'b> {
    instruction: Box<UpdatePrelaunchOracleParamsCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdatePrelaunchOracleParamsCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UpdatePrelaunchOracleParamsCpiBuilderInstruction {
            __program: program,
            admin: None,
            prelaunch_oracle: None,
            perp_market: None,
            state: None,
            params: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn admin(&mut self, admin: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.admin = Some(admin);
        self
    }

    #[inline(always)]
    pub fn prelaunch_oracle(
        &mut self,
        prelaunch_oracle: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.prelaunch_oracle = Some(prelaunch_oracle);
        self
    }

    #[inline(always)]
    pub fn perp_market(
        &mut self,
        perp_market: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.perp_market = Some(perp_market);
        self
    }

    #[inline(always)]
    pub fn state(&mut self, state: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn params(&mut self, params: PrelaunchOracleParams) -> &mut Self {
        self.instruction.params = Some(params);
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
        let args = UpdatePrelaunchOracleParamsInstructionArgs {
            params: self.instruction.params.clone().expect("params is not set"),
        };
        let instruction = UpdatePrelaunchOracleParamsCpi {
            __program: self.instruction.__program,

            admin: self.instruction.admin.expect("admin is not set"),

            prelaunch_oracle: self
                .instruction
                .prelaunch_oracle
                .expect("prelaunch_oracle is not set"),

            perp_market: self
                .instruction
                .perp_market
                .expect("perp_market is not set"),

            state: self.instruction.state.expect("state is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct UpdatePrelaunchOracleParamsCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    prelaunch_oracle: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    perp_market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    params: Option<PrelaunchOracleParams>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
