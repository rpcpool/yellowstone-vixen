#![deny(
    clippy::disallowed_methods,
    clippy::suspicious,
    clippy::style,
    clippy::clone_on_ref_ptr
)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    path::PathBuf,
    time::{Duration, Instant},
};

use clap::Parser as _;
use futures_util::StreamExt;
use tokio::time::MissedTickBehavior;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient};
use yellowstone_grpc_proto::geyser::{
    subscribe_update::UpdateOneof, SlotStatus, SubscribeRequest, SubscribeRequestFilterBlocksMeta,
    SubscribeRequestFilterEntry, SubscribeRequestFilterSlots, SubscribeRequestFilterTransactions,
    SubscribeUpdate,
};
use yellowstone_vixen_yellowstone_grpc_source::YellowstoneGrpcConfig;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum Mode {
    Continuous,
    SubscriptionIdle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum SubscriptionProfile {
    Full,
    SlotUpdatesOnly,
}

#[derive(clap::Parser)]
#[command(
    version,
    author,
    about = "Minimal Yellowstone gRPC client for reproducing idle stream behavior"
)]
struct Opts {
    /// Path to a Vixen TOML config with a [source] section.
    #[arg(long, short)]
    config: PathBuf,

    /// Warn when no messages have been received for this many seconds.
    #[arg(long, default_value_t = 5)]
    idle_warn_secs: u64,

    /// Emit a summary log every N updates.
    #[arg(long, default_value_t = 1_000)]
    log_every: u64,

    /// Run continuously, or keep resubscribing until the stream goes idle during its startup window.
    #[arg(long, value_enum, default_value_t = Mode::Continuous)]
    mode: Mode,

    /// Subscribe to the full filter set, or only the slot update filter.
    #[arg(long, value_enum, default_value_t = SubscriptionProfile::Full)]
    subscription_profile: SubscriptionProfile,

    /// Delay between subscription attempts in subscription-idle mode.
    #[arg(long, default_value_t = 1000)]
    resubscribe_delay_ms: u64,

    /// How long to watch a fresh subscription for startup-idle before retrying it.
    /// Defaults to 2x `--idle-warn-secs`.
    #[arg(long)]
    subscription_idle_window_secs: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
struct FileConfig {
    source: YellowstoneGrpcConfig,
}

#[derive(Debug, Default)]
struct Stats {
    total: u64,
    entries: u64,
    block_meta: u64,
    slot_status: u64,
    transactions: u64,
    pings: u64,
    pongs: u64,
    other: u64,
    last_seen_slot: Option<u64>,
    slot_status_counts: BTreeMap<String, u64>,
    seen_event_types: BTreeSet<&'static str>,
    seen_slot_statuses: BTreeSet<String>,
}

#[derive(Debug)]
struct Observation {
    event: &'static str,
    slot: Option<u64>,
    first_event: bool,
    first_slot_status: Option<String>,
}

impl Stats {
    fn record(&mut self, update: &SubscribeUpdate) -> Observation {
        self.total += 1;

        let mut event = "other";
        let mut slot = None;
        let mut first_slot_status = None;

        match update.update_oneof.as_ref() {
            Some(UpdateOneof::Entry(entry)) => {
                self.entries += 1;
                event = "entries";
                slot = Some(entry.slot);
            },
            Some(UpdateOneof::BlockMeta(block_meta)) => {
                self.block_meta += 1;
                event = "blockMeta";
                slot = Some(block_meta.slot);
            },
            Some(UpdateOneof::Slot(slot_update)) => {
                self.slot_status += 1;
                event = "slotStatus";
                slot = Some(slot_update.slot);

                let status = SlotStatus::try_from(slot_update.status).map_or_else(
                    |_| format!("Unknown({})", slot_update.status),
                    |slot_status| format!("{slot_status:?}"),
                );

                *self.slot_status_counts.entry(status.clone()).or_default() += 1;
                if self.seen_slot_statuses.insert(status.clone()) {
                    first_slot_status = Some(status);
                }
            },
            Some(UpdateOneof::Transaction(tx)) => {
                self.transactions += 1;
                event = "transactions";
                slot = Some(tx.slot);
            },
            Some(UpdateOneof::Ping(_)) => {
                self.pings += 1;
                event = "ping";
            },
            Some(UpdateOneof::Pong(_)) => {
                self.pongs += 1;
                event = "pong";
            },
            Some(_) | None => {
                self.other += 1;
            },
        }

        if let Some(slot) = slot {
            self.last_seen_slot = Some(slot);
        }

        Observation {
            event,
            slot,
            first_event: self.seen_event_types.insert(event),
            first_slot_status,
        }
    }
}

fn duration_millis_u64(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}

fn update_event_name(update: &SubscribeUpdate) -> &'static str {
    match update.update_oneof.as_ref() {
        Some(UpdateOneof::Entry(_)) => "entries",
        Some(UpdateOneof::BlockMeta(_)) => "blockMeta",
        Some(UpdateOneof::Slot(_)) => "slotStatus",
        Some(UpdateOneof::Transaction(_)) => "transactions",
        Some(UpdateOneof::Ping(_)) => "ping",
        Some(UpdateOneof::Pong(_)) => "pong",
        Some(_) | None => "other",
    }
}

#[allow(clippy::zero_sized_map_values)]
fn filter_map<T>(name: &str, filter: T) -> HashMap<String, T> {
    HashMap::from([(name.to_string(), filter)])
}

#[allow(clippy::zero_sized_map_values)]
fn empty_filter_map<T>() -> HashMap<String, T> { HashMap::new() }

fn build_subscribe_request(
    config: &YellowstoneGrpcConfig,
    subscription_profile: SubscriptionProfile,
) -> SubscribeRequest {
    let slots = filter_map("slotStatus", SubscribeRequestFilterSlots {
        filter_by_commitment: None,
        interslot_updates: Some(true),
    });

    let (transactions, blocks_meta, entry) = match subscription_profile {
        SubscriptionProfile::Full => (
            filter_map("transactions", SubscribeRequestFilterTransactions {
                vote: None,
                failed: None,
                signature: None,
                account_include: vec![],
                account_exclude: vec![],
                account_required: vec![],
            }),
            filter_map("blockMeta", SubscribeRequestFilterBlocksMeta {}),
            filter_map("entries", SubscribeRequestFilterEntry {}),
        ),
        SubscriptionProfile::SlotUpdatesOnly => {
            (empty_filter_map(), empty_filter_map(), empty_filter_map())
        },
    };

    SubscribeRequest {
        accounts: empty_filter_map(),
        // Keep slot lifecycle events unfiltered so idle diagnosis sees intra-slot transitions.
        slots,
        transactions,
        transactions_status: empty_filter_map(),
        blocks: empty_filter_map(),
        blocks_meta,
        entry,
        commitment: config.commitment_level.map(|level| level as i32),
        accounts_data_slice: vec![],
        ping: None,
        from_slot: config.from_slot,
    }
}

fn log_summary(stats: &Stats) {
    tracing::debug!(
        total = stats.total,
        entries = stats.entries,
        block_meta = stats.block_meta,
        slot_status = stats.slot_status,
        transactions = stats.transactions,
        pings = stats.pings,
        pongs = stats.pongs,
        other = stats.other,
        last_seen_slot = ?stats.last_seen_slot,
        slot_status_counts = ?stats.slot_status_counts,
        "stream summary"
    );
}

async fn probe_version(
    client: &mut GeyserGrpcClient<impl yellowstone_grpc_client::Interceptor + Clone>,
    attempt: u64,
    config: &YellowstoneGrpcConfig,
) {
    match client.get_version().await {
        Ok(response) => tracing::info!(
            attempt,
            endpoint = %config.endpoint,
            version = %response.version,
            "connected to gRPC endpoint"
        ),
        Err(error) => tracing::warn!(
            attempt,
            endpoint = %config.endpoint,
            error = %error,
            "connected to gRPC endpoint but version probe failed"
        ),
    }
}

fn log_subscription_request(
    attempt: u64,
    config: &YellowstoneGrpcConfig,
    subscribe_request: &SubscribeRequest,
    subscription_profile: SubscriptionProfile,
    idle_warn_secs: u64,
    log_every: u64,
) {
    tracing::info!(
        attempt,
        endpoint = %config.endpoint,
        subscription_profile = ?subscription_profile,
        has_entries = !subscribe_request.entry.is_empty(),
        has_blocks_meta = !subscribe_request.blocks_meta.is_empty(),
        has_slots = !subscribe_request.slots.is_empty(),
        has_transactions = !subscribe_request.transactions.is_empty(),
        from_slot = ?subscribe_request.from_slot,
        commitment = ?subscribe_request.commitment,
        idle_warn_secs,
        log_every,
        "subscribing to gRPC stream"
    );
}

async fn connect_client(
    config: &YellowstoneGrpcConfig,
) -> Result<GeyserGrpcClient<impl yellowstone_grpc_client::Interceptor + Clone>, DynError> {
    let timeout = Duration::from_secs(config.timeout);

    let client = GeyserGrpcClient::build_from_shared(config.endpoint.clone())?
        .x_token(config.x_token.clone())?
        .max_decoding_message_size(config.max_decoding_message_size.unwrap_or(usize::MAX))
        .accept_compressed(config.accept_compression.unwrap_or_default().into())
        .connect_timeout(timeout)
        .timeout(timeout)
        .initial_stream_window_size(16 * 1024 * 1024)        // 16 MiB
        .initial_connection_window_size(32 * 1024 * 1024)    // 32 MiB
        .http2_keep_alive_interval(Duration::from_secs(10))  // send HTTP/2 PING every 10s
        .keep_alive_timeout(Duration::from_secs(20))         // fail if no PING ACK within 20s
        .keep_alive_while_idle(true)                         // ping even when no RPCs in-flight
        .tcp_keepalive(Some(Duration::from_secs(15)))        // OS-level TCP keepalive too
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .connect()
        .await?;

    Ok(client)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AttemptOutcome {
    Finished,
    Retry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StartupWindowStatus {
    Open,
    Elapsed,
    Disabled,
}

fn startup_window_status(
    subscription_age: Duration,
    subscription_idle_window: Option<Duration>,
) -> StartupWindowStatus {
    match subscription_idle_window {
        Some(window) if subscription_age <= window => StartupWindowStatus::Open,
        Some(_) => StartupWindowStatus::Elapsed,
        None => StartupWindowStatus::Disabled,
    }
}

#[derive(Debug)]
struct AttemptState {
    stats: Stats,
    last_update_at: Instant,
    idle_since: Option<Instant>,
    total_updates_at_idle_warn: Option<u64>,
    startup_idle_observed: bool,
    attempt_started_at: Instant,
}

impl AttemptState {
    fn new() -> Self {
        Self {
            stats: Stats::default(),
            last_update_at: Instant::now(),
            idle_since: None,
            total_updates_at_idle_warn: None,
            startup_idle_observed: false,
            attempt_started_at: Instant::now(),
        }
    }

    fn subscription_age(&self) -> Duration { self.attempt_started_at.elapsed() }

    fn idle_for(&self) -> Duration { self.last_update_at.elapsed() }

    fn total_idle_ms(&self) -> u64 { duration_millis_u64(self.idle_for()) }
}

fn handle_idle_tick(
    state: &mut AttemptState,
    attempt: u64,
    idle_warn_secs: u64,
    mode: Mode,
    subscription_idle_window: Option<Duration>,
) -> Option<AttemptOutcome> {
    let subscription_age = state.subscription_age();
    let idle_for = state.idle_for();

    if idle_warn_secs > 0
        && state.idle_since.is_none()
        && idle_for >= Duration::from_secs(idle_warn_secs)
    {
        state.idle_since = Some(Instant::now());
        state.total_updates_at_idle_warn = Some(state.stats.total);

        let subscription_age_ms = duration_millis_u64(subscription_age);
        tracing::warn!(
            attempt,
            subscription_age_ms,
            idle_for_ms = duration_millis_u64(idle_for),
            idle_for_secs = idle_for.as_secs(),
            last_seen_slot = ?state.stats.last_seen_slot,
            total_updates = state.stats.total,
            "stream idle"
        );

        if mode == Mode::SubscriptionIdle && !state.startup_idle_observed {
            match startup_window_status(subscription_age, subscription_idle_window) {
                StartupWindowStatus::Open => {
                    state.startup_idle_observed = true;
                    tracing::warn!(
                        attempt,
                        subscription_age_ms,
                        total_updates = state.stats.total,
                        "subscription went idle during startup window; staying on this stream"
                    );
                },
                StartupWindowStatus::Elapsed => {
                    tracing::warn!(
                        attempt,
                        subscription_age_ms,
                        total_updates = state.stats.total,
                        "subscription went idle after startup window; retrying with a fresh \
                         subscription"
                    );
                    return Some(AttemptOutcome::Retry);
                },
                StartupWindowStatus::Disabled => {},
            }
        }
    }

    if mode == Mode::SubscriptionIdle
        && !state.startup_idle_observed
        && state.idle_since.is_none()
        && matches!(
            startup_window_status(subscription_age, subscription_idle_window),
            StartupWindowStatus::Elapsed
        )
    {
        tracing::warn!(
            attempt,
            subscription_age_ms = duration_millis_u64(subscription_age),
            total_updates = state.stats.total,
            "subscription did not go idle during startup window; retrying with a fresh \
             subscription"
        );
        return Some(AttemptOutcome::Retry);
    }

    None
}

fn handle_first_update_after_startup_idle(
    state: &mut AttemptState,
    attempt: u64,
    idle_warn_secs: u64,
    mode: Mode,
    subscription_idle_window: Option<Duration>,
    update: &SubscribeUpdate,
) -> Option<AttemptOutcome> {
    if mode != Mode::SubscriptionIdle
        || state.startup_idle_observed
        || idle_warn_secs == 0
        || state.stats.total != 0
    {
        return None;
    }

    let idle_warn_ms = duration_millis_u64(Duration::from_secs(idle_warn_secs));
    let total_idle_ms = state.total_idle_ms();
    if total_idle_ms < idle_warn_ms {
        return None;
    }

    match startup_window_status(state.subscription_age(), subscription_idle_window) {
        StartupWindowStatus::Open => {
            state.startup_idle_observed = true;
            tracing::warn!(
                attempt,
                startup_idle_ms = total_idle_ms,
                startup_idle_secs = total_idle_ms / 1000,
                first_event = update_event_name(update),
                "subscription was idle before first update; staying on this stream"
            );
            None
        },
        StartupWindowStatus::Elapsed => {
            tracing::warn!(
                attempt,
                startup_idle_ms = total_idle_ms,
                startup_idle_secs = total_idle_ms / 1000,
                first_event = update_event_name(update),
                "subscription was idle before first update, but after startup window; retrying \
                 with a fresh subscription"
            );
            Some(AttemptOutcome::Retry)
        },
        StartupWindowStatus::Disabled => None,
    }
}

fn record_update(state: &mut AttemptState, attempt: u64, update: &SubscribeUpdate, log_every: u64) {
    let previous_slot = state.stats.last_seen_slot;
    let total_idle_ms = state.total_idle_ms();
    let observation = state.stats.record(update);

    if let Some(started_at) = state.idle_since.take() {
        let total_updates_at_idle_warn = state.total_updates_at_idle_warn.take();
        tracing::warn!(
            attempt,
            total_idle_ms,
            post_warn_idle_ms = duration_millis_u64(started_at.elapsed()),
            previous_slot = ?previous_slot,
            resume_event_slot = ?observation.slot,
            slots_elapsed = ?previous_slot
                .zip(observation.slot)
                .map(|(previous, current)| current.saturating_sub(previous)),
            total_updates = state.stats.total,
            total_updates_at_idle_warn = ?total_updates_at_idle_warn,
            "stream resumed"
        );
    }

    state.last_update_at = Instant::now();

    if observation.first_event {
        tracing::debug!(
            attempt,
            event = observation.event,
            slot = ?observation.slot,
            filters = ?update.filters,
            "received first update for event type"
        );
    }

    if let Some(status) = observation.first_slot_status {
        tracing::debug!(
            attempt,
            slot = ?observation.slot,
            slot_status = %status,
            "received first update for slot status"
        );
    }

    if state.stats.total == 1 || state.stats.total.is_multiple_of(log_every.max(1)) {
        log_summary(&state.stats);
    }
}

async fn run_subscription_attempt(
    config: &YellowstoneGrpcConfig,
    idle_warn_secs: u64,
    log_every: u64,
    mode: Mode,
    subscription_profile: SubscriptionProfile,
    attempt: u64,
    subscription_idle_window: Option<Duration>,
) -> Result<AttemptOutcome, DynError> {
    let mut client = connect_client(config).await?;
    probe_version(&mut client, attempt, config).await;

    let subscribe_request = build_subscribe_request(config, subscription_profile);
    log_subscription_request(
        attempt,
        config,
        &subscribe_request,
        subscription_profile,
        idle_warn_secs,
        log_every,
    );

    let (_subscribe_tx, stream) = client
        .subscribe_with_request(Some(subscribe_request))
        .await?;
    let mut stream = Box::pin(stream);
    let mut state = AttemptState::new();
    let mut shutdown = std::pin::pin!(tokio::signal::ctrl_c());
    let mut idle_tick = tokio::time::interval(Duration::from_secs(1));
    idle_tick.set_missed_tick_behavior(MissedTickBehavior::Delay);

    let outcome = loop {
        tokio::select! {
            _ = &mut shutdown => {
                tracing::info!(attempt, "received Ctrl-C, shutting down");
                break AttemptOutcome::Finished;
            },
            _ = idle_tick.tick() => {
                if let Some(outcome) = handle_idle_tick(
                    &mut state,
                    attempt,
                    idle_warn_secs,
                    mode,
                    subscription_idle_window,
                ) {
                    break outcome;
                }
            },
            maybe_update = stream.next() => {
                match maybe_update {
                    Some(Ok(update)) => {
                        if let Some(outcome) = handle_first_update_after_startup_idle(
                            &mut state,
                            attempt,
                            idle_warn_secs,
                            mode,
                            subscription_idle_window,
                            &update,
                        ) {
                            break outcome;
                        }

                        record_update(&mut state, attempt, &update, log_every);
                    },
                    Some(Err(grpc_status)) => {
                        tracing::error!(
                            attempt,
                            code = ?grpc_status.code(),
                            message = %grpc_status.message(),
                            "gRPC stream returned an error"
                        );
                        return Err(grpc_status.into());
                    },
                    None => {
                        tracing::warn!(attempt, "gRPC stream ended");
                        break AttemptOutcome::Finished;
                    },
                }
            },
        }
    };

    log_summary(&state.stats);

    Ok(outcome)
}

#[tokio::main]
async fn main() -> Result<(), DynError> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Opts {
        config: config_path,
        idle_warn_secs,
        log_every,
        mode,
        subscription_profile,
        resubscribe_delay_ms,
        subscription_idle_window_secs,
    } = Opts::parse();

    if mode == Mode::SubscriptionIdle && idle_warn_secs == 0 {
        return Err("subscription-idle mode requires --idle-warn-secs > 0".into());
    }

    let config_text = std::fs::read_to_string(&config_path)?;
    let FileConfig { source: config } = toml::from_str(&config_text)?;

    let subscription_idle_window = if mode == Mode::SubscriptionIdle {
        Some(Duration::from_secs(
            subscription_idle_window_secs
                .unwrap_or_else(|| idle_warn_secs.saturating_mul(2).max(idle_warn_secs + 1)),
        ))
    } else {
        None
    };

    if mode == Mode::SubscriptionIdle {
        tracing::warn!(
            idle_warn_secs,
            subscription_idle_window_secs = subscription_idle_window
                .map(|window| window.as_secs())
                .unwrap_or_default(),
            resubscribe_delay_ms,
            "subscription-idle mode enabled; resubscribing until startup idle is observed"
        );
    }

    let mut attempt = 1;
    loop {
        match run_subscription_attempt(
            &config,
            idle_warn_secs,
            log_every,
            mode,
            subscription_profile,
            attempt,
            subscription_idle_window,
        )
        .await?
        {
            AttemptOutcome::Finished => break,
            AttemptOutcome::Retry => {
                tracing::info!(
                    attempt,
                    retry_delay_ms = resubscribe_delay_ms,
                    "restarting subscription attempt"
                );

                if resubscribe_delay_ms > 0 {
                    tokio::select! {
                        () = tokio::time::sleep(Duration::from_millis(resubscribe_delay_ms)) => {},
                        _ = tokio::signal::ctrl_c() => break,
                    }
                }

                attempt += 1;
            },
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use yellowstone_vixen_yellowstone_grpc_source::YellowstoneGrpcConfig;

    use super::{
        build_subscribe_request, startup_window_status, StartupWindowStatus, SubscriptionProfile,
    };

    fn config() -> YellowstoneGrpcConfig {
        YellowstoneGrpcConfig {
            endpoint: "https://example.invalid".to_string(),
            x_token: None,
            timeout: 60,
            commitment_level: None,
            from_slot: Some(42),
            max_decoding_message_size: None,
            accept_compression: None,
        }
    }

    #[test]
    fn startup_window_is_open_before_deadline() {
        assert_eq!(
            startup_window_status(Duration::from_secs(4), Some(Duration::from_secs(5))),
            StartupWindowStatus::Open
        );
    }

    #[test]
    fn startup_window_expires_after_deadline() {
        assert_eq!(
            startup_window_status(Duration::from_secs(6), Some(Duration::from_secs(5))),
            StartupWindowStatus::Elapsed
        );
    }

    #[test]
    fn builds_full_subscription_request() {
        let request = build_subscribe_request(&config(), SubscriptionProfile::Full);

        assert!(!request.transactions.is_empty());
        assert!(!request.blocks_meta.is_empty());
        assert!(!request.entry.is_empty());
        assert!(!request.slots.is_empty());
        assert_eq!(request.from_slot, Some(42));
    }

    #[test]
    fn builds_slot_updates_only_subscription_request() {
        let request = build_subscribe_request(&config(), SubscriptionProfile::SlotUpdatesOnly);

        assert!(request.transactions.is_empty());
        assert!(request.blocks_meta.is_empty());
        assert!(request.entry.is_empty());
        assert_eq!(request.slots.len(), 1);
        assert_eq!(request.from_slot, Some(42));
    }
}
