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
    geyser::SubscribeUpdate,
    tonic::{Code, Status},
};
use yellowstone_vixen::{config::YellowstoneConfig, sources::Source, Error as VixenError};
use yellowstone_vixen_core::Filters;

/// A `Source` implementation for the Yellowstone gRPC API.
#[derive(Debug, Default)]
pub struct YellowstoneFumaroleSource {
    config: FumaroleConfig,
    filters: Option<Filters>,
}

#[derive(Debug, Clone, Default)]
pub struct FumaroleConfig {
    base: Option<YellowstoneConfig>,
    /// Name of the persistent subscriber to use
    subscriber_name: String,
}

impl From<FumaroleConfig> for yellowstone_fumarole_client::config::FumaroleConfig {
    fn from(config: FumaroleConfig) -> Self {
        let base_config = config.base.expect("FumaroleConfig.base is required");

        yellowstone_fumarole_client::config::FumaroleConfig {
            endpoint: base_config.endpoint,
            x_token: base_config.x_token,
            max_decoding_message_size_bytes: 512_000_000,
            x_metadata: BTreeMap::new(),
            response_compression: None,
            request_compression: None,
            initial_connection_window_size: ByteSize::mb(100),
            initial_stream_window_size: ByteSize::mib(9),
            enable_http2_adaptive_window: true,
        }
    }
}

impl YellowstoneFumaroleSource {
    /// Create a new `YellowstoneFumaroleSource` with default values.
    #[must_use]
    pub fn new(subscriber_name: &str) -> Self {
        Self {
            config: FumaroleConfig {
                subscriber_name: subscriber_name.to_string(),
                base: None,
            },
            filters: None,
        }
    }
}

#[async_trait]
impl Source for YellowstoneFumaroleSource {
    fn name(&self) -> String { "yellowstone-fumarole".to_string() }

    async fn connect(&self, tx: Sender<Result<SubscribeUpdate, Status>>) -> Result<(), VixenError> {
        // We require that config and filters are set before connecting to the `Source`
        let filters = self.filters.clone().ok_or(VixenError::ConfigError)?;
        let config = self.config.clone();
        let subscriber_name = self.config.subscriber_name.clone();

        // TODO: add tasks pool concurrency limit through config
        let mut tasks_set = JoinSet::new();

        const MAX_PARA_DATA_STREAMS: u8 = 4; //Fumarole const

        let config = config.clone();
        let subscriber_name = subscriber_name.clone();
        let tx = tx.clone();
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

        let mut fumarole_client = FumaroleClient::connect(config.into())
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

    fn set_filters_unchecked(&mut self, filters: Filters) { self.filters = Some(filters); }

    fn set_config_unchecked(&mut self, config: YellowstoneConfig) {
        self.config.base = Some(config);
    }

    fn get_filters(&self) -> &Option<Filters> { &self.filters }

    fn get_config(&self) -> Option<YellowstoneConfig> { self.config.base.clone() }
}
