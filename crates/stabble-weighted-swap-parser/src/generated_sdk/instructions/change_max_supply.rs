//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct ChangeMaxSupply {
    pub owner: solana_program::pubkey::Pubkey,

    pub pool: solana_program::pubkey::Pubkey,
}

impl ChangeMaxSupply {
    pub fn instruction(
        &self,
        args: ChangeMaxSupplyInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: ChangeMaxSupplyInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.owner, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool, false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&ChangeMaxSupplyInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::WEIGHTED_SWAP_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeMaxSupplyInstructionData {
    discriminator: [u8; 8],
}

impl ChangeMaxSupplyInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [93, 176, 0, 205, 69, 63, 87, 80],
        }
    }
}

impl Default for ChangeMaxSupplyInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeMaxSupplyInstructionArgs {
    pub new_max_supply: u64,
}

/// Instruction builder for `ChangeMaxSupply`.
///
/// ### Accounts:
///
///   0. `[signer]` owner
///   1. `[writable]` pool
#[derive(Clone, Debug, Default)]
pub struct ChangeMaxSupplyBuilder {
    owner: Option<solana_program::pubkey::Pubkey>,
    pool: Option<solana_program::pubkey::Pubkey>,
    new_max_supply: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ChangeMaxSupplyBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn owner(&mut self, owner: solana_program::pubkey::Pubkey) -> &mut Self {
        self.owner = Some(owner);
        self
    }

    #[inline(always)]
    pub fn pool(&mut self, pool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.pool = Some(pool);
        self
    }

    #[inline(always)]
    pub fn new_max_supply(&mut self, new_max_supply: u64) -> &mut Self {
        self.new_max_supply = Some(new_max_supply);
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
        let accounts = ChangeMaxSupply {
            owner: self.owner.expect("owner is not set"),
            pool: self.pool.expect("pool is not set"),
        };
        let args = ChangeMaxSupplyInstructionArgs {
            new_max_supply: self
                .new_max_supply
                .clone()
                .expect("new_max_supply is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `change_max_supply` CPI accounts.
pub struct ChangeMaxSupplyCpiAccounts<'a, 'b> {
    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `change_max_supply` CPI instruction.
pub struct ChangeMaxSupplyCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: ChangeMaxSupplyInstructionArgs,
}

impl<'a, 'b> ChangeMaxSupplyCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ChangeMaxSupplyCpiAccounts<'a, 'b>,
        args: ChangeMaxSupplyInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            owner: accounts.owner,
            pool: accounts.pool,
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
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.owner.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&ChangeMaxSupplyInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::WEIGHTED_SWAP_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.pool.clone());
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

/// Instruction builder for `ChangeMaxSupply` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` owner
///   1. `[writable]` pool
#[derive(Clone, Debug)]
pub struct ChangeMaxSupplyCpiBuilder<'a, 'b> {
    instruction: Box<ChangeMaxSupplyCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ChangeMaxSupplyCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ChangeMaxSupplyCpiBuilderInstruction {
            __program: program,
            owner: None,
            pool: None,
            new_max_supply: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn owner(&mut self, owner: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.owner = Some(owner);
        self
    }

    #[inline(always)]
    pub fn pool(&mut self, pool: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.pool = Some(pool);
        self
    }

    #[inline(always)]
    pub fn new_max_supply(&mut self, new_max_supply: u64) -> &mut Self {
        self.instruction.new_max_supply = Some(new_max_supply);
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
        let args = ChangeMaxSupplyInstructionArgs {
            new_max_supply: self
                .instruction
                .new_max_supply
                .clone()
                .expect("new_max_supply is not set"),
        };
        let instruction = ChangeMaxSupplyCpi {
            __program: self.instruction.__program,

            owner: self.instruction.owner.expect("owner is not set"),

            pool: self.instruction.pool.expect("pool is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct ChangeMaxSupplyCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    new_max_supply: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
