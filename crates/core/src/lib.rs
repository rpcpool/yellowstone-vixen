#![deny(
    clippy::disallowed_methods,
    clippy::suspicious,
    clippy::style,
    clippy::clone_on_ref_ptr,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::module_name_repetitions)]

use std::{
    collections::{HashMap, HashSet},
    fmt,
    future::Future,
    ops,
};

use yellowstone_grpc_proto::{
    geyser::{
        subscribe_update::UpdateOneof, SubscribeRequest, SubscribeRequestFilterAccounts,
        SubscribeRequestFilterTransactions, SubscribeUpdateAccount, SubscribeUpdateTransaction,
    },
    solana::storage::confirmed_block::{CompiledInstruction, InnerInstructions},
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

pub trait Update {
    const TYPE: UpdateType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdateType {
    Account,
    Transaction,
}

impl UpdateType {
    #[must_use]
    pub fn get(update: &Option<UpdateOneof>) -> Option<Self> {
        match update {
            Some(UpdateOneof::Account(_)) => Some(Self::Account),
            Some(UpdateOneof::Transaction(_)) => Some(Self::Transaction),
            _ => None,
        }
    }
}

impl Update for AccountUpdate {
    const TYPE: UpdateType = UpdateType::Account;
}

impl Update for TransactionUpdate {
    const TYPE: UpdateType = UpdateType::Transaction;
}

pub trait Parser {
    type Input: Update;
    type Output;

    fn prefilter(&self) -> Prefilter;

    fn parse(&self, value: &Self::Input) -> impl Future<Output = ParseResult<Self::Output>> + Send;
}

#[derive(Debug)]
pub struct Prefilter {
    pub(crate) account: Option<AccountPrefilter>,
    pub(crate) transaction: Option<TransactionPrefilter>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Pubkey(pub [u8; 32]);

impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&bs58::encode(self.0).into_string())
    }
}

impl From<[u8; 32]> for Pubkey {
    #[inline]
    fn from(value: [u8; 32]) -> Self { Self(value) }
}

impl TryFrom<&[u8]> for Pubkey {
    type Error = std::array::TryFromSliceError;

    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> { value.try_into().map(Self) }
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct AccountPrefilter {
    pub accounts: HashSet<Pubkey>,
    pub owners: HashSet<Pubkey>,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct TransactionPrefilter {
    pub accounts: HashSet<Pubkey>,
}

impl Prefilter {
    #[inline]
    #[must_use]
    pub fn builder() -> PrefilterBuilder { PrefilterBuilder::default() }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum PrefilterError {
    #[error("Value already given for field {0}")]
    AlreadySet(&'static str),
    #[error("Invalid pubkey {}", bs58::encode(.0).into_string())]
    BadPubkey(Vec<u8>, std::array::TryFromSliceError),
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

// TODO: if Solana ever adds Into<[u8; 32]> for Pubkey this can be simplified
fn collect_pubkeys<I: IntoIterator>(it: I) -> Result<HashSet<Pubkey>, PrefilterError>
where I::Item: AsRef<[u8]> {
    it.into_iter()
        .map(|p| {
            let p = p.as_ref();
            p.try_into()
                .map_err(|e| PrefilterError::BadPubkey(p.to_vec(), e))
        })
        .collect()
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

    #[must_use]
    pub fn account_owners<I: IntoIterator>(self, it: I) -> Self
    where I::Item: AsRef<[u8]> {
        self.mutate(|this| {
            set_opt(
                &mut this.account_owners,
                "account_owners",
                collect_pubkeys(it)?,
            )
        })
    }

    #[must_use]
    pub fn transaction_accounts<I: IntoIterator>(self, it: I) -> Self
    where I::Item: AsRef<[u8]> {
        self.mutate(|this| {
            set_opt(
                &mut this.transaction_accounts,
                "transaction_accounts",
                collect_pubkeys(it)?,
            )
        })
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Filters<'a>(HashMap<&'a str, Prefilter>);

impl<'a> Filters<'a> {
    #[inline]
    #[must_use]
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
