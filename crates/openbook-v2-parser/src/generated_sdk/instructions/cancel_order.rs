//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct CancelOrder {
    pub signer: solana_program::pubkey::Pubkey,

    pub open_orders_account: solana_program::pubkey::Pubkey,

    pub market: solana_program::pubkey::Pubkey,

    pub bids: solana_program::pubkey::Pubkey,

    pub asks: solana_program::pubkey::Pubkey,
}

impl CancelOrder {
    pub fn instruction(
        &self,
        args: CancelOrderInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CancelOrderInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.open_orders_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.bids, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.asks, false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&CancelOrderInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::OPENBOOK_V2_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelOrderInstructionData {
    discriminator: [u8; 8],
}

impl CancelOrderInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [95, 129, 237, 240, 8, 49, 223, 132],
        }
    }
}

impl Default for CancelOrderInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelOrderInstructionArgs {
    pub order_id: u128,
}

/// Instruction builder for `CancelOrder`.
///
/// ### Accounts:
///
///   0. `[signer]` signer
///   1. `[writable]` open_orders_account
///   2. `[]` market
///   3. `[writable]` bids
///   4. `[writable]` asks
#[derive(Clone, Debug, Default)]
pub struct CancelOrderBuilder {
    signer: Option<solana_program::pubkey::Pubkey>,
    open_orders_account: Option<solana_program::pubkey::Pubkey>,
    market: Option<solana_program::pubkey::Pubkey>,
    bids: Option<solana_program::pubkey::Pubkey>,
    asks: Option<solana_program::pubkey::Pubkey>,
    order_id: Option<u128>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CancelOrderBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn signer(&mut self, signer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.signer = Some(signer);
        self
    }

    #[inline(always)]
    pub fn open_orders_account(
        &mut self,
        open_orders_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.open_orders_account = Some(open_orders_account);
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
    pub fn order_id(&mut self, order_id: u128) -> &mut Self {
        self.order_id = Some(order_id);
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
        let accounts = CancelOrder {
            signer: self.signer.expect("signer is not set"),
            open_orders_account: self
                .open_orders_account
                .expect("open_orders_account is not set"),
            market: self.market.expect("market is not set"),
            bids: self.bids.expect("bids is not set"),
            asks: self.asks.expect("asks is not set"),
        };
        let args = CancelOrderInstructionArgs {
            order_id: self.order_id.clone().expect("order_id is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `cancel_order` CPI accounts.
pub struct CancelOrderCpiAccounts<'a, 'b> {
    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub open_orders_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub market: &'b solana_program::account_info::AccountInfo<'a>,

    pub bids: &'b solana_program::account_info::AccountInfo<'a>,

    pub asks: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `cancel_order` CPI instruction.
pub struct CancelOrderCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub open_orders_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub market: &'b solana_program::account_info::AccountInfo<'a>,

    pub bids: &'b solana_program::account_info::AccountInfo<'a>,

    pub asks: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CancelOrderInstructionArgs,
}

impl<'a, 'b> CancelOrderCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CancelOrderCpiAccounts<'a, 'b>,
        args: CancelOrderInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            signer: accounts.signer,
            open_orders_account: accounts.open_orders_account,
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
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.signer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.open_orders_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.market.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.bids.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
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
        let mut data = borsh::to_vec(&CancelOrderInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::OPENBOOK_V2_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.signer.clone());
        account_infos.push(self.open_orders_account.clone());
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

/// Instruction builder for `CancelOrder` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` signer
///   1. `[writable]` open_orders_account
///   2. `[]` market
///   3. `[writable]` bids
///   4. `[writable]` asks
#[derive(Clone, Debug)]
pub struct CancelOrderCpiBuilder<'a, 'b> {
    instruction: Box<CancelOrderCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CancelOrderCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CancelOrderCpiBuilderInstruction {
            __program: program,
            signer: None,
            open_orders_account: None,
            market: None,
            bids: None,
            asks: None,
            order_id: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn signer(
        &mut self,
        signer: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.signer = Some(signer);
        self
    }

    #[inline(always)]
    pub fn open_orders_account(
        &mut self,
        open_orders_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.open_orders_account = Some(open_orders_account);
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
    pub fn order_id(&mut self, order_id: u128) -> &mut Self {
        self.instruction.order_id = Some(order_id);
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
        let args = CancelOrderInstructionArgs {
            order_id: self
                .instruction
                .order_id
                .clone()
                .expect("order_id is not set"),
        };
        let instruction = CancelOrderCpi {
            __program: self.instruction.__program,

            signer: self.instruction.signer.expect("signer is not set"),

            open_orders_account: self
                .instruction
                .open_orders_account
                .expect("open_orders_account is not set"),

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
struct CancelOrderCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    open_orders_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bids: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    asks: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    order_id: Option<u128>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
