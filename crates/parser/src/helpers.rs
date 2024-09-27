use spl_pod::solana_program::{program_option::COption, pubkey::Pubkey};
use spl_token::instruction::MAX_SIGNERS;
use yellowstone_vixen_core::KeyBytes;

use crate::{Error, Result};

pub fn check_min_accounts_req(actual: usize, expected: usize) -> Result<()> {
    if actual < expected {
        Err(Error::new(format!(
            "Too few accounts provided: expected {expected}, got {actual}"
        )))
    } else {
        Ok(())
    }
}

yellowstone_vixen_core::pubkey_convert_helpers!(spl_pod::solana_program::pubkey::Pubkey);
#[cfg(feature = "proto")]
yellowstone_vixen_core::proto_helper_traits!();

// `ELGAMAL_PUBKEY_LEN` is not publicly exported but is defined to be 32
pub type ElGamalPubkeyBytes = yellowstone_vixen_core::KeyBytes<32>;

pub trait FromCOptionPubkeyToOptString {
    fn to_opt_string(self) -> Option<String>;
}

pub trait FromVecPubkeyToVecString {
    fn to_string_vec(self) -> Vec<String>;
}

pub trait FromOptPubkeyToOptString {
    fn to_opt_string(self) -> Option<String>;
}

impl FromOptPubkeyToOptString for KeyBytes<32> {
    fn to_opt_string(self) -> Option<String> { Some(self.to_string()) }
}

impl<T: ToString> FromVecPubkeyToVecString for Vec<T> {
    fn to_string_vec(self) -> Vec<String> { self.into_iter().map(|p| p.to_string()).collect() }
}

impl FromVecPubkeyToVecString for [Pubkey; MAX_SIGNERS] {
    fn to_string_vec(self) -> Vec<String> {
        self.iter().map(std::string::ToString::to_string).collect()
    }
}

impl<A: ToString> FromOptPubkeyToOptString for Option<A> {
    fn to_opt_string(self) -> Option<String> { self.map(|p| p.to_string()) }
}

impl<A: ToString> FromCOptionPubkeyToOptString for COption<A> {
    fn to_opt_string(self) -> Option<String> {
        match self {
            COption::Some(val) => Some(val.to_string()),
            COption::None => None,
        }
    }
}
