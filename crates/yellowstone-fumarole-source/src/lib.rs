use std::collections::{BTreeMap, HashMap};

use async_trait::async_trait;
use bytesize::ByteSize;
use tokio::{sync::mpsc::Sender, task::JoinSet};
use yellowstone_fumarole_client::{DragonsmouthAdapterSession, FumaroleClient};
use yellowstone_grpc_proto::{geyser::SubscribeUpdate, tonic::Status};
use yellowstone_vixen::{config::YellowstoneConfig, sources::Source, Error as VixenError};
use yellowstone_vixen_core::Filters;

/// A `Source` implementation for the Yellowstone gRPC API.
#[derive(Debug, Default)]
pub struct YellowstoneFumaroleSource {
    config: Option<FumaroleConfig>,
    filters: Option<Filters>,
}

#[derive(Debug, Clone)]
pub struct FumaroleConfig {
    base: YellowstoneConfig,
    /// Name of the persistent subscriber to use
    subscriber_name: String,
}

impl From<FumaroleConfig> for yellowstone_fumarole_client::config::FumaroleConfig {
    fn from(config: FumaroleConfig) -> Self {
        yellowstone_fumarole_client::config::FumaroleConfig {
            endpoint: config.base.endpoint,
            x_token: config.base.x_token,
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
    pub fn new() -> Self { Self::default() }

    pub fn config(mut self, config: FumaroleConfig) -> Self {
        self.config = Some(config);
        self
    }
}

#[async_trait]
impl Source for YellowstoneFumaroleSource {
    fn name(&self) -> String { "yellowstone-fumarole".to_string() }

    async fn connect(&self, tx: Sender<Result<SubscribeUpdate, Status>>) -> Result<(), VixenError> {
        // We require that config and filters are set before connecting to the `Source`
        let filters = self.filters.clone().ok_or(VixenError::ConfigError)?;
        let config = self.config.clone().ok_or(VixenError::ConfigError)?;

        // TODO: add tasks pool concurrency limit through config
        let mut tasks_set = JoinSet::new();

        for (filter_id, prefilter) in filters.parsers_filters {
            let mut filter = Filters::new(HashMap::from([(filter_id, prefilter)]));
            filter.global_filters = filters.global_filters;
            let config = config.clone();
            let subscriber_name = config.subscriber_name.clone();
            let tx = tx.clone();

            let mut fumarole_client = FumaroleClient::connect(config.into())
                .await
                .map_err(|_| VixenError::ConfigError)?;

            let dragonsmouth_session = fumarole_client
                .dragonsmouth_subscribe(subscriber_name, filter.into())
                .await?;

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
        }

        tasks_set.join_all().await;

        Ok(())
    }

    fn set_filters_unchecked(&mut self, filters: Filters) { self.filters = Some(filters); }

    fn set_config_unchecked(&mut self, config: YellowstoneConfig) {
        self.config = Some(FumaroleConfig {
            base: config,
            subscriber_name: "default_subscriber".to_string(),
        });
    }

    fn get_filters(&self) -> &Option<Filters> { &self.filters }

    fn get_config(&self) -> Option<YellowstoneConfig> {
        self.config.as_ref().map(|config| config.base.clone())
    }
}
