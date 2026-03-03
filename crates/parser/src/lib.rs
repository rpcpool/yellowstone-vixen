#![deny(
    clippy::disallowed_methods,
    clippy::suspicious,
    clippy::style,
    clippy::clone_on_ref_ptr,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::module_name_repetitions)]
#![allow(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod error;

pub use error::*;

pub mod prelude;

pub use yellowstone_vixen_core::KeyBytes;
yellowstone_vixen_core::pubkey_convert_helpers!(solana_pubkey::Pubkey);

pub fn check_min_accounts_req(actual: usize, expected: usize) -> Result<()> {
    if actual < expected {
        Err(Error::new(format!(
            "Too few accounts provided: expected {expected}, got {actual}"
        )))
    } else {
        Ok(())
    }
}
