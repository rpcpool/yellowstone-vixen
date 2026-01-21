use std::time::Duration;

use async_trait::async_trait;
use clap::ValueEnum;
use futures_util::StreamExt;
use tokio::sync::mpsc::Sender;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::{
    geyser::{SubscribeRequest, SubscribeUpdate},
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

        let mut client = GeyserGrpcClient::build_from_shared(config.endpoint.clone())?
            .x_token(config.x_token.clone())?
            .max_decoding_message_size(config.max_decoding_message_size.unwrap_or(usize::MAX))
            .accept_compressed(config.accept_compression.unwrap_or_default().into())
            .connect_timeout(timeout)
            .timeout(timeout)
            .tls_config(ClientTlsConfig::new().with_native_roots())?
            .connect()
            .await?;

        let mut subscribe_request: SubscribeRequest = filters.into();
        if let Some(from_slot) = config.from_slot {
            subscribe_request.from_slot = Some(from_slot);
        }
        if let Some(commitment_level) = config.commitment_level {
            subscribe_request.commitment = Some(commitment_level as i32);
        }

        tracing::debug!(
            has_transactions = !subscribe_request.transactions.is_empty(),
            transaction_filters = ?subscribe_request.transactions.keys().collect::<Vec<_>>(),
            has_blocks_meta = !subscribe_request.blocks_meta.is_empty(),
            blocks_meta_filters = ?subscribe_request.blocks_meta.keys().collect::<Vec<_>>(),
            has_slots = !subscribe_request.slots.is_empty(),
            slots_filters = ?subscribe_request.slots.keys().collect::<Vec<_>>(),
            from_slot = ?subscribe_request.from_slot,
            commitment = ?subscribe_request.commitment,
            "Subscribing to gRPC stream"
        );

        let (_sub_tx, stream) = client
            .subscribe_with_request(Some(subscribe_request))
            .await?;

        let mut stream = std::pin::pin!(stream);

        tracing::debug!("gRPC stream started");

        while let Some(update) = stream.next().await {
            let res = tx.send(update).await;
            if res.is_err() {
                tracing::error!("Failed to send update to buffer");
                break; // TODO: CHECK: It should have a break there ?
            }
        }

        tracing::debug!("gRPC stream ended");

        Ok(())
    }
}
