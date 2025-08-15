use std::{collections::HashMap, time::Duration};

use async_trait::async_trait;
use futures_util::StreamExt;
use tokio::{sync::mpsc::Sender, task::JoinSet};
use tracing::info;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::{
    geyser::SubscribeUpdate,
    tonic::{transport::ClientTlsConfig, Status},
};
use yellowstone_vixen::{
    config::YellowstoneConfig, sources::SourceTrait, CommitmentLevel, Error as VixenError,
};
use yellowstone_vixen_core::Filters;

/// Yellowstone connection configuration.
#[derive(Debug, clap::Args, serde::Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct YellowstoneGrpcConfig {
    /// The endpoint of the Yellowstone server.
    #[arg(long, env)]
    pub endpoint: String,
    /// The token to use for authentication.
    #[arg(long, env)]
    pub x_token: Option<String>,
    /// The timeout for the connection.
    #[arg(long, env, default_value_t = 120)]
    pub timeout: u64,

    #[arg(long, env)]
    pub commitment_level: Option<CommitmentLevel>,

    #[arg(long, env)]
    pub from_slot: Option<u64>,
}

/// A `Source` implementation for the Yellowstone gRPC API.
#[derive(Debug)]
pub struct YellowstoneGrpcSource {
    filters: Filters,
    config: YellowstoneConfig,
}

#[async_trait]
impl SourceTrait for YellowstoneGrpcSource {
    type Config = YellowstoneConfig;

    fn new(config: Self::Config, filters: Filters) -> Self {
        Self { config, filters }
    }

    async fn connect(&self, tx: Sender<Result<SubscribeUpdate, Status>>) -> Result<(), VixenError> {
        let filters = self.filters.clone();
        let config = self.config.clone();

        let timeout = Duration::from_secs(config.timeout);

        let mut tasks_set = JoinSet::new();

        for (filter_id, prefilter) in filters.parsers_filters {
            let mut filter = Filters::new(HashMap::from([(filter_id, prefilter)]));

            let config = config.clone();
            filter.from_slot(config.from_slot);
            filter.commitment(config.commitment);

            let tx = tx.clone();

            let mut client = GeyserGrpcClient::build_from_shared(config.endpoint)?
                .x_token(config.x_token)?
                .connect_timeout(timeout)
                .timeout(timeout)
                .tls_config(ClientTlsConfig::new().with_native_roots())?
                .connect()
                .await?;

            let (_sub_tx, stream) = client.subscribe_with_request(Some(filter.into())).await?;

            tasks_set.spawn(async move {
                let mut stream = std::pin::pin!(stream);

                while let Some(update) = stream.next().await {
                    let res = tx.send(update).await;
                    if res.is_err() {
                        tracing::error!("Failed to send update to buffer");
                    }
                }
            });
        }

        tasks_set.join_all().await;

        Ok(())
    }
}
