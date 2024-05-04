use std::{collections::HashSet, fmt, pin::Pin};

use futures_util::Future;
use yellowstone_grpc_proto::geyser::{subscribe_update::UpdateOneof, SubscribeUpdate};

// use std::fmt;

// use yellowstone_grpc_proto::{
//     geyser::{
//         subscribe_update::UpdateOneof, SubscribeUpdate, SubscribeUpdateAccount,
//         SubscribeUpdateAccountInfo, SubscribeUpdateTransaction, SubscribeUpdateTransactionInfo,
//     },
//     solana::storage::confirmed_block,
// };

// fn field_into<T: TryInto<U, Error = MessageError>, U>(
//     name: &'static str,
//     value: T,
// ) -> Result<U, MessageError> {
//     value.try_into().map_err(|e| match e {
//         MessageError::MissingField(s) => MessageError::MissingField(format!("{name}.{s}")),
//         e => e,
//     })
// }

// fn field_opt<T>(name: &'static str, value: Option<T>) -> Result<T, MessageError> {
//     value.ok_or(MessageError::MissingField(name.into()))
// }

// fn field_into_opt<T: TryInto<U, Error = MessageError>, U>(
//     name: &'static str,
//     value: Option<T>,
// ) -> Result<U, MessageError> {
//     field_into(name, field_opt(name, value)?)
// }

// fn map_opt<T: TryInto<U, Error = MessageError>, U>(
//     name: &'static str,
//     value: Option<T>,
// ) -> Result<Option<U>, MessageError> {
//     value.map(|v| field_into(name, v)).transpose()
// }

// fn field_collect<I: IntoIterator, J: FromIterator<U>, U>(
//     name: &'static str,
//     it: I,
// ) -> Result<J, MessageError>
// where
//     I::Item: TryInto<U, Error = MessageError>,
// {
//     it.into_iter().map(|t| field_into(name, t)).collect()
// }

// macro_rules! var_err {
//     ($ty:ident:: $var:ident) => {
//         Err(MessageError::UnknownVariant(
//             stringify!($ty),
//             stringify!($var),
//         ))
//     };
// }

// macro_rules! field_into {
//     ($field:ident) => {
//         field_into_opt(stringify!($field), $field)
//     };
// }

// macro_rules! field_opt {
//     ($field:ident) => {
//         field_opt(stringify!($field), $field)
//     };
// }

// macro_rules! map_opt {
//     ($field:ident) => {
//         map_opt(stringify!($field), $field)
//     };
// }

// macro_rules! field_collect {
//     ($field:ident) => {
//         field_collect(stringify!($field), $field)
//     };
// }

// TODO: using Box for this is kinda dumb but whatever
struct FieldPath(Option<Box<(&'static str, FieldPath)>>);

impl FieldPath {
    const VALUE: Self = Self(None);

    #[inline]
    fn new(field: &'static str) -> Self { Self(Some(Box::new((field, Self(None))))) }
}

impl fmt::Display for FieldPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(if self.0.is_some() { "field " } else { "value" });

        let mut any = false;
        let mut curr = &self.0;
        while let Some(b) = curr {
            if std::mem::replace(&mut any, true) {
                f.write_str(".")?;
            }

            let (s, FieldPath(ref p)) = **b;
            f.write_str(s);
            curr = p;
        }

        Ok(())
    }
}

impl fmt::Debug for FieldPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();

        let mut curr = &self.0;
        while let Some(b) = curr {
            let (s, FieldPath(ref p)) = **b;
            list.entry(&s);
            curr = p;
        }

        list.finish()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Error parsing {path}: {kind}")]
pub struct Error {
    path: FieldPath,
    kind: ErrorKind,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            path: FieldPath::VALUE,
            kind,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Value was None")]
    Missing,
    #[error("Unrecognized enum variant")]
    UnknownVariant,
}

// trait FromMessage<T>: Sized {
//     fn from_msg_opt<U: Into<Option<T>>>(val: U) -> Result<Self, MessageError> {
//         Self::from_msg(val.into().ok_or(MessageError::Missing(FieldPath::VALUE))?)
//     }

//     fn from_msg(val: T) -> Result<Self, MessageError>;
// }

// impl<T, V: FromMessage<T>> FromMessage<T> for Option<V> {
//     fn from_msg_opt<U: Into<Option<T>>>(val: U) -> Result<Self, MessageError> {
//         val.into().map(FromMessage::from_msg).transpose()
//     }

//     fn from_msg(val: T) -> Result<Self, MessageError> { Self::from_msg_opt(val) }
// }

// #[derive(Debug, Clone, PartialEq)]
// #[repr(transparent)]
// pub struct Message(pub MessageBody);

// #[derive(Debug, Clone, PartialEq)]
// pub enum MessageBody {
//     Account(Account),
//     Transaction(Transaction),
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Account {
//     pub pubkey: Vec<u8>,
//     pub lamports: u64,
//     pub owner: Vec<u8>,
//     pub executable: bool,
//     pub rent_epoch: u64,
//     pub data: Vec<u8>,
//     pub write_version: u64,
//     pub txn_signature: Option<Vec<u8>>,
//     pub slot: u64,
//     pub is_startup: bool,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Transaction {
//     signature: Vec<u8>,
//     is_vote: bool,
//     transaction: TransactionBody,
//     meta: TransactionStatusMeta,
//     index: u64,
//     slot: u64,
// }

// struct TransactionInfo {
//     signature: Vec<u8>,
//     is_vote: bool,
//     transaction: TransactionBody,
//     meta: TransactionStatusMeta,
//     index: u64,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct TransactionBody {
//     pub signatures: Vec<Vec<u8>>,
//     pub message: TransactionMessage,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct TransactionStatusMeta {
//     pub err: Option<Vec<u8>>,
//     pub fee: u64,
//     pub pre_balances: Vec<u64>,
//     pub post_balances: Vec<u64>,
//     pub inner_instructions: Vec<InnerInstructions>,
//     pub inner_instructions_none: bool,
//     pub log_messages: Vec<String>,
//     pub log_messages_none: bool,
//     pub pre_token_balances: Vec<TokenBalance>,
//     pub post_token_balances: Vec<TokenBalance>,
//     pub rewards: Vec<Reward>,
//     pub loaded_writable_addresses: Vec<Vec<u8>>,
//     pub loaded_readonly_addresses: Vec<Vec<u8>>,
//     pub return_data: Option<ReturnData>,
//     pub return_data_none: bool,
//     pub compute_units_consumed: Option<u64>,
// }

// #[derive(Debug, Clone, PartialEq)]
// struct TransactionMessage {}

// #[derive(Debug, Clone, PartialEq)]
// struct InnerInstructions {}
// #[derive(Debug, Clone, PartialEq)]
// struct TokenBalance {}
// #[derive(Debug, Clone, PartialEq)]
// struct Reward {}
// #[derive(Debug, Clone, PartialEq)]
// struct ReturnData {}

// impl TryFrom<SubscribeUpdate> for Message {
//     type Error = MessageError;

//     fn try_from(value: SubscribeUpdate) -> Result<Self, Self::Error> {
//         let SubscribeUpdate {
//             filters,
//             update_oneof,
//         } = value;

//         field_into!(update_oneof).map(Self)
//     }
// }

// impl TryFrom<UpdateOneof> for MessageBody {
//     type Error = MessageError;

//     fn try_from(value: UpdateOneof) -> Result<Self, Self::Error> {
//         match value {
//             UpdateOneof::Account(a) => a.try_into().map(Self::Account),
//             UpdateOneof::Slot(_) => var_err!(UpdateOneof::Slot),
//             UpdateOneof::Transaction(t) => t.try_into().map(Self::Transaction),
//             UpdateOneof::TransactionStatus(_) => var_err!(UpdateOneof::TransactionStatus),
//             UpdateOneof::Block(_) => var_err!(UpdateOneof::Block),
//             UpdateOneof::Ping(_) => var_err!(UpdateOneof::Ping),
//             UpdateOneof::Pong(_) => var_err!(UpdateOneof::Pong),
//             UpdateOneof::BlockMeta(_) => var_err!(UpdateOneof::BlockMeta),
//             UpdateOneof::Entry(_) => var_err!(UpdateOneof::Entry),
//         }
//     }
// }

// impl TryFrom<SubscribeUpdateAccount> for Account {
//     type Error = MessageError;

//     fn try_from(value: SubscribeUpdateAccount) -> Result<Self, Self::Error> {
//         let SubscribeUpdateAccount {
//             account,
//             slot,
//             is_startup,
//         } = value;

//         let SubscribeUpdateAccountInfo {
//             pubkey,
//             lamports,
//             owner,
//             executable,
//             rent_epoch,
//             data,
//             write_version,
//             txn_signature,
//         } = field_opt!(account)?;

//         Ok(Self {
//             pubkey,
//             lamports,
//             owner,
//             executable,
//             rent_epoch,
//             data,
//             write_version,
//             txn_signature,
//             slot,
//             is_startup,
//         })
//     }
// }

// impl TryFrom<SubscribeUpdateTransaction> for Transaction {
//     type Error = MessageError;

//     fn try_from(value: SubscribeUpdateTransaction) -> Result<Self, Self::Error> {
//         let SubscribeUpdateTransaction { transaction, slot } = value;

//         let TransactionInfo {
//             signature,
//             is_vote,
//             transaction,
//             meta,
//             index,
//         } = field_into!(transaction)?;

//         Ok(Self {
//             signature,
//             is_vote,
//             transaction,
//             meta,
//             index,
//             slot,
//         })
//     }
// }

// impl TryFrom<SubscribeUpdateTransactionInfo> for TransactionInfo {
//     type Error = MessageError;

//     fn try_from(value: SubscribeUpdateTransactionInfo) -> Result<Self, Self::Error> {
//         let SubscribeUpdateTransactionInfo {
//             signature,
//             is_vote,
//             transaction,
//             meta,
//             index,
//         } = value;

//         Ok(Self {
//             signature,
//             is_vote,
//             transaction: field_into!(transaction)?,
//             meta: field_into!(meta)?,
//             index,
//         })
//     }
// }

// impl TryFrom<confirmed_block::Transaction> for TransactionBody {
//     type Error = MessageError;

//     fn try_from(value: confirmed_block::Transaction) -> Result<Self, Self::Error> {
//         let confirmed_block::Transaction {
//             signatures,
//             message,
//         } = value;

//         Ok(Self {
//             signatures,
//             message: field_into!(message)?,
//         })
//     }
// }

// impl TryFrom<confirmed_block::TransactionStatusMeta> for TransactionStatusMeta {
//     type Error = MessageError;

//     fn try_from(value: confirmed_block::TransactionStatusMeta) -> Result<Self, Self::Error> {
//         let confirmed_block::TransactionStatusMeta {
//             err,
//             fee,
//             pre_balances,
//             post_balances,
//             inner_instructions,
//             inner_instructions_none,
//             log_messages,
//             log_messages_none,
//             pre_token_balances,
//             post_token_balances,
//             rewards,
//             loaded_writable_addresses,
//             loaded_readonly_addresses,
//             return_data,
//             return_data_none,
//             compute_units_consumed,
//         } = value;

//         Ok(TransactionStatusMeta {
//             err: err.map(|confirmed_block::TransactionError { err }| err),
//             fee,
//             pre_balances,
//             post_balances,
//             inner_instructions: field_collect!(inner_instructions)?,
//             inner_instructions_none,
//             log_messages,
//             log_messages_none,
//             pre_token_balances: field_collect!(pre_token_balances)?,
//             post_token_balances: field_collect!(post_token_balances)?,
//             rewards: field_collect!(rewards)?,
//             loaded_writable_addresses,
//             loaded_readonly_addresses,
//             return_data: field_into!(return_data)?,
//             return_data_none,
//             compute_units_consumed,
//         })
//     }
// }

// impl TryFrom<confirmed_block::Message> for TransactionMessage {
//     type Error = MessageError;

//     fn try_from(value: confirmed_block::Message) -> Result<Self, Self::Error> {
//         let confirmed_block::Message {
//             header,
//             account_keys,
//             recent_blockhash,
//             instructions,
//             versioned,
//             address_table_lookups,
//         } = value;

//         Ok(TransactionMessage {})
//     }
// }

// impl TryFrom<confirmed_block::InnerInstructions> for InnerInstructions {
//     type Error = MessageError;
// }

pub enum Message {
    AccountUpdate(AccountUpdate),
    TransactionUpdate(TransactionUpdate),
}

impl TryFrom<SubscribeUpdate> for Message {
    type Error = Error;

    fn try_from(value: SubscribeUpdate) -> Result<Self, Self::Error> {
        let SubscribeUpdate {
            filters,
            update_oneof,
        } = value;
        let update = update_oneof.ok_or(ErrorKind::Missing)?;

        Ok(match update {
            UpdateOneof::Account(a) => Self::AccountUpdate(a),
            UpdateOneof::Transaction(t) => Self::TransactionUpdate(t),
            _ => return Err(ErrorKind::UnknownVariant.into()),
        })
    }
}

// TODO
pub type AccountUpdate = yellowstone_grpc_proto::geyser::SubscribeUpdateAccount;
pub type TransactionUpdate = yellowstone_grpc_proto::geyser::SubscribeUpdateTransaction;

// TODO: get the actual Solana one
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pubkey(pub [u8; 32]);

struct Prefilter {
    pub account_owners: HashSet<Pubkey>,
    pub transaction_programs: HashSet<Pubkey>,
    pub transaction_input_accounts: HashSet<Pubkey>,
}

pub trait Parser {
    fn prefilter(&self) -> Prefilter;

    fn filter_account(&self, acct: &AccountUpdate) -> bool;

    fn filter_transaction(&self, txn: &TransactionUpdate) -> bool;

    fn process_account<'a>(
        &'a self,
        acct: &'a AccountUpdate,
    ) -> impl Future<Output = ()> + Send + 'a;

    fn process_transaction<'a>(
        &'a self,
        txn: &'a TransactionUpdate,
    ) -> impl Future<Output = ()> + Send + 'a;
}

pub trait DynParser: Send + Sync {
    fn prefilter(&self) -> Prefilter;

    fn filter_account(&self, acct: &AccountUpdate) -> bool;

    fn filter_transaction(&self, txn: &TransactionUpdate) -> bool;

    fn process_account<'a>(
        &'a self,
        acct: &'a AccountUpdate,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

    fn process_transaction<'a>(
        &'a self,
        txn: &'a TransactionUpdate,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}

pub type BoxedParser = Box<dyn DynParser>;

impl Parser for BoxedParser {
    fn prefilter(&self) -> Prefilter { DynParser::prefilter(self) }

    fn filter_account(&self, acct: &AccountUpdate) -> bool { DynParser::filter_account(self, acct) }

    fn filter_transaction(&self, txn: &TransactionUpdate) -> bool {
        DynParser::filter_transaction(self, txn)
    }

    fn process_account<'a>(
        &'a self,
        acct: &'a AccountUpdate,
    ) -> impl Future<Output = ()> + Send + 'a {
        DynParser::process_account(self, acct)
    }

    fn process_transaction<'a>(
        &'a self,
        txn: &'a TransactionUpdate,
    ) -> impl Future<Output = ()> + Send + 'a {
        DynParser::process_transaction(self, txn)
    }
}

impl<T: Parser + Send + Sync> DynParser for T {
    #[inline]
    fn prefilter(&self) -> Prefilter { Parser::prefilter(self) }

    #[inline]
    fn filter_account(&self, acct: &AccountUpdate) -> bool { Parser::filter_account(self, acct) }

    #[inline]
    fn filter_transaction(&self, txn: &TransactionUpdate) -> bool {
        Parser::filter_transaction(self, txn)
    }

    #[inline]
    fn process_account<'a>(
        &'a self,
        acct: &'a AccountUpdate,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(Parser::process_account(self, acct))
    }

    #[inline]
    fn process_transaction<'a>(
        &'a self,
        txn: &'a TransactionUpdate,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(Parser::process_transaction(self, txn))
    }
}
