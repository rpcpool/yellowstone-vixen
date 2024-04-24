use std::pin::pin;

use futures_util::{Stream, StreamExt};
use topograph::{
    executor::{Executor, Nonblock, Tokio},
    prelude::*,
};
use yellowstone_grpc_proto::{geyser::SubscribeUpdate, tonic::Status};

use crate::yellowstone;

#[derive(Default, Debug, clap::Args, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BufferOpts {
    pub jobs: Option<usize>,
}

pub struct Buffer(());

pub fn run_yellowstone<I, T, S: Stream<Item = Result<SubscribeUpdate, Status>> + 'static>(
    opts: BufferOpts,
    client: yellowstone::YellowstoneStream<I, T, S>,
) -> Buffer {
    let BufferOpts { jobs } = opts;

    let exec = Executor::builder(Nonblock(Tokio))
        .num_threads(jobs)
        .build(|j, h| async move {
            tracing::info!(?j);
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
