//! Pass-through subscription parsers for TransactionUpdate and AccountUpdate.
//!
//! TransactionSubscription: Forwards transaction updates as-is so the Vixen Runtime routes them
//! to BufferingHandler for eager instruction parsing.
//!
//! AccountSubscription: Forwards account updates as-is, subscribing to the union of all
//! account_owners from registered account parsers.

use std::{borrow::Cow, collections::HashSet};

use yellowstone_vixen_core::{
    AccountUpdate, ParseResult, Parser, Prefilter, Pubkey, TransactionPrefilter,
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
    owners: Vec<Pubkey>,
}

impl AccountSubscription {
    /// Construct from a KafkaSink.
    /// Returns `None` if no account parsers are registered.
    ///
    /// Collects program IDs from all registered account parsers to build the
    /// gRPC owner prefilter, so only accounts owned by known programs are streamed.
    pub fn new(sink: &KafkaSink) -> Option<Self> {
        if !sink.has_account_parsers() {
            return None;
        }
        let owners: Vec<Pubkey> = sink
            .account_parsers()
            .iter()
            .map(|p| p.program_id())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        Some(Self { owners })
    }

    /// Construct with explicit owner program IDs.
    pub fn with_owners(owners: Vec<Pubkey>) -> Self { Self { owners } }
}

impl Parser for AccountSubscription {
    type Input = AccountUpdate;
    type Output = AccountUpdate;

    fn id(&self) -> Cow<'static, str> { "kafka-sink::AccountSubscription".into() }

    fn prefilter(&self) -> Prefilter {
        let prefilter = Prefilter::builder()
            .account_owners(self.owners.iter().map(|o| o.as_slice()))
            .build()
            .unwrap();

        tracing::info!(
            parser_id = %self.id(),
            owner_count = self.owners.len(),
            "AccountSubscription prefilter created"
        );
        prefilter
    }

    async fn parse(&self, value: &Self::Input) -> ParseResult<Self::Output> { Ok(value.clone()) }
}
