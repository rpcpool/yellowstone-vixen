use std::{borrow::Cow, collections::HashMap, pin::Pin};

use futures_util::{Future, FutureExt};
use tracing::{error, warn, Instrument, Span};
use vixen_core::{AccountUpdate, GetPrefilter, ParserId, TransactionUpdate};
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

pub use pipeline_error::Errors as PipelineErrors;

pub mod pipeline_error {
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

#[derive(Debug)]
pub struct Pipeline<P, H>(P, H);

impl<P, H> Pipeline<P, H> {
    #[inline]
    #[must_use]
    pub fn new(parser: P, handlers: H) -> Self { Self(parser, handlers) }
}

impl<P: ParserId, H> ParserId for Pipeline<P, H> {
    #[inline]
    fn id(&self) -> Cow<str> { self.0.id() }
}

impl<P: GetPrefilter, H> GetPrefilter for Pipeline<P, H> {
    #[inline]
    fn prefilter(&self) -> Prefilter { self.0.prefilter() }
}

pub type BoxPipeline<'h, T> = Box<dyn DynPipeline<T> + Send + Sync + 'h>;

pub fn boxed<'h, P: DynPipeline<T> + Send + Sync + 'h, T>(value: P) -> BoxPipeline<'h, T> {
    Box::new(value)
}

impl<P, I> Pipeline<P, I>
where
    for<'i> &'i I: IntoIterator,
    P: Parser,
    for<'i> <&'i I as IntoIterator>::Item: Handler<P::Output>,
{
    async fn handle(&self, value: &P::Input) -> Result<(), PipelineErrors> {
        let parsed = match self.0.parse(value).await {
            Ok(p) => p,
            Err(ParseError::Filtered) => return Ok(()),
            Err(ParseError::Other(e)) => return Err(PipelineErrors::Parse(e)),
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
            Err(PipelineErrors::Handlers(errs))
        }
    }
}

pub trait DynPipeline<T>: std::fmt::Debug + ParserId + GetPrefilter {
    fn handle<'h>(
        &'h self,
        value: &'h T,
    ) -> Pin<Box<dyn Future<Output = Result<(), PipelineErrors>> + Send + 'h>>;
}

impl<T> DynPipeline<T> for std::convert::Infallible {
    fn handle<'h>(
        &'h self,
        _: &'h T,
    ) -> Pin<Box<dyn Future<Output = Result<(), PipelineErrors>> + Send + 'h>> {
        match *self {}
    }
}

impl<P: std::fmt::Debug + Parser + Sync, I: std::fmt::Debug + Sync> DynPipeline<P::Input>
    for Pipeline<P, I>
where
    for<'i> &'i I: IntoIterator,
    P::Input: Sync,
    P::Output: Send + Sync,
    for<'i> <&'i I as IntoIterator>::Item: Handler<P::Output> + Send,
{
    fn handle<'h>(
        &'h self,
        value: &'h P::Input,
    ) -> Pin<Box<dyn Future<Output = Result<(), PipelineErrors>> + Send + 'h>> {
        Box::pin(Pipeline::handle(self, value))
    }
}

impl<'h, T> ParserId for BoxPipeline<'h, T> {
    fn id(&self) -> Cow<str> { <dyn DynPipeline<T>>::id(&**self) }
}

impl<'h, T> GetPrefilter for BoxPipeline<'h, T> {
    #[inline]
    fn prefilter(&self) -> Prefilter { <dyn DynPipeline<T>>::prefilter(&**self) }
}

impl<'j, T> DynPipeline<T> for BoxPipeline<'j, T> {
    #[inline]
    fn handle<'h>(
        &'h self,
        value: &'h T,
    ) -> Pin<Box<dyn Future<Output = Result<(), PipelineErrors>> + Send + 'h>> {
        <dyn DynPipeline<T>>::handle(&**self, value)
    }
}

#[derive(Debug)]
pub struct PipelineSets {
    pub account: PipelineSet<BoxPipeline<'static, AccountUpdate>>,
    pub transaction: PipelineSet<BoxPipeline<'static, TransactionUpdate>>,
}

impl PipelineSets {
    #[must_use]
    pub fn filters(&self) -> Filters {
        Filters::new(
            self.account
                .filters()
                .chain(self.transaction.filters())
                .collect(),
        )
    }
}

#[derive(Debug)]
pub struct PipelineSet<P>(HashMap<String, P>);

impl PipelineSet<std::convert::Infallible> {
    #[allow(clippy::zero_sized_map_values)]
    #[inline]
    #[must_use]
    pub fn empty() -> Self { Self(HashMap::new()) }
}

impl<P> PipelineSet<P> {
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    pub fn map<J: ParserId, F: FnMut(P) -> J>(self, f: F) -> PipelineSet<J> {
        PipelineSet::new(self.0.into_values().map(f))
    }
}

impl<P: ParserId> PipelineSet<P> {
    #[inline]
    pub fn new<I: IntoIterator<Item = P>>(it: I) -> Self { Self::from_iter(it) }
}

impl<P: GetPrefilter> PipelineSet<P> {
    #[inline]
    fn filters(&self) -> impl Iterator<Item = (&str, Prefilter)> {
        self.0.iter().map(|(k, v)| (&**k, v.prefilter()))
    }
}

impl<P> PipelineSet<P> {
    pub(crate) fn get_handlers<I>(&self, it: I) -> Pipelines<P, I> { Pipelines(self, it) }
}

impl<P: ParserId> FromIterator<P> for PipelineSet<P> {
    fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> Self {
        Self(iter.into_iter().map(|i| (i.id().into_owned(), i)).collect())
    }
}

#[derive(Debug)]
pub(crate) struct Pipelines<'m, H, I>(&'m PipelineSet<H>, I);

impl<'m, H, I: IntoIterator> Pipelines<'m, H, I>
where I::Item: AsRef<str> + Send + 'm
{
    fn get_pipelines(self) -> impl Iterator<Item = (I::Item, &'m H)> {
        let Self(pipelines, it) = self;
        it.into_iter().filter_map(|f| {
            let filter = f.as_ref();
            let pipeline = pipelines.0.get(filter);

            if pipeline.is_none() {
                warn!(filter, "No pipeline matched filter on incoming update");
            }

            pipeline.map(|p| (f, p))
        })
    }

    pub fn run<'h, T: Update, M: Instrumenter>(
        self,
        span: Span,
        value: &'h T,
        metrics: &'h Counters<M>,
    ) -> impl Future<Output = ()> + Send + 'h
    where
        H: DynPipeline<T>,
        'm: 'h,
    {
        let _span = span.entered();
        futures_util::future::join_all(self.get_pipelines().map(move |(f, h)| {
            h.handle(value)
                .map(move |r| {
                    metrics.inc_processed(T::TYPE, JobResult::from_pipeline(&r));
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
                .in_current_span()
        }))
        .map(move |v| v.into_iter().collect())
    }
}
