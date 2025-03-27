//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::generated::types::TargetOrder;
use borsh::BorshSerialize;
use borsh::BorshDeserialize;


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TargetOrders {
pub discriminator: [u8; 8],
pub owner: [u64; 4],
#[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
pub buy_orders: [TargetOrder; 50],
pub padding1: [u64; 8],
pub target_x: u128,
pub target_y: u128,
pub plan_x_buy: u128,
pub plan_y_buy: u128,
pub plan_x_sell: u128,
pub plan_y_sell: u128,
pub placed_x: u128,
pub placed_y: u128,
pub calc_pnl_x: u128,
pub calc_pnl_y: u128,
#[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
pub sell_orders: [TargetOrder; 50],
pub padding2: [u64; 6],
pub replace_buy_client_id: [u64; 10],
pub replace_sell_client_id: [u64; 10],
pub last_order_numerator: u64,
pub last_order_denominator: u64,
pub plan_orders_cur: u64,
pub place_orders_cur: u64,
pub valid_buy_order_num: u64,
pub valid_sell_order_num: u64,
pub padding3: [u64; 10],
pub free_slot_bits: u128,
}


impl TargetOrders {
      pub const LEN: usize = 2216;
  
  
  
  #[inline(always)]
  pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
    let mut data = data;
    Self::deserialize(&mut data)
  }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for TargetOrders {
  type Error = std::io::Error;

  fn try_from(account_info: &solana_program::account_info::AccountInfo<'a>) -> Result<Self, Self::Error> {
      let mut data: &[u8] = &(*account_info.data).borrow();
      Self::deserialize(&mut data)
  }
}

#[cfg(feature = "fetch")]
pub fn fetch_target_orders(
  rpc: &solana_client::rpc_client::RpcClient,
  address: &solana_program::pubkey::Pubkey,
) -> Result<crate::shared::DecodedAccount<TargetOrders>, std::io::Error> {
  let accounts = fetch_all_target_orders(rpc, &[*address])?;
  Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_target_orders(
  rpc: &solana_client::rpc_client::RpcClient,
  addresses: &[solana_program::pubkey::Pubkey],
) -> Result<Vec<crate::shared::DecodedAccount<TargetOrders>>, std::io::Error> {
    let accounts = rpc.get_multiple_accounts(addresses)
      .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::DecodedAccount<TargetOrders>> = Vec::new();
    for i in 0..addresses.len() {
      let address = addresses[i];
      let account = accounts[i].as_ref()
        .ok_or(std::io::Error::new(std::io::ErrorKind::Other, format!("Account not found: {}", address)))?;
      let data = TargetOrders::from_bytes(&account.data)?;
      decoded_accounts.push(crate::shared::DecodedAccount { address, account: account.clone(), data });
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "fetch")]
pub fn fetch_maybe_target_orders(
  rpc: &solana_client::rpc_client::RpcClient,
  address: &solana_program::pubkey::Pubkey,
) -> Result<crate::shared::MaybeAccount<TargetOrders>, std::io::Error> {
    let accounts = fetch_all_maybe_target_orders(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_maybe_target_orders(
  rpc: &solana_client::rpc_client::RpcClient,
  addresses: &[solana_program::pubkey::Pubkey],
) -> Result<Vec<crate::shared::MaybeAccount<TargetOrders>>, std::io::Error> {
    let accounts = rpc.get_multiple_accounts(addresses)
      .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::MaybeAccount<TargetOrders>> = Vec::new();
    for i in 0..addresses.len() {
      let address = addresses[i];
      if let Some(account) = accounts[i].as_ref() {
        let data = TargetOrders::from_bytes(&account.data)?;
        decoded_accounts.push(crate::shared::MaybeAccount::Exists(crate::shared::DecodedAccount { address, account: account.clone(), data }));
      } else {
        decoded_accounts.push(crate::shared::MaybeAccount::NotFound(address));
      }
    }
  Ok(decoded_accounts)
}

  #[cfg(feature = "anchor")]
  impl anchor_lang::AccountDeserialize for TargetOrders {
      fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
      }
  }

  #[cfg(feature = "anchor")]
  impl anchor_lang::AccountSerialize for TargetOrders {}

  #[cfg(feature = "anchor")]
  impl anchor_lang::Owner for TargetOrders {
      fn owner() -> Pubkey {
        crate::RAYDIUM_AMM_ID
      }
  }

  #[cfg(feature = "anchor-idl-build")]
  impl anchor_lang::IdlBuild for TargetOrders {}

  
  #[cfg(feature = "anchor-idl-build")]
  impl anchor_lang::Discriminator for TargetOrders {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
  }

