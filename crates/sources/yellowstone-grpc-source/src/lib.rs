use std::{collections::HashMap, time::Duration};

use async_trait::async_trait;
use futures_util::StreamExt;
use tokio::{sync::mpsc::Sender, task::JoinSet};
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::{
    geyser::SubscribeUpdate,
    tonic::{transport::ClientTlsConfig, Status},
};
use yellowstone_vixen::{config::YellowstoneConfig, sources::Source, Error as VixenError};
use yellowstone_vixen_core::Filters;

/// A `Source` implementation for the Yellowstone gRPC API.
#[derive(Debug, Default)]
pub struct YellowstoneGrpcSource {
    config: Option<YellowstoneConfig>,
    filters: Option<Filters>,
}

impl YellowstoneGrpcSource {
    /// Create a new `YellowstoneGrpcSource` with default values.
    #[must_use]
    pub fn new() -> Self { Self::default() }
}

#[async_trait]
impl Source for YellowstoneGrpcSource {
    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, Status>>,
    ) -> Result<JoinSet<()>, VixenError> {
        // We require that config and filters are set before connecting to the `Source`
        let filters = self.filters.clone().ok_or(VixenError::ConfigError)?;
        let config = self.config.clone().ok_or(VixenError::ConfigError)?;

        let timeout = Duration::from_secs(config.timeout);

        // TODO: add tasks pool concurrency limit through config
        let mut tasks_set = JoinSet::new();

        for (filter_id, prefilter) in filters.parsers_filters {
            let mut filter = Filters::new(HashMap::from([(filter_id, prefilter)]));
            filter.global_filters = filters.global_filters;
            let config = config.clone();
            let tx = tx.clone();

            let mut client = GeyserGrpcClient::build_from_shared(config.endpoint)?
                .x_token(config.x_token)?
                .connect_timeout(timeout)
                .timeout(timeout)
                .tls_config(ClientTlsConfig::new().with_native_roots())?
                .connect()
                .await?;

            let (_sub_tx, stream) = client.subscribe_with_request(Some(filter.into())).await?;

            tasks_set.spawn(async move {
                let mut stream = std::pin::pin!(stream);

                while let Some(update) = stream.next().await {
                    let res = tx.send(update).await;
                    if res.is_err() {
                        tracing::error!("Failed to send update to buffer");
                    }
                }
            });
        }

        Ok(tasks_set)
    }

    fn set_filters_unchecked(&mut self, filters: Filters) { self.filters = Some(filters); }

    fn set_config_unchecked(&mut self, config: YellowstoneConfig) { self.config = Some(config); }

    fn get_filters(&self) -> &Option<Filters> { &self.filters }

    fn get_config(&self) -> Option<YellowstoneConfig> { self.config.clone() }
}
