use std::{collections::HashMap, time::Duration};

use async_trait::async_trait;
use futures_util::StreamExt;
use tokio::{sync::mpsc::Sender, task::JoinSet};
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::{
    geyser::SubscribeUpdate,
    tonic::{transport::ClientTlsConfig, Status},
};
use yellowstone_vixen::{config::YellowstoneConfig, sources::SourceTrait, Error as VixenError};
use yellowstone_vixen_core::Filters;

/// A `Source` implementation for the Yellowstone gRPC API.
#[derive(Debug)]
pub struct YellowstoneGrpcSource;

#[async_trait]
impl SourceTrait for YellowstoneGrpcSource {
    type Config = YellowstoneConfig;

    fn name() -> String { "yellowstone-grpc".to_string() }

    async fn connect(
        config: Self::Config,
        filters: Filters,
        tx: Sender<Result<SubscribeUpdate, Status>>,
    ) -> Result<(), VixenError> {
        let timeout = Duration::from_secs(config.timeout);

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

        tasks_set.join_all().await;

        Ok(())
    }
}
