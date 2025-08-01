//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct CreateToken {
    pub config: solana_pubkey::Pubkey,

    pub metadata: solana_pubkey::Pubkey,

    pub mint: solana_pubkey::Pubkey,

    pub payer: solana_pubkey::Pubkey,

    pub rent: solana_pubkey::Pubkey,

    pub system_program: solana_pubkey::Pubkey,

    pub token_program: solana_pubkey::Pubkey,

    pub token_metadata_program: solana_pubkey::Pubkey,
}

impl CreateToken {
    pub fn instruction(&self, args: CreateTokenInstructionArgs) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateTokenInstructionArgs,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.config,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(self.metadata, false));
        accounts.push(solana_instruction::AccountMeta::new(self.mint, false));
        accounts.push(solana_instruction::AccountMeta::new(self.payer, true));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_metadata_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&CreateTokenInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_instruction::Instruction {
            program_id: crate::BOOP_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTokenInstructionData {
    discriminator: [u8; 8],
}

impl CreateTokenInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [84, 52, 204, 228, 24, 140, 234, 75],
        }
    }
}

impl Default for CreateTokenInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTokenInstructionArgs {
    pub salt: u64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

/// Instruction builder for `CreateToken`.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` metadata
///   2. `[writable]` mint
///   3. `[writable, signer]` payer
///   4. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   5. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   6. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   7. `[optional]` token_metadata_program (default to `metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s`)
#[derive(Clone, Debug, Default)]
pub struct CreateTokenBuilder {
    config: Option<solana_pubkey::Pubkey>,
    metadata: Option<solana_pubkey::Pubkey>,
    mint: Option<solana_pubkey::Pubkey>,
    payer: Option<solana_pubkey::Pubkey>,
    rent: Option<solana_pubkey::Pubkey>,
    system_program: Option<solana_pubkey::Pubkey>,
    token_program: Option<solana_pubkey::Pubkey>,
    token_metadata_program: Option<solana_pubkey::Pubkey>,
    salt: Option<u64>,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl CreateTokenBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn config(&mut self, config: solana_pubkey::Pubkey) -> &mut Self {
        self.config = Some(config);
        self
    }

    #[inline(always)]
    pub fn metadata(&mut self, metadata: solana_pubkey::Pubkey) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }

    #[inline(always)]
    pub fn mint(&mut self, mint: solana_pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }

    #[inline(always)]
    pub fn payer(&mut self, payer: solana_pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }

    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
        self
    }

    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }

    /// `[optional account, default to 'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s']`
    #[inline(always)]
    pub fn token_metadata_program(
        &mut self,
        token_metadata_program: solana_pubkey::Pubkey,
    ) -> &mut Self {
        self.token_metadata_program = Some(token_metadata_program);
        self
    }

    #[inline(always)]
    pub fn salt(&mut self, salt: u64) -> &mut Self {
        self.salt = Some(salt);
        self
    }

    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    #[inline(always)]
    pub fn symbol(&mut self, symbol: String) -> &mut Self {
        self.symbol = Some(symbol);
        self
    }

    #[inline(always)]
    pub fn uri(&mut self, uri: String) -> &mut Self {
        self.uri = Some(uri);
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
        let accounts =
            CreateToken {
                config: self.config.expect("config is not set"),
                metadata: self.metadata.expect("metadata is not set"),
                mint: self.mint.expect("mint is not set"),
                payer: self.payer.expect("payer is not set"),
                rent: self.rent.unwrap_or(solana_pubkey::pubkey!(
                    "SysvarRent111111111111111111111111111111111"
                )),
                system_program: self
                    .system_program
                    .unwrap_or(solana_pubkey::pubkey!("11111111111111111111111111111111")),
                token_program: self.token_program.unwrap_or(solana_pubkey::pubkey!(
                    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                )),
                token_metadata_program: self.token_metadata_program.unwrap_or(
                    solana_pubkey::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                ),
            };
        let args = CreateTokenInstructionArgs {
            salt: self.salt.clone().expect("salt is not set"),
            name: self.name.clone().expect("name is not set"),
            symbol: self.symbol.clone().expect("symbol is not set"),
            uri: self.uri.clone().expect("uri is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_token` CPI accounts.
pub struct CreateTokenCpiAccounts<'a, 'b> {
    pub config: &'b solana_account_info::AccountInfo<'a>,

    pub metadata: &'b solana_account_info::AccountInfo<'a>,

    pub mint: &'b solana_account_info::AccountInfo<'a>,

    pub payer: &'b solana_account_info::AccountInfo<'a>,

    pub rent: &'b solana_account_info::AccountInfo<'a>,

    pub system_program: &'b solana_account_info::AccountInfo<'a>,

    pub token_program: &'b solana_account_info::AccountInfo<'a>,

    pub token_metadata_program: &'b solana_account_info::AccountInfo<'a>,
}

/// `create_token` CPI instruction.
pub struct CreateTokenCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_account_info::AccountInfo<'a>,

    pub config: &'b solana_account_info::AccountInfo<'a>,

    pub metadata: &'b solana_account_info::AccountInfo<'a>,

    pub mint: &'b solana_account_info::AccountInfo<'a>,

    pub payer: &'b solana_account_info::AccountInfo<'a>,

    pub rent: &'b solana_account_info::AccountInfo<'a>,

    pub system_program: &'b solana_account_info::AccountInfo<'a>,

    pub token_program: &'b solana_account_info::AccountInfo<'a>,

    pub token_metadata_program: &'b solana_account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateTokenInstructionArgs,
}

impl<'a, 'b> CreateTokenCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: CreateTokenCpiAccounts<'a, 'b>,
        args: CreateTokenInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            config: accounts.config,
            metadata: accounts.metadata,
            mint: accounts.mint,
            payer: accounts.payer,
            rent: accounts.rent,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            token_metadata_program: accounts.token_metadata_program,
            __args: args,
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
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.config.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.metadata.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(*self.mint.key, false));
        accounts.push(solana_instruction::AccountMeta::new(*self.payer.key, true));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_metadata_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&CreateTokenInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_instruction::Instruction {
            program_id: crate::BOOP_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.config.clone());
        account_infos.push(self.metadata.clone());
        account_infos.push(self.mint.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.token_metadata_program.clone());
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

/// Instruction builder for `CreateToken` via CPI.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` metadata
///   2. `[writable]` mint
///   3. `[writable, signer]` payer
///   4. `[]` rent
///   5. `[]` system_program
///   6. `[]` token_program
///   7. `[]` token_metadata_program
#[derive(Clone, Debug)]
pub struct CreateTokenCpiBuilder<'a, 'b> {
    instruction: Box<CreateTokenCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateTokenCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateTokenCpiBuilderInstruction {
            __program: program,
            config: None,
            metadata: None,
            mint: None,
            payer: None,
            rent: None,
            system_program: None,
            token_program: None,
            token_metadata_program: None,
            salt: None,
            name: None,
            symbol: None,
            uri: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn config(&mut self, config: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.config = Some(config);
        self
    }

    #[inline(always)]
    pub fn metadata(&mut self, metadata: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.metadata = Some(metadata);
        self
    }

    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
        self
    }

    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }

    #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
        self
    }

    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }

    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn token_metadata_program(
        &mut self,
        token_metadata_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_metadata_program = Some(token_metadata_program);
        self
    }

    #[inline(always)]
    pub fn salt(&mut self, salt: u64) -> &mut Self {
        self.instruction.salt = Some(salt);
        self
    }

    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.instruction.name = Some(name);
        self
    }

    #[inline(always)]
    pub fn symbol(&mut self, symbol: String) -> &mut Self {
        self.instruction.symbol = Some(symbol);
        self
    }

    #[inline(always)]
    pub fn uri(&mut self, uri: String) -> &mut Self {
        self.instruction.uri = Some(uri);
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
        let args = CreateTokenInstructionArgs {
            salt: self.instruction.salt.clone().expect("salt is not set"),
            name: self.instruction.name.clone().expect("name is not set"),
            symbol: self.instruction.symbol.clone().expect("symbol is not set"),
            uri: self.instruction.uri.clone().expect("uri is not set"),
        };
        let instruction = CreateTokenCpi {
            __program: self.instruction.__program,

            config: self.instruction.config.expect("config is not set"),

            metadata: self.instruction.metadata.expect("metadata is not set"),

            mint: self.instruction.mint.expect("mint is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            rent: self.instruction.rent.expect("rent is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            token_metadata_program: self
                .instruction
                .token_metadata_program
                .expect("token_metadata_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CreateTokenCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    config: Option<&'b solana_account_info::AccountInfo<'a>>,
    metadata: Option<&'b solana_account_info::AccountInfo<'a>>,
    mint: Option<&'b solana_account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_metadata_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    salt: Option<u64>,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
