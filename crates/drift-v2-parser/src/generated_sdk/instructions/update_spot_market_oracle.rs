//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::generated::types::OracleSource;

/// Accounts.
#[derive(Debug)]
pub struct UpdateSpotMarketOracle {
    pub admin: solana_program::pubkey::Pubkey,

    pub state: solana_program::pubkey::Pubkey,

    pub spot_market: solana_program::pubkey::Pubkey,

    pub oracle: solana_program::pubkey::Pubkey,

    pub old_oracle: solana_program::pubkey::Pubkey,
}

impl UpdateSpotMarketOracle {
    pub fn instruction(
        &self,
        args: UpdateSpotMarketOracleInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: UpdateSpotMarketOracleInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.admin, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.state, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.spot_market,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.old_oracle,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&UpdateSpotMarketOracleInstructionData::new()).unwrap();
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
pub struct UpdateSpotMarketOracleInstructionData {
    discriminator: [u8; 8],
}

impl UpdateSpotMarketOracleInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [114, 184, 102, 37, 246, 186, 180, 99],
        }
    }
}

impl Default for UpdateSpotMarketOracleInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketOracleInstructionArgs {
    pub oracle_arg: Pubkey,
    pub oracle_source: OracleSource,
    pub skip_invariant_check: bool,
}

/// Instruction builder for `UpdateSpotMarketOracle`.
///
/// ### Accounts:
///
///   0. `[signer]` admin
///   1. `[]` state
///   2. `[writable]` spot_market
///   3. `[]` oracle
///   4. `[]` old_oracle
#[derive(Clone, Debug, Default)]
pub struct UpdateSpotMarketOracleBuilder {
    admin: Option<solana_program::pubkey::Pubkey>,
    state: Option<solana_program::pubkey::Pubkey>,
    spot_market: Option<solana_program::pubkey::Pubkey>,
    oracle: Option<solana_program::pubkey::Pubkey>,
    old_oracle: Option<solana_program::pubkey::Pubkey>,
    oracle_arg: Option<Pubkey>,
    oracle_source: Option<OracleSource>,
    skip_invariant_check: Option<bool>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UpdateSpotMarketOracleBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn admin(&mut self, admin: solana_program::pubkey::Pubkey) -> &mut Self {
        self.admin = Some(admin);
        self
    }

    #[inline(always)]
    pub fn state(&mut self, state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn spot_market(&mut self, spot_market: solana_program::pubkey::Pubkey) -> &mut Self {
        self.spot_market = Some(spot_market);
        self
    }

    #[inline(always)]
    pub fn oracle(&mut self, oracle: solana_program::pubkey::Pubkey) -> &mut Self {
        self.oracle = Some(oracle);
        self
    }

    #[inline(always)]
    pub fn old_oracle(&mut self, old_oracle: solana_program::pubkey::Pubkey) -> &mut Self {
        self.old_oracle = Some(old_oracle);
        self
    }

    #[inline(always)]
    pub fn oracle_arg(&mut self, oracle_arg: Pubkey) -> &mut Self {
        self.oracle_arg = Some(oracle_arg);
        self
    }

    #[inline(always)]
    pub fn oracle_source(&mut self, oracle_source: OracleSource) -> &mut Self {
        self.oracle_source = Some(oracle_source);
        self
    }

    #[inline(always)]
    pub fn skip_invariant_check(&mut self, skip_invariant_check: bool) -> &mut Self {
        self.skip_invariant_check = Some(skip_invariant_check);
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
        let accounts = UpdateSpotMarketOracle {
            admin: self.admin.expect("admin is not set"),
            state: self.state.expect("state is not set"),
            spot_market: self.spot_market.expect("spot_market is not set"),
            oracle: self.oracle.expect("oracle is not set"),
            old_oracle: self.old_oracle.expect("old_oracle is not set"),
        };
        let args = UpdateSpotMarketOracleInstructionArgs {
            oracle_arg: self.oracle_arg.clone().expect("oracle_arg is not set"),
            oracle_source: self
                .oracle_source
                .clone()
                .expect("oracle_source is not set"),
            skip_invariant_check: self
                .skip_invariant_check
                .clone()
                .expect("skip_invariant_check is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `update_spot_market_oracle` CPI accounts.
pub struct UpdateSpotMarketOracleCpiAccounts<'a, 'b> {
    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub spot_market: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,

    pub old_oracle: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `update_spot_market_oracle` CPI instruction.
pub struct UpdateSpotMarketOracleCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub spot_market: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,

    pub old_oracle: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: UpdateSpotMarketOracleInstructionArgs,
}

impl<'a, 'b> UpdateSpotMarketOracleCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UpdateSpotMarketOracleCpiAccounts<'a, 'b>,
        args: UpdateSpotMarketOracleInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            admin: accounts.admin,
            state: accounts.state,
            spot_market: accounts.spot_market,
            oracle: accounts.oracle,
            old_oracle: accounts.old_oracle,
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
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.admin.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.spot_market.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.oracle.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.old_oracle.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&UpdateSpotMarketOracleInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::DRIFT_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.admin.clone());
        account_infos.push(self.state.clone());
        account_infos.push(self.spot_market.clone());
        account_infos.push(self.oracle.clone());
        account_infos.push(self.old_oracle.clone());
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

/// Instruction builder for `UpdateSpotMarketOracle` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` admin
///   1. `[]` state
///   2. `[writable]` spot_market
///   3. `[]` oracle
///   4. `[]` old_oracle
#[derive(Clone, Debug)]
pub struct UpdateSpotMarketOracleCpiBuilder<'a, 'b> {
    instruction: Box<UpdateSpotMarketOracleCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdateSpotMarketOracleCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UpdateSpotMarketOracleCpiBuilderInstruction {
            __program: program,
            admin: None,
            state: None,
            spot_market: None,
            oracle: None,
            old_oracle: None,
            oracle_arg: None,
            oracle_source: None,
            skip_invariant_check: None,
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
    pub fn state(&mut self, state: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn spot_market(
        &mut self,
        spot_market: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.spot_market = Some(spot_market);
        self
    }

    #[inline(always)]
    pub fn oracle(
        &mut self,
        oracle: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.oracle = Some(oracle);
        self
    }

    #[inline(always)]
    pub fn old_oracle(
        &mut self,
        old_oracle: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.old_oracle = Some(old_oracle);
        self
    }

    #[inline(always)]
    pub fn oracle_arg(&mut self, oracle_arg: Pubkey) -> &mut Self {
        self.instruction.oracle_arg = Some(oracle_arg);
        self
    }

    #[inline(always)]
    pub fn oracle_source(&mut self, oracle_source: OracleSource) -> &mut Self {
        self.instruction.oracle_source = Some(oracle_source);
        self
    }

    #[inline(always)]
    pub fn skip_invariant_check(&mut self, skip_invariant_check: bool) -> &mut Self {
        self.instruction.skip_invariant_check = Some(skip_invariant_check);
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
        let args = UpdateSpotMarketOracleInstructionArgs {
            oracle_arg: self
                .instruction
                .oracle_arg
                .clone()
                .expect("oracle_arg is not set"),
            oracle_source: self
                .instruction
                .oracle_source
                .clone()
                .expect("oracle_source is not set"),
            skip_invariant_check: self
                .instruction
                .skip_invariant_check
                .clone()
                .expect("skip_invariant_check is not set"),
        };
        let instruction = UpdateSpotMarketOracleCpi {
            __program: self.instruction.__program,

            admin: self.instruction.admin.expect("admin is not set"),

            state: self.instruction.state.expect("state is not set"),

            spot_market: self
                .instruction
                .spot_market
                .expect("spot_market is not set"),

            oracle: self.instruction.oracle.expect("oracle is not set"),

            old_oracle: self.instruction.old_oracle.expect("old_oracle is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct UpdateSpotMarketOracleCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    spot_market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    oracle: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    old_oracle: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    oracle_arg: Option<Pubkey>,
    oracle_source: Option<OracleSource>,
    skip_invariant_check: Option<bool>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
