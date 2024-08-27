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

mod helpers;
use std::{
    collections::{HashMap, HashSet},
    fmt,
    future::Future,
    ops,
    str::FromStr,
};

pub use helpers::{get_account_from_index, Ixs, LoadedAddresses, *};
use serde::{Deserialize, Serialize};
use yellowstone_grpc_proto::geyser::{
    subscribe_update::UpdateOneof, SubscribeRequest, SubscribeRequestFilterAccounts,
    SubscribeRequestFilterTransactions, SubscribeUpdate, SubscribeUpdateAccount,
    SubscribeUpdateTransaction,
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

#[derive(Debug)]
pub enum VixenUpdateOneOf {
    Account(AccountUpdate),
    Instructions(InstructionsUpdate),
}
#[derive(Debug)]
pub struct VixenSubscribeUpdate {
    pub filters: Vec<String>,
    pub update_oneof: Option<VixenUpdateOneOf>,
}

impl TryFrom<SubscribeUpdate> for VixenSubscribeUpdate {
    type Error = String;

    fn try_from(value: SubscribeUpdate) -> Result<Self, String> {
        let filters = value.filters;
        let update_oneof = match value.update_oneof {
            Some(UpdateOneof::Account(account)) => VixenUpdateOneOf::Account(account),
            Some(UpdateOneof::Transaction(transaction)) => {
                VixenUpdateOneOf::Instructions(InstructionsUpdate::try_from(&transaction)?)
            },
            _ => return Err("No update found".to_string()),
        };
        Ok(Self {
            filters,
            update_oneof: Some(update_oneof),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Instruction {
    pub data: Vec<u8>,
    pub accounts: Vec<Pubkey>,
    pub program_id: Pubkey,
}

impl Update for InstructionsUpdate {
    const TYPE: UpdateType = UpdateType::Instructions;
}

#[derive(Debug)]
pub struct ReadableInstruction<A, D> {
    pub accounts: A,
    pub data: Option<D>,
}

impl<A, D> ReadableInstruction<A, D> {
    pub fn new(accounts: A, data: Option<D>) -> Self { Self { accounts, data } }

    pub fn from_accounts(accounts: A) -> Self {
        Self {
            accounts,
            data: None,
        }
    }
}

#[derive(Debug)]
pub struct ReadableInstructions<I> {
    pub index: u32,
    pub instructions: Vec<I>,
}

pub trait InstructionParser<C> {
    fn parse_ix(_: &Instruction) -> Result<C, String>;
}

pub type InstructionsUpdate = Ixs;

impl TryFrom<&TransactionUpdate> for InstructionsUpdate {
    type Error = String;

    fn try_from(tx_update: &TransactionUpdate) -> Result<Self, String> {
        let tx = tx_update
            .transaction
            .as_ref()
            .ok_or("No transaction found")?;

        let tx_meta = tx.meta.as_ref().ok_or("No transaction meta found")?;
        let tx_message = tx.transaction.as_ref().map_or(
            Err("No transaction message found".to_string()),
            |tx| {
                tx.message
                    .as_ref()
                    .ok_or("No transaction message found".to_owned())
            },
        )?;

        let static_account_keys = tx_message.account_keys.to_pubkey_vec()?;

        let loaded_addresses: LoadedAddresses = LoadedAddresses {
            writable: tx_meta.loaded_writable_addresses.to_pubkey_vec()?,
            readonly: tx_meta.loaded_readonly_addresses.to_pubkey_vec()?,
        };

        let tx_accounts: TxAccountKeys = TxAccountKeys {
            static_keys: static_account_keys,
            dynamic_keys: Some(loaded_addresses),
        };

        let outer_ixs = tx_message
            .instructions
            .iter()
            .map(|ix| -> Result<Instruction, String> {
                let accounts = ix
                    .accounts
                    .iter()
                    .map(|idx| get_account_from_index(*idx as usize, &tx_accounts))
                    .collect::<Result<Vec<Pubkey>, String>>()?;

                let program_id =
                    get_account_from_index(ix.program_id_index as usize, &tx_accounts)?;
                let instruction = Instruction {
                    data: ix.data.clone(),
                    accounts,
                    program_id,
                };

                Ok(instruction)
            })
            .collect::<Result<Vec<Instruction>, String>>()?;

        let mut ix_with_inner_ixs: Vec<IxWithInnerIxs> = Vec::new();

        for inner_ix in tx_meta.inner_instructions.iter() {
            if outer_ixs.get(inner_ix.index as usize).is_none() {
                return Err("no index matched with outer ixs".to_string());
            }
            let mut inner_ixs: Vec<Instruction> = Vec::with_capacity(inner_ix.instructions.len());
            for ix in inner_ix.instructions.iter() {
                let accounts = ix
                    .accounts
                    .iter()
                    .map(|idx| get_account_from_index(*idx as usize, &tx_accounts))
                    .collect::<Result<Vec<Pubkey>, String>>()?;

                let program_id =
                    get_account_from_index(ix.program_id_index as usize, &tx_accounts)?;
                let instruction = Instruction {
                    data: ix.data.clone(),
                    accounts,
                    program_id,
                };
                inner_ixs.push(instruction);
            }
            let ix_with_inner_ix = IxWithInnerIxs {
                outer_ix: outer_ixs[inner_ix.index as usize].clone(),
                inner_ixs,
            };

            ix_with_inner_ixs.push(ix_with_inner_ix);
        }

        Ok(Self {
            instructions: ix_with_inner_ixs,
        })
    }
}

pub trait Update {
    const TYPE: UpdateType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdateType {
    Account,
    Instructions,
}

impl UpdateType {
    #[must_use]
    pub fn get(update: &Option<VixenUpdateOneOf>) -> Option<Self> {
        match update {
            Some(VixenUpdateOneOf::Account(_)) => Some(Self::Account),
            Some(VixenUpdateOneOf::Instructions(_)) => Some(Self::Instructions),
            _ => None,
        }
    }
}

impl Update for AccountUpdate {
    const TYPE: UpdateType = UpdateType::Account;
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Pubkey(pub [u8; 32]);

impl fmt::Debug for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}
impl FromStr for Pubkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded = bs58::decode(s).into_vec().map_err(|e| e.to_string())?;
        let mut bytes = [0; 32];
        bytes.copy_from_slice(&decoded);
        Ok(Self(bytes))
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
