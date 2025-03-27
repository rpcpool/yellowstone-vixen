mod generated_parser;
mod generated_sdk;

pub use generated::*;
pub use generated_parser::*;
use generated_sdk as generated;
use solana_program::pubkey::Pubkey;

pub const ID: Pubkey = PERPETUALS_ID;
