mod generated_parser;
mod generated_sdk;
pub use generated::*;
pub use generated_parser::*;
use generated_sdk as generated;
use solana_pubkey::Pubkey;

mod custom_impl;

pub const ID: Pubkey = PUMP_AMM_ID;

// #[cfg(feature = "proto")]
pub mod proto_def {
    #![allow(clippy::large_enum_variant)]

    tonic::include_proto!("vixen.parser.pump_swaps");

    pub const DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("descriptor");
}
