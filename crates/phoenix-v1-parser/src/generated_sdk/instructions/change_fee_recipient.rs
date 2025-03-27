//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshSerialize;
use borsh::BorshDeserialize;

/// Accounts.
#[derive(Debug)]
pub struct ChangeFeeRecipient {
            /// Phoenix program

    
              
          pub phoenix_program: solana_program::pubkey::Pubkey,
                /// Phoenix log authority

    
              
          pub log_authority: solana_program::pubkey::Pubkey,
                /// This account holds the market state

    
              
          pub market: solana_program::pubkey::Pubkey,
                /// The market_authority account must sign to change the free recipient

    
              
          pub market_authority: solana_program::pubkey::Pubkey,
                /// New fee recipient

    
              
          pub new_fee_recipient: solana_program::pubkey::Pubkey,
      }

impl ChangeFeeRecipient {
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(&[])
  }
  #[allow(clippy::arithmetic_side_effects)]
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(5+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.phoenix_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.log_authority,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.market,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.market_authority,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.new_fee_recipient,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let data = borsh::to_vec(&ChangeFeeRecipientInstructionData::new()).unwrap();
    
    solana_program::instruction::Instruction {
      program_id: crate::PHOENIX_V1_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct ChangeFeeRecipientInstructionData {
            discriminator: u8,
      }

impl ChangeFeeRecipientInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: 109,
                  }
  }
}

impl Default for ChangeFeeRecipientInstructionData {
  fn default() -> Self {
    Self::new()
  }
}



/// Instruction builder for `ChangeFeeRecipient`.
///
/// ### Accounts:
///
          ///   0. `[]` phoenix_program
          ///   1. `[]` log_authority
                ///   2. `[writable]` market
                ///   3. `[signer]` market_authority
          ///   4. `[]` new_fee_recipient
#[derive(Clone, Debug, Default)]
pub struct ChangeFeeRecipientBuilder {
            phoenix_program: Option<solana_program::pubkey::Pubkey>,
                log_authority: Option<solana_program::pubkey::Pubkey>,
                market: Option<solana_program::pubkey::Pubkey>,
                market_authority: Option<solana_program::pubkey::Pubkey>,
                new_fee_recipient: Option<solana_program::pubkey::Pubkey>,
                __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ChangeFeeRecipientBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            /// Phoenix program
#[inline(always)]
    pub fn phoenix_program(&mut self, phoenix_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.phoenix_program = Some(phoenix_program);
                    self
    }
            /// Phoenix log authority
#[inline(always)]
    pub fn log_authority(&mut self, log_authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.log_authority = Some(log_authority);
                    self
    }
            /// This account holds the market state
#[inline(always)]
    pub fn market(&mut self, market: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.market = Some(market);
                    self
    }
            /// The market_authority account must sign to change the free recipient
#[inline(always)]
    pub fn market_authority(&mut self, market_authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.market_authority = Some(market_authority);
                    self
    }
            /// New fee recipient
#[inline(always)]
    pub fn new_fee_recipient(&mut self, new_fee_recipient: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.new_fee_recipient = Some(new_fee_recipient);
                    self
    }
            /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: solana_program::instruction::AccountMeta) -> &mut Self {
    self.__remaining_accounts.push(account);
    self
  }
  /// Add additional accounts to the instruction.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[solana_program::instruction::AccountMeta]) -> &mut Self {
    self.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[allow(clippy::clone_on_copy)]
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    let accounts = ChangeFeeRecipient {
                              phoenix_program: self.phoenix_program.expect("phoenix_program is not set"),
                                        log_authority: self.log_authority.expect("log_authority is not set"),
                                        market: self.market.expect("market is not set"),
                                        market_authority: self.market_authority.expect("market_authority is not set"),
                                        new_fee_recipient: self.new_fee_recipient.expect("new_fee_recipient is not set"),
                      };
    
    accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
  }
}

  /// `change_fee_recipient` CPI accounts.
  pub struct ChangeFeeRecipientCpiAccounts<'a, 'b> {
                  /// Phoenix program

      
                    
              pub phoenix_program: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Phoenix log authority

      
                    
              pub log_authority: &'b solana_program::account_info::AccountInfo<'a>,
                        /// This account holds the market state

      
                    
              pub market: &'b solana_program::account_info::AccountInfo<'a>,
                        /// The market_authority account must sign to change the free recipient

      
                    
              pub market_authority: &'b solana_program::account_info::AccountInfo<'a>,
                        /// New fee recipient

      
                    
              pub new_fee_recipient: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `change_fee_recipient` CPI instruction.
pub struct ChangeFeeRecipientCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
            /// Phoenix program

    
              
          pub phoenix_program: &'b solana_program::account_info::AccountInfo<'a>,
                /// Phoenix log authority

    
              
          pub log_authority: &'b solana_program::account_info::AccountInfo<'a>,
                /// This account holds the market state

    
              
          pub market: &'b solana_program::account_info::AccountInfo<'a>,
                /// The market_authority account must sign to change the free recipient

    
              
          pub market_authority: &'b solana_program::account_info::AccountInfo<'a>,
                /// New fee recipient

    
              
          pub new_fee_recipient: &'b solana_program::account_info::AccountInfo<'a>,
        }

impl<'a, 'b> ChangeFeeRecipientCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: ChangeFeeRecipientCpiAccounts<'a, 'b>,
          ) -> Self {
    Self {
      __program: program,
              phoenix_program: accounts.phoenix_program,
              log_authority: accounts.log_authority,
              market: accounts.market,
              market_authority: accounts.market_authority,
              new_fee_recipient: accounts.new_fee_recipient,
                }
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], &[])
  }
  #[inline(always)]
  pub fn invoke_with_remaining_accounts(&self, remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
  }
  #[inline(always)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
  }
  #[allow(clippy::arithmetic_side_effects)]
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed_with_remaining_accounts(
    &self,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]
  ) -> solana_program::entrypoint::ProgramResult {
    let mut accounts = Vec::with_capacity(5+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.phoenix_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.log_authority.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.market.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.market_authority.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.new_fee_recipient.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let data = borsh::to_vec(&ChangeFeeRecipientInstructionData::new()).unwrap();
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::PHOENIX_V1_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(6 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.phoenix_program.clone());
                        account_infos.push(self.log_authority.clone());
                        account_infos.push(self.market.clone());
                        account_infos.push(self.market_authority.clone());
                        account_infos.push(self.new_fee_recipient.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `ChangeFeeRecipient` via CPI.
///
/// ### Accounts:
///
          ///   0. `[]` phoenix_program
          ///   1. `[]` log_authority
                ///   2. `[writable]` market
                ///   3. `[signer]` market_authority
          ///   4. `[]` new_fee_recipient
#[derive(Clone, Debug)]
pub struct ChangeFeeRecipientCpiBuilder<'a, 'b> {
  instruction: Box<ChangeFeeRecipientCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ChangeFeeRecipientCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(ChangeFeeRecipientCpiBuilderInstruction {
      __program: program,
              phoenix_program: None,
              log_authority: None,
              market: None,
              market_authority: None,
              new_fee_recipient: None,
                                __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      /// Phoenix program
#[inline(always)]
    pub fn phoenix_program(&mut self, phoenix_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.phoenix_program = Some(phoenix_program);
                    self
    }
      /// Phoenix log authority
#[inline(always)]
    pub fn log_authority(&mut self, log_authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.log_authority = Some(log_authority);
                    self
    }
      /// This account holds the market state
#[inline(always)]
    pub fn market(&mut self, market: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.market = Some(market);
                    self
    }
      /// The market_authority account must sign to change the free recipient
#[inline(always)]
    pub fn market_authority(&mut self, market_authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.market_authority = Some(market_authority);
                    self
    }
      /// New fee recipient
#[inline(always)]
    pub fn new_fee_recipient(&mut self, new_fee_recipient: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.new_fee_recipient = Some(new_fee_recipient);
                    self
    }
            /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: &'b solana_program::account_info::AccountInfo<'a>, is_writable: bool, is_signer: bool) -> &mut Self {
    self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
    self
  }
  /// Add additional accounts to the instruction.
  ///
  /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
  /// and a `bool` indicating whether the account is a signer or not.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> &mut Self {
    self.instruction.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed(&[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
        let instruction = ChangeFeeRecipientCpi {
        __program: self.instruction.__program,
                  
          phoenix_program: self.instruction.phoenix_program.expect("phoenix_program is not set"),
                  
          log_authority: self.instruction.log_authority.expect("log_authority is not set"),
                  
          market: self.instruction.market.expect("market is not set"),
                  
          market_authority: self.instruction.market_authority.expect("market_authority is not set"),
                  
          new_fee_recipient: self.instruction.new_fee_recipient.expect("new_fee_recipient is not set"),
                    };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct ChangeFeeRecipientCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            phoenix_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                log_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                market_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                new_fee_recipient: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

