use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client_api::client_error::Result as ClientResult;
use solana_sdk::{account::Account, bs58, pubkey::Pubkey, signature::Signature};
use solana_transaction_status::{
    EncodedTransaction, EncodedTransactionWithStatusMeta, UiInstruction, UiMessage,
    UiTransactionEncoding,
};
use yellowstone_grpc_proto::geyser::{SubscribeUpdateAccount, SubscribeUpdateAccountInfo};
use yellowstone_vixen_core::{
    get_account_pubkey_from_index, Instruction, InstructionsUpdate, IxWithInnerIxs,
    Pubkey as VixenPubkey,
};

//TODO: Look these up from the Vixen.toml config file
const RPC_ENDPOINT: &str = "https://api.devnet.solana.com";
const FIXTURES_PATH: &str = "./fixtures";
const PUBKEY_REGEX: &str = r"\b[1-9A-HJ-NP-Za-km-z]{44}\b";
const TX_SIGNATURE_REGEX: &str = r"\b[1-9A-HJ-NP-Za-km-z]{88}\b";

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub data: Vec<u8>,
    pub pubkey: Pubkey,
    pub executable: bool,
    pub lamports: u64,
    pub owner: Pubkey,
    #[serde(rename = "rentEpoch")]
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

pub trait TryFromEncodedTransaction<T> {
    type Error;
    fn try_from_tx_meta(value: EncodedTransactionWithStatusMeta) -> Result<T, Self::Error>;
}

impl TryFromEncodedTransaction<InstructionsUpdate> for InstructionsUpdate {
    type Error = String;

    fn try_from_tx_meta(value: EncodedTransactionWithStatusMeta) -> Result<Self, Self::Error> {
        let encoded_tx = value.transaction;
        let inner_ixs = value
            .meta
            .map(|meta| meta.inner_instructions.map_or(None, |x| Some(x)))
            .flatten();
        let mut vixen_ixs: Vec<IxWithInnerIxs> = Vec::new();
        if let EncodedTransaction::Json(tx_data) = encoded_tx {
            if let UiMessage::Raw(raw_message) = tx_data.message {
                let account_keys = raw_message.account_keys;

                let mut outer_ixs = raw_message
                    .instructions
                    .iter()
                    .map(|ix| -> Result<Instruction, String> {
                        let accounts = ix
                            .accounts
                            .iter()
                            .map(|account| {
                                get_account_pubkey_from_index(*account as usize, &account_keys)
                            })
                            .collect::<Result<Vec<VixenPubkey>, String>>()?;
                        let program_id = get_account_pubkey_from_index(
                            ix.program_id_index as usize,
                            &account_keys,
                        )?;
                        let ix = Instruction {
                            data: decode_bs58_to_bytes(&ix.data)?,
                            accounts,
                            program_id,
                        };

                        Ok(ix)
                    })
                    .collect::<Result<Vec<Instruction>, String>>()?;
                outer_ixs.pop(); // Remove the last instruction which is a
                // set compute unit ix and will cause errors while parsing
                if let Some(inner_ixs) = inner_ixs {
                    if inner_ixs.len() == 0 {
                        for ix in outer_ixs {
                            vixen_ixs.push(IxWithInnerIxs {
                                outer_ix: ix,
                                inner_ixs: vec![],
                            });
                        }
                        return Ok(Self {
                            instructions: vixen_ixs,
                        });
                    }
                    for (idx, ix) in inner_ixs.iter().enumerate() {
                        let inner_ixs: Vec<Instruction> = ix
                            .instructions
                            .iter()
                            .map(|ix| {
                                if let UiInstruction::Compiled(compiled_ix) = ix {
                                    let accounts = compiled_ix
                                        .accounts
                                        .iter()
                                        .map(|account| {
                                            get_account_pubkey_from_index(
                                                *account as usize,
                                                &account_keys,
                                            )
                                        })
                                        .collect::<Result<Vec<VixenPubkey>, String>>()?;
                                    let program_id = get_account_pubkey_from_index(
                                        compiled_ix.program_id_index as usize,
                                        &account_keys,
                                    )?;
                                    let ix = Instruction {
                                        data: decode_bs58_to_bytes(&compiled_ix.data)?,
                                        accounts,
                                        program_id,
                                    };

                                    Ok(ix)
                                } else {
                                    Err("Invalid instruction encoding".into())
                                }
                            })
                            .collect::<Result<Vec<Instruction>, String>>()?;

                        let outer_ix = outer_ixs.get(idx).ok_or("Invalid outer ix index")?;
                        vixen_ixs.push(IxWithInnerIxs {
                            outer_ix: outer_ix.clone(),
                            inner_ixs,
                        });
                    }
                }
            } else {
                return Err("Invalid transaction encoding".into());
            }
        }

        Ok(Self {
            instructions: vixen_ixs,
        })
    }
}

#[macro_export]
macro_rules! account_fixture {
    ($pubkey:expr) => {
        $crate::load_fixture($pubkey).await.unwrap()
    };
}

#[macro_export]
macro_rules! tx_fixture {
    ($sig:expr) => {
        $crate::load_fixture($sig).await.unwrap()
    };
}

#[macro_export]
macro_rules! run_account_parse {
    ($parser:expr, $account:expr) => {
        $parser.parse(&$account).await.unwrap()
    };
}

#[macro_export]
macro_rules! run_tx_parse {
    ($parser:expr, $ixs:expr) => {
        $parser.parse(&$ixs).await.unwrap()
    };
}

pub async fn load_fixture(fixture: &str) -> Result<FixtureData, Box<dyn std::error::Error>> {
    maybe_create_fixture_dir()?;
    let path = fixture_path(fixture)?;
    if path.is_file() {
        read_fixture(path)
    } else {
        fetch_fixture(fixture).await.and_then(write_fixture(path))
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

pub fn get_rpc_client() -> RpcClient { RpcClient::new(RPC_ENDPOINT.to_string()) }

#[derive(Debug, Clone)]
pub enum FixtureData {
    Account(SubscribeUpdateAccount),
    Instructions(InstructionsUpdate),
}

// TODO: Determine pubkey vs signature based on the fixture pattern
#[must_use]
async fn fetch_fixture(fixture: &str) -> Result<FixtureData, Box<dyn std::error::Error>> {
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

            let tx = rpc_client
                .get_transaction(&signature, UiTransactionEncoding::Json)
                .await
                .map_err(|e| format!("Error fetching tx: {:?}", e))?;

            let instructions = InstructionsUpdate::try_from_tx_meta(tx.transaction)?;

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

#[derive(Debug)]
pub enum FixtureType {
    Pubkey,
    Signature,
    Invalid,
}

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

pub fn read_account_fixture(data: Vec<u8>) -> Result<FixtureData, Box<dyn std::error::Error>> {
    let account_info: AccountInfo = serde_json::from_slice(&data)?;

    Ok(FixtureData::Account(SubscribeUpdateAccount::from(
        account_info,
    )))
}

pub fn read_instructions_fixture(data: Vec<u8>) -> Result<FixtureData, Box<dyn std::error::Error>> {
    let instructions: InstructionsUpdate = serde_json::from_slice(&data)?;
    Ok(FixtureData::Instructions(instructions))
}

pub fn read_fixture(path: PathBuf) -> Result<FixtureData, Box<dyn std::error::Error>> {
    let data = std::fs::read(path.clone())?;

    let fixture_type = get_fixture_type(
        path.file_stem()
            .ok_or("Invalid fixture path")?
            .to_str()
            .ok_or("Invalid fixture path")?
            .split("_")
            .next()
            .ok_or("Invalid fixture path")?,
    );

    match fixture_type {
        FixtureType::Pubkey => read_account_fixture(data),
        FixtureType::Signature => read_instructions_fixture(data),
        FixtureType::Invalid => Err("Invalid fixture".into()),
    }
}

pub fn decode_bs58_to_bytes(bs58: &str) -> Result<Vec<u8>, String> {
    let bytes = bs58::decode(bs58)
        .into_vec()
        .map_err(|e| format!("Error decoding bs58: {:?}", e))?;
    Ok(bytes)
}
