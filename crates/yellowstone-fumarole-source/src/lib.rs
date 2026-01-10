use std::{collections::BTreeMap, num::NonZero};

use async_trait::async_trait;
use bytesize::ByteSize;
use clap::ValueEnum;
use tokio::sync::mpsc::Sender;
use yellowstone_fumarole_client::{
    DragonsmouthAdapterSession, FumaroleClient, FumaroleSubscribeConfig, DEFAULT_PARA_DATA_STREAMS,
};
pub use yellowstone_grpc_proto::tonic::codec::CompressionEncoding;
use yellowstone_grpc_proto::{
    geyser::{SubscribeRequest, SubscribeUpdate},
    tonic::Status,
};
use yellowstone_vixen::{sources::SourceTrait, CommitmentLevel, Error as VixenError};
use yellowstone_vixen_core::Filters;

/// A `Source` implementation for the Yellowstone gRPC API.
#[derive(Debug)]
pub struct YellowstoneFumaroleSource {
    filters: Filters,
    config: FumaroleConfig,
}

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

#[derive(Debug, Clone, Default, serde::Deserialize, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct FumaroleConfig {
    /// The endpoint of the Yellowstone Fumarole server.
    pub endpoint: String,
    /// The token to use for authentication.
    pub x_token: Option<String>,
    /// Name of the persistent subscriber to use
    pub subscriber_name: String,
    /// Slot commitment level
    pub commitment_level: Option<CommitmentLevel>,
    /// max incoming decoded message size in bytes
    pub max_decoding_message_size: Option<usize>,
    /// accepted compression encoding
    pub accept_compression: Option<VixenCompressionEncoding>,
}

impl From<FumaroleConfig> for yellowstone_fumarole_client::config::FumaroleConfig {
    fn from(config: FumaroleConfig) -> Self {
        yellowstone_fumarole_client::config::FumaroleConfig {
            endpoint: config.endpoint,
            x_token: config.x_token,
            max_decoding_message_size_bytes: config.max_decoding_message_size.unwrap_or(usize::MAX),
            x_metadata: BTreeMap::new(),
            response_compression: config.accept_compression.map(Into::into),
            request_compression: config.accept_compression.map(Into::into),
            initial_connection_window_size: ByteSize::mb(100),
            initial_stream_window_size: ByteSize::mib(9),
            enable_http2_adaptive_window: true,
        }
    }
}

#[async_trait]
impl SourceTrait for YellowstoneFumaroleSource {
    type Config = FumaroleConfig;

    fn new(config: Self::Config, filters: Filters) -> Self { Self { filters, config } }

    async fn connect(&self, tx: Sender<Result<SubscribeUpdate, Status>>) -> Result<(), VixenError> {
        let filters = self.filters.clone();
        let subscriber_name = self.config.subscriber_name.clone();

        let fumarole_subscribe_config = FumaroleSubscribeConfig {
            num_data_plane_tcp_connections: NonZero::new(DEFAULT_PARA_DATA_STREAMS).unwrap(),
            ..Default::default()
        };

        let mut fumarole_client = FumaroleClient::connect(self.config.clone().into())
            .await
            .expect("failing to connect to fumarole");

        let mut subscribe_request = SubscribeRequest::from(filters);

        if let Some(commitment_level) = self.config.commitment_level {
            subscribe_request.commitment = Some(commitment_level as i32);
        }

        let dragonsmouth_session = fumarole_client
            .dragonsmouth_subscribe_with_config(
                subscriber_name,
                subscribe_request,
                fumarole_subscribe_config,
            )
            .await
            .expect("failing to subscribe to fumarole");

        let DragonsmouthAdapterSession {
            sink: _,
            mut source,
            mut fumarole_handle,
        } = dragonsmouth_session;

        loop {
            tokio::select! {
                result = &mut fumarole_handle => {
                    tracing::info!("Fumarole handle closed: {:?}", result);
                    break;
                }
                maybe_update = source.recv() => match maybe_update {
                    Some(update) => {
                        if tx.send(update).await.is_err() {
                            tracing::error!("Failed to send update to buffer");
                            break;
                        }
                    }
                    None => {
                        tracing::info!("Source returned None, exiting");
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}
