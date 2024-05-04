use std::{panic::AssertUnwindSafe, pin::pin, sync::Arc};

use futures_util::{Stream, StreamExt};
use topograph::{
    executor::{Executor, Nonblock, Tokio},
    prelude::*,
};
use tracing::error;
use yellowstone_grpc_proto::{geyser::SubscribeUpdate, tonic::Status};

use crate::{parser::Message, parser_manager::ParserManager, yellowstone, Parser};

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
    P: Parser + Send + Sync + 'static,
>(
    opts: BufferOpts,
    client: yellowstone::YellowstoneStream<I, T, S>,
    manager: ParserManager<P>,
) -> Buffer {
    let BufferOpts { jobs } = opts;

    let manager = Arc::new(manager);
    let exec = Executor::builder(Nonblock(Tokio))
        .num_threads(jobs)
        .build(move |mut update: SubscribeUpdate, handle| {
            let manager = Arc::clone(&manager);
            async move {
                let filters = std::mem::take(&mut update.filters);
                let parsers = manager.get_parsers(&filters);
                let msg = match Message::try_from(update) {
                    Ok(m) => m,
                    Err(err) => {
                        error!(%err, "Received unparseable message");
                        return;
                    },
                };

                match msg {
                    Message::AccountUpdate(ref a) => parsers.run_account(a).await,
                    Message::TransactionUpdate(ref t) => parsers.run_transaction(t).await,
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
