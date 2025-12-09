#![allow(dead_code)]

use crate::grpc_autoreconnect_util::{connect_with_timeout_with_buffers, GeyserGrpcClientBufferConfig};
use crate::obfuscate::url_obfuscate_api_token;
use futures::{Sink, SinkExt, Stream, StreamExt};
use tracing::{debug, error, info, trace, warn, Level};
use std::fmt::{Debug, Display};
use std::future::Future;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc::error::SendTimeoutError;
use tokio::sync::mpsc::{self};
use tokio::task::JoinHandle;
use tokio::time::{sleep, timeout, Instant};
use tokio_util::sync::CancellationToken;
use yellowstone_grpc_client::{GeyserGrpcBuilderError, GeyserGrpcBuilderResult, GeyserGrpcClient, GeyserGrpcClientError};
use yellowstone_grpc_proto::geyser::{SubscribeRequest, SubscribeUpdate};
use yellowstone_grpc_proto::tonic::codec::CompressionEncoding;
use yellowstone_grpc_proto::tonic::service::Interceptor;
use yellowstone_grpc_proto::tonic::transport::ClientTlsConfig;
use yellowstone_grpc_proto::tonic::Status;

pub type AtomicSlot = Arc<AtomicU64>;

// 1-based attempt counter
type Attempt = u32;

// wraps payload and status messages
// clone is required by broacast channel
#[derive(Clone)]
pub enum Message {
    GeyserSubscribeUpdate(Box<SubscribeUpdate>),
    // connect (attempt=1) or reconnect(attempt=2..)
    Connecting(Attempt),
}

#[derive(Clone, Debug)]
pub struct GrpcConnectionTimeouts {
    pub connect_timeout: Duration,
    pub request_timeout: Duration,
    pub subscribe_timeout: Duration,
    pub receive_timeout: Duration,
}

#[derive(Clone)]
pub struct GrpcSourceConfig {
    pub grpc_addr: String,
    pub grpc_x_token: Option<String>,
    pub tls_config: Option<ClientTlsConfig>,
    pub max_decoding_message_size: usize,
    pub timeouts: Option<GrpcConnectionTimeouts>,
    pub compression: Option<CompressionEncoding>,
}

impl Display for GrpcSourceConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "grpc_addr {} (token? {}, compression {})",
            url_obfuscate_api_token(&self.grpc_addr),
            if self.grpc_x_token.is_some() {
                "yes"
            } else {
                "no"
            },
            self.compression
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("none".to_string())
        )?;

        Ok(())
    }
}

impl Debug for GrpcSourceConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

enum ConnectionState<
    S: Stream<Item = Result<SubscribeUpdate, Status>>,
    F: Interceptor,
    R: Sink<SubscribeRequest, Error = futures::channel::mpsc::SendError>,
> {
    NotConnected(Attempt),
    // connected but not subscribed
    Connecting(Attempt, GeyserGrpcClient<F>),
    Ready(S, R),
    // error states
    RecoverableConnectionError(Attempt),
    // non-recoverable error
    FatalError(Attempt, FatalErrorReason),
    WaitReconnect(Attempt),
    // exit signal received
    GracefulShutdown,
}

enum FatalErrorReason {
    DownstreamChannelClosed,
    ConfigurationError,
    NetworkError,
    SubscribeError,
}

// compat
pub fn create_geyser_autoconnection_task_with_mpsc(
    grpc_source: GrpcSourceConfig,
    subscribe_filter: SubscribeRequest,
    mpsc_downstream: mpsc::Sender<Result<SubscribeUpdate, Status>>,
    shutdown_token: CancellationToken,
) -> JoinHandle<()> {
    create_geyser_autoconnection_task_with_updater(
        grpc_source,
        subscribe_filter,
        mpsc_downstream,
        shutdown_token,
        None,
    )
}

/// connect to grpc source performing autoconnect if required,
/// returns mpsc channel; task will abort on fatal error
/// will shut down when receiver is dropped
///
/// read this for argument: http://www.randomhacks.net/2019/03/08/should-rust-channels-panic-on-send/
pub fn create_geyser_autoconnection_task_with_updater(
    grpc_source: GrpcSourceConfig,
    subscribe_filter: SubscribeRequest,
    mpsc_downstream: mpsc::Sender<Result<SubscribeUpdate, Status>>,
    shutdown_token: CancellationToken,
    mut subscribe_filter_update_rx: Option<mpsc::Receiver<SubscribeRequest>>,
) -> JoinHandle<()> {
    // task will be aborted when downstream receiver gets dropped
    // there are two ways to terminate: 1) using break 'main_loop 2) return from task
    let jh_geyser_task = tokio::spawn(async move {
        // use this filter for initial connect and update it if the client requests a change via client_subscribe_tx channel
        let mut subscribe_filter_on_connect = subscribe_filter;

        let (_dummy_filter_tx, dummy_filter_rx) = mpsc::channel::<SubscribeRequest>(1);
        let mut subscribe_filter_update_rx =
            subscribe_filter_update_rx.take().unwrap_or(dummy_filter_rx);
        let mut state = ConnectionState::NotConnected(1);
        let mut messages_forwarded = 0;

        'main_loop: loop {
            state = match state {
                ConnectionState::NotConnected(attempt) => {
                    let addr = grpc_source.grpc_addr.clone();
                    let token = grpc_source.grpc_x_token.clone();
                    let tls_config = grpc_source.tls_config.clone();
                    let connect_timeout = grpc_source.timeouts.as_ref().map(|t| t.connect_timeout);
                    let request_timeout = grpc_source.timeouts.as_ref().map(|t| t.request_timeout);
                    let compression = grpc_source.compression;
                    let max_decoding_message_size = grpc_source.max_decoding_message_size;
                    if attempt > 1 {
                        warn!("Connecting attempt {} to {}",
                            attempt,
                            addr);
                    } else {
                        debug!("Connecting attempt {} to {}",
                            attempt,
                            addr);
                    }

                    let buffer_config = GeyserGrpcClientBufferConfig::default();
                    trace!("Using Grpc Buffer config {:?}", buffer_config);

                    let connection_handler = |connect_result| match connect_result {
                        Ok(client) => ConnectionState::Connecting(attempt, client),
                        Err(GeyserGrpcBuilderError::MetadataValueError(_)) => {
                            ConnectionState::FatalError(
                                attempt + 1,
                                FatalErrorReason::ConfigurationError,
                            )
                        }
                        Err(GeyserGrpcBuilderError::TonicError(tonic_error)) => {
                            warn!(
                                "connect failed on {} with tonic error - aborting: {:?}",
                                grpc_source, tonic_error
                            );
                            ConnectionState::FatalError(attempt + 1, FatalErrorReason::NetworkError)
                        }
                    };

                    let fut_connector = connect_with_timeout_with_buffers(
                        addr,
                        token,
                        tls_config,
                        connect_timeout,
                        request_timeout,
                        buffer_config,
                        // zstd vs None reduces bandwitdh usage from 70MB/s to 17MB/s
                        compression,
                        max_decoding_message_size,
                    );

                    match await_or_exit(fut_connector, shutdown_token.clone()).await {
                        MaybeExit::Continue(connection_result) => {
                            connection_handler(connection_result)
                        }
                        MaybeExit::Exit => ConnectionState::GracefulShutdown,
                    }
                }
                ConnectionState::Connecting(attempt, mut client) => {
                    let subscribe_timeout =
                        grpc_source.timeouts.as_ref().map(|t| t.subscribe_timeout);
                    let subscribe_filter_on_connect = subscribe_filter_on_connect.clone();
                    debug!(
                        "Subscribe initially with filter {:?}",
                        subscribe_filter_on_connect
                    );

                    let fut_subscribe = timeout(
                        subscribe_timeout.unwrap_or(Duration::MAX),
                        client.subscribe_with_request(Some(subscribe_filter_on_connect)),
                    );

                    match await_or_exit(fut_subscribe, shutdown_token.clone()).await {
                        MaybeExit::Continue(subscribe_result_timeout) => {
                            match subscribe_result_timeout {
                                Ok(subscribe_result) => {
                                    match subscribe_result {
                                        Ok((geyser_subscribe_tx, geyser_stream)) => {
                                            if attempt > 1 {
                                                debug!(
                                                    "Subscribed to {} after {} failed attempts",
                                                    grpc_source, attempt
                                                );
                                            }
                                            ConnectionState::Ready(
                                                geyser_stream,
                                                geyser_subscribe_tx,
                                            )
                                        }
                                        Err(GeyserGrpcClientError::TonicStatus(geyser_error)) => {
                                            warn!(
                                                "subscribe failed on {} after {} attempts: {:#} - \
                                                 retrying",
                                                grpc_source, attempt, geyser_error
                                            );
                                            ConnectionState::RecoverableConnectionError(attempt + 1)
                                        }
                                        // non-recoverable
                                        Err(unrecoverable_error) => {
                                            error!(
                                                "subscribe to {} failed with unrecoverable error: \
                                                 {}",
                                                grpc_source, unrecoverable_error
                                            );
                                            ConnectionState::FatalError(
                                                attempt + 1,
                                                FatalErrorReason::SubscribeError,
                                            )
                                        }
                                    }
                                }
                                Err(_elapsed) => {
                                    warn!(
                                        "subscribe failed with timeout on {} - retrying",
                                        grpc_source
                                    );
                                    ConnectionState::RecoverableConnectionError(attempt + 1)
                                }
                            }
                        }
                        MaybeExit::Exit => ConnectionState::GracefulShutdown,
                    }
                }
                ConnectionState::RecoverableConnectionError(attempt) => {
                    let backoff_secs = 1.5_f32.powi(attempt as i32).min(15.0);
                    info!(
                        "waiting {} seconds, then reconnect to {}",
                        backoff_secs, grpc_source
                    );

                    let fut_sleep = sleep(Duration::from_secs_f32(backoff_secs));

                    match await_or_exit(fut_sleep, shutdown_token.clone()).await {
                        MaybeExit::Continue(()) => ConnectionState::NotConnected(attempt),
                        MaybeExit::Exit => ConnectionState::GracefulShutdown,
                    }
                }
                ConnectionState::FatalError(_attempt, reason) => match reason {
                    FatalErrorReason::DownstreamChannelClosed => {
                        warn!("downstream closed - aborting");
                        return;
                    }
                    FatalErrorReason::ConfigurationError => {
                        warn!("fatal configuration error - aborting");
                        return;
                    }
                    FatalErrorReason::NetworkError => {
                        warn!("fatal network error - aborting");
                        return;
                    }
                    FatalErrorReason::SubscribeError => {
                        warn!("fatal grpc subscribe error - aborting");
                        return;
                    }
                },
                ConnectionState::WaitReconnect(attempt) => {
                    let backoff_secs = 1.5_f32.powi(attempt as i32).min(15.0);
                    info!(
                        "waiting {} seconds, then reconnect to {}",
                        backoff_secs, grpc_source
                    );

                    let fut_sleep = sleep(Duration::from_secs_f32(backoff_secs));

                    match await_or_exit(fut_sleep, shutdown_token.clone()).await {
                        MaybeExit::Continue(()) => ConnectionState::NotConnected(attempt),
                        MaybeExit::Exit => ConnectionState::GracefulShutdown,
                    }
                }
                ConnectionState::Ready(mut geyser_stream, mut geyser_subscribe_tx) => {
                    let receive_timeout = grpc_source.timeouts.as_ref().map(|t| t.receive_timeout);

                    'recv_loop: loop {
                        select! {
                             _ = shutdown_token.cancelled() => {
                                info!("shutting down grpc autoconnect task on signal");
                                break 'recv_loop ConnectionState::GracefulShutdown;
                            },
                            // could model subscribe_filter_update_rx as optional here but did not figure out how
                            client_subscribe_update = subscribe_filter_update_rx.recv() => {
                                match client_subscribe_update {
                                    Some(subscribe_request) => {
                                        debug!("Subscription update from client with filter {:?}", subscribe_request);
                                        subscribe_filter_on_connect = subscribe_request.clone();
                                        // note: if the subscription is invalid, it will trigger a Tonic error:
                                        //  Status { code: InvalidArgument, message: "failed to create filter: Invalid Base58 string", source: None }
                                        if let Err(send_err) = geyser_subscribe_tx.send(subscribe_request).await {
                                            warn!("fail to send subscription update - disconnect and retry: {}", send_err);
                                            break 'recv_loop ConnectionState::WaitReconnect(1);
                                        };
                                    }
                                    None => {
                                        trace!("client subscribe channel closed, continue without");
                                        continue 'recv_loop;
                                    }
                                }
                            },
                            geyser_stream_res = timeout(
                                    receive_timeout.unwrap_or(Duration::MAX),
                                    geyser_stream.next(),
                                ) => {

                                match geyser_stream_res {
                                    Ok(Some(Ok(update_message))) => {
                                        trace!("> recv update message from {}", grpc_source);
                                        // note: first send never blocks as the mpsc channel has capacity 1
                                        let warning_threshold = if messages_forwarded == 1 {
                                            Duration::from_millis(3000)
                                        } else {
                                            Duration::from_millis(500)
                                        };
                                        let started_at = Instant::now();

                                        let fut_send = mpsc_downstream.send_timeout(
                                            Ok(update_message),
                                            warning_threshold,
                                        );

                                        let MaybeExit::Continue(mpsc_downstream_result) =
                                            await_or_exit(fut_send, shutdown_token.clone()).await
                                        else {
                                            break 'recv_loop ConnectionState::GracefulShutdown;
                                        };

                                        match mpsc_downstream_result {
                                            Ok(()) => {
                                                messages_forwarded += 1;
                                                if messages_forwarded == 1 {
                                                    // note: first send never blocks - do not print time as this is a lie
                                                    trace!("queued first update message");
                                                } else {
                                                    trace!(
                                                        "queued update message {} in {:.02}ms",
                                                        messages_forwarded,
                                                        started_at.elapsed().as_secs_f32() * 1000.0
                                                    );
                                                }
                                                continue 'recv_loop;
                                            }
                                            Err(SendTimeoutError::Timeout(the_message)) => {
                                                warn!("downstream receiver did not pick up message for {}ms - keep waiting", warning_threshold.as_millis());

                                                let fut_send = mpsc_downstream.send(the_message);

                                                let MaybeExit::Continue(mpsc_downstream_result) =
                                                    await_or_exit(fut_send, shutdown_token.clone()).await
                                                else {
                                                    break 'recv_loop ConnectionState::GracefulShutdown;
                                                };

                                                match mpsc_downstream_result {
                                                    Ok(()) => {
                                                        messages_forwarded += 1;
                                                        trace!(
                                                            "queued delayed update message {} in {:.02}ms",
                                                            messages_forwarded,
                                                            started_at.elapsed().as_secs_f32() * 1000.0
                                                        );
                                                    }
                                                    Err(_send_error) => {
                                                        warn!("downstream receiver closed, message is lost - aborting");
                                                        break 'recv_loop ConnectionState::FatalError(
                                                            0,
                                                            FatalErrorReason::DownstreamChannelClosed,
                                                        );
                                                    }
                                                }
                                            }
                                            Err(SendTimeoutError::Closed(_)) => {
                                                warn!("downstream receiver closed - aborting");
                                                break 'recv_loop ConnectionState::FatalError(
                                                    0,
                                                    FatalErrorReason::DownstreamChannelClosed,
                                                );
                                            }
                                        }
                                    }
                                    Ok(Some(Err(tonic_status))) => {
                                        // all tonic errors are recoverable
                                        warn!("tonic error on {} - retrying: {:#}", grpc_source, tonic_status);
                                        break 'recv_loop ConnectionState::WaitReconnect(1);
                                    }
                                    Ok(None) => {
                                        warn!("geyser stream closed on {} - retrying", grpc_source);
                                        break 'recv_loop ConnectionState::WaitReconnect(1);
                                    }
                                    Err(_elapsed) => {
                                        warn!("timeout on {} - retrying", grpc_source);
                                        break 'recv_loop ConnectionState::WaitReconnect(1);
                                    }
                                }; // -- END match

                            },
                        }
                    } // -- END receive loop
                }
                ConnectionState::GracefulShutdown => {
                    debug!("shutting down {} gracefully on exit signal", grpc_source);
                    break 'main_loop;
                }
            } // -- END match
        } // -- state loop; break ONLY on graceful shutdown
        debug!("gracefully exiting geyser task loop");
    });

    jh_geyser_task
}

enum MaybeExit<T> {
    Continue(T),
    Exit,
}

async fn await_or_exit<F>(future: F, shutdown_token: CancellationToken) -> MaybeExit<F::Output>
where
    F: Future,
{
    select! {
        res = future => {
            MaybeExit::Continue(res)
        },
        _ = shutdown_token.cancelled() => {
            debug!("exit on signal");
            MaybeExit::Exit
        }
    }
}
