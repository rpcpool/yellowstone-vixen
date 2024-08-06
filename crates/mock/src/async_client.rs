use serde::{Deserialize, Serialize};
use serde_json::json;
use surf::Client;

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
    rpc_endpoint: String,
    account_pubkey: &str,
) -> Result<AccountInfo, Box<dyn std::error::Error>> {
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
        .post(rpc_endpoint)
        .body(request_body.to_string())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let body: RpcResponse<ResultData> = response.body_json().await?;

    return Ok(body.result.value);
}
