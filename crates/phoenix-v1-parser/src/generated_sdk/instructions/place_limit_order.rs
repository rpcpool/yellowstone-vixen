//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::generated::types::OrderPacket;
use borsh::BorshSerialize;
use borsh::BorshDeserialize;

/// Accounts.
#[derive(Debug)]
pub struct PlaceLimitOrder {
            /// Phoenix program

    
              
          pub phoenix_program: solana_program::pubkey::Pubkey,
                /// Phoenix log authority

    
              
          pub log_authority: solana_program::pubkey::Pubkey,
                /// This account holds the market state

    
              
          pub market: solana_program::pubkey::Pubkey,
          
              
          pub trader: solana_program::pubkey::Pubkey,
          
              
          pub seat: solana_program::pubkey::Pubkey,
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

impl PlaceLimitOrder {
  pub fn instruction(&self, args: PlaceLimitOrderInstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::arithmetic_side_effects)]
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: PlaceLimitOrderInstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(10+ remaining_accounts.len());
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
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.seat,
            false
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
    let mut data = borsh::to_vec(&PlaceLimitOrderInstructionData::new()).unwrap();
          let mut args = borsh::to_vec(&args).unwrap();
      data.append(&mut args);
    
    solana_program::instruction::Instruction {
      program_id: crate::PHOENIX_V1_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct PlaceLimitOrderInstructionData {
            discriminator: u8,
            }

impl PlaceLimitOrderInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: 2,
                                }
  }
}

impl Default for PlaceLimitOrderInstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct PlaceLimitOrderInstructionArgs {
                  pub order_packet: OrderPacket,
      }


/// Instruction builder for `PlaceLimitOrder`.
///
/// ### Accounts:
///
          ///   0. `[]` phoenix_program
          ///   1. `[]` log_authority
                ///   2. `[writable]` market
                ///   3. `[signer]` trader
          ///   4. `[]` seat
                ///   5. `[writable]` base_account
                ///   6. `[writable]` quote_account
                ///   7. `[writable]` base_vault
                ///   8. `[writable]` quote_vault
                ///   9. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct PlaceLimitOrderBuilder {
            phoenix_program: Option<solana_program::pubkey::Pubkey>,
                log_authority: Option<solana_program::pubkey::Pubkey>,
                market: Option<solana_program::pubkey::Pubkey>,
                trader: Option<solana_program::pubkey::Pubkey>,
                seat: Option<solana_program::pubkey::Pubkey>,
                base_account: Option<solana_program::pubkey::Pubkey>,
                quote_account: Option<solana_program::pubkey::Pubkey>,
                base_vault: Option<solana_program::pubkey::Pubkey>,
                quote_vault: Option<solana_program::pubkey::Pubkey>,
                token_program: Option<solana_program::pubkey::Pubkey>,
                        order_packet: Option<OrderPacket>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl PlaceLimitOrderBuilder {
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
            #[inline(always)]
    pub fn seat(&mut self, seat: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.seat = Some(seat);
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
                    #[inline(always)]
      pub fn order_packet(&mut self, order_packet: OrderPacket) -> &mut Self {
        self.order_packet = Some(order_packet);
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
    let accounts = PlaceLimitOrder {
                              phoenix_program: self.phoenix_program.expect("phoenix_program is not set"),
                                        log_authority: self.log_authority.expect("log_authority is not set"),
                                        market: self.market.expect("market is not set"),
                                        trader: self.trader.expect("trader is not set"),
                                        seat: self.seat.expect("seat is not set"),
                                        base_account: self.base_account.expect("base_account is not set"),
                                        quote_account: self.quote_account.expect("quote_account is not set"),
                                        base_vault: self.base_vault.expect("base_vault is not set"),
                                        quote_vault: self.quote_vault.expect("quote_vault is not set"),
                                        token_program: self.token_program.unwrap_or(solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")),
                      };
          let args = PlaceLimitOrderInstructionArgs {
                                                              order_packet: self.order_packet.clone().expect("order_packet is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `place_limit_order` CPI accounts.
  pub struct PlaceLimitOrderCpiAccounts<'a, 'b> {
                  /// Phoenix program

      
                    
              pub phoenix_program: &'b solana_program::account_info::AccountInfo<'a>,
                        /// Phoenix log authority

      
                    
              pub log_authority: &'b solana_program::account_info::AccountInfo<'a>,
                        /// This account holds the market state

      
                    
              pub market: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub trader: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub seat: &'b solana_program::account_info::AccountInfo<'a>,
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

/// `place_limit_order` CPI instruction.
pub struct PlaceLimitOrderCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
            /// Phoenix program

    
              
          pub phoenix_program: &'b solana_program::account_info::AccountInfo<'a>,
                /// Phoenix log authority

    
              
          pub log_authority: &'b solana_program::account_info::AccountInfo<'a>,
                /// This account holds the market state

    
              
          pub market: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub trader: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub seat: &'b solana_program::account_info::AccountInfo<'a>,
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
            /// The arguments for the instruction.
    pub __args: PlaceLimitOrderInstructionArgs,
  }

impl<'a, 'b> PlaceLimitOrderCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: PlaceLimitOrderCpiAccounts<'a, 'b>,
              args: PlaceLimitOrderInstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              phoenix_program: accounts.phoenix_program,
              log_authority: accounts.log_authority,
              market: accounts.market,
              trader: accounts.trader,
              seat: accounts.seat,
              base_account: accounts.base_account,
              quote_account: accounts.quote_account,
              base_vault: accounts.base_vault,
              quote_vault: accounts.quote_vault,
              token_program: accounts.token_program,
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
    let mut accounts = Vec::with_capacity(10+ remaining_accounts.len());
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
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.seat.key,
            false
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
    let mut data = borsh::to_vec(&PlaceLimitOrderInstructionData::new()).unwrap();
          let mut args = borsh::to_vec(&self.__args).unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::PHOENIX_V1_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(11 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.phoenix_program.clone());
                        account_infos.push(self.log_authority.clone());
                        account_infos.push(self.market.clone());
                        account_infos.push(self.trader.clone());
                        account_infos.push(self.seat.clone());
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

/// Instruction builder for `PlaceLimitOrder` via CPI.
///
/// ### Accounts:
///
          ///   0. `[]` phoenix_program
          ///   1. `[]` log_authority
                ///   2. `[writable]` market
                ///   3. `[signer]` trader
          ///   4. `[]` seat
                ///   5. `[writable]` base_account
                ///   6. `[writable]` quote_account
                ///   7. `[writable]` base_vault
                ///   8. `[writable]` quote_vault
          ///   9. `[]` token_program
#[derive(Clone, Debug)]
pub struct PlaceLimitOrderCpiBuilder<'a, 'b> {
  instruction: Box<PlaceLimitOrderCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> PlaceLimitOrderCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(PlaceLimitOrderCpiBuilderInstruction {
      __program: program,
              phoenix_program: None,
              log_authority: None,
              market: None,
              trader: None,
              seat: None,
              base_account: None,
              quote_account: None,
              base_vault: None,
              quote_vault: None,
              token_program: None,
                                            order_packet: None,
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
      #[inline(always)]
    pub fn seat(&mut self, seat: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.seat = Some(seat);
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
                    #[inline(always)]
      pub fn order_packet(&mut self, order_packet: OrderPacket) -> &mut Self {
        self.instruction.order_packet = Some(order_packet);
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
          let args = PlaceLimitOrderInstructionArgs {
                                                              order_packet: self.instruction.order_packet.clone().expect("order_packet is not set"),
                                    };
        let instruction = PlaceLimitOrderCpi {
        __program: self.instruction.__program,
                  
          phoenix_program: self.instruction.phoenix_program.expect("phoenix_program is not set"),
                  
          log_authority: self.instruction.log_authority.expect("log_authority is not set"),
                  
          market: self.instruction.market.expect("market is not set"),
                  
          trader: self.instruction.trader.expect("trader is not set"),
                  
          seat: self.instruction.seat.expect("seat is not set"),
                  
          base_account: self.instruction.base_account.expect("base_account is not set"),
                  
          quote_account: self.instruction.quote_account.expect("quote_account is not set"),
                  
          base_vault: self.instruction.base_vault.expect("base_vault is not set"),
                  
          quote_vault: self.instruction.quote_vault.expect("quote_vault is not set"),
                  
          token_program: self.instruction.token_program.expect("token_program is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct PlaceLimitOrderCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            phoenix_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                log_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                trader: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                seat: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                base_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                quote_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                base_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                quote_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        order_packet: Option<OrderPacket>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

