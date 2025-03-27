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
pub struct MarkObligationForDeleveraging {
      
              
          pub risk_council: solana_program::pubkey::Pubkey,
          
              
          pub obligation: solana_program::pubkey::Pubkey,
          
              
          pub lending_market: solana_program::pubkey::Pubkey,
      }

impl MarkObligationForDeleveraging {
  pub fn instruction(&self, args: MarkObligationForDeleveragingInstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::arithmetic_side_effects)]
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: MarkObligationForDeleveragingInstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(3+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.risk_council,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.obligation,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.lending_market,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = borsh::to_vec(&MarkObligationForDeleveragingInstructionData::new()).unwrap();
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
 pub struct MarkObligationForDeleveragingInstructionData {
            discriminator: [u8; 8],
            }

impl MarkObligationForDeleveragingInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [164, 35, 182, 19, 0, 116, 243, 127],
                                }
  }
}

impl Default for MarkObligationForDeleveragingInstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct MarkObligationForDeleveragingInstructionArgs {
                  pub autodeleverage_target_ltv_pct: u8,
      }


/// Instruction builder for `MarkObligationForDeleveraging`.
///
/// ### Accounts:
///
                ///   0. `[signer]` risk_council
                ///   1. `[writable]` obligation
          ///   2. `[]` lending_market
#[derive(Clone, Debug, Default)]
pub struct MarkObligationForDeleveragingBuilder {
            risk_council: Option<solana_program::pubkey::Pubkey>,
                obligation: Option<solana_program::pubkey::Pubkey>,
                lending_market: Option<solana_program::pubkey::Pubkey>,
                        autodeleverage_target_ltv_pct: Option<u8>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl MarkObligationForDeleveragingBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn risk_council(&mut self, risk_council: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.risk_council = Some(risk_council);
                    self
    }
            #[inline(always)]
    pub fn obligation(&mut self, obligation: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.obligation = Some(obligation);
                    self
    }
            #[inline(always)]
    pub fn lending_market(&mut self, lending_market: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.lending_market = Some(lending_market);
                    self
    }
                    #[inline(always)]
      pub fn autodeleverage_target_ltv_pct(&mut self, autodeleverage_target_ltv_pct: u8) -> &mut Self {
        self.autodeleverage_target_ltv_pct = Some(autodeleverage_target_ltv_pct);
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
    let accounts = MarkObligationForDeleveraging {
                              risk_council: self.risk_council.expect("risk_council is not set"),
                                        obligation: self.obligation.expect("obligation is not set"),
                                        lending_market: self.lending_market.expect("lending_market is not set"),
                      };
          let args = MarkObligationForDeleveragingInstructionArgs {
                                                              autodeleverage_target_ltv_pct: self.autodeleverage_target_ltv_pct.clone().expect("autodeleverage_target_ltv_pct is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `mark_obligation_for_deleveraging` CPI accounts.
  pub struct MarkObligationForDeleveragingCpiAccounts<'a, 'b> {
          
                    
              pub risk_council: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub obligation: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub lending_market: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `mark_obligation_for_deleveraging` CPI instruction.
pub struct MarkObligationForDeleveragingCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub risk_council: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub obligation: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub lending_market: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: MarkObligationForDeleveragingInstructionArgs,
  }

impl<'a, 'b> MarkObligationForDeleveragingCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: MarkObligationForDeleveragingCpiAccounts<'a, 'b>,
              args: MarkObligationForDeleveragingInstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              risk_council: accounts.risk_council,
              obligation: accounts.obligation,
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
    let mut accounts = Vec::with_capacity(3+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.risk_council.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.obligation.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
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
    let mut data = borsh::to_vec(&MarkObligationForDeleveragingInstructionData::new()).unwrap();
          let mut args = borsh::to_vec(&self.__args).unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::KAMINO_LENDING_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(4 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.risk_council.clone());
                        account_infos.push(self.obligation.clone());
                        account_infos.push(self.lending_market.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `MarkObligationForDeleveraging` via CPI.
///
/// ### Accounts:
///
                ///   0. `[signer]` risk_council
                ///   1. `[writable]` obligation
          ///   2. `[]` lending_market
#[derive(Clone, Debug)]
pub struct MarkObligationForDeleveragingCpiBuilder<'a, 'b> {
  instruction: Box<MarkObligationForDeleveragingCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> MarkObligationForDeleveragingCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(MarkObligationForDeleveragingCpiBuilderInstruction {
      __program: program,
              risk_council: None,
              obligation: None,
              lending_market: None,
                                            autodeleverage_target_ltv_pct: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn risk_council(&mut self, risk_council: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.risk_council = Some(risk_council);
                    self
    }
      #[inline(always)]
    pub fn obligation(&mut self, obligation: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.obligation = Some(obligation);
                    self
    }
      #[inline(always)]
    pub fn lending_market(&mut self, lending_market: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.lending_market = Some(lending_market);
                    self
    }
                    #[inline(always)]
      pub fn autodeleverage_target_ltv_pct(&mut self, autodeleverage_target_ltv_pct: u8) -> &mut Self {
        self.instruction.autodeleverage_target_ltv_pct = Some(autodeleverage_target_ltv_pct);
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
          let args = MarkObligationForDeleveragingInstructionArgs {
                                                              autodeleverage_target_ltv_pct: self.instruction.autodeleverage_target_ltv_pct.clone().expect("autodeleverage_target_ltv_pct is not set"),
                                    };
        let instruction = MarkObligationForDeleveragingCpi {
        __program: self.instruction.__program,
                  
          risk_council: self.instruction.risk_council.expect("risk_council is not set"),
                  
          obligation: self.instruction.obligation.expect("obligation is not set"),
                  
          lending_market: self.instruction.lending_market.expect("lending_market is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct MarkObligationForDeleveragingCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            risk_council: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                obligation: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                lending_market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        autodeleverage_target_ltv_pct: Option<u8>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

