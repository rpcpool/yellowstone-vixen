//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct InitializeLbPair {
    pub lb_pair: solana_program::pubkey::Pubkey,

    pub bin_array_bitmap_extension: Option<solana_program::pubkey::Pubkey>,

    pub token_mint_x: solana_program::pubkey::Pubkey,

    pub token_mint_y: solana_program::pubkey::Pubkey,

    pub reserve_x: solana_program::pubkey::Pubkey,

    pub reserve_y: solana_program::pubkey::Pubkey,

    pub oracle: solana_program::pubkey::Pubkey,

    pub preset_parameter: solana_program::pubkey::Pubkey,

    pub funder: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub event_authority: solana_program::pubkey::Pubkey,

    pub program: solana_program::pubkey::Pubkey,
}

impl InitializeLbPair {
    pub fn instruction(
        &self,
        args: InitializeLbPairInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializeLbPairInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(14 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.lb_pair,
            false,
        ));
        if let Some(bin_array_bitmap_extension) = self.bin_array_bitmap_extension {
            accounts.push(solana_program::instruction::AccountMeta::new(
                bin_array_bitmap_extension,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LB_CLMM_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_mint_x,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_mint_y,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.reserve_x,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.reserve_y,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.oracle,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.preset_parameter,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.funder,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.event_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&InitializeLbPairInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeLbPairInstructionData {
    discriminator: [u8; 8],
}

impl InitializeLbPairInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [45, 154, 237, 210, 221, 15, 166, 92],
        }
    }
}

impl Default for InitializeLbPairInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeLbPairInstructionArgs {
    pub active_id: i32,
    pub bin_step: u16,
}

/// Instruction builder for `InitializeLbPair`.
///
/// ### Accounts:
///
///   0. `[writable]` lb_pair
///   1. `[writable, optional]` bin_array_bitmap_extension
///   2. `[]` token_mint_x
///   3. `[]` token_mint_y
///   4. `[writable]` reserve_x
///   5. `[writable]` reserve_y
///   6. `[writable]` oracle
///   7. `[]` preset_parameter
///   8. `[writable, signer]` funder
///   9. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   10. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   11. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   12. `[]` event_authority
///   13. `[]` program
#[derive(Clone, Debug, Default)]
pub struct InitializeLbPairBuilder {
    lb_pair: Option<solana_program::pubkey::Pubkey>,
    bin_array_bitmap_extension: Option<solana_program::pubkey::Pubkey>,
    token_mint_x: Option<solana_program::pubkey::Pubkey>,
    token_mint_y: Option<solana_program::pubkey::Pubkey>,
    reserve_x: Option<solana_program::pubkey::Pubkey>,
    reserve_y: Option<solana_program::pubkey::Pubkey>,
    oracle: Option<solana_program::pubkey::Pubkey>,
    preset_parameter: Option<solana_program::pubkey::Pubkey>,
    funder: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    event_authority: Option<solana_program::pubkey::Pubkey>,
    program: Option<solana_program::pubkey::Pubkey>,
    active_id: Option<i32>,
    bin_step: Option<u16>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeLbPairBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn lb_pair(&mut self, lb_pair: solana_program::pubkey::Pubkey) -> &mut Self {
        self.lb_pair = Some(lb_pair);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn bin_array_bitmap_extension(
        &mut self,
        bin_array_bitmap_extension: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.bin_array_bitmap_extension = bin_array_bitmap_extension;
        self
    }

    #[inline(always)]
    pub fn token_mint_x(&mut self, token_mint_x: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_mint_x = Some(token_mint_x);
        self
    }

    #[inline(always)]
    pub fn token_mint_y(&mut self, token_mint_y: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_mint_y = Some(token_mint_y);
        self
    }

    #[inline(always)]
    pub fn reserve_x(&mut self, reserve_x: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reserve_x = Some(reserve_x);
        self
    }

    #[inline(always)]
    pub fn reserve_y(&mut self, reserve_y: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reserve_y = Some(reserve_y);
        self
    }

    #[inline(always)]
    pub fn oracle(&mut self, oracle: solana_program::pubkey::Pubkey) -> &mut Self {
        self.oracle = Some(oracle);
        self
    }

    #[inline(always)]
    pub fn preset_parameter(
        &mut self,
        preset_parameter: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.preset_parameter = Some(preset_parameter);
        self
    }

    #[inline(always)]
    pub fn funder(&mut self, funder: solana_program::pubkey::Pubkey) -> &mut Self {
        self.funder = Some(funder);
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

    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
        self
    }

    #[inline(always)]
    pub fn event_authority(
        &mut self,
        event_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.event_authority = Some(event_authority);
        self
    }

    #[inline(always)]
    pub fn program(&mut self, program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.program = Some(program);
        self
    }

    #[inline(always)]
    pub fn active_id(&mut self, active_id: i32) -> &mut Self {
        self.active_id = Some(active_id);
        self
    }

    #[inline(always)]
    pub fn bin_step(&mut self, bin_step: u16) -> &mut Self {
        self.bin_step = Some(bin_step);
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
        let accounts = InitializeLbPair {
            lb_pair: self.lb_pair.expect("lb_pair is not set"),
            bin_array_bitmap_extension: self.bin_array_bitmap_extension,
            token_mint_x: self.token_mint_x.expect("token_mint_x is not set"),
            token_mint_y: self.token_mint_y.expect("token_mint_y is not set"),
            reserve_x: self.reserve_x.expect("reserve_x is not set"),
            reserve_y: self.reserve_y.expect("reserve_y is not set"),
            oracle: self.oracle.expect("oracle is not set"),
            preset_parameter: self.preset_parameter.expect("preset_parameter is not set"),
            funder: self.funder.expect("funder is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            rent: self.rent.unwrap_or(solana_program::pubkey!(
                "SysvarRent111111111111111111111111111111111"
            )),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = InitializeLbPairInstructionArgs {
            active_id: self.active_id.clone().expect("active_id is not set"),
            bin_step: self.bin_step.clone().expect("bin_step is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize_lb_pair` CPI accounts.
pub struct InitializeLbPairCpiAccounts<'a, 'b> {
    pub lb_pair: &'b solana_program::account_info::AccountInfo<'a>,

    pub bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub token_mint_x: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_mint_y: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve_x: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve_y: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,

    pub preset_parameter: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_lb_pair` CPI instruction.
pub struct InitializeLbPairCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub lb_pair: &'b solana_program::account_info::AccountInfo<'a>,

    pub bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub token_mint_x: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_mint_y: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve_x: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve_y: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,

    pub preset_parameter: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitializeLbPairInstructionArgs,
}

impl<'a, 'b> InitializeLbPairCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeLbPairCpiAccounts<'a, 'b>,
        args: InitializeLbPairInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            lb_pair: accounts.lb_pair,
            bin_array_bitmap_extension: accounts.bin_array_bitmap_extension,
            token_mint_x: accounts.token_mint_x,
            token_mint_y: accounts.token_mint_y,
            reserve_x: accounts.reserve_x,
            reserve_y: accounts.reserve_y,
            oracle: accounts.oracle,
            preset_parameter: accounts.preset_parameter,
            funder: accounts.funder,
            token_program: accounts.token_program,
            system_program: accounts.system_program,
            rent: accounts.rent,
            event_authority: accounts.event_authority,
            program: accounts.program,
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
        let mut accounts = Vec::with_capacity(14 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lb_pair.key,
            false,
        ));
        if let Some(bin_array_bitmap_extension) = self.bin_array_bitmap_extension {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *bin_array_bitmap_extension.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LB_CLMM_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_mint_x.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_mint_y.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reserve_x.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reserve_y.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.oracle.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.preset_parameter.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.funder.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.event_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&InitializeLbPairInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(15 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.lb_pair.clone());
        if let Some(bin_array_bitmap_extension) = self.bin_array_bitmap_extension {
            account_infos.push(bin_array_bitmap_extension.clone());
        }
        account_infos.push(self.token_mint_x.clone());
        account_infos.push(self.token_mint_y.clone());
        account_infos.push(self.reserve_x.clone());
        account_infos.push(self.reserve_y.clone());
        account_infos.push(self.oracle.clone());
        account_infos.push(self.preset_parameter.clone());
        account_infos.push(self.funder.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.event_authority.clone());
        account_infos.push(self.program.clone());
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

/// Instruction builder for `InitializeLbPair` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` lb_pair
///   1. `[writable, optional]` bin_array_bitmap_extension
///   2. `[]` token_mint_x
///   3. `[]` token_mint_y
///   4. `[writable]` reserve_x
///   5. `[writable]` reserve_y
///   6. `[writable]` oracle
///   7. `[]` preset_parameter
///   8. `[writable, signer]` funder
///   9. `[]` token_program
///   10. `[]` system_program
///   11. `[]` rent
///   12. `[]` event_authority
///   13. `[]` program
#[derive(Clone, Debug)]
pub struct InitializeLbPairCpiBuilder<'a, 'b> {
    instruction: Box<InitializeLbPairCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeLbPairCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeLbPairCpiBuilderInstruction {
            __program: program,
            lb_pair: None,
            bin_array_bitmap_extension: None,
            token_mint_x: None,
            token_mint_y: None,
            reserve_x: None,
            reserve_y: None,
            oracle: None,
            preset_parameter: None,
            funder: None,
            token_program: None,
            system_program: None,
            rent: None,
            event_authority: None,
            program: None,
            active_id: None,
            bin_step: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn lb_pair(
        &mut self,
        lb_pair: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lb_pair = Some(lb_pair);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn bin_array_bitmap_extension(
        &mut self,
        bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.bin_array_bitmap_extension = bin_array_bitmap_extension;
        self
    }

    #[inline(always)]
    pub fn token_mint_x(
        &mut self,
        token_mint_x: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_mint_x = Some(token_mint_x);
        self
    }

    #[inline(always)]
    pub fn token_mint_y(
        &mut self,
        token_mint_y: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_mint_y = Some(token_mint_y);
        self
    }

    #[inline(always)]
    pub fn reserve_x(
        &mut self,
        reserve_x: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reserve_x = Some(reserve_x);
        self
    }

    #[inline(always)]
    pub fn reserve_y(
        &mut self,
        reserve_y: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reserve_y = Some(reserve_y);
        self
    }

    #[inline(always)]
    pub fn oracle(
        &mut self,
        oracle: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.oracle = Some(oracle);
        self
    }

    #[inline(always)]
    pub fn preset_parameter(
        &mut self,
        preset_parameter: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.preset_parameter = Some(preset_parameter);
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

    #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
        self
    }

    #[inline(always)]
    pub fn event_authority(
        &mut self,
        event_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.event_authority = Some(event_authority);
        self
    }

    #[inline(always)]
    pub fn program(
        &mut self,
        program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program = Some(program);
        self
    }

    #[inline(always)]
    pub fn active_id(&mut self, active_id: i32) -> &mut Self {
        self.instruction.active_id = Some(active_id);
        self
    }

    #[inline(always)]
    pub fn bin_step(&mut self, bin_step: u16) -> &mut Self {
        self.instruction.bin_step = Some(bin_step);
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
        let args = InitializeLbPairInstructionArgs {
            active_id: self
                .instruction
                .active_id
                .clone()
                .expect("active_id is not set"),
            bin_step: self
                .instruction
                .bin_step
                .clone()
                .expect("bin_step is not set"),
        };
        let instruction = InitializeLbPairCpi {
            __program: self.instruction.__program,

            lb_pair: self.instruction.lb_pair.expect("lb_pair is not set"),

            bin_array_bitmap_extension: self.instruction.bin_array_bitmap_extension,

            token_mint_x: self
                .instruction
                .token_mint_x
                .expect("token_mint_x is not set"),

            token_mint_y: self
                .instruction
                .token_mint_y
                .expect("token_mint_y is not set"),

            reserve_x: self.instruction.reserve_x.expect("reserve_x is not set"),

            reserve_y: self.instruction.reserve_y.expect("reserve_y is not set"),

            oracle: self.instruction.oracle.expect("oracle is not set"),

            preset_parameter: self
                .instruction
                .preset_parameter
                .expect("preset_parameter is not set"),

            funder: self.instruction.funder.expect("funder is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            rent: self.instruction.rent.expect("rent is not set"),

            event_authority: self
                .instruction
                .event_authority
                .expect("event_authority is not set"),

            program: self.instruction.program.expect("program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitializeLbPairCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    lb_pair: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_mint_x: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_mint_y: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reserve_x: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reserve_y: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    oracle: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    preset_parameter: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    funder: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    active_id: Option<i32>,
    bin_step: Option<u16>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
