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
// TODO: document everything
#![allow(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod helpers;
#[cfg(feature = "proto")]
pub mod proto;
#[cfg(feature = "token-extensions")]
pub mod token_extension_program;
#[cfg(feature = "token-program")]
pub mod token_program;
