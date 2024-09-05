use std::time::Duration;

use futures_channel::mpsc::SendError;
use futures_util::{Sink, Stream};
use yellowstone_grpc_client::{GeyserGrpcClient, Interceptor};
use yellowstone_grpc_proto::{prelude::*, tonic::Status};
use yellowstone_vixen_core::Filters;

use crate::config::YellowstoneConfig;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Yellowstone client builder error")]
    Builder(#[from] yellowstone_grpc_client::GeyserGrpcBuilderError),
    #[error("Yellowstone client error")]
    Client(#[from] yellowstone_grpc_client::GeyserGrpcClientError),
}

pub struct YellowstoneStream<I, T, S> {
    #[allow(dead_code)]
    client: GeyserGrpcClient<I>,
    #[allow(dead_code)]
    sub_tx: T,
    pub(super) stream: S,
}

pub async fn connect(
    config: YellowstoneConfig,
    filters: Filters<'_>,
) -> Result<
    YellowstoneStream<
        impl Interceptor,
        impl Sink<SubscribeRequest, Error = SendError>,
        impl Stream<Item = Result<SubscribeUpdate, Status>> + 'static,
    >,
    Error,
> {
    let YellowstoneConfig {
        endpoint,
        x_token,
        timeout,
    } = config;
    let timeout = Duration::from_secs(timeout);

    // TODO: where are the docs on this stuff?
    let mut client = GeyserGrpcClient::build_from_shared(endpoint)?
        .x_token(x_token)?
        .connect_timeout(timeout)
        .timeout(timeout)
        .connect()
        .await?;

    let (sub_tx, stream) = client.subscribe_with_request(Some(filters.into())).await?;

    Ok(YellowstoneStream {
        client,
        sub_tx,
        stream,
    })
}
