use std::{pin::pin, sync::Arc};

use futures_util::{Stream, StreamExt};
use topograph::{
    executor::{Executor, Nonblock, Tokio},
    prelude::*,
};
use tracing::{warn, Instrument};
use yellowstone_grpc_proto::{
    geyser::{subscribe_update::UpdateOneof, SubscribeUpdate, SubscribeUpdatePing},
    tonic::Status,
};
use yellowstone_vixen_core::UpdateType;

use crate::{
    config::BufferConfig,
    metrics::{Counters, Instrumenter},
    stop::{self, StopCode, StopTx},
    yellowstone, PipelineSets,
};

pub struct Buffer(
    tokio::task::JoinHandle<Result<StopCode, crate::Error>>,
    StopTx,
);

impl Buffer {
    pub async fn join(self) -> Result<StopCode, crate::Error> {
        self.1.maybe_send();
        self.0
            .await
            .map_err(|e| std::io::Error::from(e).into())
            .and_then(std::convert::identity)
    }

    // TODO: use never
    pub async fn wait_for_stop(&mut self) -> Result<std::convert::Infallible, crate::Error> {
        (&mut self.0)
            .await
            .map_err(|e| std::io::Error::from(e).into())
            .and_then(|r| r.and(Err(crate::Error::ClientHangup)))
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
            UpdateOneof::Ping(SubscribeUpdatePing {}) => (),
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

    let (stop_tx, mut stop_rx) = stop::channel();

    let task = tokio::task::spawn_local(async move {
        enum Event {
            Update(Option<Result<SubscribeUpdate, Status>>),
            Stop(StopCode),
        }

        let mut stream = pin!(client.stream);
        loop {
            let event = tokio::select! {
                u = stream
                        .next()
                        .instrument(tracing::trace_span!("await_update"))
                    => Event::Update(u),
                c = &mut stop_rx => Event::Stop(c),
            };

            let update = match event {
                Event::Update(Some(u)) => u,
                Event::Update(None) => break Err(crate::Error::ServerHangup),
                Event::Stop(c) => break Ok(c),
            };

            let span = tracing::trace_span!("process_update", ?update).entered();
            match update {
                Ok(u) => {
                    if let Some(ty) = UpdateType::get(&u.update_oneof) {
                        counters.inc_received(ty);
                    }
                    exec.push(Job(span.exit(), u));
                },
                Err(e) => break Err(e.into()),
            }
        }
    });

    Buffer(task, stop_tx)
}
