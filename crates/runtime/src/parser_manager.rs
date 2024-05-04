use std::{
    collections::HashMap,
    future::Future,
};

use futures_util::FutureExt;
use tracing::warn;

use crate::{
    parser::{AccountUpdate, TransactionUpdate},
    Parser,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct ParserManager<P>(HashMap<String, P>);

impl<I, P> FromIterator<I> for ParserManager<P>
where HashMap<String, P>: FromIterator<I>
{
    #[inline]
    fn from_iter<T: IntoIterator<Item = I>>(it: T) -> Self { Self(HashMap::from_iter(it)) }
}

impl<P> From<HashMap<String, P>> for ParserManager<P> {
    #[inline]
    fn from(val: HashMap<String, P>) -> Self { Self(val) }
}

impl<P> ParserManager<P> {
    #[inline]
    pub fn new(val: HashMap<String, P>) -> Self { Self(val) }
}

impl<P: Parser> ParserManager<P> {
    pub fn filters(&self) {
        todo!()
    }
}

impl<P> ParserManager<P> {
    pub fn get_parsers<I: IntoIterator<Item = S>, S: AsRef<str>>(&self, it: I) -> Parsers<P, I> {
        Parsers(self, it)
    }
}

pub struct Parsers<'a, P, I>(&'a ParserManager<P>, I);

impl<'a, P: Parser, I: IntoIterator> Parsers<'a, P, I>
where I::Item: AsRef<str>
{
    fn get_parsers(self) -> impl Iterator<Item = &'a P> {
        let Self(manager, it) = self;
        it.into_iter().filter_map(|filter| {
            let filter = filter.as_ref();
            let parser = manager.0.get(filter);

            if parser.is_none() {
                warn!(filter, "No parser matched filter on incoming update");
            }

            parser
        })
    }

    pub fn run_account(self, acct: &'a AccountUpdate) -> impl Future<Output = ()> + Send + 'a {
        futures_util::future::join_all(self.get_parsers().filter_map(|p| {
            if !p.filter_account(acct) {
                return None;
            }

            Some(p.process_account(acct))
        }))
        .map(|v| v.into_iter().collect())
    }

    pub fn run_transaction(self, txn: &'a TransactionUpdate) -> impl Future<Output = ()> + Send + 'a {
        futures_util::future::join_all(self.get_parsers().filter_map(|p| {
            if !p.filter_transaction(txn) {
                return None;
            }

            Some(p.process_transaction(txn))
        }))
        .map(|v| v.into_iter().collect())
    }
}
