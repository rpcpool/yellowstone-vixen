use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client_api::client_error::Result as ClientResult;
use solana_sdk::{account::Account, pubkey::Pubkey};
use yellowstone_grpc_proto::geyser::{SubscribeUpdateAccount, SubscribeUpdateAccountInfo};

//TODO: Look these up from the Vixen.toml config file
const RPC_ENDPOINT: &str = "https://api.devnet.solana.com";
const FIXTURES_PATH: &str = "./fixtures";

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

#[macro_export]
macro_rules! account_fixture {
    ($pubkey:expr) => {
        $crate::load_fixture($pubkey).await.unwrap()
    };
}

#[macro_export]
macro_rules! run_parse {
    ($parser:expr, $account:expr) => {
        $parser.parse(&$account).await.unwrap()
    };
}

pub async fn load_fixture(
    fixture: &str,
) -> Result<SubscribeUpdateAccount, Box<dyn std::error::Error>> {
    maybe_create_fixture_dir()?;

    let path = fixture_path(fixture);

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

// TODO: Determine pubkey vs signature based on the fixture pattern
#[must_use]
async fn fetch_fixture(
    fixture: &str,
) -> Result<SubscribeUpdateAccount, Box<dyn std::error::Error>> {
    let pubkey = Pubkey::from_str(fixture)?;
    let rpc_client = RpcClient::new(RPC_ENDPOINT.to_string());

    let account_info = rpc_client
        .get_account(&pubkey)
        .await
        .and_then(convert_account_info(pubkey))?;

    Ok(SubscribeUpdateAccount::from(account_info))
}

fn write_fixture(
    path: PathBuf,
) -> impl Fn(SubscribeUpdateAccount) -> Result<SubscribeUpdateAccount, Box<dyn std::error::Error>> {
    move |account: SubscribeUpdateAccount| {
        let writable = AccountInfo::try_from(account.clone())?;
        let data = serde_json::to_string(&writable)?;

        fs::write(&path, data)?;

        Ok(account)
    }
}

fn maybe_create_fixture_dir() -> std::io::Result<()> {
    let dir_exists = Path::new(FIXTURES_PATH).is_dir();

    if dir_exists {
        return Ok(());
    }

    std::fs::create_dir(FIXTURES_PATH)
}

pub fn fixture_path(fixture: &str) -> PathBuf {
    let mut file_name = fixture.to_string();
    file_name.push_str(".json");

    Path::new(FIXTURES_PATH).join(file_name)
}

pub fn read_fixture(path: PathBuf) -> Result<SubscribeUpdateAccount, Box<dyn std::error::Error>> {
    let data = std::fs::read(path)?;
    let account_info: AccountInfo = serde_json::from_slice(&data)?;
    Ok(SubscribeUpdateAccount::from(account_info))
}
