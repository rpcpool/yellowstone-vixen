//! Pass-through subscription parsers for TransactionUpdate and AccountUpdate.
//!
//! TransactionSubscription: Forwards transaction updates as-is so the Vixen Runtime routes them
//! to BufferingHandler for eager instruction parsing.
//!
//! AccountSubscription: Forwards account updates as-is, subscribing to the union of all
//! account_owners from registered account parsers.

use std::borrow::Cow;

use yellowstone_vixen_core::{
    AccountPrefilter, AccountUpdate, ParseResult, Parser, Prefilter, TransactionPrefilter,
    TransactionUpdate,
};

use crate::sink::KafkaSink;

/// Pass-through subscription for transaction updates.
/// Subscribes to all transactions and forwards them as-is.
#[derive(Debug, Clone, Copy)]
pub struct TransactionSubscription;

impl Parser for TransactionSubscription {
    type Input = TransactionUpdate;
    type Output = TransactionUpdate;

    fn id(&self) -> Cow<'static, str> { "kafka-sink::TransactionSubscription".into() }

    fn prefilter(&self) -> Prefilter {
        let prefilter = Prefilter {
            transaction: Some(TransactionPrefilter {
                // Include all transactions (both successful and failed) so that
                // parsed_tx_count matches the block machine's expected_tx_count
                // which counts all executed transactions from entry data.
                failed: None,
                ..Default::default()
            }),
            ..Default::default()
        };
        tracing::info!(
            parser_id = %self.id(),
            has_transaction_filter = prefilter.transaction.is_some(),
            "TransactionSubscription prefilter created - receiving all transactions"
        );
        prefilter
    }

    async fn parse(&self, value: &Self::Input) -> ParseResult<Self::Output> { Ok(value.clone()) }
}

/// Pass-through parser for account updates.
/// Subscribes to all `account_owners` from registered account parsers (merged union).
/// Real filtering/parsing happens in the DynAccountParser dispatch.
#[derive(Debug, Clone)]
pub struct AccountSubscription {
    owners: Vec<[u8; 32]>,
}

impl AccountSubscription {
    /// Construct from a KafkaSink.
    /// Returns `None` if no account parsers are registered.
    ///
    /// Subscribes to all accounts (empty owner filter). The individual
    /// `DynAccountParser::try_parse()` handles the actual program-level filtering,
    /// since the type-erased trait doesn't expose `prefilter()`.
    pub fn new(sink: &KafkaSink) -> Option<Self> {
        if !sink.has_account_parsers() {
            return None;
        }
        Some(Self { owners: Vec::new() })
    }

    /// Construct with explicit owner program IDs.
    pub fn with_owners(owners: Vec<[u8; 32]>) -> Self { Self { owners } }
}

impl Parser for AccountSubscription {
    type Input = AccountUpdate;
    type Output = AccountUpdate;

    fn id(&self) -> Cow<'static, str> { "kafka-sink::AccountSubscription".into() }

    fn prefilter(&self) -> Prefilter {
        let prefilter = if self.owners.is_empty() {
            // Subscribe to all accounts when owners aren't known at construction time.
            Prefilter {
                account: Some(AccountPrefilter::default()),
                ..Default::default()
            }
        } else {
            Prefilter::builder()
                .account_owners(self.owners.iter().map(|o| o.as_slice()))
                .build()
                .unwrap()
        };

        tracing::info!(
            parser_id = %self.id(),
            owner_count = self.owners.len(),
            "AccountSubscription prefilter created"
        );
        prefilter
    }

    async fn parse(&self, value: &Self::Input) -> ParseResult<Self::Output> { Ok(value.clone()) }
}
