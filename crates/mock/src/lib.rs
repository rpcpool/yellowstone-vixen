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
use yellowstone_grpc_proto::geyser::{SubscribeUpdateAccount, SubscribeUpdateAccountInfo};
use yellowstone_vixen_core::{
    instruction::{InstructionShared, InstructionUpdate},
    ProgramParser, Pubkey as VixenPubkey,
};

//TODO: Look these up from the Vixen.toml config file
const RPC_ENDPOINT: &str = "https://api.devnet.solana.com";
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
    pub ix_index: IxIndex,
    pub program: SerializablePubkey,
    pub accounts: Vec<SerializablePubkey>,
    pub data: Vec<u8>,
    pub inner: Vec<SerializableInstructionUpdate>,
}

impl From<&InstructionUpdate> for SerializableInstructionUpdate {
    fn from(value: &InstructionUpdate) -> Self {
        Self {
            ix_index: [0; 2],
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
            ix_index: [idx, 0],
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
) -> Result<Vec<SerializableInstructionUpdate>, String> {
    let mut ixs: Vec<SerializableInstructionUpdate> = Vec::new();
    for (idx, ix) in ui_inner_ixs.instructions.iter().enumerate() {
        if let UiInstruction::Compiled(compiled_ix) = ix {
            let accounts_out = compiled_ix
                .accounts
                .iter()
                .map(|account| get_account_pubkey_from_index(*account as usize, accounts))
                .collect::<Result<Vec<SerializablePubkey>, String>>()?;
            let program =
                get_account_pubkey_from_index(compiled_ix.program_id_index as usize, accounts)?;

            let ix = SerializableInstructionUpdate {
                ix_index: [ui_inner_ixs.index.into(), idx],
                data: decode_bs58_to_bytes(&compiled_ix.data)?,
                accounts: accounts_out,
                program,
                inner: Vec::new(),
            };
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

                for ixs in inner_ixs {
                    let inner_ixs = try_from_ui_inner_ixs(&ixs, &account_keys, &program_id)?;
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
