use std::{collections::HashMap, time::Duration};

use async_trait::async_trait;
use tokio::{sync::mpsc::Sender, task::JoinSet};
use tokio_util::sync::CancellationToken;
use yellowstone_grpc_proto::{
    geyser::{SubscribeRequest, SubscribeUpdate},
    tonic::{transport::ClientTlsConfig, Status},
};
use yellowstone_vixen::{sources::SourceTrait, Error as VixenError};
use yellowstone_vixen_core::Filters;
use crate::autoreconnect::grpc_autoreconnect_task;
use crate::autoreconnect::grpc_autoreconnect_task::{GrpcConnectionTimeouts, GrpcSourceConfig};
use crate::YellowstoneGrpcConfig;

/// A `Source` implementation for the Yellowstone gRPC API.
#[derive(Debug)]
pub(crate) struct YellowstoneGrpcAutoconnectSource {
    filters: Filters,
    config: YellowstoneGrpcConfig,
}

#[async_trait]
impl SourceTrait for YellowstoneGrpcAutoconnectSource {
    type Config = YellowstoneGrpcConfig;

    fn new(config: Self::Config, filters: Filters) -> Self { Self { config, filters } }

    async fn connect(&self, tx: Sender<Result<SubscribeUpdate, Status>>) -> Result<(), VixenError> {

        let shutdown_token = CancellationToken::new();
        let filters = self.filters.clone();
        let config = self.config.clone();

        let timeout = Duration::from_secs(config.timeout);

        let mut tasks_set = JoinSet::new();

        for (filter_id, prefilter) in filters.parsers_filters {
            let filter = Filters::new(HashMap::from([(filter_id, prefilter)]));

            let tx = tx.clone();

            let mut subscribe_request: SubscribeRequest = filter.into();
            if let Some(from_slot) = config.from_slot {
                subscribe_request.from_slot = Some(from_slot);
            }
            if let Some(commitment_level) = config.commitment_level {
                subscribe_request.commitment = Some(commitment_level as i32);
            }

            let grpc_source = GrpcSourceConfig {
                grpc_addr: config.endpoint.clone(),
                grpc_x_token: config.x_token.clone(),
                tls_config: Some(ClientTlsConfig::new().with_native_roots()),
                max_decoding_message_size: config.max_decoding_message_size.unwrap_or(usize::MAX),
                timeouts: Some(GrpcConnectionTimeouts {
                    connect_timeout: timeout,
                    request_timeout: timeout,
                    subscribe_timeout: timeout,
                    receive_timeout: timeout,
                }),
                compression: Some(config.accept_compression.unwrap_or_default().into()),
            };
            let connect_task = grpc_autoreconnect_task::create_geyser_autoconnection_task_with_mpsc(
                grpc_source, subscribe_request,
                tx, shutdown_token.clone());
            tasks_set.spawn(connect_task);

        }

        tasks_set.join_all().await;

        Ok(())
    }
}
