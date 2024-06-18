// TODO
#![allow(dead_code, unused)]

use std::{
    collections::{HashMap, HashSet},
    future::Future,
    ops,
};

use solana_sdk::pubkey::Pubkey;
use yellowstone_grpc_proto::geyser::{
    SubscribeRequest, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
    SubscribeUpdateAccount, SubscribeUpdateTransaction,
};

type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug)]
pub enum ParseError {
    Filtered,
    Other(BoxedError),
}

impl<T: Into<BoxedError>> From<T> for ParseError {
    #[inline]
    fn from(value: T) -> Self { Self::Other(value.into()) }
}

pub type ParseResult<T> = Result<T, ParseError>;

pub type AccountUpdate = SubscribeUpdateAccount;
pub type TransactionUpdate = SubscribeUpdateTransaction;

pub trait Parser {
    type Input;
    type Output;

    fn prefilter(&self) -> Prefilter;

    fn parse(&self, value: &Self::Input) -> impl Future<Output = ParseResult<Self::Output>> + Send;
}

pub struct Prefilter {
    pub(crate) account: Option<AccountPrefilter>,
    pub(crate) transaction: Option<TransactionPrefilter>,
}

#[derive(Default, PartialEq)]
pub(crate) struct AccountPrefilter {
    pub accounts: HashSet<Pubkey>,
    pub owners: HashSet<Pubkey>,
}

#[derive(Default, PartialEq)]
pub(crate) struct TransactionPrefilter {
    pub accounts: HashSet<Pubkey>,
}

impl Prefilter {
    #[inline]
    pub fn builder() -> PrefilterBuilder { PrefilterBuilder::default() }
}

#[derive(Debug, thiserror::Error)]
pub enum PrefilterError {
    #[error("Value already given for field {0}")]
    AlreadySet(&'static str),
}

#[derive(Debug, Default)]
pub struct PrefilterBuilder {
    error: Option<PrefilterError>,
    accounts: Option<HashSet<Pubkey>>,
    account_owners: Option<HashSet<Pubkey>>,
    transaction_accounts: Option<HashSet<Pubkey>>,
}

fn set_opt<T>(opt: &mut Option<T>, field: &'static str, val: T) -> Result<(), PrefilterError> {
    if opt.is_some() {
        return Err(PrefilterError::AlreadySet(field));
    }

    *opt = Some(val);
    Ok(())
}

impl PrefilterBuilder {
    pub fn build(self) -> Result<Prefilter, PrefilterError> {
        let PrefilterBuilder {
            error,
            accounts,
            account_owners,
            transaction_accounts,
        } = self;
        if let Some(err) = error {
            return Err(err);
        }

        let account = AccountPrefilter {
            accounts: accounts.unwrap_or_default(),
            owners: account_owners.unwrap_or_default(),
        };

        let transaction = TransactionPrefilter {
            accounts: transaction_accounts.unwrap_or_default(),
        };

        Ok(Prefilter {
            account: (account != AccountPrefilter::default()).then_some(account),
            transaction: (transaction != TransactionPrefilter::default()).then_some(transaction),
        })
    }

    fn mutate<F: FnOnce(&mut Self) -> Result<(), PrefilterError>>(mut self, f: F) -> Self {
        if self.error.is_none() {
            self.error = f(&mut self).err();
        }

        self
    }

    pub fn account_owners<I: IntoIterator>(self, it: I) -> Self
    where HashSet<Pubkey>: FromIterator<I::Item> {
        self.mutate(|this| {
            set_opt(
                &mut this.account_owners,
                "account_owners",
                FromIterator::from_iter(it),
            )
        })
    }

    pub fn transaction_accounts<I: IntoIterator>(self, it: I) -> Self
    where HashSet<Pubkey>: FromIterator<I::Item> {
        self.mutate(|this| {
            set_opt(
                &mut this.transaction_accounts,
                "transaction_accounts",
                FromIterator::from_iter(it),
            )
        })
    }
}

#[repr(transparent)]
pub struct Filters<'a>(HashMap<&'a str, Prefilter>);

impl<'a> Filters<'a> {
    #[inline]
    pub const fn new(filters: HashMap<&'a str, Prefilter>) -> Self { Self(filters) }
}

impl<'a> ops::Deref for Filters<'a> {
    type Target = HashMap<&'a str, Prefilter>;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'a> From<Filters<'a>> for SubscribeRequest {
    fn from(value: Filters<'a>) -> Self {
        SubscribeRequest {
            accounts: value
                .iter()
                .filter_map(|(k, v)| {
                    let v = v.account.as_ref()?;

                    Some((k.to_owned().into(), SubscribeRequestFilterAccounts {
                        account: v.accounts.iter().map(ToString::to_string).collect(),
                        owner: v.owners.iter().map(ToString::to_string).collect(),
                        // TODO: probably a good thing to look into
                        filters: vec![],
                    }))
                })
                .collect(),
            slots: [].into_iter().collect(),
            transactions: value
                .iter()
                .filter_map(|(k, v)| {
                    let v = v.transaction.as_ref()?;

                    Some((k.to_owned().into(), SubscribeRequestFilterTransactions {
                        vote: None,
                        // TODO: make this configurable
                        failed: Some(false),
                        signature: None,
                        // TODO: figure these out
                        account_include: v.accounts.iter().map(ToString::to_string).collect(),
                        account_exclude: [].into_iter().collect(),
                        account_required: [].into_iter().collect(),
                    }))
                })
                .collect(),
            transactions_status: [].into_iter().collect(),
            blocks: [].into_iter().collect(),
            blocks_meta: [].into_iter().collect(),
            entry: [].into_iter().collect(),
            commitment: None,
            accounts_data_slice: vec![],
            ping: None,
        }
    }
}
