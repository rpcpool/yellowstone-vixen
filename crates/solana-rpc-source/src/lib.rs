use std::time::Duration;

use async_trait::async_trait;
use solana_account_decoder_client_types::UiAccountEncoding;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
};
use solana_commitment_config::CommitmentConfig;
use solana_pubkey::Pubkey;
use tokio::{sync::mpsc::Sender, task::JoinSet};
use yellowstone_grpc_proto::{
    geyser::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeUpdate, SubscribeUpdateAccount,
        SubscribeUpdateAccountInfo,
    },
    tonic::Status,
};
use yellowstone_vixen::{sources::SourceTrait, Error as VixenError};
use yellowstone_vixen_core::Filters;

/// A `Source` implementation for the Solana Accounts RPC API.
#[derive(Debug)]
pub struct SolanaAccountsRpcSource;

/// The configuration for the Solana Accounts RPC source.
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct SolanaAccountsRpcConfig {
    /// The endpoint of the RPC server.
    pub endpoint: String,
    /// The timeout for the connection.
    pub timeout: u64,
}

impl SolanaAccountsRpcSource {
    fn get_commitment_config(filters: &Filters) -> CommitmentConfig {
        match filters.global_filters.commitment {
            Some(CommitmentLevel::Finalized) => CommitmentConfig::finalized(),
            Some(CommitmentLevel::Processed) => CommitmentConfig::processed(),
            _ => CommitmentConfig::confirmed(),
        }
    }
}

#[async_trait]
impl SourceTrait for SolanaAccountsRpcSource {
    type Config = SolanaAccountsRpcConfig;

    fn name() -> String { "solana-accounts-rpc".to_string() }

    async fn connect(
        config: Self::Config,
        filters: Filters,
        tx: Sender<Result<SubscribeUpdate, Status>>,
    ) -> Result<(), VixenError> {
        let name = Self::name();

        let mut tasks_set = JoinSet::new();

        for (filter_id, prefilter) in &filters.parsers_filters {
            if let Some(account_prefilter) = &prefilter.account {
                for program in &account_prefilter.owners {
                    let program_id = Pubkey::new_from_array(program.0);
                    let config = config.clone();
                    let tx = tx.clone();
                    let filter_id = filter_id.clone();
                    let name = name.clone();

                    let client = RpcClient::new_with_timeout_and_commitment(
                        config.endpoint.clone(),
                        Duration::from_secs(config.timeout),
                        Self::get_commitment_config(&filters),
                    );

                    tasks_set.spawn(async move {
                        let slot = client.get_slot().await;

                        if let Err(e) = &slot {
                            tracing::error!(
                                "Failed to get slot: {} for source: {}, filter: {}",
                                e,
                                &name,
                                filter_id
                            );
                        }
                        let slot = slot.unwrap();

                        let accounts = client
                            .get_program_accounts_with_config(
                                &program_id,
                                RpcProgramAccountsConfig {
                                    filters: None,
                                    account_config: RpcAccountInfoConfig {
                                        encoding: Some(UiAccountEncoding::Base64),
                                        data_slice: None,
                                        commitment: None, // Already set in the client
                                        min_context_slot: None,
                                    },
                                    with_context: Some(true),
                                    sort_results: Some(false),
                                },
                            )
                            .await
                            .map_err(|e| VixenError::Io(std::io::Error::other(e.to_string())));

                        if let Err(e) = &accounts {
                            tracing::error!(
                                "Failed to get program accounts: {} for source: {}, filter: {}",
                                e,
                                &name,
                                filter_id
                            );
                        }

                        for (acc_pubkey, account) in accounts.unwrap() {
                            let update = SubscribeUpdate {
                                filters: vec![filter_id.clone()],
                                created_at: None,
                                update_oneof: Some(UpdateOneof::Account(SubscribeUpdateAccount {
                                    account: Some(SubscribeUpdateAccountInfo {
                                        pubkey: acc_pubkey.as_array().to_vec(),
                                        lamports: account.lamports,
                                        owner: account.owner.as_array().to_vec(),
                                        executable: account.executable,
                                        rent_epoch: account.rent_epoch,
                                        data: account.data,
                                        write_version: 0,
                                        txn_signature: None,
                                    }),
                                    slot,
                                    is_startup: true,
                                })),
                            };

                            let res = tx.send(Ok(update)).await;

                            if res.is_err() {
                                tracing::error!(
                                    "Failed to send update to buffer for source: {}, filter: {}",
                                    &name,
                                    filter_id
                                );
                            }
                        }
                    });
                }
            }
        }

        tasks_set.join_all().await;

        Ok(())
    }
}
