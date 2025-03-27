//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct RebalanceInsuranceVault {
    pub state: solana_program::pubkey::Pubkey,

    pub zeta_vault: solana_program::pubkey::Pubkey,

    pub insurance_vault: solana_program::pubkey::Pubkey,

    pub treasury_wallet: solana_program::pubkey::Pubkey,

    pub socialized_loss_account: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl RebalanceInsuranceVault {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.state, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.zeta_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.insurance_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.treasury_wallet,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.socialized_loss_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&RebalanceInsuranceVaultInstructionData::new()).unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::ZETA_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RebalanceInsuranceVaultInstructionData {
    discriminator: [u8; 8],
}

impl RebalanceInsuranceVaultInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [11, 196, 66, 235, 59, 237, 223, 111],
        }
    }
}

impl Default for RebalanceInsuranceVaultInstructionData {
    fn default() -> Self { Self::new() }
}

/// Instruction builder for `RebalanceInsuranceVault`.
///
/// ### Accounts:
///
///   0. `[]` state
///   1. `[writable]` zeta_vault
///   2. `[writable]` insurance_vault
///   3. `[writable]` treasury_wallet
///   4. `[writable]` socialized_loss_account
///   5. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct RebalanceInsuranceVaultBuilder {
    state: Option<solana_program::pubkey::Pubkey>,
    zeta_vault: Option<solana_program::pubkey::Pubkey>,
    insurance_vault: Option<solana_program::pubkey::Pubkey>,
    treasury_wallet: Option<solana_program::pubkey::Pubkey>,
    socialized_loss_account: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl RebalanceInsuranceVaultBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn state(&mut self, state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn zeta_vault(&mut self, zeta_vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.zeta_vault = Some(zeta_vault);
        self
    }

    #[inline(always)]
    pub fn insurance_vault(
        &mut self,
        insurance_vault: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.insurance_vault = Some(insurance_vault);
        self
    }

    #[inline(always)]
    pub fn treasury_wallet(
        &mut self,
        treasury_wallet: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.treasury_wallet = Some(treasury_wallet);
        self
    }

    #[inline(always)]
    pub fn socialized_loss_account(
        &mut self,
        socialized_loss_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.socialized_loss_account = Some(socialized_loss_account);
        self
    }

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
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
        let accounts = RebalanceInsuranceVault {
            state: self.state.expect("state is not set"),
            zeta_vault: self.zeta_vault.expect("zeta_vault is not set"),
            insurance_vault: self.insurance_vault.expect("insurance_vault is not set"),
            treasury_wallet: self.treasury_wallet.expect("treasury_wallet is not set"),
            socialized_loss_account: self
                .socialized_loss_account
                .expect("socialized_loss_account is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `rebalance_insurance_vault` CPI accounts.
pub struct RebalanceInsuranceVaultCpiAccounts<'a, 'b> {
    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub zeta_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub insurance_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub treasury_wallet: &'b solana_program::account_info::AccountInfo<'a>,

    pub socialized_loss_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `rebalance_insurance_vault` CPI instruction.
pub struct RebalanceInsuranceVaultCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub zeta_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub insurance_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub treasury_wallet: &'b solana_program::account_info::AccountInfo<'a>,

    pub socialized_loss_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> RebalanceInsuranceVaultCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: RebalanceInsuranceVaultCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            state: accounts.state,
            zeta_vault: accounts.zeta_vault,
            insurance_vault: accounts.insurance_vault,
            treasury_wallet: accounts.treasury_wallet,
            socialized_loss_account: accounts.socialized_loss_account,
            token_program: accounts.token_program,
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
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.zeta_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.insurance_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.treasury_wallet.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.socialized_loss_account.key,
            false,
        ));
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
        let data = borsh::to_vec(&RebalanceInsuranceVaultInstructionData::new()).unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::ZETA_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.state.clone());
        account_infos.push(self.zeta_vault.clone());
        account_infos.push(self.insurance_vault.clone());
        account_infos.push(self.treasury_wallet.clone());
        account_infos.push(self.socialized_loss_account.clone());
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

/// Instruction builder for `RebalanceInsuranceVault` via CPI.
///
/// ### Accounts:
///
///   0. `[]` state
///   1. `[writable]` zeta_vault
///   2. `[writable]` insurance_vault
///   3. `[writable]` treasury_wallet
///   4. `[writable]` socialized_loss_account
///   5. `[]` token_program
#[derive(Clone, Debug)]
pub struct RebalanceInsuranceVaultCpiBuilder<'a, 'b> {
    instruction: Box<RebalanceInsuranceVaultCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> RebalanceInsuranceVaultCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(RebalanceInsuranceVaultCpiBuilderInstruction {
            __program: program,
            state: None,
            zeta_vault: None,
            insurance_vault: None,
            treasury_wallet: None,
            socialized_loss_account: None,
            token_program: None,
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
    pub fn zeta_vault(
        &mut self,
        zeta_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.zeta_vault = Some(zeta_vault);
        self
    }

    #[inline(always)]
    pub fn insurance_vault(
        &mut self,
        insurance_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.insurance_vault = Some(insurance_vault);
        self
    }

    #[inline(always)]
    pub fn treasury_wallet(
        &mut self,
        treasury_wallet: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.treasury_wallet = Some(treasury_wallet);
        self
    }

    #[inline(always)]
    pub fn socialized_loss_account(
        &mut self,
        socialized_loss_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.socialized_loss_account = Some(socialized_loss_account);
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
        let instruction = RebalanceInsuranceVaultCpi {
            __program: self.instruction.__program,

            state: self.instruction.state.expect("state is not set"),

            zeta_vault: self.instruction.zeta_vault.expect("zeta_vault is not set"),

            insurance_vault: self
                .instruction
                .insurance_vault
                .expect("insurance_vault is not set"),

            treasury_wallet: self
                .instruction
                .treasury_wallet
                .expect("treasury_wallet is not set"),

            socialized_loss_account: self
                .instruction
                .socialized_loss_account
                .expect("socialized_loss_account is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct RebalanceInsuranceVaultCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    zeta_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    insurance_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    treasury_wallet: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    socialized_loss_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
