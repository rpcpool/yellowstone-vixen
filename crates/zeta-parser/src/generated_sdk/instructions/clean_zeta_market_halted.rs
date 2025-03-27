//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

use crate::generated::types::Asset;

/// Accounts.
#[derive(Debug)]
pub struct CleanZetaMarketHalted {
    pub state: solana_program::pubkey::Pubkey,

    pub market: solana_program::pubkey::Pubkey,

    pub bids: solana_program::pubkey::Pubkey,

    pub asks: solana_program::pubkey::Pubkey,
}

impl CleanZetaMarketHalted {
    pub fn instruction(
        &self,
        args: CleanZetaMarketHaltedInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CleanZetaMarketHaltedInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.state, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.bids, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.asks, false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&CleanZetaMarketHaltedInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::ZETA_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CleanZetaMarketHaltedInstructionData {
    discriminator: [u8; 8],
}

impl CleanZetaMarketHaltedInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [137, 140, 94, 18, 231, 232, 217, 204],
        }
    }
}

impl Default for CleanZetaMarketHaltedInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CleanZetaMarketHaltedInstructionArgs {
    pub asset: Asset,
}

/// Instruction builder for `CleanZetaMarketHalted`.
///
/// ### Accounts:
///
///   0. `[writable]` state
///   1. `[]` market
///   2. `[]` bids
///   3. `[]` asks
#[derive(Clone, Debug, Default)]
pub struct CleanZetaMarketHaltedBuilder {
    state: Option<solana_program::pubkey::Pubkey>,
    market: Option<solana_program::pubkey::Pubkey>,
    bids: Option<solana_program::pubkey::Pubkey>,
    asks: Option<solana_program::pubkey::Pubkey>,
    asset: Option<Asset>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CleanZetaMarketHaltedBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn state(&mut self, state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn market(&mut self, market: solana_program::pubkey::Pubkey) -> &mut Self {
        self.market = Some(market);
        self
    }

    #[inline(always)]
    pub fn bids(&mut self, bids: solana_program::pubkey::Pubkey) -> &mut Self {
        self.bids = Some(bids);
        self
    }

    #[inline(always)]
    pub fn asks(&mut self, asks: solana_program::pubkey::Pubkey) -> &mut Self {
        self.asks = Some(asks);
        self
    }

    #[inline(always)]
    pub fn asset(&mut self, asset: Asset) -> &mut Self {
        self.asset = Some(asset);
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
        let accounts = CleanZetaMarketHalted {
            state: self.state.expect("state is not set"),
            market: self.market.expect("market is not set"),
            bids: self.bids.expect("bids is not set"),
            asks: self.asks.expect("asks is not set"),
        };
        let args = CleanZetaMarketHaltedInstructionArgs {
            asset: self.asset.clone().expect("asset is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `clean_zeta_market_halted` CPI accounts.
pub struct CleanZetaMarketHaltedCpiAccounts<'a, 'b> {
    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub market: &'b solana_program::account_info::AccountInfo<'a>,

    pub bids: &'b solana_program::account_info::AccountInfo<'a>,

    pub asks: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `clean_zeta_market_halted` CPI instruction.
pub struct CleanZetaMarketHaltedCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub market: &'b solana_program::account_info::AccountInfo<'a>,

    pub bids: &'b solana_program::account_info::AccountInfo<'a>,

    pub asks: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CleanZetaMarketHaltedInstructionArgs,
}

impl<'a, 'b> CleanZetaMarketHaltedCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CleanZetaMarketHaltedCpiAccounts<'a, 'b>,
        args: CleanZetaMarketHaltedInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            state: accounts.state,
            market: accounts.market,
            bids: accounts.bids,
            asks: accounts.asks,
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
            *self.state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.market.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.bids.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.asks.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&CleanZetaMarketHaltedInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::ZETA_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.state.clone());
        account_infos.push(self.market.clone());
        account_infos.push(self.bids.clone());
        account_infos.push(self.asks.clone());
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

/// Instruction builder for `CleanZetaMarketHalted` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` state
///   1. `[]` market
///   2. `[]` bids
///   3. `[]` asks
#[derive(Clone, Debug)]
pub struct CleanZetaMarketHaltedCpiBuilder<'a, 'b> {
    instruction: Box<CleanZetaMarketHaltedCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CleanZetaMarketHaltedCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CleanZetaMarketHaltedCpiBuilderInstruction {
            __program: program,
            state: None,
            market: None,
            bids: None,
            asks: None,
            asset: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn state(&mut self, state: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn market(
        &mut self,
        market: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.market = Some(market);
        self
    }

    #[inline(always)]
    pub fn bids(&mut self, bids: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.bids = Some(bids);
        self
    }

    #[inline(always)]
    pub fn asks(&mut self, asks: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.asks = Some(asks);
        self
    }

    #[inline(always)]
    pub fn asset(&mut self, asset: Asset) -> &mut Self {
        self.instruction.asset = Some(asset);
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
        let args = CleanZetaMarketHaltedInstructionArgs {
            asset: self.instruction.asset.clone().expect("asset is not set"),
        };
        let instruction = CleanZetaMarketHaltedCpi {
            __program: self.instruction.__program,

            state: self.instruction.state.expect("state is not set"),

            market: self.instruction.market.expect("market is not set"),

            bids: self.instruction.bids.expect("bids is not set"),

            asks: self.instruction.asks.expect("asks is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CleanZetaMarketHaltedCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bids: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    asks: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    asset: Option<Asset>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
