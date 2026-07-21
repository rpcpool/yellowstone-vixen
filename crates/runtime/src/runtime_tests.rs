use std::{
    borrow::Cow,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use async_trait::async_trait;
use tokio::sync::{mpsc::Sender, oneshot};
use yellowstone_grpc_proto::{
    geyser::{subscribe_update::UpdateOneof, SlotStatus, SubscribeUpdate, SubscribeUpdateSlot},
    tonic,
};
use yellowstone_vixen_core::{ParseResult, Parser, Prefilter, SlotUpdate};

use crate::{
    config::{BufferConfig, NullConfig, VixenConfig},
    sources::{SourceExitStatus, SourceTrait},
    Error, Handler, Pipeline, Runtime,
};

async fn wait_for_runtime_ready() { tokio::time::sleep(Duration::from_millis(50)).await; }

async fn hold_channel_open_briefly() { tokio::time::sleep(Duration::from_millis(10)).await; }

fn signal_stream_ended(status_tx: oneshot::Sender<SourceExitStatus>) {
    let _ = status_tx.send(SourceExitStatus::StreamEnded);
}

fn signal_stream_error(
    status_tx: oneshot::Sender<SourceExitStatus>,
    code: tonic::Code,
    message: &str,
) {
    let _ = status_tx.send(SourceExitStatus::StreamError {
        code,
        message: message.to_string(),
    });
}

fn signal_error(status_tx: oneshot::Sender<SourceExitStatus>, message: &str) {
    let _ = status_tx.send(SourceExitStatus::Error(message.to_string()));
}

fn signal_receiver_dropped(status_tx: oneshot::Sender<SourceExitStatus>) {
    let _ = status_tx.send(SourceExitStatus::ReceiverDropped);
}

fn signal_completed(status_tx: oneshot::Sender<SourceExitStatus>) {
    let _ = status_tx.send(SourceExitStatus::Completed);
}

fn make_ping_update() -> SubscribeUpdate {
    SubscribeUpdate {
        filters: vec![],
        update_oneof: Some(
            yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof::Ping(
                yellowstone_grpc_proto::geyser::SubscribeUpdatePing {},
            ),
        ),
        created_at: None,
    }
}

const TEST_SLOT_FILTER: &str = "test::SlowSlotParser";
static SLOW_SLOT_HANDLED: AtomicUsize = AtomicUsize::new(0);

fn make_slot_update(slot: u64) -> SubscribeUpdate {
    SubscribeUpdate {
        filters: vec![TEST_SLOT_FILTER.to_string()],
        update_oneof: Some(UpdateOneof::Slot(SubscribeUpdateSlot {
            slot,
            parent: Some(slot.saturating_sub(1)),
            status: SlotStatus::SlotProcessed as i32,
            dead_error: None,
        })),
        created_at: None,
    }
}

fn default_test_config() -> VixenConfig<NullConfig> {
    VixenConfig {
        source: NullConfig,
        buffer: BufferConfig::default(),
    }
}

fn assert_server_hangup(result: Result<(), Box<Error>>) {
    assert!(result.is_err());
    assert!(matches!(*result.unwrap_err(), Error::ServerHangup));
}

fn assert_yellowstone_status(
    result: Result<(), Box<Error>>,
    expected_code: tonic::Code,
    expected_message_substring: &str,
) {
    assert!(result.is_err());
    match *result.unwrap_err() {
        Error::YellowstoneStatus(status) => {
            assert_eq!(status.code(), expected_code);
            assert!(
                status.message().contains(expected_message_substring),
                "expected message to contain {expected_message_substring:?}, got {:?}",
                status.message()
            );
        },
        other => panic!("expected YellowstoneStatus, got {other:?}"),
    }
}

fn assert_other_error(result: Result<(), Box<Error>>) {
    assert!(result.is_err());
    assert!(matches!(*result.unwrap_err(), Error::Other(_)));
}

fn create_status_channel() -> (
    oneshot::Sender<SourceExitStatus>,
    oneshot::Receiver<SourceExitStatus>,
) {
    oneshot::channel()
}

#[allow(clippy::type_complexity)]
fn create_update_channel() -> (
    Sender<Result<SubscribeUpdate, tonic::Status>>,
    tokio::sync::mpsc::Receiver<Result<SubscribeUpdate, tonic::Status>>,
) {
    tokio::sync::mpsc::channel(1)
}

fn drop_receiver<T>(rx: T) { drop(rx); }

async fn send_update_expecting_failure(tx: &Sender<Result<SubscribeUpdate, tonic::Status>>) {
    let result = tx.send(Ok(make_ping_update())).await;
    assert!(result.is_err(), "Send should fail when receiver dropped");
}

fn assert_receiver_dropped(status: &SourceExitStatus) {
    assert!(matches!(status, SourceExitStatus::ReceiverDropped));
}

fn assert_stream_ended(status: &SourceExitStatus) {
    assert!(
        matches!(status, SourceExitStatus::StreamEnded),
        "Expected StreamEnded, got {status:?}"
    );
}

fn assert_completed(status: &SourceExitStatus) {
    assert!(
        matches!(status, SourceExitStatus::Completed),
        "Expected Completed, got {status:?}"
    );
}

fn assert_stream_error_details(
    status: &SourceExitStatus,
    expected_code: tonic::Code,
    expected_msg: &str,
) {
    match status {
        SourceExitStatus::StreamError { code, message } => {
            assert_eq!(*code, expected_code);
            assert_eq!(message, expected_msg);
        },
        _ => panic!("Expected StreamError, got {status:?}"),
    }
}

fn assert_stream_error_code(status: &SourceExitStatus, expected_code: tonic::Code) {
    match status {
        SourceExitStatus::StreamError { code, .. } => {
            assert_eq!(*code, expected_code);
        },
        _ => panic!("Expected StreamError, got {status:?}"),
    }
}

fn assert_error_message(status: &SourceExitStatus, expected: &str) {
    match status {
        SourceExitStatus::Error(msg) => assert_eq!(msg, expected),
        _ => panic!("Expected Error, got {status:?}"),
    }
}

fn assert_send_fails<T, E>(result: &Result<T, E>) {
    assert!(result.is_err());
}

#[derive(Debug)]
struct MockStreamEndSource;

#[async_trait]
impl SourceTrait for MockStreamEndSource {
    type Config = NullConfig;

    fn new(_: NullConfig, _: vixen_core::Filters) -> Self { Self }

    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, tonic::Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
    ) -> Result<(), Error> {
        wait_for_runtime_ready().await;
        signal_stream_ended(status_tx);
        hold_channel_open_briefly().await;
        drop(tx);
        Ok(())
    }
}

#[derive(Debug)]
struct MockStreamErrorSource;

#[async_trait]
impl SourceTrait for MockStreamErrorSource {
    type Config = NullConfig;

    fn new(_: NullConfig, _: vixen_core::Filters) -> Self { Self }

    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, tonic::Status>>,
        _status_tx: oneshot::Sender<SourceExitStatus>,
    ) -> Result<(), Error> {
        wait_for_runtime_ready().await;
        let _ = tx
            .send(Err(tonic::Status::unavailable("server unavailable")))
            .await;
        // Buffer handles stream errors via tx channel - no need for oneshot
        hold_channel_open_briefly().await;
        Ok(())
    }
}

#[derive(Debug)]
struct MockSourceExitStreamErrorSource;

#[async_trait]
impl SourceTrait for MockSourceExitStreamErrorSource {
    type Config = NullConfig;

    fn new(_: NullConfig, _: vixen_core::Filters) -> Self { Self }

    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, tonic::Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
    ) -> Result<(), Error> {
        wait_for_runtime_ready().await;
        signal_stream_error(
            status_tx,
            tonic::Code::InvalidArgument,
            "failed to get replay position for slot 42",
        );
        hold_channel_open_briefly().await;
        drop(tx);
        Ok(())
    }
}

#[derive(Debug)]
struct MockErrorSource;

#[async_trait]
impl SourceTrait for MockErrorSource {
    type Config = NullConfig;

    fn new(_: NullConfig, _: vixen_core::Filters) -> Self { Self }

    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, tonic::Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
    ) -> Result<(), Error> {
        wait_for_runtime_ready().await;
        signal_error(status_tx, "something went wrong");
        hold_channel_open_briefly().await;
        drop(tx);
        Ok(())
    }
}

#[derive(Debug)]
struct MockStreamEndWithUpdatesSource {
    updates_to_send: u64,
}

#[async_trait]
impl SourceTrait for MockStreamEndWithUpdatesSource {
    type Config = NullConfig;

    fn new(_: NullConfig, _: vixen_core::Filters) -> Self { Self { updates_to_send: 5 } }

    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, tonic::Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
    ) -> Result<(), Error> {
        wait_for_runtime_ready().await;

        for _ in 0..self.updates_to_send {
            if tx.send(Ok(make_ping_update())).await.is_err() {
                signal_receiver_dropped(status_tx);
                return Ok(());
            }
        }

        signal_stream_ended(status_tx);
        hold_channel_open_briefly().await;
        drop(tx);
        Ok(())
    }
}

#[derive(Debug)]
struct MockCompletedWithUpdatesSource {
    updates_to_send: u64,
}

#[async_trait]
impl SourceTrait for MockCompletedWithUpdatesSource {
    type Config = NullConfig;

    fn new(_: NullConfig, _: vixen_core::Filters) -> Self { Self { updates_to_send: 3 } }

    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, tonic::Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
    ) -> Result<(), Error> {
        wait_for_runtime_ready().await;

        for slot in 0..self.updates_to_send {
            if tx.send(Ok(make_slot_update(slot))).await.is_err() {
                signal_receiver_dropped(status_tx);
                return Ok(());
            }
        }

        signal_completed(status_tx);
        drop(tx);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct SlowSlotParser;

impl Parser for SlowSlotParser {
    type Input = SlotUpdate;
    type Output = SlotUpdate;

    fn id(&self) -> Cow<'static, str> { TEST_SLOT_FILTER.into() }

    fn prefilter(&self) -> Prefilter { Prefilter::default() }

    async fn parse(&self, value: &Self::Input) -> ParseResult<Self::Output> { Ok(value.clone()) }
}

#[derive(Debug, Clone, Copy)]
struct SlowSlotHandler;

impl Handler<SlotUpdate, SlotUpdate> for SlowSlotHandler {
    async fn handle(
        &self,
        _value: &SlotUpdate,
        _raw_event: &SlotUpdate,
    ) -> crate::HandlerResult<()> {
        tokio::time::sleep(Duration::from_millis(50)).await;
        SLOW_SLOT_HANDLED.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

#[tokio::test]
async fn test_stream_end_returns_error() {
    let runtime = Runtime::<MockStreamEndSource>::builder()
        .try_build(default_test_config())
        .unwrap();

    assert_server_hangup(runtime.try_run_async().await);
}

#[tokio::test]
async fn test_stream_error_returns_error() {
    let runtime = Runtime::<MockStreamErrorSource>::builder()
        .try_build(default_test_config())
        .unwrap();

    assert!(runtime.try_run_async().await.is_err());
}

#[tokio::test]
async fn test_source_exit_stream_error_maps_to_yellowstone_status() {
    let runtime = Runtime::<MockSourceExitStreamErrorSource>::builder()
        .try_build(default_test_config())
        .unwrap();

    assert_yellowstone_status(
        runtime.try_run_async().await,
        tonic::Code::InvalidArgument,
        "replay position",
    );
}

#[tokio::test]
async fn test_error_status_returns_error() {
    let runtime = Runtime::<MockErrorSource>::builder()
        .try_build(default_test_config())
        .unwrap();

    assert_other_error(runtime.try_run_async().await);
}

#[tokio::test]
async fn test_stream_end_after_updates_returns_error() {
    let runtime = Runtime::<MockStreamEndWithUpdatesSource>::builder()
        .try_build(default_test_config())
        .unwrap();

    assert_server_hangup(runtime.try_run_async().await);
}

#[tokio::test]
async fn test_completed_source_drains_buffered_updates_before_returning() {
    SLOW_SLOT_HANDLED.store(0, Ordering::Relaxed);

    let runtime = Runtime::<MockCompletedWithUpdatesSource>::builder()
        .slot(Pipeline::new(SlowSlotParser, [SlowSlotHandler]))
        .try_build(default_test_config())
        .unwrap();

    assert!(runtime.try_run_async().await.is_ok());
    assert_eq!(
        SLOW_SLOT_HANDLED.load(Ordering::Relaxed),
        3,
        "runtime must wait for buffered slot handlers after finite source completion"
    );
}

#[tokio::test]
async fn test_source_exit_status_receiver_dropped() {
    let (tx, rx) = create_update_channel();
    let (status_tx, status_rx) = create_status_channel();

    drop_receiver(rx);
    send_update_expecting_failure(&tx).await;
    signal_receiver_dropped(status_tx);

    assert_receiver_dropped(&status_rx.await.unwrap());
}

#[tokio::test]
async fn test_source_exit_status_stream_ended() {
    let (status_tx, status_rx) = create_status_channel();

    signal_stream_ended(status_tx);

    assert_stream_ended(&status_rx.await.unwrap());
}

#[tokio::test]
async fn test_source_exit_status_completed() {
    let (status_tx, status_rx) = create_status_channel();

    signal_completed(status_tx);

    assert_completed(&status_rx.await.unwrap());
}

#[tokio::test]
async fn test_source_exit_status_stream_error_preserves_details() {
    let (status_tx, status_rx) = create_status_channel();

    signal_stream_error(status_tx, tonic::Code::PermissionDenied, "auth expired");

    assert_stream_error_details(
        &status_rx.await.unwrap(),
        tonic::Code::PermissionDenied,
        "auth expired",
    );
}

#[tokio::test]
async fn test_source_exit_status_error_preserves_message() {
    let (status_tx, status_rx) = create_status_channel();

    signal_error(status_tx, "connection timeout");

    assert_error_message(&status_rx.await.unwrap(), "connection timeout");
}

#[tokio::test]
async fn test_grpc_unavailable_error() {
    let (status_tx, status_rx) = create_status_channel();

    signal_stream_error(status_tx, tonic::Code::Unavailable, "service unavailable");

    assert_stream_error_code(&status_rx.await.unwrap(), tonic::Code::Unavailable);
}

#[tokio::test]
async fn test_grpc_unauthenticated_error() {
    let (status_tx, status_rx) = create_status_channel();

    signal_stream_error(status_tx, tonic::Code::Unauthenticated, "invalid token");

    assert_stream_error_details(
        &status_rx.await.unwrap(),
        tonic::Code::Unauthenticated,
        "invalid token",
    );
}

#[tokio::test]
async fn test_grpc_resource_exhausted_error() {
    let (status_tx, status_rx) = create_status_channel();

    signal_stream_error(
        status_tx,
        tonic::Code::ResourceExhausted,
        "rate limit exceeded",
    );

    assert_stream_error_code(&status_rx.await.unwrap(), tonic::Code::ResourceExhausted);
}

#[tokio::test]
async fn test_status_channel_dropped_before_send() {
    let (status_tx, status_rx) = create_status_channel();

    drop_receiver(status_rx);

    assert_send_fails(&status_tx.send(SourceExitStatus::StreamEnded));
}

// Buffer pool edge cases. Each test uses its own atomic counter so they stay
// correct under parallel `cargo test`.

static BURST_HANDLED: AtomicUsize = AtomicUsize::new(0);
static PANIC_SURVIVED: AtomicUsize = AtomicUsize::new(0);
static SERIAL_HANDLED: AtomicUsize = AtomicUsize::new(0);
static STOP_RESPONSIVE_HANDLED: AtomicUsize = AtomicUsize::new(0);
static ZERO_JOBS_HANDLED: AtomicUsize = AtomicUsize::new(0);

/// Emits `N` slot updates then signals `Completed`.
#[derive(Debug)]
struct MockBurstSource<const N: u64>;

#[async_trait]
impl<const N: u64> SourceTrait for MockBurstSource<N> {
    type Config = NullConfig;

    fn new(_: NullConfig, _: vixen_core::Filters) -> Self { Self }

    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, tonic::Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
    ) -> Result<(), Error> {
        wait_for_runtime_ready().await;
        for slot in 0..N {
            if tx.send(Ok(make_slot_update(slot))).await.is_err() {
                signal_receiver_dropped(status_tx);
                return Ok(());
            }
        }
        signal_completed(status_tx);
        drop(tx);
        Ok(())
    }
}

/// Bumps a counter; optionally panics on even slots or sleeps to force overlap.
#[derive(Debug, Clone, Copy)]
struct CountingHandler {
    counter: &'static AtomicUsize,
    sleep: Duration,
    panic_on_even_slot: bool,
}

impl Handler<SlotUpdate, SlotUpdate> for CountingHandler {
    async fn handle(
        &self,
        value: &SlotUpdate,
        _raw_event: &SlotUpdate,
    ) -> crate::HandlerResult<()> {
        // assert! form (not `if c { panic!() }`) satisfies clippy::manual_assert.
        assert!(
            !(self.panic_on_even_slot && value.slot.is_multiple_of(2)),
            "intentional handler panic on slot {}",
            value.slot
        );
        if !self.sleep.is_zero() {
            tokio::time::sleep(self.sleep).await;
        }
        self.counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

/// Slow handler, to actually exercise backpressure at this burst size.
#[tokio::test]
async fn edge_high_volume_drain_processes_every_update() {
    const N: u64 = 500;
    BURST_HANDLED.store(0, Ordering::Relaxed);

    let runtime = Runtime::<MockBurstSource<N>>::builder()
        .slot(Pipeline::new(SlowSlotParser, [CountingHandler {
            counter: &BURST_HANDLED,
            sleep: Duration::from_millis(1),
            panic_on_even_slot: false,
        }]))
        .try_build(default_test_config())
        .unwrap();

    assert!(runtime.try_run_async().await.is_ok());
    assert_eq!(
        BURST_HANDLED.load(Ordering::Relaxed),
        usize::try_from(N).unwrap(),
        "all {N} updates must be handled"
    );
}

/// Each handler runs as its own task, so a panic stays isolated and the pool
/// survives.
#[tokio::test]
async fn edge_panicking_handler_does_not_kill_pool() {
    const N: u64 = 20; // slots 0..20 -> 10 even (panic), 10 odd (survive)
    PANIC_SURVIVED.store(0, Ordering::Relaxed);

    let runtime = Runtime::<MockBurstSource<N>>::builder()
        .slot(Pipeline::new(SlowSlotParser, [CountingHandler {
            counter: &PANIC_SURVIVED,
            sleep: Duration::from_millis(1),
            panic_on_even_slot: true,
        }]))
        .try_build(default_test_config())
        .unwrap();

    // ~10 "task panicked" lines on stderr are expected.
    assert!(runtime.try_run_async().await.is_ok());
    assert_eq!(
        PANIC_SURVIVED.load(Ordering::Relaxed),
        10,
        "odd-slot handlers must all run despite even-slot panics"
    );
}

#[tokio::test]
async fn edge_single_job_serializes_and_drains() {
    const N: u64 = 25;
    SERIAL_HANDLED.store(0, Ordering::Relaxed);

    let config = VixenConfig {
        source: NullConfig,
        buffer: BufferConfig {
            jobs: Some(1),
            ..BufferConfig::default()
        },
    };

    let runtime = Runtime::<MockBurstSource<N>>::builder()
        .slot(Pipeline::new(SlowSlotParser, [CountingHandler {
            counter: &SERIAL_HANDLED,
            sleep: Duration::from_millis(1),
            panic_on_even_slot: false,
        }]))
        .try_build(config)
        .unwrap();

    assert!(runtime.try_run_async().await.is_ok());
    assert_eq!(
        SERIAL_HANDLED.load(Ordering::Relaxed),
        usize::try_from(N).unwrap(),
        "jobs=1 must still process every update"
    );
}

/// Parks forever, like a handler blocked on a full channel whose reader
/// stalled. Holds its job permit the whole time.
#[derive(Debug, Clone, Copy)]
struct WedgedHandler;

impl Handler<SlotUpdate, SlotUpdate> for WedgedHandler {
    async fn handle(&self, _: &SlotUpdate, _: &SlotUpdate) -> crate::HandlerResult<()> {
        std::future::pending::<()>().await;
        unreachable!("pending never resolves")
    }
}

/// Holds its permit forever, so the drain barrier can't acquire them all; must
/// give up at the ceiling instead of hanging.
#[tokio::test]
async fn edge_wedged_handler_does_not_hang_shutdown() {
    const N: u64 = 2;

    let runtime = Runtime::<MockBurstSource<N>>::builder()
        .slot(Pipeline::new(SlowSlotParser, [WedgedHandler]))
        .try_build(default_test_config())
        .unwrap();

    // Fails by hanging if the barrier is unbounded; the outer timeout is the
    // real assertion, sized well above the (test-shortened) drain ceiling.
    let ran = tokio::time::timeout(Duration::from_secs(5), runtime.try_run_async()).await;

    assert!(
        ran.is_ok(),
        "shutdown must not block on a handler that never returns"
    );
    assert!(ran.unwrap().is_ok(), "bounded drain is a clean shutdown");
}

static PROBE_IN_FLIGHT: AtomicUsize = AtomicUsize::new(0);
static PROBE_PEAK: AtomicUsize = AtomicUsize::new(0);
static PROBE_HANDLED: AtomicUsize = AtomicUsize::new(0);

/// Tracks how many copies of itself run at once, recording the peak.
#[derive(Debug, Clone, Copy)]
struct ConcurrencyProbeHandler {
    in_flight: &'static AtomicUsize,
    peak: &'static AtomicUsize,
    handled: &'static AtomicUsize,
}

impl Handler<SlotUpdate, SlotUpdate> for ConcurrencyProbeHandler {
    async fn handle(
        &self,
        value: &SlotUpdate,
        _raw_event: &SlotUpdate,
    ) -> crate::HandlerResult<()> {
        // A handler only observes in-flight concurrency; peak in-flight <= jobs
        // is the observable half of the queued+in-flight <= jobs bound.
        let current = self.in_flight.fetch_add(1, Ordering::Relaxed) + 1;
        self.peak.fetch_max(current, Ordering::Relaxed);

        // Vary latency so handlers overlap.
        tokio::time::sleep(Duration::from_millis(1 + value.slot % 5)).await;

        self.in_flight.fetch_sub(1, Ordering::Relaxed);
        self.handled.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

/// Peak must exceed 1, or the bound goes unexercised.
#[tokio::test]
async fn edge_concurrency_never_exceeds_jobs() {
    const N: u64 = 200;
    const JOBS: usize = 4;
    PROBE_IN_FLIGHT.store(0, Ordering::Relaxed);
    PROBE_PEAK.store(0, Ordering::Relaxed);
    PROBE_HANDLED.store(0, Ordering::Relaxed);

    let config = VixenConfig {
        source: NullConfig,
        buffer: BufferConfig {
            jobs: Some(JOBS),
            ..BufferConfig::default()
        },
    };

    let runtime = Runtime::<MockBurstSource<N>>::builder()
        .slot(Pipeline::new(SlowSlotParser, [ConcurrencyProbeHandler {
            in_flight: &PROBE_IN_FLIGHT,
            peak: &PROBE_PEAK,
            handled: &PROBE_HANDLED,
        }]))
        .try_build(config)
        .unwrap();

    assert!(runtime.try_run_async().await.is_ok());

    let peak = PROBE_PEAK.load(Ordering::Relaxed);
    assert!(peak <= JOBS, "peak in-flight {peak} exceeded jobs={JOBS}");
    assert!(
        peak > 1,
        "handlers never overlapped (peak {peak}); bound not exercised"
    );
    assert_eq!(
        PROBE_HANDLED.load(Ordering::Relaxed),
        usize::try_from(N).unwrap(),
        "every update must still be processed"
    );
}

static ABORT_COMPLETED: AtomicUsize = AtomicUsize::new(0);

/// Sends `PRE` updates, a stream error, then `POST` more. The buffer breaks on
/// the error and drops its receiver, so the `POST` updates are never read.
/// `PRE` < jobs so the producer never blocks on a permit before the error.
#[derive(Debug)]
struct MockAbortAfterSource<const PRE: u64, const POST: u64>;

#[async_trait]
impl<const PRE: u64, const POST: u64> SourceTrait for MockAbortAfterSource<PRE, POST> {
    type Config = NullConfig;

    fn new(_: NullConfig, _: vixen_core::Filters) -> Self { Self }

    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, tonic::Status>>,
        _status_tx: oneshot::Sender<SourceExitStatus>,
    ) -> Result<(), Error> {
        wait_for_runtime_ready().await;

        for slot in 0..PRE {
            if tx.send(Ok(make_slot_update(slot))).await.is_err() {
                return Ok(());
            }
        }

        // Trigger the abort path via a stream error.
        if tx
            .send(Err(tonic::Status::unavailable("aborting mid-burst")))
            .await
            .is_err()
        {
            return Ok(());
        }

        // Never read: the producer already broke on the error above.
        for slot in PRE..PRE + POST {
            if tx.send(Ok(make_slot_update(slot))).await.is_err() {
                break;
            }
        }

        // Keep the source alive briefly so the error surfaces via the buffer's
        // `wait_for_stop` rather than a status-channel-closed race. The source
        // task is detached, so this does not delay the shutdown return.
        tokio::time::sleep(Duration::from_millis(300)).await;

        Ok(())
    }
}

/// Asserts only the deterministic contract (`completed <= PRE`), never
/// cancelled-vs-unfinished timing, which would be flaky.
#[tokio::test]
async fn edge_abort_discards_queued_and_returns_promptly() {
    const PRE: u64 = 3; // < JOBS, so the producer never blocks on a permit
    const POST: u64 = 100;
    const JOBS: usize = 4;
    const HANDLER_SLEEP: Duration = Duration::from_secs(2);
    const ABORT_BUDGET: Duration = Duration::from_millis(1000);

    ABORT_COMPLETED.store(0, Ordering::Relaxed);

    let config = VixenConfig {
        source: NullConfig,
        buffer: BufferConfig {
            jobs: Some(JOBS),
            ..BufferConfig::default()
        },
    };

    let runtime = Runtime::<MockAbortAfterSource<PRE, POST>>::builder()
        .slot(Pipeline::new(SlowSlotParser, [CountingHandler {
            counter: &ABORT_COMPLETED,
            sleep: HANDLER_SLEEP,
            panic_on_even_slot: false,
        }]))
        .try_build(config)
        .unwrap();

    // Must return within budget, not block on the 2s handlers.
    let result = tokio::time::timeout(ABORT_BUDGET, runtime.try_run_async())
        .await
        .expect("abort must return promptly, not wait for in-flight handlers");

    assert!(result.is_err(), "a stream error must surface as an error");

    // Only the PRE pre-error updates can ever run.
    let completed = ABORT_COMPLETED.load(Ordering::Relaxed);
    assert!(
        completed <= usize::try_from(PRE).unwrap(),
        ">= {POST} post-error updates must be discarded; {completed} ran"
    );
}

/// Sends two updates then a receiver-dropped signal to force shutdown. With
/// jobs=1, the first update's handler holds the only permit, so the producer
/// parks waiting for a permit for the second one when the stop arrives.
#[derive(Debug)]
struct MockStopUnderBackpressureSource;

#[async_trait]
impl SourceTrait for MockStopUnderBackpressureSource {
    type Config = NullConfig;

    fn new(_: NullConfig, _: vixen_core::Filters) -> Self { Self }

    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, tonic::Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
    ) -> Result<(), Error> {
        wait_for_runtime_ready().await;

        // Occupies the single permit for far longer than the shutdown budget.
        let _ = tx.send(Ok(make_slot_update(0))).await;
        // Makes the producer park on the permit acquire.
        let _ = tx.send(Ok(make_slot_update(1))).await;

        // Give the producer time to dequeue update 1 and park, then force stop.
        tokio::time::sleep(Duration::from_millis(300)).await;
        signal_receiver_dropped(status_tx);
        drop(tx);
        Ok(())
    }
}

/// A stop must break the producer out of a permit wait even when a stuck
/// handler holds the only permit.
#[tokio::test]
async fn edge_stop_is_responsive_while_waiting_for_a_permit() {
    const HANDLER_SLEEP: Duration = Duration::from_secs(10);
    const SHUTDOWN_BUDGET: Duration = Duration::from_secs(3);

    STOP_RESPONSIVE_HANDLED.store(0, Ordering::Relaxed);

    let config = VixenConfig {
        source: NullConfig,
        buffer: BufferConfig {
            jobs: Some(1),
            ..BufferConfig::default()
        },
    };

    let runtime = Runtime::<MockStopUnderBackpressureSource>::builder()
        .slot(Pipeline::new(SlowSlotParser, [CountingHandler {
            counter: &STOP_RESPONSIVE_HANDLED,
            sleep: HANDLER_SLEEP,
            panic_on_even_slot: false,
        }]))
        .try_build(config)
        .unwrap();

    // Must return well before the 10s handler frees the permit.
    let result = tokio::time::timeout(SHUTDOWN_BUDGET, runtime.try_run_async())
        .await
        .expect("stop must be honored while the producer is waiting for a permit");

    assert!(result.is_ok(), "forced shutdown should return cleanly");
}

/// Clamped to 1: a 0-permit semaphore would park the producer forever.
#[tokio::test]
async fn edge_zero_jobs_does_not_deadlock() {
    const N: u64 = 10;
    ZERO_JOBS_HANDLED.store(0, Ordering::Relaxed);

    let config = VixenConfig {
        source: NullConfig,
        buffer: BufferConfig {
            jobs: Some(0),
            ..BufferConfig::default()
        },
    };

    let runtime = Runtime::<MockBurstSource<N>>::builder()
        .slot(Pipeline::new(SlowSlotParser, [CountingHandler {
            counter: &ZERO_JOBS_HANDLED,
            sleep: Duration::from_millis(1),
            panic_on_even_slot: false,
        }]))
        .try_build(config)
        .unwrap();

    // Bounded so a 0-permit deadlock regression fails instead of hanging.
    let result = tokio::time::timeout(Duration::from_secs(5), runtime.try_run_async())
        .await
        .expect("jobs=0 must not deadlock");

    assert!(result.is_ok());
    assert_eq!(
        ZERO_JOBS_HANDLED.load(Ordering::Relaxed),
        usize::try_from(N).unwrap(),
        "every update must be processed when jobs is clamped from 0 to 1"
    );
}
