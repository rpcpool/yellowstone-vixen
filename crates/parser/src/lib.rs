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

pub use error::*;

mod helpers;

#[cfg(feature = "orca")]
pub mod orca;
#[cfg(feature = "raydium")]
pub mod raydium;
#[cfg(feature = "token-extensions")]
pub mod token_extension_program;
#[cfg(feature = "token-program")]
pub mod token_program;

mod error {
    use std::{borrow::Cow, error::Error as StdError};

    pub type Result<T, E = Error> = std::result::Result<T, E>;

    #[derive(Debug, thiserror::Error)]
    #[error("{message}")]
    pub struct Error {
        message: Cow<'static, str>,
        #[source]
        inner: Option<Box<dyn StdError + Send + Sync + 'static>>,
    }

    impl Error {
        pub(crate) fn new<M: Into<Cow<'static, str>>>(message: M) -> Self {
            Self {
                message: message.into(),
                inner: None,
            }
        }

        pub(crate) fn from_inner<
            M: Into<Cow<'static, str>>,
            T: Into<Box<dyn StdError + Send + Sync + 'static>>,
        >(
            message: M,
            inner: T,
        ) -> Self {
            Self {
                message: message.into(),
                inner: Some(inner.into()),
            }
        }
    }

    pub(crate) trait ResultExt<T, E: StdError + Sized + Send + Sync + 'static>:
        Sized
    {
        fn parse_err(self, message: &'static str) -> Result<T, Error>;
    }

    impl<T, E: StdError + Sized + Send + Sync + 'static> ResultExt<T, E> for Result<T, E> {
        fn parse_err(self, message: &'static str) -> Result<T, Error> {
            self.map_err(|e| Error::from_inner(message, e))
        }
    }
}
