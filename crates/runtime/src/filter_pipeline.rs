//! `Pipeline` equivalent that allows for transaction custom filters

use std::{borrow::Cow, fmt::Debug, str::FromStr};

use futures_util::{Future, StreamExt};
use smallvec::SmallVec;
use vixen_core::{
    GetPrefilter, ParseError, Parser, ParserId, Prefilter, Pubkey, TransactionPrefilter,
};

use crate::{
    handler::{DynPipeline, PipelineErrors},
    Handler,
};

/// A wrapper around a parser that allows for custom prefilters to be applied to the transaction.
#[derive(Debug, Clone)]
pub struct FilterPipeline<P: Parser, H> {
    parser: P,
    include_accounts: Vec<Pubkey>,
    required_accounts: Vec<Pubkey>,
    handlers: H,
}

impl<P: Parser, H> GetPrefilter for FilterPipeline<P, H> {
    #[inline]
    fn prefilter(&self) -> Prefilter {
        let mut prefilter = self.parser.prefilter();
        let mut tx_prefilter = prefilter
            .transaction
            .expect("Instruction Parser must have a transaction prefilter");

        tx_prefilter.merge(TransactionPrefilter {
            accounts_include: self.include_accounts.iter().copied().collect(),
            accounts_required: self.required_accounts.iter().copied().collect(),
        });

        prefilter.transaction = Some(tx_prefilter);

        prefilter
    }
}

impl<P: Parser, H> ParserId for FilterPipeline<P, H> {
    #[inline]
    fn id(&self) -> Cow<str> { self.parser.id() }
}

impl<P: Parser, H> FilterPipeline<P, H> {
    /// Create a new `FilterPipeline` with the given parser and handlers.
    ///  This allows to set custom transaction filters.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    ///    vixen::Runtime::builder()
    ///        .source(YellowstoneGrpcSource::new())
    ///        .account(Pipeline::new(RaydiumAmmV4AccParser, [Logger]))
    ///        .instruction(FilterPipeline::new(RaydiumAmmV4IxParser, [Logger])
    ///            .include_accounts(["TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"])
    ///            .required_accounts(["675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"]),
    ///        )
    ///        .build(config)
    ///        .run();
    /// ```
    pub fn new(parser: P, handlers: H) -> Self {
        Self {
            parser,
            handlers,
            include_accounts: vec![],
            required_accounts: vec![],
        }
    }
}

impl<P, H> FilterPipeline<P, H>
where
    P: Parser + Send + Sync,
    P::Input: Send + Sync,
    H: Debug + Sync,
    for<'i> &'i H: IntoIterator,
    for<'i> <&'i H as IntoIterator>::Item: Handler<P::Output>,
{
    /// Set the included accounts for this transaction prefilter.
    ///
    /// **Note:** If the transaction does not include at least ONE of the accounts set here, the
    /// transaction will not be retrieved.
    #[must_use]
    #[allow(private_bounds)] // CustomPrefiltersAccount is meant to be used internally
    pub fn include_accounts<F: CustomPrefiltersAccount>(
        mut self,
        include_accounts: impl IntoIterator<Item = F>,
    ) -> Self {
        self.include_accounts
            .extend(include_accounts.into_iter().map(|f| f.get_pubkey()));

        self
    }

    /// Set the required accounts for this transaction prefilter.
    ///  The accounts set here **must** be present in the transaction.
    ///
    /// **Note:** If the transaction does not include ALL of the accounts set here, the
    /// transaction will not be retrieved.
    ///
    /// **The Program ID of the Parser program will always be included in this list
    #[must_use]
    #[allow(private_bounds)] // CustomPrefiltersAccount is meant to be used internally
    pub fn required_accounts<F: CustomPrefiltersAccount>(
        mut self,
        required_accounts: impl IntoIterator<Item = F>,
    ) -> Self {
        self.required_accounts
            .extend(required_accounts.into_iter().map(|f| f.get_pubkey()));

        self
    }

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
            .map(|h| async move { h.handle(parsed).await })
            .collect::<futures_util::stream::FuturesUnordered<_>>()
            .filter_map(|r| async move { r.err() })
            .collect::<SmallVec<[_; 1]>>()
            .await;

        if !errs.is_empty() {
            return Err(PipelineErrors::Handlers(errs));
        }

        Ok(())
    }
}

pub(crate) trait CustomPrefiltersAccount {
    fn get_pubkey(&self) -> Pubkey;
}

impl CustomPrefiltersAccount for Pubkey {
    fn get_pubkey(&self) -> Pubkey { *self }
}

impl CustomPrefiltersAccount for &'static str {
    fn get_pubkey(&self) -> Pubkey { Pubkey::from_str(self).unwrap() }
}

impl<P, H> DynPipeline<P::Input> for FilterPipeline<P, H>
where
    P: Parser + Debug + Send + Sync,
    P::Input: Send + Sync,
    P::Output: Send + Sync,
    H: Debug + Sync,
    for<'i> &'i H: IntoIterator,
    for<'i> <&'i H as IntoIterator>::Item: Handler<P::Output> + Send,
{
    fn handle<'h>(
        &'h self,
        value: &'h P::Input,
    ) -> std::pin::Pin<
        Box<dyn Future<Output = Result<(), crate::handler::PipelineErrors>> + Send + 'h>,
    > {
        Box::pin(FilterPipeline::handle_value(self, value))
    }
}
