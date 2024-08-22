use std::{collections::HashMap, pin::Pin};

use futures_util::{Future, FutureExt};
use tracing::{error, warn};
use yellowstone_vixen_core::{Filters, ParseError, Parser, Prefilter, Update};

use crate::metrics::{Counters, Instrumenter, JobResult};

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

#[inline]
pub const fn from_fn<F>(f: F) -> FromFn<F> { FromFn(f) }

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FromFn<F>(F);

impl<F: Fn(&T) -> U, T, U: Future<Output = HandlerResult<()>> + Send> Handler<T> for FromFn<F> {
    #[inline]
    fn handle(&self, value: &T) -> impl Future<Output = HandlerResult<()>> + Send { self.0(value) }
}

pub use handler_pack_error::Errors as HandlerPackErrors;

pub mod handler_pack_error {
    use super::BoxedError;

    #[derive(Debug)]
    pub enum Errors {
        Parse(BoxedError),
        Handlers(Vec<BoxedError>),
    }

    impl IntoIterator for Errors {
        type IntoIter = IntoIter;
        type Item = Error;

        fn into_iter(self) -> Self::IntoIter {
            match self {
                Errors::Parse(e) => IntoIter::Parse([e].into_iter()),
                Errors::Handlers(v) => IntoIter::Handlers(v.into_iter()),
            }
        }
    }

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("Error parsing input value")]
        Parse(#[source] BoxedError),
        #[error("Handler returned an error on parsed value")]
        Handler(#[source] BoxedError),
    }

    #[derive(Debug)]
    pub enum IntoIter {
        Parse(std::array::IntoIter<BoxedError, 1>),
        Handlers(std::vec::IntoIter<BoxedError>),
    }

    impl Iterator for IntoIter {
        type Item = Error;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                IntoIter::Parse(o) => o.next().map(Error::Parse),
                IntoIter::Handlers(v) => v.next().map(Error::Handler),
            }
        }
    }
}

// TODO: HandlerPack is also a really bad name (see below)

#[derive(Debug)]
pub struct HandlerPack<P, H>(P, H);

impl<P, H> HandlerPack<P, H> {
    #[inline]
    #[must_use]
    pub fn new(parser: P, handlers: H) -> Self { Self(parser, handlers) }
}

pub fn boxed<'h, P: DynHandlerPack<T> + Send + Sync + 'h, T>(
    value: P,
) -> Box<dyn DynHandlerPack<T> + Send + Sync + 'h> {
    Box::new(value)
}

impl<P, I> HandlerPack<P, I>
where
    for<'i> &'i I: IntoIterator,
    P: Parser,
    for<'i> <&'i I as IntoIterator>::Item: Handler<P::Output>,
{
    async fn handle(&self, value: &P::Input) -> Result<(), HandlerPackErrors> {
        let parsed = match self.0.parse(value).await {
            Ok(p) => p,
            Err(ParseError::Filtered) => return Ok(()),
            Err(ParseError::Other(e)) => return Err(HandlerPackErrors::Parse(e)),
        };
        let parsed = &parsed;

        // TODO: use FuturesUnordered?
        let errs: Vec<_> = futures_util::future::join_all(
            (&self.1)
                .into_iter()
                .map(|h| async move { h.handle(parsed).await }),
        )
        .await
        .into_iter()
        .filter_map(Result::err)
        .collect();

        if errs.is_empty() {
            Ok(())
        } else {
            Err(HandlerPackErrors::Handlers(errs))
        }
    }
}

pub trait GetPrefilter {
    fn prefilter(&self) -> Prefilter;
}

impl GetPrefilter for std::convert::Infallible {
    fn prefilter(&self) -> Prefilter { match *self {} }
}

impl<P: Parser, I> GetPrefilter for HandlerPack<P, I> {
    fn prefilter(&self) -> Prefilter { self.0.prefilter() }
}

impl<T> GetPrefilter for Box<dyn DynHandlerPack<T> + Send + Sync + 'static> {
    #[inline]
    fn prefilter(&self) -> Prefilter { <dyn DynHandlerPack<T>>::prefilter(&**self) }
}

pub trait DynHandlerPack<T>: GetPrefilter {
    fn handle<'h>(
        &'h self,
        value: &'h T,
    ) -> Pin<Box<dyn Future<Output = Result<(), HandlerPackErrors>> + Send + 'h>>;
}

impl<T> DynHandlerPack<T> for std::convert::Infallible {
    fn handle<'h>(
        &'h self,
        _: &'h T,
    ) -> Pin<Box<dyn Future<Output = Result<(), HandlerPackErrors>> + Send + 'h>> {
        match *self {}
    }
}

impl<P: Parser + Sync, I: Sync> DynHandlerPack<P::Input> for HandlerPack<P, I>
where
    for<'i> &'i I: IntoIterator,
    P::Input: Sync,
    P::Output: Send + Sync,
    for<'i> <&'i I as IntoIterator>::Item: Handler<P::Output> + Send,
{
    fn handle<'h>(
        &'h self,
        value: &'h P::Input,
    ) -> Pin<Box<dyn Future<Output = Result<(), HandlerPackErrors>> + Send + 'h>> {
        Box::pin(HandlerPack::handle(self, value))
    }
}

impl<T> DynHandlerPack<T> for Box<dyn DynHandlerPack<T> + Send + Sync + 'static> {
    #[inline]
    fn handle<'h>(
        &'h self,
        value: &'h T,
    ) -> Pin<Box<dyn Future<Output = Result<(), HandlerPackErrors>> + Send + 'h>> {
        <dyn DynHandlerPack<T>>::handle(&**self, value)
    }
}

// TODO: HandlerManager et al are really terrible names, plsfix

#[derive(Debug)]
pub struct HandlerManagers<A, T> {
    pub account: HandlerManager<A>,
    pub transaction: HandlerManager<T>,
}

impl<A: GetPrefilter, T: GetPrefilter> HandlerManagers<A, T> {
    #[must_use]
    pub fn filters(&self) -> Filters {
        Filters::new(
            self.account
                .0
                .iter()
                .map(|(&k, v)| (k, v.prefilter()))
                .chain(self.transaction.0.iter().map(|(&k, v)| (k, v.prefilter())))
                .collect(),
        )
    }
}

#[derive(Debug)]
pub struct HandlerManager<H>(HashMap<&'static str, H>);

impl HandlerManager<std::convert::Infallible> {
    #[allow(clippy::zero_sized_map_values)]
    #[inline]
    #[must_use]
    pub fn empty() -> Self { Self(HashMap::new()) }
}

impl<H> HandlerManager<H> {
    #[inline]
    pub fn new<I: IntoIterator<Item = H>>(it: I) -> Self { Self::from_iter(it) }
}

impl<H> HandlerManager<H> {
    pub(crate) fn get_handlers<I>(&self, it: I) -> Handlers<H, I> { Handlers(self, it) }
}

impl<H> FromIterator<H> for HandlerManager<H> {
    fn from_iter<T: IntoIterator<Item = H>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|i| (std::any::type_name_of_val(&i), i))
                .collect(),
        )
    }
}

#[derive(Debug)]
pub(crate) struct Handlers<'m, H, I>(&'m HandlerManager<H>, I);

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

    pub fn run<'h, T: Update, B: Instrumenter>(
        self,
        value: &'h T,
        metrics: &'h Counters<B>,
    ) -> impl Future<Output = ()> + Send + 'h
    where
        H: DynHandlerPack<T>,
        'm: 'h,
    {
        futures_util::future::join_all(self.get_handlers().map(move |(f, h)| {
            h.handle(value).map(move |r| {
                metrics.inc_processed(T::TYPE, JobResult::from_pack(&r));
                match r {
                    Ok(()) => (),
                    Err(v) => {
                        for e in v {
                            error!(
                                err = %crate::Chain(&e),
                                handler = f.as_ref(),
                                r#type = std::any::type_name::<T>(),
                                "Handler failed",
                            );
                        }
                    },
                }
            })
        }))
        .map(move |v| v.into_iter().collect())
    }
}
