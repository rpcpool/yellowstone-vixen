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
pub struct CancelAllOrders {
            /// Phoenix program

    
              
          pub phoenix_program: solana_program::pubkey::Pubkey,
                /// Phoenix log authority

    
              
          pub log_authority: solana_program::pubkey::Pubkey,
                /// This account holds the market state

    
              
          pub market: solana_program::pubkey::Pubkey,
          
              
          pub trader: solana_program::pubkey::Pubkey,
                /// Trader base token account

    
              
          pub base_account: solana_program::pubkey::Pubkey,
                /// Trader quote token account

    
              
          pub quote_account: solana_program::pubkey::Pubkey,
                /// Base vault PDA, seeds are [b'vault', market_address, base_mint_address]

    
              
          pub base_vault: solana_program::pubkey::Pubkey,
                /// Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]

    
              
          pub quote_vault: solana_program::pubkey::Pubkey,
                /// Token program

    
              
          pub token_program: solana_program::pubkey::Pubkey,
      }

impl CancelAllOrders {
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(&[])
  }
  #[allow(clippy::arithmetic_side_effects)]
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(9+ remaining_accounts.len());
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
            self.trader,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.base_account,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.quote_account,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.base_vault,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.quote_vault,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let data = borsh::to_vec(&CancelAllOrdersInstructionData::new()).unwrap();
    
    solana_program::instruction::Instruction {
      program_id: crate::PHOENIX_V1_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct CancelAllOrdersInstructionData {
            discriminator: u8,
      }

impl CancelAllOrdersInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: 6,
                  }
  }
}

impl Default for CancelAllOrdersInstructionData {
  fn default() -> Self {
    Self::new()
  }
}



/// Instruction builder for `CancelAllOrders`.
///
/// ### Accounts:
///
          ///   0. `[]` phoenix_program
          ///   1. `[]` log_authority
                ///   2. `[writable]` market
                ///   3. `[signer]` trader
                ///   4. `[writable]` base_account
                ///   5. `[writable]` quote_account
                ///   6. `[writable]` base_vault
                ///   7. `[writable]` quote_vault
                ///   8. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct CancelAllOrdersBuilder {
            phoenix_program: Option<solana_program::pubkey::Pubkey>,
                log_authority: Option<solana_program::pubkey::Pubkey>,
                market: Option<solana_program::pubkey::Pubkey>,
                trader: Option<solana_program::pubkey::Pubkey>,
                base_account: Option<solana_program::pubkey::Pubkey>,
                quote_account: Option<solana_program::pubkey::Pubkey>,
                base_vault: Option<solana_program::pubkey::Pubkey>,
                quote_vault: Option<solana_program::pubkey::Pubkey>,
                token_program: Option<solana_program::pubkey::Pubkey>,
                __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CancelAllOrdersBuilder {
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
            #[inline(always)]
    pub fn trader(&mut self, trader: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.trader = Some(trader);
                    self
    }
            /// Trader base token account
#[inline(always)]
    pub fn base_account(&mut self, base_account: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.base_account = Some(base_account);
                    self
    }
            /// Trader quote token account
#[inline(always)]
    pub fn quote_account(&mut self, quote_account: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.quote_account = Some(quote_account);
                    self
    }
            /// Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
#[inline(always)]
    pub fn base_vault(&mut self, base_vault: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.base_vault = Some(base_vault);
                    self
    }
            /// Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
#[inline(always)]
    pub fn quote_vault(&mut self, quote_vault: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.quote_vault = Some(quote_vault);
                    self
    }
            /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
/// Token program
#[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_program = Some(token_program);
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
    let accounts = CancelAllOrders {
                              phoenix_program: self.phoenix_program.expect("phoenix_program is not set"),
                                        log_authority: self.log_authority.expect("log_authority is not set"),
                                        market: self.market.expect("market is not set"),
                                        trader: self.trader.expect("trader is not set"),
                                        base_account: self.base_account.expect("base_account is not set"),
                                        quote_account: self.quote_account.expect("quote_account is not set"),
                                        base_vault: self.base_vault.expect("base_vault is not set"),
                                        quote_vault: self.quote_vault.expect("quote_vault is not set"),
                                        token_program: self.token_program.unwrap_or(solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")),
                      };
    
    accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
  }
}

  /// `cancel_all_orders` CPI accounts.
  pub struct CancelAllOrdersCpiAccounts<'a, 'b> {
                  /// Phoenix program

      
                    
              pub phoenix_program: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Phoenix log authority

      
                    
              pub log_authority: &'b solana_program::account_info::AccountInfo<'a>,
                        /// This account holds the market state

      
                    
              pub market: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub trader: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Trader base token account

      
                    
              pub base_account: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Trader quote token account

      
                    
              pub quote_account: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Base vault PDA, seeds are [b'vault', market_address, base_mint_address]

      
                    
              pub base_vault: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]

      
                    
              pub quote_vault: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Token program

      
                    
              pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `cancel_all_orders` CPI instruction.
pub struct CancelAllOrdersCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
            /// Phoenix program

    
              
          pub phoenix_program: &'b solana_program::account_info::AccountInfo<'a>,
                /// Phoenix log authority

    
              
          pub log_authority: &'b solana_program::account_info::AccountInfo<'a>,
                /// This account holds the market state

    
              
          pub market: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub trader: &'b solana_program::account_info::AccountInfo<'a>,
                /// Trader base token account

    
              
          pub base_account: &'b solana_program::account_info::AccountInfo<'a>,
                /// Trader quote token account

    
              
          pub quote_account: &'b solana_program::account_info::AccountInfo<'a>,
                /// Base vault PDA, seeds are [b'vault', market_address, base_mint_address]

    
              
          pub base_vault: &'b solana_program::account_info::AccountInfo<'a>,
                /// Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]

    
              
          pub quote_vault: &'b solana_program::account_info::AccountInfo<'a>,
                /// Token program

    
              
          pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
        }

impl<'a, 'b> CancelAllOrdersCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: CancelAllOrdersCpiAccounts<'a, 'b>,
          ) -> Self {
    Self {
      __program: program,
              phoenix_program: accounts.phoenix_program,
              log_authority: accounts.log_authority,
              market: accounts.market,
              trader: accounts.trader,
              base_account: accounts.base_account,
              quote_account: accounts.quote_account,
              base_vault: accounts.base_vault,
              quote_vault: accounts.quote_vault,
              token_program: accounts.token_program,
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
    let mut accounts = Vec::with_capacity(9+ remaining_accounts.len());
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
            *self.trader.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.base_account.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.quote_account.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.base_vault.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.quote_vault.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let data = borsh::to_vec(&CancelAllOrdersInstructionData::new()).unwrap();
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::PHOENIX_V1_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(10 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.phoenix_program.clone());
                        account_infos.push(self.log_authority.clone());
                        account_infos.push(self.market.clone());
                        account_infos.push(self.trader.clone());
                        account_infos.push(self.base_account.clone());
                        account_infos.push(self.quote_account.clone());
                        account_infos.push(self.base_vault.clone());
                        account_infos.push(self.quote_vault.clone());
                        account_infos.push(self.token_program.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `CancelAllOrders` via CPI.
///
/// ### Accounts:
///
          ///   0. `[]` phoenix_program
          ///   1. `[]` log_authority
                ///   2. `[writable]` market
                ///   3. `[signer]` trader
                ///   4. `[writable]` base_account
                ///   5. `[writable]` quote_account
                ///   6. `[writable]` base_vault
                ///   7. `[writable]` quote_vault
          ///   8. `[]` token_program
#[derive(Clone, Debug)]
pub struct CancelAllOrdersCpiBuilder<'a, 'b> {
  instruction: Box<CancelAllOrdersCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CancelAllOrdersCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(CancelAllOrdersCpiBuilderInstruction {
      __program: program,
              phoenix_program: None,
              log_authority: None,
              market: None,
              trader: None,
              base_account: None,
              quote_account: None,
              base_vault: None,
              quote_vault: None,
              token_program: None,
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
      #[inline(always)]
    pub fn trader(&mut self, trader: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.trader = Some(trader);
                    self
    }
      /// Trader base token account
#[inline(always)]
    pub fn base_account(&mut self, base_account: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.base_account = Some(base_account);
                    self
    }
      /// Trader quote token account
#[inline(always)]
    pub fn quote_account(&mut self, quote_account: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.quote_account = Some(quote_account);
                    self
    }
      /// Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
#[inline(always)]
    pub fn base_vault(&mut self, base_vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.base_vault = Some(base_vault);
                    self
    }
      /// Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
#[inline(always)]
    pub fn quote_vault(&mut self, quote_vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.quote_vault = Some(quote_vault);
                    self
    }
      /// Token program
#[inline(always)]
    pub fn token_program(&mut self, token_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_program = Some(token_program);
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
        let instruction = CancelAllOrdersCpi {
        __program: self.instruction.__program,
                  
          phoenix_program: self.instruction.phoenix_program.expect("phoenix_program is not set"),
                  
          log_authority: self.instruction.log_authority.expect("log_authority is not set"),
                  
          market: self.instruction.market.expect("market is not set"),
                  
          trader: self.instruction.trader.expect("trader is not set"),
                  
          base_account: self.instruction.base_account.expect("base_account is not set"),
                  
          quote_account: self.instruction.quote_account.expect("quote_account is not set"),
                  
          base_vault: self.instruction.base_vault.expect("base_vault is not set"),
                  
          quote_vault: self.instruction.quote_vault.expect("quote_vault is not set"),
                  
          token_program: self.instruction.token_program.expect("token_program is not set"),
                    };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct CancelAllOrdersCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            phoenix_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                log_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                trader: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                base_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                quote_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                base_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                quote_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

