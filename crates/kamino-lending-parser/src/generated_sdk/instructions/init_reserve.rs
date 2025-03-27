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
pub struct InitReserve {
      
              
          pub lending_market_owner: solana_program::pubkey::Pubkey,
          
              
          pub lending_market: solana_program::pubkey::Pubkey,
          
              
          pub lending_market_authority: solana_program::pubkey::Pubkey,
          
              
          pub reserve: solana_program::pubkey::Pubkey,
          
              
          pub reserve_liquidity_mint: solana_program::pubkey::Pubkey,
          
              
          pub reserve_liquidity_supply: solana_program::pubkey::Pubkey,
          
              
          pub fee_receiver: solana_program::pubkey::Pubkey,
          
              
          pub reserve_collateral_mint: solana_program::pubkey::Pubkey,
          
              
          pub reserve_collateral_supply: solana_program::pubkey::Pubkey,
          
              
          pub initial_liquidity_source: solana_program::pubkey::Pubkey,
          
              
          pub rent: solana_program::pubkey::Pubkey,
          
              
          pub liquidity_token_program: solana_program::pubkey::Pubkey,
          
              
          pub collateral_token_program: solana_program::pubkey::Pubkey,
          
              
          pub system_program: solana_program::pubkey::Pubkey,
      }

impl InitReserve {
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(&[])
  }
  #[allow(clippy::arithmetic_side_effects)]
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(14+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            self.lending_market_owner,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.lending_market,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.lending_market_authority,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.reserve,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reserve_liquidity_mint,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.reserve_liquidity_supply,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.fee_receiver,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.reserve_collateral_mint,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.reserve_collateral_supply,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.initial_liquidity_source,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.liquidity_token_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collateral_token_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let data = borsh::to_vec(&InitReserveInstructionData::new()).unwrap();
    
    solana_program::instruction::Instruction {
      program_id: crate::KAMINO_LENDING_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct InitReserveInstructionData {
            discriminator: [u8; 8],
      }

impl InitReserveInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [138, 245, 71, 225, 153, 4, 3, 43],
                  }
  }
}

impl Default for InitReserveInstructionData {
  fn default() -> Self {
    Self::new()
  }
}



/// Instruction builder for `InitReserve`.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` lending_market_owner
          ///   1. `[]` lending_market
          ///   2. `[]` lending_market_authority
                ///   3. `[writable]` reserve
          ///   4. `[]` reserve_liquidity_mint
                ///   5. `[writable]` reserve_liquidity_supply
                ///   6. `[writable]` fee_receiver
                ///   7. `[writable]` reserve_collateral_mint
                ///   8. `[writable]` reserve_collateral_supply
                ///   9. `[writable]` initial_liquidity_source
                ///   10. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
          ///   11. `[]` liquidity_token_program
          ///   12. `[]` collateral_token_program
                ///   13. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitReserveBuilder {
            lending_market_owner: Option<solana_program::pubkey::Pubkey>,
                lending_market: Option<solana_program::pubkey::Pubkey>,
                lending_market_authority: Option<solana_program::pubkey::Pubkey>,
                reserve: Option<solana_program::pubkey::Pubkey>,
                reserve_liquidity_mint: Option<solana_program::pubkey::Pubkey>,
                reserve_liquidity_supply: Option<solana_program::pubkey::Pubkey>,
                fee_receiver: Option<solana_program::pubkey::Pubkey>,
                reserve_collateral_mint: Option<solana_program::pubkey::Pubkey>,
                reserve_collateral_supply: Option<solana_program::pubkey::Pubkey>,
                initial_liquidity_source: Option<solana_program::pubkey::Pubkey>,
                rent: Option<solana_program::pubkey::Pubkey>,
                liquidity_token_program: Option<solana_program::pubkey::Pubkey>,
                collateral_token_program: Option<solana_program::pubkey::Pubkey>,
                system_program: Option<solana_program::pubkey::Pubkey>,
                __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitReserveBuilder {
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
    pub fn lending_market_authority(&mut self, lending_market_authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.lending_market_authority = Some(lending_market_authority);
                    self
    }
            #[inline(always)]
    pub fn reserve(&mut self, reserve: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.reserve = Some(reserve);
                    self
    }
            #[inline(always)]
    pub fn reserve_liquidity_mint(&mut self, reserve_liquidity_mint: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.reserve_liquidity_mint = Some(reserve_liquidity_mint);
                    self
    }
            #[inline(always)]
    pub fn reserve_liquidity_supply(&mut self, reserve_liquidity_supply: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.reserve_liquidity_supply = Some(reserve_liquidity_supply);
                    self
    }
            #[inline(always)]
    pub fn fee_receiver(&mut self, fee_receiver: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.fee_receiver = Some(fee_receiver);
                    self
    }
            #[inline(always)]
    pub fn reserve_collateral_mint(&mut self, reserve_collateral_mint: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.reserve_collateral_mint = Some(reserve_collateral_mint);
                    self
    }
            #[inline(always)]
    pub fn reserve_collateral_supply(&mut self, reserve_collateral_supply: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.reserve_collateral_supply = Some(reserve_collateral_supply);
                    self
    }
            #[inline(always)]
    pub fn initial_liquidity_source(&mut self, initial_liquidity_source: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.initial_liquidity_source = Some(initial_liquidity_source);
                    self
    }
            /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
#[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.rent = Some(rent);
                    self
    }
            #[inline(always)]
    pub fn liquidity_token_program(&mut self, liquidity_token_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.liquidity_token_program = Some(liquidity_token_program);
                    self
    }
            #[inline(always)]
    pub fn collateral_token_program(&mut self, collateral_token_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.collateral_token_program = Some(collateral_token_program);
                    self
    }
            /// `[optional account, default to '11111111111111111111111111111111']`
#[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.system_program = Some(system_program);
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
    let accounts = InitReserve {
                              lending_market_owner: self.lending_market_owner.expect("lending_market_owner is not set"),
                                        lending_market: self.lending_market.expect("lending_market is not set"),
                                        lending_market_authority: self.lending_market_authority.expect("lending_market_authority is not set"),
                                        reserve: self.reserve.expect("reserve is not set"),
                                        reserve_liquidity_mint: self.reserve_liquidity_mint.expect("reserve_liquidity_mint is not set"),
                                        reserve_liquidity_supply: self.reserve_liquidity_supply.expect("reserve_liquidity_supply is not set"),
                                        fee_receiver: self.fee_receiver.expect("fee_receiver is not set"),
                                        reserve_collateral_mint: self.reserve_collateral_mint.expect("reserve_collateral_mint is not set"),
                                        reserve_collateral_supply: self.reserve_collateral_supply.expect("reserve_collateral_supply is not set"),
                                        initial_liquidity_source: self.initial_liquidity_source.expect("initial_liquidity_source is not set"),
                                        rent: self.rent.unwrap_or(solana_program::pubkey!("SysvarRent111111111111111111111111111111111")),
                                        liquidity_token_program: self.liquidity_token_program.expect("liquidity_token_program is not set"),
                                        collateral_token_program: self.collateral_token_program.expect("collateral_token_program is not set"),
                                        system_program: self.system_program.unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
                      };
    
    accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
  }
}

  /// `init_reserve` CPI accounts.
  pub struct InitReserveCpiAccounts<'a, 'b> {
          
                    
              pub lending_market_owner: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub lending_market: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub lending_market_authority: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub reserve: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub reserve_liquidity_mint: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub reserve_liquidity_supply: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub fee_receiver: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub reserve_collateral_mint: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub reserve_collateral_supply: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub initial_liquidity_source: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub rent: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub liquidity_token_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub collateral_token_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `init_reserve` CPI instruction.
pub struct InitReserveCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub lending_market_owner: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub lending_market: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub lending_market_authority: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub reserve: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub reserve_liquidity_mint: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub reserve_liquidity_supply: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub fee_receiver: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub reserve_collateral_mint: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub reserve_collateral_supply: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub initial_liquidity_source: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub rent: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub liquidity_token_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub collateral_token_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
        }

impl<'a, 'b> InitReserveCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: InitReserveCpiAccounts<'a, 'b>,
          ) -> Self {
    Self {
      __program: program,
              lending_market_owner: accounts.lending_market_owner,
              lending_market: accounts.lending_market,
              lending_market_authority: accounts.lending_market_authority,
              reserve: accounts.reserve,
              reserve_liquidity_mint: accounts.reserve_liquidity_mint,
              reserve_liquidity_supply: accounts.reserve_liquidity_supply,
              fee_receiver: accounts.fee_receiver,
              reserve_collateral_mint: accounts.reserve_collateral_mint,
              reserve_collateral_supply: accounts.reserve_collateral_supply,
              initial_liquidity_source: accounts.initial_liquidity_source,
              rent: accounts.rent,
              liquidity_token_program: accounts.liquidity_token_program,
              collateral_token_program: accounts.collateral_token_program,
              system_program: accounts.system_program,
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
    let mut accounts = Vec::with_capacity(14+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lending_market_owner.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.lending_market.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.lending_market_authority.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reserve.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reserve_liquidity_mint.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reserve_liquidity_supply.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.fee_receiver.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reserve_collateral_mint.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reserve_collateral_supply.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.initial_liquidity_source.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.liquidity_token_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collateral_token_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let data = borsh::to_vec(&InitReserveInstructionData::new()).unwrap();
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::KAMINO_LENDING_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(15 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.lending_market_owner.clone());
                        account_infos.push(self.lending_market.clone());
                        account_infos.push(self.lending_market_authority.clone());
                        account_infos.push(self.reserve.clone());
                        account_infos.push(self.reserve_liquidity_mint.clone());
                        account_infos.push(self.reserve_liquidity_supply.clone());
                        account_infos.push(self.fee_receiver.clone());
                        account_infos.push(self.reserve_collateral_mint.clone());
                        account_infos.push(self.reserve_collateral_supply.clone());
                        account_infos.push(self.initial_liquidity_source.clone());
                        account_infos.push(self.rent.clone());
                        account_infos.push(self.liquidity_token_program.clone());
                        account_infos.push(self.collateral_token_program.clone());
                        account_infos.push(self.system_program.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `InitReserve` via CPI.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` lending_market_owner
          ///   1. `[]` lending_market
          ///   2. `[]` lending_market_authority
                ///   3. `[writable]` reserve
          ///   4. `[]` reserve_liquidity_mint
                ///   5. `[writable]` reserve_liquidity_supply
                ///   6. `[writable]` fee_receiver
                ///   7. `[writable]` reserve_collateral_mint
                ///   8. `[writable]` reserve_collateral_supply
                ///   9. `[writable]` initial_liquidity_source
          ///   10. `[]` rent
          ///   11. `[]` liquidity_token_program
          ///   12. `[]` collateral_token_program
          ///   13. `[]` system_program
#[derive(Clone, Debug)]
pub struct InitReserveCpiBuilder<'a, 'b> {
  instruction: Box<InitReserveCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitReserveCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(InitReserveCpiBuilderInstruction {
      __program: program,
              lending_market_owner: None,
              lending_market: None,
              lending_market_authority: None,
              reserve: None,
              reserve_liquidity_mint: None,
              reserve_liquidity_supply: None,
              fee_receiver: None,
              reserve_collateral_mint: None,
              reserve_collateral_supply: None,
              initial_liquidity_source: None,
              rent: None,
              liquidity_token_program: None,
              collateral_token_program: None,
              system_program: None,
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
    pub fn lending_market_authority(&mut self, lending_market_authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.lending_market_authority = Some(lending_market_authority);
                    self
    }
      #[inline(always)]
    pub fn reserve(&mut self, reserve: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.reserve = Some(reserve);
                    self
    }
      #[inline(always)]
    pub fn reserve_liquidity_mint(&mut self, reserve_liquidity_mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.reserve_liquidity_mint = Some(reserve_liquidity_mint);
                    self
    }
      #[inline(always)]
    pub fn reserve_liquidity_supply(&mut self, reserve_liquidity_supply: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.reserve_liquidity_supply = Some(reserve_liquidity_supply);
                    self
    }
      #[inline(always)]
    pub fn fee_receiver(&mut self, fee_receiver: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.fee_receiver = Some(fee_receiver);
                    self
    }
      #[inline(always)]
    pub fn reserve_collateral_mint(&mut self, reserve_collateral_mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.reserve_collateral_mint = Some(reserve_collateral_mint);
                    self
    }
      #[inline(always)]
    pub fn reserve_collateral_supply(&mut self, reserve_collateral_supply: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.reserve_collateral_supply = Some(reserve_collateral_supply);
                    self
    }
      #[inline(always)]
    pub fn initial_liquidity_source(&mut self, initial_liquidity_source: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.initial_liquidity_source = Some(initial_liquidity_source);
                    self
    }
      #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.rent = Some(rent);
                    self
    }
      #[inline(always)]
    pub fn liquidity_token_program(&mut self, liquidity_token_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.liquidity_token_program = Some(liquidity_token_program);
                    self
    }
      #[inline(always)]
    pub fn collateral_token_program(&mut self, collateral_token_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.collateral_token_program = Some(collateral_token_program);
                    self
    }
      #[inline(always)]
    pub fn system_program(&mut self, system_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.system_program = Some(system_program);
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
        let instruction = InitReserveCpi {
        __program: self.instruction.__program,
                  
          lending_market_owner: self.instruction.lending_market_owner.expect("lending_market_owner is not set"),
                  
          lending_market: self.instruction.lending_market.expect("lending_market is not set"),
                  
          lending_market_authority: self.instruction.lending_market_authority.expect("lending_market_authority is not set"),
                  
          reserve: self.instruction.reserve.expect("reserve is not set"),
                  
          reserve_liquidity_mint: self.instruction.reserve_liquidity_mint.expect("reserve_liquidity_mint is not set"),
                  
          reserve_liquidity_supply: self.instruction.reserve_liquidity_supply.expect("reserve_liquidity_supply is not set"),
                  
          fee_receiver: self.instruction.fee_receiver.expect("fee_receiver is not set"),
                  
          reserve_collateral_mint: self.instruction.reserve_collateral_mint.expect("reserve_collateral_mint is not set"),
                  
          reserve_collateral_supply: self.instruction.reserve_collateral_supply.expect("reserve_collateral_supply is not set"),
                  
          initial_liquidity_source: self.instruction.initial_liquidity_source.expect("initial_liquidity_source is not set"),
                  
          rent: self.instruction.rent.expect("rent is not set"),
                  
          liquidity_token_program: self.instruction.liquidity_token_program.expect("liquidity_token_program is not set"),
                  
          collateral_token_program: self.instruction.collateral_token_program.expect("collateral_token_program is not set"),
                  
          system_program: self.instruction.system_program.expect("system_program is not set"),
                    };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct InitReserveCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            lending_market_owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                lending_market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                lending_market_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                reserve: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                reserve_liquidity_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                reserve_liquidity_supply: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                fee_receiver: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                reserve_collateral_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                reserve_collateral_supply: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                initial_liquidity_source: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                liquidity_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                collateral_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

