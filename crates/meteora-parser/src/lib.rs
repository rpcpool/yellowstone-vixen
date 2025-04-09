mod generated_parser;
mod generated_sdk;
pub use generated::*;
pub use generated_parser::*;
use generated_sdk as generated;
use solana_program::pubkey::Pubkey;

pub const ID: Pubkey = LB_CLMM_ID;

// #[cfg(feature = "proto")]
pub mod proto_def {
    tonic::include_proto!("proto_def");

    pub const DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("proto_def");
}
