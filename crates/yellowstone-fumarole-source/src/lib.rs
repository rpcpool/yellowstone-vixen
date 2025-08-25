use std::{collections::BTreeMap, num::NonZero};

use async_trait::async_trait;
use bytesize::ByteSize;
use tokio::{sync::mpsc::Sender, task::JoinSet};
use yellowstone_fumarole_client::{
    proto::{CreateConsumerGroupRequest, InitialOffsetPolicy},
    DragonsmouthAdapterSession, FumaroleClient, FumaroleSubscribeConfig,
};
pub use yellowstone_grpc_proto::tonic::codec::CompressionEncoding;
use yellowstone_grpc_proto::{
    geyser::{SubscribeRequest, SubscribeUpdate},
    tonic::{Code, Status},
};
use yellowstone_vixen::{sources::SourceTrait, CommitmentLevel, Error as VixenError};
use yellowstone_vixen_core::Filters;

/// A `Source` implementation for the Yellowstone gRPC API.
#[derive(Debug)]
pub struct YellowstoneFumaroleSource {
    filters: Filters,
    config: FumaroleConfig,
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
    /// Prometheus metrics configuration (only available with 'prometheus' feature)
    #[cfg(feature = "prometheus")]
    pub metrics_endpoint: String,
    #[cfg(feature = "prometheus")]
    pub metrics_job_name: String,
    #[cfg(feature = "prometheus")]
    pub metrics_interval: u64,
    pub commitment_level: Option<CommitmentLevel>,
    pub from_slot: Option<u64>,
}

impl From<FumaroleConfig> for yellowstone_fumarole_client::config::FumaroleConfig {
    fn from(config: FumaroleConfig) -> Self {
        yellowstone_fumarole_client::config::FumaroleConfig {
            endpoint: config.endpoint,
            x_token: config.x_token,
            max_decoding_message_size_bytes: 512_000_000,
            x_metadata: BTreeMap::new(),
            response_compression: Some(CompressionEncoding::Zstd),
            request_compression: Some(CompressionEncoding::Zstd),
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

        // TODO: add tasks pool concurrency limit through config
        let mut tasks_set = JoinSet::new();

        const MAX_PARA_DATA_STREAMS: u8 = 4; //Fumarole const

        let fumarole_subscribe_config = FumaroleSubscribeConfig {
            num_data_plane_tcp_connections: NonZero::new(MAX_PARA_DATA_STREAMS).unwrap(),
            ..Default::default()
        };

        let (initial_offset_policy, from_slot) = match self.config.from_slot {
            Some(slot) => (InitialOffsetPolicy::FromSlot, Some(slot)),
            None => (InitialOffsetPolicy::Latest, None),
        };

        let mut fumarole_client = FumaroleClient::connect(self.config.clone().into())
            .await
            .expect("failing to connect to fumarole");

        let group_result = fumarole_client
            .create_consumer_group(CreateConsumerGroupRequest {
                consumer_group_name: subscriber_name.clone(),
                initial_offset_policy: initial_offset_policy.into(),
                // If the initial offset policy is "from-slot", this is the slot to start from.
                // If not specified, the subscriber will start from the latest slot.
                from_slot,
            })
            .await;

        match group_result {
            Ok(_) => (),
            Err(status) => {
                let code = status.code();
                match code {
                    Code::AlreadyExists => {
                        tracing::warn!(
                            "Fumarole consumer group: '{:?}' already existent",
                            subscriber_name
                        )
                    },
                    _ => panic!("Failed to create consumer group: {status:?}"),
                }
            },
        }

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
            fumarole_handle: _,
        } = dragonsmouth_session;

        tasks_set.spawn(async move {
            while let Some(update) = source.recv().await {
                let res = tx.send(update).await;
                if res.is_err() {
                    tracing::error!("Failed to send update to buffer");
                }
            }
        });

        tasks_set.join_all().await;

        Ok(())
    }
}
