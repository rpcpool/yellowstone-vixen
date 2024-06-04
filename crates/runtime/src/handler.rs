use std::{collections::HashMap, ops, pin::Pin};

use futures_util::{Future, FutureExt, TryFutureExt};
use tracing::{error, warn};
use yellowstone_vixen_core::{
    AccountUpdate, Filters, ParseError, ParseResult, Parser, Prefilter, TransactionUpdate,
};

type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type HandlerResult<T> = Result<T, BoxedError>;

pub trait Handler<T> {
    fn handle(&self, value: &T) -> impl Future<Output = HandlerResult<()>> + Send;
}

impl<T: Handler<U>, U> Handler<U> for &T {
    #[inline]
    fn handle(&self, value: &U) -> impl Future<Output = HandlerResult<()>> + Send {
        <T as Handler<U>>::handle(self, value)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FromFn<F>(F);

impl<F: Fn(&T) -> U, T, U: Future<Output = HandlerResult<()>> + Send> Handler<T> for FromFn<F> {
    #[inline]
    fn handle(&self, value: &T) -> impl Future<Output = HandlerResult<()>> + Send { self.0(value) }
}

// TODO: this should probably be merged into a cratewide runtime error?
#[derive(Debug, thiserror::Error)]
pub enum HandlerPackError {
    #[error("Error parsing input value")]
    Parse(#[source] BoxedError),
    #[error("Handler returned an error on parsed value")]
    Handler(#[source] BoxedError),
}

// TODO: HandlerPack is also a really bad name (see below)

struct HandlerPack<P, H>(P, H);

impl<P, I> HandlerPack<P, I> {
    // TODO: figure out how to ditch the Vec at some point - probably just pull in Either
    async fn handle<'h, T>(&'h self, value: &'h T) -> Result<(), Vec<HandlerPackError>>
    where
        for<'i> &'i I: IntoIterator,
        P: Parser<T>,
        for<'i> <&'i I as IntoIterator>::Item: Handler<P::Output>,
    {
        let parsed = match self.0.parse(value).await {
            Ok(p) => p,
            Err(ParseError::Filtered) => return Ok(()),
            Err(ParseError::Other(e)) => return Err(vec![HandlerPackError::Parse(e)]),
        };
        let parsed = &parsed;

        // TODO: use futuresunordered?
        let errs: Vec<_> = futures_util::future::join_all(
            (&self.1)
                .into_iter()
                .map(|h| async move { h.handle(parsed).await }),
        )
        .await
        .into_iter()
        .filter_map(|r| r.err().map(HandlerPackError::Handler))
        .collect();

        if errs.is_empty() { Ok(()) } else { Err(errs) }
    }
}

impl<P, H> From<(P, H)> for HandlerPack<P, H> {
    #[inline]
    fn from(value: (P, H)) -> Self {
        let (parser, handlers) = value;
        Self(parser, handlers)
    }
}

pub trait DynHandlerPack<T> {
    fn handle<'h>(
        &'h self,
        value: &'h T,
    ) -> Pin<Box<dyn Future<Output = Result<(), Vec<HandlerPackError>>> + Send + 'h>>;
}

impl<P: Parser<T> + Sync, I: Sync, T: Sync> DynHandlerPack<T> for HandlerPack<P, I>
where
    for<'i> &'i I: IntoIterator,
    P::Output: Send + Sync,
    for<'i> <&'i I as IntoIterator>::Item: Handler<P::Output> + Send,
{
    fn handle<'h>(
        &'h self,
        value: &'h T,
    ) -> Pin<Box<dyn Future<Output = Result<(), Vec<HandlerPackError>>> + Send + 'h>> {
        Box::pin(HandlerPack::handle(self, value))
    }
}

impl<T> DynHandlerPack<T> for Box<dyn DynHandlerPack<T> + Send + Sync + 'static> {
    fn handle<'h>(
        &'h self,
        value: &'h T,
    ) -> Pin<Box<dyn Future<Output = Result<(), Vec<HandlerPackError>>> + Send + 'h>> {
        <dyn DynHandlerPack<T>>::handle(self, value)
    }
}

// TODO: HandlerManager et al are really terrible names, plsfix

pub struct HandlerManagers<A, T> {
    pub account: HandlerManager<A>,
    pub transaction: HandlerManager<T>,
}

impl<A, T> HandlerManagers<A, T> {
    pub fn filters(&self) -> Filters { Filters::new(todo!()) }
}

pub struct HandlerManager<H>(HashMap<String, H>);

impl<H> HandlerManager<H> {
    #[inline]
    pub fn new<T: IntoIterator<Item = (String, I)>, I: Into<H>>(it: T) -> Self {
        Self::from_iter(it)
    }
}

impl<H> HandlerManager<H> {
    pub fn get_handlers<I>(&self, it: I) -> Handlers<H, I> { Handlers(self, it) }
}

impl<H, I: Into<H>> FromIterator<(String, I)> for HandlerManager<H> {
    fn from_iter<T: IntoIterator<Item = (String, I)>>(iter: T) -> Self {
        Self(iter.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

pub struct Handlers<'m, H, I>(&'m HandlerManager<H>, I);

impl<'m, H, I: IntoIterator> Handlers<'m, H, I>
where I::Item: AsRef<str> + Send + 'm
{
    fn get_handlers(self) -> impl Iterator<Item = (I::Item, &'m H)> {
        let Self(manager, it) = self;
        it.into_iter().filter_map(|f| {
            let filter = f.as_ref();
            let handler = manager.0.get(filter);

            if handler.is_none() {
                warn!(filter, "No handler matched filter on incoming update");
            }

            handler.map(|p| (f, p))
        })
    }

    pub fn run<T>(self, value: &'m T) -> impl Future<Output = ()> + Send + 'm
    where H: DynHandlerPack<T> {
        futures_util::future::join_all(self.get_handlers().map(move |(f, h)| {
            h.handle(value).map(move |r| match r {
                Ok(()) => (),
                Err(v) => {
                    for e in v {
                        error!(
                            err = ?anyhow::Error::from(e),
                            handler = f.as_ref(),
                            r#type = std::any::type_name::<T>(),
                            "Handler failed",
                        );
                    }
                },
            })
        }))
        .map(|v| v.into_iter().collect())
    }
}
