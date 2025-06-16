use std::time::Duration;

use async_trait::async_trait;
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
use yellowstone_vixen::{config::YellowstoneConfig, sources::Source, Error as VixenError};
use yellowstone_vixen_core::{Filters, GlobalFilters};

/// A `Source` implementation for the Solana Accounts RPC API.
#[derive(Debug, Default)]
pub struct SolanaAccountsRpcSource {
    config: Option<SolanaAccountsRpcConfig>,
    filters: Option<Filters>,
}

impl SolanaAccountsRpcSource {
    /// Create a new `SolanaAccountsRpcSource`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn get_commitment_config(&self) -> CommitmentConfig {
        match self.filters {
            Some(Filters {
                global_filters:
                    GlobalFilters {
                        commitment: Some(CommitmentLevel::Finalized),
                    },
                ..
            }) => CommitmentConfig::finalized(),
            Some(Filters {
                global_filters:
                    GlobalFilters {
                        commitment: Some(CommitmentLevel::Processed),
                    },
                ..
            }) => CommitmentConfig::processed(),
            _ => CommitmentConfig::confirmed(),
        }
    }
}

/// The configuration for the Solana Accounts RPC source.
#[derive(Debug, Clone)]
pub struct SolanaAccountsRpcConfig {
    /// The endpoint of the RPC server.
    pub endpoint: String,
    /// The timeout for the connection.
    pub timeout: u64,
}

#[async_trait]
impl Source for SolanaAccountsRpcSource {
    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, Status>>,
    ) -> Result<JoinSet<()>, VixenError> {
        // // We require that config and filters are set before connecting to the `Source`
        let filters = self.filters.clone().ok_or(VixenError::ConfigError)?;
        let config = self.config.clone().ok_or(VixenError::ConfigError)?;

        let mut tasks_set = JoinSet::new();

        for (filter_id, prefilter) in filters.parsers_filters {
            if let Some(account_prefilter) = prefilter.account {
                for program in account_prefilter.owners {
                    let program_id = Pubkey::new_from_array(program.0);
                    let config = config.clone();
                    let tx = tx.clone();
                    let filter_id = filter_id.clone();

                    let client = RpcClient::new_with_timeout_and_commitment(
                        config.endpoint,
                        Duration::from_secs(config.timeout),
                        self.get_commitment_config(),
                    );

                    tasks_set.spawn(async move {
                        // TODO: Only get pubkey from get_program_accounts_with_config() using data_slice
                        //  and then use get_multiple_accounts() for ~batching (process batches concurrently)
                        let accounts = client
                            .get_program_accounts_with_config(
                                &program_id,
                                RpcProgramAccountsConfig {
                                    filters: None,
                                    account_config: RpcAccountInfoConfig {
                                        encoding: None,
                                        data_slice: None,
                                        commitment: None, // Already set in the client
                                        min_context_slot: None,
                                    },
                                    with_context: Some(false),
                                    sort_results: Some(false),
                                },
                            )
                            .await
                            .map_err(|e| {
                                VixenError::Io(std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    e.to_string(),
                                ))
                            });

                        if let Err(e) = &accounts {
                            tracing::error!("Failed to get program accounts: {}", e);
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
                                    slot: 0,
                                    is_startup: true,
                                })),
                            };

                            let res = tx.send(Ok(update)).await;
                            if res.is_err() {
                                tracing::error!("Failed to send update to buffer");
                            }
                        }
                    });
                }
            }
        }

        Ok(tasks_set)
    }

    fn set_filters_unchecked(&mut self, filters: Filters) {
        self.filters = Some(filters);
    }

    fn set_config_unchecked(&mut self, config: YellowstoneConfig) {
        self.config = Some(config.into());
    }

    fn get_filters(&self) -> &Option<Filters> {
        &self.filters
    }

    fn get_config(&self) -> Option<YellowstoneConfig> {
        self.config.clone().map(SolanaAccountsRpcConfig::into)
    }
}

impl From<SolanaAccountsRpcConfig> for YellowstoneConfig {
    fn from(config: SolanaAccountsRpcConfig) -> Self {
        Self {
            endpoint: config.endpoint,
            timeout: config.timeout,
            x_token: None,
        }
    }
}

impl From<YellowstoneConfig> for SolanaAccountsRpcConfig {
    fn from(config: YellowstoneConfig) -> Self {
        Self {
            endpoint: config.endpoint,
            timeout: config.timeout,
        }
    }
}
