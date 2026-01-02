use std::{collections::HashMap, sync::Arc, time::Duration};

use async_trait::async_trait;
use clap::ValueEnum;
use futures_util::{SinkExt, StreamExt};
use tokio::{
    sync::{mpsc::Sender, Mutex},
    task::JoinSet,
    time::interval,
};
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::{
    geyser::{
        subscribe_update::UpdateOneof, SubscribeRequest, SubscribeRequestPing, SubscribeUpdate,
    },
    tonic::{codec::CompressionEncoding, transport::ClientTlsConfig, Status},
};
use yellowstone_vixen::{sources::SourceTrait, CommitmentLevel, Error as VixenError};
use yellowstone_vixen_core::Filters;

#[derive(Default, Copy, Debug, serde::Deserialize, Clone, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum VixenCompressionEncoding {
    Gzip,
    #[default]
    Zstd,
}

impl From<VixenCompressionEncoding> for CompressionEncoding {
    fn from(val: VixenCompressionEncoding) -> Self {
        match val {
            VixenCompressionEncoding::Gzip => CompressionEncoding::Gzip,
            VixenCompressionEncoding::Zstd => CompressionEncoding::Zstd,
        }
    }
}

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

    #[arg(long, env)]
    pub max_decoding_message_size: Option<usize>,

    #[arg(long, env)]
    pub accept_compression: Option<VixenCompressionEncoding>,
}

/// A `Source` implementation for the Yellowstone gRPC API.
#[derive(Debug)]
pub struct YellowstoneGrpcSource {
    filters: Filters,
    config: YellowstoneGrpcConfig,
}

#[async_trait]
impl SourceTrait for YellowstoneGrpcSource {
    type Config = YellowstoneGrpcConfig;

    fn new(config: Self::Config, filters: Filters) -> Self { Self { config, filters } }

    async fn connect(&self, tx: Sender<Result<SubscribeUpdate, Status>>) -> Result<(), VixenError> {
        let filters = self.filters.clone();
        let config = self.config.clone();

        let timeout = Duration::from_secs(config.timeout);

        let mut tasks_set = JoinSet::new();

        for (filter_id, prefilter) in filters.parsers_filters {
            let filter = Filters::new(HashMap::from([(filter_id, prefilter)]));

            let tx = tx.clone();

            let mut client = GeyserGrpcClient::build_from_shared(config.endpoint.clone())?
                .x_token(config.x_token.clone())?
                .max_decoding_message_size(config.max_decoding_message_size.unwrap_or(usize::MAX))
                .accept_compressed(config.accept_compression.unwrap_or_default().into())
                .connect_timeout(timeout)
                .timeout(timeout)
                .tls_config(ClientTlsConfig::new().with_native_roots())?
                .connect()
                .await?;

            let mut subscribe_request: SubscribeRequest = filter.into();
            if let Some(from_slot) = config.from_slot {
                subscribe_request.from_slot = Some(from_slot);
            }
            if let Some(commitment_level) = config.commitment_level {
                subscribe_request.commitment = Some(commitment_level as i32);
            }

            let (sub_tx, stream) = client
                .subscribe_with_request(Some(subscribe_request))
                .await?;

            // Wrap the subscription sender in Arc<Mutex<>> to share between tasks
            let sub_tx = Arc::new(Mutex::new(sub_tx));
            let ping_sub_tx = Arc::clone(&sub_tx);

            // Spawn a task to receive updates and respond to server pings
            tasks_set.spawn(async move {
                let mut stream = std::pin::pin!(stream);

                while let Some(update_result) = stream.next().await {
                    // Handle server pings by responding with a ping
                    if let Ok(update) = &update_result
                        && let Some(UpdateOneof::Ping(_)) = update.update_oneof
                    {
                        tracing::debug!("Received ping from server, responding...");
                        let ping_response = SubscribeRequest {
                            ping: Some(SubscribeRequestPing { id: 1 }),
                            ..Default::default()
                        };
                        if let Err(e) = sub_tx.lock().await.send(ping_response).await {
                            tracing::warn!("Failed to send ping response to server: {}", e);
                            break;
                        }
                    }

                    // Forward all updates to the buffer
                    if let Err(_) = tx.send(update_result).await {
                        // Channel closed, likely due to shutdown - exit gracefully
                        tracing::debug!("Update channel closed, shutting down receiver task");
                        break;
                    }
                }
            });

            // Spawn a task to send periodic pings every 10 seconds
            tasks_set.spawn(async move {
                let mut ping_timer = interval(Duration::from_secs(10));
                let mut ping_id = 0i32;

                loop {
                    ping_timer.tick().await;
                    ping_id = ping_id.wrapping_add(1);

                    let ping_request = SubscribeRequest {
                        ping: Some(SubscribeRequestPing { id: ping_id }),
                        ..Default::default()
                    };

                    if let Err(e) = ping_sub_tx.lock().await.send(ping_request).await {
                        tracing::warn!("Failed to send ping to server: {}", e);
                        break;
                    }
                }
            });
        }

        tasks_set.join_all().await;

        Ok(())
    }
}
