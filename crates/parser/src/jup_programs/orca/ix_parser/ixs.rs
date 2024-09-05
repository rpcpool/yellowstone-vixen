use std::fmt::Debug;

use spl_token_metadata_interface::borsh::{BorshDeserialize, BorshSerialize};
use yellowstone_vixen_core::Pubkey;

use crate::helpers::ReadableInstruction;
#[derive(Debug, Clone, Copy)]
pub struct SwapAccounts {
    pub token_program: Pubkey,
    pub token_authority: Pubkey,
    pub whirlpool: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_array0: Pubkey,
    pub tick_array1: Pubkey,
    pub tick_array2: Pubkey,
    pub oracle: Pubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct SwapIxData {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct SwapV2IxData {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SwapV2Accounts {
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub memo_program: Pubkey,
    pub token_authority: Pubkey,
    pub whirlpool: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_array0: Pubkey,
    pub tick_array1: Pubkey,
    pub tick_array2: Pubkey,
    pub oracle: Pubkey,
}

#[derive(Debug)]
pub enum OrcaProgramIx {
    Swap(ReadableInstruction<SwapAccounts, SwapIxData>),
    SwapV2(ReadableInstruction<SwapV2Accounts, SwapV2IxData>),
}
