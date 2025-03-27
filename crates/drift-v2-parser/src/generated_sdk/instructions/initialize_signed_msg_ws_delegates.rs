//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Accounts.
#[derive(Debug)]
pub struct InitializeSignedMsgWsDelegates {
    pub signed_msg_ws_delegates: solana_program::pubkey::Pubkey,

    pub authority: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
}

impl InitializeSignedMsgWsDelegates {
    pub fn instruction(
        &self,
        args: InitializeSignedMsgWsDelegatesInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializeSignedMsgWsDelegatesInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.signed_msg_ws_delegates,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data =
            borsh::to_vec(&InitializeSignedMsgWsDelegatesInstructionData::new()).unwrap();
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
pub struct InitializeSignedMsgWsDelegatesInstructionData {
    discriminator: [u8; 8],
}

impl InitializeSignedMsgWsDelegatesInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [40, 132, 96, 219, 184, 193, 80, 8],
        }
    }
}

impl Default for InitializeSignedMsgWsDelegatesInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeSignedMsgWsDelegatesInstructionArgs {
    pub delegates: Vec<Pubkey>,
}

/// Instruction builder for `InitializeSignedMsgWsDelegates`.
///
/// ### Accounts:
///
///   0. `[writable]` signed_msg_ws_delegates
///   1. `[writable, signer]` authority
///   2. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   3. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitializeSignedMsgWsDelegatesBuilder {
    signed_msg_ws_delegates: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    delegates: Option<Vec<Pubkey>>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeSignedMsgWsDelegatesBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn signed_msg_ws_delegates(
        &mut self,
        signed_msg_ws_delegates: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.signed_msg_ws_delegates = Some(signed_msg_ws_delegates);
        self
    }

    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }

    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
        self
    }

    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }

    #[inline(always)]
    pub fn delegates(&mut self, delegates: Vec<Pubkey>) -> &mut Self {
        self.delegates = Some(delegates);
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
        let accounts = InitializeSignedMsgWsDelegates {
            signed_msg_ws_delegates: self
                .signed_msg_ws_delegates
                .expect("signed_msg_ws_delegates is not set"),
            authority: self.authority.expect("authority is not set"),
            rent: self.rent.unwrap_or(solana_program::pubkey!(
                "SysvarRent111111111111111111111111111111111"
            )),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = InitializeSignedMsgWsDelegatesInstructionArgs {
            delegates: self.delegates.clone().expect("delegates is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize_signed_msg_ws_delegates` CPI accounts.
pub struct InitializeSignedMsgWsDelegatesCpiAccounts<'a, 'b> {
    pub signed_msg_ws_delegates: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_signed_msg_ws_delegates` CPI instruction.
pub struct InitializeSignedMsgWsDelegatesCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub signed_msg_ws_delegates: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitializeSignedMsgWsDelegatesInstructionArgs,
}

impl<'a, 'b> InitializeSignedMsgWsDelegatesCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeSignedMsgWsDelegatesCpiAccounts<'a, 'b>,
        args: InitializeSignedMsgWsDelegatesInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            signed_msg_ws_delegates: accounts.signed_msg_ws_delegates,
            authority: accounts.authority,
            rent: accounts.rent,
            system_program: accounts.system_program,
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
            *self.signed_msg_ws_delegates.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data =
            borsh::to_vec(&InitializeSignedMsgWsDelegatesInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::DRIFT_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.signed_msg_ws_delegates.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.system_program.clone());
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

/// Instruction builder for `InitializeSignedMsgWsDelegates` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` signed_msg_ws_delegates
///   1. `[writable, signer]` authority
///   2. `[]` rent
///   3. `[]` system_program
#[derive(Clone, Debug)]
pub struct InitializeSignedMsgWsDelegatesCpiBuilder<'a, 'b> {
    instruction: Box<InitializeSignedMsgWsDelegatesCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeSignedMsgWsDelegatesCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeSignedMsgWsDelegatesCpiBuilderInstruction {
            __program: program,
            signed_msg_ws_delegates: None,
            authority: None,
            rent: None,
            system_program: None,
            delegates: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn signed_msg_ws_delegates(
        &mut self,
        signed_msg_ws_delegates: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.signed_msg_ws_delegates = Some(signed_msg_ws_delegates);
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
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
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
    pub fn delegates(&mut self, delegates: Vec<Pubkey>) -> &mut Self {
        self.instruction.delegates = Some(delegates);
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
        let args = InitializeSignedMsgWsDelegatesInstructionArgs {
            delegates: self
                .instruction
                .delegates
                .clone()
                .expect("delegates is not set"),
        };
        let instruction = InitializeSignedMsgWsDelegatesCpi {
            __program: self.instruction.__program,

            signed_msg_ws_delegates: self
                .instruction
                .signed_msg_ws_delegates
                .expect("signed_msg_ws_delegates is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            rent: self.instruction.rent.expect("rent is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitializeSignedMsgWsDelegatesCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    signed_msg_ws_delegates: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    delegates: Option<Vec<Pubkey>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
