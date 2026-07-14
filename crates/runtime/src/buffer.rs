use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tokio::sync::{mpsc::Receiver, OwnedSemaphorePermit, Semaphore};
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
    stop::{self, StopCode, StopTx},
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

/// Processes one decoded update against the pipeline sets. Spawned as its own
/// task per job by the bridge thread, so a panic isolates to that task.
async fn run_job(pipelines: &PipelineSets, job: Job) {
    let Job(
        span,
        SubscribeUpdate {
            filters,
            update_oneof,
            created_at: _,
        },
    ) = job;
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

impl Buffer {
    #[allow(clippy::large_enum_variant)]
    pub fn run_yellowstone(
        config: BufferConfig,
        mut stream: Receiver<Result<SubscribeUpdate, Status>>,
        pipelines: PipelineSets,
    ) -> Self {
        let BufferConfig {
            jobs,
            sources_channel_size: _,
        } = config;
        // Default to the CPU count. `available_parallelism` is always >= 1, so
        // `jobs` is a valid semaphore size.
        let jobs = jobs.unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(std::num::NonZeroUsize::get)
                .unwrap_or(1)
        });

        let pipelines = Arc::new(pipelines);

        // Job queue. The semaphore below is the real bound: the producer takes a
        // permit before sending, so at most `jobs` jobs are ever outstanding and
        // the queue can't exceed `jobs`. crossbeam is synchronous, so a dedicated
        // thread bridges it to tokio (below).
        //
        // Unbounded, not `bounded(jobs)`, on purpose: a bounded `send` blocks the
        // calling thread when the channel is full, and the producer is an async
        // task, so that would stall an executor worker. An unbounded send never
        // blocks.
        let (tx, rx) = crossbeam_channel::unbounded::<(OwnedSemaphorePermit, Job)>();

        // Concurrency cap: at most `jobs` handler tasks run at once.
        let sem = Arc::new(Semaphore::new(jobs));

        // When set, the bridge discards still-queued jobs instead of spawning
        // them (the abort path below).
        let abort = Arc::new(AtomicBool::new(false));

        // Bridges the sync crossbeam receiver to the async handlers: pulls each
        // `(permit, job)` and spawns the handler as its own task, so a panic
        // isolates to that task. The permit is released when the task completes.
        let rt = tokio::runtime::Handle::current();
        let bridge = {
            let pipelines = Arc::clone(&pipelines);
            let abort = Arc::clone(&abort);

            std::thread::spawn(move || {
                while let Ok((permit, job)) = rx.recv() {
                    if abort.load(Ordering::Relaxed) {
                        // Discard queued work: dropping the permit frees its slot
                        // and the job is not processed.
                        drop(permit);
                        continue;
                    }

                    let pipelines = Arc::clone(&pipelines);
                    rt.spawn(async move {
                        run_job(&pipelines, job).await;
                        drop(permit);
                    });
                }
            })
        };

        let (stop_tx, mut stop_rx) = stop::channel();

        let task = tokio::task::spawn(async move {
            enum Event {
                Update(Option<Result<SubscribeUpdate, Status>>),
                Stop(StopCode),
            }

            let (result, drain) = loop {
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
                            break (Err(crate::Error::YellowstoneStatus(e)), false);
                        },
                    },
                    Event::Update(None) => {
                        tracing::info!("Update stream closed");
                        break (Ok(StopCode::default()), true);
                    },
                    Event::Stop(c) => break (Ok(c), false),
                };

                // Build the trace span and bump the received counter, then exit
                // to a plain (Send) span so it can be held across the await below.
                let span = tracing::trace_span!("process_update", ?update).entered();

                #[cfg(feature = "prometheus")]
                if let Some(update_oneof) = update.update_oneof.as_ref() {
                    let update_type = metrics::UpdateType::from(update_oneof);
                    metrics::increment_received_updates(update_type);
                }

                let span = span.exit();

                // Take a permit before enqueueing; this awaits once `jobs` are
                // outstanding (backpressure). This must stay the only acquire
                // site: the drain barrier below waits for permits to reach zero
                // after tx is dropped, so a second acquirer or a cloned tx would
                // break it silently.
                let permit = Arc::clone(&sem)
                    .acquire_owned()
                    .await
                    .expect("job semaphore is never closed");

                if tx.send((permit, Job(span, update))).is_err() {
                    // Bridge is gone (receiver dropped); nothing can drain, stop.
                    break (Ok(StopCode::default()), false);
                }
            };

            // No more jobs coming; once the channel drains `rx.recv()` errors and
            // the bridge exits. `tx` was the only producer, so after this drop no
            // further permit can be acquired, which the drain barrier below needs.
            drop(tx);

            if drain {
                // Clean close: acquiring all `jobs` permits can only succeed once
                // nothing is queued or in flight, so this blocks until fully
                // drained. `jobs` is small, so the u32 cast never saturates.
                let all_permits = u32::try_from(jobs).unwrap_or(u32::MAX);
                let _ = Arc::clone(&sem).acquire_many_owned(all_permits).await;
            } else {
                // Error/stop: tell the bridge to drop queued work. In-flight
                // handlers are detached and left to finish.
                abort.store(true, Ordering::Relaxed);
            }

            // Join the bridge off the async worker so we never block the
            // executor; it exits promptly once tx is dropped.
            let _ = tokio::task::spawn_blocking(move || bridge.join()).await;

            result
        });

        Self(task, stop_tx)
    }
}
