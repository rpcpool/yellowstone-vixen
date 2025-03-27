//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct InitializeMarketNode {
    pub zeta_group: solana_program::pubkey::Pubkey,

    pub market_node: solana_program::pubkey::Pubkey,

    pub greeks: solana_program::pubkey::Pubkey,

    pub payer: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
}

impl InitializeMarketNode {
    pub fn instruction(
        &self,
        args: InitializeMarketNodeInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializeMarketNodeInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.zeta_group,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.market_node,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.greeks,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&InitializeMarketNodeInstructionData::new()).unwrap();
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
pub struct InitializeMarketNodeInstructionData {
    discriminator: [u8; 8],
}

impl InitializeMarketNodeInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [50, 118, 21, 21, 179, 248, 23, 128],
        }
    }
}

impl Default for InitializeMarketNodeInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeMarketNodeInstructionArgs {
    pub nonce: u8,
    pub index: u8,
}

/// Instruction builder for `InitializeMarketNode`.
///
/// ### Accounts:
///
///   0. `[]` zeta_group
///   1. `[writable]` market_node
///   2. `[writable]` greeks
///   3. `[writable, signer]` payer
///   4. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitializeMarketNodeBuilder {
    zeta_group: Option<solana_program::pubkey::Pubkey>,
    market_node: Option<solana_program::pubkey::Pubkey>,
    greeks: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    nonce: Option<u8>,
    index: Option<u8>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeMarketNodeBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn zeta_group(&mut self, zeta_group: solana_program::pubkey::Pubkey) -> &mut Self {
        self.zeta_group = Some(zeta_group);
        self
    }

    #[inline(always)]
    pub fn market_node(&mut self, market_node: solana_program::pubkey::Pubkey) -> &mut Self {
        self.market_node = Some(market_node);
        self
    }

    #[inline(always)]
    pub fn greeks(&mut self, greeks: solana_program::pubkey::Pubkey) -> &mut Self {
        self.greeks = Some(greeks);
        self
    }

    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }

    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }

    #[inline(always)]
    pub fn nonce(&mut self, nonce: u8) -> &mut Self {
        self.nonce = Some(nonce);
        self
    }

    #[inline(always)]
    pub fn index(&mut self, index: u8) -> &mut Self {
        self.index = Some(index);
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
        let accounts = InitializeMarketNode {
            zeta_group: self.zeta_group.expect("zeta_group is not set"),
            market_node: self.market_node.expect("market_node is not set"),
            greeks: self.greeks.expect("greeks is not set"),
            payer: self.payer.expect("payer is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = InitializeMarketNodeInstructionArgs {
            nonce: self.nonce.clone().expect("nonce is not set"),
            index: self.index.clone().expect("index is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize_market_node` CPI accounts.
pub struct InitializeMarketNodeCpiAccounts<'a, 'b> {
    pub zeta_group: &'b solana_program::account_info::AccountInfo<'a>,

    pub market_node: &'b solana_program::account_info::AccountInfo<'a>,

    pub greeks: &'b solana_program::account_info::AccountInfo<'a>,

    pub payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_market_node` CPI instruction.
pub struct InitializeMarketNodeCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub zeta_group: &'b solana_program::account_info::AccountInfo<'a>,

    pub market_node: &'b solana_program::account_info::AccountInfo<'a>,

    pub greeks: &'b solana_program::account_info::AccountInfo<'a>,

    pub payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitializeMarketNodeInstructionArgs,
}

impl<'a, 'b> InitializeMarketNodeCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeMarketNodeCpiAccounts<'a, 'b>,
        args: InitializeMarketNodeInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            zeta_group: accounts.zeta_group,
            market_node: accounts.market_node,
            greeks: accounts.greeks,
            payer: accounts.payer,
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
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.zeta_group.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.market_node.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.greeks.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
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
        let mut data = borsh::to_vec(&InitializeMarketNodeInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::ZETA_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.zeta_group.clone());
        account_infos.push(self.market_node.clone());
        account_infos.push(self.greeks.clone());
        account_infos.push(self.payer.clone());
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

/// Instruction builder for `InitializeMarketNode` via CPI.
///
/// ### Accounts:
///
///   0. `[]` zeta_group
///   1. `[writable]` market_node
///   2. `[writable]` greeks
///   3. `[writable, signer]` payer
///   4. `[]` system_program
#[derive(Clone, Debug)]
pub struct InitializeMarketNodeCpiBuilder<'a, 'b> {
    instruction: Box<InitializeMarketNodeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeMarketNodeCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeMarketNodeCpiBuilderInstruction {
            __program: program,
            zeta_group: None,
            market_node: None,
            greeks: None,
            payer: None,
            system_program: None,
            nonce: None,
            index: None,
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
    pub fn market_node(
        &mut self,
        market_node: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.market_node = Some(market_node);
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
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
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
    pub fn nonce(&mut self, nonce: u8) -> &mut Self {
        self.instruction.nonce = Some(nonce);
        self
    }

    #[inline(always)]
    pub fn index(&mut self, index: u8) -> &mut Self {
        self.instruction.index = Some(index);
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
        let args = InitializeMarketNodeInstructionArgs {
            nonce: self.instruction.nonce.clone().expect("nonce is not set"),
            index: self.instruction.index.clone().expect("index is not set"),
        };
        let instruction = InitializeMarketNodeCpi {
            __program: self.instruction.__program,

            zeta_group: self.instruction.zeta_group.expect("zeta_group is not set"),

            market_node: self
                .instruction
                .market_node
                .expect("market_node is not set"),

            greeks: self.instruction.greeks.expect("greeks is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

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
struct InitializeMarketNodeCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    zeta_group: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    market_node: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    greeks: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    nonce: Option<u8>,
    index: Option<u8>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
