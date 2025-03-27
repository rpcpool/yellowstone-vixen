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
pub struct UpdateLendingMarket {
      
              
          pub lending_market_owner: solana_program::pubkey::Pubkey,
          
              
          pub lending_market: solana_program::pubkey::Pubkey,
      }

impl UpdateLendingMarket {
  pub fn instruction(&self, args: UpdateLendingMarketInstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::arithmetic_side_effects)]
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: UpdateLendingMarketInstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(2+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.lending_market_owner,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.lending_market,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = borsh::to_vec(&UpdateLendingMarketInstructionData::new()).unwrap();
          let mut args = borsh::to_vec(&args).unwrap();
      data.append(&mut args);
    
    solana_program::instruction::Instruction {
      program_id: crate::KAMINO_LENDING_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct UpdateLendingMarketInstructionData {
            discriminator: [u8; 8],
                  }

impl UpdateLendingMarketInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [209, 157, 53, 210, 97, 180, 31, 45],
                                              }
  }
}

impl Default for UpdateLendingMarketInstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct UpdateLendingMarketInstructionArgs {
                  pub mode: u64,
                pub value: [u8; 72],
      }


/// Instruction builder for `UpdateLendingMarket`.
///
/// ### Accounts:
///
                ///   0. `[signer]` lending_market_owner
                ///   1. `[writable]` lending_market
#[derive(Clone, Debug, Default)]
pub struct UpdateLendingMarketBuilder {
            lending_market_owner: Option<solana_program::pubkey::Pubkey>,
                lending_market: Option<solana_program::pubkey::Pubkey>,
                        mode: Option<u64>,
                value: Option<[u8; 72]>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UpdateLendingMarketBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn lending_market_owner(&mut self, lending_market_owner: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.lending_market_owner = Some(lending_market_owner);
                    self
    }
            #[inline(always)]
    pub fn lending_market(&mut self, lending_market: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.lending_market = Some(lending_market);
                    self
    }
                    #[inline(always)]
      pub fn mode(&mut self, mode: u64) -> &mut Self {
        self.mode = Some(mode);
        self
      }
                #[inline(always)]
      pub fn value(&mut self, value: [u8; 72]) -> &mut Self {
        self.value = Some(value);
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
    let accounts = UpdateLendingMarket {
                              lending_market_owner: self.lending_market_owner.expect("lending_market_owner is not set"),
                                        lending_market: self.lending_market.expect("lending_market is not set"),
                      };
          let args = UpdateLendingMarketInstructionArgs {
                                                              mode: self.mode.clone().expect("mode is not set"),
                                                                  value: self.value.clone().expect("value is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `update_lending_market` CPI accounts.
  pub struct UpdateLendingMarketCpiAccounts<'a, 'b> {
          
                    
              pub lending_market_owner: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub lending_market: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `update_lending_market` CPI instruction.
pub struct UpdateLendingMarketCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub lending_market_owner: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub lending_market: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: UpdateLendingMarketInstructionArgs,
  }

impl<'a, 'b> UpdateLendingMarketCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: UpdateLendingMarketCpiAccounts<'a, 'b>,
              args: UpdateLendingMarketInstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              lending_market_owner: accounts.lending_market_owner,
              lending_market: accounts.lending_market,
                    __args: args,
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
    let mut accounts = Vec::with_capacity(2+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.lending_market_owner.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lending_market.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let mut data = borsh::to_vec(&UpdateLendingMarketInstructionData::new()).unwrap();
          let mut args = borsh::to_vec(&self.__args).unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::KAMINO_LENDING_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(3 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.lending_market_owner.clone());
                        account_infos.push(self.lending_market.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `UpdateLendingMarket` via CPI.
///
/// ### Accounts:
///
                ///   0. `[signer]` lending_market_owner
                ///   1. `[writable]` lending_market
#[derive(Clone, Debug)]
pub struct UpdateLendingMarketCpiBuilder<'a, 'b> {
  instruction: Box<UpdateLendingMarketCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdateLendingMarketCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(UpdateLendingMarketCpiBuilderInstruction {
      __program: program,
              lending_market_owner: None,
              lending_market: None,
                                            mode: None,
                                value: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn lending_market_owner(&mut self, lending_market_owner: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.lending_market_owner = Some(lending_market_owner);
                    self
    }
      #[inline(always)]
    pub fn lending_market(&mut self, lending_market: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.lending_market = Some(lending_market);
                    self
    }
                    #[inline(always)]
      pub fn mode(&mut self, mode: u64) -> &mut Self {
        self.instruction.mode = Some(mode);
        self
      }
                #[inline(always)]
      pub fn value(&mut self, value: [u8; 72]) -> &mut Self {
        self.instruction.value = Some(value);
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
          let args = UpdateLendingMarketInstructionArgs {
                                                              mode: self.instruction.mode.clone().expect("mode is not set"),
                                                                  value: self.instruction.value.clone().expect("value is not set"),
                                    };
        let instruction = UpdateLendingMarketCpi {
        __program: self.instruction.__program,
                  
          lending_market_owner: self.instruction.lending_market_owner.expect("lending_market_owner is not set"),
                  
          lending_market: self.instruction.lending_market.expect("lending_market is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct UpdateLendingMarketCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            lending_market_owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                lending_market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        mode: Option<u64>,
                value: Option<[u8; 72]>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

