//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct LockPosition {
    pub pool: solana_pubkey::Pubkey,

    pub position: solana_pubkey::Pubkey,

    pub vesting: solana_pubkey::Pubkey,
    /// The token account for nft
    pub position_nft_account: solana_pubkey::Pubkey,
    /// owner of position
    pub owner: solana_pubkey::Pubkey,

    pub payer: solana_pubkey::Pubkey,

    pub system_program: solana_pubkey::Pubkey,

    pub event_authority: solana_pubkey::Pubkey,

    pub program: solana_pubkey::Pubkey,
}

impl LockPosition {
    pub fn instruction(
        &self,
        args: LockPositionInstructionArgs,
    ) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: LockPositionInstructionArgs,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.pool, false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(self.position, false));
        accounts.push(solana_instruction::AccountMeta::new(self.vesting, true));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.position_nft_account,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.owner, true,
        ));
        accounts.push(solana_instruction::AccountMeta::new(self.payer, true));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.event_authority,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&LockPositionInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_instruction::Instruction {
            program_id: crate::CP_AMM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LockPositionInstructionData {
    discriminator: [u8; 8],
}

impl LockPositionInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [227, 62, 2, 252, 247, 10, 171, 185],
        }
    }
}

impl Default for LockPositionInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LockPositionInstructionArgs {
    pub cliff_point: Option<u64>,
    pub period_frequency: u64,
    pub cliff_unlock_liquidity: u128,
    pub liquidity_per_period: u128,
    pub number_of_period: u16,
}

/// Instruction builder for `LockPosition`.
///
/// ### Accounts:
///
///   0. `[]` pool
///   1. `[writable]` position
///   2. `[writable, signer]` vesting
///   3. `[]` position_nft_account
///   4. `[signer]` owner
///   5. `[writable, signer]` payer
///   6. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   7. `[]` event_authority
///   8. `[]` program
#[derive(Clone, Debug, Default)]
pub struct LockPositionBuilder {
    pool: Option<solana_pubkey::Pubkey>,
    position: Option<solana_pubkey::Pubkey>,
    vesting: Option<solana_pubkey::Pubkey>,
    position_nft_account: Option<solana_pubkey::Pubkey>,
    owner: Option<solana_pubkey::Pubkey>,
    payer: Option<solana_pubkey::Pubkey>,
    system_program: Option<solana_pubkey::Pubkey>,
    event_authority: Option<solana_pubkey::Pubkey>,
    program: Option<solana_pubkey::Pubkey>,
    cliff_point: Option<u64>,
    period_frequency: Option<u64>,
    cliff_unlock_liquidity: Option<u128>,
    liquidity_per_period: Option<u128>,
    number_of_period: Option<u16>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl LockPositionBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn pool(&mut self, pool: solana_pubkey::Pubkey) -> &mut Self {
        self.pool = Some(pool);
        self
    }

    #[inline(always)]
    pub fn position(&mut self, position: solana_pubkey::Pubkey) -> &mut Self {
        self.position = Some(position);
        self
    }

    #[inline(always)]
    pub fn vesting(&mut self, vesting: solana_pubkey::Pubkey) -> &mut Self {
        self.vesting = Some(vesting);
        self
    }

    /// The token account for nft
    #[inline(always)]
    pub fn position_nft_account(
        &mut self,
        position_nft_account: solana_pubkey::Pubkey,
    ) -> &mut Self {
        self.position_nft_account = Some(position_nft_account);
        self
    }

    /// owner of position
    #[inline(always)]
    pub fn owner(&mut self, owner: solana_pubkey::Pubkey) -> &mut Self {
        self.owner = Some(owner);
        self
    }

    #[inline(always)]
    pub fn payer(&mut self, payer: solana_pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }

    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }

    #[inline(always)]
    pub fn event_authority(&mut self, event_authority: solana_pubkey::Pubkey) -> &mut Self {
        self.event_authority = Some(event_authority);
        self
    }

    #[inline(always)]
    pub fn program(&mut self, program: solana_pubkey::Pubkey) -> &mut Self {
        self.program = Some(program);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn cliff_point(&mut self, cliff_point: u64) -> &mut Self {
        self.cliff_point = Some(cliff_point);
        self
    }

    #[inline(always)]
    pub fn period_frequency(&mut self, period_frequency: u64) -> &mut Self {
        self.period_frequency = Some(period_frequency);
        self
    }

    #[inline(always)]
    pub fn cliff_unlock_liquidity(&mut self, cliff_unlock_liquidity: u128) -> &mut Self {
        self.cliff_unlock_liquidity = Some(cliff_unlock_liquidity);
        self
    }

    #[inline(always)]
    pub fn liquidity_per_period(&mut self, liquidity_per_period: u128) -> &mut Self {
        self.liquidity_per_period = Some(liquidity_per_period);
        self
    }

    #[inline(always)]
    pub fn number_of_period(&mut self, number_of_period: u16) -> &mut Self {
        self.number_of_period = Some(number_of_period);
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
        let accounts = LockPosition {
            pool: self.pool.expect("pool is not set"),
            position: self.position.expect("position is not set"),
            vesting: self.vesting.expect("vesting is not set"),
            position_nft_account: self
                .position_nft_account
                .expect("position_nft_account is not set"),
            owner: self.owner.expect("owner is not set"),
            payer: self.payer.expect("payer is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_pubkey::pubkey!("11111111111111111111111111111111")),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = LockPositionInstructionArgs {
            cliff_point: self.cliff_point.clone(),
            period_frequency: self
                .period_frequency
                .clone()
                .expect("period_frequency is not set"),
            cliff_unlock_liquidity: self
                .cliff_unlock_liquidity
                .clone()
                .expect("cliff_unlock_liquidity is not set"),
            liquidity_per_period: self
                .liquidity_per_period
                .clone()
                .expect("liquidity_per_period is not set"),
            number_of_period: self
                .number_of_period
                .clone()
                .expect("number_of_period is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `lock_position` CPI accounts.
pub struct LockPositionCpiAccounts<'a, 'b> {
    pub pool: &'b solana_account_info::AccountInfo<'a>,

    pub position: &'b solana_account_info::AccountInfo<'a>,

    pub vesting: &'b solana_account_info::AccountInfo<'a>,
    /// The token account for nft
    pub position_nft_account: &'b solana_account_info::AccountInfo<'a>,
    /// owner of position
    pub owner: &'b solana_account_info::AccountInfo<'a>,

    pub payer: &'b solana_account_info::AccountInfo<'a>,

    pub system_program: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
}

/// `lock_position` CPI instruction.
pub struct LockPositionCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_account_info::AccountInfo<'a>,

    pub pool: &'b solana_account_info::AccountInfo<'a>,

    pub position: &'b solana_account_info::AccountInfo<'a>,

    pub vesting: &'b solana_account_info::AccountInfo<'a>,
    /// The token account for nft
    pub position_nft_account: &'b solana_account_info::AccountInfo<'a>,
    /// owner of position
    pub owner: &'b solana_account_info::AccountInfo<'a>,

    pub payer: &'b solana_account_info::AccountInfo<'a>,

    pub system_program: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: LockPositionInstructionArgs,
}

impl<'a, 'b> LockPositionCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: LockPositionCpiAccounts<'a, 'b>,
        args: LockPositionInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            pool: accounts.pool,
            position: accounts.position,
            vesting: accounts.vesting,
            position_nft_account: accounts.position_nft_account,
            owner: accounts.owner,
            payer: accounts.payer,
            system_program: accounts.system_program,
            event_authority: accounts.event_authority,
            program: accounts.program,
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
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.pool.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.position.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.vesting.key,
            true,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.position_nft_account.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.owner.key,
            true,
        ));
        accounts.push(solana_instruction::AccountMeta::new(*self.payer.key, true));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.event_authority.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&LockPositionInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_instruction::Instruction {
            program_id: crate::CP_AMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(10 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.pool.clone());
        account_infos.push(self.position.clone());
        account_infos.push(self.vesting.clone());
        account_infos.push(self.position_nft_account.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.event_authority.clone());
        account_infos.push(self.program.clone());
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

/// Instruction builder for `LockPosition` via CPI.
///
/// ### Accounts:
///
///   0. `[]` pool
///   1. `[writable]` position
///   2. `[writable, signer]` vesting
///   3. `[]` position_nft_account
///   4. `[signer]` owner
///   5. `[writable, signer]` payer
///   6. `[]` system_program
///   7. `[]` event_authority
///   8. `[]` program
#[derive(Clone, Debug)]
pub struct LockPositionCpiBuilder<'a, 'b> {
    instruction: Box<LockPositionCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> LockPositionCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(LockPositionCpiBuilderInstruction {
            __program: program,
            pool: None,
            position: None,
            vesting: None,
            position_nft_account: None,
            owner: None,
            payer: None,
            system_program: None,
            event_authority: None,
            program: None,
            cliff_point: None,
            period_frequency: None,
            cliff_unlock_liquidity: None,
            liquidity_per_period: None,
            number_of_period: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn pool(&mut self, pool: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.pool = Some(pool);
        self
    }

    #[inline(always)]
    pub fn position(&mut self, position: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.position = Some(position);
        self
    }

    #[inline(always)]
    pub fn vesting(&mut self, vesting: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.vesting = Some(vesting);
        self
    }

    /// The token account for nft
    #[inline(always)]
    pub fn position_nft_account(
        &mut self,
        position_nft_account: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position_nft_account = Some(position_nft_account);
        self
    }

    /// owner of position
    #[inline(always)]
    pub fn owner(&mut self, owner: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.owner = Some(owner);
        self
    }

    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
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
    pub fn event_authority(
        &mut self,
        event_authority: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.event_authority = Some(event_authority);
        self
    }

    #[inline(always)]
    pub fn program(&mut self, program: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.program = Some(program);
        self
    }

    /// `[optional argument]`
    #[inline(always)]
    pub fn cliff_point(&mut self, cliff_point: u64) -> &mut Self {
        self.instruction.cliff_point = Some(cliff_point);
        self
    }

    #[inline(always)]
    pub fn period_frequency(&mut self, period_frequency: u64) -> &mut Self {
        self.instruction.period_frequency = Some(period_frequency);
        self
    }

    #[inline(always)]
    pub fn cliff_unlock_liquidity(&mut self, cliff_unlock_liquidity: u128) -> &mut Self {
        self.instruction.cliff_unlock_liquidity = Some(cliff_unlock_liquidity);
        self
    }

    #[inline(always)]
    pub fn liquidity_per_period(&mut self, liquidity_per_period: u128) -> &mut Self {
        self.instruction.liquidity_per_period = Some(liquidity_per_period);
        self
    }

    #[inline(always)]
    pub fn number_of_period(&mut self, number_of_period: u16) -> &mut Self {
        self.instruction.number_of_period = Some(number_of_period);
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
        let args = LockPositionInstructionArgs {
            cliff_point: self.instruction.cliff_point.clone(),
            period_frequency: self
                .instruction
                .period_frequency
                .clone()
                .expect("period_frequency is not set"),
            cliff_unlock_liquidity: self
                .instruction
                .cliff_unlock_liquidity
                .clone()
                .expect("cliff_unlock_liquidity is not set"),
            liquidity_per_period: self
                .instruction
                .liquidity_per_period
                .clone()
                .expect("liquidity_per_period is not set"),
            number_of_period: self
                .instruction
                .number_of_period
                .clone()
                .expect("number_of_period is not set"),
        };
        let instruction = LockPositionCpi {
            __program: self.instruction.__program,

            pool: self.instruction.pool.expect("pool is not set"),

            position: self.instruction.position.expect("position is not set"),

            vesting: self.instruction.vesting.expect("vesting is not set"),

            position_nft_account: self
                .instruction
                .position_nft_account
                .expect("position_nft_account is not set"),

            owner: self.instruction.owner.expect("owner is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

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
struct LockPositionCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    pool: Option<&'b solana_account_info::AccountInfo<'a>>,
    position: Option<&'b solana_account_info::AccountInfo<'a>>,
    vesting: Option<&'b solana_account_info::AccountInfo<'a>>,
    position_nft_account: Option<&'b solana_account_info::AccountInfo<'a>>,
    owner: Option<&'b solana_account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_account_info::AccountInfo<'a>>,
    program: Option<&'b solana_account_info::AccountInfo<'a>>,
    cliff_point: Option<u64>,
    period_frequency: Option<u64>,
    cliff_unlock_liquidity: Option<u128>,
    liquidity_per_period: Option<u128>,
    number_of_period: Option<u16>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
