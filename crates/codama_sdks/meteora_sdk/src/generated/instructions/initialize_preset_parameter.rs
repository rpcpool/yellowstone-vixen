//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct InitializePresetParameter {
    pub preset_parameter: solana_program::pubkey::Pubkey,

    pub admin: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,
}

impl InitializePresetParameter {
    pub fn instruction(
        &self,
        args: InitializePresetParameterInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializePresetParameterInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.preset_parameter,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.admin, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&InitializePresetParameterInstructionData::new()).unwrap();
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
pub struct InitializePresetParameterInstructionData {
    discriminator: [u8; 8],
}

impl InitializePresetParameterInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [66, 188, 71, 211, 98, 109, 14, 186],
        }
    }
}

impl Default for InitializePresetParameterInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePresetParameterInstructionArgs {
    pub bin_step: u16,
    pub base_factor: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub variable_fee_control: u32,
    pub max_volatility_accumulator: u32,
    pub min_bin_id: i32,
    pub max_bin_id: i32,
    pub protocol_share: u16,
}

/// Instruction builder for `InitializePresetParameter`.
///
/// ### Accounts:
///
///   0. `[writable]` preset_parameter
///   1. `[writable, signer]` admin
///   2. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   3. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitializePresetParameterBuilder {
    preset_parameter: Option<solana_program::pubkey::Pubkey>,
    admin: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    bin_step: Option<u16>,
    base_factor: Option<u16>,
    filter_period: Option<u16>,
    decay_period: Option<u16>,
    reduction_factor: Option<u16>,
    variable_fee_control: Option<u32>,
    max_volatility_accumulator: Option<u32>,
    min_bin_id: Option<i32>,
    max_bin_id: Option<i32>,
    protocol_share: Option<u16>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializePresetParameterBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn preset_parameter(
        &mut self,
        preset_parameter: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.preset_parameter = Some(preset_parameter);
        self
    }

    #[inline(always)]
    pub fn admin(&mut self, admin: solana_program::pubkey::Pubkey) -> &mut Self {
        self.admin = Some(admin);
        self
    }

    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }

    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
        self
    }

    #[inline(always)]
    pub fn bin_step(&mut self, bin_step: u16) -> &mut Self {
        self.bin_step = Some(bin_step);
        self
    }

    #[inline(always)]
    pub fn base_factor(&mut self, base_factor: u16) -> &mut Self {
        self.base_factor = Some(base_factor);
        self
    }

    #[inline(always)]
    pub fn filter_period(&mut self, filter_period: u16) -> &mut Self {
        self.filter_period = Some(filter_period);
        self
    }

    #[inline(always)]
    pub fn decay_period(&mut self, decay_period: u16) -> &mut Self {
        self.decay_period = Some(decay_period);
        self
    }

    #[inline(always)]
    pub fn reduction_factor(&mut self, reduction_factor: u16) -> &mut Self {
        self.reduction_factor = Some(reduction_factor);
        self
    }

    #[inline(always)]
    pub fn variable_fee_control(&mut self, variable_fee_control: u32) -> &mut Self {
        self.variable_fee_control = Some(variable_fee_control);
        self
    }

    #[inline(always)]
    pub fn max_volatility_accumulator(&mut self, max_volatility_accumulator: u32) -> &mut Self {
        self.max_volatility_accumulator = Some(max_volatility_accumulator);
        self
    }

    #[inline(always)]
    pub fn min_bin_id(&mut self, min_bin_id: i32) -> &mut Self {
        self.min_bin_id = Some(min_bin_id);
        self
    }

    #[inline(always)]
    pub fn max_bin_id(&mut self, max_bin_id: i32) -> &mut Self {
        self.max_bin_id = Some(max_bin_id);
        self
    }

    #[inline(always)]
    pub fn protocol_share(&mut self, protocol_share: u16) -> &mut Self {
        self.protocol_share = Some(protocol_share);
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
        let accounts = InitializePresetParameter {
            preset_parameter: self.preset_parameter.expect("preset_parameter is not set"),
            admin: self.admin.expect("admin is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            rent: self.rent.unwrap_or(solana_program::pubkey!(
                "SysvarRent111111111111111111111111111111111"
            )),
        };
        let args = InitializePresetParameterInstructionArgs {
            bin_step: self.bin_step.clone().expect("bin_step is not set"),
            base_factor: self.base_factor.clone().expect("base_factor is not set"),
            filter_period: self
                .filter_period
                .clone()
                .expect("filter_period is not set"),
            decay_period: self.decay_period.clone().expect("decay_period is not set"),
            reduction_factor: self
                .reduction_factor
                .clone()
                .expect("reduction_factor is not set"),
            variable_fee_control: self
                .variable_fee_control
                .clone()
                .expect("variable_fee_control is not set"),
            max_volatility_accumulator: self
                .max_volatility_accumulator
                .clone()
                .expect("max_volatility_accumulator is not set"),
            min_bin_id: self.min_bin_id.clone().expect("min_bin_id is not set"),
            max_bin_id: self.max_bin_id.clone().expect("max_bin_id is not set"),
            protocol_share: self
                .protocol_share
                .clone()
                .expect("protocol_share is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize_preset_parameter` CPI accounts.
pub struct InitializePresetParameterCpiAccounts<'a, 'b> {
    pub preset_parameter: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_preset_parameter` CPI instruction.
pub struct InitializePresetParameterCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub preset_parameter: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitializePresetParameterInstructionArgs,
}

impl<'a, 'b> InitializePresetParameterCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializePresetParameterCpiAccounts<'a, 'b>,
        args: InitializePresetParameterInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            preset_parameter: accounts.preset_parameter,
            admin: accounts.admin,
            system_program: accounts.system_program,
            rent: accounts.rent,
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
            *self.preset_parameter.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.admin.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&InitializePresetParameterInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.preset_parameter.clone());
        account_infos.push(self.admin.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.rent.clone());
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

/// Instruction builder for `InitializePresetParameter` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` preset_parameter
///   1. `[writable, signer]` admin
///   2. `[]` system_program
///   3. `[]` rent
#[derive(Clone, Debug)]
pub struct InitializePresetParameterCpiBuilder<'a, 'b> {
    instruction: Box<InitializePresetParameterCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializePresetParameterCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializePresetParameterCpiBuilderInstruction {
            __program: program,
            preset_parameter: None,
            admin: None,
            system_program: None,
            rent: None,
            bin_step: None,
            base_factor: None,
            filter_period: None,
            decay_period: None,
            reduction_factor: None,
            variable_fee_control: None,
            max_volatility_accumulator: None,
            min_bin_id: None,
            max_bin_id: None,
            protocol_share: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn preset_parameter(
        &mut self,
        preset_parameter: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.preset_parameter = Some(preset_parameter);
        self
    }

    #[inline(always)]
    pub fn admin(&mut self, admin: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.admin = Some(admin);
        self
    }

    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }

    #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
        self
    }

    #[inline(always)]
    pub fn bin_step(&mut self, bin_step: u16) -> &mut Self {
        self.instruction.bin_step = Some(bin_step);
        self
    }

    #[inline(always)]
    pub fn base_factor(&mut self, base_factor: u16) -> &mut Self {
        self.instruction.base_factor = Some(base_factor);
        self
    }

    #[inline(always)]
    pub fn filter_period(&mut self, filter_period: u16) -> &mut Self {
        self.instruction.filter_period = Some(filter_period);
        self
    }

    #[inline(always)]
    pub fn decay_period(&mut self, decay_period: u16) -> &mut Self {
        self.instruction.decay_period = Some(decay_period);
        self
    }

    #[inline(always)]
    pub fn reduction_factor(&mut self, reduction_factor: u16) -> &mut Self {
        self.instruction.reduction_factor = Some(reduction_factor);
        self
    }

    #[inline(always)]
    pub fn variable_fee_control(&mut self, variable_fee_control: u32) -> &mut Self {
        self.instruction.variable_fee_control = Some(variable_fee_control);
        self
    }

    #[inline(always)]
    pub fn max_volatility_accumulator(&mut self, max_volatility_accumulator: u32) -> &mut Self {
        self.instruction.max_volatility_accumulator = Some(max_volatility_accumulator);
        self
    }

    #[inline(always)]
    pub fn min_bin_id(&mut self, min_bin_id: i32) -> &mut Self {
        self.instruction.min_bin_id = Some(min_bin_id);
        self
    }

    #[inline(always)]
    pub fn max_bin_id(&mut self, max_bin_id: i32) -> &mut Self {
        self.instruction.max_bin_id = Some(max_bin_id);
        self
    }

    #[inline(always)]
    pub fn protocol_share(&mut self, protocol_share: u16) -> &mut Self {
        self.instruction.protocol_share = Some(protocol_share);
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
        let args = InitializePresetParameterInstructionArgs {
            bin_step: self
                .instruction
                .bin_step
                .clone()
                .expect("bin_step is not set"),
            base_factor: self
                .instruction
                .base_factor
                .clone()
                .expect("base_factor is not set"),
            filter_period: self
                .instruction
                .filter_period
                .clone()
                .expect("filter_period is not set"),
            decay_period: self
                .instruction
                .decay_period
                .clone()
                .expect("decay_period is not set"),
            reduction_factor: self
                .instruction
                .reduction_factor
                .clone()
                .expect("reduction_factor is not set"),
            variable_fee_control: self
                .instruction
                .variable_fee_control
                .clone()
                .expect("variable_fee_control is not set"),
            max_volatility_accumulator: self
                .instruction
                .max_volatility_accumulator
                .clone()
                .expect("max_volatility_accumulator is not set"),
            min_bin_id: self
                .instruction
                .min_bin_id
                .clone()
                .expect("min_bin_id is not set"),
            max_bin_id: self
                .instruction
                .max_bin_id
                .clone()
                .expect("max_bin_id is not set"),
            protocol_share: self
                .instruction
                .protocol_share
                .clone()
                .expect("protocol_share is not set"),
        };
        let instruction = InitializePresetParameterCpi {
            __program: self.instruction.__program,

            preset_parameter: self
                .instruction
                .preset_parameter
                .expect("preset_parameter is not set"),

            admin: self.instruction.admin.expect("admin is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            rent: self.instruction.rent.expect("rent is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitializePresetParameterCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    preset_parameter: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bin_step: Option<u16>,
    base_factor: Option<u16>,
    filter_period: Option<u16>,
    decay_period: Option<u16>,
    reduction_factor: Option<u16>,
    variable_fee_control: Option<u32>,
    max_volatility_accumulator: Option<u32>,
    min_bin_id: Option<i32>,
    max_bin_id: Option<i32>,
    protocol_share: Option<u16>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
