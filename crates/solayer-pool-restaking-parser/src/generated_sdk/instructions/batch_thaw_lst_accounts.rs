//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct BatchThawLstAccounts {
    pub signer: solana_program::pubkey::Pubkey,

    pub solayer_admin: solana_program::pubkey::Pubkey,

    pub lst_mint: solana_program::pubkey::Pubkey,

    pub rst_mint: solana_program::pubkey::Pubkey,

    pub pool: solana_program::pubkey::Pubkey,

    pub associated_token_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
}

impl BatchThawLstAccounts {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.signer,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.solayer_admin,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.lst_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.rst_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.pool, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.associated_token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&BatchThawLstAccountsInstructionData::new()).unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::RESTAKING_PROGRAM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BatchThawLstAccountsInstructionData {
    discriminator: [u8; 8],
}

impl BatchThawLstAccountsInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [183, 174, 77, 40, 182, 134, 202, 213],
        }
    }
}

impl Default for BatchThawLstAccountsInstructionData {
    fn default() -> Self { Self::new() }
}

/// Instruction builder for `BatchThawLstAccounts`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` signer
///   1. `[signer]` solayer_admin
///   2. `[]` lst_mint
///   3. `[writable]` rst_mint
///   4. `[]` pool
///   5. `[]` associated_token_program
///   6. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   7. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct BatchThawLstAccountsBuilder {
    signer: Option<solana_program::pubkey::Pubkey>,
    solayer_admin: Option<solana_program::pubkey::Pubkey>,
    lst_mint: Option<solana_program::pubkey::Pubkey>,
    rst_mint: Option<solana_program::pubkey::Pubkey>,
    pool: Option<solana_program::pubkey::Pubkey>,
    associated_token_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl BatchThawLstAccountsBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn signer(&mut self, signer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.signer = Some(signer);
        self
    }

    #[inline(always)]
    pub fn solayer_admin(&mut self, solayer_admin: solana_program::pubkey::Pubkey) -> &mut Self {
        self.solayer_admin = Some(solayer_admin);
        self
    }

    #[inline(always)]
    pub fn lst_mint(&mut self, lst_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.lst_mint = Some(lst_mint);
        self
    }

    #[inline(always)]
    pub fn rst_mint(&mut self, rst_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rst_mint = Some(rst_mint);
        self
    }

    #[inline(always)]
    pub fn pool(&mut self, pool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.pool = Some(pool);
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

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }

    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
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
        let accounts = BatchThawLstAccounts {
            signer: self.signer.expect("signer is not set"),
            solayer_admin: self.solayer_admin.expect("solayer_admin is not set"),
            lst_mint: self.lst_mint.expect("lst_mint is not set"),
            rst_mint: self.rst_mint.expect("rst_mint is not set"),
            pool: self.pool.expect("pool is not set"),
            associated_token_program: self
                .associated_token_program
                .expect("associated_token_program is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `batch_thaw_lst_accounts` CPI accounts.
pub struct BatchThawLstAccountsCpiAccounts<'a, 'b> {
    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub solayer_admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub lst_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub rst_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `batch_thaw_lst_accounts` CPI instruction.
pub struct BatchThawLstAccountsCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub solayer_admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub lst_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub rst_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> BatchThawLstAccountsCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: BatchThawLstAccountsCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            signer: accounts.signer,
            solayer_admin: accounts.solayer_admin,
            lst_mint: accounts.lst_mint,
            rst_mint: accounts.rst_mint,
            pool: accounts.pool,
            associated_token_program: accounts.associated_token_program,
            token_program: accounts.token_program,
            system_program: accounts.system_program,
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
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.signer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.solayer_admin.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.lst_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.rst_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.pool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.associated_token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
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
        let data = borsh::to_vec(&BatchThawLstAccountsInstructionData::new()).unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::RESTAKING_PROGRAM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.signer.clone());
        account_infos.push(self.solayer_admin.clone());
        account_infos.push(self.lst_mint.clone());
        account_infos.push(self.rst_mint.clone());
        account_infos.push(self.pool.clone());
        account_infos.push(self.associated_token_program.clone());
        account_infos.push(self.token_program.clone());
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

/// Instruction builder for `BatchThawLstAccounts` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` signer
///   1. `[signer]` solayer_admin
///   2. `[]` lst_mint
///   3. `[writable]` rst_mint
///   4. `[]` pool
///   5. `[]` associated_token_program
///   6. `[]` token_program
///   7. `[]` system_program
#[derive(Clone, Debug)]
pub struct BatchThawLstAccountsCpiBuilder<'a, 'b> {
    instruction: Box<BatchThawLstAccountsCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> BatchThawLstAccountsCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(BatchThawLstAccountsCpiBuilderInstruction {
            __program: program,
            signer: None,
            solayer_admin: None,
            lst_mint: None,
            rst_mint: None,
            pool: None,
            associated_token_program: None,
            token_program: None,
            system_program: None,
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
    pub fn solayer_admin(
        &mut self,
        solayer_admin: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.solayer_admin = Some(solayer_admin);
        self
    }

    #[inline(always)]
    pub fn lst_mint(
        &mut self,
        lst_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lst_mint = Some(lst_mint);
        self
    }

    #[inline(always)]
    pub fn rst_mint(
        &mut self,
        rst_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.rst_mint = Some(rst_mint);
        self
    }

    #[inline(always)]
    pub fn pool(&mut self, pool: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.pool = Some(pool);
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
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
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
        let instruction = BatchThawLstAccountsCpi {
            __program: self.instruction.__program,

            signer: self.instruction.signer.expect("signer is not set"),

            solayer_admin: self
                .instruction
                .solayer_admin
                .expect("solayer_admin is not set"),

            lst_mint: self.instruction.lst_mint.expect("lst_mint is not set"),

            rst_mint: self.instruction.rst_mint.expect("rst_mint is not set"),

            pool: self.instruction.pool.expect("pool is not set"),

            associated_token_program: self
                .instruction
                .associated_token_program
                .expect("associated_token_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct BatchThawLstAccountsCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    solayer_admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lst_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rst_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
