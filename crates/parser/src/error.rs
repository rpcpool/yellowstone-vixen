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
    pub fn new<M: Into<Cow<'static, str>>>(message: M) -> Self {
        Self {
            message: message.into(),
            inner: None,
        }
    }

    pub fn from_inner<
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

pub trait ResultExt<T, E: StdError + Sized + Send + Sync + 'static>: Sized {
    fn parse_err(self, message: &'static str) -> Result<T, Error>;
}

impl<T, E: StdError + Sized + Send + Sync + 'static> ResultExt<T, E> for Result<T, E> {
    fn parse_err(self, message: &'static str) -> Result<T, Error> {
        self.map_err(|e| Error::from_inner(message, e))
    }
}
