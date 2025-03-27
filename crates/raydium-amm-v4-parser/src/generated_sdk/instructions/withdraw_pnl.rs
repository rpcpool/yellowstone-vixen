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
pub struct WithdrawPnl {
      
              
          pub token_program: solana_program::pubkey::Pubkey,
          
              
          pub amm: solana_program::pubkey::Pubkey,
          
              
          pub amm_config: solana_program::pubkey::Pubkey,
          
              
          pub amm_authority: solana_program::pubkey::Pubkey,
          
              
          pub amm_open_orders: solana_program::pubkey::Pubkey,
          
              
          pub pool_coin_token_account: solana_program::pubkey::Pubkey,
          
              
          pub pool_pc_token_account: solana_program::pubkey::Pubkey,
          
              
          pub coin_pnl_token_account: solana_program::pubkey::Pubkey,
          
              
          pub pc_pnl_token_account: solana_program::pubkey::Pubkey,
          
              
          pub pnl_owner_account: solana_program::pubkey::Pubkey,
          
              
          pub amm_target_orders: solana_program::pubkey::Pubkey,
          
              
          pub serum_program: solana_program::pubkey::Pubkey,
          
              
          pub serum_market: solana_program::pubkey::Pubkey,
          
              
          pub serum_event_queue: solana_program::pubkey::Pubkey,
          
              
          pub serum_coin_vault_account: solana_program::pubkey::Pubkey,
          
              
          pub serum_pc_vault_account: solana_program::pubkey::Pubkey,
          
              
          pub serum_vault_signer: solana_program::pubkey::Pubkey,
      }

impl WithdrawPnl {
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(&[])
  }
  #[allow(clippy::arithmetic_side_effects)]
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(17+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.amm,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.amm_config,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.amm_authority,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.amm_open_orders,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool_coin_token_account,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool_pc_token_account,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.coin_pnl_token_account,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.pc_pnl_token_account,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.pnl_owner_account,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.amm_target_orders,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.serum_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.serum_market,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.serum_event_queue,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.serum_coin_vault_account,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.serum_pc_vault_account,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.serum_vault_signer,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let data = borsh::to_vec(&WithdrawPnlInstructionData::new()).unwrap();
    
    solana_program::instruction::Instruction {
      program_id: crate::RAYDIUM_AMM_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct WithdrawPnlInstructionData {
            discriminator: [u8; 8],
      }

impl WithdrawPnlInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [86, 36, 158, 158, 92, 241, 251, 94],
                  }
  }
}

impl Default for WithdrawPnlInstructionData {
  fn default() -> Self {
    Self::new()
  }
}



/// Instruction builder for `WithdrawPnl`.
///
/// ### Accounts:
///
                ///   0. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
                ///   1. `[writable]` amm
          ///   2. `[]` amm_config
          ///   3. `[]` amm_authority
                ///   4. `[writable]` amm_open_orders
                ///   5. `[writable]` pool_coin_token_account
                ///   6. `[writable]` pool_pc_token_account
                ///   7. `[writable]` coin_pnl_token_account
                ///   8. `[writable]` pc_pnl_token_account
                ///   9. `[signer]` pnl_owner_account
                ///   10. `[writable]` amm_target_orders
          ///   11. `[]` serum_program
                ///   12. `[writable]` serum_market
          ///   13. `[]` serum_event_queue
                ///   14. `[writable]` serum_coin_vault_account
                ///   15. `[writable]` serum_pc_vault_account
          ///   16. `[]` serum_vault_signer
#[derive(Clone, Debug, Default)]
pub struct WithdrawPnlBuilder {
            token_program: Option<solana_program::pubkey::Pubkey>,
                amm: Option<solana_program::pubkey::Pubkey>,
                amm_config: Option<solana_program::pubkey::Pubkey>,
                amm_authority: Option<solana_program::pubkey::Pubkey>,
                amm_open_orders: Option<solana_program::pubkey::Pubkey>,
                pool_coin_token_account: Option<solana_program::pubkey::Pubkey>,
                pool_pc_token_account: Option<solana_program::pubkey::Pubkey>,
                coin_pnl_token_account: Option<solana_program::pubkey::Pubkey>,
                pc_pnl_token_account: Option<solana_program::pubkey::Pubkey>,
                pnl_owner_account: Option<solana_program::pubkey::Pubkey>,
                amm_target_orders: Option<solana_program::pubkey::Pubkey>,
                serum_program: Option<solana_program::pubkey::Pubkey>,
                serum_market: Option<solana_program::pubkey::Pubkey>,
                serum_event_queue: Option<solana_program::pubkey::Pubkey>,
                serum_coin_vault_account: Option<solana_program::pubkey::Pubkey>,
                serum_pc_vault_account: Option<solana_program::pubkey::Pubkey>,
                serum_vault_signer: Option<solana_program::pubkey::Pubkey>,
                __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl WithdrawPnlBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
#[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_program = Some(token_program);
                    self
    }
            #[inline(always)]
    pub fn amm(&mut self, amm: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.amm = Some(amm);
                    self
    }
            #[inline(always)]
    pub fn amm_config(&mut self, amm_config: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.amm_config = Some(amm_config);
                    self
    }
            #[inline(always)]
    pub fn amm_authority(&mut self, amm_authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.amm_authority = Some(amm_authority);
                    self
    }
            #[inline(always)]
    pub fn amm_open_orders(&mut self, amm_open_orders: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.amm_open_orders = Some(amm_open_orders);
                    self
    }
            #[inline(always)]
    pub fn pool_coin_token_account(&mut self, pool_coin_token_account: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.pool_coin_token_account = Some(pool_coin_token_account);
                    self
    }
            #[inline(always)]
    pub fn pool_pc_token_account(&mut self, pool_pc_token_account: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.pool_pc_token_account = Some(pool_pc_token_account);
                    self
    }
            #[inline(always)]
    pub fn coin_pnl_token_account(&mut self, coin_pnl_token_account: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.coin_pnl_token_account = Some(coin_pnl_token_account);
                    self
    }
            #[inline(always)]
    pub fn pc_pnl_token_account(&mut self, pc_pnl_token_account: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.pc_pnl_token_account = Some(pc_pnl_token_account);
                    self
    }
            #[inline(always)]
    pub fn pnl_owner_account(&mut self, pnl_owner_account: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.pnl_owner_account = Some(pnl_owner_account);
                    self
    }
            #[inline(always)]
    pub fn amm_target_orders(&mut self, amm_target_orders: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.amm_target_orders = Some(amm_target_orders);
                    self
    }
            #[inline(always)]
    pub fn serum_program(&mut self, serum_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.serum_program = Some(serum_program);
                    self
    }
            #[inline(always)]
    pub fn serum_market(&mut self, serum_market: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.serum_market = Some(serum_market);
                    self
    }
            #[inline(always)]
    pub fn serum_event_queue(&mut self, serum_event_queue: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.serum_event_queue = Some(serum_event_queue);
                    self
    }
            #[inline(always)]
    pub fn serum_coin_vault_account(&mut self, serum_coin_vault_account: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.serum_coin_vault_account = Some(serum_coin_vault_account);
                    self
    }
            #[inline(always)]
    pub fn serum_pc_vault_account(&mut self, serum_pc_vault_account: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.serum_pc_vault_account = Some(serum_pc_vault_account);
                    self
    }
            #[inline(always)]
    pub fn serum_vault_signer(&mut self, serum_vault_signer: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.serum_vault_signer = Some(serum_vault_signer);
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
    let accounts = WithdrawPnl {
                              token_program: self.token_program.unwrap_or(solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")),
                                        amm: self.amm.expect("amm is not set"),
                                        amm_config: self.amm_config.expect("amm_config is not set"),
                                        amm_authority: self.amm_authority.expect("amm_authority is not set"),
                                        amm_open_orders: self.amm_open_orders.expect("amm_open_orders is not set"),
                                        pool_coin_token_account: self.pool_coin_token_account.expect("pool_coin_token_account is not set"),
                                        pool_pc_token_account: self.pool_pc_token_account.expect("pool_pc_token_account is not set"),
                                        coin_pnl_token_account: self.coin_pnl_token_account.expect("coin_pnl_token_account is not set"),
                                        pc_pnl_token_account: self.pc_pnl_token_account.expect("pc_pnl_token_account is not set"),
                                        pnl_owner_account: self.pnl_owner_account.expect("pnl_owner_account is not set"),
                                        amm_target_orders: self.amm_target_orders.expect("amm_target_orders is not set"),
                                        serum_program: self.serum_program.expect("serum_program is not set"),
                                        serum_market: self.serum_market.expect("serum_market is not set"),
                                        serum_event_queue: self.serum_event_queue.expect("serum_event_queue is not set"),
                                        serum_coin_vault_account: self.serum_coin_vault_account.expect("serum_coin_vault_account is not set"),
                                        serum_pc_vault_account: self.serum_pc_vault_account.expect("serum_pc_vault_account is not set"),
                                        serum_vault_signer: self.serum_vault_signer.expect("serum_vault_signer is not set"),
                      };
    
    accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
  }
}

  /// `withdraw_pnl` CPI accounts.
  pub struct WithdrawPnlCpiAccounts<'a, 'b> {
          
                    
              pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub amm: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub amm_config: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub amm_authority: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub amm_open_orders: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub pool_coin_token_account: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub pool_pc_token_account: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub coin_pnl_token_account: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub pc_pnl_token_account: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub pnl_owner_account: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub amm_target_orders: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub serum_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub serum_market: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub serum_event_queue: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub serum_coin_vault_account: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub serum_pc_vault_account: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub serum_vault_signer: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `withdraw_pnl` CPI instruction.
pub struct WithdrawPnlCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub amm: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub amm_config: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub amm_authority: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub amm_open_orders: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub pool_coin_token_account: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub pool_pc_token_account: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub coin_pnl_token_account: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub pc_pnl_token_account: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub pnl_owner_account: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub amm_target_orders: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub serum_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub serum_market: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub serum_event_queue: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub serum_coin_vault_account: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub serum_pc_vault_account: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub serum_vault_signer: &'b solana_program::account_info::AccountInfo<'a>,
        }

impl<'a, 'b> WithdrawPnlCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: WithdrawPnlCpiAccounts<'a, 'b>,
          ) -> Self {
    Self {
      __program: program,
              token_program: accounts.token_program,
              amm: accounts.amm,
              amm_config: accounts.amm_config,
              amm_authority: accounts.amm_authority,
              amm_open_orders: accounts.amm_open_orders,
              pool_coin_token_account: accounts.pool_coin_token_account,
              pool_pc_token_account: accounts.pool_pc_token_account,
              coin_pnl_token_account: accounts.coin_pnl_token_account,
              pc_pnl_token_account: accounts.pc_pnl_token_account,
              pnl_owner_account: accounts.pnl_owner_account,
              amm_target_orders: accounts.amm_target_orders,
              serum_program: accounts.serum_program,
              serum_market: accounts.serum_market,
              serum_event_queue: accounts.serum_event_queue,
              serum_coin_vault_account: accounts.serum_coin_vault_account,
              serum_pc_vault_account: accounts.serum_pc_vault_account,
              serum_vault_signer: accounts.serum_vault_signer,
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
    let mut accounts = Vec::with_capacity(17+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.amm.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.amm_config.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.amm_authority.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.amm_open_orders.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool_coin_token_account.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool_pc_token_account.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.coin_pnl_token_account.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pc_pnl_token_account.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.pnl_owner_account.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.amm_target_orders.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.serum_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.serum_market.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.serum_event_queue.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.serum_coin_vault_account.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.serum_pc_vault_account.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.serum_vault_signer.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let data = borsh::to_vec(&WithdrawPnlInstructionData::new()).unwrap();
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::RAYDIUM_AMM_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(18 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.token_program.clone());
                        account_infos.push(self.amm.clone());
                        account_infos.push(self.amm_config.clone());
                        account_infos.push(self.amm_authority.clone());
                        account_infos.push(self.amm_open_orders.clone());
                        account_infos.push(self.pool_coin_token_account.clone());
                        account_infos.push(self.pool_pc_token_account.clone());
                        account_infos.push(self.coin_pnl_token_account.clone());
                        account_infos.push(self.pc_pnl_token_account.clone());
                        account_infos.push(self.pnl_owner_account.clone());
                        account_infos.push(self.amm_target_orders.clone());
                        account_infos.push(self.serum_program.clone());
                        account_infos.push(self.serum_market.clone());
                        account_infos.push(self.serum_event_queue.clone());
                        account_infos.push(self.serum_coin_vault_account.clone());
                        account_infos.push(self.serum_pc_vault_account.clone());
                        account_infos.push(self.serum_vault_signer.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `WithdrawPnl` via CPI.
///
/// ### Accounts:
///
          ///   0. `[]` token_program
                ///   1. `[writable]` amm
          ///   2. `[]` amm_config
          ///   3. `[]` amm_authority
                ///   4. `[writable]` amm_open_orders
                ///   5. `[writable]` pool_coin_token_account
                ///   6. `[writable]` pool_pc_token_account
                ///   7. `[writable]` coin_pnl_token_account
                ///   8. `[writable]` pc_pnl_token_account
                ///   9. `[signer]` pnl_owner_account
                ///   10. `[writable]` amm_target_orders
          ///   11. `[]` serum_program
                ///   12. `[writable]` serum_market
          ///   13. `[]` serum_event_queue
                ///   14. `[writable]` serum_coin_vault_account
                ///   15. `[writable]` serum_pc_vault_account
          ///   16. `[]` serum_vault_signer
#[derive(Clone, Debug)]
pub struct WithdrawPnlCpiBuilder<'a, 'b> {
  instruction: Box<WithdrawPnlCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> WithdrawPnlCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(WithdrawPnlCpiBuilderInstruction {
      __program: program,
              token_program: None,
              amm: None,
              amm_config: None,
              amm_authority: None,
              amm_open_orders: None,
              pool_coin_token_account: None,
              pool_pc_token_account: None,
              coin_pnl_token_account: None,
              pc_pnl_token_account: None,
              pnl_owner_account: None,
              amm_target_orders: None,
              serum_program: None,
              serum_market: None,
              serum_event_queue: None,
              serum_coin_vault_account: None,
              serum_pc_vault_account: None,
              serum_vault_signer: None,
                                __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn token_program(&mut self, token_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_program = Some(token_program);
                    self
    }
      #[inline(always)]
    pub fn amm(&mut self, amm: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.amm = Some(amm);
                    self
    }
      #[inline(always)]
    pub fn amm_config(&mut self, amm_config: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.amm_config = Some(amm_config);
                    self
    }
      #[inline(always)]
    pub fn amm_authority(&mut self, amm_authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.amm_authority = Some(amm_authority);
                    self
    }
      #[inline(always)]
    pub fn amm_open_orders(&mut self, amm_open_orders: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.amm_open_orders = Some(amm_open_orders);
                    self
    }
      #[inline(always)]
    pub fn pool_coin_token_account(&mut self, pool_coin_token_account: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.pool_coin_token_account = Some(pool_coin_token_account);
                    self
    }
      #[inline(always)]
    pub fn pool_pc_token_account(&mut self, pool_pc_token_account: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.pool_pc_token_account = Some(pool_pc_token_account);
                    self
    }
      #[inline(always)]
    pub fn coin_pnl_token_account(&mut self, coin_pnl_token_account: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.coin_pnl_token_account = Some(coin_pnl_token_account);
                    self
    }
      #[inline(always)]
    pub fn pc_pnl_token_account(&mut self, pc_pnl_token_account: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.pc_pnl_token_account = Some(pc_pnl_token_account);
                    self
    }
      #[inline(always)]
    pub fn pnl_owner_account(&mut self, pnl_owner_account: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.pnl_owner_account = Some(pnl_owner_account);
                    self
    }
      #[inline(always)]
    pub fn amm_target_orders(&mut self, amm_target_orders: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.amm_target_orders = Some(amm_target_orders);
                    self
    }
      #[inline(always)]
    pub fn serum_program(&mut self, serum_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.serum_program = Some(serum_program);
                    self
    }
      #[inline(always)]
    pub fn serum_market(&mut self, serum_market: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.serum_market = Some(serum_market);
                    self
    }
      #[inline(always)]
    pub fn serum_event_queue(&mut self, serum_event_queue: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.serum_event_queue = Some(serum_event_queue);
                    self
    }
      #[inline(always)]
    pub fn serum_coin_vault_account(&mut self, serum_coin_vault_account: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.serum_coin_vault_account = Some(serum_coin_vault_account);
                    self
    }
      #[inline(always)]
    pub fn serum_pc_vault_account(&mut self, serum_pc_vault_account: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.serum_pc_vault_account = Some(serum_pc_vault_account);
                    self
    }
      #[inline(always)]
    pub fn serum_vault_signer(&mut self, serum_vault_signer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.serum_vault_signer = Some(serum_vault_signer);
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
        let instruction = WithdrawPnlCpi {
        __program: self.instruction.__program,
                  
          token_program: self.instruction.token_program.expect("token_program is not set"),
                  
          amm: self.instruction.amm.expect("amm is not set"),
                  
          amm_config: self.instruction.amm_config.expect("amm_config is not set"),
                  
          amm_authority: self.instruction.amm_authority.expect("amm_authority is not set"),
                  
          amm_open_orders: self.instruction.amm_open_orders.expect("amm_open_orders is not set"),
                  
          pool_coin_token_account: self.instruction.pool_coin_token_account.expect("pool_coin_token_account is not set"),
                  
          pool_pc_token_account: self.instruction.pool_pc_token_account.expect("pool_pc_token_account is not set"),
                  
          coin_pnl_token_account: self.instruction.coin_pnl_token_account.expect("coin_pnl_token_account is not set"),
                  
          pc_pnl_token_account: self.instruction.pc_pnl_token_account.expect("pc_pnl_token_account is not set"),
                  
          pnl_owner_account: self.instruction.pnl_owner_account.expect("pnl_owner_account is not set"),
                  
          amm_target_orders: self.instruction.amm_target_orders.expect("amm_target_orders is not set"),
                  
          serum_program: self.instruction.serum_program.expect("serum_program is not set"),
                  
          serum_market: self.instruction.serum_market.expect("serum_market is not set"),
                  
          serum_event_queue: self.instruction.serum_event_queue.expect("serum_event_queue is not set"),
                  
          serum_coin_vault_account: self.instruction.serum_coin_vault_account.expect("serum_coin_vault_account is not set"),
                  
          serum_pc_vault_account: self.instruction.serum_pc_vault_account.expect("serum_pc_vault_account is not set"),
                  
          serum_vault_signer: self.instruction.serum_vault_signer.expect("serum_vault_signer is not set"),
                    };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct WithdrawPnlCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                amm: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                amm_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                amm_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                amm_open_orders: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                pool_coin_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                pool_pc_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                coin_pnl_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                pc_pnl_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                pnl_owner_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                amm_target_orders: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                serum_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                serum_market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                serum_event_queue: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                serum_coin_vault_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                serum_pc_vault_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                serum_vault_signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

