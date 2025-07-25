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
pub struct InitializePositionByOperator {
    pub payer: solana_pubkey::Pubkey,

    pub base: solana_pubkey::Pubkey,

    pub position: solana_pubkey::Pubkey,

    pub lb_pair: solana_pubkey::Pubkey,

    pub owner: solana_pubkey::Pubkey,
    /// operator
    pub operator: solana_pubkey::Pubkey,

    pub operator_token_x: solana_pubkey::Pubkey,

    pub owner_token_x: solana_pubkey::Pubkey,

    pub system_program: solana_pubkey::Pubkey,

    pub event_authority: solana_pubkey::Pubkey,

    pub program: solana_pubkey::Pubkey,
}

impl InitializePositionByOperator {
    pub fn instruction(
        &self,
        args: InitializePositionByOperatorInstructionArgs,
    ) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializePositionByOperatorInstructionArgs,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(self.payer, true));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.base, true,
        ));
        accounts.push(solana_instruction::AccountMeta::new(self.position, false));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.lb_pair,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.owner, false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.operator,
            true,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.operator_token_x,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.owner_token_x,
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
        let mut data = borsh::to_vec(&InitializePositionByOperatorInstructionData::new()).unwrap();
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
pub struct InitializePositionByOperatorInstructionData {
    discriminator: [u8; 8],
}

impl InitializePositionByOperatorInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [251, 189, 190, 244, 117, 254, 35, 148],
        }
    }
}

impl Default for InitializePositionByOperatorInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePositionByOperatorInstructionArgs {
    pub lower_bin_id: i32,
    pub width: i32,
    pub fee_owner: Pubkey,
    pub lock_release_point: u64,
}

/// Instruction builder for `InitializePositionByOperator`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` payer
///   1. `[signer]` base
///   2. `[writable]` position
///   3. `[]` lb_pair
///   4. `[]` owner
///   5. `[signer]` operator
///   6. `[]` operator_token_x
///   7. `[]` owner_token_x
///   8. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   9. `[]` event_authority
///   10. `[]` program
#[derive(Clone, Debug, Default)]
pub struct InitializePositionByOperatorBuilder {
    payer: Option<solana_pubkey::Pubkey>,
    base: Option<solana_pubkey::Pubkey>,
    position: Option<solana_pubkey::Pubkey>,
    lb_pair: Option<solana_pubkey::Pubkey>,
    owner: Option<solana_pubkey::Pubkey>,
    operator: Option<solana_pubkey::Pubkey>,
    operator_token_x: Option<solana_pubkey::Pubkey>,
    owner_token_x: Option<solana_pubkey::Pubkey>,
    system_program: Option<solana_pubkey::Pubkey>,
    event_authority: Option<solana_pubkey::Pubkey>,
    program: Option<solana_pubkey::Pubkey>,
    lower_bin_id: Option<i32>,
    width: Option<i32>,
    fee_owner: Option<Pubkey>,
    lock_release_point: Option<u64>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl InitializePositionByOperatorBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn payer(&mut self, payer: solana_pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }

    #[inline(always)]
    pub fn base(&mut self, base: solana_pubkey::Pubkey) -> &mut Self {
        self.base = Some(base);
        self
    }

    #[inline(always)]
    pub fn position(&mut self, position: solana_pubkey::Pubkey) -> &mut Self {
        self.position = Some(position);
        self
    }

    #[inline(always)]
    pub fn lb_pair(&mut self, lb_pair: solana_pubkey::Pubkey) -> &mut Self {
        self.lb_pair = Some(lb_pair);
        self
    }

    #[inline(always)]
    pub fn owner(&mut self, owner: solana_pubkey::Pubkey) -> &mut Self {
        self.owner = Some(owner);
        self
    }

    /// operator
    #[inline(always)]
    pub fn operator(&mut self, operator: solana_pubkey::Pubkey) -> &mut Self {
        self.operator = Some(operator);
        self
    }

    #[inline(always)]
    pub fn operator_token_x(&mut self, operator_token_x: solana_pubkey::Pubkey) -> &mut Self {
        self.operator_token_x = Some(operator_token_x);
        self
    }

    #[inline(always)]
    pub fn owner_token_x(&mut self, owner_token_x: solana_pubkey::Pubkey) -> &mut Self {
        self.owner_token_x = Some(owner_token_x);
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
    pub fn lower_bin_id(&mut self, lower_bin_id: i32) -> &mut Self {
        self.lower_bin_id = Some(lower_bin_id);
        self
    }

    #[inline(always)]
    pub fn width(&mut self, width: i32) -> &mut Self {
        self.width = Some(width);
        self
    }

    #[inline(always)]
    pub fn fee_owner(&mut self, fee_owner: Pubkey) -> &mut Self {
        self.fee_owner = Some(fee_owner);
        self
    }

    #[inline(always)]
    pub fn lock_release_point(&mut self, lock_release_point: u64) -> &mut Self {
        self.lock_release_point = Some(lock_release_point);
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
        let accounts = InitializePositionByOperator {
            payer: self.payer.expect("payer is not set"),
            base: self.base.expect("base is not set"),
            position: self.position.expect("position is not set"),
            lb_pair: self.lb_pair.expect("lb_pair is not set"),
            owner: self.owner.expect("owner is not set"),
            operator: self.operator.expect("operator is not set"),
            operator_token_x: self.operator_token_x.expect("operator_token_x is not set"),
            owner_token_x: self.owner_token_x.expect("owner_token_x is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_pubkey::pubkey!("11111111111111111111111111111111")),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = InitializePositionByOperatorInstructionArgs {
            lower_bin_id: self.lower_bin_id.clone().expect("lower_bin_id is not set"),
            width: self.width.clone().expect("width is not set"),
            fee_owner: self.fee_owner.clone().expect("fee_owner is not set"),
            lock_release_point: self
                .lock_release_point
                .clone()
                .expect("lock_release_point is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize_position_by_operator` CPI accounts.
pub struct InitializePositionByOperatorCpiAccounts<'a, 'b> {
    pub payer: &'b solana_account_info::AccountInfo<'a>,

    pub base: &'b solana_account_info::AccountInfo<'a>,

    pub position: &'b solana_account_info::AccountInfo<'a>,

    pub lb_pair: &'b solana_account_info::AccountInfo<'a>,

    pub owner: &'b solana_account_info::AccountInfo<'a>,
    /// operator
    pub operator: &'b solana_account_info::AccountInfo<'a>,

    pub operator_token_x: &'b solana_account_info::AccountInfo<'a>,

    pub owner_token_x: &'b solana_account_info::AccountInfo<'a>,

    pub system_program: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
}

/// `initialize_position_by_operator` CPI instruction.
pub struct InitializePositionByOperatorCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_account_info::AccountInfo<'a>,

    pub payer: &'b solana_account_info::AccountInfo<'a>,

    pub base: &'b solana_account_info::AccountInfo<'a>,

    pub position: &'b solana_account_info::AccountInfo<'a>,

    pub lb_pair: &'b solana_account_info::AccountInfo<'a>,

    pub owner: &'b solana_account_info::AccountInfo<'a>,
    /// operator
    pub operator: &'b solana_account_info::AccountInfo<'a>,

    pub operator_token_x: &'b solana_account_info::AccountInfo<'a>,

    pub owner_token_x: &'b solana_account_info::AccountInfo<'a>,

    pub system_program: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitializePositionByOperatorInstructionArgs,
}

impl<'a, 'b> InitializePositionByOperatorCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: InitializePositionByOperatorCpiAccounts<'a, 'b>,
        args: InitializePositionByOperatorInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            payer: accounts.payer,
            base: accounts.base,
            position: accounts.position,
            lb_pair: accounts.lb_pair,
            owner: accounts.owner,
            operator: accounts.operator,
            operator_token_x: accounts.operator_token_x,
            owner_token_x: accounts.owner_token_x,
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
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(*self.payer.key, true));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.base.key,
            true,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.position.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.lb_pair.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.owner.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.operator.key,
            true,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.operator_token_x.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.owner_token_x.key,
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
        let mut data = borsh::to_vec(&InitializePositionByOperatorInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(12 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.base.clone());
        account_infos.push(self.position.clone());
        account_infos.push(self.lb_pair.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.operator.clone());
        account_infos.push(self.operator_token_x.clone());
        account_infos.push(self.owner_token_x.clone());
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

/// Instruction builder for `InitializePositionByOperator` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` payer
///   1. `[signer]` base
///   2. `[writable]` position
///   3. `[]` lb_pair
///   4. `[]` owner
///   5. `[signer]` operator
///   6. `[]` operator_token_x
///   7. `[]` owner_token_x
///   8. `[]` system_program
///   9. `[]` event_authority
///   10. `[]` program
#[derive(Clone, Debug)]
pub struct InitializePositionByOperatorCpiBuilder<'a, 'b> {
    instruction: Box<InitializePositionByOperatorCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializePositionByOperatorCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializePositionByOperatorCpiBuilderInstruction {
            __program: program,
            payer: None,
            base: None,
            position: None,
            lb_pair: None,
            owner: None,
            operator: None,
            operator_token_x: None,
            owner_token_x: None,
            system_program: None,
            event_authority: None,
            program: None,
            lower_bin_id: None,
            width: None,
            fee_owner: None,
            lock_release_point: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }

    #[inline(always)]
    pub fn base(&mut self, base: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.base = Some(base);
        self
    }

    #[inline(always)]
    pub fn position(&mut self, position: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.position = Some(position);
        self
    }

    #[inline(always)]
    pub fn lb_pair(&mut self, lb_pair: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.lb_pair = Some(lb_pair);
        self
    }

    #[inline(always)]
    pub fn owner(&mut self, owner: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.owner = Some(owner);
        self
    }

    /// operator
    #[inline(always)]
    pub fn operator(&mut self, operator: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.operator = Some(operator);
        self
    }

    #[inline(always)]
    pub fn operator_token_x(
        &mut self,
        operator_token_x: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.operator_token_x = Some(operator_token_x);
        self
    }

    #[inline(always)]
    pub fn owner_token_x(
        &mut self,
        owner_token_x: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.owner_token_x = Some(owner_token_x);
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
    pub fn lower_bin_id(&mut self, lower_bin_id: i32) -> &mut Self {
        self.instruction.lower_bin_id = Some(lower_bin_id);
        self
    }

    #[inline(always)]
    pub fn width(&mut self, width: i32) -> &mut Self {
        self.instruction.width = Some(width);
        self
    }

    #[inline(always)]
    pub fn fee_owner(&mut self, fee_owner: Pubkey) -> &mut Self {
        self.instruction.fee_owner = Some(fee_owner);
        self
    }

    #[inline(always)]
    pub fn lock_release_point(&mut self, lock_release_point: u64) -> &mut Self {
        self.instruction.lock_release_point = Some(lock_release_point);
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
        let args = InitializePositionByOperatorInstructionArgs {
            lower_bin_id: self
                .instruction
                .lower_bin_id
                .clone()
                .expect("lower_bin_id is not set"),
            width: self.instruction.width.clone().expect("width is not set"),
            fee_owner: self
                .instruction
                .fee_owner
                .clone()
                .expect("fee_owner is not set"),
            lock_release_point: self
                .instruction
                .lock_release_point
                .clone()
                .expect("lock_release_point is not set"),
        };
        let instruction = InitializePositionByOperatorCpi {
            __program: self.instruction.__program,

            payer: self.instruction.payer.expect("payer is not set"),

            base: self.instruction.base.expect("base is not set"),

            position: self.instruction.position.expect("position is not set"),

            lb_pair: self.instruction.lb_pair.expect("lb_pair is not set"),

            owner: self.instruction.owner.expect("owner is not set"),

            operator: self.instruction.operator.expect("operator is not set"),

            operator_token_x: self
                .instruction
                .operator_token_x
                .expect("operator_token_x is not set"),

            owner_token_x: self
                .instruction
                .owner_token_x
                .expect("owner_token_x is not set"),

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
struct InitializePositionByOperatorCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    payer: Option<&'b solana_account_info::AccountInfo<'a>>,
    base: Option<&'b solana_account_info::AccountInfo<'a>>,
    position: Option<&'b solana_account_info::AccountInfo<'a>>,
    lb_pair: Option<&'b solana_account_info::AccountInfo<'a>>,
    owner: Option<&'b solana_account_info::AccountInfo<'a>>,
    operator: Option<&'b solana_account_info::AccountInfo<'a>>,
    operator_token_x: Option<&'b solana_account_info::AccountInfo<'a>>,
    owner_token_x: Option<&'b solana_account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_account_info::AccountInfo<'a>>,
    program: Option<&'b solana_account_info::AccountInfo<'a>>,
    lower_bin_id: Option<i32>,
    width: Option<i32>,
    fee_owner: Option<Pubkey>,
    lock_release_point: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
