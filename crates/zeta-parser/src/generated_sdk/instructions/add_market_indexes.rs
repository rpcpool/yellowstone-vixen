//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct AddMarketIndexes {
    pub market_indexes: solana_program::pubkey::Pubkey,

    pub zeta_group: solana_program::pubkey::Pubkey,
}

impl AddMarketIndexes {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.market_indexes,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.zeta_group,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&AddMarketIndexesInstructionData::new()).unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::ZETA_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddMarketIndexesInstructionData {
    discriminator: [u8; 8],
}

impl AddMarketIndexesInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [94, 246, 144, 175, 4, 164, 233, 252],
        }
    }
}

impl Default for AddMarketIndexesInstructionData {
    fn default() -> Self { Self::new() }
}

/// Instruction builder for `AddMarketIndexes`.
///
/// ### Accounts:
///
///   0. `[writable]` market_indexes
///   1. `[writable]` zeta_group
#[derive(Clone, Debug, Default)]
pub struct AddMarketIndexesBuilder {
    market_indexes: Option<solana_program::pubkey::Pubkey>,
    zeta_group: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl AddMarketIndexesBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn market_indexes(&mut self, market_indexes: solana_program::pubkey::Pubkey) -> &mut Self {
        self.market_indexes = Some(market_indexes);
        self
    }

    #[inline(always)]
    pub fn zeta_group(&mut self, zeta_group: solana_program::pubkey::Pubkey) -> &mut Self {
        self.zeta_group = Some(zeta_group);
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
        let accounts = AddMarketIndexes {
            market_indexes: self.market_indexes.expect("market_indexes is not set"),
            zeta_group: self.zeta_group.expect("zeta_group is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `add_market_indexes` CPI accounts.
pub struct AddMarketIndexesCpiAccounts<'a, 'b> {
    pub market_indexes: &'b solana_program::account_info::AccountInfo<'a>,

    pub zeta_group: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `add_market_indexes` CPI instruction.
pub struct AddMarketIndexesCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub market_indexes: &'b solana_program::account_info::AccountInfo<'a>,

    pub zeta_group: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> AddMarketIndexesCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: AddMarketIndexesCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            market_indexes: accounts.market_indexes,
            zeta_group: accounts.zeta_group,
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
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.market_indexes.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.zeta_group.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = borsh::to_vec(&AddMarketIndexesInstructionData::new()).unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::ZETA_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.market_indexes.clone());
        account_infos.push(self.zeta_group.clone());
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

/// Instruction builder for `AddMarketIndexes` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` market_indexes
///   1. `[writable]` zeta_group
#[derive(Clone, Debug)]
pub struct AddMarketIndexesCpiBuilder<'a, 'b> {
    instruction: Box<AddMarketIndexesCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> AddMarketIndexesCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(AddMarketIndexesCpiBuilderInstruction {
            __program: program,
            market_indexes: None,
            zeta_group: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn market_indexes(
        &mut self,
        market_indexes: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.market_indexes = Some(market_indexes);
        self
    }

    #[inline(always)]
    pub fn zeta_group(
        &mut self,
        zeta_group: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.zeta_group = Some(zeta_group);
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
        let instruction = AddMarketIndexesCpi {
            __program: self.instruction.__program,

            market_indexes: self
                .instruction
                .market_indexes
                .expect("market_indexes is not set"),

            zeta_group: self.instruction.zeta_group.expect("zeta_group is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct AddMarketIndexesCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    market_indexes: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    zeta_group: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
