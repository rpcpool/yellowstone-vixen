use yellowstone_vixen_core::{ParseResult, Parser, Prefilter, TransactionUpdate};

use crate::tx_parser::vixen_transaction::VixenTransaction;

pub struct TokenProgramParser;

impl Parser for TokenProgramParser {
    type Input = TransactionUpdate;
    type Output = VixenTransaction;

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .transaction_accounts([spl_token::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, tx: &TransactionUpdate) -> ParseResult<Self::Output> { todo!() }
}
