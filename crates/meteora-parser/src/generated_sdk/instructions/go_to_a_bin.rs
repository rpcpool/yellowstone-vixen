//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct GoToABin {
    pub lb_pair: solana_pubkey::Pubkey,

    pub bin_array_bitmap_extension: Option<solana_pubkey::Pubkey>,

    pub from_bin_array: Option<solana_pubkey::Pubkey>,

    pub to_bin_array: Option<solana_pubkey::Pubkey>,

    pub event_authority: solana_pubkey::Pubkey,

    pub program: solana_pubkey::Pubkey,
}

impl GoToABin {
    pub fn instruction(&self, args: GoToABinInstructionArgs) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }

    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: GoToABinInstructionArgs,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(self.lb_pair, false));
        if let Some(bin_array_bitmap_extension) = self.bin_array_bitmap_extension {
            accounts.push(solana_instruction::AccountMeta::new_readonly(
                bin_array_bitmap_extension,
                false,
            ));
        } else {
            accounts.push(solana_instruction::AccountMeta::new_readonly(
                crate::LB_CLMM_ID,
                false,
            ));
        }
        if let Some(from_bin_array) = self.from_bin_array {
            accounts.push(solana_instruction::AccountMeta::new_readonly(
                from_bin_array,
                false,
            ));
        } else {
            accounts.push(solana_instruction::AccountMeta::new_readonly(
                crate::LB_CLMM_ID,
                false,
            ));
        }
        if let Some(to_bin_array) = self.to_bin_array {
            accounts.push(solana_instruction::AccountMeta::new_readonly(
                to_bin_array,
                false,
            ));
        } else {
            accounts.push(solana_instruction::AccountMeta::new_readonly(
                crate::LB_CLMM_ID,
                false,
            ));
        }
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.event_authority,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&GoToABinInstructionData::new()).unwrap();
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
pub struct GoToABinInstructionData {
    discriminator: [u8; 8],
}

impl GoToABinInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [146, 72, 174, 224, 40, 253, 84, 174],
        }
    }
}

impl Default for GoToABinInstructionData {
    fn default() -> Self { Self::new() }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GoToABinInstructionArgs {
    pub bin_id: i32,
}

/// Instruction builder for `GoToABin`.
///
/// ### Accounts:
///
///   0. `[writable]` lb_pair
///   1. `[optional]` bin_array_bitmap_extension
///   2. `[optional]` from_bin_array
///   3. `[optional]` to_bin_array
///   4. `[]` event_authority
///   5. `[]` program
#[derive(Clone, Debug, Default)]
pub struct GoToABinBuilder {
    lb_pair: Option<solana_pubkey::Pubkey>,
    bin_array_bitmap_extension: Option<solana_pubkey::Pubkey>,
    from_bin_array: Option<solana_pubkey::Pubkey>,
    to_bin_array: Option<solana_pubkey::Pubkey>,
    event_authority: Option<solana_pubkey::Pubkey>,
    program: Option<solana_pubkey::Pubkey>,
    bin_id: Option<i32>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl GoToABinBuilder {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn lb_pair(&mut self, lb_pair: solana_pubkey::Pubkey) -> &mut Self {
        self.lb_pair = Some(lb_pair);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn bin_array_bitmap_extension(
        &mut self,
        bin_array_bitmap_extension: Option<solana_pubkey::Pubkey>,
    ) -> &mut Self {
        self.bin_array_bitmap_extension = bin_array_bitmap_extension;
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn from_bin_array(&mut self, from_bin_array: Option<solana_pubkey::Pubkey>) -> &mut Self {
        self.from_bin_array = from_bin_array;
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn to_bin_array(&mut self, to_bin_array: Option<solana_pubkey::Pubkey>) -> &mut Self {
        self.to_bin_array = to_bin_array;
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

    #[inline(always)]
    pub fn bin_id(&mut self, bin_id: i32) -> &mut Self {
        self.bin_id = Some(bin_id);
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
        let accounts = GoToABin {
            lb_pair: self.lb_pair.expect("lb_pair is not set"),
            bin_array_bitmap_extension: self.bin_array_bitmap_extension,
            from_bin_array: self.from_bin_array,
            to_bin_array: self.to_bin_array,
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = GoToABinInstructionArgs {
            bin_id: self.bin_id.clone().expect("bin_id is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `go_to_a_bin` CPI accounts.
pub struct GoToABinCpiAccounts<'a, 'b> {
    pub lb_pair: &'b solana_account_info::AccountInfo<'a>,

    pub bin_array_bitmap_extension: Option<&'b solana_account_info::AccountInfo<'a>>,

    pub from_bin_array: Option<&'b solana_account_info::AccountInfo<'a>>,

    pub to_bin_array: Option<&'b solana_account_info::AccountInfo<'a>>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
}

/// `go_to_a_bin` CPI instruction.
pub struct GoToABinCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_account_info::AccountInfo<'a>,

    pub lb_pair: &'b solana_account_info::AccountInfo<'a>,

    pub bin_array_bitmap_extension: Option<&'b solana_account_info::AccountInfo<'a>>,

    pub from_bin_array: Option<&'b solana_account_info::AccountInfo<'a>>,

    pub to_bin_array: Option<&'b solana_account_info::AccountInfo<'a>>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: GoToABinInstructionArgs,
}

impl<'a, 'b> GoToABinCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: GoToABinCpiAccounts<'a, 'b>,
        args: GoToABinInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            lb_pair: accounts.lb_pair,
            bin_array_bitmap_extension: accounts.bin_array_bitmap_extension,
            from_bin_array: accounts.from_bin_array,
            to_bin_array: accounts.to_bin_array,
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
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(
            *self.lb_pair.key,
            false,
        ));
        if let Some(bin_array_bitmap_extension) = self.bin_array_bitmap_extension {
            accounts.push(solana_instruction::AccountMeta::new_readonly(
                *bin_array_bitmap_extension.key,
                false,
            ));
        } else {
            accounts.push(solana_instruction::AccountMeta::new_readonly(
                crate::LB_CLMM_ID,
                false,
            ));
        }
        if let Some(from_bin_array) = self.from_bin_array {
            accounts.push(solana_instruction::AccountMeta::new_readonly(
                *from_bin_array.key,
                false,
            ));
        } else {
            accounts.push(solana_instruction::AccountMeta::new_readonly(
                crate::LB_CLMM_ID,
                false,
            ));
        }
        if let Some(to_bin_array) = self.to_bin_array {
            accounts.push(solana_instruction::AccountMeta::new_readonly(
                *to_bin_array.key,
                false,
            ));
        } else {
            accounts.push(solana_instruction::AccountMeta::new_readonly(
                crate::LB_CLMM_ID,
                false,
            ));
        }
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
        let mut data = borsh::to_vec(&GoToABinInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.lb_pair.clone());
        if let Some(bin_array_bitmap_extension) = self.bin_array_bitmap_extension {
            account_infos.push(bin_array_bitmap_extension.clone());
        }
        if let Some(from_bin_array) = self.from_bin_array {
            account_infos.push(from_bin_array.clone());
        }
        if let Some(to_bin_array) = self.to_bin_array {
            account_infos.push(to_bin_array.clone());
        }
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

/// Instruction builder for `GoToABin` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` lb_pair
///   1. `[optional]` bin_array_bitmap_extension
///   2. `[optional]` from_bin_array
///   3. `[optional]` to_bin_array
///   4. `[]` event_authority
///   5. `[]` program
#[derive(Clone, Debug)]
pub struct GoToABinCpiBuilder<'a, 'b> {
    instruction: Box<GoToABinCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> GoToABinCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(GoToABinCpiBuilderInstruction {
            __program: program,
            lb_pair: None,
            bin_array_bitmap_extension: None,
            from_bin_array: None,
            to_bin_array: None,
            event_authority: None,
            program: None,
            bin_id: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }

    #[inline(always)]
    pub fn lb_pair(&mut self, lb_pair: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.lb_pair = Some(lb_pair);
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn bin_array_bitmap_extension(
        &mut self,
        bin_array_bitmap_extension: Option<&'b solana_account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.bin_array_bitmap_extension = bin_array_bitmap_extension;
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn from_bin_array(
        &mut self,
        from_bin_array: Option<&'b solana_account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.from_bin_array = from_bin_array;
        self
    }

    /// `[optional account]`
    #[inline(always)]
    pub fn to_bin_array(
        &mut self,
        to_bin_array: Option<&'b solana_account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.to_bin_array = to_bin_array;
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

    #[inline(always)]
    pub fn bin_id(&mut self, bin_id: i32) -> &mut Self {
        self.instruction.bin_id = Some(bin_id);
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
        let args = GoToABinInstructionArgs {
            bin_id: self.instruction.bin_id.clone().expect("bin_id is not set"),
        };
        let instruction = GoToABinCpi {
            __program: self.instruction.__program,

            lb_pair: self.instruction.lb_pair.expect("lb_pair is not set"),

            bin_array_bitmap_extension: self.instruction.bin_array_bitmap_extension,

            from_bin_array: self.instruction.from_bin_array,

            to_bin_array: self.instruction.to_bin_array,

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
struct GoToABinCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    lb_pair: Option<&'b solana_account_info::AccountInfo<'a>>,
    bin_array_bitmap_extension: Option<&'b solana_account_info::AccountInfo<'a>>,
    from_bin_array: Option<&'b solana_account_info::AccountInfo<'a>>,
    to_bin_array: Option<&'b solana_account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_account_info::AccountInfo<'a>>,
    program: Option<&'b solana_account_info::AccountInfo<'a>>,
    bin_id: Option<i32>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
