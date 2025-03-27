//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

use crate::generated::types::TradeParams;

/// Accounts.
#[derive(Debug)]
pub struct Buy {
    pub sender: solana_program::pubkey::Pubkey,

    pub sender_token_account: solana_program::pubkey::Pubkey,

    pub curve_account: solana_program::pubkey::Pubkey,

    pub curve_token_account: solana_program::pubkey::Pubkey,

    pub dex_fee: solana_program::pubkey::Pubkey,

    pub helio_fee: solana_program::pubkey::Pubkey,

    pub mint: solana_program::pubkey::Pubkey,

    pub config_account: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub associated_token_program: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
}

impl Buy {
    pub fn instruction(
        &self,
        args: BuyInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: BuyInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.sender,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.sender_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.curve_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.curve_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.dex_fee,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.helio_fee,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.mint, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.config_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.associated_token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&BuyInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::TOKEN_LAUNCHPAD_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuyInstructionData {
    discriminator: [u8; 8],
}

impl BuyInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [102, 6, 61, 18, 1, 218, 235, 234],
        }
    }
}

impl Default for BuyInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuyInstructionArgs {
    pub data: TradeParams,
}

/// Instruction builder for `Buy`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` sender
///   1. `[writable]` sender_token_account
///   2. `[writable]` curve_account
///   3. `[writable]` curve_token_account
///   4. `[writable]` dex_fee
///   5. `[writable]` helio_fee
///   6. `[]` mint
///   7. `[]` config_account
///   8. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   9. `[]` associated_token_program
///   10. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct BuyBuilder {
    sender: Option<solana_program::pubkey::Pubkey>,
    sender_token_account: Option<solana_program::pubkey::Pubkey>,
    curve_account: Option<solana_program::pubkey::Pubkey>,
    curve_token_account: Option<solana_program::pubkey::Pubkey>,
    dex_fee: Option<solana_program::pubkey::Pubkey>,
    helio_fee: Option<solana_program::pubkey::Pubkey>,
    mint: Option<solana_program::pubkey::Pubkey>,
    config_account: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    associated_token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    data: Option<TradeParams>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl BuyBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn sender(&mut self, sender: solana_program::pubkey::Pubkey) -> &mut Self {
        self.sender = Some(sender);
        self
    }

    #[inline(always)]
    pub fn sender_token_account(
        &mut self,
        sender_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.sender_token_account = Some(sender_token_account);
        self
    }

    #[inline(always)]
    pub fn curve_account(&mut self, curve_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.curve_account = Some(curve_account);
        self
    }

    #[inline(always)]
    pub fn curve_token_account(
        &mut self,
        curve_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.curve_token_account = Some(curve_token_account);
        self
    }

    #[inline(always)]
    pub fn dex_fee(&mut self, dex_fee: solana_program::pubkey::Pubkey) -> &mut Self {
        self.dex_fee = Some(dex_fee);
        self
    }

    #[inline(always)]
    pub fn helio_fee(&mut self, helio_fee: solana_program::pubkey::Pubkey) -> &mut Self {
        self.helio_fee = Some(helio_fee);
        self
    }

    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }

    #[inline(always)]
    pub fn config_account(&mut self, config_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.config_account = Some(config_account);
        self
    }

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.associated_token_program = Some(associated_token_program);
        self
    }

    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }

    #[inline(always)]
    pub fn data(&mut self, data: TradeParams) -> &mut Self {
        self.data = Some(data);
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
        let accounts = Buy {
            sender: self.sender.expect("sender is not set"),
            sender_token_account: self
                .sender_token_account
                .expect("sender_token_account is not set"),
            curve_account: self.curve_account.expect("curve_account is not set"),
            curve_token_account: self
                .curve_token_account
                .expect("curve_token_account is not set"),
            dex_fee: self.dex_fee.expect("dex_fee is not set"),
            helio_fee: self.helio_fee.expect("helio_fee is not set"),
            mint: self.mint.expect("mint is not set"),
            config_account: self.config_account.expect("config_account is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            associated_token_program: self
                .associated_token_program
                .expect("associated_token_program is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = BuyInstructionArgs {
            data: self.data.clone().expect("data is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `buy` CPI accounts.
pub struct BuyCpiAccounts<'a, 'b> {
    pub sender: &'b solana_program::account_info::AccountInfo<'a>,

    pub sender_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub curve_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub curve_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub dex_fee: &'b solana_program::account_info::AccountInfo<'a>,

    pub helio_fee: &'b solana_program::account_info::AccountInfo<'a>,

    pub mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub config_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `buy` CPI instruction.
pub struct BuyCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub sender: &'b solana_program::account_info::AccountInfo<'a>,

    pub sender_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub curve_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub curve_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub dex_fee: &'b solana_program::account_info::AccountInfo<'a>,

    pub helio_fee: &'b solana_program::account_info::AccountInfo<'a>,

    pub mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub config_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: BuyInstructionArgs,
}

impl<'a, 'b> BuyCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: BuyCpiAccounts<'a, 'b>,
        args: BuyInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            sender: accounts.sender,
            sender_token_account: accounts.sender_token_account,
            curve_account: accounts.curve_account,
            curve_token_account: accounts.curve_token_account,
            dex_fee: accounts.dex_fee,
            helio_fee: accounts.helio_fee,
            mint: accounts.mint,
            config_account: accounts.config_account,
            token_program: accounts.token_program,
            associated_token_program: accounts.associated_token_program,
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
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.sender.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.sender_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.curve_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.curve_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.dex_fee.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.helio_fee.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.config_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.associated_token_program.key,
            false,
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
        let mut data = borsh::to_vec(&BuyInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::TOKEN_LAUNCHPAD_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(12 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.sender.clone());
        account_infos.push(self.sender_token_account.clone());
        account_infos.push(self.curve_account.clone());
        account_infos.push(self.curve_token_account.clone());
        account_infos.push(self.dex_fee.clone());
        account_infos.push(self.helio_fee.clone());
        account_infos.push(self.mint.clone());
        account_infos.push(self.config_account.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.associated_token_program.clone());
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

/// Instruction builder for `Buy` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` sender
///   1. `[writable]` sender_token_account
///   2. `[writable]` curve_account
///   3. `[writable]` curve_token_account
///   4. `[writable]` dex_fee
///   5. `[writable]` helio_fee
///   6. `[]` mint
///   7. `[]` config_account
///   8. `[]` token_program
///   9. `[]` associated_token_program
///   10. `[]` system_program
#[derive(Clone, Debug)]
pub struct BuyCpiBuilder<'a, 'b> {
    instruction: Box<BuyCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> BuyCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(BuyCpiBuilderInstruction {
            __program: program,
            sender: None,
            sender_token_account: None,
            curve_account: None,
            curve_token_account: None,
            dex_fee: None,
            helio_fee: None,
            mint: None,
            config_account: None,
            token_program: None,
            associated_token_program: None,
            system_program: None,
            data: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn sender(
        &mut self,
        sender: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.sender = Some(sender);
        self
    }

    #[inline(always)]
    pub fn sender_token_account(
        &mut self,
        sender_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.sender_token_account = Some(sender_token_account);
        self
    }

    #[inline(always)]
    pub fn curve_account(
        &mut self,
        curve_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.curve_account = Some(curve_account);
        self
    }

    #[inline(always)]
    pub fn curve_token_account(
        &mut self,
        curve_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.curve_token_account = Some(curve_token_account);
        self
    }

    #[inline(always)]
    pub fn dex_fee(
        &mut self,
        dex_fee: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.dex_fee = Some(dex_fee);
        self
    }

    #[inline(always)]
    pub fn helio_fee(
        &mut self,
        helio_fee: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.helio_fee = Some(helio_fee);
        self
    }

    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
        self
    }

    #[inline(always)]
    pub fn config_account(
        &mut self,
        config_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.config_account = Some(config_account);
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
    pub fn associated_token_program(
        &mut self,
        associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.associated_token_program = Some(associated_token_program);
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
    pub fn data(&mut self, data: TradeParams) -> &mut Self {
        self.instruction.data = Some(data);
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
        let args = BuyInstructionArgs {
            data: self.instruction.data.clone().expect("data is not set"),
        };
        let instruction = BuyCpi {
            __program: self.instruction.__program,

            sender: self.instruction.sender.expect("sender is not set"),

            sender_token_account: self
                .instruction
                .sender_token_account
                .expect("sender_token_account is not set"),

            curve_account: self
                .instruction
                .curve_account
                .expect("curve_account is not set"),

            curve_token_account: self
                .instruction
                .curve_token_account
                .expect("curve_token_account is not set"),

            dex_fee: self.instruction.dex_fee.expect("dex_fee is not set"),

            helio_fee: self.instruction.helio_fee.expect("helio_fee is not set"),

            mint: self.instruction.mint.expect("mint is not set"),

            config_account: self
                .instruction
                .config_account
                .expect("config_account is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            associated_token_program: self
                .instruction
                .associated_token_program
                .expect("associated_token_program is not set"),

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
struct BuyCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    sender: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    sender_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    curve_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    curve_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    dex_fee: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    helio_fee: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    config_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    data: Option<TradeParams>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
