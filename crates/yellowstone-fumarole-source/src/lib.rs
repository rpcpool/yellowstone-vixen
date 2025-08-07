use std::{collections::BTreeMap, num::NonZero};

use async_trait::async_trait;
use bytesize::ByteSize;
#[cfg(feature = "prometheus")]
use tokio::task::JoinHandle;
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
use yellowstone_vixen::{sources::Source, Error as VixenError};
use yellowstone_vixen_core::Filters;

/// A `Source` implementation for the Yellowstone gRPC API.
#[derive(Debug, Default)]
pub struct YellowstoneFumaroleSource {
    filters: Option<Filters>,
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

impl YellowstoneFumaroleSource {
    /// Create a new `YellowstoneFumaroleSource` with default values.
    #[must_use]
    pub fn new() -> Self { Default::default() }

    #[cfg(feature = "prometheus")]
    fn register_metrics(&self, config: &FumaroleConfig) -> JoinHandle<()> {
        use std::time::Duration;

        let prometheus_registry = prometheus::Registry::new();
        yellowstone_fumarole_client::metrics::register_metrics(&prometheus_registry);

        // Use the conditional config fields
        let export_interval = Duration::from_secs(config.metrics_interval);
        let job_name = config.metrics_job_name.clone();
        let metrics_endpoint = config.metrics_endpoint.clone();

        // Spawn metrics pusher task
        let registry_clone = prometheus_registry.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(export_interval);
            loop {
                interval.tick().await;

                let metrics = registry_clone.gather();
                let job_name = job_name.clone();
                let metrics_endpoint = metrics_endpoint.clone();
                let _ = tokio::task::spawn_blocking(move || {
                    if let Err(e) = prometheus::push_metrics(
                        &job_name,
                        prometheus::labels! {},
                        &metrics_endpoint,
                        metrics,
                        None,
                    ) {
                        tracing::error!("Failed to push Fumarole metrics: {e:?}");
                    }
                })
                .await;
            }
        })
    }
}

#[async_trait]
impl Source for YellowstoneFumaroleSource {
    fn name(&self) -> String { "fumarole".to_string() }

    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, Status>>,
        raw_config: toml::Value,
    ) -> Result<(), VixenError> {
        let config: FumaroleConfig = serde::Deserialize::deserialize(raw_config)
            .expect("Failed to deserialize FumaroleConfig");
        let filters = self.filters.clone().ok_or(VixenError::ConfigError)?;
        let subscriber_name = config.subscriber_name.clone();

        #[cfg(feature = "prometheus")]
        self.register_metrics(&config);

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

    fn get_filters(&self) -> &Option<Filters> { &self.filters }
}
