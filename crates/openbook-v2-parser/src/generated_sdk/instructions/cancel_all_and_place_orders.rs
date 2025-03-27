//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

use crate::generated::types::{PlaceMultipleOrdersArgs, PlaceOrderType};

/// Accounts.
#[derive(Debug)]
pub struct CancelAllAndPlaceOrders {
    pub signer: solana_program::pubkey::Pubkey,

    pub open_orders_account: solana_program::pubkey::Pubkey,

    pub open_orders_admin: Option<solana_program::pubkey::Pubkey>,

    pub user_quote_account: solana_program::pubkey::Pubkey,

    pub user_base_account: solana_program::pubkey::Pubkey,

    pub market: solana_program::pubkey::Pubkey,

    pub bids: solana_program::pubkey::Pubkey,

    pub asks: solana_program::pubkey::Pubkey,

    pub event_heap: solana_program::pubkey::Pubkey,

    pub market_quote_vault: solana_program::pubkey::Pubkey,

    pub market_base_vault: solana_program::pubkey::Pubkey,

    pub oracle_a: Option<solana_program::pubkey::Pubkey>,

    pub oracle_b: Option<solana_program::pubkey::Pubkey>,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl CancelAllAndPlaceOrders {
    pub fn instruction(
        &self,
        args: CancelAllAndPlaceOrdersInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CancelAllAndPlaceOrdersInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(14 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.open_orders_account,
            false,
        ));
        if let Some(open_orders_admin) = self.open_orders_admin {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                open_orders_admin,
                true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::OPENBOOK_V2_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_quote_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_base_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.market,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.bids, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.asks, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.event_heap,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.market_quote_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.market_base_vault,
            false,
        ));
        if let Some(oracle_a) = self.oracle_a {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                oracle_a, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::OPENBOOK_V2_ID,
                false,
            ));
        }
        if let Some(oracle_b) = self.oracle_b {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                oracle_b, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::OPENBOOK_V2_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&CancelAllAndPlaceOrdersInstructionData::new()).unwrap();
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
pub struct CancelAllAndPlaceOrdersInstructionData {
    discriminator: [u8; 8],
}

impl CancelAllAndPlaceOrdersInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [128, 155, 222, 60, 186, 40, 225, 50],
        }
    }
}

impl Default for CancelAllAndPlaceOrdersInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelAllAndPlaceOrdersInstructionArgs {
    pub orders_type: PlaceOrderType,
    pub bids_arg: Vec<PlaceMultipleOrdersArgs>,
    pub asks_arg: Vec<PlaceMultipleOrdersArgs>,
    pub limit: u8,
}

/// Instruction builder for `CancelAllAndPlaceOrders`.
///
/// ### Accounts:
///
///   0. `[signer]` signer
///   1. `[writable]` open_orders_account
///   2. `[signer, optional]` open_orders_admin
///   3. `[writable]` user_quote_account
///   4. `[writable]` user_base_account
///   5. `[writable]` market
///   6. `[writable]` bids
///   7. `[writable]` asks
///   8. `[writable]` event_heap
///   9. `[writable]` market_quote_vault
///   10. `[writable]` market_base_vault
///   11. `[optional]` oracle_a
///   12. `[optional]` oracle_b
///   13. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct CancelAllAndPlaceOrdersBuilder {
    signer: Option<solana_program::pubkey::Pubkey>,
    open_orders_account: Option<solana_program::pubkey::Pubkey>,
    open_orders_admin: Option<solana_program::pubkey::Pubkey>,
    user_quote_account: Option<solana_program::pubkey::Pubkey>,
    user_base_account: Option<solana_program::pubkey::Pubkey>,
    market: Option<solana_program::pubkey::Pubkey>,
    bids: Option<solana_program::pubkey::Pubkey>,
    asks: Option<solana_program::pubkey::Pubkey>,
    event_heap: Option<solana_program::pubkey::Pubkey>,
    market_quote_vault: Option<solana_program::pubkey::Pubkey>,
    market_base_vault: Option<solana_program::pubkey::Pubkey>,
    oracle_a: Option<solana_program::pubkey::Pubkey>,
    oracle_b: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    orders_type: Option<PlaceOrderType>,
    bids_arg: Option<Vec<PlaceMultipleOrdersArgs>>,
    asks_arg: Option<Vec<PlaceMultipleOrdersArgs>>,
    limit: Option<u8>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CancelAllAndPlaceOrdersBuilder {
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

    /// `[optional account]`
    #[inline(always)]
    pub fn open_orders_admin(
        &mut self,
        open_orders_admin: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.open_orders_admin = open_orders_admin;
        self
    }

    #[inline(always)]
    pub fn user_quote_account(
        &mut self,
        user_quote_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.user_quote_account = Some(user_quote_account);
        self
    }

    #[inline(always)]
    pub fn user_base_account(
        &mut self,
        user_base_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.user_base_account = Some(user_base_account);
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
    pub fn event_heap(&mut self, event_heap: solana_program::pubkey::Pubkey) -> &mut Self {
        self.event_heap = Some(event_heap);
        self
    }

    #[inline(always)]
    pub fn market_quote_vault(
        &mut self,
        market_quote_vault: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.market_quote_vault = Some(market_quote_vault);
        self
    }

    #[inline(always)]
    pub fn market_base_vault(
        &mut self,
        market_base_vault: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.market_base_vault = Some(market_base_vault);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn oracle_a(&mut self, oracle_a: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.oracle_a = oracle_a;
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn oracle_b(&mut self, oracle_b: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.oracle_b = oracle_b;
        self
    }

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn orders_type(&mut self, orders_type: PlaceOrderType) -> &mut Self {
        self.orders_type = Some(orders_type);
        self
    }

    #[inline(always)]
    pub fn bids_arg(&mut self, bids_arg: Vec<PlaceMultipleOrdersArgs>) -> &mut Self {
        self.bids_arg = Some(bids_arg);
        self
    }

    #[inline(always)]
    pub fn asks_arg(&mut self, asks_arg: Vec<PlaceMultipleOrdersArgs>) -> &mut Self {
        self.asks_arg = Some(asks_arg);
        self
    }

    #[inline(always)]
    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);
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
        let accounts = CancelAllAndPlaceOrders {
            signer: self.signer.expect("signer is not set"),
            open_orders_account: self
                .open_orders_account
                .expect("open_orders_account is not set"),
            open_orders_admin: self.open_orders_admin,
            user_quote_account: self
                .user_quote_account
                .expect("user_quote_account is not set"),
            user_base_account: self
                .user_base_account
                .expect("user_base_account is not set"),
            market: self.market.expect("market is not set"),
            bids: self.bids.expect("bids is not set"),
            asks: self.asks.expect("asks is not set"),
            event_heap: self.event_heap.expect("event_heap is not set"),
            market_quote_vault: self
                .market_quote_vault
                .expect("market_quote_vault is not set"),
            market_base_vault: self
                .market_base_vault
                .expect("market_base_vault is not set"),
            oracle_a: self.oracle_a,
            oracle_b: self.oracle_b,
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };
        let args = CancelAllAndPlaceOrdersInstructionArgs {
            orders_type: self.orders_type.clone().expect("orders_type is not set"),
            bids_arg: self.bids_arg.clone().expect("bids_arg is not set"),
            asks_arg: self.asks_arg.clone().expect("asks_arg is not set"),
            limit: self.limit.clone().expect("limit is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `cancel_all_and_place_orders` CPI accounts.
pub struct CancelAllAndPlaceOrdersCpiAccounts<'a, 'b> {
    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub open_orders_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub open_orders_admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub user_quote_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_base_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub market: &'b solana_program::account_info::AccountInfo<'a>,

    pub bids: &'b solana_program::account_info::AccountInfo<'a>,

    pub asks: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_heap: &'b solana_program::account_info::AccountInfo<'a>,

    pub market_quote_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub market_base_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub oracle_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `cancel_all_and_place_orders` CPI instruction.
pub struct CancelAllAndPlaceOrdersCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub open_orders_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub open_orders_admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub user_quote_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_base_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub market: &'b solana_program::account_info::AccountInfo<'a>,

    pub bids: &'b solana_program::account_info::AccountInfo<'a>,

    pub asks: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_heap: &'b solana_program::account_info::AccountInfo<'a>,

    pub market_quote_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub market_base_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub oracle_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CancelAllAndPlaceOrdersInstructionArgs,
}

impl<'a, 'b> CancelAllAndPlaceOrdersCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CancelAllAndPlaceOrdersCpiAccounts<'a, 'b>,
        args: CancelAllAndPlaceOrdersInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            signer: accounts.signer,
            open_orders_account: accounts.open_orders_account,
            open_orders_admin: accounts.open_orders_admin,
            user_quote_account: accounts.user_quote_account,
            user_base_account: accounts.user_base_account,
            market: accounts.market,
            bids: accounts.bids,
            asks: accounts.asks,
            event_heap: accounts.event_heap,
            market_quote_vault: accounts.market_quote_vault,
            market_base_vault: accounts.market_base_vault,
            oracle_a: accounts.oracle_a,
            oracle_b: accounts.oracle_b,
            token_program: accounts.token_program,
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
        let mut accounts = Vec::with_capacity(14 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.signer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.open_orders_account.key,
            false,
        ));
        if let Some(open_orders_admin) = self.open_orders_admin {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *open_orders_admin.key,
                true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::OPENBOOK_V2_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_quote_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_base_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
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
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.event_heap.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.market_quote_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.market_base_vault.key,
            false,
        ));
        if let Some(oracle_a) = self.oracle_a {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *oracle_a.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::OPENBOOK_V2_ID,
                false,
            ));
        }
        if let Some(oracle_b) = self.oracle_b {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *oracle_b.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::OPENBOOK_V2_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&CancelAllAndPlaceOrdersInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::OPENBOOK_V2_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(15 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.signer.clone());
        account_infos.push(self.open_orders_account.clone());
        if let Some(open_orders_admin) = self.open_orders_admin {
            account_infos.push(open_orders_admin.clone());
        }
        account_infos.push(self.user_quote_account.clone());
        account_infos.push(self.user_base_account.clone());
        account_infos.push(self.market.clone());
        account_infos.push(self.bids.clone());
        account_infos.push(self.asks.clone());
        account_infos.push(self.event_heap.clone());
        account_infos.push(self.market_quote_vault.clone());
        account_infos.push(self.market_base_vault.clone());
        if let Some(oracle_a) = self.oracle_a {
            account_infos.push(oracle_a.clone());
        }
        if let Some(oracle_b) = self.oracle_b {
            account_infos.push(oracle_b.clone());
        }
        account_infos.push(self.token_program.clone());
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

/// Instruction builder for `CancelAllAndPlaceOrders` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` signer
///   1. `[writable]` open_orders_account
///   2. `[signer, optional]` open_orders_admin
///   3. `[writable]` user_quote_account
///   4. `[writable]` user_base_account
///   5. `[writable]` market
///   6. `[writable]` bids
///   7. `[writable]` asks
///   8. `[writable]` event_heap
///   9. `[writable]` market_quote_vault
///   10. `[writable]` market_base_vault
///   11. `[optional]` oracle_a
///   12. `[optional]` oracle_b
///   13. `[]` token_program
#[derive(Clone, Debug)]
pub struct CancelAllAndPlaceOrdersCpiBuilder<'a, 'b> {
    instruction: Box<CancelAllAndPlaceOrdersCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CancelAllAndPlaceOrdersCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CancelAllAndPlaceOrdersCpiBuilderInstruction {
            __program: program,
            signer: None,
            open_orders_account: None,
            open_orders_admin: None,
            user_quote_account: None,
            user_base_account: None,
            market: None,
            bids: None,
            asks: None,
            event_heap: None,
            market_quote_vault: None,
            market_base_vault: None,
            oracle_a: None,
            oracle_b: None,
            token_program: None,
            orders_type: None,
            bids_arg: None,
            asks_arg: None,
            limit: None,
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

    /// `[optional account]`
    #[inline(always)]
    pub fn open_orders_admin(
        &mut self,
        open_orders_admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.open_orders_admin = open_orders_admin;
        self
    }

    #[inline(always)]
    pub fn user_quote_account(
        &mut self,
        user_quote_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_quote_account = Some(user_quote_account);
        self
    }

    #[inline(always)]
    pub fn user_base_account(
        &mut self,
        user_base_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_base_account = Some(user_base_account);
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
    pub fn event_heap(
        &mut self,
        event_heap: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.event_heap = Some(event_heap);
        self
    }

    #[inline(always)]
    pub fn market_quote_vault(
        &mut self,
        market_quote_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.market_quote_vault = Some(market_quote_vault);
        self
    }

    #[inline(always)]
    pub fn market_base_vault(
        &mut self,
        market_base_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.market_base_vault = Some(market_base_vault);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn oracle_a(
        &mut self,
        oracle_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.oracle_a = oracle_a;
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn oracle_b(
        &mut self,
        oracle_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.oracle_b = oracle_b;
        self
    }

    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn orders_type(&mut self, orders_type: PlaceOrderType) -> &mut Self {
        self.instruction.orders_type = Some(orders_type);
        self
    }

    #[inline(always)]
    pub fn bids_arg(&mut self, bids_arg: Vec<PlaceMultipleOrdersArgs>) -> &mut Self {
        self.instruction.bids_arg = Some(bids_arg);
        self
    }

    #[inline(always)]
    pub fn asks_arg(&mut self, asks_arg: Vec<PlaceMultipleOrdersArgs>) -> &mut Self {
        self.instruction.asks_arg = Some(asks_arg);
        self
    }

    #[inline(always)]
    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.instruction.limit = Some(limit);
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
        let args = CancelAllAndPlaceOrdersInstructionArgs {
            orders_type: self
                .instruction
                .orders_type
                .clone()
                .expect("orders_type is not set"),
            bids_arg: self
                .instruction
                .bids_arg
                .clone()
                .expect("bids_arg is not set"),
            asks_arg: self
                .instruction
                .asks_arg
                .clone()
                .expect("asks_arg is not set"),
            limit: self.instruction.limit.clone().expect("limit is not set"),
        };
        let instruction = CancelAllAndPlaceOrdersCpi {
            __program: self.instruction.__program,

            signer: self.instruction.signer.expect("signer is not set"),

            open_orders_account: self
                .instruction
                .open_orders_account
                .expect("open_orders_account is not set"),

            open_orders_admin: self.instruction.open_orders_admin,

            user_quote_account: self
                .instruction
                .user_quote_account
                .expect("user_quote_account is not set"),

            user_base_account: self
                .instruction
                .user_base_account
                .expect("user_base_account is not set"),

            market: self.instruction.market.expect("market is not set"),

            bids: self.instruction.bids.expect("bids is not set"),

            asks: self.instruction.asks.expect("asks is not set"),

            event_heap: self.instruction.event_heap.expect("event_heap is not set"),

            market_quote_vault: self
                .instruction
                .market_quote_vault
                .expect("market_quote_vault is not set"),

            market_base_vault: self
                .instruction
                .market_base_vault
                .expect("market_base_vault is not set"),

            oracle_a: self.instruction.oracle_a,

            oracle_b: self.instruction.oracle_b,

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CancelAllAndPlaceOrdersCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    open_orders_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    open_orders_admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_quote_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_base_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bids: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    asks: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    event_heap: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    market_quote_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    market_base_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    oracle_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    oracle_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    orders_type: Option<PlaceOrderType>,
    bids_arg: Option<Vec<PlaceMultipleOrdersArgs>>,
    asks_arg: Option<Vec<PlaceMultipleOrdersArgs>>,
    limit: Option<u8>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
