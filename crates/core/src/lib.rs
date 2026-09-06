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
//! for the `shipstern` family of crates.  This crate should be used
//! as a dependency instead of `shipstern` for crates that intend to
//! define and export Shipstern parsers as libraries without needing to access the
//! runtime functionality of Shipstern.

use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::{self, Debug},
    future::Future,
    str::FromStr,
    sync::Arc,
};

use borsh::{BorshDeserialize, BorshSerialize};
use serde::Deserialize;
use yellowstone_grpc_proto::geyser::{
    self, subscribe_request_filter_accounts_filter as wire_filter,
    subscribe_request_filter_accounts_filter_lamports as wire_lamports,
    subscribe_request_filter_accounts_filter_memcmp as wire_memcmp, SubscribeRequest,
    SubscribeRequestAccountsDataSlice, SubscribeRequestFilterAccounts,
    SubscribeRequestFilterAccountsFilter, SubscribeRequestFilterAccountsFilterLamports,
    SubscribeRequestFilterAccountsFilterMemcmp, SubscribeRequestFilterBlocks,
    SubscribeRequestFilterBlocksMeta, SubscribeRequestFilterSlots,
    SubscribeRequestFilterTransactions, SubscribeUpdateAccount, SubscribeUpdateAccountInfo,
    SubscribeUpdateBlock, SubscribeUpdateBlockMeta, SubscribeUpdateSlot,
    SubscribeUpdateTransaction,
};

pub extern crate bs58;

#[cfg(feature = "proto")]
pub extern crate shipstern_proto;

pub mod instruction;
pub mod log_messages;

#[cfg(feature = "proto")]
pub mod proto;

type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// An error returned by a Shipstern parser
#[derive(Debug)]
pub enum ParseError {
    /// The parser received an undesired update and requested to skip
    /// processing for it.  No error will be logged by the Shipstern runtime, and
    /// no handlers registered to this parser will be executed.
    Filtered,
    /// No instruction discriminator matched the input data.
    DiscriminatorNotFound(String),
    /// The parser encountered an error while processing an update.
    Other(BoxedError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Filtered => f.write_str("filtered"),
            Self::DiscriminatorNotFound(msg) => write!(f, "DiscriminatorNotFound({msg})"),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
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
/// a Shipstern update (typically [`AccountUpdate`], [`TransactionUpdate`], or
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
    fn program_id(&self) -> Pubkey;
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
#[derive(Debug, Default, Clone, PartialEq)]
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

/// How the bytes of a [`AccountFilter::Memcmp`] comparison are written.
///
/// The wire accepts three encodings of the same bytes. `Bytes` is the one to
/// reach for; the string forms exist because config documents cannot carry raw
/// bytes.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MemcmpData {
    /// Raw bytes.
    Bytes(Vec<u8>),
    /// Base58, the encoding pubkeys and signatures already use.
    Base58(String),
    /// Base64, for data that is not key-shaped.
    Base64(String),
}

/// A lamport-balance comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LamportsCmp {
    /// Balance equals this.
    Eq(u64),
    /// Balance does not equal this.
    Ne(u64),
    /// Balance is below this.
    Lt(u64),
    /// Balance is above this.
    Gt(u64),
}

/// A server-side comparison on an account's contents, narrowing a subscription
/// beyond the account and owner keys.
///
/// Several filters on one parser are **`ANDed`** by the server, so adding one
/// only ever narrows what that parser receives.
///
/// ```rust, ignore
/// Prefilter::builder()
///     .account_owners([spl_token::ID])
///     .account_filters([AccountFilter::DataSize(165)])
/// ```
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccountFilter {
    /// Compare bytes at an offset.
    Memcmp {
        /// Byte offset into the account data.
        offset: u64,
        /// The bytes to compare against.
        data: MemcmpData,
    },
    /// Match accounts whose data is exactly this many bytes.
    DataSize(u64),
    /// Match only initialized SPL token accounts.
    TokenAccountState(bool),
    /// Compare the lamport balance.
    Lamports(LamportsCmp),
}

/// A window of account data to receive instead of the whole account.
///
/// Applies to the subscription as a whole rather than to one parser, so it
/// lives on the source configuration rather than on a [`Prefilter`].
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct AccountsDataSlice {
    /// Byte offset to start at.
    pub offset: u64,
    /// Number of bytes to take.
    pub length: u64,
}

/// A prefilter for matching accounts.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AccountPrefilter {
    /// The accounts that this prefilter will match.
    pub accounts: HashSet<Pubkey>,
    /// The owners that this prefilter will match.
    pub owners: HashSet<Pubkey>,
    /// Server-side comparisons on the account contents. Several are `ANDed`, so
    /// each one narrows further. Empty means no comparison.
    pub filters: Vec<AccountFilter>,
    /// Whether to require a non-empty transaction signature on the update.
    /// `None` receives every account update, which is the default.
    pub nonempty_txn_signature: Option<bool>,
}

impl AccountPrefilter {
    /// Merge another account prefilter into this one, producing a prefilter
    /// that describes the union of the two.
    pub fn merge(&mut self, other: AccountPrefilter) {
        let Self {
            accounts,
            owners,
            filters,
            nonempty_txn_signature,
        } = self;
        accounts.extend(other.accounts);
        owners.extend(other.owners);

        // The server ANDs the filter list, so keeping both sides' entries would
        // deliver the intersection. Two prefilters wanting different data
        // comparisons cannot be expressed as one list, so the union is the
        // absence of a comparison: over-deliver and let the parser reject.
        if *filters != other.filters {
            filters.clear();
        }

        // `None` receives everything, so it absorbs any narrower choice.
        if *nonempty_txn_signature != other.nonempty_txn_signature {
            *nonempty_txn_signature = None;
        }
    }
}

/// A prefilter for matching transactions.
#[derive(Debug, Clone, PartialEq)]
pub struct TransactionPrefilter {
    /// The transaction **must** include at least **ONE** of these accounts. Otherwise, the transaction
    ///  won't be retrieved.
    pub accounts_include: HashSet<Pubkey>,
    /// These accounts **must** be present in the transaction.
    ///  That means if any of the accounts are not included in the transaction, the transaction
    ///  won't be retrieved.
    pub accounts_required: HashSet<Pubkey>,
    /// Transactions touching any of these are **not** retrieved.
    pub accounts_exclude: HashSet<Pubkey>,
    /// Whether to receive vote transactions. `None` receives all, which is the
    /// default.
    pub vote: Option<bool>,
    /// Receive only the transaction with this signature. `None` receives all.
    pub signature: Option<String>,
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
            accounts_exclude: HashSet::new(),
            accounts_required: HashSet::new(),
            vote: None,          // Receive vote and non-vote alike, as before
            signature: None,     // No single-signature narrowing, as before
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
            accounts_exclude,
            accounts_required,
            vote,
            signature,
            failed,
        } = self;

        accounts_include.extend(other.accounts_include);
        accounts_required.extend(other.accounts_required);

        // Exclusion is a negation, so the union of what two prefilters receive
        // is the intersection of what they exclude: a key only one side hides
        // must still reach the other.
        accounts_exclude.retain(|key| other.accounts_exclude.contains(key));

        // Both of these narrow, so any disagreement widens to "receive all".
        if *vote != other.vote {
            *vote = None;
        }

        if *signature != other.signature {
            *signature = None;
        }

        // The union of two success/failure filters accepts everything the two
        //  accept together. Only when both sides agree on a concrete value does
        //  that value survive; any disagreement (or an existing "any" filter)
        //  widens to `None`, i.e. accept all transactions.
        *failed = match (*failed, other.failed) {
            (Some(a), Some(b)) if a == b => Some(a),
            _ => None,
        };
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
    pub accounts_include: HashSet<Pubkey>,
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
    /// Whether to receive the extra updates the server emits between slots.
    /// `None` leaves the server default.
    pub interslot_updates: Option<bool>,
}

impl Default for SlotPrefilter {
    fn default() -> Self {
        Self {
            filter_by_commitment: true,
            interslot_updates: None, // Leave the server default, as before
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

        // Interslot updates only add traffic, so either side asking for them
        // wins, and an explicit request beats leaving the server default.
        lhs.interslot_updates = match (lhs.interslot_updates, rhs.interslot_updates) {
            (Some(a), Some(b)) => Some(a || b),
            (Some(want), None) | (None, Some(want)) => Some(want),
            (None, None) => None,
        };
    }
}

/// Helper macro for converting Shipstern's [`Pubkey`] to a Solana ed25519
/// public key.
///
/// Invoking the macro with the name of a publicly-exported Solana `Pubkey`
/// type (e.g. `pubkey_convert_helpers!(solana_sdk::pubkey::Pubkey);`) will
/// define two functions:
///
/// - `pub(crate) fn into_shipstern_pubkey(`<Solana Pubkey>`) -> shipstern_core::Pubkey;`
/// - `pub(crate) fn from_shipstern_pubkey(shipstern_core::Pubkey) -> <Solana Pubkey>;`
///
/// These can be used as a convenience for quickly converting between Solana
/// public keys and their representation in Shipstern.  Shipstern does not use the
/// built-in Solana `Pubkey` type, nor does it provide `From`/`Into` impls for
/// it, to avoid creating an unnecessary dependency on any specific version of
/// the full Solana SDK.
#[macro_export]
macro_rules! pubkey_convert_helpers {
    ($ty:ty) => {
        pub(crate) fn into_shipstern_pubkey(value: $ty) -> $crate::Pubkey {
            value.to_bytes().into()
        }

        pub(crate) fn from_shipstern_pubkey(value: $crate::Pubkey) -> $ty {
            value.into_bytes().into()
        }
    };
}

/// A 32-byte Solana public key.
///
/// This is a convenience alias for [`KeyBytes<32>`] that provides a familiar
/// name for Solana developers.
pub type Pubkey = KeyBytes<32>;

/// Protobuf wrapper for a 32-byte public key.
///
/// This struct wraps raw public key bytes for protobuf serialization.
/// It derives [`prost::Message`] so it can be used as a nested message field
/// (`message PublicKey { bytes value = 1; }`).
///
/// Generated code uses [`Pubkey`] in struct fields and converts to/from this
/// wrapper at proto encode/decode time.
#[cfg(feature = "proto")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKeyProtoWrapper {
    /// The raw bytes of the public key.
    #[prost(bytes = "vec", tag = "1")]
    pub value: Vec<u8>,
}

#[cfg(feature = "proto")]
impl PublicKeyProtoWrapper {
    /// Creates a new `PublicKeyProtoWrapper` from any type convertible to `Vec<u8>`.
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

impl<const LEN: usize> Default for KeyBytes<LEN> {
    fn default() -> Self { Self([0u8; LEN]) }
}

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

/// `prost::Message` impl for [`Pubkey`] (`KeyBytes<32>`).
///
/// Delegates to [`PublicKeyProtoWrapper`] so there is a single source of truth
/// for the `message PublicKey { bytes value = 1; }` wire format.
#[cfg(feature = "proto")]
impl ::prost::Message for KeyBytes<32> {
    fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
        PublicKeyProtoWrapper::new(self.0).encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut impl ::prost::bytes::Buf,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError> {
        let mut wrapper = PublicKeyProtoWrapper::new(self.0);

        wrapper.merge_field(tag, wire_type, buf, ctx)?;

        let bytes = &wrapper.value;

        if bytes.len() != 32 {
            // DecodeError::new is doc(hidden) + deprecated but explicitly intended
            // for Message implementations, which is exactly our use case.
            #[allow(deprecated)]
            return Err(::prost::DecodeError::new(
                "expected exactly 32 bytes for Pubkey",
            ));
        }

        self.0.copy_from_slice(bytes);

        Ok(())
    }

    fn encoded_len(&self) -> usize { PublicKeyProtoWrapper::new(self.0).encoded_len() }

    fn clear(&mut self) { self.0 = [0u8; 32]; }
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

impl<const LEN: usize> serde::Serialize for KeyBytes<LEN> {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(&bs58::encode(&self.0).into_string())
    }
}

impl<'de, const LEN: usize> serde::Deserialize<'de> for KeyBytes<LEN> {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        let s = <&str>::deserialize(de)?;
        let v = bs58::decode(s).into_vec().map_err(D::Error::custom)?;
        let arr: [u8; LEN] = v.try_into().map_err(|v: Vec<u8>| {
            D::Error::custom(format!(
                "KeyBytes<{LEN}>: expected {LEN} bytes, got {}",
                v.len()
            ))
        })?;
        Ok(Self(arr))
    }
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
    /// A memcmp comparison carried an encoding the server would reject.
    #[error("Invalid {encoding} in memcmp data at offset {offset}: {message}")]
    BadMemcmpData {
        /// Which encoding failed to decode.
        encoding: &'static str,
        /// The offset the comparison was declared at.
        offset: u64,
        /// What the decoder said.
        message: String,
    },
    /// A memcmp comparison had nothing to compare against.
    #[error("Empty memcmp data at offset {0}, which matches every account")]
    EmptyMemcmpData(u64),
    /// A data slice asked for no bytes, which yields empty account data.
    #[error("Zero-length accounts data slice at offset {0}")]
    ZeroLengthDataSlice(u64),
}

impl AccountFilter {
    /// Check that the server could act on this filter.
    ///
    /// Encoding is validated here rather than at send time, so a bad config
    /// fails at load instead of taking the subscription down mid-run.
    ///
    /// # Errors
    ///
    /// See [`PrefilterError::BadMemcmpData`] and
    /// [`PrefilterError::EmptyMemcmpData`].
    ///
    pub fn validate(&self) -> Result<(), PrefilterError> {
        let Self::Memcmp { offset, data } = self else {
            return Ok(());
        };

        let decoded = match data {
            MemcmpData::Bytes(bytes) => bytes.clone(),
            MemcmpData::Base58(text) => {
                bs58::decode(text)
                    .into_vec()
                    .map_err(|err| PrefilterError::BadMemcmpData {
                        encoding: "base58",
                        offset: *offset,
                        message: err.to_string(),
                    })?
            },
            MemcmpData::Base64(text) => {
                base64_decode(text).ok_or_else(|| PrefilterError::BadMemcmpData {
                    encoding: "base64",
                    offset: *offset,
                    message: "not valid base64".to_owned(),
                })?
            },
        };

        if decoded.is_empty() {
            return Err(PrefilterError::EmptyMemcmpData(*offset));
        }

        Ok(())
    }
}

/// Decode standard base64 without pulling in a dependency for it.
fn base64_decode(text: &str) -> Option<Vec<u8>> {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let text = text.trim_end_matches('=');
    let mut out = Vec::with_capacity(text.len() * 3 / 4);
    let mut acc: u32 = 0;
    let mut bits = 0_u32;

    for byte in text.bytes() {
        let value = u32::try_from(TABLE.iter().position(|c| *c == byte)?)
            .expect("a table index is at most 63");
        acc = (acc << 6) | value;
        bits += 6;

        if bits >= 8 {
            bits -= 8;
            out.push(((acc >> bits) & 0xff) as u8);
        }
    }

    Some(out)
}

impl AccountsDataSlice {
    /// Check that the slice would return bytes.
    ///
    /// # Errors
    ///
    /// See [`PrefilterError::ZeroLengthDataSlice`].
    ///
    pub fn validate(&self) -> Result<(), PrefilterError> {
        if self.length == 0 {
            return Err(PrefilterError::ZeroLengthDataSlice(self.offset));
        }

        Ok(())
    }
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
    block_accounts_include: Option<HashSet<Pubkey>>,
    /// Matching [`BlockPrefilter::include_accounts`]
    block_include_accounts: bool,
    /// Matching [`BlockPrefilter::include_transactions`]
    block_include_transactions: bool,
    /// Matching [`BlockPrefilter::include_entries`]
    block_include_entries: bool,
    /// Including all accounts
    accounts_include_all: bool,
    /// Matching [`AccountPrefilter::accounts`]
    accounts: Option<HashSet<Pubkey>>,
    /// Matching [`AccountPrefilter::account_owners`]
    account_owners: Option<HashSet<Pubkey>>,
    /// Matching [`AccountPrefilter::filters`]
    account_filters: Option<Vec<AccountFilter>>,
    /// Matching [`AccountPrefilter::nonempty_txn_signature`]
    account_nonempty_txn_signature: Option<bool>,
    /// Matching [`TransactionPrefilter::accounts_exclude`]
    transaction_accounts_exclude: Option<HashSet<Pubkey>>,
    /// Matching [`TransactionPrefilter::vote`]
    transaction_vote: Option<bool>,
    /// Matching [`TransactionPrefilter::signature`]
    transaction_signature: Option<String>,
    /// Matching [`SlotPrefilter::interslot_updates`]
    slot_interslot_updates: Option<bool>,
    /// Matching [`TransactionPrefilter::accounts_include`]
    transaction_accounts_include: Option<HashSet<Pubkey>>,
    /// Matching [`TransactionPrefilter::accounts_required`]
    transaction_accounts_required: Option<HashSet<Pubkey>>,
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
            account_filters,
            account_nonempty_txn_signature,
            transaction_accounts_exclude,
            transaction_vote,
            transaction_signature,
            slot_interslot_updates,
        } = self;
        if let Some(err) = error {
            return Err(err);
        }

        let account = AccountPrefilter {
            accounts: accounts.unwrap_or_default(),
            owners: account_owners.unwrap_or_default(),
            filters: account_filters.unwrap_or_default(),
            nonempty_txn_signature: account_nonempty_txn_signature,
        };

        let transaction = TransactionPrefilter {
            accounts_include: transaction_accounts_include.unwrap_or_default(),
            accounts_exclude: transaction_accounts_exclude.unwrap_or_default(),
            accounts_required: transaction_accounts_required.unwrap_or_default(),
            vote: transaction_vote,
            signature: transaction_signature,
            ..Default::default()
        };

        let block_meta = BlockMetaPrefilter {};

        let block = BlockPrefilter {
            accounts_include: block_accounts_include.unwrap_or_default(),
            include_accounts: block_include_accounts,
            include_transactions: block_include_transactions,
            include_entries: block_include_entries,
        };

        let slot = SlotPrefilter {
            interslot_updates: slot_interslot_updates,
            ..Default::default()
        };

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

    /// Narrow the account subscription with server-side data comparisons.
    ///
    /// Several comparisons are `ANDed` by the server, so each one narrows
    /// further. Validated here, so a bad encoding fails the build rather than
    /// the subscription.
    ///
    /// ```rust, ignore
    /// Prefilter::builder()
    ///     .account_owners([spl_token::ID])
    ///     .account_filters([AccountFilter::DataSize(165)])
    /// ```
    ///
    pub fn account_filters<I: IntoIterator<Item = AccountFilter>>(self, it: I) -> Self {
        self.mutate(|this| {
            let filters: Vec<_> = it.into_iter().collect();

            for filter in &filters {
                filter.validate()?;
            }

            set_opt(&mut this.account_filters, "account_filters", filters)
        })
    }

    /// Receive only account updates carrying a transaction signature.
    ///
    /// The default receives every update, including those with no signature.
    ///
    pub fn account_nonempty_txn_signature(self, required: bool) -> Self {
        self.mutate(|this| {
            this.account_nonempty_txn_signature = Some(required);
            Ok(())
        })
    }

    /// Drop transactions touching any of these accounts.
    ///
    /// Exclusion beats inclusion on the wire, so a key here is not delivered
    /// even when another axis would have matched it.
    ///
    pub fn transaction_accounts_exclude<I: IntoIterator>(self, it: I) -> Self
    where I::Item: AsRef<[u8]> {
        self.mutate(|this| {
            set_opt(
                &mut this.transaction_accounts_exclude,
                "transaction_accounts_exclude",
                collect_pubkeys(it)?,
            )
        })
    }

    /// Receive only vote transactions, or only non-vote ones.
    ///
    /// The default receives both.
    ///
    pub fn transaction_vote(self, vote: bool) -> Self {
        self.mutate(|this| {
            this.transaction_vote = Some(vote);
            Ok(())
        })
    }

    /// Receive only the transaction with this signature.
    pub fn transaction_signature(self, signature: impl Into<String>) -> Self {
        self.mutate(|this| {
            set_opt(
                &mut this.transaction_signature,
                "transaction_signature",
                signature.into(),
            )
        })
    }

    /// Receive the extra updates the server emits between slots.
    pub fn slot_interslot_updates(self, wanted: bool) -> Self {
        self.mutate(|this| {
            this.slot_interslot_updates = Some(wanted);
            Ok(())
        })
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

/// A collection of filters for a Shipstern subscription.
#[derive(Debug, Clone, PartialEq)]
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

    /// The prefilter registered under `parser_id`, if any.
    #[inline]
    #[must_use]
    pub fn get(&self, parser_id: &str) -> Option<&Prefilter> { self.parsers_filters.get(parser_id) }

    /// Parser IDs this set subscribes for.
    #[inline]
    pub fn parser_ids(&self) -> impl Iterator<Item = &str> {
        self.parsers_filters.keys().map(String::as_str)
    }

    /// Replace the prefilter under `parser_id`, returning the previous one.
    ///
    /// ```rust, ignore
    /// let narrower = Prefilter::builder().account_owners([mint]).build()?;
    /// filters.insert(parser.id(), narrower);
    /// ```
    ///
    #[inline]
    pub fn insert(
        &mut self,
        parser_id: impl Into<String>,
        prefilter: Prefilter,
    ) -> Option<Prefilter> {
        self.parsers_filters.insert(parser_id.into(), prefilter)
    }

    /// Union `prefilter` into the one under `parser_id`, inserting it when
    /// there is none yet.
    ///
    /// ```rust, ignore
    /// let extra = Prefilter::builder().account_owners([new_mint]).build()?;
    /// filters.merge(parser.id(), extra);
    /// ```
    ///
    /// The union is over the field sets, which widens a field the server reads
    /// as OR and narrows one it reads as AND. Two cases to know:
    ///
    /// - An empty set means "match everything" on the wire, so merging into a
    ///   match-all **narrows** it: one owner merged into an
    ///   `accounts_include_all` prefilter leaves that owner alone subscribed.
    /// - `accounts_required` is an AND on the wire, so unioning it **narrows**
    ///   too: `{A}` merged with `{B}` requires a transaction to touch both.
    ///
    /// Use [`Self::insert`] to replace a prefilter deliberately, and check
    /// [`Self::get`] first when the existing one may be either shape.
    ///
    pub fn merge(&mut self, parser_id: impl Into<String>, prefilter: Prefilter) {
        match self.parsers_filters.entry(parser_id.into()) {
            Entry::Occupied(mut existing) => existing.get_mut().merge(prefilter),
            Entry::Vacant(slot) => {
                slot.insert(prefilter);
            },
        }
    }

    /// Drop the prefilter under `parser_id`, so the set no longer subscribes
    /// for that parser. Returns the removed prefilter.
    #[inline]
    pub fn remove(&mut self, parser_id: &str) -> Option<Prefilter> {
        self.parsers_filters.remove(parser_id)
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

impl From<&MemcmpData> for wire_memcmp::Data {
    fn from(value: &MemcmpData) -> Self {
        match value {
            MemcmpData::Bytes(bytes) => Self::Bytes(bytes.clone()),
            MemcmpData::Base58(text) => Self::Base58(text.clone()),
            MemcmpData::Base64(text) => Self::Base64(text.clone()),
        }
    }
}

impl From<LamportsCmp> for wire_lamports::Cmp {
    fn from(value: LamportsCmp) -> Self {
        match value {
            LamportsCmp::Eq(lamports) => Self::Eq(lamports),
            LamportsCmp::Ne(lamports) => Self::Ne(lamports),
            LamportsCmp::Lt(lamports) => Self::Lt(lamports),
            LamportsCmp::Gt(lamports) => Self::Gt(lamports),
        }
    }
}

impl From<&AccountFilter> for SubscribeRequestFilterAccountsFilter {
    fn from(value: &AccountFilter) -> Self {
        let filter = match value {
            AccountFilter::Memcmp { offset, data } => {
                wire_filter::Filter::Memcmp(SubscribeRequestFilterAccountsFilterMemcmp {
                    offset: *offset,
                    data: Some(data.into()),
                })
            },
            AccountFilter::DataSize(size) => wire_filter::Filter::Datasize(*size),
            AccountFilter::TokenAccountState(state) => {
                wire_filter::Filter::TokenAccountState(*state)
            },
            AccountFilter::Lamports(cmp) => {
                wire_filter::Filter::Lamports(SubscribeRequestFilterAccountsFilterLamports {
                    cmp: Some((*cmp).into()),
                })
            },
        };

        Self {
            filter: Some(filter),
        }
    }
}

impl From<AccountsDataSlice> for SubscribeRequestAccountsDataSlice {
    fn from(value: AccountsDataSlice) -> Self {
        Self {
            offset: value.offset,
            length: value.length,
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
                        filters: v.filters.iter().map(Into::into).collect(),
                        nonempty_txn_signature: v.nonempty_txn_signature,
                        // Cuckoo-filter account matching (proto 12.6) is not
                        // exposed through `AccountPrefilter`; opt out for now.
                        cuckoo_accounts_filter: None,
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
                        interslot_updates: slot_filter.interslot_updates,
                    }))
                })
                .collect(),
            transactions: value
                .parsers_filters
                .iter()
                .filter_map(|(k, v)| {
                    let v = v.transaction.as_ref()?;

                    Some((k.clone(), SubscribeRequestFilterTransactions {
                        vote: v.vote,
                        failed: v.failed,
                        // Cuckoo-filter account matching (proto 12.6.0) is not
                        // exposed through `TransactionFilter`; opt out for now.
                        cuckoo_account_include: None,
                        signature: v.signature.clone(),
                        account_include: v
                            .accounts_include
                            .iter()
                            .map(ToString::to_string)
                            .collect(),
                        account_exclude: v
                            .accounts_exclude
                            .iter()
                            .map(ToString::to_string)
                            .collect(),
                        account_required: v
                            .accounts_required
                            .iter()
                            .map(ToString::to_string)
                            .collect(),
                        token_accounts: None,
                    }))
                })
                .collect(),
            // Transaction-status and entry updates are separate subscription
            // kinds with no pipeline type behind them, so nothing could consume
            // what they delivered. Unsupported rather than unimplemented.
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
                        cuckoo_account_include: None,
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
            // Request-level rather than per-parser, so it is set by the source
            // from its configuration after this conversion.
            accounts_data_slice: vec![],
            ping: None,
            from_slot: None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn owned_by(marker: u8) -> Prefilter {
        Prefilter {
            account: Some(AccountPrefilter {
                accounts: HashSet::new(),
                owners: HashSet::from([Pubkey::new([marker; 32])]),
            }),
            ..Default::default()
        }
    }

    fn owners_of(filters: &Filters, parser_id: &str) -> HashSet<Pubkey> {
        filters
            .get(parser_id)
            .and_then(|prefilter| prefilter.account.as_ref())
            .map(|account| account.owners.clone())
            .unwrap_or_default()
    }

    #[test]
    fn test_filters_merge_unions_into_an_existing_prefilter() {
        let mut filters = Filters::new(HashMap::from([("p".to_owned(), owned_by(1))]));

        filters.merge("p", owned_by(2));

        assert_eq!(
            owners_of(&filters, "p"),
            HashSet::from([Pubkey::new([1; 32]), Pubkey::new([2; 32])])
        );
    }

    #[test]
    fn test_filters_merge_inserts_when_absent() {
        let mut filters = Filters::new(HashMap::new());

        filters.merge("p", owned_by(1));

        assert_eq!(
            owners_of(&filters, "p"),
            HashSet::from([Pubkey::new([1; 32])])
        );
    }

    #[test]
    fn test_filters_insert_replaces_and_returns_the_previous_prefilter() {
        let mut filters = Filters::new(HashMap::from([("p".to_owned(), owned_by(1))]));

        let previous = filters.insert("p", owned_by(2));

        assert_eq!(
            previous
                .and_then(|prefilter| prefilter.account)
                .map(|a| a.owners),
            Some(HashSet::from([Pubkey::new([1; 32])]))
        );
        assert_eq!(
            owners_of(&filters, "p"),
            HashSet::from([Pubkey::new([2; 32])])
        );
    }

    /// An empty field set means "match everything" on the wire, and `merge`
    /// unions the sets, so merging one owner into a match-all leaves that owner
    /// alone subscribed. Documented on `merge` because it reads as widening and
    /// is not; `insert` is the way to replace a match-all deliberately.
    #[test]
    fn test_filters_merge_narrows_a_match_all() {
        let match_all = Prefilter {
            account: Some(AccountPrefilter::default()),
            ..Default::default()
        };

        let mut filters = Filters::new(HashMap::from([("p".to_owned(), match_all)]));

        filters.merge("p", owned_by(1));

        assert_eq!(
            owners_of(&filters, "p"),
            HashSet::from([Pubkey::new([1; 32])]),
            "merging into a match-all narrows it rather than widening"
        );
    }

    /// `accounts_required` is an AND on the wire, so unioning two required sets
    /// asks for transactions touching both rather than either. Documented on
    /// `merge` because it reads as widening and is not.
    #[test]
    fn test_filters_merge_narrows_accounts_required() {
        let requiring = |key: u8| Prefilter {
            transaction: Some(TransactionPrefilter {
                accounts_required: HashSet::from([Pubkey::new([key; 32])]),
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut filters = Filters::new(HashMap::from([("p".to_owned(), requiring(1))]));

        filters.merge("p", requiring(2));

        let required = filters
            .get("p")
            .and_then(|prefilter| prefilter.transaction.as_ref())
            .map(|transaction| transaction.accounts_required.clone())
            .expect("required set must be present");

        assert_eq!(
            required,
            HashSet::from([Pubkey::new([1; 32]), Pubkey::new([2; 32])]),
            "merging required accounts asks for both, which is narrower"
        );
    }

    #[test]
    fn test_filters_remove_drops_the_parser() {
        let mut filters = Filters::new(HashMap::from([("p".to_owned(), owned_by(1))]));

        assert!(filters.remove("p").is_some());

        assert!(filters.get("p").is_none());
        assert_eq!(filters.parser_ids().count(), 0);
        assert!(filters.remove("p").is_none());
    }

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

    fn one(prefilter: Prefilter) -> SubscribeRequest {
        Filters::new(HashMap::from([("p".to_owned(), prefilter)])).into()
    }

    fn account_with(build: fn(PrefilterBuilder) -> PrefilterBuilder) -> SubscribeRequest {
        one(
            build(Prefilter::builder().account_owners([Pubkey::new([9; 32])]))
                .build()
                .expect("prefilter must build"),
        )
    }

    /// Every field #303 listed has to survive the conversion, so one assertion
    /// each on the built request rather than on the prefilter.
    #[test]
    fn memcmp_filter_reaches_the_request() {
        let request = account_with(|b| {
            b.account_filters([AccountFilter::Memcmp {
                offset: 8,
                data: MemcmpData::Bytes(vec![1, 2, 3]),
            }])
        });

        let filters = &request.accounts.get("p").expect("account filter").filters;

        assert_eq!(filters.len(), 1);
        assert!(matches!(
            filters[0].filter,
            Some(wire_filter::Filter::Memcmp(ref memcmp))
                if memcmp.offset == 8
                    && memcmp.data == Some(wire_memcmp::Data::Bytes(vec![1, 2, 3]))
        ));
    }

    #[test]
    fn datasize_token_state_and_lamports_reach_the_request() {
        let request = account_with(|b| {
            b.account_filters([
                AccountFilter::DataSize(165),
                AccountFilter::TokenAccountState(true),
                AccountFilter::Lamports(LamportsCmp::Gt(1_000)),
            ])
        });

        let filters = &request.accounts.get("p").expect("account filter").filters;

        assert_eq!(filters.len(), 3);
        assert!(matches!(
            filters[0].filter,
            Some(wire_filter::Filter::Datasize(165))
        ));
        assert!(matches!(
            filters[1].filter,
            Some(wire_filter::Filter::TokenAccountState(true))
        ));
        assert!(matches!(
            filters[2].filter,
            Some(wire_filter::Filter::Lamports(ref cmp))
                if cmp.cmp == Some(wire_lamports::Cmp::Gt(1_000))
        ));
    }

    #[test]
    fn nonempty_txn_signature_reaches_the_request() {
        let request = account_with(|b| b.account_nonempty_txn_signature(true));

        assert_eq!(
            request
                .accounts
                .get("p")
                .expect("account filter")
                .nonempty_txn_signature,
            Some(true)
        );
    }

    #[test]
    fn transaction_exclude_vote_and_signature_reach_the_request() {
        let prefilter = Prefilter::builder()
            .transaction_accounts_include([Pubkey::new([1; 32])])
            .transaction_accounts_exclude([Pubkey::new([2; 32])])
            .transaction_vote(false)
            .transaction_signature("sig")
            .build()
            .expect("prefilter must build");

        let request = one(prefilter);
        let transactions = request.transactions.get("p").expect("txn filter");

        assert_eq!(transactions.account_exclude, vec![
            Pubkey::new([2; 32]).to_string()
        ]);
        assert_eq!(transactions.vote, Some(false));
        assert_eq!(transactions.signature.as_deref(), Some("sig"));
    }

    #[test]
    fn interslot_updates_reaches_the_request() {
        let prefilter = Prefilter::builder()
            .slots()
            .slot_interslot_updates(true)
            .build()
            .expect("prefilter must build");

        assert_eq!(
            one(prefilter)
                .slots
                .get("p")
                .expect("slot filter")
                .interslot_updates,
            Some(true)
        );
    }

    /// The defaults have to leave the wire exactly as it was before the fields
    /// existed, or every existing subscription changes shape.
    #[test]
    fn defaults_leave_the_request_unchanged() {
        let request = account_with(|b| b);
        let accounts = request.accounts.get("p").expect("account filter");

        assert!(accounts.filters.is_empty());
        assert_eq!(accounts.nonempty_txn_signature, None);
        assert!(request.accounts_data_slice.is_empty());
    }

    #[test]
    fn memcmp_validation_rejects_bad_encodings() {
        let bad_base58 = AccountFilter::Memcmp {
            offset: 0,
            data: MemcmpData::Base58("0OIl".to_owned()),
        };
        assert!(matches!(
            bad_base58.validate(),
            Err(PrefilterError::BadMemcmpData { .. })
        ));

        let empty = AccountFilter::Memcmp {
            offset: 4,
            data: MemcmpData::Bytes(vec![]),
        };
        assert!(matches!(
            empty.validate(),
            Err(PrefilterError::EmptyMemcmpData(4))
        ));

        let good = AccountFilter::Memcmp {
            offset: 0,
            data: MemcmpData::Base64("AQID".to_owned()),
        };
        assert!(good.validate().is_ok());
    }

    #[test]
    fn zero_length_data_slice_is_rejected() {
        assert!(matches!(
            AccountsDataSlice {
                offset: 3,
                length: 0
            }
            .validate(),
            Err(PrefilterError::ZeroLengthDataSlice(3))
        ));
        assert!(AccountsDataSlice {
            offset: 0,
            length: 8
        }
        .validate()
        .is_ok());
    }

    /// The server ANDs the filter list, so two prefilters asking for different
    /// comparisons cannot both be honoured; the union has to drop the
    /// comparison and over-deliver.
    #[test]
    fn merging_different_account_filters_widens_to_none() {
        let with = |size| AccountPrefilter {
            filters: vec![AccountFilter::DataSize(size)],
            ..Default::default()
        };

        let mut lhs = with(165);
        lhs.merge(with(82));

        assert!(lhs.filters.is_empty());

        let mut same = with(165);
        same.merge(with(165));

        assert_eq!(same.filters, vec![AccountFilter::DataSize(165)]);
    }

    /// Exclusion is a negation, so the union of deliveries is the intersection
    /// of the exclude sets.
    #[test]
    fn merging_excludes_keeps_only_the_common_ones() {
        let excluding = |keys: &[u8]| TransactionPrefilter {
            accounts_exclude: keys.iter().map(|k| Pubkey::new([*k; 32])).collect(),
            ..Default::default()
        };

        let mut lhs = excluding(&[1, 2]);
        lhs.merge(excluding(&[2, 3]));

        assert_eq!(
            lhs.accounts_exclude,
            HashSet::from([Pubkey::new([2; 32])]),
            "only a key both sides exclude stays excluded"
        );
    }

    fn transaction_prefilter(failed: Option<bool>) -> TransactionPrefilter {
        TransactionPrefilter {
            accounts_include: HashSet::new(),
            accounts_required: HashSet::new(),
            failed,
            ..Default::default()
        }
    }

    #[test]
    fn test_transaction_prefilter_merge_same_values_unchanged() {
        {
            let mut a = transaction_prefilter(Some(false));
            a.merge(transaction_prefilter(Some(false)));
            assert_eq!(
                a.failed,
                Some(false),
                "Some(false) + Some(false) => Some(false)"
            );
        }

        {
            let mut a = transaction_prefilter(Some(true));
            a.merge(transaction_prefilter(Some(true)));
            assert_eq!(
                a.failed,
                Some(true),
                "Some(true) + Some(true) => Some(true)"
            );
        }

        {
            let mut a = transaction_prefilter(None);
            a.merge(transaction_prefilter(None));
            assert_eq!(a.failed, None, "None + None => None");
        }
    }

    #[test]
    fn test_transaction_prefilter_merge_different_values_widen_to_none() {
        let mut a = transaction_prefilter(Some(false));
        a.merge(transaction_prefilter(Some(true)));

        assert_eq!(
            a.failed, None,
            "BUG: Some(false) + Some(true) must widen to None (accept all)"
        );
    }

    #[test]
    fn test_transaction_prefilter_merge_none_widens_to_none() {
        {
            let mut a = transaction_prefilter(Some(false));
            a.merge(transaction_prefilter(None));
            assert_eq!(a.failed, None, "Some(false) + None => None");
        }

        {
            let mut a = transaction_prefilter(None);
            a.merge(transaction_prefilter(Some(true)));
            assert_eq!(a.failed, None, "None + Some(true) => None");
        }
    }

    #[test]
    fn test_transaction_prefilter_merge_commutativity() {
        let values = [Some(false), Some(true), None];

        for &lhs in &values {
            for &rhs in &values {
                let mut ab = transaction_prefilter(lhs);
                ab.merge(transaction_prefilter(rhs));

                let mut ba = transaction_prefilter(rhs);
                ba.merge(transaction_prefilter(lhs));

                assert_eq!(
                    ab, ba,
                    "merge({lhs:?}, {rhs:?}) should equal merge({rhs:?}, {lhs:?}) (commutativity)"
                );
            }
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
        let key1: Pubkey = [1u8; 32].into();
        let key2: Pubkey = [2u8; 32].into();

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
