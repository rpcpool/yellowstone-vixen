use std::{collections::BTreeMap, num::NonZero};

use async_trait::async_trait;
use bytesize::ByteSize;
use tokio::{sync::mpsc::Sender, task::JoinSet};
use tonic::{codec::CompressionEncoding, Code};
use yellowstone_fumarole_client::{
    proto::{CreateConsumerGroupRequest, InitialOffsetPolicy},
    DragonsmouthAdapterSession, FumaroleClient, FumaroleSubscribeConfig,
};
use yellowstone_grpc_proto::{geyser::SubscribeUpdate, tonic::Status};
use yellowstone_vixen::{sources::SourceTrait, Error as VixenError};
use yellowstone_vixen_core::Filters;

/// A `Source` implementation for the Yellowstone gRPC API.
#[derive(Debug)]
pub struct YellowstoneFumaroleSource {
    filters: Filters,
    config: FumaroleConfig,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
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

    fn name() -> String { "fumarole".to_string() }

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

        let (initial_offset_policy, from_slot) =
            if let Some(from_slot) = filters.global_filters.from_slot {
                (InitialOffsetPolicy::FromSlot, Some(from_slot))
            } else {
                (InitialOffsetPolicy::Latest, None)
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

        let dragonsmouth_session = fumarole_client
            .dragonsmouth_subscribe_with_config(
                subscriber_name,
                filters.into(),
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
