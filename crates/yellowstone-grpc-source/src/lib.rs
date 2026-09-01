use std::time::Duration;

use async_trait::async_trait;
use clap::ValueEnum;
use futures_util::{SinkExt, StreamExt};
use shipstern::{
    sources::{SourceExitStatus, SourceTrait},
    CommitmentLevel, Error as ShipsternError,
};
use shipstern_core::Filters;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    oneshot,
};
use yellowstone_grpc_client::{Backoff, GeyserGrpcClient, ReconnectConfig, SubscribeRequestSink};
use yellowstone_grpc_proto::{
    geyser::{SubscribeRequest, SubscribeUpdate},
    tonic::{codec::CompressionEncoding, transport::ClientTlsConfig, Status},
};

#[derive(Default, Copy, Debug, serde::Deserialize, Clone, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum ShipsternCompressionEncoding {
    Gzip,
    #[default]
    Zstd,
}

impl From<ShipsternCompressionEncoding> for CompressionEncoding {
    fn from(val: ShipsternCompressionEncoding) -> Self {
        match val {
            ShipsternCompressionEncoding::Gzip => CompressionEncoding::Gzip,
            ShipsternCompressionEncoding::Zstd => CompressionEncoding::Zstd,
        }
    }
}

const fn default_auto_reconnect() -> bool { true }

/// Reconnect defaults, deliberately sturdier than the client library's
/// (3 retries / 10ms base), which gives up in under ~100ms. With these,
/// 10 retries doubling from 500ms cover outages up to ~4 minutes
/// (500ms, 1s, 2s, 4s, 8s, 16s, 32s, 64s, 128s, 256s).
const DEFAULT_RECONNECT_MAX_RETRIES: u32 = 10;
const DEFAULT_RECONNECT_INITIAL_BACKOFF: Duration = Duration::from_millis(500);
const DEFAULT_RECONNECT_MULTIPLIER: f64 = 2.0;

/// Yellowstone connection configuration.
#[derive(Debug, clap::Args, serde::Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct YellowstoneGrpcConfig {
    /// The endpoint of the Yellowstone server.
    #[arg(long, env)]
    pub endpoint: String,
    /// The token to use for authentication.
    #[arg(long, env)]
    pub x_token: Option<String>,
    /// The timeout for the connection.
    #[arg(long, env, default_value_t = 120)]
    pub timeout: u64,

    #[arg(long, env)]
    pub commitment_level: Option<CommitmentLevel>,

    #[arg(long, env)]
    pub from_slot: Option<u64>,

    #[arg(long, env)]
    pub max_decoding_message_size: Option<usize>,

    #[arg(long, env)]
    pub accept_compression: Option<ShipsternCompressionEncoding>,

    /// Enable the client's built-in auto-reconnect on the gRPC stream.
    ///
    /// When enabled, the stream reconnects with exponential backoff after a
    /// transient failure, resumes from the last seen slot, and deduplicates
    /// replayed events. The server must have `replay_stored_slots` configured
    /// for gap-free recovery.
    ///
    /// Defaults to `true`: a config file omitting this key (including ones that
    /// predate the field) gets auto-reconnect. Set to `false` to opt out.
    #[arg(long, env, default_value_t = true)]
    #[serde(default = "default_auto_reconnect")]
    pub auto_reconnect: bool,

    /// Max reconnect attempts before the stream gives up.
    ///
    /// Only applies when `auto_reconnect` is set. Falls back to the client
    /// library default when unset.
    #[arg(long, env)]
    pub reconnect_max_retries: Option<u32>,

    /// Number of recent slots retained for dedup during the replay window.
    ///
    /// Only applies when `auto_reconnect` is set. Falls back to the client
    /// library default when unset.
    #[arg(long, env)]
    pub reconnect_slot_retention: Option<usize>,
}

impl YellowstoneGrpcConfig {
    /// Build the auto-reconnect config from the user-facing flags.
    ///
    /// Uses sturdier backoff defaults than the client library (see
    /// [`DEFAULT_RECONNECT_MAX_RETRIES`] and friends); `reconnect_max_retries`
    /// and `reconnect_slot_retention` override the retry count and dedup window.
    ///
    /// Example output:
    /// ```rust, ignore
    /// // auto_reconnect = false
    /// assert!(config.reconnect_config().is_none());
    ///
    /// // auto_reconnect = true, no overrides -> sturdy defaults
    /// let rc = config.reconnect_config().unwrap();
    /// assert_eq!(rc.backoff.max_retries, 10);
    /// assert_eq!(rc.backoff.multiplier, 2.0);
    /// ```
    ///
    /// Returns `None` when auto-reconnect is disabled.
    pub fn reconnect_config(&self) -> Option<ReconnectConfig> {
        if !self.auto_reconnect {
            return None;
        }

        let max_retries = self
            .reconnect_max_retries
            .unwrap_or(DEFAULT_RECONNECT_MAX_RETRIES);

        let backoff = Backoff::new(
            DEFAULT_RECONNECT_INITIAL_BACKOFF,
            DEFAULT_RECONNECT_MULTIPLIER,
            max_retries,
        );

        // Start from the library default for fields we keep (slot_retention),
        // then swap in the sturdier backoff.
        let mut config = ReconnectConfig::default().with_backoff(backoff);

        if let Some(slot_retention) = self.reconnect_slot_retention {
            config.slot_retention = slot_retention;
        }

        Some(config)
    }
}

/// A `Source` implementation for the Yellowstone gRPC API.
#[derive(Debug)]
pub struct YellowstoneGrpcSource {
    filters: Filters,
    config: YellowstoneGrpcConfig,
}

/// Build the wire subscription for `filters`, layering on the commitment the
/// `From<Filters>` conversion leaves unset.
///
/// `from_slot` is deliberately not applied here. It is a one-time start
/// position rather than a steady-state setting, and repeating it on a
/// mid-stream update is destructive: yellowstone-grpc-geyser treats it as a
/// replay request and either replays every slot since, or ends the stream
/// with `from_slot is not supported` when it has no replay buffer, while
/// richat rejects the request outright if the set contains blocks. The client
/// library agrees, overwriting the field with the live checkpoint on
/// reconnect rather than reusing the configured value. Only the initial
/// subscribe sets it.
///
fn build_subscribe_request(filters: Filters, config: &YellowstoneGrpcConfig) -> SubscribeRequest {
    let mut request: SubscribeRequest = filters.into();

    if let Some(commitment_level) = config.commitment_level {
        request.commitment = Some(commitment_level as i32);
    }

    request
}

/// How often a filter set held after a sink rejection is retried.
const RETRY_HELD_FILTERS_EVERY: Duration = Duration::from_secs(5);

/// Send `filters` to the server, handing it back unsent if the sink rejects it.
///
/// A rejection means the request channel is disconnected. With auto-reconnect
/// enabled, which is the default, that is a transient reconnect window rather
/// than a fatal error: the stream yields nothing during it and the client
/// library swaps a fresh sender into this sink once it recovers. Dropping the
/// set here would lose it silently, because the sink only records a request
/// into its reconnect state after a successful send, so the reconnect would
/// resubscribe with the previous filters.
///
async fn send_filter_update(
    sink: &mut SubscribeRequestSink,
    config: &YellowstoneGrpcConfig,
    filters: Filters,
) -> Option<Filters> {
    let request = build_subscribe_request(filters.clone(), config);

    tracing::debug!(
        accounts = request.accounts.len(),
        transactions = request.transactions.len(),
        slots = request.slots.len(),
        blocks = request.blocks.len(),
        blocks_meta = request.blocks_meta.len(),
        "Sending filter update to the live subscription"
    );

    if let Err(err) = sink.send(request).await {
        tracing::warn!(
            %err,
            "Filter update rejected by the sink, holding it until the stream recovers"
        );

        return Some(filters);
    }

    None
}

#[async_trait]
impl SourceTrait for YellowstoneGrpcSource {
    type Config = YellowstoneGrpcConfig;

    fn new(config: Self::Config, filters: Filters) -> Self { Self { config, filters } }

    fn supports_filter_updates() -> bool { true }

    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
    ) -> Result<(), ShipsternError> {
        let (_, closed) = mpsc::channel(1);

        self.run(tx, status_tx, closed).await
    }

    async fn connect_with_filter_updates(
        &self,
        tx: Sender<Result<SubscribeUpdate, Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
        filter_updates: Receiver<Filters>,
    ) -> Result<(), ShipsternError> {
        self.run(tx, status_tx, filter_updates).await
    }
}

impl YellowstoneGrpcSource {
    /// Open the subscription and pump updates until the stream ends, sending
    /// any filter set that arrives on `filter_updates` to the server so it
    /// replaces the live subscription.
    ///
    /// An already closed `filter_updates` is expected rather than an error:
    /// it is what `connect` passes when the caller never asked for updates,
    /// and it retires the update branch after one poll.
    ///
    async fn run(
        &self,
        tx: Sender<Result<SubscribeUpdate, Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
        mut filter_updates: Receiver<Filters>,
    ) -> Result<(), ShipsternError> {
        let filters = self.filters.clone();
        let config = self.config.clone();
        let timeout = Duration::from_secs(config.timeout);

        let mut builder = GeyserGrpcClient::build_from_shared(config.endpoint.clone())?
            .x_token(config.x_token.clone())?
            .max_decoding_message_size(config.max_decoding_message_size.unwrap_or(usize::MAX))
            .accept_compressed(config.accept_compression.unwrap_or_default().into())
            .connect_timeout(timeout)
            .timeout(timeout)
            .tls_config(ClientTlsConfig::new().with_native_roots())?;

        if let Some(reconnect_config) = config.reconnect_config() {
            tracing::debug!(?reconnect_config, "Auto-reconnect enabled");
            builder = builder.set_reconnect_config(reconnect_config);
        }

        let mut client = builder.connect().await?;

        let mut subscribe_request = build_subscribe_request(filters, &config);
        subscribe_request.from_slot = config.from_slot;

        tracing::debug!(
            has_transactions = !subscribe_request.transactions.is_empty(),
            transaction_filters = ?subscribe_request.transactions.keys().collect::<Vec<_>>(),
            has_blocks_meta = !subscribe_request.blocks_meta.is_empty(),
            blocks_meta_filters = ?subscribe_request.blocks_meta.keys().collect::<Vec<_>>(),
            has_slots = !subscribe_request.slots.is_empty(),
            slots_filters = ?subscribe_request.slots.keys().collect::<Vec<_>>(),
            from_slot = ?subscribe_request.from_slot,
            commitment = ?subscribe_request.commitment,
            "Subscribing to gRPC stream"
        );

        let (mut sink, stream) = client
            .subscribe_with_request(Some(subscribe_request))
            .await?;

        let mut stream = std::pin::pin!(stream);

        tracing::debug!("gRPC stream started");

        let mut accept_filter_updates = true;
        let mut pending_filters: Option<Filters> = None;
        let mut filter_updates_sent: u64 = 0;

        // A held set cannot wait on the stream to produce again. The filter it
        // is replacing may match nothing, and the client swallows its own
        // keepalive messages rather than yielding them, so there are live
        // connections on which no update ever arrives to retry from.
        let mut retry = tokio::time::interval(RETRY_HELD_FILTERS_EVERY);
        retry.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        let exit_status = loop {
            tokio::select! {
                update = stream.next() => match update {
                    Some(Ok(update)) => {
                        if tx.send(Ok(update)).await.is_err() {
                            tracing::info!("Receiver dropped, stopping source");
                            // Defensive only - normally unreachable because Signal/Buffer
                            // branch wins first when receiver drops.
                            break SourceExitStatus::ReceiverDropped;
                        }

                        // The stream producing again is the signal that any
                        // reconnect has landed and the sink has a live sender,
                        // so this is the moment to retry a held filter set.
                        if let Some(filters) = pending_filters.take() {
                            pending_filters =
                                send_filter_update(&mut sink, &config, filters).await;

                            if pending_filters.is_none() {
                                filter_updates_sent += 1;
                            }
                        }
                    },
                    Some(Err(status)) => {
                        // A server that rejects a filter set answers on the stream
                        // rather than the sink, and the codes it uses for that are
                        // not recoverable, so this is where a bad update surfaces.
                        // Report the count so an operator can tell that apart from
                        // an unrelated server error.
                        tracing::warn!(
                            code = ?status.code(),
                            message = %status.message(),
                            filter_updates_sent,
                            "Received error status from stream"
                        );
                        let code = status.code();
                        let message = status.message().to_string();
                        let _ = tx.send(Err(status)).await;
                        break SourceExitStatus::StreamError { code, message };
                    },
                    None => {
                        break SourceExitStatus::StreamEnded;
                    },
                },

                _ = retry.tick(), if pending_filters.is_some() => {
                    if let Some(filters) = pending_filters.take() {
                        pending_filters = send_filter_update(&mut sink, &config, filters).await;

                        if pending_filters.is_none() {
                            filter_updates_sent += 1;
                        }
                    }
                },

                update = filter_updates.recv(), if accept_filter_updates => {
                    let Some(filters) = update else {
                        // Nobody holds the sending half any more, so leave this
                        // branch alone for the rest of the connection.
                        accept_filter_updates = false;
                        continue;
                    };

                    // A newer set supersedes anything still held, because every
                    // set is complete rather than a delta.
                    pending_filters = send_filter_update(&mut sink, &config, filters).await;

                    if pending_filters.is_none() {
                        filter_updates_sent += 1;
                    }
                },
            }
        };

        if pending_filters.is_some() {
            tracing::warn!("Connection ended with a filter update still unsent");
        }

        let _ = status_tx.send(exit_status);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use shipstern_core::Filters;

    use super::{build_subscribe_request, CommitmentLevel, YellowstoneGrpcConfig};

    fn config_from(toml_src: &str) -> YellowstoneGrpcConfig {
        toml::from_str(toml_src).expect("config must deserialize")
    }

    /// The startup subscribe and every later filter update are built by the
    /// same function, so the commitment `Filters` does not carry lands on both
    /// rather than only on the initial request.
    #[test]
    fn subscribe_request_carries_commitment() {
        let config = config_from(
            r#"
            endpoint = "https://example.rpcpool.com"
            timeout = 60
            commitment-level = "finalized"
        "#,
        );

        let request = build_subscribe_request(Filters::new(HashMap::new()), &config);

        assert_eq!(request.commitment, Some(CommitmentLevel::Finalized as i32));
    }

    /// A configured `from-slot` must never reach a mid-stream update. Servers
    /// read it as a replay request, so repeating it would replay the whole gap
    /// or end the stream outright. Only the initial subscribe sets it, and it
    /// is set at that call site rather than here.
    #[test]
    fn subscribe_request_omits_from_slot() {
        let config = config_from(
            r#"
            endpoint = "https://example.rpcpool.com"
            timeout = 60
            from-slot = 350000000
        "#,
        );

        let request = build_subscribe_request(Filters::new(HashMap::new()), &config);

        assert_eq!(request.from_slot, None);
    }

    /// Without a commitment the request keeps what the `Filters` conversion
    /// produced, which leaves it unset.
    #[test]
    fn subscribe_request_omits_unset_commitment() {
        let config = config_from(
            r#"
            endpoint = "https://example.rpcpool.com"
            timeout = 60
        "#,
        );

        let request = build_subscribe_request(Filters::new(HashMap::new()), &config);

        assert_eq!(request.commitment, None);
    }

    /// A config file predating the reconnect fields must still deserialize:
    /// missing `Option` keys become `None`, and the missing `auto-reconnect`
    /// key defaults to `true` via `#[serde(default = "default_auto_reconnect")]`,
    /// so legacy configs get auto-reconnect.
    #[test]
    fn deserializes_legacy_config_with_reconnect_on_by_default() {
        let legacy = r#"
            endpoint = "https://example.rpcpool.com"
            x-token = "secret"
            timeout = 60
        "#;

        let config: YellowstoneGrpcConfig =
            toml::from_str(legacy).expect("legacy config must deserialize");

        assert!(config.auto_reconnect);
        assert!(config.reconnect_config().is_some());
    }

    /// Auto-reconnect can be explicitly disabled.
    #[test]
    fn reconnect_can_be_disabled() {
        let disabled = r#"
            endpoint = "https://example.rpcpool.com"
            timeout = 60
            auto-reconnect = false
        "#;

        let config: YellowstoneGrpcConfig =
            toml::from_str(disabled).expect("config must deserialize");

        assert!(!config.auto_reconnect);
        assert!(config.reconnect_config().is_none());
    }

    /// With no overrides, the helper yields the sturdy built-in defaults
    /// (not the weak library defaults), and keeps the library slot_retention.
    #[test]
    fn reconnect_config_uses_sturdy_defaults() {
        let config: YellowstoneGrpcConfig = toml::from_str(
            r#"
            endpoint = "https://example.rpcpool.com"
            timeout = 60
            auto-reconnect = true
        "#,
        )
        .expect("config must deserialize");

        let reconnect = config.reconnect_config().expect("auto-reconnect enabled");

        assert_eq!(
            reconnect.backoff.max_retries,
            super::DEFAULT_RECONNECT_MAX_RETRIES
        );
        assert_eq!(
            reconnect.backoff.multiplier,
            super::DEFAULT_RECONNECT_MULTIPLIER
        );
        assert_eq!(
            reconnect.backoff.initial_interval,
            super::DEFAULT_RECONNECT_INITIAL_BACKOFF
        );
        // slot_retention is not overridden, so it keeps the library default.
        assert_eq!(
            reconnect.slot_retention,
            yellowstone_grpc_client::ReconnectConfig::default().slot_retention
        );
    }

    /// Config overrides win over the built-in defaults.
    #[test]
    fn reconnect_config_applies_overrides() {
        let config: YellowstoneGrpcConfig = toml::from_str(
            r#"
            endpoint = "https://example.rpcpool.com"
            timeout = 60
            auto-reconnect = true
            reconnect-max-retries = 25
            reconnect-slot-retention = 300
        "#,
        )
        .expect("config must deserialize");

        let reconnect = config.reconnect_config().expect("auto-reconnect enabled");

        assert_eq!(reconnect.backoff.max_retries, 25);
        assert_eq!(reconnect.slot_retention, 300);
    }
}
