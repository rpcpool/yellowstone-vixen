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
    };
    check_or_create_fixtures_dir();
    let account_data_exists = check_account_exists_on_fixtures(pubkey, cluster);

    if !account_data_exists {
        let account = fetch_and_write_account_data(cluster, pubkey).await;
        match account {
            Some(account) => Some(get_subscribe_update_account(pubkey, account)),
            None => None,
        }
    } else {
        let account = fetch_account_data_from_file(pubkey, cluster);
        match account {
            Some(account) => Some(get_subscribe_update_account(pubkey, account)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    const VIXEN_TEST_MINT_DEVNET: &str = "AZLFB7QYN8oZK8wfUt65feHFjDonhiPHQGDcoWfZDPFf";
    use solana_sdk::genesis_config::ClusterType;
    use utils::test_parsing_token_extension_program;

    use super::*;
    #[tokio::test]
    async fn mock() {
        let account = load_account_fixtures(VIXEN_TEST_MINT_DEVNET, ClusterType::Devnet).await;
        assert!(account.is_some(), "Account not found");
        let account = account.unwrap();
        test_parsing_token_extension_program(account).await;
    }
}
