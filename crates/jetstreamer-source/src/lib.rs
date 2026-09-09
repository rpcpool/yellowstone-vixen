use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use futures_util::FutureExt;
use jetstreamer_firehose::firehose::{firehose, BlockData, EntryData, OnErrorFn, TransactionData};
use shipstern::{
    sources::{SourceExitStatus, SourceTrait},
    Error as ShipsternError,
};
use shipstern_core::Filters;
use tokio::sync::{broadcast, mpsc, mpsc::Sender, oneshot};
use tracing::{debug, error, info};
use yellowstone_grpc_proto::{
    geyser::{
        subscribe_update::UpdateOneof, SlotStatus, SubscribeUpdate, SubscribeUpdateBlock,
        SubscribeUpdateBlockMeta, SubscribeUpdateSlot,
    },
    solana::storage::confirmed_block::{BlockHeight, UnixTimestamp},
};

type SharedError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Dedicated side-channel event surfaced for skipped slots during backfill.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PossibleLeaderSkippedEvent {
    pub slot: u64,
}

/// Env vars that `jetstreamer-firehose` reads at startup.
///
/// Because `std::env::set_var` is unsound once other threads exist,
/// callers must apply these **before** the async runtime starts.
/// [`connect`] then validates that the process env still matches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessEnvConfig {
    pub network: String,
    pub compact_index_base_url: String,
    pub network_capacity_mb: String,
}

impl ProcessEnvConfig {
    /// Extract the expected env configuration from a [`JetstreamSourceConfig`].
    pub fn from_config(config: &JetstreamSourceConfig) -> Self {
        Self {
            network: config.network.clone(),
            compact_index_base_url: config.compact_index_base_url.clone(),
            network_capacity_mb: config.network_capacity_mb.to_string(),
        }
    }

    /// Snapshot the *current* process environment.
    pub fn from_process() -> Self {
        Self {
            network: std::env::var("JETSTREAMER_NETWORK").unwrap_or_default(),
            compact_index_base_url: std::env::var("JETSTREAMER_COMPACT_INDEX_BASE_URL")
                .unwrap_or_default(),
            network_capacity_mb: std::env::var("JETSTREAMER_NETWORK_CAPACITY_MB")
                .unwrap_or_default(),
        }
    }

    /// Write the values into the process environment.
    ///
    /// # Safety
    ///
    /// Must be called while no other threads are running (i.e. before
    /// the Tokio runtime is created). Calling this after other threads
    /// exist is undefined behaviour on most platforms.
    pub unsafe fn apply(&self) {
        unsafe {
            std::env::set_var("JETSTREAMER_NETWORK", &self.network);
            std::env::set_var(
                "JETSTREAMER_COMPACT_INDEX_BASE_URL",
                &self.compact_index_base_url,
            );
            std::env::set_var("JETSTREAMER_NETWORK_CAPACITY_MB", &self.network_capacity_mb);
        }
    }

    /// Return `Ok(())` if `self` matches `actual`, or a descriptive error.
    pub fn validate_matches(&self, actual: &ProcessEnvConfig) -> Result<(), Error> {
        let mut mismatches = Vec::new();

        if self.network != actual.network {
            mismatches.push(format!(
                "JETSTREAMER_NETWORK: expected {:?}, got {:?}",
                self.network, actual.network
            ));
        }
        if self.compact_index_base_url != actual.compact_index_base_url {
            mismatches.push(format!(
                "JETSTREAMER_COMPACT_INDEX_BASE_URL: expected {:?}, got {:?}",
                self.compact_index_base_url, actual.compact_index_base_url
            ));
        }
        if self.network_capacity_mb != actual.network_capacity_mb {
            mismatches.push(format!(
                "JETSTREAMER_NETWORK_CAPACITY_MB: expected {:?}, got {:?}",
                self.network_capacity_mb, actual.network_capacity_mb
            ));
        }

        if mismatches.is_empty() {
            Ok(())
        } else {
            Err(Error::EnvMismatch(mismatches.join("; ")))
        }
    }
}

/// Set the jetstreamer-firehose env vars from `config`.
///
/// # Safety
///
/// Must be called **before** the Tokio runtime (or any other threads)
/// are started. The canonical call-site is the top of `fn main()`,
/// before `#[tokio::main]` or `Runtime::new()`.
///
/// ## Example
///
/// ```rust, ignore
/// fn main() -> anyhow::Result<()> {
///     // … parse CLI / config …
///     unsafe { shipstern_jetstream_source::init_process_env(&config) };
///     tokio_main(config)
/// }
///
/// #[tokio::main]
/// async fn tokio_main(config: JetstreamSourceConfig) -> anyhow::Result<()> {
///     // … build runtime, connect, etc. …
/// }
/// ```
pub unsafe fn init_process_env(config: &JetstreamSourceConfig) {
    let env_config = ProcessEnvConfig::from_config(config);
    unsafe { env_config.apply() };
}

/// Filter IDs bucketed by the `UpdateOneof` variant each one actually consumes.
///
/// The runtime dispatches by variant: `UpdateOneof::Block` only ever reaches
/// block pipelines, `BlockMeta` only block-meta pipelines, and so on. A
/// `block_meta` filter ID riding along on a `Block` update matches no pipeline
/// and is dropped, so every bucket needs its own emission.
#[derive(Debug, Default)]
struct FilterMatches {
    block: Vec<String>,
    block_meta: Vec<String>,
    slot: Vec<String>,
    transaction: Vec<String>,
    /// True iff any block filter requested `include_entries`. When false we
    /// skip the per-entry buffering work entirely.
    wants_entries: bool,
}

struct ShipsternStreamHandler {
    tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
    skipped_slots_tx: Option<mpsc::Sender<PossibleLeaderSkippedEvent>>,
    // Cache matching filters to avoid iteration per item
    block_matches: Vec<String>,
    block_meta_matches: Vec<String>,
    slot_matches: Vec<String>,
    transaction_matches: Vec<String>,
    wants_entries: bool,
    // Per-slot buffer of entries arriving via `on_entry`. Drained when the
    // matching `BlockData::Block` is emitted. Upstream `firehose` emits all
    // entries for a slot on a single thread before the slot's block message,
    // so slot is a sufficient key.
    entry_buffer: Mutex<HashMap<u64, Vec<EntryData>>>,
}

impl ShipsternStreamHandler {
    fn new(
        tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
        skipped_slots_tx: Option<mpsc::Sender<PossibleLeaderSkippedEvent>>,
        filters: Filters,
    ) -> Self {
        let matches = Self::precalculate_filters(&filters);

        info!(
            block_filters = matches.block.len(),
            block_meta_filters = matches.block_meta.len(),
            slot_filters = matches.slot.len(),
            transaction_filters = matches.transaction.len(),
            wants_entries = matches.wants_entries,
            "Initialized ShipsternStreamHandler with cached filters"
        );

        let FilterMatches {
            block: block_matches,
            block_meta: block_meta_matches,
            slot: slot_matches,
            transaction: transaction_matches,
            wants_entries,
        } = matches;

        Self {
            tx,
            skipped_slots_tx,
            block_matches,
            block_meta_matches,
            slot_matches,
            transaction_matches,
            wants_entries,
            entry_buffer: Mutex::new(HashMap::new()),
        }
    }

    fn precalculate_filters(filters: &Filters) -> FilterMatches {
        let mut matches = FilterMatches::default();

        for (filter_id, prefilter) in &filters.parsers_filters {
            // 1. Block matches. Only a `block` prefilter consumes `UpdateOneof::Block`.
            if let Some(block_filter) = &prefilter.block
                && (block_filter.include_transactions
                    || block_filter.include_accounts
                    || block_filter.include_entries)
            {
                matches.block.push(filter_id.clone());
                if block_filter.include_entries {
                    matches.wants_entries = true;
                }
            }

            // 2. Block-meta and slot matches get their own updates. Bucketing them
            // under `block` would attach their IDs to `UpdateOneof::Block`, which
            // the runtime routes only to block pipelines, dropping them silently.
            if prefilter.block_meta.is_some() {
                matches.block_meta.push(filter_id.clone());
            }
            if prefilter.slot.is_some() {
                matches.slot.push(filter_id.clone());
            }

            // 3. Calculate Transaction Matches
            // Instruction parsers need transactions to extract instructions from,
            // so any parser with a transaction filter must receive all transactions.
            // The jetstreamer-firehose API does not support per-account filtering,
            // so we include all transactions whenever a transaction filter is present.
            if prefilter.transaction.is_some() {
                matches.transaction.push(filter_id.clone());
            }
        }

        matches
    }

    /// Wrap `update_oneof` in a [`SubscribeUpdate`] addressed to `filters` and
    /// hand it to the runtime, mapping a closed channel to [`Error::ChannelSend`].
    async fn send(
        &self,
        filters: Vec<String>,
        update_oneof: UpdateOneof,
    ) -> Result<(), SharedError> {
        let update = SubscribeUpdate {
            filters,
            update_oneof: Some(update_oneof),
            created_at: Some(yellowstone_grpc_proto::prost_types::Timestamp::from(
                std::time::SystemTime::now(),
            )),
        };

        self.tx.send(Ok(update)).await.map_err(|e| {
            let error_msg = format!("Failed to send update: {}", e);
            error!("{}", error_msg);
            Box::new(Error::ChannelSend(error_msg)) as SharedError
        })
    }

    /// Lock `entry_buffer`, converting a poisoned mutex into a structured
    /// error instead of panicking inside the hot streaming loop.
    fn lock_entry_buffer(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, HashMap<u64, Vec<EntryData>>>, SharedError> {
        self.entry_buffer.lock().map_err(|poison| {
            error!(
                error = %poison,
                "entry_buffer mutex poisoned; another task panicked while holding the lock"
            );
            Box::new(Error::EntryBufferPoisoned(poison.to_string())) as SharedError
        })
    }

    async fn process_entry(&self, entry: EntryData) -> Result<(), SharedError> {
        if !self.wants_entries {
            return Ok(());
        }

        let slot = entry.slot;
        {
            let mut buf = self.lock_entry_buffer()?;
            buf.entry(slot).or_default().push(entry);
        }

        Ok(())
    }

    async fn process_block(&self, block: BlockData) -> Result<(), SharedError> {
        debug!(slot = block.slot(), "Processing block");

        match block {
            BlockData::Block {
                parent_slot,
                parent_blockhash,
                slot,
                blockhash,
                rewards,
                block_time,
                block_height,
                executed_transaction_count,
                entry_count,
            } => {
                // Always drain, even when nothing consumes the entries: a slot
                // whose buffer is left behind pins its entries for the rest of
                // the run.
                let buffered = if self.wants_entries {
                    self.lock_entry_buffer()?.remove(&slot).unwrap_or_default()
                } else {
                    Vec::new()
                };

                if self.block_matches.is_empty()
                    && self.block_meta_matches.is_empty()
                    && self.slot_matches.is_empty()
                {
                    debug!(
                        slot,
                        "No block, block-meta, or slot filters interested; skipping"
                    );
                    return Ok(());
                }

                if !self.block_matches.is_empty() {
                    if self.wants_entries && buffered.len() as u64 != entry_count {
                        debug!(
                            slot,
                            buffered = buffered.len(),
                            entry_count,
                            "Buffered entry count differs from block entry_count"
                        );
                    }

                    debug!(
                        slot,
                        filters = ?self.block_matches,
                        "Sending block update with {} filter matches",
                        self.block_matches.len()
                    );

                    self.send(
                        self.block_matches.clone(),
                        UpdateOneof::Block(SubscribeUpdateBlock {
                            slot,
                            blockhash: blockhash.to_string(),
                            rewards: Some(convert::keyed_rewards(&rewards)),
                            block_time: block_time.map(|bt| UnixTimestamp { timestamp: bt }),
                            block_height: block_height.map(|bh| BlockHeight { block_height: bh }),
                            executed_transaction_count,
                            transactions: vec![],
                            updated_account_count: 0,
                            accounts: vec![],
                            entries: convert::entries(buffered),
                            entries_count: entry_count,
                            parent_slot,
                            parent_blockhash: parent_blockhash.to_string(),
                        }),
                    )
                    .await?;
                }

                if !self.block_meta_matches.is_empty() {
                    debug!(
                        slot,
                        filters = ?self.block_meta_matches,
                        "Sending block meta update with {} filter matches",
                        self.block_meta_matches.len()
                    );

                    self.send(
                        self.block_meta_matches.clone(),
                        UpdateOneof::BlockMeta(SubscribeUpdateBlockMeta {
                            slot,
                            blockhash: blockhash.to_string(),
                            rewards: Some(convert::keyed_rewards(&rewards)),
                            block_time: block_time.map(|bt| UnixTimestamp { timestamp: bt }),
                            block_height: block_height.map(|bh| BlockHeight { block_height: bh }),
                            parent_slot,
                            parent_blockhash: parent_blockhash.to_string(),
                            executed_transaction_count,
                            entries_count: entry_count,
                        }),
                    )
                    .await?;
                }

                if !self.slot_matches.is_empty() {
                    debug!(
                        slot,
                        filters = ?self.slot_matches,
                        "Sending slot update with {} filter matches",
                        self.slot_matches.len()
                    );

                    // Old Faithful archives only carry finalized history, so a
                    // replayed slot has exactly one status transition to report.
                    // `SlotPrefilter::filter_by_commitment = false` cannot yield
                    // the intermediate processed/confirmed/dead transitions here.
                    self.send(
                        self.slot_matches.clone(),
                        UpdateOneof::Slot(SubscribeUpdateSlot {
                            slot,
                            parent: Some(parent_slot),
                            status: SlotStatus::SlotFinalized as i32,
                            dead_error: None,
                        }),
                    )
                    .await?;
                }
            },
            BlockData::PossibleLeaderSkipped { slot } => {
                debug!(
                    slot,
                    "Surfacing possibly leader-skipped slot on side channel"
                );

                if let Some(skipped_slots_tx) = &self.skipped_slots_tx {
                    skipped_slots_tx
                        .send(PossibleLeaderSkippedEvent { slot })
                        .await
                        .map_err(|e| {
                            let error_msg =
                                format!("Failed to send possible leader-skipped slot: {}", e);
                            error!("{}", error_msg);
                            Box::new(Error::ChannelSend(error_msg)) as SharedError
                        })?;
                }
            },
        }

        Ok(())
    }

    async fn process_transaction(&self, tx_data: TransactionData) -> Result<(), SharedError> {
        debug!(
            signature = ?tx_data.signature,
            slot = tx_data.slot,
            index = tx_data.transaction_slot_index,
            is_vote = tx_data.is_vote,
            "Processing transaction"
        );

        // Use cached matches
        if self.transaction_matches.is_empty() {
            debug!(
                signature = ?tx_data.signature,
                slot = tx_data.slot,
                "No filters matched, skipping transaction"
            );
            return Ok(());
        }

        // Create transaction info structure
        let transaction_info = Some(
            yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo {
                signature: tx_data.signature.as_ref().to_vec(),
                is_vote: tx_data.is_vote,
                transaction: Some(convert::transaction(tx_data.transaction)),
                meta: Some(convert::transaction_status_meta(
                    tx_data.transaction_status_meta,
                )),
                index: tx_data.transaction_slot_index as u64,
            },
        );

        debug!(
            slot = tx_data.slot,
            filters = ?self.transaction_matches,
            "Sending transaction update with {} filter matches",
            self.transaction_matches.len()
        );

        self.send(
            self.transaction_matches.clone(),
            UpdateOneof::Transaction(yellowstone_grpc_proto::geyser::SubscribeUpdateTransaction {
                slot: tx_data.slot,
                transaction: transaction_info,
            }),
        )
        .await
    }
}

/// Jetstream source configuration
#[derive(Debug, Clone, serde::Deserialize, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct JetstreamSourceConfig {
    /// Old Faithful archive URL
    #[arg(long, env)]
    pub archive_url: String,

    /// Slot range configuration
    #[command(flatten)]
    pub range: SlotRangeConfig,

    /// Number of parallel threads
    #[arg(long, env, default_value = "4")]
    pub threads: usize,

    /// Network name (mainnet, testnet, devnet)
    #[arg(long, env, default_value = "mainnet")]
    pub network: String,

    /// Compact index base URL
    #[arg(long, env, default_value = "https://files.old-faithful.net")]
    pub compact_index_base_url: String,

    /// Network capacity in MB
    #[arg(long, env, default_value = "1000")]
    pub network_capacity_mb: usize,

    /// Sequential mode: single firehose worker thread with parallel ripget
    /// downloads. Required by upstream for the high-throughput (≥150k TPS)
    /// path; `threads` then configures ripget range concurrency.
    #[arg(long, env, default_value = "false")]
    #[serde(default)]
    pub sequential: bool,

    /// Process epochs from highest to lowest instead of lowest to highest.
    /// Slots *within* an epoch are still emitted in ascending order, because
    /// the underlying CAR archive can only be streamed forward.
    ///
    /// Upstream treats this as sequential-only and turns `sequential` on
    /// implicitly (`sequential || reverse`), so enabling `reverse` alone
    /// also forfeits the multi-threaded work-stealing path.
    #[arg(long, env, default_value = "false")]
    #[serde(default)]
    pub reverse: bool,

    /// Ripget hot/cold window size in bytes when sequential mode is active
    /// (`sequential` or `reverse` set). Ignored otherwise. `None` falls back to
    /// [`DEFAULT_SEQUENTIAL_BUFFER_WINDOW_BYTES`]; set it explicitly for a
    /// larger window on a fast link.
    #[arg(long, env)]
    #[serde(default)]
    pub buffer_window_bytes: Option<u64>,

    /// Emit a structured progress stats line every N slots, sourced from
    /// upstream `firehose` `StatsTracking` aggregates (blocks, transactions,
    /// entries, leader-skipped slots). `0` disables progress logging.
    #[arg(long, env, default_value = "10000")]
    #[serde(default = "default_stats_interval_slots")]
    pub stats_interval_slots: u64,

    /// Optional side channel for `PossibleLeaderSkipped` events.
    #[serde(skip)]
    #[arg(skip)]
    pub possible_leader_skipped_tx: Option<mpsc::Sender<PossibleLeaderSkippedEvent>>,

    /// Optional cooperative-shutdown signal forwarded to upstream
    /// `firehose()`. When the caller broadcasts `()` on the paired
    /// `broadcast::Sender`, the firehose loop unwinds at the next slot
    /// boundary instead of running to completion.
    ///
    /// We store a `Sender` (not a `Receiver`) so the config remains
    /// `Clone`; `connect()` calls `.subscribe()` to obtain its own
    /// receiver when wiring the firehose call.
    #[serde(skip)]
    #[arg(skip)]
    pub shutdown_signal_tx: Option<broadcast::Sender<()>>,
}

fn default_stats_interval_slots() -> u64 { 10_000 }

/// Ripget window used in sequential mode when the caller sets none.
///
/// Ripget fills the whole window before yielding the first block, and that
/// fill must finish inside upstream's fixed 180s `read_raw_header` timeout.
/// Upstream's default is `min(4 GiB, 15% of available RAM)`, which on a
/// high-RAM host cannot download in time, so the run times out and retries
/// without ever emitting a slot.
///
/// Measured against files.old-faithful.net at ~7 MiB/s, same 5-slot range:
///
/// ```text, ignore
///    64 MiB ->   9.5s
///   256 MiB ->  33.6s
///     1 GiB -> 145.6s
///   2.4 GiB -> never completes (180s timeout)
/// ```
pub const DEFAULT_SEQUENTIAL_BUFFER_WINDOW_BYTES: u64 = 256 * 1024 * 1024;

/// Resolve the ripget window handed to `firehose()`.
///
/// An explicit `configured` value is returned as-is; the default only applies
/// in sequential or reverse mode, where upstream would otherwise use its
/// RAM-derived one. Upstream discards a window below 2 (`filter(|v| *v >= 2)`),
/// so an explicit value wins only for `>= 2`.
///
/// Example output:
///
/// ```text, ignore
/// (seq: false, rev: false, cfg: None)       -> None
/// (seq: true,  rev: false, cfg: None)       -> Some(268435456)
/// (seq: false, rev: true,  cfg: None)       -> Some(268435456)
/// (seq: true,  rev: false, cfg: Some(4096)) -> Some(4096)
/// ```
pub fn effective_buffer_window_bytes(
    sequential: bool,
    reverse: bool,
    configured: Option<u64>,
) -> Option<u64> {
    configured.or_else(|| (sequential || reverse).then_some(DEFAULT_SEQUENTIAL_BUFFER_WINDOW_BYTES))
}

/// Configuration for slot ranges or epochs
#[derive(Debug, Clone, serde::Deserialize, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct SlotRangeConfig {
    /// Start slot (conflicts with epoch)
    #[arg(long, env, conflicts_with = "epoch")]
    pub slot_start: Option<u64>,

    /// End slot (requires slot_start, conflicts with epoch)
    #[arg(long, env, requires = "slot_start", conflicts_with = "epoch")]
    pub slot_end: Option<u64>,

    /// Epoch number (conflicts with slot_start)
    #[arg(long, env, conflicts_with = "slot_start")]
    pub epoch: Option<u64>,
}

impl SlotRangeConfig {
    /// Convert configuration to a half-open slot range.
    ///
    /// Returns `(start_slot, end_slot_exclusive)` — the range processed is
    /// `[start_slot, end_slot_exclusive)`, matching Rust's `start..end`
    /// semantics used by `firehose()`.
    ///
    /// - **Epoch mode**: covers all slots in the epoch.
    /// - **Explicit mode**: `slot_start` is inclusive, `slot_end` is
    ///   **exclusive** (the first slot *not* processed).
    pub fn to_slot_range(&self) -> Result<(u64, u64), Error> {
        match (self.slot_start, self.slot_end, self.epoch) {
            (Some(start), Some(end), None) => {
                if start >= end {
                    return Err(Error::InvalidConfig(
                        "slot_start must be < slot_end (slot_end is exclusive)".into(),
                    ));
                }
                Ok((start, end))
            },
            (None, None, Some(epoch)) => {
                const SLOTS_PER_EPOCH: u64 = 432_000;
                let start = epoch * SLOTS_PER_EPOCH;
                let end = (epoch + 1) * SLOTS_PER_EPOCH;
                info!(
                    epoch,
                    start_slot = start,
                    end_slot_exclusive = end,
                    "Resolved epoch to slot range"
                );
                Ok((start, end))
            },
            _ => Err(Error::InvalidConfig(
                "Must specify either (slot_start + slot_end) or epoch, not both".into(),
            )),
        }
    }
}

/// Jetstream source for historical Solana data streaming
#[derive(Debug)]
pub struct JetstreamSource {
    filters: Filters,
    config: JetstreamSourceConfig,
}

#[async_trait]
impl SourceTrait for JetstreamSource {
    type Config = JetstreamSourceConfig;

    fn new(config: Self::Config, filters: Filters) -> Self { Self { config, filters } }

    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
    ) -> Result<(), ShipsternError> {
        let config = self.config.clone();
        let filters = self.filters.clone();

        // jetstreamer-firehose reads configuration exclusively through env vars.
        // The caller must have set them *before* the runtime started via
        // `init_process_env()`. We only validate here — no mutation.
        {
            let expected = ProcessEnvConfig::from_config(&config);
            expected.validate_matches(&ProcessEnvConfig::from_process())?;
        }

        // `reverse` implies sequential upstream, so it also makes the window
        // meaningful — only warn when neither mode is active.
        if config.buffer_window_bytes.is_some() && !config.sequential && !config.reverse {
            tracing::warn!(
                "`buffer_window_bytes` is set but neither `sequential` nor `reverse` is enabled; \
                 the value will be ignored by jetstreamer-firehose"
            );
        }

        tokio::spawn(async move {
            let exit_status = match Self::stream_loop(config, filters, tx.clone()).await {
                Ok(()) => SourceExitStatus::Completed,
                Err(e) => {
                    error!(error = %e, "Jetstream streaming failed");
                    let _ = tx
                        .send(Err(yellowstone_grpc_proto::tonic::Status::internal(
                            e.to_string(),
                        )))
                        .await;
                    SourceExitStatus::Error(e.to_string())
                },
            };
            let _ = status_tx.send(exit_status);
        });

        Ok(())
    }
}

/// Log a structured firehose progress pulse.
///
/// Registered as the upstream `StatsTracking` callback so periodic stats come
/// straight from the engine's own aggregates — blocks, transactions, entries,
/// and leader-skipped slots — rather than a hand-rolled block counter.
///
/// `on_stats` fires once per worker thread when that thread crosses a
/// `stats_interval_slots` boundary; the counters are global aggregates, so
/// `thread_id` is logged to identify which worker emitted the pulse.
///
/// Example output:
///
/// ```text, ignore
/// INFO Firehose progress thread_id=2 slots=20000 blocks=19987 transactions=4821334 entries=20000 leader_skipped_slots=13 tps=152000
/// ```
fn log_firehose_stats(
    thread_id: usize,
    stats: jetstreamer_firehose::firehose::Stats,
) -> futures_util::future::BoxFuture<'static, Result<(), SharedError>> {
    async move {
        let elapsed = stats.time_since_last_pulse.as_secs_f64();
        let tps = if elapsed > 0.0 {
            (stats.transactions_since_last_pulse as f64 / elapsed).round() as u64
        } else {
            0
        };

        info!(
            thread_id,
            slots = stats.slots_processed,
            blocks = stats.blocks_processed,
            transactions = stats.transactions_processed,
            entries = stats.entries_processed,
            leader_skipped_slots = stats.leader_skipped_slots,
            tps,
            "Firehose progress"
        );

        Ok(())
    }
    .boxed()
}

impl JetstreamSource {
    async fn stream_loop(
        config: JetstreamSourceConfig,
        filters: Filters,
        tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
    ) -> Result<(), Error> {
        let (start_slot, end_slot) = config.range.to_slot_range().map_err(|e| {
            Error::SlotRangeResolution(format!(
                "Failed to resolve slot range from config {:?}: {}",
                config.range, e
            ))
        })?;

        info!(
            start_slot,
            end_slot,
            archive_url = %config.archive_url,
            threads = config.threads,
            "Starting Jetstream historical replay"
        );

        let handler = Arc::new(ShipsternStreamHandler::new(
            tx.clone(),
            config.possible_leader_skipped_tx.clone(),
            filters.clone(),
        ));

        let handler_on_block = handler.clone();
        let on_block = Some(move |_thread_id: usize, block: BlockData| {
            let handler_callback = handler_on_block.clone();
            async move { handler_callback.process_block(block).await }.boxed()
        });

        let handler_on_tx = handler.clone();
        let on_tx = Some(move |_thread_id: usize, tx: TransactionData| {
            let handler_callback = handler_on_tx.clone();
            async move { handler_callback.process_transaction(tx).await }.boxed()
        });

        // Register an `on_entry` callback only when at least one filter requested
        // entries; otherwise let the firehose skip entry decoding entirely.
        let on_entry = if handler.wants_entries {
            let handler_on_entry = handler.clone();
            Some(move |_thread_id: usize, entry: EntryData| {
                let handler_callback = handler_on_entry.clone();
                async move { handler_callback.process_entry(entry).await }.boxed()
            })
        } else {
            None
        };

        // Subscribe to the caller-provided shutdown channel, if any. Subscribing
        // here (after the receiver-less period during config construction) means
        // signals broadcast before this line are lost — callers that need
        // deterministic shutdown should keep the `Sender` alive and broadcast
        // only after `connect()` returns.
        let shutdown_signal = config.shutdown_signal_tx.as_ref().map(|tx| tx.subscribe());

        // Register upstream `StatsTracking` so periodic progress comes from the
        // firehose's own aggregates. `stats_interval_slots == 0` disables it —
        // this also avoids upstream's unguarded `slot % interval` (a `0`
        // interval would divide by zero).
        let stats_tracking = (config.stats_interval_slots != 0).then_some(
            jetstreamer_firehose::firehose::StatsTracking {
                on_stats: log_firehose_stats,
                tracking_interval_slots: config.stats_interval_slots,
            },
        );

        // Upstream's default window can be too large to download inside its own
        // header-read timeout, which hangs the run. Use a bounded default.
        let buffer_window_bytes = effective_buffer_window_bytes(
            config.sequential,
            config.reverse,
            config.buffer_window_bytes,
        );

        if config.buffer_window_bytes.is_none() && (config.sequential || config.reverse) {
            info!(
                buffer_window_bytes = DEFAULT_SEQUENTIAL_BUFFER_WINDOW_BYTES,
                "sequential mode active with no explicit buffer window; using the vixen default \
                 instead of upstream's RAM-derived one"
            );
        }

        let result = firehose(
            config.threads as u64,
            config.sequential,
            config.reverse,
            buffer_window_bytes,
            start_slot..end_slot,
            on_block,
            on_tx,
            on_entry,
            None::<jetstreamer_firehose::firehose::OnRewardFn>,
            None::<OnErrorFn>,
            stats_tracking,
            shutdown_signal,
        )
        .await;

        if let Err((error, slot)) = result {
            let error_msg = format!("{:?}", error);
            if error_msg.contains("incomplete frame") {
                error!(
                    slot,
                    error = %error_msg,
                    "Corrupted CAR file detected"
                );
                return Err(Error::Jetstreamer(format!(
                    "Corrupted data at slot {}: {}. Try a different epoch or slot range.",
                    slot, error_msg
                )));
            } else {
                return Err(Error::Jetstreamer(format!(
                    "Firehose error at slot {}: {:?}",
                    slot, error
                )));
            }
        }

        info!(
            start_slot,
            end_slot, "Jetstream historical replay completed successfully"
        );
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Plugin execution error: {0}")]
    PluginExecution(String),

    #[error("Data conversion error: {0}")]
    DataConversion(String),

    #[error("Channel send error: {0}")]
    ChannelSend(String),

    #[error("Thread join error: {0}")]
    ThreadJoin(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Slot range resolution error: {0}")]
    SlotRangeResolution(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Jetstreamer firehose error: {0}")]
    Jetstreamer(String),

    #[error("Process env does not match config (did you call init_process_env?): {0}")]
    EnvMismatch(String),

    #[error("Entry buffer mutex poisoned: {0}")]
    EntryBufferPoisoned(String),
}

impl From<Error> for ShipsternError {
    fn from(e: Error) -> Self {
        match e {
            Error::Io(io_err) => ShipsternError::Io(io_err),
            // ShipsternError only exposes an Io variant for generic errors.
            // Wrap with `io::Error::other` but preserve the original error as
            // the source (via `Box<dyn Error>`) so callers can still downcast.
            other => ShipsternError::Io(std::io::Error::other(other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoch_to_slot_conversion() {
        let config = SlotRangeConfig {
            slot_start: None,
            slot_end: None,
            epoch: Some(800),
        };
        let (start, end_exclusive) = config.to_slot_range().unwrap();
        assert_eq!(start, 345_600_000);
        assert_eq!(end_exclusive, 346_032_000); // end-exclusive: first slot of next epoch
    }

    #[test]
    fn test_slot_range_validation() {
        let config = SlotRangeConfig {
            slot_start: Some(100),
            slot_end: Some(50),
            epoch: None,
        };
        assert!(config.to_slot_range().is_err());
    }

    #[test]
    fn test_slot_range_empty_range_rejected() {
        let config = SlotRangeConfig {
            slot_start: Some(100),
            slot_end: Some(100),
            epoch: None,
        };
        assert!(
            config.to_slot_range().is_err(),
            "start == end is an empty range"
        );
    }

    #[test]
    fn test_invalid_config_both_epoch_and_slots() {
        let config = SlotRangeConfig {
            slot_start: Some(100),
            slot_end: Some(200),
            epoch: Some(800),
        };
        assert!(config.to_slot_range().is_err());
    }

    #[test]
    fn test_jetstream_source_creation() {
        let config = JetstreamSourceConfig {
            archive_url: "https://api.old-faithful.net".to_string(),
            range: SlotRangeConfig {
                slot_start: Some(1000),
                slot_end: Some(2000),
                epoch: None,
            },
            threads: 4,
            network: "mainnet".to_string(),
            compact_index_base_url: "https://files.old-faithful.net".to_string(),
            network_capacity_mb: 1000,
            sequential: false,
            reverse: false,
            buffer_window_bytes: None,
            stats_interval_slots: 10_000,
            possible_leader_skipped_tx: None,
            shutdown_signal_tx: None,
        };

        let filters = Filters::new(std::collections::HashMap::new());
        let source = JetstreamSource::new(config, filters);

        assert_eq!(source.config.archive_url, "https://api.old-faithful.net");
        assert_eq!(source.config.threads, 4);
        assert_eq!(source.config.network, "mainnet");
        assert!(!source.config.sequential);
        assert!(source.config.buffer_window_bytes.is_none());
    }

    #[test]
    fn test_jetstream_source_config_toml_roundtrip() {
        let toml_str = r#"
archive-url = "https://api.old-faithful.net"
threads = 4
network = "mainnet"
compact-index-base-url = "https://files.old-faithful.net"
network-capacity-mb = 1000
sequential = true
reverse = true
buffer-window-bytes = 1073741824
stats-interval-slots = 500

[range]
slot-start = 1000
slot-end = 2000
"#;

        let config: JetstreamSourceConfig =
            toml::from_str(toml_str).expect("valid TOML for JetstreamSourceConfig");

        assert!(config.sequential);
        assert!(config.reverse);
        assert_eq!(config.buffer_window_bytes, Some(1_073_741_824));
        assert_eq!(config.stats_interval_slots, 500);
        assert_eq!(config.archive_url, "https://api.old-faithful.net");
        assert_eq!(config.threads, 4);
    }

    #[test]
    fn test_jetstream_source_config_toml_defaults_when_omitted() {
        let toml_str = r#"
archive-url = "https://api.old-faithful.net"
threads = 4
network = "mainnet"
compact-index-base-url = "https://files.old-faithful.net"
network-capacity-mb = 1000

[range]
slot-start = 1000
slot-end = 2000
"#;

        let config: JetstreamSourceConfig = toml::from_str(toml_str)
            .expect("TOML omitting `sequential` and `buffer-window-bytes` must deserialize");

        assert!(
            !config.sequential,
            "`sequential` must default to false when absent from TOML"
        );
        assert!(
            !config.reverse,
            "`reverse` must default to false when absent from TOML"
        );
        assert!(
            config.buffer_window_bytes.is_none(),
            "`buffer_window_bytes` must default to None when absent from TOML"
        );
        assert_eq!(
            config.stats_interval_slots, 10_000,
            "`stats_interval_slots` must default to 10_000 when absent from TOML"
        );
    }

    /// Upstream's RAM-derived window can be too large to download inside its
    /// own 180s header-read timeout, which hangs the run instead of failing it.
    /// Sequential runs must therefore get a bounded window by default, while an
    /// explicit choice is always honoured.
    #[test]
    fn sequential_modes_get_a_bounded_default_buffer_window() {
        assert_eq!(
            effective_buffer_window_bytes(false, false, None),
            None,
            "no window should be injected when upstream would ignore it"
        );

        for (sequential, reverse) in [(true, false), (false, true), (true, true)] {
            assert_eq!(
                effective_buffer_window_bytes(sequential, reverse, None),
                Some(DEFAULT_SEQUENTIAL_BUFFER_WINDOW_BYTES),
                "sequential={sequential} reverse={reverse} must fall back to the vixen default"
            );
        }
    }

    #[test]
    fn explicit_buffer_window_always_wins() {
        // Including a value far larger than the default — opting back into
        // upstream's throughput-oriented behaviour must remain possible.
        let huge = DEFAULT_SEQUENTIAL_BUFFER_WINDOW_BYTES * 16;

        assert_eq!(
            effective_buffer_window_bytes(true, false, Some(huge)),
            Some(huge)
        );
        assert_eq!(
            effective_buffer_window_bytes(false, true, Some(4096)),
            Some(4096)
        );
        assert_eq!(
            effective_buffer_window_bytes(false, false, Some(4096)),
            Some(4096),
            "a value set while sequential mode is off is passed through untouched"
        );
    }

    #[tokio::test]
    async fn possible_leader_skipped_events_use_side_channel() {
        let (updates_tx, mut updates_rx) = mpsc::channel(4);
        let (skipped_tx, mut skipped_rx) = mpsc::channel(4);
        let handler = ShipsternStreamHandler::new(
            updates_tx,
            Some(skipped_tx),
            Filters::new(std::collections::HashMap::new()),
        );

        handler
            .process_block(BlockData::PossibleLeaderSkipped { slot: 123 })
            .await
            .expect("side-channel send should succeed");

        let skipped = skipped_rx.recv().await.expect("skipped event");
        assert_eq!(skipped, PossibleLeaderSkippedEvent { slot: 123 });
        assert!(
            updates_rx.try_recv().is_err(),
            "no fake block update should be emitted"
        );
    }

    #[tokio::test]
    async fn buffered_entries_attach_to_block_when_include_entries_set() {
        use std::collections::HashMap as StdHashMap;

        use shipstern_core::{BlockPrefilter, Prefilter};
        use solana_hash::Hash;
        use solana_runtime::bank::KeyedRewardsAndNumPartitions;

        let mut prefilters = StdHashMap::new();
        prefilters.insert("block-with-entries".to_string(), Prefilter {
            account: None,
            transaction: None,
            block_meta: None,
            block: Some(BlockPrefilter {
                accounts_include: Default::default(),
                include_transactions: false,
                include_accounts: false,
                include_entries: true,
            }),
            slot: None,
        });
        let filters = Filters::new(prefilters);

        let (updates_tx, mut updates_rx) = mpsc::channel(4);
        let handler = ShipsternStreamHandler::new(updates_tx, None, filters);
        assert!(handler.wants_entries, "filter requested entries");

        for entry_index in 0..3 {
            handler
                .process_entry(EntryData {
                    slot: 42,
                    entry_index,
                    transaction_indexes: (entry_index * 2)..(entry_index * 2 + 2),
                    num_hashes: 12_500,
                    hash: Hash::new_from_array([entry_index as u8; 32]),
                })
                .await
                .expect("entry buffer push");
        }

        handler
            .process_block(BlockData::Block {
                parent_slot: 41,
                parent_blockhash: Hash::default(),
                slot: 42,
                blockhash: Hash::default(),
                rewards: KeyedRewardsAndNumPartitions {
                    keyed_rewards: Vec::new(),
                    num_partitions: None,
                },
                block_time: None,
                block_height: None,
                executed_transaction_count: 6,
                entry_count: 3,
            })
            .await
            .expect("block emission");

        let update = updates_rx.recv().await.expect("block update");
        let UpdateOneof::Block(block) = update
            .expect("ok")
            .update_oneof
            .expect("update_oneof present")
        else {
            panic!("expected Block variant");
        };

        assert_eq!(block.entries_count, 3);
        assert_eq!(block.entries.len(), 3);
        assert_eq!(block.entries[0].index, 0);
        assert_eq!(block.entries[0].starting_transaction_index, 0);
        assert_eq!(block.entries[0].executed_transaction_count, 2);
        assert_eq!(block.entries[2].index, 2);
        assert_eq!(block.entries[2].starting_transaction_index, 4);

        // Buffer must be drained after emission — second block on the same slot
        // would otherwise leak stale entries.
        assert!(
            handler
                .entry_buffer
                .lock()
                .expect("entry_buffer poisoned")
                .is_empty(),
            "entry buffer must be drained after block emission"
        );
    }

    #[tokio::test]
    async fn shutdown_signal_round_trip_through_config() {
        // Mirrors how `connect()` consumes the channel: store a `Sender` on
        // the config, then `.subscribe()` to get a `Receiver` at the firehose
        // call site. Broadcasts must reach the receiver.
        let (shutdown_tx, _) = broadcast::channel::<()>(1);
        let config = JetstreamSourceConfig {
            archive_url: "https://api.old-faithful.net".to_string(),
            range: SlotRangeConfig {
                slot_start: Some(1000),
                slot_end: Some(2000),
                epoch: None,
            },
            threads: 4,
            network: "mainnet".to_string(),
            compact_index_base_url: "https://files.old-faithful.net".to_string(),
            network_capacity_mb: 1000,
            sequential: false,
            reverse: false,
            buffer_window_bytes: None,
            stats_interval_slots: 10_000,
            possible_leader_skipped_tx: None,
            shutdown_signal_tx: Some(shutdown_tx.clone()),
        };

        // Config must remain Clone — `broadcast::Sender` is Clone, so the
        // outer derive should still hold. Compile-time check; the runtime
        // assertion confirms the cloned Sender points at the same channel.
        let config_cloned = config.clone();
        let mut rx = config_cloned
            .shutdown_signal_tx
            .as_ref()
            .expect("sender present")
            .subscribe();

        shutdown_tx.send(()).expect("at least one receiver");
        rx.recv().await.expect("broadcast delivered");
    }

    #[tokio::test]
    async fn process_entry_returns_error_when_buffer_mutex_poisoned() {
        use std::collections::HashMap as StdHashMap;

        use shipstern_core::{BlockPrefilter, Prefilter};

        let mut prefilters = StdHashMap::new();
        prefilters.insert("wants-entries".to_string(), Prefilter {
            account: None,
            transaction: None,
            block_meta: None,
            block: Some(BlockPrefilter {
                accounts_include: Default::default(),
                include_transactions: false,
                include_accounts: false,
                include_entries: true,
            }),
            slot: None,
        });
        let filters = Filters::new(prefilters);

        let (updates_tx, _updates_rx) = mpsc::channel(4);
        let handler = Arc::new(ShipsternStreamHandler::new(updates_tx, None, filters));

        // Poison the mutex by panicking while holding the lock on a worker
        // thread. After the join, any subsequent `.lock()` returns Err.
        let poisoner = Arc::clone(&handler);
        let join = std::thread::spawn(move || {
            let _guard = poisoner.entry_buffer.lock().unwrap();
            panic!("intentional poison for test");
        })
        .join();
        assert!(join.is_err(), "poisoner thread should have panicked");

        let result = handler
            .process_entry(EntryData {
                slot: 1,
                entry_index: 0,
                transaction_indexes: 0..0,
                num_hashes: 0,
                hash: solana_hash::Hash::default(),
            })
            .await;

        let err = result.expect_err("poisoned mutex must surface as error, not panic");
        let downcast = err
            .downcast_ref::<Error>()
            .expect("error must downcast to jetstream-source Error");
        assert!(
            matches!(downcast, Error::EntryBufferPoisoned(_)),
            "expected EntryBufferPoisoned, got {downcast:?}"
        );
    }

    /// Build a `Filters` set holding a single prefilter under `filter_id`.
    fn filters_with(filter_id: &str, prefilter: shipstern_core::Prefilter) -> Filters {
        let mut prefilters = std::collections::HashMap::new();
        prefilters.insert(filter_id.to_string(), prefilter);

        Filters::new(prefilters)
    }

    /// A block covering every slot-scoped variant, so one call can prove which
    /// updates a given prefilter does and does not produce.
    fn sample_block() -> BlockData {
        use solana_hash::Hash;
        use solana_runtime::bank::KeyedRewardsAndNumPartitions;

        BlockData::Block {
            parent_slot: 41,
            parent_blockhash: Hash::default(),
            slot: 42,
            blockhash: Hash::default(),
            rewards: KeyedRewardsAndNumPartitions {
                keyed_rewards: Vec::new(),
                num_partitions: None,
            },
            block_time: Some(1_700_000_000),
            block_height: Some(7),
            executed_transaction_count: 6,
            entry_count: 3,
        }
    }

    /// The runtime dispatches by `UpdateOneof` variant, so a `block_meta`
    /// prefilter must get an actual `BlockMeta` update. Riding along on a
    /// `Block` update would match no pipeline and be dropped.
    #[tokio::test]
    async fn block_meta_prefilter_receives_a_block_meta_update() {
        use shipstern_core::{BlockMetaPrefilter, Prefilter};

        let filters = filters_with("wants-block-meta", Prefilter {
            account: None,
            transaction: None,
            block_meta: Some(BlockMetaPrefilter {}),
            block: None,
            slot: None,
        });

        let (updates_tx, mut updates_rx) = mpsc::channel(4);
        let handler = ShipsternStreamHandler::new(updates_tx, None, filters);

        assert!(
            handler.block_matches.is_empty(),
            "a block-meta prefilter must not be bucketed as a block filter"
        );

        handler
            .process_block(sample_block())
            .await
            .expect("block emission");

        let update = updates_rx.recv().await.expect("update").expect("ok");
        assert_eq!(update.filters, vec!["wants-block-meta".to_string()]);

        let Some(UpdateOneof::BlockMeta(meta)) = update.update_oneof else {
            panic!("expected BlockMeta variant, got {:?}", update.update_oneof);
        };

        assert_eq!(meta.slot, 42);
        assert_eq!(meta.parent_slot, 41);
        assert_eq!(meta.executed_transaction_count, 6);
        assert_eq!(meta.entries_count, 3);
        assert_eq!(meta.block_time.map(|t| t.timestamp), Some(1_700_000_000));
        assert_eq!(meta.block_height.map(|h| h.block_height), Some(7));
        assert!(meta.rewards.is_some(), "rewards must be forwarded");

        assert!(
            updates_rx.try_recv().is_err(),
            "no Block update should be emitted for a block-meta-only filter"
        );
    }

    /// Archived history is finalized, so a replayed slot reports exactly one
    /// status transition rather than the live processed/confirmed sequence.
    #[tokio::test]
    async fn slot_prefilter_receives_a_finalized_slot_update() {
        use shipstern_core::{Prefilter, SlotPrefilter};

        let filters = filters_with("wants-slots", Prefilter {
            account: None,
            transaction: None,
            block_meta: None,
            block: None,
            slot: Some(SlotPrefilter::default()),
        });

        let (updates_tx, mut updates_rx) = mpsc::channel(4);
        let handler = ShipsternStreamHandler::new(updates_tx, None, filters);

        assert!(
            handler.block_matches.is_empty(),
            "a slot prefilter must not be bucketed as a block filter"
        );

        handler
            .process_block(sample_block())
            .await
            .expect("block emission");

        let update = updates_rx.recv().await.expect("update").expect("ok");
        assert_eq!(update.filters, vec!["wants-slots".to_string()]);

        let Some(UpdateOneof::Slot(slot_update)) = update.update_oneof else {
            panic!("expected Slot variant, got {:?}", update.update_oneof);
        };

        assert_eq!(slot_update.slot, 42);
        assert_eq!(slot_update.parent, Some(41));
        assert_eq!(slot_update.status, SlotStatus::SlotFinalized as i32);
        assert_eq!(slot_update.dead_error, None);

        assert!(
            updates_rx.try_recv().is_err(),
            "no Block update should be emitted for a slot-only filter"
        );
    }

    /// One parser asking for all three variants must get all three, each
    /// addressed to its own filter ID.
    #[tokio::test]
    async fn every_requested_variant_is_emitted_for_one_block() {
        use shipstern_core::{BlockMetaPrefilter, BlockPrefilter, Prefilter, SlotPrefilter};

        let filters = filters_with("wants-everything", Prefilter {
            account: None,
            transaction: None,
            block_meta: Some(BlockMetaPrefilter {}),
            block: Some(BlockPrefilter {
                accounts_include: Default::default(),
                include_transactions: true,
                include_accounts: false,
                include_entries: false,
            }),
            slot: Some(SlotPrefilter::default()),
        });

        let (updates_tx, mut updates_rx) = mpsc::channel(8);
        let handler = ShipsternStreamHandler::new(updates_tx, None, filters);

        handler
            .process_block(sample_block())
            .await
            .expect("block emission");

        let mut variants = Vec::new();
        while let Ok(update) = updates_rx.try_recv() {
            let update = update.expect("ok");
            assert_eq!(update.filters, vec!["wants-everything".to_string()]);
            variants.push(match update.update_oneof {
                Some(UpdateOneof::Block(_)) => "block",
                Some(UpdateOneof::BlockMeta(_)) => "block_meta",
                Some(UpdateOneof::Slot(_)) => "slot",
                other => panic!("unexpected variant {other:?}"),
            });
        }

        assert_eq!(variants, ["block", "block_meta", "slot"]);
    }

    #[tokio::test]
    async fn entries_are_not_buffered_when_no_filter_requests_them() {
        let (updates_tx, _updates_rx) = mpsc::channel(4);
        let handler = ShipsternStreamHandler::new(
            updates_tx,
            None,
            Filters::new(std::collections::HashMap::new()),
        );
        assert!(
            !handler.wants_entries,
            "no filter set => entries should be skipped"
        );

        handler
            .process_entry(EntryData {
                slot: 1,
                entry_index: 0,
                transaction_indexes: 0..0,
                num_hashes: 0,
                hash: solana_hash::Hash::default(),
            })
            .await
            .expect("noop entry");

        assert!(
            handler
                .entry_buffer
                .lock()
                .expect("entry_buffer poisoned")
                .is_empty(),
            "buffer must stay empty when wants_entries is false"
        );
    }

    #[test]
    fn test_multiple_epochs() {
        for epoch in [800, 801, 802] {
            let config = SlotRangeConfig {
                slot_start: None,
                slot_end: None,
                epoch: Some(epoch),
            };
            let (start, end_exclusive) = config.to_slot_range().unwrap();
            assert_eq!(start, epoch * 432_000);
            assert_eq!(end_exclusive, (epoch + 1) * 432_000); // end-exclusive
        }
    }

    #[test]
    fn keyed_rewards_round_trip_through_proto() {
        use solana_accounts_db::stake_rewards::StakeRewardInfo;
        use solana_pubkey::Pubkey;
        use solana_reward_info::RewardType as SdkRewardType;
        use solana_runtime::bank::KeyedRewardsAndNumPartitions;
        use yellowstone_grpc_proto::solana::storage::confirmed_block as proto;

        let fee_pk = Pubkey::new_unique();
        let rent_pk = Pubkey::new_unique();
        let staking_pk = Pubkey::new_unique();
        let voting_pk = Pubkey::new_unique();

        // `solana_runtime::reward_info::RewardInfo` sits behind a private module
        // in 4.2, so the fixtures are built as `StakeRewardInfo` and converted
        // through the public `From` impl.
        let input = KeyedRewardsAndNumPartitions {
            keyed_rewards: vec![
                (
                    fee_pk,
                    StakeRewardInfo {
                        reward_type: SdkRewardType::Fee,
                        lamports: 1,
                        post_balance: 100,
                        commission_bps: None,
                    }
                    .into(),
                ),
                (
                    rent_pk,
                    StakeRewardInfo {
                        reward_type: SdkRewardType::Rent,
                        lamports: -2, // i64, can be negative
                        post_balance: 200,
                        commission_bps: None,
                    }
                    .into(),
                ),
                (
                    staking_pk,
                    StakeRewardInfo {
                        reward_type: SdkRewardType::Staking,
                        lamports: 3,
                        post_balance: 300,
                        commission_bps: Some(700),
                    }
                    .into(),
                ),
                (
                    voting_pk,
                    StakeRewardInfo {
                        reward_type: SdkRewardType::Voting,
                        lamports: 4,
                        post_balance: 400,
                        commission_bps: Some(0),
                    }
                    .into(),
                ),
            ],
            num_partitions: Some(64),
        };

        let out = convert::keyed_rewards(&input);

        assert_eq!(out.rewards.len(), 4);

        // Pubkey strings round-trip via Display
        assert_eq!(out.rewards[0].pubkey, fee_pk.to_string());
        assert_eq!(out.rewards[1].pubkey, rent_pk.to_string());
        assert_eq!(out.rewards[2].pubkey, staking_pk.to_string());
        assert_eq!(out.rewards[3].pubkey, voting_pk.to_string());

        // Proto enum discriminants: Unspecified=0, Fee=1, Rent=2, Staking=3, Voting=4.
        assert_eq!(out.rewards[0].reward_type, proto::RewardType::Fee as i32);
        assert_eq!(out.rewards[1].reward_type, proto::RewardType::Rent as i32);
        assert_eq!(
            out.rewards[2].reward_type,
            proto::RewardType::Staking as i32
        );
        assert_eq!(out.rewards[3].reward_type, proto::RewardType::Voting as i32);

        // Lamports + post_balance pass through unchanged.
        assert_eq!(out.rewards[1].lamports, -2);
        assert_eq!(out.rewards[2].post_balance, 300);

        // Basis points come straight from the source, empty when absent.
        assert_eq!(out.rewards[0].commission_bps, "");
        assert_eq!(out.rewards[2].commission_bps, "700");
        assert_eq!(out.rewards[3].commission_bps, "0");

        // Percent is back-derived, and only when the bps divide evenly.
        assert_eq!(out.rewards[0].commission, "");
        assert_eq!(out.rewards[2].commission, "7");
        assert_eq!(out.rewards[3].commission, "0");

        // num_partitions wrapped in proto NumPartitions.
        assert_eq!(
            out.num_partitions,
            Some(proto::NumPartitions { num_partitions: 64 })
        );
    }

    /// The second reward conversion path. `transaction_status_meta` carries its
    /// own `proto::Reward` mapping, so `commission_bps` has to be pinned here
    /// too rather than relying on `keyed_rewards` coverage alone.
    #[test]
    fn transaction_status_meta_rewards_carry_commission_bps() {
        use solana_transaction_status::{Reward, RewardType, TransactionStatusMeta};

        let meta = TransactionStatusMeta {
            rewards: Some(vec![
                Reward {
                    pubkey: "voter".to_string(),
                    lamports: 5,
                    post_balance: 50,
                    reward_type: Some(RewardType::Voting),
                    commission: Some(7),
                    commission_bps: Some(700),
                },
                Reward {
                    pubkey: "fee-payer".to_string(),
                    lamports: -1,
                    post_balance: 10,
                    reward_type: Some(RewardType::Fee),
                    commission: None,
                    commission_bps: None,
                },
            ]),
            ..Default::default()
        };

        let out = convert::transaction_status_meta(meta);

        assert_eq!(out.rewards.len(), 2);
        assert_eq!(out.rewards[0].commission, "7");
        assert_eq!(out.rewards[0].commission_bps, "700");
        assert_eq!(out.rewards[1].commission, "");
        assert_eq!(out.rewards[1].commission_bps, "");
    }

    #[test]
    fn keyed_rewards_empty_input_yields_empty_proto() {
        use solana_runtime::bank::KeyedRewardsAndNumPartitions;

        let empty = KeyedRewardsAndNumPartitions {
            keyed_rewards: vec![],
            num_partitions: None,
        };
        let out = convert::keyed_rewards(&empty);
        assert!(out.rewards.is_empty());
        assert!(out.num_partitions.is_none());
    }

    /// A commission that is not a whole number of percent has no exact `u8`
    /// representation. Basis points must survive untouched, and the percent
    /// field must go empty rather than report a rate nobody set.
    #[test]
    fn keyed_rewards_preserve_exact_basis_points() {
        use solana_accounts_db::stake_rewards::StakeRewardInfo;
        use solana_pubkey::Pubkey;
        use solana_reward_info::RewardType as SdkRewardType;
        use solana_runtime::bank::KeyedRewardsAndNumPartitions;

        let odd = Pubkey::new_unique();
        let round = Pubkey::new_unique();

        let input = KeyedRewardsAndNumPartitions {
            keyed_rewards: vec![
                (
                    odd,
                    StakeRewardInfo {
                        reward_type: SdkRewardType::Voting,
                        lamports: 1,
                        post_balance: 10,
                        commission_bps: Some(1234),
                    }
                    .into(),
                ),
                (
                    round,
                    StakeRewardInfo {
                        reward_type: SdkRewardType::Voting,
                        lamports: 2,
                        post_balance: 20,
                        commission_bps: Some(1200),
                    }
                    .into(),
                ),
            ],
            num_partitions: None,
        };

        let out = convert::keyed_rewards(&input);

        // 1234 bps stays 1234 bps, and is never rounded down to 1200.
        assert_eq!(out.rewards[0].commission_bps, "1234");
        assert_eq!(out.rewards[0].commission, "");

        // 1200 bps divides evenly, so the percent field is still filled in.
        assert_eq!(out.rewards[1].commission_bps, "1200");
        assert_eq!(out.rewards[1].commission, "12");
    }

    /// Agave 4.2 added `RewardType::DeactivatedStake`, and the proto gained a
    /// matching `DeactivatedStake = 5`. The old wildcard arm mapped it to
    /// `Unspecified`, which silently lost the reward type.
    #[test]
    fn deactivated_stake_reward_type_survives_both_conversions() {
        use solana_accounts_db::stake_rewards::StakeRewardInfo;
        use solana_pubkey::Pubkey;
        use solana_reward_info::RewardType as SdkRewardType;
        use solana_runtime::bank::KeyedRewardsAndNumPartitions;
        use solana_transaction_status::{
            Reward, RewardType as StatusRewardType, TransactionStatusMeta,
        };
        use yellowstone_grpc_proto::solana::storage::confirmed_block as proto;

        let keyed = convert::keyed_rewards(&KeyedRewardsAndNumPartitions {
            keyed_rewards: vec![(
                Pubkey::new_unique(),
                StakeRewardInfo {
                    reward_type: SdkRewardType::DeactivatedStake,
                    lamports: -5,
                    post_balance: 0,
                    commission_bps: None,
                }
                .into(),
            )],
            num_partitions: None,
        });
        assert_eq!(
            keyed.rewards[0].reward_type,
            proto::RewardType::DeactivatedStake as i32
        );

        let meta = convert::transaction_status_meta(TransactionStatusMeta {
            rewards: Some(vec![Reward {
                pubkey: "staker".to_string(),
                lamports: -5,
                post_balance: 0,
                reward_type: Some(StatusRewardType::DeactivatedStake),
                commission: None,
                commission_bps: None,
            }]),
            ..Default::default()
        });
        assert_eq!(
            meta.rewards[0].reward_type,
            proto::RewardType::DeactivatedStake as i32
        );
    }

    mod transaction_versions {
        use solana_hash::Hash;
        use solana_message::{
            compiled_instruction::CompiledInstruction,
            legacy,
            v0::{self, MessageAddressTableLookup},
            v1::{self, TransactionConfig},
            MessageHeader, VersionedMessage,
        };
        use solana_pubkey::Pubkey;
        use solana_signature::Signature;
        use solana_transaction::versioned::VersionedTransaction;

        use crate::convert;

        fn header() -> MessageHeader {
            MessageHeader {
                num_required_signatures: 1,
                num_readonly_signed_accounts: 0,
                num_readonly_unsigned_accounts: 1,
            }
        }

        fn instructions() -> Vec<CompiledInstruction> {
            vec![CompiledInstruction {
                program_id_index: 2,
                accounts: vec![0, 1],
                data: vec![7, 7, 7],
            }]
        }

        fn signed(message: VersionedMessage) -> VersionedTransaction {
            VersionedTransaction {
                signatures: vec![Signature::from([3u8; 64])],
                message,
            }
        }

        fn v1_message(config: TransactionConfig) -> VersionedMessage {
            VersionedMessage::V1(v1::Message {
                header: header(),
                config,
                lifetime_specifier: Hash::new_from_array([9u8; 32]),
                account_keys: vec![
                    Pubkey::new_unique(),
                    Pubkey::new_unique(),
                    Pubkey::new_unique(),
                ],
                instructions: instructions(),
            })
        }

        #[test]
        fn legacy_is_unversioned_and_carries_no_config() {
            let blockhash = Hash::new_from_array([1u8; 32]);
            let out = convert::transaction(signed(VersionedMessage::Legacy(legacy::Message {
                header: header(),
                account_keys: vec![Pubkey::new_unique(), Pubkey::new_unique()],
                recent_blockhash: blockhash,
                instructions: instructions(),
            })));

            let msg = out.message.expect("message");
            assert!(!msg.versioned);
            assert!(msg.config.is_none(), "legacy must never gain a config");
            assert!(msg.address_table_lookups.is_empty());
            assert_eq!(msg.recent_blockhash, blockhash.as_ref().to_vec());
        }

        #[test]
        fn v0_keeps_lookups_and_carries_no_config() {
            let lookup_key = Pubkey::new_unique();
            let out = convert::transaction(signed(VersionedMessage::V0(v0::Message {
                header: header(),
                account_keys: vec![Pubkey::new_unique(), Pubkey::new_unique()],
                recent_blockhash: Hash::new_from_array([2u8; 32]),
                instructions: instructions(),
                address_table_lookups: vec![MessageAddressTableLookup {
                    account_key: lookup_key,
                    writable_indexes: vec![4, 5],
                    readonly_indexes: vec![6],
                }],
            })));

            let msg = out.message.expect("message");
            assert!(msg.versioned);
            assert!(
                msg.config.is_none(),
                "a config on V0 would make it read as V1 downstream"
            );
            assert_eq!(msg.address_table_lookups.len(), 1);
            assert_eq!(
                msg.address_table_lookups[0].account_key,
                lookup_key.as_ref().to_vec()
            );
            assert_eq!(msg.address_table_lookups[0].writable_indexes, vec![4, 5]);
            assert_eq!(msg.address_table_lookups[0].readonly_indexes, vec![6]);
        }

        #[test]
        fn v1_preserves_every_config_field_and_drops_no_message_data() {
            let message = v1_message(
                TransactionConfig::empty()
                    .with_priority_fee(5_000)
                    .with_compute_unit_limit(200_000)
                    .with_loaded_accounts_data_size_limit(65_536)
                    .with_heap_size(262_144),
            );
            let VersionedMessage::V1(ref original) = message else {
                unreachable!("constructed as V1")
            };
            let expected_keys: Vec<Vec<u8>> = original
                .account_keys
                .iter()
                .map(|k| k.as_ref().to_vec())
                .collect();
            let expected_lifetime = original.lifetime_specifier.as_ref().to_vec();

            let out = convert::transaction(signed(message.clone()));

            let msg = out.message.expect("message");
            assert!(msg.versioned);
            assert!(
                msg.address_table_lookups.is_empty(),
                "V1 has no address lookup tables"
            );

            let config = msg.config.expect("V1 config must be present");
            assert_eq!(config.priority_fee, Some(5_000));
            assert_eq!(config.compute_unit_limit, Some(200_000));
            assert_eq!(config.loaded_accounts_data_size_limit, Some(65_536));
            assert_eq!(config.heap_size, Some(262_144));

            assert_eq!(msg.account_keys, expected_keys);
            assert_eq!(msg.recent_blockhash, expected_lifetime);
            assert_eq!(msg.instructions.len(), 1);
            assert_eq!(msg.instructions[0].program_id_index, 2);
            assert_eq!(msg.instructions[0].accounts, vec![0, 1]);
            assert_eq!(msg.instructions[0].data, vec![7, 7, 7]);

            let hdr = msg.header.expect("header");
            assert_eq!(hdr.num_required_signatures, 1);
            assert_eq!(hdr.num_readonly_unsigned_accounts, 1);

            assert_eq!(out.signatures, vec![vec![3u8; 64]]);
        }

        /// The proto marks a message as V1 by the *presence* of `config`, not by
        /// its contents. A V1 message that sets no budget fields still has to
        /// arrive as `Some(..)`; collapsing it to `None` is the silent V1 -> V0
        /// downgrade this whole change exists to prevent.
        #[test]
        fn v1_with_an_empty_config_is_still_v1() {
            let out = convert::transaction(signed(v1_message(TransactionConfig::empty())));

            let msg = out.message.expect("message");
            let config = msg
                .config
                .expect("an empty V1 config must still be present");

            assert_eq!(config.priority_fee, None);
            assert_eq!(config.compute_unit_limit, None);
            assert_eq!(config.loaded_accounts_data_size_limit, None);
            assert_eq!(config.heap_size, None);

            // `versioned` alone cannot tell V1 from V0, so it is never the
            // discriminator: both are true here.
            assert!(msg.versioned);
        }

        /// An explicit zero is a value the sender chose. It must not fold into
        /// "unset", which would hand the runtime a default instead.
        #[test]
        fn v1_zero_valued_config_fields_stay_present() {
            let out = convert::transaction(signed(v1_message(
                TransactionConfig::empty()
                    .with_priority_fee(0)
                    .with_compute_unit_limit(0)
                    .with_loaded_accounts_data_size_limit(0),
            )));

            let config = out
                .message
                .expect("message")
                .config
                .expect("config present");

            assert_eq!(config.priority_fee, Some(0));
            assert_eq!(config.compute_unit_limit, Some(0));
            assert_eq!(config.loaded_accounts_data_size_limit, Some(0));
            // Left unset by the builder, so it stays absent.
            assert_eq!(config.heap_size, None);
        }

        /// Build a V1 transaction whose wire encoding is exactly `target` bytes
        /// by padding the single instruction's data.
        fn v1_transaction_of_wire_size(target: usize) -> VersionedTransaction {
            let mut tx = signed(v1_message(
                TransactionConfig::empty()
                    .with_priority_fee(1)
                    .with_compute_unit_limit(2),
            ));

            let base = wincode::serialize(&tx).expect("serialize").len();
            let pad = target
                .checked_sub(base)
                .expect("target smaller than the empty-payload encoding");

            let VersionedMessage::V1(ref mut msg) = tx.message else {
                unreachable!("constructed as V1")
            };
            msg.instructions[0]
                .data
                .extend(std::iter::repeat_n(0xab, pad));

            let encoded = wincode::serialize(&tx).expect("serialize");
            assert_eq!(encoded.len(), target, "helper must hit the size exactly");

            tx
        }

        /// The decoder boundary Jetstreamer actually crosses.
        ///
        /// `Transaction::as_parsed` in jetstreamer-firehose is a single call to
        /// `wincode::deserialize` into a `VersionedTransaction`, so feeding wire
        /// bytes through the same call and into [`convert::transaction`] covers
        /// every step between the archive and the proto except the CAR
        /// dataframe read, which is version-agnostic byte plumbing.
        ///
        /// These bytes are constructed locally rather than captured from a
        /// cluster, but they are produced by the same `solana-message` 4.4.1
        /// and `solana-transaction` 4.1.6 wincode schema that decodes real
        /// traffic, so the encoding itself is the real wire format.
        #[test]
        fn v1_wire_bytes_survive_the_firehose_decode_path() {
            let original = signed(v1_message(
                TransactionConfig::empty()
                    .with_priority_fee(12_345)
                    .with_compute_unit_limit(1_400_000)
                    .with_loaded_accounts_data_size_limit(131_072)
                    .with_heap_size(65_536),
            ));

            let wire = wincode::serialize(&original).expect("serialize");

            // V1 inverts the Legacy/V0 layout: the message comes first, and the
            // signatures are appended as a fixed-length array with no ShortU16
            // count, since the header already says how many there are.
            assert_eq!(
                wire[0],
                solana_message::v1::V1_PREFIX,
                "V1 wire bytes start with the 0x81 prefix, not a signature count"
            );
            let sig_bytes = original.signatures.len() * 64;
            assert_eq!(
                &wire[wire.len() - sig_bytes..],
                original.signatures[0].as_ref(),
                "signatures are the trailing bytes of a V1 transaction"
            );

            // The exact call jetstreamer-firehose makes on archive bytes.
            let decoded: VersionedTransaction =
                wincode::deserialize(&wire).expect("wire bytes must decode");

            // The failure this guards against is a decoder that reads the
            // versioned bit, ignores the version number and yields V0.
            let VersionedMessage::V1(ref decoded_msg) = decoded.message else {
                panic!("V1 wire bytes decoded as {:?}", decoded.message)
            };
            assert_eq!(decoded_msg.config.priority_fee, Some(12_345));

            let out = convert::transaction(decoded.clone());
            let msg = out.message.expect("message");
            let config = msg.config.expect("config must survive the decode path");
            assert_eq!(config.priority_fee, Some(12_345));
            assert_eq!(config.compute_unit_limit, Some(1_400_000));
            assert_eq!(config.loaded_accounts_data_size_limit, Some(131_072));
            assert_eq!(config.heap_size, Some(65_536));
            assert!(msg.versioned);
            assert!(msg.address_table_lookups.is_empty());

            // Message bytes and signatures both reconstruct exactly.
            assert_eq!(
                decoded.message.serialize(),
                original.message.serialize(),
                "reconstructed message bytes differ from the original"
            );
            assert_eq!(decoded.signatures, original.signatures);
            assert_eq!(
                out.signatures,
                original
                    .signatures
                    .iter()
                    .map(|s| s.as_ref().to_vec())
                    .collect::<Vec<_>>()
            );
            assert_eq!(wincode::serialize(&decoded).expect("serialize"), wire);
        }

        /// Upstream states in `solana-transaction`'s own tests that "v1
        /// transaction format is not compatible with bincode". Pinning that
        /// here stops anyone from later rewriting the round-trip above in terms
        /// of bincode and believing it still proves wire compatibility.
        #[test]
        fn bincode_does_not_produce_v1_wire_bytes() {
            let tx = signed(v1_message(
                TransactionConfig::empty().with_priority_fee(9_000),
            ));

            let wire = wincode::serialize(&tx).expect("wincode");
            let bincoded = bincode::serialize(&tx).expect("bincode");

            assert_ne!(
                wire, bincoded,
                "if these ever match, the bincode caveat is gone and this test should be \
                 revisited rather than deleted"
            );
        }

        /// V1 raised the transaction ceiling from 1232 bytes to 4096. Shipstern
        /// only reads transactions, so it holds no size constant of its own -
        /// this pins that it stays that way, across and beyond the old cap.
        #[test]
        fn v1_conversion_is_size_agnostic_across_the_old_1232_cap() {
            for target in [1232usize, 1233, 2048, 3072, 4095, 4096] {
                let tx = v1_transaction_of_wire_size(target);
                let original = wincode::serialize(&tx).expect("serialize");
                assert_eq!(original.len(), target);

                // The wire bytes decode back to V1, not to V0.
                let decoded: VersionedTransaction =
                    wincode::deserialize(&original).expect("v1 bytes must decode");
                assert!(
                    matches!(decoded.message, VersionedMessage::V1(_)),
                    "{target}-byte transaction decoded as something other than V1"
                );

                let out = convert::transaction(decoded);
                let msg = out.message.expect("message");

                assert!(msg.versioned);
                assert!(msg.config.is_some(), "{target}-byte V1 lost its config");
                assert!(msg.address_table_lookups.is_empty());
                assert_eq!(msg.instructions[0].data.len(), {
                    let VersionedMessage::V1(ref m) = tx.message else {
                        unreachable!()
                    };
                    m.instructions[0].data.len()
                });

                // Re-serializing the original transaction reproduces the input
                // byte for byte, so nothing was lost on the way in.
                assert_eq!(
                    wincode::serialize(&tx).expect("serialize"),
                    original,
                    "{target}-byte V1 did not round-trip"
                );
            }
        }
    }
}

mod convert {
    use jetstreamer_firehose::firehose::EntryData;
    use solana_message::VersionedMessage;
    use solana_runtime::bank::{KeyedRewardsAndNumPartitions, RewardType};
    use solana_transaction::versioned::VersionedTransaction;
    use solana_transaction_status::{TransactionStatusMeta, TransactionTokenBalance};
    use yellowstone_grpc_proto::{
        geyser::SubscribeUpdateEntry, solana::storage::confirmed_block as proto,
    };

    /// Convert buffered firehose entries into the proto shape carried on
    /// `SubscribeUpdateBlock.entries`. Caller is responsible for ordering;
    /// upstream emits entries in `entry_index` order on a single thread per
    /// slot, so the buffered `Vec` is already sorted.
    pub fn entries(buffered: Vec<EntryData>) -> Vec<SubscribeUpdateEntry> {
        buffered
            .into_iter()
            .map(|e| SubscribeUpdateEntry {
                slot: e.slot,
                index: e.entry_index as u64,
                num_hashes: e.num_hashes,
                hash: e.hash.to_bytes().to_vec(),
                executed_transaction_count: (e.transaction_indexes.end
                    - e.transaction_indexes.start)
                    as u64,
                starting_transaction_index: e.transaction_indexes.start as u64,
            })
            .collect()
    }

    /// Convert the firehose's `KeyedRewardsAndNumPartitions` into the proto
    /// `Rewards` shape that `SubscribeUpdateBlock` carries. The proto enum
    /// uses `Unspecified=0, Fee=1, Rent=2, Staking=3, Voting=4,
    /// DeactivatedStake=5`; the Solana SDK enum has no `Unspecified`, so the
    /// mapping is total.
    ///
    /// Agave 4.2 dropped `RewardInfo::commission: Option<u8>` and left only
    /// `commission_bps: Option<u16>` (SIMD-0291), so basis points are the sole
    /// source here and are forwarded verbatim. The percent field is still
    /// filled in for existing consumers, but only when the basis points divide
    /// evenly: 1234 bps has no exact `u8` percent, and truncating it to 12
    /// would report a commission the validator never set.
    ///
    /// Example output:
    ///
    /// ```text, ignore
    /// commission_bps: Some(700)  -> commission: "7",  commission_bps: "700"
    /// commission_bps: Some(1234) -> commission: "",   commission_bps: "1234"
    /// commission_bps: None       -> commission: "",   commission_bps: ""
    /// ```
    ///
    pub fn keyed_rewards(keyed: &KeyedRewardsAndNumPartitions) -> proto::Rewards {
        let rewards = keyed
            .keyed_rewards
            .iter()
            .map(|(address, info)| {
                let reward_type = match info.reward_type {
                    RewardType::Fee => proto::RewardType::Fee,
                    RewardType::Rent => proto::RewardType::Rent,
                    RewardType::Staking => proto::RewardType::Staking,
                    RewardType::Voting => proto::RewardType::Voting,
                    RewardType::DeactivatedStake => proto::RewardType::DeactivatedStake,
                } as i32;

                proto::Reward {
                    pubkey: address.to_string(),
                    lamports: info.lamports,
                    post_balance: info.post_balance,
                    reward_type,
                    commission: info
                        .commission_bps
                        .filter(|bps| bps % 100 == 0)
                        .map(|bps| (bps / 100).to_string())
                        .unwrap_or_default(),
                    commission_bps: info
                        .commission_bps
                        .map(|bps| bps.to_string())
                        .unwrap_or_default(),
                }
            })
            .collect();

        proto::Rewards {
            rewards,
            num_partitions: keyed
                .num_partitions
                .map(|num_partitions| proto::NumPartitions { num_partitions }),
        }
    }

    /// The fields of a message that depend on which version it is.
    ///
    /// Legacy, V0 and V1 agree on the header, account keys, lifetime specifier
    /// and instructions. They disagree on exactly three things, so those are
    /// named here and every match arm in [`transaction`] has to state all three
    /// rather than inheriting a default.
    ///
    /// Example output:
    ///
    /// ```text, ignore
    /// Legacy -> versioned: false, address_table_lookups: [],     config: None
    /// V0     -> versioned: true,  address_table_lookups: [..],   config: None
    /// V1     -> versioned: true,  address_table_lookups: [],     config: Some(..)
    /// ```
    ///
    struct MessageParts {
        header: solana_message::MessageHeader,
        account_keys: Vec<solana_pubkey::Pubkey>,
        /// Legacy and V0 call this `recent_blockhash`, V1 calls it
        /// `lifetime_specifier`. Both are the same 32 bytes and the proto
        /// carries them in the same field.
        lifetime: Vec<u8>,
        instructions: Vec<solana_message::compiled_instruction::CompiledInstruction>,
        versioned: bool,
        address_table_lookups: Vec<proto::MessageAddressTableLookup>,
        config: Option<proto::TransactionConfig>,
    }

    pub fn transaction(tx: VersionedTransaction) -> proto::Transaction {
        let signatures = tx.signatures.iter().map(|s| s.as_ref().to_vec()).collect();

        let message = {
            let parts = match tx.message {
                VersionedMessage::Legacy(msg) => MessageParts {
                    header: msg.header,
                    account_keys: msg.account_keys,
                    lifetime: msg.recent_blockhash.as_ref().to_vec(),
                    instructions: msg.instructions,
                    versioned: false,
                    address_table_lookups: vec![],
                    config: None,
                },
                VersionedMessage::V0(msg) => MessageParts {
                    header: msg.header,
                    account_keys: msg.account_keys,
                    lifetime: msg.recent_blockhash.as_ref().to_vec(),
                    instructions: msg.instructions,
                    versioned: true,
                    address_table_lookups: msg
                        .address_table_lookups
                        .into_iter()
                        .map(|l| proto::MessageAddressTableLookup {
                            account_key: l.account_key.as_ref().to_vec(),
                            writable_indexes: l.writable_indexes,
                            readonly_indexes: l.readonly_indexes,
                        })
                        .collect(),
                    config: None,
                },
                // V1 (SIMD-0385) replaces ComputeBudget instructions with an
                // inline config and drops address lookup tables entirely.
                //
                // `config` is wrapped unconditionally: the proto uses its
                // presence, not its contents, to mark a message as V1, so a V1
                // message whose fields are all unset still has to serialize as
                // `Some(TransactionConfig::default())`. Collapsing that to
                // `None` would downgrade the message to V0 on the wire.
                VersionedMessage::V1(msg) => MessageParts {
                    header: msg.header,
                    account_keys: msg.account_keys,
                    lifetime: msg.lifetime_specifier.as_ref().to_vec(),
                    instructions: msg.instructions,
                    versioned: true,
                    address_table_lookups: vec![],
                    config: Some(proto::TransactionConfig {
                        priority_fee: msg.config.priority_fee,
                        compute_unit_limit: msg.config.compute_unit_limit,
                        loaded_accounts_data_size_limit: msg.config.loaded_accounts_data_size_limit,
                        heap_size: msg.config.heap_size,
                    }),
                },
            };

            proto::Message {
                header: Some(proto::MessageHeader {
                    num_required_signatures: parts.header.num_required_signatures as u32,
                    num_readonly_signed_accounts: parts.header.num_readonly_signed_accounts as u32,
                    num_readonly_unsigned_accounts: parts.header.num_readonly_unsigned_accounts
                        as u32,
                }),
                account_keys: parts
                    .account_keys
                    .iter()
                    .map(|k| k.as_ref().to_vec())
                    .collect(),
                recent_blockhash: parts.lifetime,
                instructions: parts
                    .instructions
                    .into_iter()
                    .map(|ix| proto::CompiledInstruction {
                        program_id_index: ix.program_id_index as u32,
                        accounts: ix.accounts,
                        data: ix.data,
                    })
                    .collect(),
                versioned: parts.versioned,
                address_table_lookups: parts.address_table_lookups,
                config: parts.config,
            }
        };

        proto::Transaction {
            signatures,
            message: Some(message),
        }
    }

    pub fn transaction_status_meta(meta: TransactionStatusMeta) -> proto::TransactionStatusMeta {
        let inner_instructions_none = meta.inner_instructions.is_none();
        let log_messages_none = meta.log_messages.is_none();
        let return_data_none = meta.return_data.is_none();

        proto::TransactionStatusMeta {
            err: meta.status.err().map(|e| proto::TransactionError {
                err: bincode::serialize(&e).unwrap_or_default(),
            }),
            fee: meta.fee,
            pre_balances: meta.pre_balances,
            post_balances: meta.post_balances,
            inner_instructions: meta
                .inner_instructions
                .into_iter()
                .flatten()
                .map(|ix| proto::InnerInstructions {
                    index: ix.index as u32,
                    instructions: ix
                        .instructions
                        .into_iter()
                        .map(|i| proto::InnerInstruction {
                            program_id_index: i.instruction.program_id_index as u32,
                            accounts: i.instruction.accounts,
                            data: i.instruction.data,
                            stack_height: i.stack_height,
                        })
                        .collect(),
                })
                .collect(),
            inner_instructions_none,
            log_messages: meta.log_messages.unwrap_or_default(),
            log_messages_none,
            pre_token_balances: meta
                .pre_token_balances
                .into_iter()
                .flatten()
                .map(convert_token_balance)
                .collect(),
            post_token_balances: meta
                .post_token_balances
                .into_iter()
                .flatten()
                .map(convert_token_balance)
                .collect(),
            rewards: meta
                .rewards
                .into_iter()
                .flatten()
                .map(|r| proto::Reward {
                    pubkey: r.pubkey,
                    lamports: r.lamports,
                    post_balance: r.post_balance,
                    reward_type: match r.reward_type {
                        Some(RewardType::Fee) => proto::RewardType::Fee as i32,
                        Some(RewardType::Rent) => proto::RewardType::Rent as i32,
                        Some(RewardType::Staking) => proto::RewardType::Staking as i32,
                        Some(RewardType::Voting) => proto::RewardType::Voting as i32,
                        Some(RewardType::DeactivatedStake) => {
                            proto::RewardType::DeactivatedStake as i32
                        },
                        None => proto::RewardType::Unspecified as i32,
                    },
                    // Unlike `RewardInfo`, `solana_transaction_status::Reward`
                    // kept both fields, so each one is forwarded from its own
                    // source rather than derived from the other.
                    commission: r.commission.map(|c| c.to_string()).unwrap_or_default(),
                    commission_bps: r
                        .commission_bps
                        .map(|bps| bps.to_string())
                        .unwrap_or_default(),
                })
                .collect(),
            loaded_writable_addresses: meta
                .loaded_addresses
                .writable
                .iter()
                .map(|k| k.as_ref().to_vec())
                .collect(),
            loaded_readonly_addresses: meta
                .loaded_addresses
                .readonly
                .iter()
                .map(|k| k.as_ref().to_vec())
                .collect(),
            return_data: meta.return_data.map(|r| proto::ReturnData {
                program_id: r.program_id.as_ref().to_vec(),
                data: r.data,
            }),
            return_data_none,
            compute_units_consumed: meta.compute_units_consumed,
            cost_units: None,
        }
    }

    fn convert_token_balance(tb: TransactionTokenBalance) -> proto::TokenBalance {
        proto::TokenBalance {
            account_index: tb.account_index as u32,
            mint: tb.mint,
            ui_token_amount: Some(proto::UiTokenAmount {
                ui_amount: tb.ui_token_amount.ui_amount.unwrap_or_default(),
                decimals: tb.ui_token_amount.decimals as u32,
                amount: tb.ui_token_amount.amount,
                ui_amount_string: tb.ui_token_amount.ui_amount_string,
            }),
            owner: tb.owner,
            program_id: tb.program_id,
        }
    }
}
