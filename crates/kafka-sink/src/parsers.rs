//! Pass-through parser for TransactionUpdate.
//!
//! Forwards transaction updates as-is so the Vixen Runtime routes them
//! to BufferingHandler for eager instruction parsing.

use std::borrow::Cow;

use yellowstone_vixen_core::{
    ParseResult, Parser, Prefilter, TransactionPrefilter, TransactionUpdate,
};

/// Pass-through parser for transaction updates.
/// Subscribes to all transactions and forwards them as-is.
#[derive(Debug, Clone, Copy)]
pub struct TransactionParser;

impl Parser for TransactionParser {
    type Input = TransactionUpdate;
    type Output = TransactionUpdate;

    fn id(&self) -> Cow<'static, str> { "kafka-sink::TransactionParser".into() }

    fn prefilter(&self) -> Prefilter {
        let prefilter = Prefilter {
            transaction: Some(TransactionPrefilter::default()),
            ..Default::default()
        };
        tracing::info!(
            parser_id = %self.id(),
            has_transaction_filter = prefilter.transaction.is_some(),
            "TransactionParser prefilter created - receiving all transactions"
        );
        prefilter
    }

    async fn parse(&self, value: &Self::Input) -> ParseResult<Self::Output> { Ok(value.clone()) }
}
