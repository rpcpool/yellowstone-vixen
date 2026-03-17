#![allow(dead_code)]

mod check_protobuf_format;

pub use check_protobuf_format::check_protobuf_format;
use yellowstone_vixen_core::Pubkey;

pub fn p(s: &str) -> Pubkey { s.parse().unwrap() }
