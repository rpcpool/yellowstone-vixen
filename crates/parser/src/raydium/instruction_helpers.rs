use borsh::{BorshDeserialize, BorshSerialize};
use yellowstone_vixen_core::Pubkey;

pub const SWAP_IX_DISC: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];

#[derive(Debug, Clone, Copy)]
pub struct SwapAccounts {
    /// The user performing the swap
    pub payer: Pubkey,
    /// The factory state to read protocol fees
    pub amm_config: Pubkey,
    /// The program account of the pool in which the swap will be performed
    pub pool_state: Pubkey,
    /// The user token account for input token
    pub input_token_account: Pubkey,
    /// The user token account for output token
    pub output_token_account: Pubkey,
    /// The vault token account for input token
    pub input_vault: Pubkey,
    /// The vault token account for output token
    pub output_vault: Pubkey,
    /// The program account for the most recent oracle observation
    pub observation_state: Pubkey,
    /// SPL program for token transfers
    pub token_program: Pubkey,
    /// The program account for the tick array
    pub tick_array: Pubkey,
}

pub const SWAP_V2_IX_DISC: [u8; 8] = [43, 4, 237, 11, 26, 201, 30, 98];

#[derive(Debug, Clone, Copy)]
pub struct SwapV2Accounts {
    /// The user performing the swap
    pub payer: Pubkey,
    /// The factory state to read protocol fees
    pub amm_config: Pubkey,
    /// The program account of the pool in which the swap will be performed
    pub pool_state: Pubkey,
    /// The user token account for input token
    pub input_token_account: Pubkey,
    /// The user token account for output token
    pub output_token_account: Pubkey,
    /// The vault token account for input token
    pub input_vault: Pubkey,
    /// The vault token account for output token
    pub output_vault: Pubkey,
    /// The program account for the most recent oracle observation
    pub observation_state: Pubkey,
    /// SPL program for token transfers
    pub token_program: Pubkey,
    /// SPL 2022 Token program
    pub token_2022_program: Pubkey,
    /// Memo program
    pub memo_program: Pubkey,
    /// Input vault mint
    pub input_vault_mint: Pubkey,
    /// Output vault mint
    pub output_vault_mint: Pubkey,
    /// The program account for the tick array
    pub tick_array: Pubkey,
}

#[derive(Debug, Clone, Copy, BorshDeserialize, BorshSerialize)]
pub struct SwapIxData {
    /// The amount of input token to swap
    pub amount: u64,
    /// The minimum amount of output token to receive
    pub other_amount_threshold: u64,
    /// The square root of the maximum price to accept
    pub sqrt_price_limit_x64: u128,
    /// Whether the amount specified is the input or output
    pub is_base_input: bool,
}

#[derive(Debug)]
pub enum RaydiumProgramIx {
    Swap(SwapAccounts, SwapIxData),
    SwapV2(SwapV2Accounts, SwapIxData),
}
