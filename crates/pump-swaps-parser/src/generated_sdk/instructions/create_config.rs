//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

/// Accounts.
#[derive(Debug)]
pub struct CreateConfig {
    pub admin: solana_pubkey::Pubkey,

    pub global_config: solana_pubkey::Pubkey,

    pub system_program: solana_pubkey::Pubkey,

    pub event_authority: solana_pubkey::Pubkey,

    pub program: solana_pubkey::Pubkey,
}

impl CreateConfig {
    pub fn instruction(
        &self,
        args: CreateConfigInstructionArgs,
    ) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateConfigInstructionArgs,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(self.admin, true));
        accounts.push(solana_instruction::AccountMeta::new(
            self.global_config,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
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
        let mut data = borsh::to_vec(&CreateConfigInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_instruction::Instruction {
            program_id: crate::PUMP_AMM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateConfigInstructionData {
    discriminator: [u8; 8],
}

impl CreateConfigInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [201, 207, 243, 114, 75, 111, 47, 189],
        }
    }
}

impl Default for CreateConfigInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateConfigInstructionArgs {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
    pub coin_creator_fee_basis_points: u64,
}

/// Instruction builder for `CreateConfig`.
///
/// ### Accounts:
///
///   0. `[writable, signer, optional]` admin (default to `8LWu7QM2dGR1G8nKDHthckea57bkCzXyBTAKPJUBDHo8`)
///   1. `[writable]` global_config
///   2. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   3. `[]` event_authority
///   4. `[]` program
#[derive(Clone, Debug, Default)]
pub struct CreateConfigBuilder {
    admin: Option<solana_pubkey::Pubkey>,
    global_config: Option<solana_pubkey::Pubkey>,
    system_program: Option<solana_pubkey::Pubkey>,
    event_authority: Option<solana_pubkey::Pubkey>,
    program: Option<solana_pubkey::Pubkey>,
    lp_fee_basis_points: Option<u64>,
    protocol_fee_basis_points: Option<u64>,
    protocol_fee_recipients: Option<[Pubkey; 8]>,
    coin_creator_fee_basis_points: Option<u64>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl CreateConfigBuilder {
    pub fn new() -> Self { Self::default() }

    /// `[optional account, default to '8LWu7QM2dGR1G8nKDHthckea57bkCzXyBTAKPJUBDHo8']`
    #[inline(always)]
    pub fn admin(&mut self, admin: solana_pubkey::Pubkey) -> &mut Self {
        self.admin = Some(admin);
        self
    }

    #[inline(always)]
    pub fn global_config(&mut self, global_config: solana_pubkey::Pubkey) -> &mut Self {
        self.global_config = Some(global_config);
        self
    }

    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
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
    pub fn lp_fee_basis_points(&mut self, lp_fee_basis_points: u64) -> &mut Self {
        self.lp_fee_basis_points = Some(lp_fee_basis_points);
        self
    }

    #[inline(always)]
    pub fn protocol_fee_basis_points(&mut self, protocol_fee_basis_points: u64) -> &mut Self {
        self.protocol_fee_basis_points = Some(protocol_fee_basis_points);
        self
    }

    #[inline(always)]
    pub fn protocol_fee_recipients(&mut self, protocol_fee_recipients: [Pubkey; 8]) -> &mut Self {
        self.protocol_fee_recipients = Some(protocol_fee_recipients);
        self
    }

    #[inline(always)]
    pub fn coin_creator_fee_basis_points(
        &mut self,
        coin_creator_fee_basis_points: u64,
    ) -> &mut Self {
        self.coin_creator_fee_basis_points = Some(coin_creator_fee_basis_points);
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
        let accounts = CreateConfig {
            admin: self.admin.unwrap_or(solana_pubkey::pubkey!(
                "8LWu7QM2dGR1G8nKDHthckea57bkCzXyBTAKPJUBDHo8"
            )),
            global_config: self.global_config.expect("global_config is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_pubkey::pubkey!("11111111111111111111111111111111")),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = CreateConfigInstructionArgs {
            lp_fee_basis_points: self
                .lp_fee_basis_points
                .clone()
                .expect("lp_fee_basis_points is not set"),
            protocol_fee_basis_points: self
                .protocol_fee_basis_points
                .clone()
                .expect("protocol_fee_basis_points is not set"),
            protocol_fee_recipients: self
                .protocol_fee_recipients
                .clone()
                .expect("protocol_fee_recipients is not set"),
            coin_creator_fee_basis_points: self
                .coin_creator_fee_basis_points
                .clone()
                .expect("coin_creator_fee_basis_points is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_config` CPI accounts.
pub struct CreateConfigCpiAccounts<'a, 'b> {
    pub admin: &'b solana_account_info::AccountInfo<'a>,

    pub global_config: &'b solana_account_info::AccountInfo<'a>,

    pub system_program: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
}

/// `create_config` CPI instruction.
pub struct CreateConfigCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_account_info::AccountInfo<'a>,

    pub admin: &'b solana_account_info::AccountInfo<'a>,

    pub global_config: &'b solana_account_info::AccountInfo<'a>,

    pub system_program: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateConfigInstructionArgs,
}

impl<'a, 'b> CreateConfigCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: CreateConfigCpiAccounts<'a, 'b>,
        args: CreateConfigInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            admin: accounts.admin,
            global_config: accounts.global_config,
            system_program: accounts.system_program,
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
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(*self.admin.key, true));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.global_config.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
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
        let mut data = borsh::to_vec(&CreateConfigInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_instruction::Instruction {
            program_id: crate::PUMP_AMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.admin.clone());
        account_infos.push(self.global_config.clone());
        account_infos.push(self.system_program.clone());
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

/// Instruction builder for `CreateConfig` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` admin
///   1. `[writable]` global_config
///   2. `[]` system_program
///   3. `[]` event_authority
///   4. `[]` program
#[derive(Clone, Debug)]
pub struct CreateConfigCpiBuilder<'a, 'b> {
    instruction: Box<CreateConfigCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateConfigCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateConfigCpiBuilderInstruction {
            __program: program,
            admin: None,
            global_config: None,
            system_program: None,
            event_authority: None,
            program: None,
            lp_fee_basis_points: None,
            protocol_fee_basis_points: None,
            protocol_fee_recipients: None,
            coin_creator_fee_basis_points: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn admin(&mut self, admin: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.admin = Some(admin);
        self
    }

    #[inline(always)]
    pub fn global_config(
        &mut self,
        global_config: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.global_config = Some(global_config);
        self
    }

    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
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
    pub fn lp_fee_basis_points(&mut self, lp_fee_basis_points: u64) -> &mut Self {
        self.instruction.lp_fee_basis_points = Some(lp_fee_basis_points);
        self
    }

    #[inline(always)]
    pub fn protocol_fee_basis_points(&mut self, protocol_fee_basis_points: u64) -> &mut Self {
        self.instruction.protocol_fee_basis_points = Some(protocol_fee_basis_points);
        self
    }

    #[inline(always)]
    pub fn protocol_fee_recipients(&mut self, protocol_fee_recipients: [Pubkey; 8]) -> &mut Self {
        self.instruction.protocol_fee_recipients = Some(protocol_fee_recipients);
        self
    }

    #[inline(always)]
    pub fn coin_creator_fee_basis_points(
        &mut self,
        coin_creator_fee_basis_points: u64,
    ) -> &mut Self {
        self.instruction.coin_creator_fee_basis_points = Some(coin_creator_fee_basis_points);
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
        let args = CreateConfigInstructionArgs {
            lp_fee_basis_points: self
                .instruction
                .lp_fee_basis_points
                .clone()
                .expect("lp_fee_basis_points is not set"),
            protocol_fee_basis_points: self
                .instruction
                .protocol_fee_basis_points
                .clone()
                .expect("protocol_fee_basis_points is not set"),
            protocol_fee_recipients: self
                .instruction
                .protocol_fee_recipients
                .clone()
                .expect("protocol_fee_recipients is not set"),
            coin_creator_fee_basis_points: self
                .instruction
                .coin_creator_fee_basis_points
                .clone()
                .expect("coin_creator_fee_basis_points is not set"),
        };
        let instruction = CreateConfigCpi {
            __program: self.instruction.__program,

            admin: self.instruction.admin.expect("admin is not set"),

            global_config: self
                .instruction
                .global_config
                .expect("global_config is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

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
struct CreateConfigCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    admin: Option<&'b solana_account_info::AccountInfo<'a>>,
    global_config: Option<&'b solana_account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_account_info::AccountInfo<'a>>,
    program: Option<&'b solana_account_info::AccountInfo<'a>>,
    lp_fee_basis_points: Option<u64>,
    protocol_fee_basis_points: Option<u64>,
    protocol_fee_recipients: Option<[Pubkey; 8]>,
    coin_creator_fee_basis_points: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
