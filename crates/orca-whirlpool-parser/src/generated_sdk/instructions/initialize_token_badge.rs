//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct InitializeTokenBadge {
    pub whirlpools_config: solana_program::pubkey::Pubkey,

    pub whirlpools_config_extension: solana_program::pubkey::Pubkey,

    pub token_badge_authority: solana_program::pubkey::Pubkey,

    pub token_mint: solana_program::pubkey::Pubkey,

    pub token_badge: solana_program::pubkey::Pubkey,

    pub funder: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
}

impl InitializeTokenBadge {
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.whirlpools_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.whirlpools_config_extension,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_badge_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_badge,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.funder,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&InitializeTokenBadgeInstructionData::new()).unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeTokenBadgeInstructionData {
    discriminator: [u8; 8],
}

impl InitializeTokenBadgeInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [253, 77, 205, 95, 27, 224, 89, 223],
        }
    }
}

impl Default for InitializeTokenBadgeInstructionData {
    fn default() -> Self { Self::new() }
}

/// Instruction builder for `InitializeTokenBadge`.
///
/// ### Accounts:
///
///   0. `[]` whirlpools_config
///   1. `[]` whirlpools_config_extension
///   2. `[signer]` token_badge_authority
///   3. `[]` token_mint
///   4. `[writable]` token_badge
///   5. `[writable, signer]` funder
///   6. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitializeTokenBadgeBuilder {
    whirlpools_config: Option<solana_program::pubkey::Pubkey>,
    whirlpools_config_extension: Option<solana_program::pubkey::Pubkey>,
    token_badge_authority: Option<solana_program::pubkey::Pubkey>,
    token_mint: Option<solana_program::pubkey::Pubkey>,
    token_badge: Option<solana_program::pubkey::Pubkey>,
    funder: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeTokenBadgeBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn whirlpools_config(
        &mut self,
        whirlpools_config: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.whirlpools_config = Some(whirlpools_config);
        self
    }

    #[inline(always)]
    pub fn whirlpools_config_extension(
        &mut self,
        whirlpools_config_extension: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.whirlpools_config_extension = Some(whirlpools_config_extension);
        self
    }

    #[inline(always)]
    pub fn token_badge_authority(
        &mut self,
        token_badge_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_badge_authority = Some(token_badge_authority);
        self
    }

    #[inline(always)]
    pub fn token_mint(&mut self, token_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_mint = Some(token_mint);
        self
    }

    #[inline(always)]
    pub fn token_badge(&mut self, token_badge: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_badge = Some(token_badge);
        self
    }

    #[inline(always)]
    pub fn funder(&mut self, funder: solana_program::pubkey::Pubkey) -> &mut Self {
        self.funder = Some(funder);
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
        let accounts = InitializeTokenBadge {
            whirlpools_config: self
                .whirlpools_config
                .expect("whirlpools_config is not set"),
            whirlpools_config_extension: self
                .whirlpools_config_extension
                .expect("whirlpools_config_extension is not set"),
            token_badge_authority: self
                .token_badge_authority
                .expect("token_badge_authority is not set"),
            token_mint: self.token_mint.expect("token_mint is not set"),
            token_badge: self.token_badge.expect("token_badge is not set"),
            funder: self.funder.expect("funder is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `initialize_token_badge` CPI accounts.
pub struct InitializeTokenBadgeCpiAccounts<'a, 'b> {
    pub whirlpools_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpools_config_extension: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_badge_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_badge: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_token_badge` CPI instruction.
pub struct InitializeTokenBadgeCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpools_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpools_config_extension: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_badge_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_badge: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> InitializeTokenBadgeCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeTokenBadgeCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            whirlpools_config: accounts.whirlpools_config,
            whirlpools_config_extension: accounts.whirlpools_config_extension,
            token_badge_authority: accounts.token_badge_authority,
            token_mint: accounts.token_mint,
            token_badge: accounts.token_badge,
            funder: accounts.funder,
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
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.whirlpools_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.whirlpools_config_extension.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_badge_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_badge.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.funder.key,
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
        let data = borsh::to_vec(&InitializeTokenBadgeInstructionData::new()).unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(8 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.whirlpools_config.clone());
        account_infos.push(self.whirlpools_config_extension.clone());
        account_infos.push(self.token_badge_authority.clone());
        account_infos.push(self.token_mint.clone());
        account_infos.push(self.token_badge.clone());
        account_infos.push(self.funder.clone());
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

/// Instruction builder for `InitializeTokenBadge` via CPI.
///
/// ### Accounts:
///
///   0. `[]` whirlpools_config
///   1. `[]` whirlpools_config_extension
///   2. `[signer]` token_badge_authority
///   3. `[]` token_mint
///   4. `[writable]` token_badge
///   5. `[writable, signer]` funder
///   6. `[]` system_program
#[derive(Clone, Debug)]
pub struct InitializeTokenBadgeCpiBuilder<'a, 'b> {
    instruction: Box<InitializeTokenBadgeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeTokenBadgeCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeTokenBadgeCpiBuilderInstruction {
            __program: program,
            whirlpools_config: None,
            whirlpools_config_extension: None,
            token_badge_authority: None,
            token_mint: None,
            token_badge: None,
            funder: None,
            system_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn whirlpools_config(
        &mut self,
        whirlpools_config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.whirlpools_config = Some(whirlpools_config);
        self
    }

    #[inline(always)]
    pub fn whirlpools_config_extension(
        &mut self,
        whirlpools_config_extension: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.whirlpools_config_extension = Some(whirlpools_config_extension);
        self
    }

    #[inline(always)]
    pub fn token_badge_authority(
        &mut self,
        token_badge_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_badge_authority = Some(token_badge_authority);
        self
    }

    #[inline(always)]
    pub fn token_mint(
        &mut self,
        token_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_mint = Some(token_mint);
        self
    }

    #[inline(always)]
    pub fn token_badge(
        &mut self,
        token_badge: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_badge = Some(token_badge);
        self
    }

    #[inline(always)]
    pub fn funder(
        &mut self,
        funder: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.funder = Some(funder);
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
        let instruction = InitializeTokenBadgeCpi {
            __program: self.instruction.__program,

            whirlpools_config: self
                .instruction
                .whirlpools_config
                .expect("whirlpools_config is not set"),

            whirlpools_config_extension: self
                .instruction
                .whirlpools_config_extension
                .expect("whirlpools_config_extension is not set"),

            token_badge_authority: self
                .instruction
                .token_badge_authority
                .expect("token_badge_authority is not set"),

            token_mint: self.instruction.token_mint.expect("token_mint is not set"),

            token_badge: self
                .instruction
                .token_badge
                .expect("token_badge is not set"),

            funder: self.instruction.funder.expect("funder is not set"),

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
struct InitializeTokenBadgeCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    whirlpools_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    whirlpools_config_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_badge_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_badge: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    funder: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
