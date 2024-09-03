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
// TODO: document everything
#![allow(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc)]

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fmt::{self, Debug},
    future::Future,
    ops,
    str::FromStr,
};

use yellowstone_grpc_proto::geyser::{
    SubscribeRequest, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
    SubscribeUpdateAccount, SubscribeUpdateTransaction,
};

pub extern crate bs58;

pub mod instruction;

type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug)]
pub enum ParseError {
    Filtered,
    Other(BoxedError),
}

impl<T: Into<BoxedError>> From<T> for ParseError {
    #[inline]
    fn from(value: T) -> Self {
        Self::Other(value.into())
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

pub type AccountUpdate = SubscribeUpdateAccount;
pub type TransactionUpdate = SubscribeUpdateTransaction;

pub trait Parser {
    type Input;
    type Output;

    fn id(&self) -> Cow<str>;

    fn prefilter(&self) -> Prefilter;

    fn parse(&self, value: &Self::Input) -> impl Future<Output = ParseResult<Self::Output>> + Send;
}

pub trait ProgramParser: Parser {
    fn program_id(&self) -> Pubkey;
}

pub trait ParserId {
    fn id(&self) -> Cow<str>;
}

impl ParserId for std::convert::Infallible {
    #[inline]
    fn id(&self) -> Cow<str> {
        match *self {}
    }
}

impl<T: Parser> ParserId for T {
    #[inline]
    fn id(&self) -> Cow<str> {
        Parser::id(self)
    }
}

pub trait GetPrefilter {
    fn prefilter(&self) -> Prefilter;
}

impl GetPrefilter for std::convert::Infallible {
    #[inline]
    fn prefilter(&self) -> Prefilter {
        match *self {}
    }
}

impl<T: Parser> GetPrefilter for T {
    #[inline]
    fn prefilter(&self) -> Prefilter {
        Parser::prefilter(self)
    }
}

// TODO: why are so many fields on the prefilters and prefilter builder optional???
#[derive(Debug, Default)]
pub struct Prefilter {
    pub(crate) account: Option<AccountPrefilter>,
    pub(crate) transaction: Option<TransactionPrefilter>,
}

fn merge_opt<T, F: FnOnce(&mut T, T)>(lhs: &mut Option<T>, rhs: Option<T>, f: F) {
    match (lhs.as_mut(), rhs) {
        (None, r) => *lhs = r,
        (Some(_), None) => (),
        (Some(l), Some(r)) => f(l, r),
    }
}

impl Prefilter {
    #[inline]
    pub fn builder() -> PrefilterBuilder {
        PrefilterBuilder::default()
    }

    pub fn merge(&mut self, other: Prefilter) {
        let Self {
            account,
            transaction,
        } = self;
        merge_opt(account, other.account, AccountPrefilter::merge);
        merge_opt(transaction, other.transaction, TransactionPrefilter::merge);
    }
}

impl FromIterator<Prefilter> for Prefilter {
    fn from_iter<T: IntoIterator<Item = Prefilter>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let Some(ret) = iter.next() else {
            return Self::default();
        };
        iter.fold(ret, |mut l, r| {
            l.merge(r);
            l
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct AccountPrefilter {
    pub accounts: HashSet<Pubkey>,
    pub owners: HashSet<Pubkey>,
}

impl AccountPrefilter {
    pub fn merge(&mut self, other: AccountPrefilter) {
        let Self { accounts, owners } = self;
        accounts.extend(other.accounts);
        owners.extend(other.owners);
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct TransactionPrefilter {
    pub accounts: HashSet<Pubkey>,
}

impl TransactionPrefilter {
    pub fn merge(&mut self, other: TransactionPrefilter) {
        let Self { accounts } = self;
        accounts.extend(other.accounts);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pubkey(pub [u8; 32]);

impl Debug for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&bs58::encode(self.0).into_string())
    }
}

impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&bs58::encode(self.0).into_string())
    }
}

impl From<[u8; 32]> for Pubkey {
    #[inline]
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl TryFrom<&[u8]> for Pubkey {
    type Error = std::array::TryFromSliceError;

    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum PubkeyFromStrError {
    #[error("Invalid base58 string")]
    Bs58(#[from] bs58::decode::Error),
    #[error("Invalid key length, must be 32 bytes")]
    Len(#[from] std::array::TryFromSliceError),
}

impl FromStr for Pubkey {
    type Err = PubkeyFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        bs58::decode(s)
            .into_vec()?
            .as_slice()
            .try_into()
            .map_err(Into::into)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum PrefilterError {
    #[error("Value already given for field {0}")]
    AlreadySet(&'static str),
    #[error("Invalid pubkey {}", bs58::encode(.0).into_string())]
    BadPubkey(Vec<u8>, std::array::TryFromSliceError),
}

#[derive(Debug, Default)]
#[must_use = "Consider calling .build() on this builder"]
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
where
    I::Item: AsRef<[u8]>,
{
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

    pub fn account_owners<I: IntoIterator>(self, it: I) -> Self
    where
        I::Item: AsRef<[u8]>,
    {
        self.mutate(|this| {
            set_opt(
                &mut this.account_owners,
                "account_owners",
                collect_pubkeys(it)?,
            )
        })
    }

    pub fn transaction_accounts<I: IntoIterator>(self, it: I) -> Self
    where
        I::Item: AsRef<[u8]>,
    {
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
    pub const fn new(filters: HashMap<&'a str, Prefilter>) -> Self {
        Self(filters)
    }
}

impl<'a> ops::Deref for Filters<'a> {
    type Target = HashMap<&'a str, Prefilter>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<Filters<'a>> for SubscribeRequest {
    fn from(value: Filters<'a>) -> Self {
        SubscribeRequest {
            accounts: value
                .iter()
                .filter_map(|(k, v)| {
                    let v = v.account.as_ref()?;

                    Some((
                        k.to_owned().into(),
                        SubscribeRequestFilterAccounts {
                            account: v.accounts.iter().map(ToString::to_string).collect(),
                            owner: v.owners.iter().map(ToString::to_string).collect(),
                            // TODO: probably a good thing to look into
                            filters: vec![],
                        },
                    ))
                })
                .collect(),
            slots: [].into_iter().collect(),
            transactions: value
                .iter()
                .filter_map(|(k, v)| {
                    let v = v.transaction.as_ref()?;

                    Some((
                        k.to_owned().into(),
                        SubscribeRequestFilterTransactions {
                            vote: None,
                            // TODO: make this configurable
                            failed: Some(false),
                            signature: None,
                            // TODO: figure these out
                            account_include: v.accounts.iter().map(ToString::to_string).collect(),
                            account_exclude: [].into_iter().collect(),
                            account_required: [].into_iter().collect(),
                        },
                    ))
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
