use std::{pin::pin, sync::Arc};

use futures_util::{Stream, StreamExt};
use tokio::sync::oneshot;
use topograph::{
    executor::{Executor, Nonblock, Tokio},
    prelude::*,
};
use tracing::{warn, Instrument};
use yellowstone_grpc_proto::{
    geyser::{subscribe_update::UpdateOneof, SubscribeUpdate},
    tonic::Status,
};
use yellowstone_vixen_core::UpdateType;

use crate::{
    config::BufferConfig,
    metrics::{Counters, Instrumenter},
    yellowstone, PipelineSets,
};

pub struct Buffer(oneshot::Receiver<crate::Error>);

impl Buffer {
    // TODO: use never
    #[inline]
    pub async fn wait_for_stop(self) -> Result<std::convert::Infallible, crate::Error> {
        self.0
            .await
            .map_err(|_| crate::Error::ClientHangup)
            .and_then(Err)
    }
}

struct Job(tracing::Span, SubscribeUpdate);

struct Handler<M: Instrumenter> {
    pipelines: Arc<PipelineSets>,
    counters: Arc<Counters<M>>,
}
impl<M: Instrumenter> Clone for Handler<M> {
    fn clone(&self) -> Self {
        let Self {
            pipelines,
            counters,
        } = self;
        Self {
            pipelines: Arc::clone(pipelines),
            counters: Arc::clone(counters),
        }
    }
}
impl<M: Instrumenter, H: Send> topograph::AsyncHandler<Job, H> for Handler<M> {
    type Output = ();

    async fn handle(&self, update: Job, _: H) {
        let Self {
            pipelines,
            counters,
        } = self;
        let Job(
            span,
            SubscribeUpdate {
                filters,
                update_oneof,
            },
        ) = update;
        let Some(update) = update_oneof else { return };

        match update {
            UpdateOneof::Account(a) => {
                pipelines
                    .account
                    .get_handlers(&filters)
                    .run(span, &a, counters)
                    .await;
            },
            UpdateOneof::Transaction(t) => {
                pipelines
                    .transaction
                    .get_handlers(&filters)
                    .run(span, &t, counters)
                    .await;
            },
            var => warn!(?var, "Unknown update variant"),
        }
    }
}

pub fn run_yellowstone<
    I,
    T,
    S: Stream<Item = Result<SubscribeUpdate, Status>> + 'static,
    M: Instrumenter,
>(
    config: BufferConfig,
    client: yellowstone::YellowstoneStream<I, T, S>,
    pipelines: PipelineSets,
    counters: Counters<M>,
) -> Buffer {
    let BufferConfig { jobs } = config;

    let pipelines = Arc::new(pipelines);
    let counters = Arc::new(counters);
    let exec = Executor::builder(Nonblock(Tokio))
        .max_concurrency(jobs)
        .build_async(Handler {
            pipelines,
            counters: Arc::clone(&counters),
        })
        .unwrap_or_else(|i| match i {});

    let (tx, rx) = oneshot::channel();

    tokio::task::spawn_local(async move {
        let mut stream = pin!(client.stream);
        while let Some(update) = stream
            .next()
            .instrument(tracing::trace_span!("await_update"))
            .await
        {
            let span = tracing::trace_span!("process_update", ?update).entered();
            match update {
                Ok(u) => {
                    if let Some(ty) = UpdateType::get(&u.update_oneof) {
                        counters.inc_received(ty);
                    }
                    exec.push(Job(span.exit(), u));
                },
                Err(e) => {
                    tx.send(e.into()).unwrap_or_else(|err| {
                        warn!(%err, "Yellowstone stream returned an error after stop requested");
                    });
                    return;
                },
            }
        }

        tx.send(crate::Error::ServerHangup).unwrap_or_else(|_| {
            warn!("Yellowstone client and server both hung up");
        });
    });

    Buffer(rx)
}
