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
    str::FromStr,
    sync::Arc,
};

use borsh::{BorshDeserialize, BorshSerialize};
use serde::Deserialize;
use yellowstone_grpc_proto::geyser::{
    self, SubscribeRequest, SubscribeRequestFilterAccounts, SubscribeRequestFilterBlocks,
    SubscribeRequestFilterBlocksMeta, SubscribeRequestFilterSlots,
    SubscribeRequestFilterTransactions, SubscribeUpdateAccount, SubscribeUpdateAccountInfo,
    SubscribeUpdateBlock, SubscribeUpdateBlockMeta, SubscribeUpdateSlot,
    SubscribeUpdateTransaction,
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
/// An account update from Yellowstone.
pub type AccountUpdateInfo = SubscribeUpdateAccountInfo;
/// A transaction update from Yellowstone.
pub type TransactionUpdate = SubscribeUpdateTransaction;
/// A block meta update from Yellowstone.
pub type BlockMetaUpdate = SubscribeUpdateBlockMeta;
/// A block update from Yellowstone.
pub type BlockUpdate = SubscribeUpdateBlock;
/// A slot update from Yellowstone.
pub type SlotUpdate = SubscribeUpdateSlot;

/// Generic output type for instruction parsers that wraps shared data for all instructions
/// in the given transaction.
///
/// This is the recommended structure for an `Parser::Output` associated type, for the case that the parser
/// wants to expose the `InstructionShared` data to the `Handler`s
#[derive(Debug)]
pub struct InstructionUpdateOutput<T> {
    /// The parsed instruction.
    pub parsed_ix: T,
    /// Shared data for all instructions in the given transaction.
    pub shared_data: Arc<instruction::InstructionShared>,
}

/// A core trait that defines the parse logic for producing a parsed value from
/// a Vixen update (typically [`AccountUpdate`], [`TransactionUpdate`], or
/// [`InstructionUpdate`](instruction::InstructionUpdate)).
pub trait Parser {
    /// The input update type for this parser.
    type Input;

    /// The type of the parsed value produced by this parser.
    /// When the `proto` feature is enabled, this type must implement `prost::Message`
    /// for protobuf serialization compatibility.
    #[cfg(feature = "proto")]
    type Output: ::prost::Message;

    /// The type of the parsed value produced by this parser.
    #[cfg(not(feature = "proto"))]
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
    fn id(&self) -> Cow<'static, str>;

    /// Filter data passed to Yellowstone to coarsely narrow down updates
    /// to values parseable by this parser.
    fn prefilter(&self) -> Prefilter;

    /// Parse the given update into a parsed value.
    fn parse(&self, value: &Self::Input) -> impl Future<Output = ParseResult<Self::Output>> + Send;
}

/// A parser that parses all relevant updates for a particular program ID.
pub trait ProgramParser: Parser {
    /// The program ID that this parser is associated with.
    fn program_id(&self) -> KeyBytes<32>;
}

/// Helper trait for getting the ID of a parser.
pub trait ParserId {
    /// Get the ID of this parser, see [`Parser::id`].
    fn id(&self) -> Cow<'static, str>;
}

impl ParserId for std::convert::Infallible {
    #[inline]
    fn id(&self) -> Cow<'static, str> { match *self {} }
}

impl<T: Parser> ParserId for T {
    #[inline]
    fn id(&self) -> Cow<'static, str> { Parser::id(self) }
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
#[derive(Debug, Default, Clone)]
pub struct Prefilter {
    /// Filters for account updates.
    pub account: Option<AccountPrefilter>,
    /// Filters for transaction updates.
    pub transaction: Option<TransactionPrefilter>,
    /// Filters for block meta updates.
    pub block_meta: Option<BlockMetaPrefilter>,
    /// Filters for block updates.
    pub block: Option<BlockPrefilter>,
    /// Filters for slot updates.
    pub slot: Option<SlotPrefilter>,
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
            block_meta,
            block,
            slot,
        } = self;
        merge_opt(account, other.account, AccountPrefilter::merge);
        merge_opt(transaction, other.transaction, TransactionPrefilter::merge);
        merge_opt(block_meta, other.block_meta, BlockMetaPrefilter::merge);
        merge_opt(block, other.block, BlockPrefilter::merge);
        merge_opt(slot, other.slot, SlotPrefilter::merge);
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

/// A prefilter for matching accounts.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AccountPrefilter {
    /// The accounts that this prefilter will match.
    pub accounts: HashSet<KeyBytes<32>>,
    /// The owners that this prefilter will match.
    pub owners: HashSet<KeyBytes<32>>,
}

impl AccountPrefilter {
    /// Merge another account prefilter into this one, producing a prefilter
    /// that describes the union of the two.
    pub fn merge(&mut self, other: AccountPrefilter) {
        let Self { accounts, owners } = self;
        accounts.extend(other.accounts);
        owners.extend(other.owners);
    }
}

/// A prefilter for matching transactions.
#[derive(Debug, Clone, PartialEq)]
pub struct TransactionPrefilter {
    /// The transaction **must** include at least **ONE** of these accounts. Otherwise, the transaction
    ///  won't be retrieved.
    pub accounts_include: HashSet<KeyBytes<32>>,
    /// These accounts **must** be present in the transaction.
    ///  That means if any of the accounts are not included in the transaction, the transaction
    ///  won't be retrieved.
    pub accounts_required: HashSet<KeyBytes<32>>,
    /// Filter by transaction success/failure status.
    /// - `None`: Include all transactions (required for "any" filter in Richat)
    /// - `Some(false)`: Only successful transactions (default)
    /// - `Some(true)`: Only failed transactions
    pub failed: Option<bool>,
}

impl Default for TransactionPrefilter {
    fn default() -> Self {
        Self {
            accounts_include: HashSet::new(),
            accounts_required: HashSet::new(),
            failed: Some(false), // Default to successful transactions (keep original behaviour)
        }
    }
}

impl TransactionPrefilter {
    /// Merge another transaction prefilter into this one, producing a prefilter
    /// that describes the union of the two (consensus or all).
    pub fn merge(&mut self, other: TransactionPrefilter) {
        let Self {
            accounts_include,
            accounts_required,
            failed,
        } = self;

        accounts_include.extend(other.accounts_include);
        accounts_required.extend(other.accounts_required);

        if other.failed.is_none() {
            *failed = None;
        }
    }
}

/// A prefilter for matching block metadata updates.
#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct BlockMetaPrefilter {}

impl BlockMetaPrefilter {
    /// Merge another block metadata prefilter into this one.
    /// This function currently does nothing as the struct has no fields.
    pub fn merge(_lhs: &mut Self, _rhs: Self) {}
}

/// A prefilter for matching block updates.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockPrefilter {
    /// filter transactions and accounts that use any account from the list
    pub accounts_include: HashSet<KeyBytes<32>>,
    /// include all transactions
    pub include_transactions: bool,
    /// include all account updates
    pub include_accounts: bool,
    /// include all entries
    pub include_entries: bool,
}

impl BlockPrefilter {
    /// Merge another block prefilter into this one.
    pub fn merge(&mut self, other: BlockPrefilter) {
        let Self {
            accounts_include,
            include_transactions,
            include_accounts,
            include_entries,
        } = self;

        accounts_include.extend(other.accounts_include);
        *include_accounts |= other.include_accounts;
        *include_transactions |= other.include_transactions;
        *include_entries |= other.include_entries;
    }
}

/// A prefilter for matching slot updates.
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SlotPrefilter {
    /// If true (default), only receive slot updates at the connection's commitment level.
    /// If false, receive ALL slot status transitions (processed, confirmed, finalized, dead).
    pub filter_by_commitment: bool,
}

impl Default for SlotPrefilter {
    fn default() -> Self {
        Self {
            filter_by_commitment: true,
        }
    }
}

impl SlotPrefilter {
    /// Merge another slot prefilter into this one, producing a union of both filters, the more permissive wins.
    /// `filter_by_commitment` controls which slot status updates are received:
    /// - `true`: Only receive updates at the connection's commitment level
    /// - `false`: Receive ALL slot status transitions (processed, confirmed, finalized, dead)
    pub fn merge(lhs: &mut Self, rhs: Self) {
        lhs.filter_by_commitment = lhs.filter_by_commitment && rhs.filter_by_commitment;
    }
}

/// Helper macro for converting Vixen's [`KeyBytes<32>`] to a Solana ed25519
/// public key.
///
/// Invoking the macro with the name of a publicly-exported Solana `Pubkey`
/// type (e.g. `pubkey_convert_helpers!(solana_sdk::pubkey::Pubkey);`) will
/// define two functions:
///
/// - `pub(crate) fn into_vixen_pubkey(`<Solana Pubkey>`) -> yellowstone_vixen_core::KeyBytes<32>;`
/// - `pub(crate) fn from_vixen_pubkey(yellowstone_vixen_core::KeyBytes<32>) -> <Solana Pubkey>;`
///
/// These can be used as a convenience for quickly converting between Solana
/// public keys and their representation in Vixen.  Vixen does not use the
/// built-in Solana `Pubkey` type, nor does it provide `From`/`Into` impls for
/// it, to avoid creating an unnecessary dependency on any specific version of
/// the full Solana SDK.
#[macro_export]
macro_rules! pubkey_convert_helpers {
    ($ty:ty) => {
        pub(crate) fn into_vixen_pubkey(value: $ty) -> $crate::KeyBytes<32> {
            value.to_bytes().into()
        }

        pub(crate) fn from_vixen_pubkey(value: $crate::KeyBytes<32>) -> $ty {
            value.into_bytes().into()
        }
    };
}

/// Protobuf wrapper for a 32-byte public key.
///
/// This struct wraps raw public key bytes for protobuf serialization.  When the
/// `proto` feature is enabled it derives [`prost::Message`] so it can be used
/// as a nested message field (`message PublicKey { bytes value = 1; }`).
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "proto", derive(::prost::Message))]
#[cfg_attr(not(feature = "proto"), derive(Debug, Default))]
pub struct PublicKey {
    /// The raw bytes of the public key.
    #[cfg_attr(feature = "proto", prost(bytes = "vec", tag = "1"))]
    pub value: Vec<u8>,
}

impl PublicKey {
    /// Creates a new `PublicKey` from any type convertible to `Vec<u8>`.
    pub fn new(value: impl Into<Vec<u8>>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

/// Generic wrapper for a fixed-length array of cryptographic key bytes,
/// convertible to or from a base58-encoded string.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct KeyBytes<const LEN: usize>(pub [u8; LEN]);

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

impl<const LEN: usize> BorshSerialize for KeyBytes<LEN> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.0.serialize(writer)
    }
}

impl<const LEN: usize> BorshDeserialize for KeyBytes<LEN> {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let bytes = <[u8; LEN]>::deserialize_reader(reader)?;
        Ok(Self(bytes))
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
    /// An error occurred while parsing a public key as a [`KeyBytes<32>`].
    #[error("Invalid pubkey {}", bs58::encode(.0).into_string())]
    BadPubkey(Vec<u8>, std::array::TryFromSliceError),
}

/// A builder for constructing a prefilter.
#[derive(Debug, Default)]
#[must_use = "Consider calling .build() on this builder"]
#[allow(clippy::struct_excessive_bools)]
pub struct PrefilterBuilder {
    error: Option<PrefilterError>,
    slots: bool,
    block_metas: bool,
    /// Matching [`BlockPrefilter::accounts`]
    block_accounts_include: Option<HashSet<KeyBytes<32>>>,
    /// Matching [`BlockPrefilter::include_accounts`]
    block_include_accounts: bool,
    /// Matching [`BlockPrefilter::include_transactions`]
    block_include_transactions: bool,
    /// Matching [`BlockPrefilter::include_entries`]
    block_include_entries: bool,
    /// Including all accounts
    accounts_include_all: bool,
    /// Matching [`AccountPrefilter::accounts`]
    accounts: Option<HashSet<KeyBytes<32>>>,
    /// Matching [`AccountPrefilter::account_owners`]
    account_owners: Option<HashSet<KeyBytes<32>>>,
    /// Matching [`TransactionPrefilter::accounts_include`]
    transaction_accounts_include: Option<HashSet<KeyBytes<32>>>,
    /// Matching [`TransactionPrefilter::accounts_required`]
    transaction_accounts_required: Option<HashSet<KeyBytes<32>>>,
}

fn set_opt<T>(opt: &mut Option<T>, field: &'static str, val: T) -> Result<(), PrefilterError> {
    if opt.is_some() {
        return Err(PrefilterError::AlreadySet(field));
    }

    *opt = Some(val);
    Ok(())
}

// TODO: if Solana ever adds Into<[u8; 32]> for Pubkey this can be simplified
fn collect_pubkeys<I: IntoIterator>(it: I) -> Result<HashSet<KeyBytes<32>>, PrefilterError>
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
            accounts_include_all,
            accounts,
            account_owners,
            slots,
            block_metas,
            block_accounts_include,
            block_include_accounts,
            block_include_entries,
            block_include_transactions,
            transaction_accounts_include,
            transaction_accounts_required,
        } = self;
        if let Some(err) = error {
            return Err(err);
        }

        let account = AccountPrefilter {
            accounts: accounts.unwrap_or_default(),
            owners: account_owners.unwrap_or_default(),
        };

        let transaction = TransactionPrefilter {
            accounts_include: transaction_accounts_include.unwrap_or_default(),
            accounts_required: transaction_accounts_required.unwrap_or_default(),
            ..Default::default()
        };

        let block_meta = BlockMetaPrefilter {};

        let block = BlockPrefilter {
            accounts_include: block_accounts_include.unwrap_or_default(),
            include_accounts: block_include_accounts,
            include_transactions: block_include_transactions,
            include_entries: block_include_entries,
        };

        let slot = SlotPrefilter::default();

        let account = if accounts_include_all {
            Some(AccountPrefilter::default())
        } else {
            (account != AccountPrefilter::default()).then_some(account)
        };

        Ok(Prefilter {
            account,
            transaction: (transaction != TransactionPrefilter::default()).then_some(transaction),
            block_meta: block_metas.then_some(block_meta),
            block: (block != BlockPrefilter::default()).then_some(block),
            slot: slots.then_some(slot),
        })
    }

    fn mutate<F: FnOnce(&mut Self) -> Result<(), PrefilterError>>(mut self, f: F) -> Self {
        if self.error.is_none() {
            self.error = f(&mut self).err();
        }

        self
    }

    /// Set prefilter will request slot updates.
    pub fn slots(self) -> Self {
        self.mutate(|this| {
            this.slots = true;
            Ok(())
        })
    }

    /// Set prefilter will request `block_metas` updates.
    pub fn block_metas(self) -> Self {
        self.mutate(|this| {
            this.block_metas = true;
            Ok(())
        })
    }

    /// Set `accounts_include_all` filter
    pub fn accounts_include_all(self) -> Self {
        self.mutate(|this| {
            this.accounts_include_all = true;
            Ok(())
        })
    }

    /// Set the accounts that this prefilter will match.
    pub fn accounts<I: IntoIterator>(self, it: I) -> Self
    where I::Item: AsRef<[u8]> {
        self.mutate(|this| set_opt(&mut this.accounts, "accounts", collect_pubkeys(it)?))
    }

    /// Set the `account_owners` that this prefilter will match.
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

    /// Set the required accounts for this transaction prefilter.
    ///  The accounts set here **must** be present in the transaction.
    ///
    /// **Note:** If the transaction does not include ALL of the accounts set here, the
    /// transaction will not be retrieved.
    pub fn transaction_accounts<I: IntoIterator>(self, it: I) -> Self
    where I::Item: AsRef<[u8]> {
        self.mutate(|this| {
            set_opt(
                &mut this.transaction_accounts_required,
                "transaction_accounts_required",
                collect_pubkeys(it)?,
            )
        })
    }

    /// Set the included accounts for this transaction prefilter.
    ///
    /// **Note:** If the transaction does not include at least ONE of the accounts set here, the
    /// transaction will not be retrieved.
    pub fn transaction_accounts_include<I: IntoIterator>(self, it: I) -> Self
    where I::Item: AsRef<[u8]> {
        self.mutate(|this| {
            set_opt(
                &mut this.transaction_accounts_include,
                "transaction_accounts_include",
                collect_pubkeys(it)?,
            )
        })
    }

    /// Set the included accounts for this block prefilter.
    pub fn block_accounts_include<I: IntoIterator>(self, it: I) -> Self
    where I::Item: AsRef<[u8]> {
        self.mutate(|this| {
            set_opt(
                &mut this.block_accounts_include,
                "block_accounts_include",
                collect_pubkeys(it)?,
            )
        })
    }

    /// Set the `include_accounts` flag for this block prefilter.
    pub fn block_include_accounts(self) -> Self {
        self.mutate(|this| {
            this.block_include_accounts = true;
            Ok(())
        })
    }

    /// Set the `include_transactions` flag for this block prefilter.
    pub fn block_include_transactions(self) -> Self {
        self.mutate(|this| {
            this.block_include_transactions = true;
            Ok(())
        })
    }

    /// Set the `include_entries` flag for this block prefilter.
    pub fn block_include_entries(self) -> Self {
        self.mutate(|this| {
            this.block_include_entries = true;
            Ok(())
        })
    }
}

/// A collection of filters for a Vixen subscription.
#[derive(Debug, Clone)]
pub struct Filters {
    /// Filters for each parser.
    pub parsers_filters: HashMap<String, Prefilter>,
}

impl Filters {
    /// Construct a new collection of filters.
    #[inline]
    #[must_use]
    pub fn new(filters: HashMap<String, Prefilter>) -> Self {
        Self {
            parsers_filters: filters,
        }
    }
}

/// Type mirroring the `CommitmentLevel` enum in the `geyser` crate but serializable.
/// Used to avoid need for custom deserialization logic.
#[derive(Debug, Clone, Copy, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum CommitmentLevel {
    /// Processed
    Processed,
    /// Confirmed
    Confirmed,
    /// Finalized
    Finalized,
}

impl From<geyser::CommitmentLevel> for CommitmentLevel {
    fn from(value: geyser::CommitmentLevel) -> Self {
        match value {
            geyser::CommitmentLevel::Processed => Self::Processed,
            geyser::CommitmentLevel::Confirmed => Self::Confirmed,
            geyser::CommitmentLevel::Finalized => Self::Finalized,
        }
    }
}

impl From<Filters> for SubscribeRequest {
    fn from(value: Filters) -> Self {
        SubscribeRequest {
            accounts: value
                .parsers_filters
                .iter()
                .filter_map(|(k, v)| {
                    let v = v.account.as_ref()?;

                    Some((k.clone(), SubscribeRequestFilterAccounts {
                        account: v.accounts.iter().map(ToString::to_string).collect(),
                        owner: v.owners.iter().map(ToString::to_string).collect(),
                        // TODO: probably a good thing to look into
                        filters: vec![],
                        // We receive all accounts updates
                        nonempty_txn_signature: None,
                    }))
                })
                .collect(),
            slots: value
                .parsers_filters
                .iter()
                .filter_map(|(k, v)| {
                    let slot_filter = v.slot.as_ref()?;
                    Some((k.clone(), SubscribeRequestFilterSlots {
                        filter_by_commitment: Some(slot_filter.filter_by_commitment),
                        interslot_updates: None,
                    }))
                })
                .collect(),
            transactions: value
                .parsers_filters
                .iter()
                .filter_map(|(k, v)| {
                    let v = v.transaction.as_ref()?;

                    Some((k.clone(), SubscribeRequestFilterTransactions {
                        vote: None,
                        failed: v.failed,
                        signature: None,
                        account_include: v
                            .accounts_include
                            .iter()
                            .map(ToString::to_string)
                            .collect(),
                        account_exclude: [].into_iter().collect(),
                        account_required: v
                            .accounts_required
                            .iter()
                            .map(ToString::to_string)
                            .collect(),
                    }))
                })
                .collect(),
            transactions_status: [].into_iter().collect(),
            blocks: value
                .parsers_filters
                .iter()
                .filter_map(|(k, v)| {
                    let v = v.block.as_ref()?;

                    Some((k.clone(), SubscribeRequestFilterBlocks {
                        account_include: v
                            .accounts_include
                            .iter()
                            .map(ToString::to_string)
                            .collect(),
                        include_transactions: Some(v.include_transactions),
                        include_accounts: Some(v.include_accounts),
                        include_entries: Some(v.include_entries),
                    }))
                })
                .collect(),
            blocks_meta: value
                .parsers_filters
                .iter()
                .filter_map(|(k, v)| {
                    v.block_meta?;
                    Some((k.clone(), SubscribeRequestFilterBlocksMeta {}))
                })
                .collect(),
            entry: [].into_iter().collect(),
            commitment: None,
            accounts_data_slice: vec![],
            ping: None,
            from_slot: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn block_prefilter(
        include_accounts: bool,
        include_transactions: bool,
        include_entries: bool,
    ) -> BlockPrefilter {
        BlockPrefilter {
            accounts_include: HashSet::new(),
            include_accounts,
            include_transactions,
            include_entries,
        }
    }

    #[test]
    fn test_block_prefilter_merge_basic_union() {
        let mut a = block_prefilter(true, false, false);
        let b = block_prefilter(false, true, false);

        a.merge(b);

        assert!(
            a.include_accounts,
            "BUG: include_accounts was true, should remain true after merge with false"
        );
        assert!(
            a.include_transactions,
            "include_transactions should be true after merge"
        );
        assert!(
            !a.include_entries,
            "include_entries should remain false (neither requested)"
        );
    }

    #[test]
    fn test_block_prefilter_merge_idempotence() {
        let original = block_prefilter(true, false, true);
        let mut a = original.clone();
        let b = original.clone();

        a.merge(b);

        assert_eq!(a, original, "merge(A, A) should equal A (idempotence)");
    }

    #[test]
    fn test_block_prefilter_merge_commutativity() {
        let a_orig = block_prefilter(true, false, true);
        let b_orig = block_prefilter(false, true, false);

        let mut a = a_orig.clone();
        a.merge(b_orig.clone());

        let mut b = b_orig.clone();
        b.merge(a_orig.clone());

        assert_eq!(a, b, "merge(A, B) should equal merge(B, A) (commutativity)");
    }

    #[test]
    fn test_block_prefilter_merge_associativity() {
        let a_orig = block_prefilter(true, false, false);
        let b_orig = block_prefilter(false, true, false);
        let c_orig = block_prefilter(false, false, true);

        let mut ab = a_orig.clone();
        ab.merge(b_orig.clone());
        let mut abc_left = ab;
        abc_left.merge(c_orig.clone());

        let mut bc = b_orig.clone();
        bc.merge(c_orig.clone());
        let mut abc_right = a_orig.clone();
        abc_right.merge(bc);

        assert_eq!(
            abc_left, abc_right,
            "merge(merge(A, B), C) should equal merge(A, merge(B, C)) (associativity)"
        );
    }

    #[test]
    fn test_block_prefilter_merge_identity() {
        let original = block_prefilter(true, true, false);
        let mut a = original.clone();
        let default = BlockPrefilter::default();

        a.merge(default);

        assert_eq!(
            a, original,
            "merge(A, default) should equal A (identity element)"
        );
    }

    #[test]
    fn test_block_prefilter_merge_monotonicity() {
        let mut a = block_prefilter(true, true, true);
        let b = block_prefilter(false, false, false);

        a.merge(b);

        assert!(
            a.include_accounts,
            "BUG: include_accounts was true, must remain true after merge"
        );
        assert!(
            a.include_transactions,
            "BUG: include_transactions was true, must remain true after merge"
        );
        assert!(
            a.include_entries,
            "BUG: include_entries was true, must remain true after merge"
        );
    }

    #[test]
    fn test_block_prefilter_merge_truth_table() {
        for lhs in [false, true] {
            for rhs in [false, true] {
                let expected = lhs || rhs;

                {
                    let mut a = block_prefilter(lhs, false, false);
                    let b = block_prefilter(rhs, false, false);
                    a.merge(b);
                    assert_eq!(
                        a.include_accounts, expected,
                        "include_accounts: {lhs} OR {rhs} should be {expected}"
                    );
                }

                {
                    let mut a = block_prefilter(false, lhs, false);
                    let b = block_prefilter(false, rhs, false);
                    a.merge(b);
                    assert_eq!(
                        a.include_transactions, expected,
                        "include_transactions: {lhs} OR {rhs} should be {expected}"
                    );
                }

                {
                    let mut a = block_prefilter(false, false, lhs);
                    let b = block_prefilter(false, false, rhs);
                    a.merge(b);
                    assert_eq!(
                        a.include_entries, expected,
                        "include_entries: {lhs} OR {rhs} should be {expected}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_block_prefilter_merge_hashset_union() {
        let key1: KeyBytes<32> = [1u8; 32].into();
        let key2: KeyBytes<32> = [2u8; 32].into();

        let mut a = BlockPrefilter {
            accounts_include: [key1].into_iter().collect(),
            include_accounts: false,
            include_transactions: false,
            include_entries: false,
        };

        let b = BlockPrefilter {
            accounts_include: [key2].into_iter().collect(),
            include_accounts: false,
            include_transactions: false,
            include_entries: false,
        };

        a.merge(b);

        assert!(
            a.accounts_include.contains(&key1),
            "key1 should be in merged set"
        );
        assert!(
            a.accounts_include.contains(&key2),
            "key2 should be in merged set"
        );
        assert_eq!(a.accounts_include.len(), 2, "merged set should have 2 keys");
    }

    #[test]
    fn test_prefilter_merge_block_or_semantics() {
        let mut p1 = Prefilter {
            block: Some(block_prefilter(true, false, false)),
            ..Default::default()
        };

        let p2 = Prefilter {
            block: Some(block_prefilter(false, true, false)),
            ..Default::default()
        };

        p1.merge(p2);

        let block = p1.block.expect("block prefilter should exist after merge");
        assert!(
            block.include_accounts,
            "Prefilter merge: include_accounts should be true"
        );
        assert!(
            block.include_transactions,
            "Prefilter merge: include_transactions should be true"
        );
    }

    #[test]
    fn test_prefilter_from_iterator_block_or_semantics() {
        let p1 = Prefilter {
            block: Some(block_prefilter(true, false, false)),
            ..Default::default()
        };
        let p2 = Prefilter {
            block: Some(block_prefilter(false, true, false)),
            ..Default::default()
        };
        let p3 = Prefilter {
            block: Some(block_prefilter(false, false, true)),
            ..Default::default()
        };

        let combined: Prefilter = [p1, p2, p3].into_iter().collect();

        let block = combined
            .block
            .expect("block prefilter should exist after collect");
        assert!(
            block.include_accounts,
            "FromIterator: include_accounts should be true"
        );
        assert!(
            block.include_transactions,
            "FromIterator: include_transactions should be true"
        );
        assert!(
            block.include_entries,
            "FromIterator: include_entries should be true"
        );
    }
}
