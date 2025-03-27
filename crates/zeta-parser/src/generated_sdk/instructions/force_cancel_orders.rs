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
pub struct ForceCancelOrders {
    pub zeta_group: solana_program::pubkey::Pubkey,

    pub greeks: solana_program::pubkey::Pubkey,

    pub oracle: solana_program::pubkey::Pubkey,

    pub oracle_backup_feed: solana_program::pubkey::Pubkey,

    pub oracle_backup_program: solana_program::pubkey::Pubkey,

    pub state: solana_program::pubkey::Pubkey,

    pub margin_account: solana_program::pubkey::Pubkey,

    pub dex_program: solana_program::pubkey::Pubkey,

    pub serum_authority: solana_program::pubkey::Pubkey,

    pub open_orders: solana_program::pubkey::Pubkey,

    pub market: solana_program::pubkey::Pubkey,

    pub bids: solana_program::pubkey::Pubkey,

    pub asks: solana_program::pubkey::Pubkey,

    pub event_queue: solana_program::pubkey::Pubkey,
}

impl ForceCancelOrders {
    pub fn instruction(
        &self,
        args: ForceCancelOrdersInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: ForceCancelOrdersInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(14 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.greeks,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_feed,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.oracle_backup_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.state, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.open_orders,
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
            self.event_queue,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&ForceCancelOrdersInstructionData::new()).unwrap();
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
pub struct ForceCancelOrdersInstructionData {
    discriminator: [u8; 8],
}

impl ForceCancelOrdersInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [64, 181, 196, 63, 222, 72, 64, 232],
        }
    }
}

impl Default for ForceCancelOrdersInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForceCancelOrdersInstructionArgs {
    pub asset: Asset,
}

/// Instruction builder for `ForceCancelOrders`.
///
/// ### Accounts:
///
///   0. `[]` zeta_group
///   1. `[]` greeks
///   2. `[]` oracle
///   3. `[]` oracle_backup_feed
///   4. `[]` oracle_backup_program
///   5. `[]` state
///   6. `[writable]` margin_account
///   7. `[]` dex_program
///   8. `[]` serum_authority
///   9. `[writable]` open_orders
///   10. `[writable]` market
///   11. `[writable]` bids
///   12. `[writable]` asks
///   13. `[writable]` event_queue
#[derive(Clone, Debug, Default)]
pub struct ForceCancelOrdersBuilder {
    zeta_group: Option<solana_program::pubkey::Pubkey>,
    greeks: Option<solana_program::pubkey::Pubkey>,
    oracle: Option<solana_program::pubkey::Pubkey>,
    oracle_backup_feed: Option<solana_program::pubkey::Pubkey>,
    oracle_backup_program: Option<solana_program::pubkey::Pubkey>,
    state: Option<solana_program::pubkey::Pubkey>,
    margin_account: Option<solana_program::pubkey::Pubkey>,
    dex_program: Option<solana_program::pubkey::Pubkey>,
    serum_authority: Option<solana_program::pubkey::Pubkey>,
    open_orders: Option<solana_program::pubkey::Pubkey>,
    market: Option<solana_program::pubkey::Pubkey>,
    bids: Option<solana_program::pubkey::Pubkey>,
    asks: Option<solana_program::pubkey::Pubkey>,
    event_queue: Option<solana_program::pubkey::Pubkey>,
    asset: Option<Asset>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ForceCancelOrdersBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn zeta_group(&mut self, zeta_group: solana_program::pubkey::Pubkey) -> &mut Self {
        self.zeta_group = Some(zeta_group);
        self
    }

    #[inline(always)]
    pub fn greeks(&mut self, greeks: solana_program::pubkey::Pubkey) -> &mut Self {
        self.greeks = Some(greeks);
        self
    }

    #[inline(always)]
    pub fn oracle(&mut self, oracle: solana_program::pubkey::Pubkey) -> &mut Self {
        self.oracle = Some(oracle);
        self
    }

    #[inline(always)]
    pub fn oracle_backup_feed(
        &mut self,
        oracle_backup_feed: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.oracle_backup_feed = Some(oracle_backup_feed);
        self
    }

    #[inline(always)]
    pub fn oracle_backup_program(
        &mut self,
        oracle_backup_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.oracle_backup_program = Some(oracle_backup_program);
        self
    }

    #[inline(always)]
    pub fn state(&mut self, state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn margin_account(&mut self, margin_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.margin_account = Some(margin_account);
        self
    }

    #[inline(always)]
    pub fn dex_program(&mut self, dex_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.dex_program = Some(dex_program);
        self
    }

    #[inline(always)]
    pub fn serum_authority(
        &mut self,
        serum_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.serum_authority = Some(serum_authority);
        self
    }

    #[inline(always)]
    pub fn open_orders(&mut self, open_orders: solana_program::pubkey::Pubkey) -> &mut Self {
        self.open_orders = Some(open_orders);
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
    pub fn event_queue(&mut self, event_queue: solana_program::pubkey::Pubkey) -> &mut Self {
        self.event_queue = Some(event_queue);
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
        let accounts = ForceCancelOrders {
            zeta_group: self.zeta_group.expect("zeta_group is not set"),
            greeks: self.greeks.expect("greeks is not set"),
            oracle: self.oracle.expect("oracle is not set"),
            oracle_backup_feed: self
                .oracle_backup_feed
                .expect("oracle_backup_feed is not set"),
            oracle_backup_program: self
                .oracle_backup_program
                .expect("oracle_backup_program is not set"),
            state: self.state.expect("state is not set"),
            margin_account: self.margin_account.expect("margin_account is not set"),
            dex_program: self.dex_program.expect("dex_program is not set"),
            serum_authority: self.serum_authority.expect("serum_authority is not set"),
            open_orders: self.open_orders.expect("open_orders is not set"),
            market: self.market.expect("market is not set"),
            bids: self.bids.expect("bids is not set"),
            asks: self.asks.expect("asks is not set"),
            event_queue: self.event_queue.expect("event_queue is not set"),
        };
        let args = ForceCancelOrdersInstructionArgs {
            asset: self.asset.clone().expect("asset is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `force_cancel_orders` CPI accounts.
pub struct ForceCancelOrdersCpiAccounts<'a, 'b> {
    pub zeta_group: &'b solana_program::account_info::AccountInfo<'a>,

    pub greeks: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle_backup_feed: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle_backup_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub margin_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub dex_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub serum_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub open_orders: &'b solana_program::account_info::AccountInfo<'a>,

    pub market: &'b solana_program::account_info::AccountInfo<'a>,

    pub bids: &'b solana_program::account_info::AccountInfo<'a>,

    pub asks: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_queue: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `force_cancel_orders` CPI instruction.
pub struct ForceCancelOrdersCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub zeta_group: &'b solana_program::account_info::AccountInfo<'a>,

    pub greeks: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle_backup_feed: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle_backup_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub margin_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub dex_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub serum_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub open_orders: &'b solana_program::account_info::AccountInfo<'a>,

    pub market: &'b solana_program::account_info::AccountInfo<'a>,

    pub bids: &'b solana_program::account_info::AccountInfo<'a>,

    pub asks: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_queue: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: ForceCancelOrdersInstructionArgs,
}

impl<'a, 'b> ForceCancelOrdersCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ForceCancelOrdersCpiAccounts<'a, 'b>,
        args: ForceCancelOrdersInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            zeta_group: accounts.zeta_group,
            greeks: accounts.greeks,
            oracle: accounts.oracle,
            oracle_backup_feed: accounts.oracle_backup_feed,
            oracle_backup_program: accounts.oracle_backup_program,
            state: accounts.state,
            margin_account: accounts.margin_account,
            dex_program: accounts.dex_program,
            serum_authority: accounts.serum_authority,
            open_orders: accounts.open_orders,
            market: accounts.market,
            bids: accounts.bids,
            asks: accounts.asks,
            event_queue: accounts.event_queue,
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
            *self.zeta_group.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.greeks.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.oracle.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.oracle_backup_feed.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.oracle_backup_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.margin_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.dex_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.serum_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.open_orders.key,
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
            *self.event_queue.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&ForceCancelOrdersInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::ZETA_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(15 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.zeta_group.clone());
        account_infos.push(self.greeks.clone());
        account_infos.push(self.oracle.clone());
        account_infos.push(self.oracle_backup_feed.clone());
        account_infos.push(self.oracle_backup_program.clone());
        account_infos.push(self.state.clone());
        account_infos.push(self.margin_account.clone());
        account_infos.push(self.dex_program.clone());
        account_infos.push(self.serum_authority.clone());
        account_infos.push(self.open_orders.clone());
        account_infos.push(self.market.clone());
        account_infos.push(self.bids.clone());
        account_infos.push(self.asks.clone());
        account_infos.push(self.event_queue.clone());
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

/// Instruction builder for `ForceCancelOrders` via CPI.
///
/// ### Accounts:
///
///   0. `[]` zeta_group
///   1. `[]` greeks
///   2. `[]` oracle
///   3. `[]` oracle_backup_feed
///   4. `[]` oracle_backup_program
///   5. `[]` state
///   6. `[writable]` margin_account
///   7. `[]` dex_program
///   8. `[]` serum_authority
///   9. `[writable]` open_orders
///   10. `[writable]` market
///   11. `[writable]` bids
///   12. `[writable]` asks
///   13. `[writable]` event_queue
#[derive(Clone, Debug)]
pub struct ForceCancelOrdersCpiBuilder<'a, 'b> {
    instruction: Box<ForceCancelOrdersCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ForceCancelOrdersCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ForceCancelOrdersCpiBuilderInstruction {
            __program: program,
            zeta_group: None,
            greeks: None,
            oracle: None,
            oracle_backup_feed: None,
            oracle_backup_program: None,
            state: None,
            margin_account: None,
            dex_program: None,
            serum_authority: None,
            open_orders: None,
            market: None,
            bids: None,
            asks: None,
            event_queue: None,
            asset: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn zeta_group(
        &mut self,
        zeta_group: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.zeta_group = Some(zeta_group);
        self
    }

    #[inline(always)]
    pub fn greeks(
        &mut self,
        greeks: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.greeks = Some(greeks);
        self
    }

    #[inline(always)]
    pub fn oracle(
        &mut self,
        oracle: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.oracle = Some(oracle);
        self
    }

    #[inline(always)]
    pub fn oracle_backup_feed(
        &mut self,
        oracle_backup_feed: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.oracle_backup_feed = Some(oracle_backup_feed);
        self
    }

    #[inline(always)]
    pub fn oracle_backup_program(
        &mut self,
        oracle_backup_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.oracle_backup_program = Some(oracle_backup_program);
        self
    }

    #[inline(always)]
    pub fn state(&mut self, state: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn margin_account(
        &mut self,
        margin_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.margin_account = Some(margin_account);
        self
    }

    #[inline(always)]
    pub fn dex_program(
        &mut self,
        dex_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.dex_program = Some(dex_program);
        self
    }

    #[inline(always)]
    pub fn serum_authority(
        &mut self,
        serum_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.serum_authority = Some(serum_authority);
        self
    }

    #[inline(always)]
    pub fn open_orders(
        &mut self,
        open_orders: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.open_orders = Some(open_orders);
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
    pub fn event_queue(
        &mut self,
        event_queue: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.event_queue = Some(event_queue);
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
        let args = ForceCancelOrdersInstructionArgs {
            asset: self.instruction.asset.clone().expect("asset is not set"),
        };
        let instruction = ForceCancelOrdersCpi {
            __program: self.instruction.__program,

            zeta_group: self.instruction.zeta_group.expect("zeta_group is not set"),

            greeks: self.instruction.greeks.expect("greeks is not set"),

            oracle: self.instruction.oracle.expect("oracle is not set"),

            oracle_backup_feed: self
                .instruction
                .oracle_backup_feed
                .expect("oracle_backup_feed is not set"),

            oracle_backup_program: self
                .instruction
                .oracle_backup_program
                .expect("oracle_backup_program is not set"),

            state: self.instruction.state.expect("state is not set"),

            margin_account: self
                .instruction
                .margin_account
                .expect("margin_account is not set"),

            dex_program: self
                .instruction
                .dex_program
                .expect("dex_program is not set"),

            serum_authority: self
                .instruction
                .serum_authority
                .expect("serum_authority is not set"),

            open_orders: self
                .instruction
                .open_orders
                .expect("open_orders is not set"),

            market: self.instruction.market.expect("market is not set"),

            bids: self.instruction.bids.expect("bids is not set"),

            asks: self.instruction.asks.expect("asks is not set"),

            event_queue: self
                .instruction
                .event_queue
                .expect("event_queue is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct ForceCancelOrdersCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    zeta_group: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    greeks: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    oracle: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    oracle_backup_feed: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    oracle_backup_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    margin_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    dex_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    serum_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    open_orders: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bids: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    asks: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    event_queue: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    asset: Option<Asset>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
