use std::{collections::HashMap, future::Future, ops};

use futures_util::FutureExt;
use tracing::{error, warn};

use crate::{
    parser::{AccountUpdate, Prefilter, TransactionUpdate},
    Parser,
};

pub struct Filters<'a>(HashMap<&'a str, Prefilter>);

impl<'a> ops::Deref for Filters<'a> {
    type Target = HashMap<&'a str, Prefilter>;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}

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
    pub fn filters(&self) -> Filters {
        Filters(
            self.0
                .iter()
                .map(|(k, v)| (k.as_str(), v.prefilter()))
                .collect(),
        )
    }
}

impl<P> ParserManager<P> {
    pub fn get_parsers<I: IntoIterator<Item = S>, S: AsRef<str>>(&self, it: I) -> Parsers<P, I> {
        Parsers(self, it)
    }
}

pub struct Parsers<'a, P, I>(&'a ParserManager<P>, I);

impl<'a, P: Parser, I: IntoIterator> Parsers<'a, P, I>
where I::Item: AsRef<str> + Send + 'a
{
    fn get_parsers(self) -> impl Iterator<Item = (I::Item, &'a P)> {
        let Self(manager, it) = self;
        it.into_iter().filter_map(|f| {
            let filter = f.as_ref();
            let parser = manager.0.get(filter);

            if parser.is_none() {
                warn!(filter, "No parser matched filter on incoming update");
            }

            parser.map(|p| (f, p))
        })
    }

    pub fn run_account(self, acct: &'a AccountUpdate) -> impl Future<Output = ()> + Send + 'a {
        futures_util::future::join_all(self.get_parsers().filter_map(|(f, p)| {
            if !p.filter_account(acct) {
                return None;
            }

            Some(p.process_account(acct).map(move |r| match r {
                Ok(()) => (),
                Err(err) => error!(
                    %err,
                    parser = f.as_ref(),
                    method = "run_account",
                    "Parser handler failed",
                ),
            }))
        }))
        .map(|v| v.into_iter().collect())
    }

    pub fn run_transaction(
        self,
        txn: &'a TransactionUpdate,
    ) -> impl Future<Output = ()> + Send + 'a {
        futures_util::future::join_all(self.get_parsers().filter_map(|(f,p)| {
            if !p.filter_transaction(txn) {
                return None;
            }

            Some(p.process_transaction(txn).map(move |r| match r {
                Ok(()) => (),
                Err(err) => error!(
                    %err,
                    parser = f.as_ref(),
                    method = "run_transaction",
                    "Parser handler failed",
                ),
            }))
        }))
        .map(|v| v.into_iter().collect())
    }
}
