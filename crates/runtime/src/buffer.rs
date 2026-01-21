use std::sync::Arc;

use tokio::sync::mpsc::Receiver;
use topograph::{
    executor::{self, Executor, Nonblock, Tokio},
    prelude::*,
};
use tracing::{warn, Instrument};
use yellowstone_grpc_proto::{
    geyser::{subscribe_update::UpdateOneof, SubscribeUpdate, SubscribeUpdatePing},
    tonic::Status,
};

#[cfg(feature = "prometheus")]
use crate::metrics;
use crate::{
    config::BufferConfig,
    handler::PipelineSets,
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
        // Potential SubscribeUpdate errors are already converted to `crate::Error::YellowstoneStatus` errors
        let result = match (&mut self.0).await {
            Ok(update_result) => update_result,
            Err(e) => return Err(crate::Error::Io(std::io::Error::from(e))),
        };

        match result {
            Ok(_stop_code) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

struct Job(tracing::Span, SubscribeUpdate);

struct Handler {
    pipelines: Arc<PipelineSets>,
}
impl Clone for Handler {
    fn clone(&self) -> Self {
        let Self { pipelines } = self;
        Self {
            pipelines: Arc::clone(pipelines),
        }
    }
}
impl<H: Send> topograph::AsyncHandler<Job, H> for Handler {
    type Output = ();

    async fn handle(&self, update: Job, _: H) {
        let Self { pipelines } = self;
        let Job(
            span,
            SubscribeUpdate {
                filters,
                update_oneof,
                created_at: _,
            },
        ) = update;
        let Some(update) = update_oneof else { return };

        #[cfg(feature = "prometheus")]
        let update_type = metrics::UpdateType::from(&update);

        match update {
            UpdateOneof::Account(a) => {
                pipelines
                    .account
                    .get_handlers(&filters)
                    .run(
                        span,
                        &a,
                        #[cfg(feature = "prometheus")]
                        update_type,
                    )
                    .instrument(tracing::info_span!("vixen.process.account"))
                    .await;
            },
            UpdateOneof::Transaction(t) => {
                let transaction_fut = pipelines.transaction.get_handlers(&filters).run(
                    span.clone(),
                    &t,
                    #[cfg(feature = "prometheus")]
                    update_type,
                );

                let instruction_fut = pipelines.instruction.get_handlers(&filters).run(
                    span,
                    &t,
                    #[cfg(feature = "prometheus")]
                    update_type,
                );

                futures_util::future::join_all([transaction_fut, instruction_fut])
                    .instrument(tracing::info_span!("vixen.process.transaction"))
                    .await;
            },
            UpdateOneof::BlockMeta(b) => {
                pipelines
                    .block_meta
                    .get_handlers(&filters)
                    .run(
                        span,
                        &b,
                        #[cfg(feature = "prometheus")]
                        update_type,
                    )
                    .instrument(tracing::info_span!("vixen.process.block_meta"))
                    .await;
            },
            UpdateOneof::Block(b) => {
                pipelines
                    .block
                    .get_handlers(&filters)
                    .run(
                        span,
                        &b,
                        #[cfg(feature = "prometheus")]
                        update_type,
                    )
                    .instrument(tracing::info_span!("vixen.process.block"))
                    .await;
            },
            UpdateOneof::Slot(s) => {
                pipelines
                    .slot
                    .get_handlers(&filters)
                    .run(
                        span,
                        &s,
                        #[cfg(feature = "prometheus")]
                        update_type,
                    )
                    .instrument(tracing::info_span!("vixen.process.slot"))
                    .await;
            },
            UpdateOneof::Ping(SubscribeUpdatePing {}) => (),
            var => warn!(?var, "Unknown update variant"),
        }
    }
}

impl Buffer {
    fn dispatch<E: ExecutorHandle<Job>>(exec: &E, update: SubscribeUpdate) {
        let span = tracing::trace_span!("process_update", ?update).entered();

        #[cfg(feature = "prometheus")]
        if let Some(update_oneof) = update.update_oneof.as_ref() {
            let update_type = metrics::UpdateType::from(update_oneof);
            metrics::increment_received_updates(update_type);
        }

        exec.push(Job(span.exit(), update));
    }

    fn run_impl<
        B: FnOnce(executor::Builder<Job, Nonblock<Tokio>>) -> executor::Builder<Job, Nonblock<Tokio>>,
        S: FnOnce(Executor<Job, Nonblock<Tokio>>, StopRx) -> TaskHandle,
    >(
        config: BufferConfig,
        pipelines: PipelineSets,
        build: B,
        spawn: S,
    ) -> Self {
        let BufferConfig {
            jobs,
            sources_channel_size: _,
        } = config;

        let pipelines = Arc::new(pipelines);

        let exec = build(Executor::builder(Nonblock(Tokio)).max_concurrency(jobs))
            .build_async(Handler { pipelines })
            .unwrap_or_else(|i| match i {});

        let (stop_tx, rx) = stop::channel();

        let task = spawn(exec, rx);
        Self(task, stop_tx)
    }

    #[allow(clippy::large_enum_variant)]
    pub fn run_yellowstone(
        config: BufferConfig,
        mut stream: Receiver<Result<SubscribeUpdate, Status>>,
        pipelines: PipelineSets,
    ) -> Self {
        Self::run_impl(
            config,
            pipelines,
            std::convert::identity,
            |exec, mut stop_rx| {
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
                            Event::Update(Some(u)) => match u {
                                Ok(u) => u,
                                Err(e) => {
                                    tracing::error!(
                                        code = ?e.code(),
                                        message = %e.message(),
                                        "Yellowstone grpc stream error"
                                    );
                                    return Err(crate::Error::YellowstoneStatus(e));
                                },
                            },
                            Event::Update(None) => {
                                tracing::warn!("Server stopped sending updates");
                                break Ok(StopCode::default());
                            },
                            Event::Stop(c) => break Ok(c),
                        };

                        Self::dispatch(&exec, update);
                    }
                })
            },
        )
    }
}
