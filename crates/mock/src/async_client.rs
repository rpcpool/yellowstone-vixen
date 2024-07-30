use serde::{Deserialize, Serialize};
use solana_sdk::genesis_config::ClusterType;

use surf::Client;

use serde_json::json;

use crate::utils::AccountInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub result: T,
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultData {
    pub context: Context,
    pub value: AccountInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub slot: u64,
}

pub async fn fetch_account_async(
    cluster: ClusterType,
    account_pubkey: &str,
) -> Result<AccountInfo, Box<dyn std::error::Error>> {
    let rpc_url = match cluster {
        ClusterType::MainnetBeta => "https://api.mainnet-beta.solana.com".to_string(),
        ClusterType::Devnet => "https://api.devnet.solana.com".to_string().to_string(),
        ClusterType::Testnet => "https://api.testnet.solana.com".to_string(),
        _ => "https://api.mainnet-beta.solana.com".to_string(),
    };
    let client = Client::new();
    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAccountInfo",
        "params": [
            account_pubkey,
            {
                "encoding": "base64"
            }
        ]
    });

    let mut response = client
        .post(rpc_url)
        .body(request_body.to_string())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let body: RpcResponse<ResultData> = response.body_json().await?;

    return Ok(body.result.value);
}
