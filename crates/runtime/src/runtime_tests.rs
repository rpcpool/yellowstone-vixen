use std::time::Duration;

use async_trait::async_trait;
use tokio::sync::{mpsc::Sender, oneshot};
use yellowstone_grpc_proto::{geyser::SubscribeUpdate, tonic};

use crate::{
    config::{BufferConfig, NullConfig, VixenConfig},
    sources::{SourceExitStatus, SourceTrait},
    Error, Runtime,
};

// ========== Test helpers ==========

async fn wait_for_runtime_ready() {
    tokio::time::sleep(Duration::from_millis(50)).await;
}

async fn hold_channel_open_briefly() {
    tokio::time::sleep(Duration::from_millis(10)).await;
}

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

fn assert_other_error(result: Result<(), Box<Error>>) {
    assert!(result.is_err());
    assert!(matches!(*result.unwrap_err(), Error::Other(_)));
}

// ========== Channel factory helpers ==========

fn create_status_channel() -> (
    oneshot::Sender<SourceExitStatus>,
    oneshot::Receiver<SourceExitStatus>,
) {
    oneshot::channel()
}

fn create_update_channel() -> (
    Sender<Result<SubscribeUpdate, tonic::Status>>,
    tokio::sync::mpsc::Receiver<Result<SubscribeUpdate, tonic::Status>>,
) {
    tokio::sync::mpsc::channel(1)
}

// ========== Action helpers ==========

fn drop_receiver<T>(rx: T) {
    drop(rx);
}

async fn send_update_expecting_failure(tx: &Sender<Result<SubscribeUpdate, tonic::Status>>) {
    let result = tx.send(Ok(make_ping_update())).await;
    assert!(result.is_err(), "Send should fail when receiver dropped");
}


// ========== Assertion helpers ==========

fn assert_receiver_dropped(status: SourceExitStatus) {
    assert!(matches!(status, SourceExitStatus::ReceiverDropped));
}

fn assert_stream_ended(status: SourceExitStatus) {
    assert!(
        matches!(status, SourceExitStatus::StreamEnded),
        "Expected StreamEnded, got {:?}",
        status
    );
}

fn assert_completed(status: SourceExitStatus) {
    assert!(
        matches!(status, SourceExitStatus::Completed),
        "Expected Completed, got {:?}",
        status
    );
}

fn assert_stream_error_details(
    status: SourceExitStatus,
    expected_code: tonic::Code,
    expected_msg: &str,
) {
    match status {
        SourceExitStatus::StreamError { code, message } => {
            assert_eq!(code, expected_code);
            assert_eq!(message, expected_msg);
        },
        _ => panic!("Expected StreamError, got {:?}", status),
    }
}

fn assert_stream_error_code(status: SourceExitStatus, expected_code: tonic::Code) {
    match status {
        SourceExitStatus::StreamError { code, .. } => {
            assert_eq!(code, expected_code);
        },
        _ => panic!("Expected StreamError, got {:?}", status),
    }
}

fn assert_error_message(status: SourceExitStatus, expected: &str) {
    match status {
        SourceExitStatus::Error(msg) => assert_eq!(msg, expected),
        _ => panic!("Expected Error, got {:?}", status),
    }
}

fn assert_send_fails<T, E>(result: Result<T, E>) {
    assert!(result.is_err());
}

// ========== Mock sources ==========

#[derive(Debug)]
struct MockStreamEndSource;

#[async_trait]
impl SourceTrait for MockStreamEndSource {
    type Config = NullConfig;
    fn new(_: NullConfig, _: vixen_core::Filters) -> Self {
        Self
    }

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
    fn new(_: NullConfig, _: vixen_core::Filters) -> Self {
        Self
    }

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
struct MockErrorSource;

#[async_trait]
impl SourceTrait for MockErrorSource {
    type Config = NullConfig;
    fn new(_: NullConfig, _: vixen_core::Filters) -> Self {
        Self
    }

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
    fn new(_: NullConfig, _: vixen_core::Filters) -> Self {
        Self { updates_to_send: 5 }
    }

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

// ========== Integration tests ==========

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

// ========== Unit tests for SourceExitStatus ==========

#[tokio::test]
async fn test_source_exit_status_receiver_dropped() {
    let (tx, rx) = create_update_channel();
    let (status_tx, status_rx) = create_status_channel();

    drop_receiver(rx);
    send_update_expecting_failure(&tx).await;
    signal_receiver_dropped(status_tx);

    assert_receiver_dropped(status_rx.await.unwrap());
}

#[tokio::test]
async fn test_source_exit_status_stream_ended() {
    let (status_tx, status_rx) = create_status_channel();

    signal_stream_ended(status_tx);

    assert_stream_ended(status_rx.await.unwrap());
}

#[tokio::test]
async fn test_source_exit_status_completed() {
    let (status_tx, status_rx) = create_status_channel();

    signal_completed(status_tx);

    assert_completed(status_rx.await.unwrap());
}

#[tokio::test]
async fn test_source_exit_status_stream_error_preserves_details() {
    let (status_tx, status_rx) = create_status_channel();

    signal_stream_error(status_tx, tonic::Code::PermissionDenied, "auth expired");

    assert_stream_error_details(
        status_rx.await.unwrap(),
        tonic::Code::PermissionDenied,
        "auth expired",
    );
}

#[tokio::test]
async fn test_source_exit_status_error_preserves_message() {
    let (status_tx, status_rx) = create_status_channel();

    signal_error(status_tx, "connection timeout");

    assert_error_message(status_rx.await.unwrap(), "connection timeout");
}

// ========== Unit tests for various gRPC error codes ==========

#[tokio::test]
async fn test_grpc_unavailable_error() {
    let (status_tx, status_rx) = create_status_channel();

    signal_stream_error(status_tx, tonic::Code::Unavailable, "service unavailable");

    assert_stream_error_code(status_rx.await.unwrap(), tonic::Code::Unavailable);
}

#[tokio::test]
async fn test_grpc_unauthenticated_error() {
    let (status_tx, status_rx) = create_status_channel();

    signal_stream_error(status_tx, tonic::Code::Unauthenticated, "invalid token");

    assert_stream_error_details(
        status_rx.await.unwrap(),
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

    assert_stream_error_code(status_rx.await.unwrap(), tonic::Code::ResourceExhausted);
}

// ========== Edge case tests ==========

#[tokio::test]
async fn test_status_channel_dropped_before_send() {
    let (status_tx, status_rx) = create_status_channel();

    drop_receiver(status_rx);

    assert_send_fails(status_tx.send(SourceExitStatus::StreamEnded));
}
