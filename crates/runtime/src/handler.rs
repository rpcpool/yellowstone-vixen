//! Helper types for bundling [Vixen parsers](crate::vixen_core::Parser) and
//! handler callbacks.

use std::{borrow::Cow, collections::HashMap, pin::Pin};

use futures_util::{Future, FutureExt, StreamExt};
use smallvec::SmallVec;
use tracing::{trace, Instrument, Span};
use vixen_core::{
    instruction::Path as CpiPath, AccountUpdate, BlockMetaUpdate, BlockUpdate, GetPrefilter,
    ParserId, SlotUpdate, TransactionUpdate,
};
use yellowstone_vixen_core::{Filters, ParseError, Parser, Prefilter};

#[cfg(feature = "prometheus")]
use crate::metrics;

type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;
/// The result returned by a handler.
pub type HandlerResult<T> = Result<T, BoxedError>;

// --- starttx 5PkaEdcz...
//    >>> 3 ENTER
//      > 3.1 tx 5PkaEdcz...
//      > 3.3 tx 5PkaEdcz...
//      > 3.4 tx 5PkaEdcz...
//    <<< 3 RETURN
//    >>> 4 ENTER
//      > 4.2 tx 5PkaEdcz...
//    <<< 4 RETURN
//    > 5 tx 5PkaEdcz...
// ==

/// More callback hooks from transaction traversal.
#[derive(Debug)]
pub enum LifecycleEvent<'a> {
    /// A transaction has started.
    TxStart,
    /// A transaction has ended.
    TxEnd,
    /// CPI call from the provided path has been entered.
    /// note: called for all CPI enters - not only the filtered ones
    CpiEnter {
        /// CPI caller path (i.e. the parent in the parent-child relation)
        caller_cpi_path: &'a CpiPath,
    },
    /// CPI call returned to the provided path.
    /// note: called for all CPI returns - not only the filtered ones
    CpiReturn {
        /// CPI caller path (i.e. the parent in the parent-child relation)
        caller_cpi_path: &'a CpiPath,
    },
}

/// A handler callback for a parsed value and its corresponding raw event.
pub trait Handler<T, R>
where R: Sync
{
    /// Consume the parsed value together with the raw event.
    fn handle(&self, value: &T, raw_event: &R) -> impl Future<Output = HandlerResult<()>> + Send;

    /// Called on lifecycle events (transaction start/end, CPI enter/return).
    ///
    /// / ! \ Lifecycle delivery is fail-fast. / ! \
    /// If a lifecycle handler returns an error, dispatch stops immediately and
    /// subsequent lifecycle events for the current transaction are not guaranteed
    /// to be emitted. Consumers must not rely on receiving a complete, balanced
    /// lifecycle sequence (for example `CpiEnter` without `CpiReturn`, or `TxStart`
    /// without `TxEnd`).
    fn handle_lifecycle(
        &self,
        _txn: &TransactionUpdate,
        _instruction_shared: &InstructionShared,
        _event: &LifecycleEvent<'_>,
    ) -> impl Future<Output = HandlerResult<()>> + Send {
        async { Ok(()) }
    }
}

impl<T: Handler<U, R>, U, R> Handler<U, R> for &T
where R: Sync
{
    #[inline]
    fn handle(&self, value: &U, raw_event: &R) -> impl Future<Output = HandlerResult<()>> + Send {
        <T as Handler<U, R>>::handle(self, value, raw_event)
    }

    #[inline]
    fn handle_lifecycle(
        &self,
        txn: &TransactionUpdate,
        instruction_shared: &InstructionShared,
        event: &LifecycleEvent<'_>,
    ) -> impl Future<Output = HandlerResult<()>> + Send {
        <T as Handler<U, R>>::handle_lifecycle(self, txn, instruction_shared, event)
    }
}

pub(crate) use pipeline_error::Errors as PipelineErrors;
use vixen_core::instruction::InstructionShared;

mod pipeline_error {
    use smallvec::SmallVec;

    use super::BoxedError;

    #[derive(Debug, Clone, Copy)]
    #[must_use]
    pub struct Handled(());

    impl Handled {
        #[inline]
        pub fn as_unit(self) { let Self(()) = self; }
    }

    #[derive(Debug)]
    pub enum Errors {
        Parse(BoxedError),
        Handlers(SmallVec<[BoxedError; 1]>),
        AlreadyHandled(Handled),
    }

    impl Errors {
        #[inline]
        #[must_use]
        pub fn parse<E: std::error::Error + Send + Sync + 'static>(e: E) -> Self {
            Self::Parse(Box::new(e))
        }

        pub fn handle<T>(self, handler: &str) -> Handled {
            for e in self {
                tracing::error!(
                    err = %crate::Chain(&e),
                    handler,
                    r#type = std::any::type_name::<T>(),
                    "Handler failed",
                );
            }

            Handled(())
        }
    }

    impl IntoIterator for Errors {
        type IntoIter = IntoIter;
        type Item = Error;

        fn into_iter(self) -> Self::IntoIter {
            match self {
                Errors::Parse(e) => IntoIter::Parse([e].into_iter()),
                Errors::Handlers(v) => IntoIter::Handlers(v.into_iter()),
                Errors::AlreadyHandled(Handled(())) => IntoIter::AlreadyHandled,
            }
        }
    }

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("Error parsing input value: ({0})")]
        Parser(#[source] BoxedError),
        #[error("Handler returned an error on parsed value: ({0})")]
        Handler(#[source] BoxedError),
    }

    #[derive(Debug)]
    pub enum IntoIter {
        Parse(std::array::IntoIter<BoxedError, 1>),
        Handlers(smallvec::IntoIter<[BoxedError; 1]>),
        AlreadyHandled,
    }

    impl Iterator for IntoIter {
        type Item = Error;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::Parse(o) => o.next().map(Error::Parser),
                Self::Handlers(v) => v.next().map(Error::Handler),
                Self::AlreadyHandled => None,
            }
        }
    }
}

/// A parser and a set of handlers its output is passed to.
#[derive(Debug)]
pub struct Pipeline<P, H>(P, H);

impl<P, H> Pipeline<P, H> {
    /// Create a new pipeline from a parser and a list of handlers.
    #[inline]
    #[must_use]
    pub fn new(parser: P, handlers: H) -> Self { Self(parser, handlers) }
}

impl<P: ParserId, H> ParserId for Pipeline<P, H> {
    #[inline]
    fn id(&self) -> Cow<'static, str> { self.0.id() }
}

impl<P: GetPrefilter, H> GetPrefilter for Pipeline<P, H> {
    #[inline]
    fn prefilter(&self) -> Prefilter { self.0.prefilter() }
}

/// A boxed pipeline.
pub type BoxPipeline<'h, T> = Box<dyn DynPipeline<T> + Send + Sync + 'h>;

impl<P, I> Pipeline<P, I>
where
    for<'i> &'i I: IntoIterator,
    P: Parser,
    P::Input: Sync,
    for<'i> <&'i I as IntoIterator>::Item: Handler<P::Output, P::Input>,
{
    /// Handle fn for `Pipeline`
    ///
    /// # Errors
    /// If any of the related handlers executions errors, returns those errors
    pub async fn handle(&self, value: &P::Input) -> Result<(), PipelineErrors> {
        let parsed = match self
            .0
            .parse(value)
            .instrument(tracing::info_span!("vixen.parse",))
            .await
        {
            Ok(p) => p,
            Err(ParseError::Filtered) => return Ok(()),
            Err(ParseError::Other(e)) => return Err(PipelineErrors::Parse(e)),
        };
        let parsed = &parsed;

        let errs = (&self.1)
            .into_iter()
            .map(|h| async move {
                h.handle(parsed, value)
                    .instrument(tracing::info_span!("vixen.handle",))
                    .await
            })
            .collect::<futures_util::stream::FuturesUnordered<_>>()
            .filter_map(|r| async move { r.err() })
            .collect::<SmallVec<[_; 1]>>()
            .await;

        if errs.is_empty() {
            Ok(())
        } else {
            Err(PipelineErrors::Handlers(errs))
        }
    }

    /// Notify all handlers of a lifecycle event.
    ///
    /// # Errors
    /// If any handler returns an error, all errors are collected and returned.
    pub async fn handle_lifecycle(
        &self,
        txn: &TransactionUpdate,
        instruction_shared: &InstructionShared,
        event: &LifecycleEvent<'_>,
    ) -> Result<(), PipelineErrors> {
        let errs = (&self.1)
            .into_iter()
            .map(|h| async move { h.handle_lifecycle(txn, instruction_shared, event).await })
            .collect::<futures_util::stream::FuturesUnordered<_>>()
            .filter_map(|r| async move { r.err() })
            .collect::<SmallVec<[_; 1]>>()
            .await;

        if errs.is_empty() {
            Ok(())
        } else {
            Err(PipelineErrors::Handlers(errs))
        }
    }
}

/// Object-safe trait for parsing and handling values.
pub trait DynPipeline<T>: std::fmt::Debug + ParserId + GetPrefilter {
    /// Pass the provided value to the parser and handlers comprising this
    /// pipeline.
    fn handle<'h>(
        &'h self,
        value: &'h T,
    ) -> Pin<Box<dyn Future<Output = Result<(), PipelineErrors>> + Send + 'h>>;

    /// Optional callback for lifecycle events (tx start/end, CPI enter/return).
    fn handle_lifecycle<'h>(
        &'h self,
        _txn: &'h TransactionUpdate,
        _instruction_shared: &'h InstructionShared,
        _event: &'h LifecycleEvent<'h>,
    ) -> Pin<Box<dyn Future<Output = Result<(), PipelineErrors>> + Send + 'h>> {
        Box::pin(async move { Ok(()) })
    }
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
    for<'i> <&'i I as IntoIterator>::Item: Handler<P::Output, P::Input> + Send,
{
    fn handle<'h>(
        &'h self,
        value: &'h P::Input,
    ) -> Pin<Box<dyn Future<Output = Result<(), PipelineErrors>> + Send + 'h>> {
        Box::pin(Pipeline::handle(self, value))
    }

    fn handle_lifecycle<'h>(
        &'h self,
        txn: &'h TransactionUpdate,
        instruction_shared: &'h InstructionShared,
        event: &'h LifecycleEvent<'h>,
    ) -> Pin<Box<dyn Future<Output = Result<(), PipelineErrors>> + Send + 'h>> {
        Box::pin(Pipeline::handle_lifecycle(self, txn, instruction_shared, event))
    }
}

impl<T> ParserId for BoxPipeline<'_, T> {
    fn id(&self) -> Cow<'static, str> { <dyn DynPipeline<T>>::id(&**self) }
}

impl<T> GetPrefilter for BoxPipeline<'_, T> {
    #[inline]
    fn prefilter(&self) -> Prefilter { <dyn DynPipeline<T>>::prefilter(&**self) }
}

impl<T> DynPipeline<T> for BoxPipeline<'_, T> {
    #[inline]
    fn handle<'h>(
        &'h self,
        value: &'h T,
    ) -> Pin<Box<dyn Future<Output = Result<(), PipelineErrors>> + Send + 'h>> {
        <dyn DynPipeline<T>>::handle(&**self, value)
    }

    #[inline]
    fn handle_lifecycle<'h>(
        &'h self,
        txn: &'h TransactionUpdate,
        instruction_shared: &'h InstructionShared,
        event: &'h LifecycleEvent<'h>,
    ) -> Pin<Box<dyn Future<Output = Result<(), PipelineErrors>> + Send + 'h>> {
        <dyn DynPipeline<T>>::handle_lifecycle(&**self, txn, instruction_shared, event)
    }
}

#[derive(Debug)]
pub(crate) struct PipelineSets {
    pub account: PipelineSet<BoxPipeline<'static, AccountUpdate>>,
    pub transaction: PipelineSet<BoxPipeline<'static, TransactionUpdate>>,
    pub instruction: PipelineSet<BoxPipeline<'static, TransactionUpdate>>,
    pub block_meta: PipelineSet<BoxPipeline<'static, BlockMetaUpdate>>,
    pub block: PipelineSet<BoxPipeline<'static, BlockUpdate>>,
    pub slot: PipelineSet<BoxPipeline<'static, SlotUpdate>>,
}

impl PipelineSets {
    #[must_use]
    pub fn filters(&self) -> Filters {
        Filters::new(
            self.account
                .filters()
                .chain(self.transaction.filters())
                .chain(self.instruction.filters())
                .chain(self.block_meta.filters())
                .chain(self.block.filters())
                .chain(self.slot.filters())
                .collect(),
        )
    }
}

#[derive(Debug)]
pub(crate) struct PipelineSet<P>(HashMap<String, P>);

impl<P> PipelineSet<P> {
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    #[inline]
    #[must_use]
    pub fn new() -> Self { Self(HashMap::new()) }

    #[inline]
    pub fn insert(&mut self, key: String, value: P) -> Option<P> { self.0.insert(key, value) }
}

impl<P: GetPrefilter> PipelineSet<P> {
    #[inline]
    fn filters(&self) -> impl Iterator<Item = (String, Prefilter)> {
        // # Each filter key is going to be the parser::id()
        self.0
            .iter()
            .map(|(k, v)| (k.clone(), v.prefilter()))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<P> PipelineSet<P> {
    pub(crate) fn get_handlers<I>(&'_ self, it: I) -> Pipelines<'_, P, I> { Pipelines(self, it) }
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
                trace!(filter, "No pipeline matched filter on incoming update");
            }

            pipeline.map(|p| (f, p))
        })
    }

    pub fn run<'h, T>(
        self,
        span: Span,
        value: &'h T,
        #[cfg(feature = "prometheus")] update_type: metrics::UpdateType,
    ) -> impl Future<Output = ()> + Send + 'h
    where
        H: DynPipeline<T>,
        'm: 'h,
    {
        let _span = span.entered();
        futures_util::future::join_all(self.get_pipelines().map(move |(f, h)| {
            h.handle(value)
                .map(move |r| {
                    #[cfg(feature = "prometheus")]
                    metrics::increment_processed_updates(&r, update_type);

                    match r {
                        Ok(()) => (),
                        Err(v) => v.handle::<T>(f.as_ref()).as_unit(),
                    }
                })
                .in_current_span()
        }))
            .map(move |v| v.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    use vixen_core::instruction::{InstructionShared, InstructionUpdate};
    use vixen_core::{ParseError, Parser, Prefilter, TransactionUpdate};
    use yellowstone_grpc_proto::prelude::MessageHeader;
    use yellowstone_grpc_proto::solana::storage::confirmed_block::{
        CompiledInstruction, Message, Transaction, TransactionStatusMeta,
    };
    use yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo;

    use super::{Handler, HandlerResult, LifecycleEvent, Pipeline, PipelineErrors};
    use crate::handler::DynPipeline;
    use crate::instruction::InstructionPipeline;

    // -- helpers ----------------------------------------------------------

    /// A trivial parsed output type.
    #[derive(Debug)]
    struct Unit;

    /// A parser that always succeeds and returns `Unit`.
    #[derive(Debug)]
    struct OkParser;

    impl Parser for OkParser {
        type Input = TransactionUpdate;
        type Output = Unit;

        fn id(&self) -> Cow<'static, str> { "OkParser".into() }

        fn prefilter(&self) -> Prefilter { Prefilter::default() }

        async fn parse(
            &self,
            _value: &Self::Input,
        ) -> Result<Self::Output, ParseError> {
            Ok(Unit)
        }
    }

    /// Handler whose `handle_lifecycle` always returns an error.
    #[derive(Debug)]
    struct FailLifecycle;

    impl Handler<Unit, TransactionUpdate> for FailLifecycle {
        async fn handle(
            &self,
            _value: &Unit,
            _raw: &TransactionUpdate,
        ) -> HandlerResult<()> {
            Ok(())
        }

        async fn handle_lifecycle(
            &self,
            _txn: &TransactionUpdate,
            _instruction_shared: &InstructionShared,
            _event: &LifecycleEvent<'_>,
        ) -> HandlerResult<()> {
            Err("lifecycle error".into())
        }
    }

    /// Handler whose `handle_lifecycle` always succeeds (default impl).
    struct OkHandler;

    impl Handler<Unit, TransactionUpdate> for OkHandler {
        async fn handle(
            &self,
            _value: &Unit,
            _raw: &TransactionUpdate,
        ) -> HandlerResult<()> {
            Ok(())
        }
    }

    /// Handler that counts how many times `handle_lifecycle` is called.
    struct CountingHandler {
        count: AtomicUsize,
    }

    impl CountingHandler {
        fn new() -> Self {
            Self {
                count: AtomicUsize::new(0),
            }
        }

        fn count(&self) -> usize { self.count.load(Ordering::SeqCst) }
    }

    impl Handler<Unit, TransactionUpdate> for CountingHandler {
        async fn handle(
            &self,
            _value: &Unit,
            _raw: &TransactionUpdate,
        ) -> HandlerResult<()> {
            Ok(())
        }

        async fn handle_lifecycle(
            &self,
            _txn: &TransactionUpdate,
            _instruction_shared: &InstructionShared,
            _event: &LifecycleEvent<'_>,
        ) -> HandlerResult<()> {
            self.count.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    // -- tests ------------------------------------------------------------

    #[tokio::test]
    async fn pipeline_handle_lifecycle_propagates_error() {
        let pipeline = Pipeline::new(OkParser, [FailLifecycle]);
        let txn = TransactionUpdate::default();
        let shared = InstructionShared::default();

        let result = pipeline
            .handle_lifecycle(&txn, &shared, &LifecycleEvent::TxStart)
            .await;

        assert!(result.is_err(), "expected lifecycle error to propagate");
    }

    #[tokio::test]
    async fn pipeline_handle_lifecycle_ok_when_handler_succeeds() {
        let pipeline = Pipeline::new(OkParser, [OkHandler]);
        let txn = TransactionUpdate::default();
        let shared = InstructionShared::default();

        let result = pipeline
            .handle_lifecycle(&txn, &shared, &LifecycleEvent::TxEnd)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn dyn_pipeline_handle_lifecycle_propagates_error() {
        let pipeline = Pipeline::new(OkParser, [FailLifecycle]);
        let txn = TransactionUpdate::default();
        let shared = InstructionShared::default();

        let dyn_pipe: &dyn DynPipeline<TransactionUpdate> = &pipeline;
        let result = dyn_pipe
            .handle_lifecycle(&txn, &shared, &LifecycleEvent::TxStart)
            .await;

        assert!(result.is_err(), "DynPipeline should propagate lifecycle errors");
    }

    #[tokio::test]
    async fn lifecycle_error_from_one_handler_is_collected() {
        // One handler succeeds, one fails — the error should still surface.
        let pipeline = Pipeline::new(OkParser, [FailLifecycle]);
        let txn = TransactionUpdate::default();
        let shared = InstructionShared::default();

        let result = pipeline
            .handle_lifecycle(&txn, &shared, &LifecycleEvent::CpiEnter {
                caller_cpi_path: &vec![0u32].into(),
            })
            .await;

        match result {
            Err(PipelineErrors::Handlers(errs)) => {
                assert_eq!(errs.len(), 1, "expected exactly one handler error");
            },
            other => panic!("expected Handlers error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn handler_default_lifecycle_returns_ok() {
        let handler = OkHandler;
        let txn = TransactionUpdate::default();
        let shared = InstructionShared::default();

        let result = Handler::<Unit, TransactionUpdate>::handle_lifecycle(
            &handler,
            &txn,
            &shared,
            &LifecycleEvent::TxStart,
        )
        .await;

        assert!(result.is_ok(), "default handle_lifecycle should return Ok");
    }

    #[tokio::test]
    async fn lifecycle_called_for_each_event_variant() {
        let handler = CountingHandler::new();
        let pipeline = Pipeline::new(OkParser, [&handler]);
        let txn = TransactionUpdate::default();
        let shared = InstructionShared::default();
        let cpi_path = vec![1u32].into();

        let events = [
            LifecycleEvent::TxStart,
            LifecycleEvent::CpiEnter {
                caller_cpi_path: &cpi_path,
            },
            LifecycleEvent::CpiReturn {
                caller_cpi_path: &cpi_path,
            },
            LifecycleEvent::TxEnd,
        ];

        for event in &events {
            pipeline
                .handle_lifecycle(&txn, &shared, event)
                .await
                .expect("lifecycle should succeed");
        }

        assert_eq!(handler.count(), 4, "expected one call per lifecycle event");
    }

    // -- instruction-level helpers ----------------------------------------

    /// Build a minimal `TransactionUpdate` with one instruction whose
    /// `program_id_index` points at a single 32-byte account key.
    fn make_txn_with_one_instruction() -> TransactionUpdate {
        let account_key = vec![0u8; 32];

        TransactionUpdate {
            transaction: Some(SubscribeUpdateTransactionInfo {
                signature: vec![0u8; 64],
                is_vote: false,
                index: 0,
                transaction: Some(Transaction {
                    signatures: vec![],
                    message: Some(Message {
                        header: Some(MessageHeader {
                            num_required_signatures: 1,
                            num_readonly_signed_accounts: 0,
                            num_readonly_unsigned_accounts: 0,
                        }),
                        account_keys: vec![account_key],
                        recent_blockhash: vec![0u8; 32],
                        instructions: vec![CompiledInstruction {
                            program_id_index: 0,
                            accounts: vec![],
                            data: vec![],
                        }],
                        versioned: false,
                        address_table_lookups: vec![],
                    }),
                }),
                meta: Some(TransactionStatusMeta {
                    err: None,
                    fee: 0,
                    pre_balances: vec![0],
                    post_balances: vec![0],
                    inner_instructions: vec![],
                    inner_instructions_none: false,
                    log_messages: vec![],
                    log_messages_none: false,
                    pre_token_balances: vec![],
                    post_token_balances: vec![],
                    rewards: vec![],
                    loaded_writable_addresses: vec![],
                    loaded_readonly_addresses: vec![],
                    return_data: None,
                    return_data_none: true,
                    compute_units_consumed: Some(0),
                    cost_units: None,
                }),
            }),
            slot: 1,
        }
    }

    /// Parser for `InstructionUpdate` that always succeeds.
    #[derive(Debug)]
    struct IxParser;

    impl Parser for IxParser {
        type Input = InstructionUpdate;
        type Output = Unit;

        fn id(&self) -> Cow<'static, str> { "IxParser".into() }

        fn prefilter(&self) -> Prefilter { Prefilter::default() }

        async fn parse(
            &self,
            _value: &Self::Input,
        ) -> Result<Self::Output, vixen_core::ParseError> {
            Ok(Unit)
        }
    }

    /// Owned equivalent of `LifecycleEvent` for collecting in tests.
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum OwnedLifecycleEvent {
        TxStart,
        TxEnd,
        CpiEnter { caller_cpi_path: Vec<u32> },
        CpiReturn { caller_cpi_path: Vec<u32> },
    }

    impl OwnedLifecycleEvent {
        fn from_ref(event: &LifecycleEvent<'_>) -> Self {
            match event {
                LifecycleEvent::TxStart => Self::TxStart,
                LifecycleEvent::TxEnd => Self::TxEnd,
                LifecycleEvent::CpiEnter { caller_cpi_path } => Self::CpiEnter {
                    caller_cpi_path: caller_cpi_path.as_slice().to_vec(),
                },
                LifecycleEvent::CpiReturn { caller_cpi_path } => Self::CpiReturn {
                    caller_cpi_path: caller_cpi_path.as_slice().to_vec(),
                },
            }
        }
    }

    /// Handler for `InstructionUpdate` that always fails on `handle` but
    /// records every lifecycle event it receives.
    #[derive(Debug)]
    struct FailHandleRecordLifecycleIx {
        events: Mutex<Vec<OwnedLifecycleEvent>>,
    }

    impl FailHandleRecordLifecycleIx {
        fn new() -> Self {
            Self {
                events: Mutex::new(Vec::new()),
            }
        }

        fn events(&self) -> Vec<OwnedLifecycleEvent> { self.events.lock().unwrap().clone() }
    }

    impl Handler<Unit, InstructionUpdate> for FailHandleRecordLifecycleIx {
        async fn handle(
            &self,
            _value: &Unit,
            _raw: &InstructionUpdate,
        ) -> HandlerResult<()> {
            Err("handle error".into())
        }

        async fn handle_lifecycle(
            &self,
            _txn: &TransactionUpdate,
            _instruction_shared: &InstructionShared,
            event: &LifecycleEvent<'_>,
        ) -> HandlerResult<()> {
            self.events.lock().unwrap().push(OwnedLifecycleEvent::from_ref(event));
            Ok(())
        }
    }

    /// Allow shared ownership so the handler can be both inside the pipeline
    /// (which requires `'static`) and inspected from the test.
    impl<T: Handler<Unit, InstructionUpdate> + Send + Sync> Handler<Unit, InstructionUpdate>
        for std::sync::Arc<T>
    {
        async fn handle(
            &self,
            value: &Unit,
            raw: &InstructionUpdate,
        ) -> HandlerResult<()> {
            T::handle(self, value, raw).await
        }

        async fn handle_lifecycle(
            &self,
            txn: &TransactionUpdate,
            instruction_shared: &InstructionShared,
            event: &LifecycleEvent<'_>,
        ) -> HandlerResult<()> {
            T::handle_lifecycle(self, txn, instruction_shared, event).await
        }
    }

    // -- tests ------------------------------------------------------------

    #[tokio::test]
    async fn instruction_pipeline_lifecycle_called_before_handle_error() {
        use std::sync::Arc;

        let handler = Arc::new(FailHandleRecordLifecycleIx::new());

        let inner_pipeline: super::BoxPipeline<'static, InstructionUpdate> =
            Box::new(Pipeline::new(IxParser, [Arc::clone(&handler)]));

        let pipeline = InstructionPipeline::new(vec![inner_pipeline])
            .expect("non-empty pipeline list");

        let txn = make_txn_with_one_instruction();
        let _result = pipeline.handle(&txn).await;

        // TxStart and TxEnd lifecycle calls bracket the handle invocation.
        // Even though handle() returns an error, both must have been delivered.
        assert_eq!(
            handler.events(),
            vec![OwnedLifecycleEvent::TxStart, OwnedLifecycleEvent::TxEnd],
        );
    }
}
