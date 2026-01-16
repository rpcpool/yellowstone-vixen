mod autoreconnect;
mod simple;

use async_trait::async_trait;

use clap::ValueEnum;
use tokio::sync::mpsc::Sender;
use yellowstone_grpc_proto::geyser::SubscribeUpdate;
use yellowstone_grpc_proto::tonic::codec::CompressionEncoding;
use yellowstone_grpc_proto::tonic::Status;
use simple::simple_connector;
use yellowstone_vixen::Error;
use yellowstone_vixen::sources::SourceTrait;
use yellowstone_vixen_core::{CommitmentLevel, Filters};
use crate::autoreconnect::grpc_autoreconnect;

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

    #[arg(long, env)]
    #[serde(default)]
    pub strategy: YellowstoneGrpcStrategy,
}

#[derive(Default, Debug, clap::ValueEnum, serde::Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum YellowstoneGrpcStrategy {
    /// Simple connector without auto-reconnect.
    #[default]
    Simple,
    /// Auto-reconnect connector.
    Autoreconnect,
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

    fn new(config: Self::Config, filters: Filters) -> Self {
        YellowstoneGrpcSource {
            filters,
            config,
        }
    }

    async fn connect(&self, tx: Sender<Result<SubscribeUpdate, Status>>) -> Result<(), Error> {
        match self.config.strategy {
            YellowstoneGrpcStrategy::Simple => {
                let simple_source = simple_connector::YellowstoneGrpcSimpleSource::new(
                    self.config.clone(),
                    self.filters.clone(),
                );
                simple_source.connect(tx).await
            }
            YellowstoneGrpcStrategy::Autoreconnect => {
                let autoreconnect_source = grpc_autoreconnect::YellowstoneGrpcAutoconnectSource::new(
                    self.config.clone(),
                    self.filters.clone(),
                );
                autoreconnect_source.connect(tx).await
            }
        }
    }
}
