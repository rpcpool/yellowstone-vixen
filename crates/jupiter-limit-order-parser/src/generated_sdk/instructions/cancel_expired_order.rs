//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct CancelExpiredOrder {
    pub order: solana_program::pubkey::Pubkey,
    /// CHECK
    pub reserve: solana_program::pubkey::Pubkey,

    pub maker: solana_program::pubkey::Pubkey,
    /// CHECK, it is not important if it is sol input mint
    pub maker_input_account: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub input_mint: Option<solana_program::pubkey::Pubkey>,
}

impl CancelExpiredOrder {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.order, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.reserve,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.maker, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.maker_input_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        if let Some(input_mint) = self.input_mint {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                input_mint, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LIMIT_ORDER_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&CancelExpiredOrderInstructionData::new()).unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::LIMIT_ORDER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelExpiredOrderInstructionData {
    discriminator: [u8; 8],
}

impl CancelExpiredOrderInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [216, 120, 64, 235, 155, 19, 229, 99],
        }
    }
}

impl Default for CancelExpiredOrderInstructionData {
    fn default() -> Self { Self::new() }
}

/// Instruction builder for `CancelExpiredOrder`.
///
/// ### Accounts:
///
///   0. `[writable]` order
///   1. `[writable]` reserve
///   2. `[writable]` maker
///   3. `[writable]` maker_input_account
///   4. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   5. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   6. `[optional]` input_mint
#[derive(Clone, Debug, Default)]
pub struct CancelExpiredOrderBuilder {
    order: Option<solana_program::pubkey::Pubkey>,
    reserve: Option<solana_program::pubkey::Pubkey>,
    maker: Option<solana_program::pubkey::Pubkey>,
    maker_input_account: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    input_mint: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CancelExpiredOrderBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn order(&mut self, order: solana_program::pubkey::Pubkey) -> &mut Self {
        self.order = Some(order);
        self
    }

    /// CHECK
    #[inline(always)]
    pub fn reserve(&mut self, reserve: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reserve = Some(reserve);
        self
    }

    #[inline(always)]
    pub fn maker(&mut self, maker: solana_program::pubkey::Pubkey) -> &mut Self {
        self.maker = Some(maker);
        self
    }

    /// CHECK, it is not important if it is sol input mint
    #[inline(always)]
    pub fn maker_input_account(
        &mut self,
        maker_input_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.maker_input_account = Some(maker_input_account);
        self
    }

    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn input_mint(&mut self, input_mint: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.input_mint = input_mint;
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
        let accounts = CancelExpiredOrder {
            order: self.order.expect("order is not set"),
            reserve: self.reserve.expect("reserve is not set"),
            maker: self.maker.expect("maker is not set"),
            maker_input_account: self
                .maker_input_account
                .expect("maker_input_account is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            input_mint: self.input_mint,
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `cancel_expired_order` CPI accounts.
pub struct CancelExpiredOrderCpiAccounts<'a, 'b> {
    pub order: &'b solana_program::account_info::AccountInfo<'a>,
    /// CHECK
    pub reserve: &'b solana_program::account_info::AccountInfo<'a>,

    pub maker: &'b solana_program::account_info::AccountInfo<'a>,
    /// CHECK, it is not important if it is sol input mint
    pub maker_input_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub input_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `cancel_expired_order` CPI instruction.
pub struct CancelExpiredOrderCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub order: &'b solana_program::account_info::AccountInfo<'a>,
    /// CHECK
    pub reserve: &'b solana_program::account_info::AccountInfo<'a>,

    pub maker: &'b solana_program::account_info::AccountInfo<'a>,
    /// CHECK, it is not important if it is sol input mint
    pub maker_input_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub input_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

impl<'a, 'b> CancelExpiredOrderCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CancelExpiredOrderCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            order: accounts.order,
            reserve: accounts.reserve,
            maker: accounts.maker,
            maker_input_account: accounts.maker_input_account,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            input_mint: accounts.input_mint,
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
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.order.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reserve.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.maker.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.maker_input_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        if let Some(input_mint) = self.input_mint {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *input_mint.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LIMIT_ORDER_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = borsh::to_vec(&CancelExpiredOrderInstructionData::new()).unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LIMIT_ORDER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(8 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.order.clone());
        account_infos.push(self.reserve.clone());
        account_infos.push(self.maker.clone());
        account_infos.push(self.maker_input_account.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        if let Some(input_mint) = self.input_mint {
            account_infos.push(input_mint.clone());
        }
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

/// Instruction builder for `CancelExpiredOrder` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` order
///   1. `[writable]` reserve
///   2. `[writable]` maker
///   3. `[writable]` maker_input_account
///   4. `[]` system_program
///   5. `[]` token_program
///   6. `[optional]` input_mint
#[derive(Clone, Debug)]
pub struct CancelExpiredOrderCpiBuilder<'a, 'b> {
    instruction: Box<CancelExpiredOrderCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CancelExpiredOrderCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CancelExpiredOrderCpiBuilderInstruction {
            __program: program,
            order: None,
            reserve: None,
            maker: None,
            maker_input_account: None,
            system_program: None,
            token_program: None,
            input_mint: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn order(&mut self, order: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.order = Some(order);
        self
    }

    /// CHECK
    #[inline(always)]
    pub fn reserve(
        &mut self,
        reserve: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reserve = Some(reserve);
        self
    }

    #[inline(always)]
    pub fn maker(&mut self, maker: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.maker = Some(maker);
        self
    }

    /// CHECK, it is not important if it is sol input mint
    #[inline(always)]
    pub fn maker_input_account(
        &mut self,
        maker_input_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.maker_input_account = Some(maker_input_account);
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
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn input_mint(
        &mut self,
        input_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.input_mint = input_mint;
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
        let instruction = CancelExpiredOrderCpi {
            __program: self.instruction.__program,

            order: self.instruction.order.expect("order is not set"),

            reserve: self.instruction.reserve.expect("reserve is not set"),

            maker: self.instruction.maker.expect("maker is not set"),

            maker_input_account: self
                .instruction
                .maker_input_account
                .expect("maker_input_account is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            input_mint: self.instruction.input_mint,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CancelExpiredOrderCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    order: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reserve: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    maker: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    maker_input_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    input_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
