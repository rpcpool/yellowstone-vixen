//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct UpdateBaseFeeParameters {
    pub lb_pair: solana_pubkey::Pubkey,

    pub admin: solana_pubkey::Pubkey,

    pub event_authority: solana_pubkey::Pubkey,

    pub program: solana_pubkey::Pubkey,
}

impl UpdateBaseFeeParameters {
    pub fn instruction(
        &self,
        args: UpdateBaseFeeParametersInstructionArgs,
    ) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: UpdateBaseFeeParametersInstructionArgs,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(self.lb_pair, false));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.admin, true,
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
        let mut data = borsh::to_vec(&UpdateBaseFeeParametersInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateBaseFeeParametersInstructionData {
    discriminator: [u8; 8],
}

impl UpdateBaseFeeParametersInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [75, 168, 223, 161, 16, 195, 3, 47],
        }
    }
}

impl Default for UpdateBaseFeeParametersInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateBaseFeeParametersInstructionArgs {
    pub protocol_share: u16,
    pub base_factor: u16,
    pub base_fee_power_factor: u8,
}

/// Instruction builder for `UpdateBaseFeeParameters`.
///
/// ### Accounts:
///
///   0. `[writable]` lb_pair
///   1. `[signer]` admin
///   2. `[]` event_authority
///   3. `[]` program
#[derive(Clone, Debug, Default)]
pub struct UpdateBaseFeeParametersBuilder {
    lb_pair: Option<solana_pubkey::Pubkey>,
    admin: Option<solana_pubkey::Pubkey>,
    event_authority: Option<solana_pubkey::Pubkey>,
    program: Option<solana_pubkey::Pubkey>,
    protocol_share: Option<u16>,
    base_factor: Option<u16>,
    base_fee_power_factor: Option<u8>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl UpdateBaseFeeParametersBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn lb_pair(&mut self, lb_pair: solana_pubkey::Pubkey) -> &mut Self {
        self.lb_pair = Some(lb_pair);
        self
    }

    #[inline(always)]
    pub fn admin(&mut self, admin: solana_pubkey::Pubkey) -> &mut Self {
        self.admin = Some(admin);
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
    pub fn protocol_share(&mut self, protocol_share: u16) -> &mut Self {
        self.protocol_share = Some(protocol_share);
        self
    }

    #[inline(always)]
    pub fn base_factor(&mut self, base_factor: u16) -> &mut Self {
        self.base_factor = Some(base_factor);
        self
    }

    #[inline(always)]
    pub fn base_fee_power_factor(&mut self, base_fee_power_factor: u8) -> &mut Self {
        self.base_fee_power_factor = Some(base_fee_power_factor);
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
        let accounts = UpdateBaseFeeParameters {
            lb_pair: self.lb_pair.expect("lb_pair is not set"),
            admin: self.admin.expect("admin is not set"),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = UpdateBaseFeeParametersInstructionArgs {
            protocol_share: self
                .protocol_share
                .clone()
                .expect("protocol_share is not set"),
            base_factor: self.base_factor.clone().expect("base_factor is not set"),
            base_fee_power_factor: self
                .base_fee_power_factor
                .clone()
                .expect("base_fee_power_factor is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `update_base_fee_parameters` CPI accounts.
pub struct UpdateBaseFeeParametersCpiAccounts<'a, 'b> {
    pub lb_pair: &'b solana_account_info::AccountInfo<'a>,

    pub admin: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
}

/// `update_base_fee_parameters` CPI instruction.
pub struct UpdateBaseFeeParametersCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_account_info::AccountInfo<'a>,

    pub lb_pair: &'b solana_account_info::AccountInfo<'a>,

    pub admin: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: UpdateBaseFeeParametersInstructionArgs,
}

impl<'a, 'b> UpdateBaseFeeParametersCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: UpdateBaseFeeParametersCpiAccounts<'a, 'b>,
        args: UpdateBaseFeeParametersInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            lb_pair: accounts.lb_pair,
            admin: accounts.admin,
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
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(
            *self.lb_pair.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.admin.key,
            true,
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
        let mut data = borsh::to_vec(&UpdateBaseFeeParametersInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.lb_pair.clone());
        account_infos.push(self.admin.clone());
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

/// Instruction builder for `UpdateBaseFeeParameters` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` lb_pair
///   1. `[signer]` admin
///   2. `[]` event_authority
///   3. `[]` program
#[derive(Clone, Debug)]
pub struct UpdateBaseFeeParametersCpiBuilder<'a, 'b> {
    instruction: Box<UpdateBaseFeeParametersCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdateBaseFeeParametersCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UpdateBaseFeeParametersCpiBuilderInstruction {
            __program: program,
            lb_pair: None,
            admin: None,
            event_authority: None,
            program: None,
            protocol_share: None,
            base_factor: None,
            base_fee_power_factor: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn lb_pair(&mut self, lb_pair: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.lb_pair = Some(lb_pair);
        self
    }

    #[inline(always)]
    pub fn admin(&mut self, admin: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.admin = Some(admin);
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
    pub fn protocol_share(&mut self, protocol_share: u16) -> &mut Self {
        self.instruction.protocol_share = Some(protocol_share);
        self
    }

    #[inline(always)]
    pub fn base_factor(&mut self, base_factor: u16) -> &mut Self {
        self.instruction.base_factor = Some(base_factor);
        self
    }

    #[inline(always)]
    pub fn base_fee_power_factor(&mut self, base_fee_power_factor: u8) -> &mut Self {
        self.instruction.base_fee_power_factor = Some(base_fee_power_factor);
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
        let args = UpdateBaseFeeParametersInstructionArgs {
            protocol_share: self
                .instruction
                .protocol_share
                .clone()
                .expect("protocol_share is not set"),
            base_factor: self
                .instruction
                .base_factor
                .clone()
                .expect("base_factor is not set"),
            base_fee_power_factor: self
                .instruction
                .base_fee_power_factor
                .clone()
                .expect("base_fee_power_factor is not set"),
        };
        let instruction = UpdateBaseFeeParametersCpi {
            __program: self.instruction.__program,

            lb_pair: self.instruction.lb_pair.expect("lb_pair is not set"),

            admin: self.instruction.admin.expect("admin is not set"),

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
struct UpdateBaseFeeParametersCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    lb_pair: Option<&'b solana_account_info::AccountInfo<'a>>,
    admin: Option<&'b solana_account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_account_info::AccountInfo<'a>>,
    program: Option<&'b solana_account_info::AccountInfo<'a>>,
    protocol_share: Option<u16>,
    base_factor: Option<u16>,
    base_fee_power_factor: Option<u8>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
