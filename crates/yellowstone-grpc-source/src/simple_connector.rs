use std::{collections::HashMap, time::Duration};

use async_trait::async_trait;
use clap::ValueEnum;
use futures_util::StreamExt;
use tokio::{sync::mpsc::Sender, task::JoinSet};
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::{
    geyser::{SubscribeRequest, SubscribeUpdate},
    tonic::{codec::CompressionEncoding, transport::ClientTlsConfig, Status},
};
use yellowstone_vixen::{sources::SourceTrait, CommitmentLevel, Error as VixenError};
use yellowstone_vixen_core::Filters;
use crate::{YellowstoneGrpcConfig};

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
        let filters = self.filters.clone();
        let config = self.config.clone();

        let timeout = Duration::from_secs(config.timeout);

        let mut tasks_set = JoinSet::new();

        for (filter_id, prefilter) in filters.parsers_filters {
            let filter = Filters::new(HashMap::from([(filter_id, prefilter)]));

            let tx = tx.clone();

            let mut client = GeyserGrpcClient::build_from_shared(config.endpoint.clone())?
                .x_token(config.x_token.clone())?
                .max_decoding_message_size(config.max_decoding_message_size.unwrap_or(usize::MAX))
                .accept_compressed(config.accept_compression.unwrap_or_default().into())
                .connect_timeout(timeout)
                .timeout(timeout)
                .tls_config(ClientTlsConfig::new().with_native_roots())?
                .connect()
                .await?;

            let mut subscribe_request: SubscribeRequest = filter.into();
            if let Some(from_slot) = config.from_slot {
                subscribe_request.from_slot = Some(from_slot);
            }
            if let Some(commitment_level) = config.commitment_level {
                subscribe_request.commitment = Some(commitment_level as i32);
            }

            let (_sub_tx, stream) = client
                .subscribe_with_request(Some(subscribe_request))
                .await?;

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
