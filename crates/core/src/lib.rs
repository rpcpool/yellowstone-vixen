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

//! This crate provides the core components necessary for implementing parsers
//! for the `yellowstone-vixen` family of crates.  This crate should be used
//! as a dependency instead of `yellowstone-vixen` for crates that intend to
//! define and export Vixen parsers as libraries without needing to access the
//! runtime functionality of Vixen.

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
#[cfg(feature = "proto")]
pub extern crate yellowstone_vixen_proto;

pub mod instruction;
#[cfg(feature = "proto")]
pub mod proto;

type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// An error returned by a Vixen parser
#[derive(Debug)]
pub enum ParseError {
    /// The parser received an undesired update and requested to skip
    /// processing for it.  No error will be logged by the Vixen runtime, and
    /// no handlers registered to this parser will be executed.
    Filtered,
    /// The parser encountered an error while processing an update.
    Other(BoxedError),
}

impl<T: Into<BoxedError>> From<T> for ParseError {
    #[inline]
    fn from(value: T) -> Self { Self::Other(value.into()) }
}

/// The result of parsing an update.
pub type ParseResult<T> = Result<T, ParseError>;

/// An account update from Yellowstone.
pub type AccountUpdate = SubscribeUpdateAccount;
/// A transaction update from Yellowstone.
pub type TransactionUpdate = SubscribeUpdateTransaction;

/// A core trait that defines the parse logic for producing a parsed value from
/// a Vixen update (typically [`AccountUpdate`], [`TransactionUpdate`], or
/// [`InstructionUpdate`](instruction::InstructionUpdate)).
pub trait Parser {
    /// The input update type for this parser.
    type Input;
    /// The type of the parsed value produced by this parser.
    type Output;

    /// A unique ID for this parser.  Used to associate the parser with its
    /// requested prefilter data.
    ///
    /// **NOTE:** For parsers that do not accept configuration when constructed
    /// (e.g. a parser that accepts all updates of a certain type from a
    /// specific program), the ID may be as simple as the fully-qualified type
    /// name of the parser.  However, for parsers that produce a different
    /// prefilter depending on some internal configuration, instances that
    /// output differing prefilters _must_ output different IDs.
    fn id(&self) -> Cow<str>;

    /// Filter data passed to Yellowstone to coarsely narrow down updates
    /// to values parseable by this parser.
    fn prefilter(&self) -> Prefilter;

    /// Parse the given update into a parsed value.
    fn parse(&self, value: &Self::Input) -> impl Future<Output = ParseResult<Self::Output>> + Send;
}

/// A parser that parses all relevant updates for a particular program ID.
pub trait ProgramParser: Parser {
    /// The program ID that this parser is associated with.
    fn program_id(&self) -> Pubkey;
}

/// Helper trait for getting the ID of a parser.
pub trait ParserId {
    /// Get the ID of this parser, see [`Parser::id`].
    fn id(&self) -> Cow<str>;
}

impl ParserId for std::convert::Infallible {
    #[inline]
    fn id(&self) -> Cow<str> { match *self {} }
}

impl<T: Parser> ParserId for T {
    #[inline]
    fn id(&self) -> Cow<str> { Parser::id(self) }
}

/// Helper trait for getting the prefilter of a parser.
pub trait GetPrefilter {
    /// Get the prefilter of this parser, see [`Parser::prefilter`].
    fn prefilter(&self) -> Prefilter;
}

impl GetPrefilter for std::convert::Infallible {
    #[inline]
    fn prefilter(&self) -> Prefilter { match *self {} }
}

impl<T: Parser> GetPrefilter for T {
    #[inline]
    fn prefilter(&self) -> Prefilter { Parser::prefilter(self) }
}

// TODO: why are so many fields on the prefilters and prefilter builder optional???
/// A prefilter for narrowing down the updates that a parser will receive.
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
    /// Create a new prefilter builder.
    #[inline]
    pub fn builder() -> PrefilterBuilder { PrefilterBuilder::default() }

    /// Merge another prefilter into this one, producing a prefilter that
    /// describes the union of the two.
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

/// Helper macro for converting Vixen's [`Pubkey`] to a Solana ed25519 public
/// key.
///
/// Invoking the macro with the name of a publicly-exported Solana `Pubkey`
/// type (e.g. `pubkey_convert_helpers!(solana_sdk::pubkey::Pubkey);`) will
/// define two functions:
///
/// - `pub(crate) fn into_vixen_pubkey(`<Solana Pubkey>`) -> yellowstone_vixen_core::Pubkey;`
/// - `pub(crate) fn from_vixen_pubkey(yellowstone_vixen_core::Pubkey) -> <Solana Pubkey>;`
///
/// These can be used as a convenience for quickly converting between Solana
/// public keys and their representation in Vixen.  Vixen does not use the
/// built-in Solana `Pubkey` type, nor does it provide `From`/`Into` impls for
/// it, to avoid creating an unnecessary dependency on any specific version of
/// the full Solana SDK.
#[macro_export]
macro_rules! pubkey_convert_helpers {
    ($ty:ty) => {
        pub(crate) fn into_vixen_pubkey(value: $ty) -> $crate::Pubkey { value.to_bytes().into() }

        pub(crate) fn from_vixen_pubkey(value: $crate::Pubkey) -> $ty { value.into_bytes().into() }
    };
}

/// Helper type representing a Solana public key.
///
/// This type is functionally equivalent to the `Pubkey` type from the Solana
/// SDK, and it can be trivially converted to or from one by passing the
/// underlying `[u8; 32]` array.  Vixen uses this `Pubkey` type to avoid
/// depending on the Solana SDK, as this can lead to version conflicts when
/// working with Solana program crates.
pub type Pubkey = KeyBytes<32>;

/// Generic wrapper for a fixed-length array of cryptographic key bytes,
/// convertible to or from a base58-encoded string.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct KeyBytes<const LEN: usize>([u8; LEN]);

impl<const LEN: usize> Debug for KeyBytes<LEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("KeyBytes")
            .field(&bs58::encode(self.0).into_string())
            .finish()
    }
}

impl<const LEN: usize> fmt::Display for KeyBytes<LEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&bs58::encode(self.0).into_string())
    }
}

impl<const LEN: usize> From<[u8; LEN]> for KeyBytes<LEN> {
    #[inline]
    fn from(value: [u8; LEN]) -> Self { Self(value) }
}

impl<const LEN: usize> From<KeyBytes<LEN>> for [u8; LEN] {
    #[inline]
    fn from(value: KeyBytes<LEN>) -> Self { value.0 }
}

impl<const LEN: usize> std::ops::Deref for KeyBytes<LEN> {
    type Target = [u8; LEN];

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<const LEN: usize> std::ops::DerefMut for KeyBytes<LEN> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<const LEN: usize> AsRef<[u8; LEN]> for KeyBytes<LEN> {
    fn as_ref(&self) -> &[u8; LEN] { self }
}

impl<const LEN: usize> AsMut<[u8; LEN]> for KeyBytes<LEN> {
    fn as_mut(&mut self) -> &mut [u8; LEN] { self }
}

impl<const LEN: usize> std::borrow::Borrow<[u8; LEN]> for KeyBytes<LEN> {
    fn borrow(&self) -> &[u8; LEN] { self }
}

impl<const LEN: usize> std::borrow::BorrowMut<[u8; LEN]> for KeyBytes<LEN> {
    fn borrow_mut(&mut self) -> &mut [u8; LEN] { self }
}

impl<const LEN: usize> AsRef<[u8]> for KeyBytes<LEN> {
    fn as_ref(&self) -> &[u8] { self.as_slice() }
}

impl<const LEN: usize> AsMut<[u8]> for KeyBytes<LEN> {
    fn as_mut(&mut self) -> &mut [u8] { self.as_mut_slice() }
}

impl<const LEN: usize> std::borrow::Borrow<[u8]> for KeyBytes<LEN> {
    fn borrow(&self) -> &[u8] { self.as_ref() }
}

impl<const LEN: usize> std::borrow::BorrowMut<[u8]> for KeyBytes<LEN> {
    fn borrow_mut(&mut self) -> &mut [u8] { self.as_mut() }
}

type KeyFromSliceError = std::array::TryFromSliceError;

impl<const LEN: usize> TryFrom<&[u8]> for KeyBytes<LEN> {
    type Error = KeyFromSliceError;

    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> { value.try_into().map(Self) }
}

impl<const LEN: usize> KeyBytes<LEN> {
    /// Construct a new instance from the provided key bytes
    #[must_use]
    pub fn new(bytes: [u8; LEN]) -> Self { bytes.into() }

    /// Return the public key bytes contained in this instance
    #[must_use]
    pub fn into_bytes(self) -> [u8; LEN] { self.into() }

    /// Attempt to convert the provided byte slice to a new key byte array
    ///
    /// # Errors
    /// This function returns an error if calling `KeyBytes::try_from(slice)`
    /// returns an error.
    pub fn try_from_ref<T: AsRef<[u8]>>(key: T) -> Result<Self, KeyFromSliceError> {
        key.as_ref().try_into()
    }

    /// Compare the public key bytes contained in this array with the given byte
    /// slice
    pub fn equals_ref<T: AsRef<[u8]>>(&self, other: T) -> bool {
        self.as_slice().eq(other.as_ref())
    }
}

/// An error that can occur when parsing a key from a base58 string.
#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum KeyFromStrError<const LEN: usize = 32> {
    /// The string was not a valid base58 string.
    #[error("Invalid base58 string")]
    Bs58(#[from] bs58::decode::Error),
    /// The parsed base58 data was not the correct length for a public key.
    #[error("Invalid key length, must be {LEN} bytes")]
    Len(#[from] std::array::TryFromSliceError),
}

impl<const LEN: usize> FromStr for KeyBytes<LEN> {
    type Err = KeyFromStrError<LEN>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        bs58::decode(s)
            .into_vec()?
            .as_slice()
            .try_into()
            .map_err(Into::into)
    }
}

impl<const LEN: usize> TryFrom<&str> for KeyBytes<LEN> {
    type Error = KeyFromStrError<LEN>;

    fn try_from(value: &str) -> Result<Self, Self::Error> { value.parse() }
}

impl<const LEN: usize> TryFrom<String> for KeyBytes<LEN> {
    type Error = KeyFromStrError<LEN>;

    fn try_from(value: String) -> Result<Self, Self::Error> { value.parse() }
}

impl<const LEN: usize> TryFrom<Cow<'_, str>> for KeyBytes<LEN> {
    type Error = KeyFromStrError<LEN>;

    fn try_from(value: Cow<str>) -> Result<Self, Self::Error> { value.parse() }
}

/// An error that can occur when building a prefilter.
#[derive(Debug, Clone, thiserror::Error)]
pub enum PrefilterError {
    /// A value was already set for a field that can only be set once.
    #[error("Value already given for field {0}")]
    AlreadySet(&'static str),
    /// An error occurred while parsing a public key as a [`Pubkey`].
    #[error("Invalid pubkey {}", bs58::encode(.0).into_string())]
    BadPubkey(Vec<u8>, std::array::TryFromSliceError),
}

/// A builder for constructing a prefilter.
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
    /// Build the prefilter from the given data.
    ///
    /// # Errors
    /// Returns an error if any of the fields provided are invalid.
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

    /// Set the accounts that this prefilter will match.
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

    /// Set the accounts mentioned by transactions that this prefilter will
    /// match.
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

/// A collection of filters for a Vixen subscription.
#[derive(Debug)]
#[repr(transparent)]
pub struct Filters<'a>(HashMap<&'a str, Prefilter>);

impl<'a> Filters<'a> {
    /// Construct a new collection of filters.
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
