use std::{pin::pin, sync::Arc};

use futures_util::{Stream, StreamExt};
use topograph::{
    executor::{Executor, Nonblock, Tokio},
    prelude::*,
};
use tracing::{error, warn};
use yellowstone_grpc_proto::{
    geyser::{subscribe_update::UpdateOneof, SubscribeUpdate},
    tonic::Status,
};
use yellowstone_vixen_core::{AccountUpdate, TransactionUpdate};

use crate::{handler::DynHandlerPack, yellowstone, HandlerManagers};

#[derive(Default, Debug, clap::Args, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BufferOpts {
    pub jobs: Option<usize>,
}

pub struct Buffer(());

pub fn run_yellowstone<
    I,
    T,
    S: Stream<Item = Result<SubscribeUpdate, Status>> + 'static,
    A: DynHandlerPack<AccountUpdate> + Send + Sync + 'static,
    X: DynHandlerPack<TransactionUpdate> + Send + Sync + 'static,
>(
    opts: BufferOpts,
    client: yellowstone::YellowstoneStream<I, T, S>,
    manager: HandlerManagers<A, X>,
) -> Buffer {
    let BufferOpts { jobs } = opts;

    let manager = Arc::new(manager);
    let exec = Executor::builder(Nonblock(Tokio))
        .num_threads(jobs)
        .build(move |update, _| {
            let manager = Arc::clone(&manager);
            async move {
                let SubscribeUpdate {
                    filters,
                    update_oneof,
                } = update;
                let Some(update) = update_oneof else { return };

                match update {
                    UpdateOneof::Account(a) => manager.account.get_handlers(&filters).run(&a).await,
                    UpdateOneof::Transaction(t) => {
                        manager.transaction.get_handlers(&filters).run(&t).await
                    },
                    var => warn!(?var, "Unknown update variant"),
                }
            }
        })
        .unwrap_or_else(|i| match i {});

    tokio::task::spawn_local(async move {
        let mut stream = pin!(client.stream);
        while let Some(update) = stream.next().await {
            match update {
                Ok(u) => exec.push(u),
                Err(e) => todo!("{e}"),
            }
        }

        todo!("warn when the stream hangs up");
    });

    Buffer(())
}
