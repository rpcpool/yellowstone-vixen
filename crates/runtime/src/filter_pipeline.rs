//! `Pipeline` equivalent that allows for transaction custom filters

use std::{borrow::Cow, fmt::Debug};

use futures_util::{Future, StreamExt};
use smallvec::SmallVec;
use vixen_core::{
    instruction::Path, GetPrefilter, ParseError, Parser, ParserId, Prefilter, PrefilterBuilder,
    TransactionUpdate,
};

use crate::{
    handler::{DynPipeline, PipelineErrors},
    Handler,
};

/// A wrapper around a parser that allows for custom prefilters to be applied to the transaction.
#[derive(Debug, Clone)]
pub struct FilterPipeline<P: Parser, H> {
    parser: P,
    handlers: H,
    additional_filters: Prefilter,
}

impl<P: Parser, H> GetPrefilter for FilterPipeline<P, H> {
    #[inline]
    fn prefilter(&self) -> Prefilter {
        let mut prefilter = self.parser.prefilter();
        prefilter.merge(self.additional_filters.clone());

        prefilter
    }
}

impl<P: Parser, H> ParserId for FilterPipeline<P, H> {
    #[inline]
    fn id(&self) -> Cow<'static, str> { self.parser.id() }
}

impl<P: Parser, H> FilterPipeline<P, H> {
    /// Create a new `FilterPipeline` with the given parser and handlers.
    ///  This allows to set custom transaction filters.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    ///
    ///    vixen::Runtime::builder()
    ///        .source(YellowstoneGrpcSource::new())
    ///        .account(Pipeline::new(RaydiumAmmV4AccParser, [Logger]))
    ///        .instruction(FilterPipeline::new(RaydiumAmmV4IxParser, [RaydiumAmmV4IxLogger], Prefilter::builder()
    ///            .transaction_accounts_include([
    ///                KeyBytes::<32>::from_str("GH8Ers4yzKR3UKDvgVu8cqJfGzU4cU62mTeg9bcJ7ug6").unwrap(),
    ///                KeyBytes::<32>::from_str("4xxM4cdb6MEsCxM52xvYqkNbzvdeWWsPDZrBcTqVGUar").unwrap()
    ///            ])
    ///            .transaction_accounts([
    ///                KeyBytes::<32>::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap()
    ///            ]),
    ///        ))
    ///        .build(config)
    ///        .run();
    /// ```
    ///
    /// # Panics
    /// If the additional filters are invalid, this function will panic.
    pub fn new(parser: P, handlers: H, additional_filters: PrefilterBuilder) -> Self {
        Self {
            parser,
            handlers,
            additional_filters: additional_filters.build().unwrap(),
        }
    }
}

impl<P, H> FilterPipeline<P, H>
where
    P: Parser + Send + Sync,
    P::Input: Send + Sync,
    H: Debug + Sync,
    for<'i> &'i H: IntoIterator,
    for<'i> <&'i H as IntoIterator>::Item: Handler<P::Output, P::Input>,
{
    /// Handle fn for `FilterPipeline`
    ///
    /// # Errors
    /// If any of the related handlers executions errors, returns those errors
    pub async fn handle_value(&self, value: &P::Input) -> Result<(), PipelineErrors> {
        let parsed = match self.parser.parse(value).await {
            Ok(p) => p,
            Err(ParseError::Filtered) => return Ok(()),
            Err(ParseError::Other(e)) => return Err(PipelineErrors::Parse(e)),
        };
        let parsed = &parsed;

        let errs = self
            .handlers
            .into_iter()
            .map(|h| async move { h.handle(parsed, value).await })
            .collect::<futures_util::stream::FuturesUnordered<_>>()
            .filter_map(|r| async move { r.err() })
            .collect::<SmallVec<[_; 1]>>()
            .await;

        if !errs.is_empty() {
            return Err(PipelineErrors::Handlers(errs));
        }

        Ok(())
    }

    /// Notify all handlers that a transaction has been entered.
    ///
    /// # Errors
    /// If any handler returns an error, all errors are collected and returned.
    pub async fn handle_tx_start(&self, txn: &TransactionUpdate) -> Result<(), PipelineErrors> {
        let errs = self
            .handlers
            .into_iter()
            .map(|h| async move { h.handle_tx_start(txn).await })
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

    /// Notify all handlers that the transaction has ended.
    ///
    /// # Errors
    /// If any handler returns an error, all errors are collected and returned.
    pub async fn handle_tx_end(&self, txn: &TransactionUpdate) -> Result<(), PipelineErrors> {
        let errs = self
            .handlers
            .into_iter()
            .map(|h| async move { h.handle_tx_end(txn).await })
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

    /// Notify all handlers that execution is entering CPI calls from a caller
    /// node into its children.
    ///
    /// # Errors
    /// If any handler returns an error, all errors are collected and returned.
    pub async fn handle_cpi_enter(&self, caller_cpi_path: &Path) -> Result<(), PipelineErrors> {
        let errs = self
            .handlers
            .into_iter()
            .map(|h| async move { h.handle_cpi_enter(caller_cpi_path).await })
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

    /// Notify all handlers that execution has returned from CPI calls to a
    /// caller node.
    ///
    /// # Errors
    /// If any handler returns an error, all errors are collected and returned.
    pub async fn handle_cpi_return(&self, caller_cpi_path: &Path) -> Result<(), PipelineErrors> {
        let errs = self
            .handlers
            .into_iter()
            .map(|h| async move { h.handle_cpi_return(caller_cpi_path).await })
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

impl<P, H> DynPipeline<P::Input> for FilterPipeline<P, H>
where
    P: Parser + Debug + Send + Sync,
    P::Input: Send + Sync,
    P::Output: Send + Sync,
    H: Debug + Sync,
    for<'i> &'i H: IntoIterator,
    for<'i> <&'i H as IntoIterator>::Item: Handler<P::Output, P::Input> + Send,
{
    fn handle<'h>(
        &'h self,
        value: &'h P::Input,
    ) -> std::pin::Pin<
        Box<dyn Future<Output = Result<(), crate::handler::PipelineErrors>> + Send + 'h>,
    > {
        Box::pin(FilterPipeline::handle_value(self, value))
    }

    fn handle_tx_start<'h>(
        &'h self,
        txn: &'h TransactionUpdate,
    ) -> std::pin::Pin<Box<dyn Future<Output = ()> + Send + 'h>> {
        Box::pin(async move {
            if let Err(e) = FilterPipeline::handle_tx_start(self, txn).await {
                e.handle::<P::Input>(&self.id()).as_unit();
            }
        })
    }

    fn handle_tx_end<'h>(
        &'h self,
        txn: &'h TransactionUpdate,
    ) -> std::pin::Pin<Box<dyn Future<Output = ()> + Send + 'h>> {
        Box::pin(async move {
            if let Err(e) = FilterPipeline::handle_tx_end(self, txn).await {
                e.handle::<P::Input>(&self.id()).as_unit();
            }
        })
    }

    fn handle_cpi_enter<'h>(
        &'h self,
        caller_cpi_path: &'h Path,
    ) -> std::pin::Pin<Box<dyn Future<Output = ()> + Send + 'h>> {
        Box::pin(async move {
            if let Err(e) = FilterPipeline::handle_cpi_enter(self, caller_cpi_path).await {
                e.handle::<P::Input>(&self.id()).as_unit();
            }
        })
    }

    fn handle_cpi_return<'h>(
        &'h self,
        caller_cpi_path: &'h Path,
    ) -> std::pin::Pin<Box<dyn Future<Output = ()> + Send + 'h>> {
        Box::pin(async move {
            if let Err(e) = FilterPipeline::handle_cpi_return(self, caller_cpi_path).await {
                e.handle::<P::Input>(&self.id()).as_unit();
            }
        })
    }
}
