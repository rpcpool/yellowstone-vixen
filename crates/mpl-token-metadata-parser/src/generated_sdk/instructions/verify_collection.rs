//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct VerifyCollection {
    /// Metadata account
    pub metadata: solana_program::pubkey::Pubkey,
    /// Collection Update authority
    pub collection_authority: solana_program::pubkey::Pubkey,
    /// payer
    pub payer: solana_program::pubkey::Pubkey,
    /// Mint of the Collection
    pub collection_mint: solana_program::pubkey::Pubkey,
    /// Metadata Account of the Collection
    pub collection: solana_program::pubkey::Pubkey,
    /// MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: solana_program::pubkey::Pubkey,
    /// Collection Authority Record PDA
    pub collection_authority_record: Option<solana_program::pubkey::Pubkey>,
}

impl VerifyCollection {
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
            self.metadata,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.collection_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collection_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collection,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collection_master_edition_account,
            false,
        ));
        if let Some(collection_authority_record) = self.collection_authority_record {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                collection_authority_record,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&VerifyCollectionInstructionData::new()).unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::TOKEN_METADATA_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerifyCollectionInstructionData {
    discriminator: u8,
}

impl VerifyCollectionInstructionData {
    pub fn new() -> Self { Self { discriminator: 18 } }
}

impl Default for VerifyCollectionInstructionData {
    fn default() -> Self { Self::new() }
}

/// Instruction builder for `VerifyCollection`.
///
/// ### Accounts:
///
///   0. `[writable]` metadata
///   1. `[writable, signer]` collection_authority
///   2. `[writable, signer]` payer
///   3. `[]` collection_mint
///   4. `[]` collection
///   5. `[]` collection_master_edition_account
///   6. `[optional]` collection_authority_record
#[derive(Clone, Debug, Default)]
pub struct VerifyCollectionBuilder {
    metadata: Option<solana_program::pubkey::Pubkey>,
    collection_authority: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    collection_mint: Option<solana_program::pubkey::Pubkey>,
    collection: Option<solana_program::pubkey::Pubkey>,
    collection_master_edition_account: Option<solana_program::pubkey::Pubkey>,
    collection_authority_record: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl VerifyCollectionBuilder {
    pub fn new() -> Self { Self::default() }

    /// Metadata account
    #[inline(always)]
    pub fn metadata(&mut self, metadata: solana_program::pubkey::Pubkey) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }

    /// Collection Update authority
    #[inline(always)]
    pub fn collection_authority(
        &mut self,
        collection_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.collection_authority = Some(collection_authority);
        self
    }

    /// payer
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }

    /// Mint of the Collection
    #[inline(always)]
    pub fn collection_mint(
        &mut self,
        collection_mint: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.collection_mint = Some(collection_mint);
        self
    }

    /// Metadata Account of the Collection
    #[inline(always)]
    pub fn collection(&mut self, collection: solana_program::pubkey::Pubkey) -> &mut Self {
        self.collection = Some(collection);
        self
    }

    /// MasterEdition2 Account of the Collection Token
    #[inline(always)]
    pub fn collection_master_edition_account(
        &mut self,
        collection_master_edition_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.collection_master_edition_account = Some(collection_master_edition_account);
        self
    }

    /// `[optional account]`
    /// Collection Authority Record PDA
    #[inline(always)]
    pub fn collection_authority_record(
        &mut self,
        collection_authority_record: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.collection_authority_record = collection_authority_record;
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
        let accounts = VerifyCollection {
            metadata: self.metadata.expect("metadata is not set"),
            collection_authority: self
                .collection_authority
                .expect("collection_authority is not set"),
            payer: self.payer.expect("payer is not set"),
            collection_mint: self.collection_mint.expect("collection_mint is not set"),
            collection: self.collection.expect("collection is not set"),
            collection_master_edition_account: self
                .collection_master_edition_account
                .expect("collection_master_edition_account is not set"),
            collection_authority_record: self.collection_authority_record,
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `verify_collection` CPI accounts.
pub struct VerifyCollectionCpiAccounts<'a, 'b> {
    /// Metadata account
    pub metadata: &'b solana_program::account_info::AccountInfo<'a>,
    /// Collection Update authority
    pub collection_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// payer
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Mint of the Collection
    pub collection_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// Metadata Account of the Collection
    pub collection: &'b solana_program::account_info::AccountInfo<'a>,
    /// MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Collection Authority Record PDA
    pub collection_authority_record: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `verify_collection` CPI instruction.
pub struct VerifyCollectionCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Metadata account
    pub metadata: &'b solana_program::account_info::AccountInfo<'a>,
    /// Collection Update authority
    pub collection_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// payer
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Mint of the Collection
    pub collection_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// Metadata Account of the Collection
    pub collection: &'b solana_program::account_info::AccountInfo<'a>,
    /// MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Collection Authority Record PDA
    pub collection_authority_record: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

impl<'a, 'b> VerifyCollectionCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: VerifyCollectionCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            metadata: accounts.metadata,
            collection_authority: accounts.collection_authority,
            payer: accounts.payer,
            collection_mint: accounts.collection_mint,
            collection: accounts.collection,
            collection_master_edition_account: accounts.collection_master_edition_account,
            collection_authority_record: accounts.collection_authority_record,
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
            *self.metadata.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.collection_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collection_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collection.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collection_master_edition_account.key,
            false,
        ));
        if let Some(collection_authority_record) = self.collection_authority_record {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *collection_authority_record.key,
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
        let data = borsh::to_vec(&VerifyCollectionInstructionData::new()).unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::TOKEN_METADATA_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(8 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.metadata.clone());
        account_infos.push(self.collection_authority.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.collection_mint.clone());
        account_infos.push(self.collection.clone());
        account_infos.push(self.collection_master_edition_account.clone());
        if let Some(collection_authority_record) = self.collection_authority_record {
            account_infos.push(collection_authority_record.clone());
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

/// Instruction builder for `VerifyCollection` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` metadata
///   1. `[writable, signer]` collection_authority
///   2. `[writable, signer]` payer
///   3. `[]` collection_mint
///   4. `[]` collection
///   5. `[]` collection_master_edition_account
///   6. `[optional]` collection_authority_record
#[derive(Clone, Debug)]
pub struct VerifyCollectionCpiBuilder<'a, 'b> {
    instruction: Box<VerifyCollectionCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> VerifyCollectionCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(VerifyCollectionCpiBuilderInstruction {
            __program: program,
            metadata: None,
            collection_authority: None,
            payer: None,
            collection_mint: None,
            collection: None,
            collection_master_edition_account: None,
            collection_authority_record: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    /// Metadata account
    #[inline(always)]
    pub fn metadata(
        &mut self,
        metadata: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.metadata = Some(metadata);
        self
    }

    /// Collection Update authority
    #[inline(always)]
    pub fn collection_authority(
        &mut self,
        collection_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collection_authority = Some(collection_authority);
        self
    }

    /// payer
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }

    /// Mint of the Collection
    #[inline(always)]
    pub fn collection_mint(
        &mut self,
        collection_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collection_mint = Some(collection_mint);
        self
    }

    /// Metadata Account of the Collection
    #[inline(always)]
    pub fn collection(
        &mut self,
        collection: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collection = Some(collection);
        self
    }

    /// MasterEdition2 Account of the Collection Token
    #[inline(always)]
    pub fn collection_master_edition_account(
        &mut self,
        collection_master_edition_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collection_master_edition_account =
            Some(collection_master_edition_account);
        self
    }

    /// `[optional account]`
    /// Collection Authority Record PDA
    #[inline(always)]
    pub fn collection_authority_record(
        &mut self,
        collection_authority_record: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.collection_authority_record = collection_authority_record;
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
        let instruction = VerifyCollectionCpi {
            __program: self.instruction.__program,

            metadata: self.instruction.metadata.expect("metadata is not set"),

            collection_authority: self
                .instruction
                .collection_authority
                .expect("collection_authority is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            collection_mint: self
                .instruction
                .collection_mint
                .expect("collection_mint is not set"),

            collection: self.instruction.collection.expect("collection is not set"),

            collection_master_edition_account: self
                .instruction
                .collection_master_edition_account
                .expect("collection_master_edition_account is not set"),

            collection_authority_record: self.instruction.collection_authority_record,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct VerifyCollectionCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    metadata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collection_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collection_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collection: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collection_master_edition_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collection_authority_record: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
