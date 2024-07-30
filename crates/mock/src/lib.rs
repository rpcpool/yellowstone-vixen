use solana_sdk::genesis_config::ClusterType;
use yellowstone_grpc_proto::geyser::SubscribeUpdateAccount;

pub mod async_client;
pub mod utils;

pub async fn load_account_fixtures(
    pubkey: &str,
    cluster: ClusterType,
) -> Option<SubscribeUpdateAccount> {
    use utils::{
        check_account_exists_on_fixtures, check_or_create_fixtures_dir,
        fetch_account_data_from_file, fetch_and_write_account_data, get_subscribe_update_account,
        AccountInfo,
    };
    check_or_create_fixtures_dir();
    let account_data_exists = check_account_exists_on_fixtures(pubkey, cluster);
    let account: Option<AccountInfo>;
    if account_data_exists {
        account = fetch_account_data_from_file(pubkey, cluster);
    } else {
        account = fetch_and_write_account_data(cluster, pubkey).await;
    }
    account.map_or(None, |account| {
        get_subscribe_update_account(pubkey, account)
    })
}

#[cfg(test)]
mod tests {
    const VIXEN_TEST_MINT_DEVNET: &str = "3SmPYPvZfEmroktLiJsgaNENuPEud3Z52zSfLQ1zJdkK";
    use solana_sdk::genesis_config::ClusterType;
    use yellowstone_vixen_core::Parser;
    use yellowstone_vixen_parser::{TokenProgramParser, TokenProgramState};

    use super::*;
    #[tokio::test]
    async fn mock() {
        let account = load_account_fixtures(VIXEN_TEST_MINT_DEVNET, ClusterType::Devnet)
            .await
            .unwrap();
        let parser = TokenProgramParser;

        let data = parser.parse(&account).await.unwrap();

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
}
