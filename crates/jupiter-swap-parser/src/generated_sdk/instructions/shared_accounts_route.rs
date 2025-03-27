//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

use crate::generated::types::RoutePlanStep;

/// Accounts.
#[derive(Debug)]
pub struct SharedAccountsRoute {
    pub token_program: solana_program::pubkey::Pubkey,

    pub program_authority: solana_program::pubkey::Pubkey,

    pub user_transfer_authority: solana_program::pubkey::Pubkey,

    pub source_token_account: solana_program::pubkey::Pubkey,

    pub program_source_token_account: solana_program::pubkey::Pubkey,

    pub program_destination_token_account: solana_program::pubkey::Pubkey,

    pub destination_token_account: solana_program::pubkey::Pubkey,

    pub source_mint: solana_program::pubkey::Pubkey,

    pub destination_mint: solana_program::pubkey::Pubkey,

    pub platform_fee_account: Option<solana_program::pubkey::Pubkey>,

    pub token2022_program: Option<solana_program::pubkey::Pubkey>,

    pub event_authority: solana_program::pubkey::Pubkey,

    pub program: solana_program::pubkey::Pubkey,
}

impl SharedAccountsRoute {
    pub fn instruction(
        &self,
        args: SharedAccountsRouteInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: SharedAccountsRouteInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(13 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.program_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.user_transfer_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.source_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.program_source_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.program_destination_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.destination_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.source_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.destination_mint,
            false,
        ));
        if let Some(platform_fee_account) = self.platform_fee_account {
            accounts.push(solana_program::instruction::AccountMeta::new(
                platform_fee_account,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::JUPITER_ID,
                false,
            ));
        }
        if let Some(token2022_program) = self.token2022_program {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                token2022_program,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::JUPITER_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.event_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&SharedAccountsRouteInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SharedAccountsRouteInstructionData {
    discriminator: [u8; 8],
}

impl SharedAccountsRouteInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [193, 32, 155, 51, 65, 214, 156, 129],
        }
    }
}

impl Default for SharedAccountsRouteInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SharedAccountsRouteInstructionArgs {
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}

/// Instruction builder for `SharedAccountsRoute`.
///
/// ### Accounts:
///
///   0. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   1. `[]` program_authority
///   2. `[signer]` user_transfer_authority
///   3. `[writable]` source_token_account
///   4. `[writable]` program_source_token_account
///   5. `[writable]` program_destination_token_account
///   6. `[writable]` destination_token_account
///   7. `[]` source_mint
///   8. `[]` destination_mint
///   9. `[writable, optional]` platform_fee_account
///   10. `[optional]` token2022_program
///   11. `[optional]` event_authority (default to `D8cy77BBepLMngZx6ZukaTff5hCt1HrWyKk3Hnd9oitf`)
///   12. `[]` program
#[derive(Clone, Debug, Default)]
pub struct SharedAccountsRouteBuilder {
    token_program: Option<solana_program::pubkey::Pubkey>,
    program_authority: Option<solana_program::pubkey::Pubkey>,
    user_transfer_authority: Option<solana_program::pubkey::Pubkey>,
    source_token_account: Option<solana_program::pubkey::Pubkey>,
    program_source_token_account: Option<solana_program::pubkey::Pubkey>,
    program_destination_token_account: Option<solana_program::pubkey::Pubkey>,
    destination_token_account: Option<solana_program::pubkey::Pubkey>,
    source_mint: Option<solana_program::pubkey::Pubkey>,
    destination_mint: Option<solana_program::pubkey::Pubkey>,
    platform_fee_account: Option<solana_program::pubkey::Pubkey>,
    token2022_program: Option<solana_program::pubkey::Pubkey>,
    event_authority: Option<solana_program::pubkey::Pubkey>,
    program: Option<solana_program::pubkey::Pubkey>,
    id: Option<u8>,
    route_plan: Option<Vec<RoutePlanStep>>,
    in_amount: Option<u64>,
    quoted_out_amount: Option<u64>,
    slippage_bps: Option<u16>,
    platform_fee_bps: Option<u8>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl SharedAccountsRouteBuilder {
    pub fn new() -> Self { Self::default() }

    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }

    #[inline(always)]
    pub fn program_authority(
        &mut self,
        program_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.program_authority = Some(program_authority);
        self
    }

    #[inline(always)]
    pub fn user_transfer_authority(
        &mut self,
        user_transfer_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.user_transfer_authority = Some(user_transfer_authority);
        self
    }

    #[inline(always)]
    pub fn source_token_account(
        &mut self,
        source_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.source_token_account = Some(source_token_account);
        self
    }

    #[inline(always)]
    pub fn program_source_token_account(
        &mut self,
        program_source_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.program_source_token_account = Some(program_source_token_account);
        self
    }

    #[inline(always)]
    pub fn program_destination_token_account(
        &mut self,
        program_destination_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.program_destination_token_account = Some(program_destination_token_account);
        self
    }

    #[inline(always)]
    pub fn destination_token_account(
        &mut self,
        destination_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.destination_token_account = Some(destination_token_account);
        self
    }

    #[inline(always)]
    pub fn source_mint(&mut self, source_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.source_mint = Some(source_mint);
        self
    }

    #[inline(always)]
    pub fn destination_mint(
        &mut self,
        destination_mint: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.destination_mint = Some(destination_mint);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn platform_fee_account(
        &mut self,
        platform_fee_account: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.platform_fee_account = platform_fee_account;
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn token2022_program(
        &mut self,
        token2022_program: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.token2022_program = token2022_program;
        self
    }

    /// `[optional account, default to 'D8cy77BBepLMngZx6ZukaTff5hCt1HrWyKk3Hnd9oitf']`
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
    pub fn id(&mut self, id: u8) -> &mut Self {
        self.id = Some(id);
        self
    }

    #[inline(always)]
    pub fn route_plan(&mut self, route_plan: Vec<RoutePlanStep>) -> &mut Self {
        self.route_plan = Some(route_plan);
        self
    }

    #[inline(always)]
    pub fn in_amount(&mut self, in_amount: u64) -> &mut Self {
        self.in_amount = Some(in_amount);
        self
    }

    #[inline(always)]
    pub fn quoted_out_amount(&mut self, quoted_out_amount: u64) -> &mut Self {
        self.quoted_out_amount = Some(quoted_out_amount);
        self
    }

    #[inline(always)]
    pub fn slippage_bps(&mut self, slippage_bps: u16) -> &mut Self {
        self.slippage_bps = Some(slippage_bps);
        self
    }

    #[inline(always)]
    pub fn platform_fee_bps(&mut self, platform_fee_bps: u8) -> &mut Self {
        self.platform_fee_bps = Some(platform_fee_bps);
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
        let accounts = SharedAccountsRoute {
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            program_authority: self
                .program_authority
                .expect("program_authority is not set"),
            user_transfer_authority: self
                .user_transfer_authority
                .expect("user_transfer_authority is not set"),
            source_token_account: self
                .source_token_account
                .expect("source_token_account is not set"),
            program_source_token_account: self
                .program_source_token_account
                .expect("program_source_token_account is not set"),
            program_destination_token_account: self
                .program_destination_token_account
                .expect("program_destination_token_account is not set"),
            destination_token_account: self
                .destination_token_account
                .expect("destination_token_account is not set"),
            source_mint: self.source_mint.expect("source_mint is not set"),
            destination_mint: self.destination_mint.expect("destination_mint is not set"),
            platform_fee_account: self.platform_fee_account,
            token2022_program: self.token2022_program,
            event_authority: self.event_authority.unwrap_or(solana_program::pubkey!(
                "D8cy77BBepLMngZx6ZukaTff5hCt1HrWyKk3Hnd9oitf"
            )),
            program: self.program.expect("program is not set"),
        };
        let args = SharedAccountsRouteInstructionArgs {
            id: self.id.clone().expect("id is not set"),
            route_plan: self.route_plan.clone().expect("route_plan is not set"),
            in_amount: self.in_amount.clone().expect("in_amount is not set"),
            quoted_out_amount: self
                .quoted_out_amount
                .clone()
                .expect("quoted_out_amount is not set"),
            slippage_bps: self.slippage_bps.clone().expect("slippage_bps is not set"),
            platform_fee_bps: self
                .platform_fee_bps
                .clone()
                .expect("platform_fee_bps is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `shared_accounts_route` CPI accounts.
pub struct SharedAccountsRouteCpiAccounts<'a, 'b> {
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub program_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_transfer_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub source_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub program_source_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub program_destination_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub destination_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub source_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub destination_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub platform_fee_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub token2022_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `shared_accounts_route` CPI instruction.
pub struct SharedAccountsRouteCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub program_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_transfer_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub source_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub program_source_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub program_destination_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub destination_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub source_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub destination_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub platform_fee_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub token2022_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: SharedAccountsRouteInstructionArgs,
}

impl<'a, 'b> SharedAccountsRouteCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: SharedAccountsRouteCpiAccounts<'a, 'b>,
        args: SharedAccountsRouteInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            token_program: accounts.token_program,
            program_authority: accounts.program_authority,
            user_transfer_authority: accounts.user_transfer_authority,
            source_token_account: accounts.source_token_account,
            program_source_token_account: accounts.program_source_token_account,
            program_destination_token_account: accounts.program_destination_token_account,
            destination_token_account: accounts.destination_token_account,
            source_mint: accounts.source_mint,
            destination_mint: accounts.destination_mint,
            platform_fee_account: accounts.platform_fee_account,
            token2022_program: accounts.token2022_program,
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
        let mut accounts = Vec::with_capacity(13 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.program_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.user_transfer_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.source_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.program_source_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.program_destination_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.destination_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.source_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.destination_mint.key,
            false,
        ));
        if let Some(platform_fee_account) = self.platform_fee_account {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *platform_fee_account.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::JUPITER_ID,
                false,
            ));
        }
        if let Some(token2022_program) = self.token2022_program {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *token2022_program.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::JUPITER_ID,
                false,
            ));
        }
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
        let mut data = borsh::to_vec(&SharedAccountsRouteInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(14 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.program_authority.clone());
        account_infos.push(self.user_transfer_authority.clone());
        account_infos.push(self.source_token_account.clone());
        account_infos.push(self.program_source_token_account.clone());
        account_infos.push(self.program_destination_token_account.clone());
        account_infos.push(self.destination_token_account.clone());
        account_infos.push(self.source_mint.clone());
        account_infos.push(self.destination_mint.clone());
        if let Some(platform_fee_account) = self.platform_fee_account {
            account_infos.push(platform_fee_account.clone());
        }
        if let Some(token2022_program) = self.token2022_program {
            account_infos.push(token2022_program.clone());
        }
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

/// Instruction builder for `SharedAccountsRoute` via CPI.
///
/// ### Accounts:
///
///   0. `[]` token_program
///   1. `[]` program_authority
///   2. `[signer]` user_transfer_authority
///   3. `[writable]` source_token_account
///   4. `[writable]` program_source_token_account
///   5. `[writable]` program_destination_token_account
///   6. `[writable]` destination_token_account
///   7. `[]` source_mint
///   8. `[]` destination_mint
///   9. `[writable, optional]` platform_fee_account
///   10. `[optional]` token2022_program
///   11. `[]` event_authority
///   12. `[]` program
#[derive(Clone, Debug)]
pub struct SharedAccountsRouteCpiBuilder<'a, 'b> {
    instruction: Box<SharedAccountsRouteCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SharedAccountsRouteCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SharedAccountsRouteCpiBuilderInstruction {
            __program: program,
            token_program: None,
            program_authority: None,
            user_transfer_authority: None,
            source_token_account: None,
            program_source_token_account: None,
            program_destination_token_account: None,
            destination_token_account: None,
            source_mint: None,
            destination_mint: None,
            platform_fee_account: None,
            token2022_program: None,
            event_authority: None,
            program: None,
            id: None,
            route_plan: None,
            in_amount: None,
            quoted_out_amount: None,
            slippage_bps: None,
            platform_fee_bps: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
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
    pub fn program_authority(
        &mut self,
        program_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program_authority = Some(program_authority);
        self
    }

    #[inline(always)]
    pub fn user_transfer_authority(
        &mut self,
        user_transfer_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_transfer_authority = Some(user_transfer_authority);
        self
    }

    #[inline(always)]
    pub fn source_token_account(
        &mut self,
        source_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.source_token_account = Some(source_token_account);
        self
    }

    #[inline(always)]
    pub fn program_source_token_account(
        &mut self,
        program_source_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program_source_token_account = Some(program_source_token_account);
        self
    }

    #[inline(always)]
    pub fn program_destination_token_account(
        &mut self,
        program_destination_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program_destination_token_account =
            Some(program_destination_token_account);
        self
    }

    #[inline(always)]
    pub fn destination_token_account(
        &mut self,
        destination_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.destination_token_account = Some(destination_token_account);
        self
    }

    #[inline(always)]
    pub fn source_mint(
        &mut self,
        source_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.source_mint = Some(source_mint);
        self
    }

    #[inline(always)]
    pub fn destination_mint(
        &mut self,
        destination_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.destination_mint = Some(destination_mint);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn platform_fee_account(
        &mut self,
        platform_fee_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.platform_fee_account = platform_fee_account;
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn token2022_program(
        &mut self,
        token2022_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.token2022_program = token2022_program;
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
    pub fn id(&mut self, id: u8) -> &mut Self {
        self.instruction.id = Some(id);
        self
    }

    #[inline(always)]
    pub fn route_plan(&mut self, route_plan: Vec<RoutePlanStep>) -> &mut Self {
        self.instruction.route_plan = Some(route_plan);
        self
    }

    #[inline(always)]
    pub fn in_amount(&mut self, in_amount: u64) -> &mut Self {
        self.instruction.in_amount = Some(in_amount);
        self
    }

    #[inline(always)]
    pub fn quoted_out_amount(&mut self, quoted_out_amount: u64) -> &mut Self {
        self.instruction.quoted_out_amount = Some(quoted_out_amount);
        self
    }

    #[inline(always)]
    pub fn slippage_bps(&mut self, slippage_bps: u16) -> &mut Self {
        self.instruction.slippage_bps = Some(slippage_bps);
        self
    }

    #[inline(always)]
    pub fn platform_fee_bps(&mut self, platform_fee_bps: u8) -> &mut Self {
        self.instruction.platform_fee_bps = Some(platform_fee_bps);
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
        let args = SharedAccountsRouteInstructionArgs {
            id: self.instruction.id.clone().expect("id is not set"),
            route_plan: self
                .instruction
                .route_plan
                .clone()
                .expect("route_plan is not set"),
            in_amount: self
                .instruction
                .in_amount
                .clone()
                .expect("in_amount is not set"),
            quoted_out_amount: self
                .instruction
                .quoted_out_amount
                .clone()
                .expect("quoted_out_amount is not set"),
            slippage_bps: self
                .instruction
                .slippage_bps
                .clone()
                .expect("slippage_bps is not set"),
            platform_fee_bps: self
                .instruction
                .platform_fee_bps
                .clone()
                .expect("platform_fee_bps is not set"),
        };
        let instruction = SharedAccountsRouteCpi {
            __program: self.instruction.__program,

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            program_authority: self
                .instruction
                .program_authority
                .expect("program_authority is not set"),

            user_transfer_authority: self
                .instruction
                .user_transfer_authority
                .expect("user_transfer_authority is not set"),

            source_token_account: self
                .instruction
                .source_token_account
                .expect("source_token_account is not set"),

            program_source_token_account: self
                .instruction
                .program_source_token_account
                .expect("program_source_token_account is not set"),

            program_destination_token_account: self
                .instruction
                .program_destination_token_account
                .expect("program_destination_token_account is not set"),

            destination_token_account: self
                .instruction
                .destination_token_account
                .expect("destination_token_account is not set"),

            source_mint: self
                .instruction
                .source_mint
                .expect("source_mint is not set"),

            destination_mint: self
                .instruction
                .destination_mint
                .expect("destination_mint is not set"),

            platform_fee_account: self.instruction.platform_fee_account,

            token2022_program: self.instruction.token2022_program,

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
struct SharedAccountsRouteCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_transfer_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    source_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program_source_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program_destination_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    destination_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    source_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    destination_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    platform_fee_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token2022_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    id: Option<u8>,
    route_plan: Option<Vec<RoutePlanStep>>,
    in_amount: Option<u64>,
    quoted_out_amount: Option<u64>,
    slippage_bps: Option<u16>,
    platform_fee_bps: Option<u8>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
