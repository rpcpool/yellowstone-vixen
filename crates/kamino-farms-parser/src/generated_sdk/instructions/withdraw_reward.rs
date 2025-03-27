//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct WithdrawReward {
    pub farm_admin: solana_program::pubkey::Pubkey,

    pub farm_state: solana_program::pubkey::Pubkey,

    pub reward_mint: solana_program::pubkey::Pubkey,

    pub reward_vault: solana_program::pubkey::Pubkey,

    pub farm_vaults_authority: solana_program::pubkey::Pubkey,

    pub admin_reward_token_ata: solana_program::pubkey::Pubkey,

    pub scope_prices: Option<solana_program::pubkey::Pubkey>,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl WithdrawReward {
    pub fn instruction(
        &self,
        args: WithdrawRewardInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: WithdrawRewardInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.farm_admin,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.farm_state,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reward_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.reward_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.farm_vaults_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.admin_reward_token_ata,
            false,
        ));
        if let Some(scope_prices) = self.scope_prices {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                scope_prices,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::FARMS_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&WithdrawRewardInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::FARMS_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawRewardInstructionData {
    discriminator: [u8; 8],
}

impl WithdrawRewardInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [191, 187, 176, 137, 9, 25, 187, 244],
        }
    }
}

impl Default for WithdrawRewardInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawRewardInstructionArgs {
    pub amount: u64,
    pub reward_index: u64,
}

/// Instruction builder for `WithdrawReward`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` farm_admin
///   1. `[writable]` farm_state
///   2. `[]` reward_mint
///   3. `[writable]` reward_vault
///   4. `[]` farm_vaults_authority
///   5. `[writable]` admin_reward_token_ata
///   6. `[optional]` scope_prices
///   7. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct WithdrawRewardBuilder {
    farm_admin: Option<solana_program::pubkey::Pubkey>,
    farm_state: Option<solana_program::pubkey::Pubkey>,
    reward_mint: Option<solana_program::pubkey::Pubkey>,
    reward_vault: Option<solana_program::pubkey::Pubkey>,
    farm_vaults_authority: Option<solana_program::pubkey::Pubkey>,
    admin_reward_token_ata: Option<solana_program::pubkey::Pubkey>,
    scope_prices: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    amount: Option<u64>,
    reward_index: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl WithdrawRewardBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn farm_admin(&mut self, farm_admin: solana_program::pubkey::Pubkey) -> &mut Self {
        self.farm_admin = Some(farm_admin);
        self
    }

    #[inline(always)]
    pub fn farm_state(&mut self, farm_state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.farm_state = Some(farm_state);
        self
    }

    #[inline(always)]
    pub fn reward_mint(&mut self, reward_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reward_mint = Some(reward_mint);
        self
    }

    #[inline(always)]
    pub fn reward_vault(&mut self, reward_vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reward_vault = Some(reward_vault);
        self
    }

    #[inline(always)]
    pub fn farm_vaults_authority(
        &mut self,
        farm_vaults_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.farm_vaults_authority = Some(farm_vaults_authority);
        self
    }

    #[inline(always)]
    pub fn admin_reward_token_ata(
        &mut self,
        admin_reward_token_ata: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.admin_reward_token_ata = Some(admin_reward_token_ata);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn scope_prices(
        &mut self,
        scope_prices: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.scope_prices = scope_prices;
        self
    }

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.amount = Some(amount);
        self
    }

    #[inline(always)]
    pub fn reward_index(&mut self, reward_index: u64) -> &mut Self {
        self.reward_index = Some(reward_index);
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
        let accounts = WithdrawReward {
            farm_admin: self.farm_admin.expect("farm_admin is not set"),
            farm_state: self.farm_state.expect("farm_state is not set"),
            reward_mint: self.reward_mint.expect("reward_mint is not set"),
            reward_vault: self.reward_vault.expect("reward_vault is not set"),
            farm_vaults_authority: self
                .farm_vaults_authority
                .expect("farm_vaults_authority is not set"),
            admin_reward_token_ata: self
                .admin_reward_token_ata
                .expect("admin_reward_token_ata is not set"),
            scope_prices: self.scope_prices,
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };
        let args = WithdrawRewardInstructionArgs {
            amount: self.amount.clone().expect("amount is not set"),
            reward_index: self.reward_index.clone().expect("reward_index is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `withdraw_reward` CPI accounts.
pub struct WithdrawRewardCpiAccounts<'a, 'b> {
    pub farm_admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub farm_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub farm_vaults_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin_reward_token_ata: &'b solana_program::account_info::AccountInfo<'a>,

    pub scope_prices: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `withdraw_reward` CPI instruction.
pub struct WithdrawRewardCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub farm_admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub farm_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub farm_vaults_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin_reward_token_ata: &'b solana_program::account_info::AccountInfo<'a>,

    pub scope_prices: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: WithdrawRewardInstructionArgs,
}

impl<'a, 'b> WithdrawRewardCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: WithdrawRewardCpiAccounts<'a, 'b>,
        args: WithdrawRewardInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            farm_admin: accounts.farm_admin,
            farm_state: accounts.farm_state,
            reward_mint: accounts.reward_mint,
            reward_vault: accounts.reward_vault,
            farm_vaults_authority: accounts.farm_vaults_authority,
            admin_reward_token_ata: accounts.admin_reward_token_ata,
            scope_prices: accounts.scope_prices,
            token_program: accounts.token_program,
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
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.farm_admin.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.farm_state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reward_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.farm_vaults_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.admin_reward_token_ata.key,
            false,
        ));
        if let Some(scope_prices) = self.scope_prices {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *scope_prices.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::FARMS_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&WithdrawRewardInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::FARMS_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.farm_admin.clone());
        account_infos.push(self.farm_state.clone());
        account_infos.push(self.reward_mint.clone());
        account_infos.push(self.reward_vault.clone());
        account_infos.push(self.farm_vaults_authority.clone());
        account_infos.push(self.admin_reward_token_ata.clone());
        if let Some(scope_prices) = self.scope_prices {
            account_infos.push(scope_prices.clone());
        }
        account_infos.push(self.token_program.clone());
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

/// Instruction builder for `WithdrawReward` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` farm_admin
///   1. `[writable]` farm_state
///   2. `[]` reward_mint
///   3. `[writable]` reward_vault
///   4. `[]` farm_vaults_authority
///   5. `[writable]` admin_reward_token_ata
///   6. `[optional]` scope_prices
///   7. `[]` token_program
#[derive(Clone, Debug)]
pub struct WithdrawRewardCpiBuilder<'a, 'b> {
    instruction: Box<WithdrawRewardCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> WithdrawRewardCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(WithdrawRewardCpiBuilderInstruction {
            __program: program,
            farm_admin: None,
            farm_state: None,
            reward_mint: None,
            reward_vault: None,
            farm_vaults_authority: None,
            admin_reward_token_ata: None,
            scope_prices: None,
            token_program: None,
            amount: None,
            reward_index: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn farm_admin(
        &mut self,
        farm_admin: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.farm_admin = Some(farm_admin);
        self
    }

    #[inline(always)]
    pub fn farm_state(
        &mut self,
        farm_state: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.farm_state = Some(farm_state);
        self
    }

    #[inline(always)]
    pub fn reward_mint(
        &mut self,
        reward_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_mint = Some(reward_mint);
        self
    }

    #[inline(always)]
    pub fn reward_vault(
        &mut self,
        reward_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_vault = Some(reward_vault);
        self
    }

    #[inline(always)]
    pub fn farm_vaults_authority(
        &mut self,
        farm_vaults_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.farm_vaults_authority = Some(farm_vaults_authority);
        self
    }

    #[inline(always)]
    pub fn admin_reward_token_ata(
        &mut self,
        admin_reward_token_ata: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.admin_reward_token_ata = Some(admin_reward_token_ata);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn scope_prices(
        &mut self,
        scope_prices: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.scope_prices = scope_prices;
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
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.instruction.amount = Some(amount);
        self
    }

    #[inline(always)]
    pub fn reward_index(&mut self, reward_index: u64) -> &mut Self {
        self.instruction.reward_index = Some(reward_index);
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
        let args = WithdrawRewardInstructionArgs {
            amount: self.instruction.amount.clone().expect("amount is not set"),
            reward_index: self
                .instruction
                .reward_index
                .clone()
                .expect("reward_index is not set"),
        };
        let instruction = WithdrawRewardCpi {
            __program: self.instruction.__program,

            farm_admin: self.instruction.farm_admin.expect("farm_admin is not set"),

            farm_state: self.instruction.farm_state.expect("farm_state is not set"),

            reward_mint: self
                .instruction
                .reward_mint
                .expect("reward_mint is not set"),

            reward_vault: self
                .instruction
                .reward_vault
                .expect("reward_vault is not set"),

            farm_vaults_authority: self
                .instruction
                .farm_vaults_authority
                .expect("farm_vaults_authority is not set"),

            admin_reward_token_ata: self
                .instruction
                .admin_reward_token_ata
                .expect("admin_reward_token_ata is not set"),

            scope_prices: self.instruction.scope_prices,

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct WithdrawRewardCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    farm_admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    farm_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    farm_vaults_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    admin_reward_token_ata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    scope_prices: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    amount: Option<u64>,
    reward_index: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
