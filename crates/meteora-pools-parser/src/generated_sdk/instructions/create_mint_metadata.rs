//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct CreateMintMetadata {
    /// Pool account
    pub pool: solana_pubkey::Pubkey,
    /// LP mint account of the pool
    pub lp_mint: solana_pubkey::Pubkey,
    /// Vault A LP account of the pool
    pub a_vault_lp: solana_pubkey::Pubkey,

    pub mint_metadata: solana_pubkey::Pubkey,

    pub metadata_program: solana_pubkey::Pubkey,
    /// System program.
    pub system_program: solana_pubkey::Pubkey,
    /// Payer
    pub payer: solana_pubkey::Pubkey,
}

impl CreateMintMetadata {
    pub fn instruction(&self) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.pool, false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.lp_mint,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.a_vault_lp,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.mint_metadata,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.metadata_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(self.payer, true));
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&CreateMintMetadataInstructionData::new()).unwrap();

        solana_instruction::Instruction {
            program_id: crate::AMM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateMintMetadataInstructionData {
    discriminator: [u8; 8],
}

impl CreateMintMetadataInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [13, 70, 168, 41, 250, 100, 148, 90],
        }
    }
}

impl Default for CreateMintMetadataInstructionData {
    fn default() -> Self { Self::new() }
}

/// Instruction builder for `CreateMintMetadata`.
///
/// ### Accounts:
///
///   0. `[]` pool
///   1. `[]` lp_mint
///   2. `[]` a_vault_lp
///   3. `[writable]` mint_metadata
///   4. `[]` metadata_program
///   5. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   6. `[writable, signer]` payer
#[derive(Clone, Debug, Default)]
pub struct CreateMintMetadataBuilder {
    pool: Option<solana_pubkey::Pubkey>,
    lp_mint: Option<solana_pubkey::Pubkey>,
    a_vault_lp: Option<solana_pubkey::Pubkey>,
    mint_metadata: Option<solana_pubkey::Pubkey>,
    metadata_program: Option<solana_pubkey::Pubkey>,
    system_program: Option<solana_pubkey::Pubkey>,
    payer: Option<solana_pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl CreateMintMetadataBuilder {
    pub fn new() -> Self { Self::default() }

    /// Pool account
    #[inline(always)]
    pub fn pool(&mut self, pool: solana_pubkey::Pubkey) -> &mut Self {
        self.pool = Some(pool);
        self
    }

    /// LP mint account of the pool
    #[inline(always)]
    pub fn lp_mint(&mut self, lp_mint: solana_pubkey::Pubkey) -> &mut Self {
        self.lp_mint = Some(lp_mint);
        self
    }

    /// Vault A LP account of the pool
    #[inline(always)]
    pub fn a_vault_lp(&mut self, a_vault_lp: solana_pubkey::Pubkey) -> &mut Self {
        self.a_vault_lp = Some(a_vault_lp);
        self
    }

    #[inline(always)]
    pub fn mint_metadata(&mut self, mint_metadata: solana_pubkey::Pubkey) -> &mut Self {
        self.mint_metadata = Some(mint_metadata);
        self
    }

    #[inline(always)]
    pub fn metadata_program(&mut self, metadata_program: solana_pubkey::Pubkey) -> &mut Self {
        self.metadata_program = Some(metadata_program);
        self
    }

    /// `[optional account, default to '11111111111111111111111111111111']`
    /// System program.
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }

    /// Payer
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }

    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(&mut self, account: solana_instruction::AccountMeta) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }

    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }

    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_instruction::Instruction {
        let accounts = CreateMintMetadata {
            pool: self.pool.expect("pool is not set"),
            lp_mint: self.lp_mint.expect("lp_mint is not set"),
            a_vault_lp: self.a_vault_lp.expect("a_vault_lp is not set"),
            mint_metadata: self.mint_metadata.expect("mint_metadata is not set"),
            metadata_program: self.metadata_program.expect("metadata_program is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_pubkey::pubkey!("11111111111111111111111111111111")),
            payer: self.payer.expect("payer is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `create_mint_metadata` CPI accounts.
pub struct CreateMintMetadataCpiAccounts<'a, 'b> {
    /// Pool account
    pub pool: &'b solana_account_info::AccountInfo<'a>,
    /// LP mint account of the pool
    pub lp_mint: &'b solana_account_info::AccountInfo<'a>,
    /// Vault A LP account of the pool
    pub a_vault_lp: &'b solana_account_info::AccountInfo<'a>,

    pub mint_metadata: &'b solana_account_info::AccountInfo<'a>,

    pub metadata_program: &'b solana_account_info::AccountInfo<'a>,
    /// System program.
    pub system_program: &'b solana_account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'b solana_account_info::AccountInfo<'a>,
}

/// `create_mint_metadata` CPI instruction.
pub struct CreateMintMetadataCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_account_info::AccountInfo<'a>,
    /// Pool account
    pub pool: &'b solana_account_info::AccountInfo<'a>,
    /// LP mint account of the pool
    pub lp_mint: &'b solana_account_info::AccountInfo<'a>,
    /// Vault A LP account of the pool
    pub a_vault_lp: &'b solana_account_info::AccountInfo<'a>,

    pub mint_metadata: &'b solana_account_info::AccountInfo<'a>,

    pub metadata_program: &'b solana_account_info::AccountInfo<'a>,
    /// System program.
    pub system_program: &'b solana_account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'b solana_account_info::AccountInfo<'a>,
}

impl<'a, 'b> CreateMintMetadataCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: CreateMintMetadataCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            pool: accounts.pool,
            lp_mint: accounts.lp_mint,
            a_vault_lp: accounts.a_vault_lp,
            mint_metadata: accounts.mint_metadata,
            metadata_program: accounts.metadata_program,
            system_program: accounts.system_program,
            payer: accounts.payer,
        }
    }

    #[inline(always)]
    pub fn invoke(&self) -> solana_program_entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }

    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(&'b solana_account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program_entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }

    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program_entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(&'b solana_account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program_entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.pool.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.lp_mint.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.a_vault_lp.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.mint_metadata.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.metadata_program.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(*self.payer.key, true));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = borsh::to_vec(&CreateMintMetadataInstructionData::new()).unwrap();

        let instruction = solana_instruction::Instruction {
            program_id: crate::AMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(8 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.pool.clone());
        account_infos.push(self.lp_mint.clone());
        account_infos.push(self.a_vault_lp.clone());
        account_infos.push(self.mint_metadata.clone());
        account_infos.push(self.metadata_program.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.payer.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_cpi::invoke(&instruction, &account_infos)
        } else {
            solana_cpi::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `CreateMintMetadata` via CPI.
///
/// ### Accounts:
///
///   0. `[]` pool
///   1. `[]` lp_mint
///   2. `[]` a_vault_lp
///   3. `[writable]` mint_metadata
///   4. `[]` metadata_program
///   5. `[]` system_program
///   6. `[writable, signer]` payer
#[derive(Clone, Debug)]
pub struct CreateMintMetadataCpiBuilder<'a, 'b> {
    instruction: Box<CreateMintMetadataCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateMintMetadataCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateMintMetadataCpiBuilderInstruction {
            __program: program,
            pool: None,
            lp_mint: None,
            a_vault_lp: None,
            mint_metadata: None,
            metadata_program: None,
            system_program: None,
            payer: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    /// Pool account
    #[inline(always)]
    pub fn pool(&mut self, pool: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.pool = Some(pool);
        self
    }

    /// LP mint account of the pool
    #[inline(always)]
    pub fn lp_mint(&mut self, lp_mint: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.lp_mint = Some(lp_mint);
        self
    }

    /// Vault A LP account of the pool
    #[inline(always)]
    pub fn a_vault_lp(
        &mut self,
        a_vault_lp: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.a_vault_lp = Some(a_vault_lp);
        self
    }

    #[inline(always)]
    pub fn mint_metadata(
        &mut self,
        mint_metadata: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.mint_metadata = Some(mint_metadata);
        self
    }

    #[inline(always)]
    pub fn metadata_program(
        &mut self,
        metadata_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.metadata_program = Some(metadata_program);
        self
    }

    /// System program.
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }

    /// Payer
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }

    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_account_info::AccountInfo<'a>,
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
        accounts: &[(&'b solana_account_info::AccountInfo<'a>, bool, bool)],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }

    #[inline(always)]
    pub fn invoke(&self) -> solana_program_entrypoint::ProgramResult { self.invoke_signed(&[]) }

    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program_entrypoint::ProgramResult {
        let instruction = CreateMintMetadataCpi {
            __program: self.instruction.__program,

            pool: self.instruction.pool.expect("pool is not set"),

            lp_mint: self.instruction.lp_mint.expect("lp_mint is not set"),

            a_vault_lp: self.instruction.a_vault_lp.expect("a_vault_lp is not set"),

            mint_metadata: self
                .instruction
                .mint_metadata
                .expect("mint_metadata is not set"),

            metadata_program: self
                .instruction
                .metadata_program
                .expect("metadata_program is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            payer: self.instruction.payer.expect("payer is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CreateMintMetadataCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    pool: Option<&'b solana_account_info::AccountInfo<'a>>,
    lp_mint: Option<&'b solana_account_info::AccountInfo<'a>>,
    a_vault_lp: Option<&'b solana_account_info::AccountInfo<'a>>,
    mint_metadata: Option<&'b solana_account_info::AccountInfo<'a>>,
    metadata_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
