//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct SettleRevenueToInsuranceFund {
    pub state: solana_program::pubkey::Pubkey,

    pub spot_market: solana_program::pubkey::Pubkey,

    pub spot_market_vault: solana_program::pubkey::Pubkey,

    pub drift_signer: solana_program::pubkey::Pubkey,

    pub insurance_fund_vault: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl SettleRevenueToInsuranceFund {
    pub fn instruction(
        &self,
        args: SettleRevenueToInsuranceFundInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: SettleRevenueToInsuranceFundInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.state, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.spot_market,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.spot_market_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.drift_signer,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.insurance_fund_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&SettleRevenueToInsuranceFundInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::DRIFT_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SettleRevenueToInsuranceFundInstructionData {
    discriminator: [u8; 8],
}

impl SettleRevenueToInsuranceFundInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [200, 120, 93, 136, 69, 38, 199, 159],
        }
    }
}

impl Default for SettleRevenueToInsuranceFundInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SettleRevenueToInsuranceFundInstructionArgs {
    pub spot_market_index: u16,
}

/// Instruction builder for `SettleRevenueToInsuranceFund`.
///
/// ### Accounts:
///
///   0. `[]` state
///   1. `[writable]` spot_market
///   2. `[writable]` spot_market_vault
///   3. `[]` drift_signer
///   4. `[writable]` insurance_fund_vault
///   5. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct SettleRevenueToInsuranceFundBuilder {
    state: Option<solana_program::pubkey::Pubkey>,
    spot_market: Option<solana_program::pubkey::Pubkey>,
    spot_market_vault: Option<solana_program::pubkey::Pubkey>,
    drift_signer: Option<solana_program::pubkey::Pubkey>,
    insurance_fund_vault: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    spot_market_index: Option<u16>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl SettleRevenueToInsuranceFundBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn state(&mut self, state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn spot_market(&mut self, spot_market: solana_program::pubkey::Pubkey) -> &mut Self {
        self.spot_market = Some(spot_market);
        self
    }

    #[inline(always)]
    pub fn spot_market_vault(
        &mut self,
        spot_market_vault: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.spot_market_vault = Some(spot_market_vault);
        self
    }

    #[inline(always)]
    pub fn drift_signer(&mut self, drift_signer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.drift_signer = Some(drift_signer);
        self
    }

    #[inline(always)]
    pub fn insurance_fund_vault(
        &mut self,
        insurance_fund_vault: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.insurance_fund_vault = Some(insurance_fund_vault);
        self
    }

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn spot_market_index(&mut self, spot_market_index: u16) -> &mut Self {
        self.spot_market_index = Some(spot_market_index);
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
        let accounts = SettleRevenueToInsuranceFund {
            state: self.state.expect("state is not set"),
            spot_market: self.spot_market.expect("spot_market is not set"),
            spot_market_vault: self
                .spot_market_vault
                .expect("spot_market_vault is not set"),
            drift_signer: self.drift_signer.expect("drift_signer is not set"),
            insurance_fund_vault: self
                .insurance_fund_vault
                .expect("insurance_fund_vault is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };
        let args = SettleRevenueToInsuranceFundInstructionArgs {
            spot_market_index: self
                .spot_market_index
                .clone()
                .expect("spot_market_index is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `settle_revenue_to_insurance_fund` CPI accounts.
pub struct SettleRevenueToInsuranceFundCpiAccounts<'a, 'b> {
    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub spot_market: &'b solana_program::account_info::AccountInfo<'a>,

    pub spot_market_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub drift_signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub insurance_fund_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `settle_revenue_to_insurance_fund` CPI instruction.
pub struct SettleRevenueToInsuranceFundCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub spot_market: &'b solana_program::account_info::AccountInfo<'a>,

    pub spot_market_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub drift_signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub insurance_fund_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: SettleRevenueToInsuranceFundInstructionArgs,
}

impl<'a, 'b> SettleRevenueToInsuranceFundCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: SettleRevenueToInsuranceFundCpiAccounts<'a, 'b>,
        args: SettleRevenueToInsuranceFundInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            state: accounts.state,
            spot_market: accounts.spot_market,
            spot_market_vault: accounts.spot_market_vault,
            drift_signer: accounts.drift_signer,
            insurance_fund_vault: accounts.insurance_fund_vault,
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
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.spot_market.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.spot_market_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.drift_signer.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.insurance_fund_vault.key,
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
        let mut data = borsh::to_vec(&SettleRevenueToInsuranceFundInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::DRIFT_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.state.clone());
        account_infos.push(self.spot_market.clone());
        account_infos.push(self.spot_market_vault.clone());
        account_infos.push(self.drift_signer.clone());
        account_infos.push(self.insurance_fund_vault.clone());
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

/// Instruction builder for `SettleRevenueToInsuranceFund` via CPI.
///
/// ### Accounts:
///
///   0. `[]` state
///   1. `[writable]` spot_market
///   2. `[writable]` spot_market_vault
///   3. `[]` drift_signer
///   4. `[writable]` insurance_fund_vault
///   5. `[]` token_program
#[derive(Clone, Debug)]
pub struct SettleRevenueToInsuranceFundCpiBuilder<'a, 'b> {
    instruction: Box<SettleRevenueToInsuranceFundCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SettleRevenueToInsuranceFundCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SettleRevenueToInsuranceFundCpiBuilderInstruction {
            __program: program,
            state: None,
            spot_market: None,
            spot_market_vault: None,
            drift_signer: None,
            insurance_fund_vault: None,
            token_program: None,
            spot_market_index: None,
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
    pub fn spot_market(
        &mut self,
        spot_market: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.spot_market = Some(spot_market);
        self
    }

    #[inline(always)]
    pub fn spot_market_vault(
        &mut self,
        spot_market_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.spot_market_vault = Some(spot_market_vault);
        self
    }

    #[inline(always)]
    pub fn drift_signer(
        &mut self,
        drift_signer: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.drift_signer = Some(drift_signer);
        self
    }

    #[inline(always)]
    pub fn insurance_fund_vault(
        &mut self,
        insurance_fund_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.insurance_fund_vault = Some(insurance_fund_vault);
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
    pub fn spot_market_index(&mut self, spot_market_index: u16) -> &mut Self {
        self.instruction.spot_market_index = Some(spot_market_index);
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
        let args = SettleRevenueToInsuranceFundInstructionArgs {
            spot_market_index: self
                .instruction
                .spot_market_index
                .clone()
                .expect("spot_market_index is not set"),
        };
        let instruction = SettleRevenueToInsuranceFundCpi {
            __program: self.instruction.__program,

            state: self.instruction.state.expect("state is not set"),

            spot_market: self
                .instruction
                .spot_market
                .expect("spot_market is not set"),

            spot_market_vault: self
                .instruction
                .spot_market_vault
                .expect("spot_market_vault is not set"),

            drift_signer: self
                .instruction
                .drift_signer
                .expect("drift_signer is not set"),

            insurance_fund_vault: self
                .instruction
                .insurance_fund_vault
                .expect("insurance_fund_vault is not set"),

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
struct SettleRevenueToInsuranceFundCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    spot_market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    spot_market_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    drift_signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    insurance_fund_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    spot_market_index: Option<u16>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
