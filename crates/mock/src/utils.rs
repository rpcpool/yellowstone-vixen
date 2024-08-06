use std::{fs, path::Path, str::FromStr};

use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{account::Account, genesis_config::ClusterType, pubkey::Pubkey};
use yellowstone_grpc_proto::geyser::{SubscribeUpdateAccount, SubscribeUpdateAccountInfo};
use yellowstone_vixen_core::Parser;
use yellowstone_vixen_parser::{TokenProgramParser, TokenProgramState};

use crate::async_client::fetch_account_async;

const FIXTURES_PATH: &str = "./fixtures";

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub data: Vec<String>,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: u64,
    pub space: u64,
}

impl TryFrom<AccountInfo> for Account {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: AccountInfo) -> Result<Self, Self::Error> {
        let data = value.data.first().ok_or("No data found in account info")?;
        let data = general_purpose::STANDARD.decode(data)?;
        Ok(Account {
            lamports: value.lamports,
            data,
            owner: Pubkey::from_str(&value.owner).unwrap(),
            executable: value.executable,
            rent_epoch: value.rent_epoch,
        })
    }
}

pub fn fetch_account_sync(pubkey: &Pubkey, rpc_endpoint: String) -> Option<Account> {
    let rpc_client = RpcClient::new(rpc_endpoint);
    let account = rpc_client.get_account(pubkey);
    account.ok()
}

pub async fn test_parsing_token_program_account(sub_account_update: SubscribeUpdateAccount) {
    let parser = TokenProgramParser;

    let data = parser.parse(&sub_account_update).await.unwrap();

    match data {
        TokenProgramState::TokenAccount(token_account) => {
            println!("Token Account: {:#?}", token_account);
        },
        TokenProgramState::Mint(mint) => {
            println!("Mint: {:#?}", mint);
        },
        TokenProgramState::Multisig(multisig) => {
            println!("Multisig: {:#?}", multisig);
        },
    }
}

pub fn get_subscribe_update_account(
    pubkey: &str,
    account_info: AccountInfo,
) -> Option<SubscribeUpdateAccount> {
    let account = Account::try_from(account_info).ok()?;
    let pubkey = Pubkey::from_str(pubkey).ok()?;
    let subscriber_account_info = SubscribeUpdateAccountInfo {
        pubkey: pubkey.as_ref().to_owned(),
        lamports: account.lamports,
        data: account.data.to_vec(),
        owner: account.owner.as_ref().to_owned(),
        executable: account.executable,
        rent_epoch: account.rent_epoch,
        write_version: 0,
        txn_signature: None,
    };

    Some(SubscribeUpdateAccount {
        account: Some(subscriber_account_info),
        slot: 0,
        is_startup: false,
    })
}

pub fn check_or_create_fixtures_dir() {
    let dir_exists = Path::new(FIXTURES_PATH).is_dir();
    if !dir_exists {
        std::fs::create_dir(FIXTURES_PATH).unwrap();
    }
}

pub fn get_account_data_file_path(pubkey: &str, cluster: ClusterType) -> Option<String> {
    let dir_exits = Path::new(FIXTURES_PATH).is_dir();
    if !dir_exits {
        return None;
    }
    let mut file_name = format_public_key(pubkey);
    match cluster {
        ClusterType::Devnet => file_name.push_str("-devnet"),
        ClusterType::Testnet => file_name.push_str("-testnet"),
        ClusterType::MainnetBeta => file_name.push_str("-mainnet"),
        _ => file_name.push_str("-mainnet"),
    }
    file_name.push_str(".json");

    let file_path = Path::new(FIXTURES_PATH).join(file_name);

    return Some(file_path.to_str().unwrap().to_string());
}

pub fn check_account_exists_on_fixtures(pubkey: &str, cluster: ClusterType) -> bool {
    let file_path = get_account_data_file_path(pubkey, cluster);
    file_path.map_or(false, |path| Path::new(&path).is_file())
}

pub fn read_from_file(file_path: &str) -> Option<AccountInfo> {
    let file = std::fs::read(file_path);
    file.map(|data| serde_json::from_slice(&data).unwrap()).ok()
}

pub fn fetch_account_data_from_file(pubkey: &str, cluster: ClusterType) -> Option<AccountInfo> {
    let account_file_path = get_account_data_file_path(pubkey, cluster);
    account_file_path.map_or(None, |path| read_from_file(&path))
}

pub fn write_to_file(file_path: &str, account: &AccountInfo) {
    let data = serde_json::to_string(account).unwrap();
    let write_res = fs::write(file_path, data);
    if write_res.is_err() {
        println!("Error writing account data to file: {}", file_path);
    }
    println!("Account data written to file: {}", file_path);
}

pub fn format_public_key(pubkey_str: &str) -> String {
    if pubkey_str.len() <= 6 {
        return pubkey_str.to_string();
    }
    let start = &pubkey_str[..3];
    let end = &pubkey_str[pubkey_str.len() - 3..];
    format!("{}..{}", start, end)
}

pub async fn fetch_and_write_account_data(
    cluster: ClusterType,
    rpc_endpoint: String,
    pubkey: &str,
) -> Option<AccountInfo> {
    let account = fetch_account_async(rpc_endpoint, pubkey).await;
    if account.is_ok() {
        let account = account.unwrap();
        let account_file_path = get_account_data_file_path(pubkey, cluster);
        if account_file_path.is_none() {
            println!(
                "Error creating account data file path for pubkey: {}",
                pubkey
            );
            return None;
        }
        write_to_file(&account_file_path.unwrap(), &account);
        return Some(account);
    } else {
        println!("Error fetching account data for pubkey: {}", pubkey);
        return None;
    }
}
