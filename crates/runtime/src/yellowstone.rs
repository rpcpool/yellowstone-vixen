use std::time::Duration;

use futures_channel::mpsc::SendError;
use futures_util::{Sink, Stream};
use yellowstone_grpc_client::{GeyserGrpcClient, Interceptor};
use yellowstone_grpc_proto::{prelude::*, tonic::Status};

use crate::{
    parser_manager::{Filters, ParserManager},
    Parser,
};

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

pub async fn connect<P: Parser>(
    opts: YellowstoneOpts,
    manager: &ParserManager<P>,
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

    let filters = manager.filters();

    // TODO: where are the docs on this stuff?
    let req = SubscribeRequest {
        accounts: filters
            .iter()
            .filter_map(|(k, v)| {
                let v = v.account.as_ref()?;

                Some((k.to_owned().into(), SubscribeRequestFilterAccounts {
                    account: v.accounts.iter().map(ToString::to_string).collect(),
                    owner: v.owners.iter().map(ToString::to_string).collect(),
                    // TODO: probably a good thing to look into
                    filters: vec![],
                }))
            })
            .collect(),
        slots: [].into_iter().collect(),
        transactions: filters
            .iter()
            .filter_map(|(k, v)| {
                let v = v.transaction.as_ref()?;

                Some((k.to_owned().into(), SubscribeRequestFilterTransactions {
                    vote: None,
                    // TODO: make this configurable
                    failed: Some(false),
                    signature: None,
                    // TODO: figure these out
                    account_include: v.accounts.iter().map(ToString::to_string).collect(),
                    account_exclude: [].into_iter().collect(),
                    account_required: [].into_iter().collect(),
                }))
            })
            .collect(),
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
