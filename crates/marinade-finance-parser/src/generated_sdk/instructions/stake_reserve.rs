//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct StakeReserve {
    pub state: solana_program::pubkey::Pubkey,

    pub validator_list: solana_program::pubkey::Pubkey,

    pub stake_list: solana_program::pubkey::Pubkey,

    pub validator_vote: solana_program::pubkey::Pubkey,

    pub reserve_pda: solana_program::pubkey::Pubkey,

    pub stake_account: solana_program::pubkey::Pubkey,

    pub stake_deposit_authority: solana_program::pubkey::Pubkey,

    pub rent_payer: solana_program::pubkey::Pubkey,

    pub clock: solana_program::pubkey::Pubkey,

    pub epoch_schedule: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub stake_history: solana_program::pubkey::Pubkey,

    pub stake_config: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub stake_program: solana_program::pubkey::Pubkey,
}

impl StakeReserve {
    pub fn instruction(
        &self,
        args: StakeReserveInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: StakeReserveInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(15 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.state, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.validator_list,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.stake_list,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.validator_vote,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.reserve_pda,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.stake_account,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.stake_deposit_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.rent_payer,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.clock, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.epoch_schedule,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.stake_history,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.stake_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.stake_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&StakeReserveInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::MARINADE_FINANCE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakeReserveInstructionData {
    discriminator: [u8; 8],
}

impl StakeReserveInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [87, 217, 23, 179, 205, 25, 113, 129],
        }
    }
}

impl Default for StakeReserveInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakeReserveInstructionArgs {
    pub validator_index: u32,
}

/// Instruction builder for `StakeReserve`.
///
/// ### Accounts:
///
///   0. `[writable]` state
///   1. `[writable]` validator_list
///   2. `[writable]` stake_list
///   3. `[writable]` validator_vote
///   4. `[writable]` reserve_pda
///   5. `[writable, signer]` stake_account
///   6. `[]` stake_deposit_authority
///   7. `[writable, signer]` rent_payer
///   8. `[]` clock
///   9. `[]` epoch_schedule
///   10. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   11. `[]` stake_history
///   12. `[]` stake_config
///   13. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   14. `[]` stake_program
#[derive(Clone, Debug, Default)]
pub struct StakeReserveBuilder {
    state: Option<solana_program::pubkey::Pubkey>,
    validator_list: Option<solana_program::pubkey::Pubkey>,
    stake_list: Option<solana_program::pubkey::Pubkey>,
    validator_vote: Option<solana_program::pubkey::Pubkey>,
    reserve_pda: Option<solana_program::pubkey::Pubkey>,
    stake_account: Option<solana_program::pubkey::Pubkey>,
    stake_deposit_authority: Option<solana_program::pubkey::Pubkey>,
    rent_payer: Option<solana_program::pubkey::Pubkey>,
    clock: Option<solana_program::pubkey::Pubkey>,
    epoch_schedule: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    stake_history: Option<solana_program::pubkey::Pubkey>,
    stake_config: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    stake_program: Option<solana_program::pubkey::Pubkey>,
    validator_index: Option<u32>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl StakeReserveBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn state(&mut self, state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.state = Some(state);
        self
    }

    #[inline(always)]
    pub fn validator_list(&mut self, validator_list: solana_program::pubkey::Pubkey) -> &mut Self {
        self.validator_list = Some(validator_list);
        self
    }

    #[inline(always)]
    pub fn stake_list(&mut self, stake_list: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake_list = Some(stake_list);
        self
    }

    #[inline(always)]
    pub fn validator_vote(&mut self, validator_vote: solana_program::pubkey::Pubkey) -> &mut Self {
        self.validator_vote = Some(validator_vote);
        self
    }

    #[inline(always)]
    pub fn reserve_pda(&mut self, reserve_pda: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reserve_pda = Some(reserve_pda);
        self
    }

    #[inline(always)]
    pub fn stake_account(&mut self, stake_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake_account = Some(stake_account);
        self
    }

    #[inline(always)]
    pub fn stake_deposit_authority(
        &mut self,
        stake_deposit_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.stake_deposit_authority = Some(stake_deposit_authority);
        self
    }

    #[inline(always)]
    pub fn rent_payer(&mut self, rent_payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent_payer = Some(rent_payer);
        self
    }

    #[inline(always)]
    pub fn clock(&mut self, clock: solana_program::pubkey::Pubkey) -> &mut Self {
        self.clock = Some(clock);
        self
    }

    #[inline(always)]
    pub fn epoch_schedule(&mut self, epoch_schedule: solana_program::pubkey::Pubkey) -> &mut Self {
        self.epoch_schedule = Some(epoch_schedule);
        self
    }

    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
        self
    }

    #[inline(always)]
    pub fn stake_history(&mut self, stake_history: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake_history = Some(stake_history);
        self
    }

    #[inline(always)]
    pub fn stake_config(&mut self, stake_config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake_config = Some(stake_config);
        self
    }

    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }

    #[inline(always)]
    pub fn stake_program(&mut self, stake_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake_program = Some(stake_program);
        self
    }

    #[inline(always)]
    pub fn validator_index(&mut self, validator_index: u32) -> &mut Self {
        self.validator_index = Some(validator_index);
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
        let accounts = StakeReserve {
            state: self.state.expect("state is not set"),
            validator_list: self.validator_list.expect("validator_list is not set"),
            stake_list: self.stake_list.expect("stake_list is not set"),
            validator_vote: self.validator_vote.expect("validator_vote is not set"),
            reserve_pda: self.reserve_pda.expect("reserve_pda is not set"),
            stake_account: self.stake_account.expect("stake_account is not set"),
            stake_deposit_authority: self
                .stake_deposit_authority
                .expect("stake_deposit_authority is not set"),
            rent_payer: self.rent_payer.expect("rent_payer is not set"),
            clock: self.clock.expect("clock is not set"),
            epoch_schedule: self.epoch_schedule.expect("epoch_schedule is not set"),
            rent: self.rent.unwrap_or(solana_program::pubkey!(
                "SysvarRent111111111111111111111111111111111"
            )),
            stake_history: self.stake_history.expect("stake_history is not set"),
            stake_config: self.stake_config.expect("stake_config is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            stake_program: self.stake_program.expect("stake_program is not set"),
        };
        let args = StakeReserveInstructionArgs {
            validator_index: self
                .validator_index
                .clone()
                .expect("validator_index is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `stake_reserve` CPI accounts.
pub struct StakeReserveCpiAccounts<'a, 'b> {
    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub validator_list: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_list: &'b solana_program::account_info::AccountInfo<'a>,

    pub validator_vote: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve_pda: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_deposit_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent_payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub clock: &'b solana_program::account_info::AccountInfo<'a>,

    pub epoch_schedule: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_history: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `stake_reserve` CPI instruction.
pub struct StakeReserveCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub validator_list: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_list: &'b solana_program::account_info::AccountInfo<'a>,

    pub validator_vote: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve_pda: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_deposit_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent_payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub clock: &'b solana_program::account_info::AccountInfo<'a>,

    pub epoch_schedule: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_history: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: StakeReserveInstructionArgs,
}

impl<'a, 'b> StakeReserveCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: StakeReserveCpiAccounts<'a, 'b>,
        args: StakeReserveInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            state: accounts.state,
            validator_list: accounts.validator_list,
            stake_list: accounts.stake_list,
            validator_vote: accounts.validator_vote,
            reserve_pda: accounts.reserve_pda,
            stake_account: accounts.stake_account,
            stake_deposit_authority: accounts.stake_deposit_authority,
            rent_payer: accounts.rent_payer,
            clock: accounts.clock,
            epoch_schedule: accounts.epoch_schedule,
            rent: accounts.rent,
            stake_history: accounts.stake_history,
            stake_config: accounts.stake_config,
            system_program: accounts.system_program,
            stake_program: accounts.stake_program,
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
        let mut accounts = Vec::with_capacity(15 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.validator_list.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.stake_list.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.validator_vote.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reserve_pda.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.stake_account.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.stake_deposit_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.rent_payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.clock.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.epoch_schedule.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.stake_history.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.stake_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.stake_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&StakeReserveInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MARINADE_FINANCE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(16 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.state.clone());
        account_infos.push(self.validator_list.clone());
        account_infos.push(self.stake_list.clone());
        account_infos.push(self.validator_vote.clone());
        account_infos.push(self.reserve_pda.clone());
        account_infos.push(self.stake_account.clone());
        account_infos.push(self.stake_deposit_authority.clone());
        account_infos.push(self.rent_payer.clone());
        account_infos.push(self.clock.clone());
        account_infos.push(self.epoch_schedule.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.stake_history.clone());
        account_infos.push(self.stake_config.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.stake_program.clone());
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

/// Instruction builder for `StakeReserve` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` state
///   1. `[writable]` validator_list
///   2. `[writable]` stake_list
///   3. `[writable]` validator_vote
///   4. `[writable]` reserve_pda
///   5. `[writable, signer]` stake_account
///   6. `[]` stake_deposit_authority
///   7. `[writable, signer]` rent_payer
///   8. `[]` clock
///   9. `[]` epoch_schedule
///   10. `[]` rent
///   11. `[]` stake_history
///   12. `[]` stake_config
///   13. `[]` system_program
///   14. `[]` stake_program
#[derive(Clone, Debug)]
pub struct StakeReserveCpiBuilder<'a, 'b> {
    instruction: Box<StakeReserveCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> StakeReserveCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(StakeReserveCpiBuilderInstruction {
            __program: program,
            state: None,
            validator_list: None,
            stake_list: None,
            validator_vote: None,
            reserve_pda: None,
            stake_account: None,
            stake_deposit_authority: None,
            rent_payer: None,
            clock: None,
            epoch_schedule: None,
            rent: None,
            stake_history: None,
            stake_config: None,
            system_program: None,
            stake_program: None,
            validator_index: None,
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
    pub fn validator_list(
        &mut self,
        validator_list: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.validator_list = Some(validator_list);
        self
    }

    #[inline(always)]
    pub fn stake_list(
        &mut self,
        stake_list: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_list = Some(stake_list);
        self
    }

    #[inline(always)]
    pub fn validator_vote(
        &mut self,
        validator_vote: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.validator_vote = Some(validator_vote);
        self
    }

    #[inline(always)]
    pub fn reserve_pda(
        &mut self,
        reserve_pda: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reserve_pda = Some(reserve_pda);
        self
    }

    #[inline(always)]
    pub fn stake_account(
        &mut self,
        stake_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_account = Some(stake_account);
        self
    }

    #[inline(always)]
    pub fn stake_deposit_authority(
        &mut self,
        stake_deposit_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_deposit_authority = Some(stake_deposit_authority);
        self
    }

    #[inline(always)]
    pub fn rent_payer(
        &mut self,
        rent_payer: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.rent_payer = Some(rent_payer);
        self
    }

    #[inline(always)]
    pub fn clock(&mut self, clock: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.clock = Some(clock);
        self
    }

    #[inline(always)]
    pub fn epoch_schedule(
        &mut self,
        epoch_schedule: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.epoch_schedule = Some(epoch_schedule);
        self
    }

    #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
        self
    }

    #[inline(always)]
    pub fn stake_history(
        &mut self,
        stake_history: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_history = Some(stake_history);
        self
    }

    #[inline(always)]
    pub fn stake_config(
        &mut self,
        stake_config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_config = Some(stake_config);
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
    pub fn stake_program(
        &mut self,
        stake_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_program = Some(stake_program);
        self
    }

    #[inline(always)]
    pub fn validator_index(&mut self, validator_index: u32) -> &mut Self {
        self.instruction.validator_index = Some(validator_index);
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
        let args = StakeReserveInstructionArgs {
            validator_index: self
                .instruction
                .validator_index
                .clone()
                .expect("validator_index is not set"),
        };
        let instruction = StakeReserveCpi {
            __program: self.instruction.__program,

            state: self.instruction.state.expect("state is not set"),

            validator_list: self
                .instruction
                .validator_list
                .expect("validator_list is not set"),

            stake_list: self.instruction.stake_list.expect("stake_list is not set"),

            validator_vote: self
                .instruction
                .validator_vote
                .expect("validator_vote is not set"),

            reserve_pda: self
                .instruction
                .reserve_pda
                .expect("reserve_pda is not set"),

            stake_account: self
                .instruction
                .stake_account
                .expect("stake_account is not set"),

            stake_deposit_authority: self
                .instruction
                .stake_deposit_authority
                .expect("stake_deposit_authority is not set"),

            rent_payer: self.instruction.rent_payer.expect("rent_payer is not set"),

            clock: self.instruction.clock.expect("clock is not set"),

            epoch_schedule: self
                .instruction
                .epoch_schedule
                .expect("epoch_schedule is not set"),

            rent: self.instruction.rent.expect("rent is not set"),

            stake_history: self
                .instruction
                .stake_history
                .expect("stake_history is not set"),

            stake_config: self
                .instruction
                .stake_config
                .expect("stake_config is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            stake_program: self
                .instruction
                .stake_program
                .expect("stake_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct StakeReserveCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    validator_list: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_list: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    validator_vote: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reserve_pda: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_deposit_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent_payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    clock: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    epoch_schedule: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_history: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    validator_index: Option<u32>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
