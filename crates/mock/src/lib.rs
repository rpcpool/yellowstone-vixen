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
    fmt::{Debug, Display},
    fs,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

pub use futures;
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_request::RpcRequest};
use solana_rpc_client_api::client_error::Result as ClientResult;
use solana_sdk::{account::Account, bs58, pubkey::Pubkey, signature::Signature};
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedConfirmedTransactionWithStatusMeta,
    EncodedTransaction, EncodedTransactionWithStatusMeta, UiCompiledInstruction,
    UiInnerInstructions, UiInstruction, UiMessage,
};
use yellowstone_grpc_proto::{
    geyser::{
        SubscribeUpdateAccount, SubscribeUpdateAccountInfo, SubscribeUpdateTransaction,
        SubscribeUpdateTransactionInfo,
    },
    solana::storage::confirmed_block::{
        CompiledInstruction, InnerInstruction, InnerInstructions, Message, Transaction,
        TransactionStatusMeta,
    },
};
use yellowstone_vixen_core::{
    instruction::{InstructionShared, InstructionUpdate},
    ProgramParser, Pubkey as VixenPubkey, TransactionUpdate,
};

//TODO: Look these up from the Vixen.toml config file
const RPC_ENDPOINT: &str = "https://api.mainnet-beta.solana.com";
const FIXTURES_PATH: &str = "./fixtures";
const PUBKEY_REGEX: &str = r"^[1-9A-HJ-NP-Za-km-z]{32,44}$";
const TX_SIGNATURE_REGEX: &str = r"^[1-9A-HJ-NP-Za-km-z]{64,90}$";
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub data: Vec<u8>,
    pub pubkey: Pubkey,
    pub executable: bool,
    pub lamports: u64,
    pub owner: Pubkey,
    pub rent_epoch: u64,
    pub space: u64,
}

impl From<AccountInfo> for SubscribeUpdateAccount {
    fn from(value: AccountInfo) -> Self {
        Self {
            is_startup: false,
            slot: 0,
            account: Some(SubscribeUpdateAccountInfo {
                txn_signature: None,
                write_version: 0,
                pubkey: value.pubkey.to_bytes().to_vec(),
                data: value.data,
                executable: value.executable,
                lamports: value.lamports,
                owner: value.owner.to_bytes().to_vec(),
                rent_epoch: value.rent_epoch,
            }),
        }
    }
}

impl TryFrom<SubscribeUpdateAccount> for AccountInfo {
    type Error = &'static str;

    fn try_from(value: SubscribeUpdateAccount) -> Result<Self, Self::Error> {
        let account_info = value.account.ok_or("Missing account info")?;

        let pubkey = Pubkey::new_from_array(
            account_info
                .pubkey
                .try_into()
                .map_err(|_| "Invalid pubkey length")?,
        );

        let owner = Pubkey::new_from_array(
            account_info
                .owner
                .try_into()
                .map_err(|_| "Invalid owner length")?,
        );

        Ok(Self {
            pubkey,
            data: account_info.data.clone(),
            executable: account_info.executable,
            lamports: account_info.lamports,
            owner,
            rent_epoch: account_info.rent_epoch,
            space: account_info.data.len() as u64,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SerializablePubkey(pub [u8; 32]);

impl Debug for SerializablePubkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { Display::fmt(self, f) }
}

impl Display for SerializablePubkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&bs58::encode(&self.0).into_string())
    }
}

impl From<VixenPubkey> for SerializablePubkey {
    fn from(value: VixenPubkey) -> Self { Self(value.into_bytes()) }
}

impl From<SerializablePubkey> for VixenPubkey {
    fn from(value: SerializablePubkey) -> Self { Self::new(value.0) }
}

pub type IxIndex = [usize; 2]; // [outer_ix_index, inner_ix_index]

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SerializableInstructionUpdate {
    pub ix_index: u16,
    pub parent_program: Option<SerializablePubkey>,
    pub program: SerializablePubkey,
    pub accounts: Vec<SerializablePubkey>,
    pub data: Vec<u8>,
    pub inner: Vec<SerializableInstructionUpdate>,
}

impl From<&InstructionUpdate> for SerializableInstructionUpdate {
    fn from(value: &InstructionUpdate) -> Self {
        Self {
            ix_index: value.ix_index,
            parent_program: value
                .parent_program
                .map(|p| SerializablePubkey(p.into_bytes())),
            program: SerializablePubkey(value.program.0),
            accounts: value
                .accounts
                .iter()
                .map(|x| SerializablePubkey(x.into_bytes()))
                .collect(),
            data: value.data.clone(),
            inner: value.inner.iter().map(Into::into).collect(),
        }
    }
}

impl From<&SerializableInstructionUpdate> for InstructionUpdate {
    fn from(value: &SerializableInstructionUpdate) -> Self {
        Self {
            program: value.program.into(),
            accounts: value.accounts.iter().copied().map(Into::into).collect(),
            data: value.data.clone(),
            shared: Arc::new(InstructionShared::default()),
            inner: value.inner.iter().map(Into::into).collect(),
            ix_index: value.ix_index,
            parent_program: value.parent_program.map(Into::into),
            parsed_logs: vec![],
        }
    }
}

pub fn get_account_pubkey_from_index(
    index: usize,
    accounts: &[String],
) -> Result<SerializablePubkey, String> {
    if accounts.is_empty() {
        return Err("No accounts found".to_string());
    }

    accounts.get(index).map_or(
        Err(format!(
            "Account index {} out of bounds for {} accounts",
            index,
            accounts.len(),
        )),
        |account| {
            Ok(VixenPubkey::from_str(account)
                .map_err(|e| e.to_string())?
                .into())
        },
    )
}

fn try_from_ui_instructions(
    ui_ixs: &[UiCompiledInstruction],
    accounts: &[String],
    program_id: &str,
) -> Result<Vec<SerializableInstructionUpdate>, String> {
    let mut ixs: Vec<SerializableInstructionUpdate> = Vec::new();
    for (idx, ix) in ui_ixs.iter().enumerate() {
        let accounts_out = ix
            .accounts
            .iter()
            .map(|account| get_account_pubkey_from_index(*account as usize, accounts))
            .collect::<Result<Vec<SerializablePubkey>, String>>()?;
        let program = get_account_pubkey_from_index(ix.program_id_index as usize, accounts)?;

        let ix = SerializableInstructionUpdate {
            ix_index: u16::try_from(idx).unwrap_or(u16::MAX),
            parent_program: None,
            data: decode_bs58_to_bytes(&ix.data)?,
            accounts: accounts_out,
            program,
            inner: Vec::new(),
        };

        ixs.push(ix);
    }
    Ok(filter_ixs(ixs, program_id))
}

fn try_from_ui_inner_ixs(
    ui_inner_ixs: &UiInnerInstructions,
    accounts: &[String],
    program_id: &str,
    next_idx: &mut u16,
) -> Result<Vec<SerializableInstructionUpdate>, String> {
    let mut ixs: Vec<SerializableInstructionUpdate> = Vec::new();
    for ix in &ui_inner_ixs.instructions {
        if let UiInstruction::Compiled(compiled_ix) = ix {
            let accounts_out = compiled_ix
                .accounts
                .iter()
                .map(|account| get_account_pubkey_from_index(*account as usize, accounts))
                .collect::<Result<Vec<SerializablePubkey>, String>>()?;
            let program =
                get_account_pubkey_from_index(compiled_ix.program_id_index as usize, accounts)?;

            let ix = SerializableInstructionUpdate {
                ix_index: *next_idx,
                parent_program: None, // TODO: This should be set to the actual parent program when parsing inner instructions
                data: decode_bs58_to_bytes(&compiled_ix.data)?,
                accounts: accounts_out,
                program,
                inner: Vec::new(),
            };
            *next_idx += 1;
            ixs.push(ix);
        } else {
            return Err("Invalid inner instruction".into());
        }
    }
    Ok(filter_ixs(ixs, program_id))
}

fn filter_ixs(
    ixs: Vec<SerializableInstructionUpdate>,
    program_id: &str,
) -> Vec<SerializableInstructionUpdate> {
    // Filter out instructions that matches the program
    ixs.into_iter()
        .filter(|ix| ix.program.to_string().eq(program_id))
        .collect::<Vec<SerializableInstructionUpdate>>()
}

#[allow(clippy::too_many_lines)]
fn convert_to_transaction_update(
    value: EncodedConfirmedTransactionWithStatusMeta,
) -> Result<TransactionUpdate, String> {
    let EncodedConfirmedTransactionWithStatusMeta {
        transaction,
        slot,
        block_time: _,
    } = value;
    let EncodedTransactionWithStatusMeta {
        transaction,
        meta,
        version: _,
    } = transaction;

    let mut account_keys: Vec<Vec<u8>> = Vec::new();
    let mut instructions: Vec<CompiledInstruction> = Vec::new();
    let mut inner_instructions: Vec<InnerInstructions> = Vec::new();
    let mut signatures: Vec<Vec<u8>> = Vec::new();
    let message_header: Option<yellowstone_grpc_proto::prelude::MessageHeader>;
    let recent_blockhash: Vec<u8>;

    if let EncodedTransaction::Json(tx_data) = transaction {
        if let UiMessage::Raw(raw_message) = tx_data.message {
            // Extract and convert message header
            message_header = Some(yellowstone_grpc_proto::prelude::MessageHeader {
                num_required_signatures: u32::from(raw_message.header.num_required_signatures),
                num_readonly_signed_accounts: u32::from(
                    raw_message.header.num_readonly_signed_accounts,
                ),
                num_readonly_unsigned_accounts: u32::from(
                    raw_message.header.num_readonly_unsigned_accounts,
                ),
            });

            // Extract recent blockhash
            recent_blockhash = bs58::decode(&raw_message.recent_blockhash)
                .into_vec()
                .map_err(|e| format!("Error decoding recent blockhash: {e:?}"))?;

            // Convert account keys from strings to bytes
            for key_str in raw_message.account_keys {
                let key_bytes = bs58::decode(key_str)
                    .into_vec()
                    .map_err(|e| format!("Error decoding account key: {e:?}"))?;
                account_keys.push(key_bytes);
            }

            // Convert instructions
            for ui_instruction in raw_message.instructions {
                instructions.push(CompiledInstruction {
                    program_id_index: u32::from(ui_instruction.program_id_index),
                    accounts: ui_instruction.accounts,
                    data: decode_bs58_to_bytes(&ui_instruction.data)?,
                });
            }

            // Convert signatures
            for sig_str in tx_data.signatures {
                let sig_bytes = bs58::decode(sig_str)
                    .into_vec()
                    .map_err(|e| format!("Error decoding signature: {e:?}"))?;
                signatures.push(sig_bytes);
            }
        } else {
            return Err("Invalid transaction encoding".into());
        }
    } else {
        return Err("Invalid transaction encoding".into());
    }

    let mut loaded_writable_addresses: Vec<Vec<u8>> = Vec::new();
    let mut loaded_readonly_addresses: Vec<Vec<u8>> = Vec::new();

    if let Some(meta) = &meta {
        // Convert inner instructions
        if let OptionSerializer::Some(inner_ixs) = &meta.inner_instructions {
            for inner_ix in inner_ixs {
                let mut converted_instructions: Vec<InnerInstruction> = Vec::new();
                for ui_instruction in &inner_ix.instructions {
                    if let UiInstruction::Compiled(compiled_ix) = ui_instruction {
                        converted_instructions.push(InnerInstruction {
                            program_id_index: u32::from(compiled_ix.program_id_index),
                            accounts: compiled_ix.accounts.clone(),
                            data: decode_bs58_to_bytes(&compiled_ix.data)?,
                            stack_height: compiled_ix.stack_height,
                        });
                    }
                }
                inner_instructions.push(InnerInstructions {
                    index: u32::from(inner_ix.index),
                    instructions: converted_instructions,
                });
            }
        }

        // Convert loaded addresses
        if let OptionSerializer::Some(loaded) = &meta.loaded_addresses {
            for addr_str in &loaded.writable {
                let addr_bytes = bs58::decode(addr_str)
                    .into_vec()
                    .map_err(|e| format!("Error decoding loaded writable address: {e:?}"))?;
                loaded_writable_addresses.push(addr_bytes);
            }
            for addr_str in &loaded.readonly {
                let addr_bytes = bs58::decode(addr_str)
                    .into_vec()
                    .map_err(|e| format!("Error decoding loaded readonly address: {e:?}"))?;
                loaded_readonly_addresses.push(addr_bytes);
            }
        }
    }

    let transaction_info = SubscribeUpdateTransactionInfo {
        signature: signatures.first().cloned().unwrap_or_default(),
        is_vote: false, // Default for mock
        transaction: Some(Transaction {
            signatures,
            message: Some(Message {
                header: message_header,
                account_keys,
                recent_blockhash,
                instructions,
                versioned: false,
                address_table_lookups: vec![],
            }),
        }),
        meta: meta.map(|m| TransactionStatusMeta {
            err: None, // Convert transaction error types is complex, skip for mock
            fee: m.fee,
            pre_balances: m.pre_balances,
            post_balances: m.post_balances,
            inner_instructions,
            inner_instructions_none: false,
            log_messages: match m.log_messages {
                OptionSerializer::Some(logs) => logs,
                OptionSerializer::None | OptionSerializer::Skip => vec![],
            },
            log_messages_none: false,
            pre_token_balances: match m.pre_token_balances {
                OptionSerializer::Some(_) | OptionSerializer::None | OptionSerializer::Skip => {
                    vec![]
                }, // Token balance conversion is complex, skip for mock
            },
            post_token_balances: match m.post_token_balances {
                OptionSerializer::Some(_) | OptionSerializer::None | OptionSerializer::Skip => {
                    vec![]
                }, // Token balance conversion is complex, skip for mock
            },
            rewards: match m.rewards {
                OptionSerializer::Some(_) | OptionSerializer::None | OptionSerializer::Skip => {
                    vec![]
                }, // Reward conversion is complex, skip for mock
            },
            loaded_writable_addresses,
            loaded_readonly_addresses,
            return_data: None,
            return_data_none: false,
            compute_units_consumed: match m.compute_units_consumed {
                OptionSerializer::Some(units) => Some(units),
                OptionSerializer::None | OptionSerializer::Skip => None,
            },
            cost_units: None,
        }),
        index: 0, // Default for mock
    };

    Ok(SubscribeUpdateTransaction {
        slot,
        transaction: Some(transaction_info),
    })
}

fn try_from_tx_meta<P: ProgramParser>(
    value: EncodedConfirmedTransactionWithStatusMeta,
    parser: &P,
) -> Result<Vec<SerializableInstructionUpdate>, String> {
    let EncodedConfirmedTransactionWithStatusMeta {
        transaction,
        slot: _,
        block_time: _,
    } = value;
    let EncodedTransactionWithStatusMeta {
        transaction,
        meta,
        version: _,
    } = transaction;
    let mut inner_ixs: Option<Vec<UiInnerInstructions>> = None;

    let mut account_keys: Vec<String> = Vec::new();
    let program_id = parser.program_id().to_string();

    if let EncodedTransaction::Json(tx_data) = transaction {
        if let UiMessage::Raw(raw_message) = tx_data.message {
            account_keys.extend(raw_message.account_keys);

            if let Some(meta) = meta {
                inner_ixs = meta.inner_instructions.map(Some).flatten();

                if let OptionSerializer::Some(loaded) = meta.loaded_addresses {
                    for address in loaded.writable {
                        account_keys.push(address);
                    }

                    for address in loaded.readonly {
                        account_keys.push(address);
                    }
                }
            }

            // filtering outer instructions by program id
            let mut program_filtered_ixs =
                try_from_ui_instructions(&raw_message.instructions, &account_keys, &program_id)?;

            // filtering inner instructions by program id
            if let Some(inner_ixs) = inner_ixs {
                if inner_ixs.is_empty() {
                    return Ok(program_filtered_ixs);
                }

                let mut next_idx = u16::try_from(program_filtered_ixs.len()).unwrap_or(u16::MAX);
                for ixs in inner_ixs {
                    let inner_ixs =
                        try_from_ui_inner_ixs(&ixs, &account_keys, &program_id, &mut next_idx)?;
                    if inner_ixs.is_empty() {
                        continue;
                    }

                    program_filtered_ixs.extend(inner_ixs);
                }

                return Ok(program_filtered_ixs);
            }
        } else {
            return Err("Invalid transaction encoding".into());
        }
    }

    Err("Invalid transaction encoding".into())
}

#[macro_export]
macro_rules! account_fixture {
    ($pubkey:expr, $parser:expr) => {
        match $crate::load_fixture($pubkey, $parser).await.unwrap() {
            FixtureData::Account(a) => {
                run_account_parse!($parser, a)
            },
            f @ _ => panic!("Invalid account fixture {f:?}"),
        }
    };
}

#[macro_export]
macro_rules! tx_fixture {
    ($sig:expr, $parser:expr) => {
        match $crate::load_fixture($sig, $parser).await.unwrap() {
            $crate::FixtureData::Instructions(ixs) => {
                let futures = ixs.iter().map(|ix| {
                    let parser = $parser.clone();
                    async move { $crate::run_ix_parse!(parser, ix) }
                });
                $crate::futures::future::join_all(futures).await
            },
            f @ _ => panic!("Invalid transaction fixture {f:?}"),
        }
    };
}
#[macro_export]
macro_rules! run_account_parse {
    ($parser:expr, $account:expr) => {
        $parser.parse(&$account).await.unwrap()
    };
}

#[macro_export]
macro_rules! run_ix_parse {
    ($parser:expr, $ix:expr) => {
        $parser.parse(&$ix.into()).await.unwrap()
    };
}

/// Create a mock `TransactionUpdate` from a transaction signature for testing
pub async fn create_mock_transaction_update(
    signature: &str,
) -> Result<TransactionUpdate, Box<dyn std::error::Error>> {
    let sig = Signature::from_str(signature)?;
    let rpc_client = get_rpc_client();

    let params = json!([sig.to_string(), {
        "encoding": "json",
        "maxSupportedTransactionVersion": 0
    }]);

    let tx = rpc_client
        .send(RpcRequest::GetTransaction, params)
        .await
        .map_err(|e| format!("Error fetching tx: {e:?}"))?;

    convert_to_transaction_update(tx).map_err(Into::into)
}

/// Parse instructions from a `TransactionUpdate` using the core `parse_from_txn` logic
pub fn parse_instructions_from_txn_update(
    txn_update: &TransactionUpdate,
) -> Result<Vec<InstructionUpdate>, Box<dyn std::error::Error>> {
    InstructionUpdate::parse_from_txn(txn_update).map_err(Into::into)
}

pub async fn load_fixture<P: ProgramParser>(
    fixture: &str,
    parser: &P,
) -> Result<FixtureData, Box<dyn std::error::Error>> {
    maybe_create_fixture_dir()?;
    let path = fixture_path(fixture)?;
    if path.is_file() {
        read_fixture(&path)
    } else {
        fetch_fixture(fixture, parser)
            .await
            .and_then(write_fixture(path))
    }
}

fn convert_account_info(pubkey: Pubkey) -> impl Fn(Account) -> ClientResult<AccountInfo> {
    move |value: Account| {
        Ok(AccountInfo {
            data: value.data.clone(),
            executable: value.executable,
            lamports: value.lamports,
            owner: value.owner,
            rent_epoch: value.rent_epoch,
            space: value.data.len() as u64,
            pubkey,
        })
    }
}

#[must_use]
pub fn get_rpc_client() -> RpcClient { RpcClient::new(RPC_ENDPOINT.to_string()) }

#[derive(Debug, Clone)]
pub enum FixtureData {
    Account(SubscribeUpdateAccount),
    Instructions(Vec<SerializableInstructionUpdate>),
}

async fn fetch_fixture<P: ProgramParser>(
    fixture: &str,
    parser: &P,
) -> Result<FixtureData, Box<dyn std::error::Error>> {
    let fixture_type = get_fixture_type(fixture);

    match fixture_type {
        FixtureType::Pubkey => {
            let pubkey = Pubkey::from_str(fixture)?;
            let rpc_client = get_rpc_client();

            let account_info = rpc_client
                .get_account(&pubkey)
                .await
                .and_then(convert_account_info(pubkey))?;

            Ok(FixtureData::Account(SubscribeUpdateAccount::from(
                account_info,
            )))
        },
        FixtureType::Signature => {
            let signature = Signature::from_str(fixture)?;
            let rpc_client = get_rpc_client();

            let params = json!([signature.to_string(), {
                "encoding": "json",
                "maxSupportedTransactionVersion": 0
            }]);

            let tx = rpc_client
                .send(RpcRequest::GetTransaction, params)
                .await
                .map_err(|e| format!("Error fetching tx: {e:?}"))?;

            let instructions = try_from_tx_meta(tx, parser)?;

            Ok(FixtureData::Instructions(instructions))
        },
        FixtureType::Invalid => Err("Invalid fixture".into()),
    }
}

fn write_fixture(
    path: PathBuf,
) -> impl Fn(FixtureData) -> Result<FixtureData, Box<dyn std::error::Error>> {
    move |data: FixtureData| {
        match data.clone() {
            FixtureData::Account(account) => {
                let writable = AccountInfo::try_from(account.clone())?;
                let data = serde_json::to_string(&writable)?;

                fs::write(&path, data)?;
            },
            FixtureData::Instructions(instructions) => {
                let data = serde_json::to_string(&instructions)?;
                fs::write(&path, data)?;
            },
        }
        Ok(data)
    }
}

fn maybe_create_fixture_dir() -> std::io::Result<()> {
    let dir_exists = Path::new(FIXTURES_PATH).is_dir();

    if dir_exists {
        return Ok(());
    }

    std::fs::create_dir(FIXTURES_PATH)
}

#[derive(Debug, Clone, Copy)]
pub enum FixtureType {
    Pubkey,
    Signature,
    Invalid,
}

#[must_use]
pub fn get_fixture_type(fixture: &str) -> FixtureType {
    if regex::Regex::new(TX_SIGNATURE_REGEX)
        .unwrap()
        .is_match(fixture)
    {
        FixtureType::Signature
    } else if regex::Regex::new(PUBKEY_REGEX).unwrap().is_match(fixture) {
        FixtureType::Pubkey
    } else {
        FixtureType::Invalid
    }
}

pub fn fixture_path(fixture: &str) -> Result<PathBuf, String> {
    let mut file_name = fixture.to_string();
    let fixture_type = get_fixture_type(fixture);
    match fixture_type {
        FixtureType::Pubkey => file_name.push_str("_account"),
        FixtureType::Signature => file_name.push_str("_tx"),
        FixtureType::Invalid => return Err("Invalid fixture".to_string()),
    }
    file_name.push_str(".json");

    Ok(Path::new(FIXTURES_PATH).join(file_name))
}

pub fn read_account_fixture(data: &[u8]) -> Result<FixtureData, Box<dyn std::error::Error>> {
    let account_info: AccountInfo = serde_json::from_slice(data)?;

    Ok(FixtureData::Account(SubscribeUpdateAccount::from(
        account_info,
    )))
}

pub fn read_instructions_fixture(data: &[u8]) -> Result<FixtureData, Box<dyn std::error::Error>> {
    let instructions: Vec<SerializableInstructionUpdate> = serde_json::from_slice(data)?;
    Ok(FixtureData::Instructions(instructions))
}

pub fn read_fixture(path: &Path) -> Result<FixtureData, Box<dyn std::error::Error>> {
    let data = std::fs::read(path)?;

    let fixture_type = get_fixture_type(
        path.file_stem()
            .ok_or("Invalid fixture path")?
            .to_str()
            .ok_or("Invalid fixture path")?
            .split('_')
            .next()
            .ok_or("Invalid fixture path")?,
    );

    match fixture_type {
        FixtureType::Pubkey => read_account_fixture(&data),
        FixtureType::Signature => read_instructions_fixture(&data),
        FixtureType::Invalid => Err("Invalid fixture".into()),
    }
}

pub fn decode_bs58_to_bytes(bs58: &str) -> Result<Vec<u8>, String> {
    let bytes = bs58::decode(bs58)
        .into_vec()
        .map_err(|e| format!("Error decoding bs58: {e:?}"))?;
    Ok(bytes)
}
