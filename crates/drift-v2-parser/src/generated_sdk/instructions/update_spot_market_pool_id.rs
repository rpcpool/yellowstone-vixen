//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct UpdateSpotMarketPoolId {
    pub admin: solana_program::pubkey::Pubkey,

    pub state: solana_program::pubkey::Pubkey,

    pub spot_market: solana_program::pubkey::Pubkey,
}

impl UpdateSpotMarketPoolId {
    pub fn instruction(
        &self,
        args: UpdateSpotMarketPoolIdInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: UpdateSpotMarketPoolIdInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
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
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&UpdateSpotMarketPoolIdInstructionData::new()).unwrap();
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
pub struct UpdateSpotMarketPoolIdInstructionData {
    discriminator: [u8; 8],
}

impl UpdateSpotMarketPoolIdInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [22, 213, 197, 160, 139, 193, 81, 149],
        }
    }
}

impl Default for UpdateSpotMarketPoolIdInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketPoolIdInstructionArgs {
    pub pool_id: u8,
}

/// Instruction builder for `UpdateSpotMarketPoolId`.
///
/// ### Accounts:
///
///   0. `[signer]` admin
///   1. `[]` state
///   2. `[writable]` spot_market
#[derive(Clone, Debug, Default)]
pub struct UpdateSpotMarketPoolIdBuilder {
    admin: Option<solana_program::pubkey::Pubkey>,
    state: Option<solana_program::pubkey::Pubkey>,
    spot_market: Option<solana_program::pubkey::Pubkey>,
    pool_id: Option<u8>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UpdateSpotMarketPoolIdBuilder {
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
    pub fn pool_id(&mut self, pool_id: u8) -> &mut Self {
        self.pool_id = Some(pool_id);
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
        let accounts = UpdateSpotMarketPoolId {
            admin: self.admin.expect("admin is not set"),
            state: self.state.expect("state is not set"),
            spot_market: self.spot_market.expect("spot_market is not set"),
        };
        let args = UpdateSpotMarketPoolIdInstructionArgs {
            pool_id: self.pool_id.clone().expect("pool_id is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `update_spot_market_pool_id` CPI accounts.
pub struct UpdateSpotMarketPoolIdCpiAccounts<'a, 'b> {
    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub spot_market: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `update_spot_market_pool_id` CPI instruction.
pub struct UpdateSpotMarketPoolIdCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub spot_market: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: UpdateSpotMarketPoolIdInstructionArgs,
}

impl<'a, 'b> UpdateSpotMarketPoolIdCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UpdateSpotMarketPoolIdCpiAccounts<'a, 'b>,
        args: UpdateSpotMarketPoolIdInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            admin: accounts.admin,
            state: accounts.state,
            spot_market: accounts.spot_market,
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
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
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
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&UpdateSpotMarketPoolIdInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::DRIFT_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(4 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.admin.clone());
        account_infos.push(self.state.clone());
        account_infos.push(self.spot_market.clone());
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

/// Instruction builder for `UpdateSpotMarketPoolId` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` admin
///   1. `[]` state
///   2. `[writable]` spot_market
#[derive(Clone, Debug)]
pub struct UpdateSpotMarketPoolIdCpiBuilder<'a, 'b> {
    instruction: Box<UpdateSpotMarketPoolIdCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdateSpotMarketPoolIdCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UpdateSpotMarketPoolIdCpiBuilderInstruction {
            __program: program,
            admin: None,
            state: None,
            spot_market: None,
            pool_id: None,
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
    pub fn pool_id(&mut self, pool_id: u8) -> &mut Self {
        self.instruction.pool_id = Some(pool_id);
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
        let args = UpdateSpotMarketPoolIdInstructionArgs {
            pool_id: self
                .instruction
                .pool_id
                .clone()
                .expect("pool_id is not set"),
        };
        let instruction = UpdateSpotMarketPoolIdCpi {
            __program: self.instruction.__program,

            admin: self.instruction.admin.expect("admin is not set"),

            state: self.instruction.state.expect("state is not set"),

            spot_market: self
                .instruction
                .spot_market
                .expect("spot_market is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct UpdateSpotMarketPoolIdCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    spot_market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool_id: Option<u8>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
