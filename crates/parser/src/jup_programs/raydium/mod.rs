use spl_pod::solana_program::pubkey::Pubkey;

pub mod account_parser;

pub mod ix_parser;

pub const RADIUM_V3_PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    165, 213, 202, 158, 4, 207, 93, 181, 144, 183, 20, 186, 47, 227, 44, 177, 89, 19, 63, 193, 193,
    146, 183, 34, 87, 253, 7, 211, 156, 176, 64, 30,
]);
