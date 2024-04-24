use std::time::Duration;

use futures_channel::mpsc::SendError;
use futures_util::{Sink, Stream};
use yellowstone_grpc_client::{GeyserGrpcClient, Interceptor};
use yellowstone_grpc_proto::{prelude::*, tonic::Status};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Yellowstone client builder error")]
    Builder(#[from] yellowstone_grpc_client::GeyserGrpcBuilderError),
    #[error("Yellowstone client error")]
    Client(#[from] yellowstone_grpc_client::GeyserGrpcClientError),
}

#[derive(Debug, clap::Args, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct YellowstoneOpts {
    #[clap(long, env)]
    pub endpoint: String,
    #[clap(long, env)]
    pub x_token: Option<String>,
    #[clap(long, env)]
    pub timeout: u64,
}

pub struct YellowstoneStream<I, T, S> {
    client: GeyserGrpcClient<I>,
    sub_tx: T,
    pub(super) stream: S,
}

pub async fn connect(
    opts: YellowstoneOpts,
) -> Result<
    YellowstoneStream<
        impl Interceptor,
        impl Sink<SubscribeRequest, Error = SendError>,
        impl Stream<Item = Result<SubscribeUpdate, Status>> + 'static,
    >,
    Error,
> {
    let YellowstoneOpts {
        endpoint,
        x_token,
        timeout,
    } = opts;
    let timeout = Duration::from_secs(timeout);

    let req = SubscribeRequest {
        accounts: [].into_iter().collect(),
        slots: [].into_iter().collect(),
        transactions: [].into_iter().collect(),
        transactions_status: [].into_iter().collect(),
        blocks: [].into_iter().collect(),
        blocks_meta: [].into_iter().collect(),
        entry: [].into_iter().collect(),
        commitment: None,
        accounts_data_slice: vec![],
        ping: None,
    };

    let mut client = GeyserGrpcClient::build_from_shared(endpoint)?
        .x_token(x_token)?
        .connect_timeout(timeout)
        .timeout(timeout)
        .connect()
        .await?;

    let (sub_tx, stream) = client.subscribe_with_request(Some(req)).await?;

    Ok(YellowstoneStream {
        client,
        sub_tx,
        stream,
    })
}
