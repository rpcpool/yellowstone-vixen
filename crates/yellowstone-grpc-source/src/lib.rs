use std::{collections::HashMap, time::Duration};

use async_trait::async_trait;
use clap::ValueEnum;
use futures_util::StreamExt;
use tokio::{sync::mpsc::Sender, task::JoinSet};
use tokio_util::sync::CancellationToken;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::{
    geyser::{SubscribeRequest, SubscribeUpdate},
    tonic::{codec::CompressionEncoding, transport::ClientTlsConfig, Status},
};
use yellowstone_vixen::{sources::SourceTrait, CommitmentLevel, Error as VixenError};
use yellowstone_vixen_core::Filters;
use crate::grpc_autoreconnect_task::{GrpcConnectionTimeouts, GrpcSourceConfig};

mod grpc_autoreconnect_util;
mod grpc_autoreconnect_task;
mod obfuscate;

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

        let shutdown_token = CancellationToken::new();
        let filters = self.filters.clone();
        let config = self.config.clone();

        let timeout = Duration::from_secs(config.timeout);

        let mut tasks_set = JoinSet::new();

        for (filter_id, prefilter) in filters.parsers_filters {
            let filter = Filters::new(HashMap::from([(filter_id, prefilter)]));

            let tx = tx.clone();

            let tls_config = ClientTlsConfig::new().with_native_roots();
            // let mut client = GeyserGrpcClient::build_from_shared(config.endpoint.clone())?
            //     .x_token(config.x_token.clone())?
            //     .max_decoding_message_size(config.max_decoding_message_size.unwrap_or(usize::MAX))
            //     .accept_compressed(config.accept_compression.unwrap_or_default().into())
            //     .connect_timeout(timeout)
            //     .timeout(timeout)
            //     .tls_config(tls_config)?
            //     .connect()
            //     .await?;

            let mut subscribe_request: SubscribeRequest = filter.into();
            if let Some(from_slot) = config.from_slot {
                subscribe_request.from_slot = Some(from_slot);
            }
            if let Some(commitment_level) = config.commitment_level {
                subscribe_request.commitment = Some(commitment_level as i32);
            }

            let timeout = Duration::from_secs(config.timeout);
            let grpc_source = GrpcSourceConfig {
                grpc_addr: config.endpoint.clone(),
                grpc_x_token: config.x_token.clone(),
                tls_config: Some(tls_config),
                max_decoding_message_size: config.max_decoding_message_size.unwrap_or(usize::MAX),
                timeouts: Some(GrpcConnectionTimeouts {
                    connect_timeout: timeout,
                    request_timeout: timeout,
                    subscribe_timeout: timeout,
                    receive_timeout: timeout,
                }),
                compression: Some(config.accept_compression.unwrap_or_default().into()),
            };
            let connect_task = grpc_autoreconnect_task::create_geyser_autoconnection_task_with_mpsc(
                grpc_source, subscribe_request,
                tx, shutdown_token.clone());
            tasks_set.spawn(connect_task);

            // let (_sub_tx, stream) = client
            //     .subscribe_with_request(Some(subscribe_request))
            //     .await?;

            // tasks_set.spawn(async move {
            //     let mut stream = std::pin::pin!(stream);
            //
            //     while let Some(update) = stream.next().await {
            //         let res = tx.send(update).await;
            //         if res.is_err() {
            //             tracing::error!("Failed to send update to buffer");
            //         }
            //     }
            // });
        }

        tasks_set.join_all().await;

        Ok(())
    }
}
