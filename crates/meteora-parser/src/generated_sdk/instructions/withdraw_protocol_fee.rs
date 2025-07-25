//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

use crate::generated::types::RemainingAccountsInfo;

/// Accounts.
#[derive(Debug)]
pub struct WithdrawProtocolFee {
    pub lb_pair: solana_pubkey::Pubkey,

    pub reserve_x: solana_pubkey::Pubkey,

    pub reserve_y: solana_pubkey::Pubkey,

    pub token_x_mint: solana_pubkey::Pubkey,

    pub token_y_mint: solana_pubkey::Pubkey,

    pub receiver_token_x: solana_pubkey::Pubkey,

    pub receiver_token_y: solana_pubkey::Pubkey,

    pub claim_fee_operator: solana_pubkey::Pubkey,
    /// operator
    pub operator: solana_pubkey::Pubkey,

    pub token_x_program: solana_pubkey::Pubkey,

    pub token_y_program: solana_pubkey::Pubkey,

    pub memo_program: solana_pubkey::Pubkey,
}

impl WithdrawProtocolFee {
    pub fn instruction(
        &self,
        args: WithdrawProtocolFeeInstructionArgs,
    ) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: WithdrawProtocolFeeInstructionArgs,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(self.lb_pair, false));
        accounts.push(solana_instruction::AccountMeta::new(self.reserve_x, false));
        accounts.push(solana_instruction::AccountMeta::new(self.reserve_y, false));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_x_mint,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_y_mint,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.receiver_token_x,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.receiver_token_y,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.claim_fee_operator,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.operator,
            true,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_x_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_y_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.memo_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&WithdrawProtocolFeeInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawProtocolFeeInstructionData {
    discriminator: [u8; 8],
}

impl WithdrawProtocolFeeInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [158, 201, 158, 189, 33, 93, 162, 103],
        }
    }
}

impl Default for WithdrawProtocolFeeInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawProtocolFeeInstructionArgs {
    pub amount_x: u64,
    pub amount_y: u64,
    pub remaining_accounts_info: RemainingAccountsInfo,
}

/// Instruction builder for `WithdrawProtocolFee`.
///
/// ### Accounts:
///
///   0. `[writable]` lb_pair
///   1. `[writable]` reserve_x
///   2. `[writable]` reserve_y
///   3. `[]` token_x_mint
///   4. `[]` token_y_mint
///   5. `[writable]` receiver_token_x
///   6. `[writable]` receiver_token_y
///   7. `[]` claim_fee_operator
///   8. `[signer]` operator
///   9. `[]` token_x_program
///   10. `[]` token_y_program
///   11. `[]` memo_program
#[derive(Clone, Debug, Default)]
pub struct WithdrawProtocolFeeBuilder {
    lb_pair: Option<solana_pubkey::Pubkey>,
    reserve_x: Option<solana_pubkey::Pubkey>,
    reserve_y: Option<solana_pubkey::Pubkey>,
    token_x_mint: Option<solana_pubkey::Pubkey>,
    token_y_mint: Option<solana_pubkey::Pubkey>,
    receiver_token_x: Option<solana_pubkey::Pubkey>,
    receiver_token_y: Option<solana_pubkey::Pubkey>,
    claim_fee_operator: Option<solana_pubkey::Pubkey>,
    operator: Option<solana_pubkey::Pubkey>,
    token_x_program: Option<solana_pubkey::Pubkey>,
    token_y_program: Option<solana_pubkey::Pubkey>,
    memo_program: Option<solana_pubkey::Pubkey>,
    amount_x: Option<u64>,
    amount_y: Option<u64>,
    remaining_accounts_info: Option<RemainingAccountsInfo>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl WithdrawProtocolFeeBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn lb_pair(&mut self, lb_pair: solana_pubkey::Pubkey) -> &mut Self {
        self.lb_pair = Some(lb_pair);
        self
    }

    #[inline(always)]
    pub fn reserve_x(&mut self, reserve_x: solana_pubkey::Pubkey) -> &mut Self {
        self.reserve_x = Some(reserve_x);
        self
    }

    #[inline(always)]
    pub fn reserve_y(&mut self, reserve_y: solana_pubkey::Pubkey) -> &mut Self {
        self.reserve_y = Some(reserve_y);
        self
    }

    #[inline(always)]
    pub fn token_x_mint(&mut self, token_x_mint: solana_pubkey::Pubkey) -> &mut Self {
        self.token_x_mint = Some(token_x_mint);
        self
    }

    #[inline(always)]
    pub fn token_y_mint(&mut self, token_y_mint: solana_pubkey::Pubkey) -> &mut Self {
        self.token_y_mint = Some(token_y_mint);
        self
    }

    #[inline(always)]
    pub fn receiver_token_x(&mut self, receiver_token_x: solana_pubkey::Pubkey) -> &mut Self {
        self.receiver_token_x = Some(receiver_token_x);
        self
    }

    #[inline(always)]
    pub fn receiver_token_y(&mut self, receiver_token_y: solana_pubkey::Pubkey) -> &mut Self {
        self.receiver_token_y = Some(receiver_token_y);
        self
    }

    #[inline(always)]
    pub fn claim_fee_operator(&mut self, claim_fee_operator: solana_pubkey::Pubkey) -> &mut Self {
        self.claim_fee_operator = Some(claim_fee_operator);
        self
    }

    /// operator
    #[inline(always)]
    pub fn operator(&mut self, operator: solana_pubkey::Pubkey) -> &mut Self {
        self.operator = Some(operator);
        self
    }

    #[inline(always)]
    pub fn token_x_program(&mut self, token_x_program: solana_pubkey::Pubkey) -> &mut Self {
        self.token_x_program = Some(token_x_program);
        self
    }

    #[inline(always)]
    pub fn token_y_program(&mut self, token_y_program: solana_pubkey::Pubkey) -> &mut Self {
        self.token_y_program = Some(token_y_program);
        self
    }

    #[inline(always)]
    pub fn memo_program(&mut self, memo_program: solana_pubkey::Pubkey) -> &mut Self {
        self.memo_program = Some(memo_program);
        self
    }

    #[inline(always)]
    pub fn amount_x(&mut self, amount_x: u64) -> &mut Self {
        self.amount_x = Some(amount_x);
        self
    }

    #[inline(always)]
    pub fn amount_y(&mut self, amount_y: u64) -> &mut Self {
        self.amount_y = Some(amount_y);
        self
    }

    #[inline(always)]
    pub fn remaining_accounts_info(
        &mut self,
        remaining_accounts_info: RemainingAccountsInfo,
    ) -> &mut Self {
        self.remaining_accounts_info = Some(remaining_accounts_info);
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
        let accounts = WithdrawProtocolFee {
            lb_pair: self.lb_pair.expect("lb_pair is not set"),
            reserve_x: self.reserve_x.expect("reserve_x is not set"),
            reserve_y: self.reserve_y.expect("reserve_y is not set"),
            token_x_mint: self.token_x_mint.expect("token_x_mint is not set"),
            token_y_mint: self.token_y_mint.expect("token_y_mint is not set"),
            receiver_token_x: self.receiver_token_x.expect("receiver_token_x is not set"),
            receiver_token_y: self.receiver_token_y.expect("receiver_token_y is not set"),
            claim_fee_operator: self
                .claim_fee_operator
                .expect("claim_fee_operator is not set"),
            operator: self.operator.expect("operator is not set"),
            token_x_program: self.token_x_program.expect("token_x_program is not set"),
            token_y_program: self.token_y_program.expect("token_y_program is not set"),
            memo_program: self.memo_program.expect("memo_program is not set"),
        };
        let args = WithdrawProtocolFeeInstructionArgs {
            amount_x: self.amount_x.clone().expect("amount_x is not set"),
            amount_y: self.amount_y.clone().expect("amount_y is not set"),
            remaining_accounts_info: self
                .remaining_accounts_info
                .clone()
                .expect("remaining_accounts_info is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `withdraw_protocol_fee` CPI accounts.
pub struct WithdrawProtocolFeeCpiAccounts<'a, 'b> {
    pub lb_pair: &'b solana_account_info::AccountInfo<'a>,

    pub reserve_x: &'b solana_account_info::AccountInfo<'a>,

    pub reserve_y: &'b solana_account_info::AccountInfo<'a>,

    pub token_x_mint: &'b solana_account_info::AccountInfo<'a>,

    pub token_y_mint: &'b solana_account_info::AccountInfo<'a>,

    pub receiver_token_x: &'b solana_account_info::AccountInfo<'a>,

    pub receiver_token_y: &'b solana_account_info::AccountInfo<'a>,

    pub claim_fee_operator: &'b solana_account_info::AccountInfo<'a>,
    /// operator
    pub operator: &'b solana_account_info::AccountInfo<'a>,

    pub token_x_program: &'b solana_account_info::AccountInfo<'a>,

    pub token_y_program: &'b solana_account_info::AccountInfo<'a>,

    pub memo_program: &'b solana_account_info::AccountInfo<'a>,
}

/// `withdraw_protocol_fee` CPI instruction.
pub struct WithdrawProtocolFeeCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_account_info::AccountInfo<'a>,

    pub lb_pair: &'b solana_account_info::AccountInfo<'a>,

    pub reserve_x: &'b solana_account_info::AccountInfo<'a>,

    pub reserve_y: &'b solana_account_info::AccountInfo<'a>,

    pub token_x_mint: &'b solana_account_info::AccountInfo<'a>,

    pub token_y_mint: &'b solana_account_info::AccountInfo<'a>,

    pub receiver_token_x: &'b solana_account_info::AccountInfo<'a>,

    pub receiver_token_y: &'b solana_account_info::AccountInfo<'a>,

    pub claim_fee_operator: &'b solana_account_info::AccountInfo<'a>,
    /// operator
    pub operator: &'b solana_account_info::AccountInfo<'a>,

    pub token_x_program: &'b solana_account_info::AccountInfo<'a>,

    pub token_y_program: &'b solana_account_info::AccountInfo<'a>,

    pub memo_program: &'b solana_account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: WithdrawProtocolFeeInstructionArgs,
}

impl<'a, 'b> WithdrawProtocolFeeCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: WithdrawProtocolFeeCpiAccounts<'a, 'b>,
        args: WithdrawProtocolFeeInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            lb_pair: accounts.lb_pair,
            reserve_x: accounts.reserve_x,
            reserve_y: accounts.reserve_y,
            token_x_mint: accounts.token_x_mint,
            token_y_mint: accounts.token_y_mint,
            receiver_token_x: accounts.receiver_token_x,
            receiver_token_y: accounts.receiver_token_y,
            claim_fee_operator: accounts.claim_fee_operator,
            operator: accounts.operator,
            token_x_program: accounts.token_x_program,
            token_y_program: accounts.token_y_program,
            memo_program: accounts.memo_program,
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
        let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(
            *self.lb_pair.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.reserve_x.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.reserve_y.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_x_mint.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_y_mint.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.receiver_token_x.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.receiver_token_y.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.claim_fee_operator.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.operator.key,
            true,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_x_program.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_y_program.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.memo_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&WithdrawProtocolFeeInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(13 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.lb_pair.clone());
        account_infos.push(self.reserve_x.clone());
        account_infos.push(self.reserve_y.clone());
        account_infos.push(self.token_x_mint.clone());
        account_infos.push(self.token_y_mint.clone());
        account_infos.push(self.receiver_token_x.clone());
        account_infos.push(self.receiver_token_y.clone());
        account_infos.push(self.claim_fee_operator.clone());
        account_infos.push(self.operator.clone());
        account_infos.push(self.token_x_program.clone());
        account_infos.push(self.token_y_program.clone());
        account_infos.push(self.memo_program.clone());
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

/// Instruction builder for `WithdrawProtocolFee` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` lb_pair
///   1. `[writable]` reserve_x
///   2. `[writable]` reserve_y
///   3. `[]` token_x_mint
///   4. `[]` token_y_mint
///   5. `[writable]` receiver_token_x
///   6. `[writable]` receiver_token_y
///   7. `[]` claim_fee_operator
///   8. `[signer]` operator
///   9. `[]` token_x_program
///   10. `[]` token_y_program
///   11. `[]` memo_program
#[derive(Clone, Debug)]
pub struct WithdrawProtocolFeeCpiBuilder<'a, 'b> {
    instruction: Box<WithdrawProtocolFeeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> WithdrawProtocolFeeCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(WithdrawProtocolFeeCpiBuilderInstruction {
            __program: program,
            lb_pair: None,
            reserve_x: None,
            reserve_y: None,
            token_x_mint: None,
            token_y_mint: None,
            receiver_token_x: None,
            receiver_token_y: None,
            claim_fee_operator: None,
            operator: None,
            token_x_program: None,
            token_y_program: None,
            memo_program: None,
            amount_x: None,
            amount_y: None,
            remaining_accounts_info: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn lb_pair(&mut self, lb_pair: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.lb_pair = Some(lb_pair);
        self
    }

    #[inline(always)]
    pub fn reserve_x(&mut self, reserve_x: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.reserve_x = Some(reserve_x);
        self
    }

    #[inline(always)]
    pub fn reserve_y(&mut self, reserve_y: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.reserve_y = Some(reserve_y);
        self
    }

    #[inline(always)]
    pub fn token_x_mint(
        &mut self,
        token_x_mint: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_x_mint = Some(token_x_mint);
        self
    }

    #[inline(always)]
    pub fn token_y_mint(
        &mut self,
        token_y_mint: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_y_mint = Some(token_y_mint);
        self
    }

    #[inline(always)]
    pub fn receiver_token_x(
        &mut self,
        receiver_token_x: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.receiver_token_x = Some(receiver_token_x);
        self
    }

    #[inline(always)]
    pub fn receiver_token_y(
        &mut self,
        receiver_token_y: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.receiver_token_y = Some(receiver_token_y);
        self
    }

    #[inline(always)]
    pub fn claim_fee_operator(
        &mut self,
        claim_fee_operator: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.claim_fee_operator = Some(claim_fee_operator);
        self
    }

    /// operator
    #[inline(always)]
    pub fn operator(&mut self, operator: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.operator = Some(operator);
        self
    }

    #[inline(always)]
    pub fn token_x_program(
        &mut self,
        token_x_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_x_program = Some(token_x_program);
        self
    }

    #[inline(always)]
    pub fn token_y_program(
        &mut self,
        token_y_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_y_program = Some(token_y_program);
        self
    }

    #[inline(always)]
    pub fn memo_program(
        &mut self,
        memo_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.memo_program = Some(memo_program);
        self
    }

    #[inline(always)]
    pub fn amount_x(&mut self, amount_x: u64) -> &mut Self {
        self.instruction.amount_x = Some(amount_x);
        self
    }

    #[inline(always)]
    pub fn amount_y(&mut self, amount_y: u64) -> &mut Self {
        self.instruction.amount_y = Some(amount_y);
        self
    }

    #[inline(always)]
    pub fn remaining_accounts_info(
        &mut self,
        remaining_accounts_info: RemainingAccountsInfo,
    ) -> &mut Self {
        self.instruction.remaining_accounts_info = Some(remaining_accounts_info);
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
        let args = WithdrawProtocolFeeInstructionArgs {
            amount_x: self
                .instruction
                .amount_x
                .clone()
                .expect("amount_x is not set"),
            amount_y: self
                .instruction
                .amount_y
                .clone()
                .expect("amount_y is not set"),
            remaining_accounts_info: self
                .instruction
                .remaining_accounts_info
                .clone()
                .expect("remaining_accounts_info is not set"),
        };
        let instruction = WithdrawProtocolFeeCpi {
            __program: self.instruction.__program,

            lb_pair: self.instruction.lb_pair.expect("lb_pair is not set"),

            reserve_x: self.instruction.reserve_x.expect("reserve_x is not set"),

            reserve_y: self.instruction.reserve_y.expect("reserve_y is not set"),

            token_x_mint: self
                .instruction
                .token_x_mint
                .expect("token_x_mint is not set"),

            token_y_mint: self
                .instruction
                .token_y_mint
                .expect("token_y_mint is not set"),

            receiver_token_x: self
                .instruction
                .receiver_token_x
                .expect("receiver_token_x is not set"),

            receiver_token_y: self
                .instruction
                .receiver_token_y
                .expect("receiver_token_y is not set"),

            claim_fee_operator: self
                .instruction
                .claim_fee_operator
                .expect("claim_fee_operator is not set"),

            operator: self.instruction.operator.expect("operator is not set"),

            token_x_program: self
                .instruction
                .token_x_program
                .expect("token_x_program is not set"),

            token_y_program: self
                .instruction
                .token_y_program
                .expect("token_y_program is not set"),

            memo_program: self
                .instruction
                .memo_program
                .expect("memo_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct WithdrawProtocolFeeCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    lb_pair: Option<&'b solana_account_info::AccountInfo<'a>>,
    reserve_x: Option<&'b solana_account_info::AccountInfo<'a>>,
    reserve_y: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_x_mint: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_y_mint: Option<&'b solana_account_info::AccountInfo<'a>>,
    receiver_token_x: Option<&'b solana_account_info::AccountInfo<'a>>,
    receiver_token_y: Option<&'b solana_account_info::AccountInfo<'a>>,
    claim_fee_operator: Option<&'b solana_account_info::AccountInfo<'a>>,
    operator: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_x_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_y_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    memo_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    amount_x: Option<u64>,
    amount_y: Option<u64>,
    remaining_accounts_info: Option<RemainingAccountsInfo>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
