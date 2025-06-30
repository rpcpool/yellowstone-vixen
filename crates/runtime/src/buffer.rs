use std::sync::Arc;

use tokio::sync::mpsc::Receiver;
use topograph::{
    executor::{self, Executor, Nonblock, Tokio},
    prelude::*,
};
use tracing::warn;
use yellowstone_grpc_proto::{
    geyser::{subscribe_update::UpdateOneof, SubscribeUpdate, SubscribeUpdatePing},
    tonic::Status,
};

use crate::{
    config::BufferConfig,
    handler::PipelineSets,
    metrics::{Counters, Instrumenter, UpdateType},
    stop::{self, StopCode, StopRx, StopTx},
};

type TaskHandle = tokio::task::JoinHandle<Result<StopCode, crate::Error>>;
pub struct Buffer(TaskHandle, StopTx);

impl Buffer {
    pub async fn join(self) -> Result<StopCode, crate::Error> {
        self.1.maybe_send();
        self.0
            .await
            .map_err(|e| std::io::Error::from(e).into())
            .and_then(std::convert::identity)
    }

    pub async fn wait_for_stop(&mut self) -> Result<(), crate::Error> {
        (&mut self.0)
            .await
            .map_err(|e| std::io::Error::from(e).into())
            .map(|_| ())
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
                created_at: _,
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
            UpdateOneof::BlockMeta(b) => {
                pipelines
                    .block_meta
                    .get_handlers(&filters)
                    .run(span, &b, counters)
                    .await;
            },
            UpdateOneof::Ping(SubscribeUpdatePing {}) => (),
            var => warn!(?var, "Unknown update variant"),
        }
    }
}

impl Buffer {
    fn dispatch<M: Instrumenter, E: ExecutorHandle<Job>>(
        exec: &E,
        update: SubscribeUpdate,
        counters: &Counters<M>,
    ) {
        let span = tracing::trace_span!("process_update", ?update).entered();
        if let Some(ty) = UpdateType::get(update.update_oneof.as_ref()) {
            counters.inc_received(ty);
        }
        exec.push(Job(span.exit(), update));
    }

    fn run_impl<
        M: Instrumenter,
        B: FnOnce(executor::Builder<Job, Nonblock<Tokio>>) -> executor::Builder<Job, Nonblock<Tokio>>,
        S: FnOnce(Executor<Job, Nonblock<Tokio>>, StopRx, Arc<Counters<M>>) -> TaskHandle,
    >(
        config: BufferConfig,
        pipelines: PipelineSets,
        counters: Counters<M>,
        build: B,
        spawn: S,
    ) -> Self {
        let BufferConfig {
            jobs,
            sources_channel_size: _,
        } = config;

        let pipelines = Arc::new(pipelines);
        let counters = Arc::new(counters);
        let exec = build(Executor::builder(Nonblock(Tokio)).max_concurrency(jobs))
            .build_async(Handler {
                pipelines,
                counters: Arc::clone(&counters),
            })
            .unwrap_or_else(|i| match i {});

        let (stop_tx, rx) = stop::channel();

        let task = spawn(exec, rx, counters);
        Self(task, stop_tx)
    }

    pub fn run_yellowstone<M: Instrumenter>(
        config: BufferConfig,
        mut stream: Receiver<Result<SubscribeUpdate, Status>>,
        pipelines: PipelineSets,
        counters: Counters<M>,
    ) -> Self {
        Self::run_impl(
            config,
            pipelines,
            counters,
            std::convert::identity,
            |exec, mut stop_rx, counters| {
                tokio::task::spawn(async move {
                    enum Event {
                        Update(Option<Result<SubscribeUpdate, Status>>),
                        Stop(StopCode),
                    }

                    loop {
                        let event = tokio::select! {
                            u = stream.recv() => Event::Update(u),
                            c = &mut stop_rx => Event::Stop(c),
                        };

                        let update = match event {
                            Event::Update(Some(u)) => u,
                            Event::Update(None) => {
                                tracing::warn!("Server stopped sending updates");
                                break Ok(StopCode::default());
                            },
                            Event::Stop(c) => break Ok(c),
                        };

                        Self::dispatch(&exec, update?, &counters);
                    }
                })
            },
        )
    }
}
