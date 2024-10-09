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
