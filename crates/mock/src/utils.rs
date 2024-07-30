use base64::engine::general_purpose;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::str::FromStr;

use serde_json::{self};

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{account::Account, genesis_config::ClusterType};
use yellowstone_grpc_proto::geyser::{SubscribeUpdateAccount, SubscribeUpdateAccountInfo};
use yellowstone_vixen_core::{AccountUpdate, Parser};
use yellowstone_vixen_parser::{
    TokenExtensionProgramParser, TokenExtensionState, TokenProgramParser, TokenProgramState,
};

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

pub async fn fetch_account_sync(pubkey: &Pubkey, cluster: ClusterType) -> Option<Account> {
    let rpc_url = match cluster {
        ClusterType::MainnetBeta => "https://api.mainnet-beta.solana.com".to_string(),
        ClusterType::Devnet => "https://api.devnet.solana.com".to_string(),
        ClusterType::Testnet => "https://api.testnet.solana.com".to_string(),
        _ => "https://api.mainnet-beta.solana.com".to_string(),
    };

    let rpc_client = RpcClient::new(rpc_url);
    let account = rpc_client.get_account(pubkey);
    match account {
        Ok(acc) => Some(acc),
        Err(_) => None,
    }
}

pub async fn test_parsing_token_program(sub_account_update: SubscribeUpdateAccount) {
    let parser = TokenProgramParser;

    let data = parser.parse(&sub_account_update).await;

    assert!(data.is_ok(), "Error parsing account");

    let data = data.unwrap();

    match data {
        TokenProgramState::TokenAccount(token_account) => {
            println!("Token Account: {:#?}", token_account);
        }
        TokenProgramState::Mint(mint) => {
            println!("Mint: {:#?}", mint);
        }
        TokenProgramState::Multisig(multisig) => {
            println!("Multisig: {:#?}", multisig);
        }
    }
}

pub async fn test_parsing_token_extension_program(sub_account_update: SubscribeUpdateAccount) {
    let parser = TokenExtensionProgramParser;

    let data = parser.parse(&sub_account_update).await;

    assert!(data.is_ok(), "Error parsing account");

    let data = data.unwrap();

    match data {
        TokenExtensionState::ExtendedTokenAccount(ext_token_account) => {
            println!("Token Account with Extensions: {:#?}", ext_token_account);
        }
        TokenExtensionState::ExtendedMint(ext_mint_account) => {
            println!("Mint Account with Extensions: {:#?}", ext_mint_account);
        }
        TokenExtensionState::Multisig(multisig) => {
            println!("Multisig: {:#?}", multisig);
        }
    }
}

pub async fn test_custom_parser(
    sub_account_update: SubscribeUpdateAccount,
    parser: impl Parser<Input = AccountUpdate>,
) {
    let data = parser.parse(&sub_account_update).await;

    assert!(data.is_ok(), "Error parsing account");

    let data = data.unwrap();
}

pub fn run_async_task(callback: impl std::future::Future<Output = ()>) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(callback);
}

pub fn get_subscribe_update_account(pubkey: &str, account: AccountInfo) -> SubscribeUpdateAccount {
    let account = to_account(account);
    let pubkey = Pubkey::from_str(pubkey).unwrap();
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

    SubscribeUpdateAccount {
        account: Some(subscriber_account_info),
        slot: 0,
        is_startup: false,
    }
}

pub fn to_account(account_info: AccountInfo) -> Account {
    let account_data = account_info.data.first().unwrap();
    let account_data = general_purpose::STANDARD.decode(account_data).unwrap();
    Account {
        lamports: account_info.lamports,
        data: account_data,
        owner: Pubkey::from_str(&account_info.owner).unwrap(),
        executable: account_info.executable,
        rent_epoch: account_info.rent_epoch,
    }
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
    match file_path {
        Some(path) => Path::new(&path).is_file(),
        None => false,
    }
}

pub fn read_from_file(file_path: &str) -> Option<AccountInfo> {
    let file = std::fs::read(file_path);
    match file {
        Ok(data) => {
            let account: AccountInfo = serde_json::from_slice(&data).unwrap();
            Some(account)
        }
        Err(_) => None,
    }
}

pub fn fetch_account_data_from_file(pubkey: &str, cluster: ClusterType) -> Option<AccountInfo> {
    let account_file_path = get_account_data_file_path(pubkey, cluster);
    match account_file_path {
        Some(path) => read_from_file(&path),
        None => None,
    }
}

pub fn write_to_file(file_path: &str, account: &AccountInfo) {
    let data = serde_json::to_string(account).unwrap();
    let write_res = fs::write(file_path, data);
    match write_res {
        Ok(_) => println!("Data written to file"),
        Err(_) => println!("Error writing data to file"),
    }
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
    pubkey: &str,
) -> Option<AccountInfo> {
    let account = fetch_account_async(cluster, pubkey).await;
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
