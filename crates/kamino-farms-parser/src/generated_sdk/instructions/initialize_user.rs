//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct InitializeUser {
    pub authority: solana_program::pubkey::Pubkey,

    pub payer: solana_program::pubkey::Pubkey,

    pub owner: solana_program::pubkey::Pubkey,

    pub delegatee: solana_program::pubkey::Pubkey,

    pub user_state: solana_program::pubkey::Pubkey,

    pub farm_state: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,
}

impl InitializeUser {
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.owner, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.delegatee,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_state,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.farm_state,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&InitializeUserInstructionData::new()).unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::FARMS_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeUserInstructionData {
    discriminator: [u8; 8],
}

impl InitializeUserInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [111, 17, 185, 250, 60, 122, 38, 254],
        }
    }
}

impl Default for InitializeUserInstructionData {
    fn default() -> Self { Self::new() }
}

/// Instruction builder for `InitializeUser`.
///
/// ### Accounts:
///
///   0. `[signer]` authority
///   1. `[writable, signer]` payer
///   2. `[]` owner
///   3. `[]` delegatee
///   4. `[writable]` user_state
///   5. `[writable]` farm_state
///   6. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   7. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitializeUserBuilder {
    authority: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    owner: Option<solana_program::pubkey::Pubkey>,
    delegatee: Option<solana_program::pubkey::Pubkey>,
    user_state: Option<solana_program::pubkey::Pubkey>,
    farm_state: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeUserBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }

    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }

    #[inline(always)]
    pub fn owner(&mut self, owner: solana_program::pubkey::Pubkey) -> &mut Self {
        self.owner = Some(owner);
        self
    }

    #[inline(always)]
    pub fn delegatee(&mut self, delegatee: solana_program::pubkey::Pubkey) -> &mut Self {
        self.delegatee = Some(delegatee);
        self
    }

    #[inline(always)]
    pub fn user_state(&mut self, user_state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user_state = Some(user_state);
        self
    }

    #[inline(always)]
    pub fn farm_state(&mut self, farm_state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.farm_state = Some(farm_state);
        self
    }

    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }

    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
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
        let accounts = InitializeUser {
            authority: self.authority.expect("authority is not set"),
            payer: self.payer.expect("payer is not set"),
            owner: self.owner.expect("owner is not set"),
            delegatee: self.delegatee.expect("delegatee is not set"),
            user_state: self.user_state.expect("user_state is not set"),
            farm_state: self.farm_state.expect("farm_state is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            rent: self.rent.unwrap_or(solana_program::pubkey!(
                "SysvarRent111111111111111111111111111111111"
            )),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `initialize_user` CPI accounts.
pub struct InitializeUserCpiAccounts<'a, 'b> {
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub delegatee: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub farm_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_user` CPI instruction.
pub struct InitializeUserCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub delegatee: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub farm_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> InitializeUserCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeUserCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            authority: accounts.authority,
            payer: accounts.payer,
            owner: accounts.owner,
            delegatee: accounts.delegatee,
            user_state: accounts.user_state,
            farm_state: accounts.farm_state,
            system_program: accounts.system_program,
            rent: accounts.rent,
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.owner.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.delegatee.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.farm_state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = borsh::to_vec(&InitializeUserInstructionData::new()).unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::FARMS_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.delegatee.clone());
        account_infos.push(self.user_state.clone());
        account_infos.push(self.farm_state.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.rent.clone());
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

/// Instruction builder for `InitializeUser` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` authority
///   1. `[writable, signer]` payer
///   2. `[]` owner
///   3. `[]` delegatee
///   4. `[writable]` user_state
///   5. `[writable]` farm_state
///   6. `[]` system_program
///   7. `[]` rent
#[derive(Clone, Debug)]
pub struct InitializeUserCpiBuilder<'a, 'b> {
    instruction: Box<InitializeUserCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeUserCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeUserCpiBuilderInstruction {
            __program: program,
            authority: None,
            payer: None,
            owner: None,
            delegatee: None,
            user_state: None,
            farm_state: None,
            system_program: None,
            rent: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }

    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }

    #[inline(always)]
    pub fn owner(&mut self, owner: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.owner = Some(owner);
        self
    }

    #[inline(always)]
    pub fn delegatee(
        &mut self,
        delegatee: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.delegatee = Some(delegatee);
        self
    }

    #[inline(always)]
    pub fn user_state(
        &mut self,
        user_state: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_state = Some(user_state);
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
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }

    #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
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
        let instruction = InitializeUserCpi {
            __program: self.instruction.__program,

            authority: self.instruction.authority.expect("authority is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            owner: self.instruction.owner.expect("owner is not set"),

            delegatee: self.instruction.delegatee.expect("delegatee is not set"),

            user_state: self.instruction.user_state.expect("user_state is not set"),

            farm_state: self.instruction.farm_state.expect("farm_state is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            rent: self.instruction.rent.expect("rent is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitializeUserCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    delegatee: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    farm_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
