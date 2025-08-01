//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

use crate::generated::types::PoolFees;

/// Accounts.
#[derive(Debug)]
pub struct SetPoolFees {
    /// Pool account (PDA)
    pub pool: solana_pubkey::Pubkey,
    /// Fee operator account
    pub fee_operator: solana_pubkey::Pubkey,
}

impl SetPoolFees {
    pub fn instruction(&self, args: SetPoolFeesInstructionArgs) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: SetPoolFeesInstructionArgs,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(self.pool, false));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.fee_operator,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&SetPoolFeesInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_instruction::Instruction {
            program_id: crate::AMM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetPoolFeesInstructionData {
    discriminator: [u8; 8],
}

impl SetPoolFeesInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [102, 44, 158, 54, 205, 37, 126, 78],
        }
    }
}

impl Default for SetPoolFeesInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetPoolFeesInstructionArgs {
    pub fees: PoolFees,
    pub new_partner_fee_numerator: u64,
}

/// Instruction builder for `SetPoolFees`.
///
/// ### Accounts:
///
///   0. `[writable]` pool
///   1. `[signer]` fee_operator
#[derive(Clone, Debug, Default)]
pub struct SetPoolFeesBuilder {
    pool: Option<solana_pubkey::Pubkey>,
    fee_operator: Option<solana_pubkey::Pubkey>,
    fees: Option<PoolFees>,
    new_partner_fee_numerator: Option<u64>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl SetPoolFeesBuilder {
    pub fn new() -> Self { Self::default() }

    /// Pool account (PDA)
    #[inline(always)]
    pub fn pool(&mut self, pool: solana_pubkey::Pubkey) -> &mut Self {
        self.pool = Some(pool);
        self
    }

    /// Fee operator account
    #[inline(always)]
    pub fn fee_operator(&mut self, fee_operator: solana_pubkey::Pubkey) -> &mut Self {
        self.fee_operator = Some(fee_operator);
        self
    }

    #[inline(always)]
    pub fn fees(&mut self, fees: PoolFees) -> &mut Self {
        self.fees = Some(fees);
        self
    }

    #[inline(always)]
    pub fn new_partner_fee_numerator(&mut self, new_partner_fee_numerator: u64) -> &mut Self {
        self.new_partner_fee_numerator = Some(new_partner_fee_numerator);
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
        let accounts = SetPoolFees {
            pool: self.pool.expect("pool is not set"),
            fee_operator: self.fee_operator.expect("fee_operator is not set"),
        };
        let args = SetPoolFeesInstructionArgs {
            fees: self.fees.clone().expect("fees is not set"),
            new_partner_fee_numerator: self
                .new_partner_fee_numerator
                .clone()
                .expect("new_partner_fee_numerator is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `set_pool_fees` CPI accounts.
pub struct SetPoolFeesCpiAccounts<'a, 'b> {
    /// Pool account (PDA)
    pub pool: &'b solana_account_info::AccountInfo<'a>,
    /// Fee operator account
    pub fee_operator: &'b solana_account_info::AccountInfo<'a>,
}

/// `set_pool_fees` CPI instruction.
pub struct SetPoolFeesCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_account_info::AccountInfo<'a>,
    /// Pool account (PDA)
    pub pool: &'b solana_account_info::AccountInfo<'a>,
    /// Fee operator account
    pub fee_operator: &'b solana_account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: SetPoolFeesInstructionArgs,
}

impl<'a, 'b> SetPoolFeesCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: SetPoolFeesCpiAccounts<'a, 'b>,
        args: SetPoolFeesInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            pool: accounts.pool,
            fee_operator: accounts.fee_operator,
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
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(*self.pool.key, false));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.fee_operator.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&SetPoolFeesInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_instruction::Instruction {
            program_id: crate::AMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.pool.clone());
        account_infos.push(self.fee_operator.clone());
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

/// Instruction builder for `SetPoolFees` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` pool
///   1. `[signer]` fee_operator
#[derive(Clone, Debug)]
pub struct SetPoolFeesCpiBuilder<'a, 'b> {
    instruction: Box<SetPoolFeesCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SetPoolFeesCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SetPoolFeesCpiBuilderInstruction {
            __program: program,
            pool: None,
            fee_operator: None,
            fees: None,
            new_partner_fee_numerator: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    /// Pool account (PDA)
    #[inline(always)]
    pub fn pool(&mut self, pool: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.pool = Some(pool);
        self
    }

    /// Fee operator account
    #[inline(always)]
    pub fn fee_operator(
        &mut self,
        fee_operator: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.fee_operator = Some(fee_operator);
        self
    }

    #[inline(always)]
    pub fn fees(&mut self, fees: PoolFees) -> &mut Self {
        self.instruction.fees = Some(fees);
        self
    }

    #[inline(always)]
    pub fn new_partner_fee_numerator(&mut self, new_partner_fee_numerator: u64) -> &mut Self {
        self.instruction.new_partner_fee_numerator = Some(new_partner_fee_numerator);
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
        let args = SetPoolFeesInstructionArgs {
            fees: self.instruction.fees.clone().expect("fees is not set"),
            new_partner_fee_numerator: self
                .instruction
                .new_partner_fee_numerator
                .clone()
                .expect("new_partner_fee_numerator is not set"),
        };
        let instruction = SetPoolFeesCpi {
            __program: self.instruction.__program,

            pool: self.instruction.pool.expect("pool is not set"),

            fee_operator: self
                .instruction
                .fee_operator
                .expect("fee_operator is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct SetPoolFeesCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    pool: Option<&'b solana_account_info::AccountInfo<'a>>,
    fee_operator: Option<&'b solana_account_info::AccountInfo<'a>>,
    fees: Option<PoolFees>,
    new_partner_fee_numerator: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
