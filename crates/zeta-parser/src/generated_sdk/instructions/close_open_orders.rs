//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct CloseOpenOrders {
    pub state: solana_program::pubkey::Pubkey,

    pub zeta_group: solana_program::pubkey::Pubkey,

    pub dex_program: solana_program::pubkey::Pubkey,

    pub open_orders: solana_program::pubkey::Pubkey,

    pub margin_account: solana_program::pubkey::Pubkey,

    pub authority: solana_program::pubkey::Pubkey,

    pub market: solana_program::pubkey::Pubkey,

    pub serum_authority: solana_program::pubkey::Pubkey,

    pub open_orders_map: solana_program::pubkey::Pubkey,
}

impl CloseOpenOrders {
    pub fn instruction(
        &self,
        args: CloseOpenOrdersInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CloseOpenOrdersInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.state, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.dex_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.open_orders,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.margin_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.market,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.serum_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.open_orders_map,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&CloseOpenOrdersInstructionData::new()).unwrap();
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
pub struct CloseOpenOrdersInstructionData {
    discriminator: [u8; 8],
}

impl CloseOpenOrdersInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [200, 216, 63, 239, 7, 230, 255, 20],
        }
    }
}

impl Default for CloseOpenOrdersInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CloseOpenOrdersInstructionArgs {
    pub map_nonce: u8,
}

/// Instruction builder for `CloseOpenOrders`.
///
/// ### Accounts:
///
///   0. `[]` state
///   1. `[]` zeta_group
///   2. `[]` dex_program
///   3. `[writable]` open_orders
///   4. `[writable]` margin_account
///   5. `[writable, signer]` authority
///   6. `[]` market
///   7. `[]` serum_authority
///   8. `[writable]` open_orders_map
#[derive(Clone, Debug, Default)]
pub struct CloseOpenOrdersBuilder {
    state: Option<solana_program::pubkey::Pubkey>,
    zeta_group: Option<solana_program::pubkey::Pubkey>,
    dex_program: Option<solana_program::pubkey::Pubkey>,
    open_orders: Option<solana_program::pubkey::Pubkey>,
    margin_account: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    market: Option<solana_program::pubkey::Pubkey>,
    serum_authority: Option<solana_program::pubkey::Pubkey>,
    open_orders_map: Option<solana_program::pubkey::Pubkey>,
    map_nonce: Option<u8>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CloseOpenOrdersBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn state(&mut self, state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn zeta_group(&mut self, zeta_group: solana_program::pubkey::Pubkey) -> &mut Self {
        self.zeta_group = Some(zeta_group);
        self
    }

    #[inline(always)]
    pub fn dex_program(&mut self, dex_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.dex_program = Some(dex_program);
        self
    }

    #[inline(always)]
    pub fn open_orders(&mut self, open_orders: solana_program::pubkey::Pubkey) -> &mut Self {
        self.open_orders = Some(open_orders);
        self
    }

    #[inline(always)]
    pub fn margin_account(&mut self, margin_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.margin_account = Some(margin_account);
        self
    }

    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }

    #[inline(always)]
    pub fn market(&mut self, market: solana_program::pubkey::Pubkey) -> &mut Self {
        self.market = Some(market);
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
    pub fn open_orders_map(
        &mut self,
        open_orders_map: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.open_orders_map = Some(open_orders_map);
        self
    }

    #[inline(always)]
    pub fn map_nonce(&mut self, map_nonce: u8) -> &mut Self {
        self.map_nonce = Some(map_nonce);
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
        let accounts = CloseOpenOrders {
            state: self.state.expect("state is not set"),
            zeta_group: self.zeta_group.expect("zeta_group is not set"),
            dex_program: self.dex_program.expect("dex_program is not set"),
            open_orders: self.open_orders.expect("open_orders is not set"),
            margin_account: self.margin_account.expect("margin_account is not set"),
            authority: self.authority.expect("authority is not set"),
            market: self.market.expect("market is not set"),
            serum_authority: self.serum_authority.expect("serum_authority is not set"),
            open_orders_map: self.open_orders_map.expect("open_orders_map is not set"),
        };
        let args = CloseOpenOrdersInstructionArgs {
            map_nonce: self.map_nonce.clone().expect("map_nonce is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `close_open_orders` CPI accounts.
pub struct CloseOpenOrdersCpiAccounts<'a, 'b> {
    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub zeta_group: &'b solana_program::account_info::AccountInfo<'a>,

    pub dex_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub open_orders: &'b solana_program::account_info::AccountInfo<'a>,

    pub margin_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub market: &'b solana_program::account_info::AccountInfo<'a>,

    pub serum_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub open_orders_map: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `close_open_orders` CPI instruction.
pub struct CloseOpenOrdersCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub zeta_group: &'b solana_program::account_info::AccountInfo<'a>,

    pub dex_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub open_orders: &'b solana_program::account_info::AccountInfo<'a>,

    pub margin_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub market: &'b solana_program::account_info::AccountInfo<'a>,

    pub serum_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub open_orders_map: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CloseOpenOrdersInstructionArgs,
}

impl<'a, 'b> CloseOpenOrdersCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CloseOpenOrdersCpiAccounts<'a, 'b>,
        args: CloseOpenOrdersInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            state: accounts.state,
            zeta_group: accounts.zeta_group,
            dex_program: accounts.dex_program,
            open_orders: accounts.open_orders,
            margin_account: accounts.margin_account,
            authority: accounts.authority,
            market: accounts.market,
            serum_authority: accounts.serum_authority,
            open_orders_map: accounts.open_orders_map,
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
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.zeta_group.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.dex_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.open_orders.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.margin_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.market.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.serum_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.open_orders_map.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&CloseOpenOrdersInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::ZETA_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(10 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.state.clone());
        account_infos.push(self.zeta_group.clone());
        account_infos.push(self.dex_program.clone());
        account_infos.push(self.open_orders.clone());
        account_infos.push(self.margin_account.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.market.clone());
        account_infos.push(self.serum_authority.clone());
        account_infos.push(self.open_orders_map.clone());
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

/// Instruction builder for `CloseOpenOrders` via CPI.
///
/// ### Accounts:
///
///   0. `[]` state
///   1. `[]` zeta_group
///   2. `[]` dex_program
///   3. `[writable]` open_orders
///   4. `[writable]` margin_account
///   5. `[writable, signer]` authority
///   6. `[]` market
///   7. `[]` serum_authority
///   8. `[writable]` open_orders_map
#[derive(Clone, Debug)]
pub struct CloseOpenOrdersCpiBuilder<'a, 'b> {
    instruction: Box<CloseOpenOrdersCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CloseOpenOrdersCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CloseOpenOrdersCpiBuilderInstruction {
            __program: program,
            state: None,
            zeta_group: None,
            dex_program: None,
            open_orders: None,
            margin_account: None,
            authority: None,
            market: None,
            serum_authority: None,
            open_orders_map: None,
            map_nonce: None,
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
    pub fn zeta_group(
        &mut self,
        zeta_group: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.zeta_group = Some(zeta_group);
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
    pub fn open_orders(
        &mut self,
        open_orders: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.open_orders = Some(open_orders);
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
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
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
    pub fn serum_authority(
        &mut self,
        serum_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.serum_authority = Some(serum_authority);
        self
    }

    #[inline(always)]
    pub fn open_orders_map(
        &mut self,
        open_orders_map: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.open_orders_map = Some(open_orders_map);
        self
    }

    #[inline(always)]
    pub fn map_nonce(&mut self, map_nonce: u8) -> &mut Self {
        self.instruction.map_nonce = Some(map_nonce);
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
        let args = CloseOpenOrdersInstructionArgs {
            map_nonce: self
                .instruction
                .map_nonce
                .clone()
                .expect("map_nonce is not set"),
        };
        let instruction = CloseOpenOrdersCpi {
            __program: self.instruction.__program,

            state: self.instruction.state.expect("state is not set"),

            zeta_group: self.instruction.zeta_group.expect("zeta_group is not set"),

            dex_program: self
                .instruction
                .dex_program
                .expect("dex_program is not set"),

            open_orders: self
                .instruction
                .open_orders
                .expect("open_orders is not set"),

            margin_account: self
                .instruction
                .margin_account
                .expect("margin_account is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            market: self.instruction.market.expect("market is not set"),

            serum_authority: self
                .instruction
                .serum_authority
                .expect("serum_authority is not set"),

            open_orders_map: self
                .instruction
                .open_orders_map
                .expect("open_orders_map is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CloseOpenOrdersCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    zeta_group: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    dex_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    open_orders: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    margin_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    serum_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    open_orders_map: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    map_nonce: Option<u8>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
