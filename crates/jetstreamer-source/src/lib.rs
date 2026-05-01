use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use async_trait::async_trait;
use futures_util::FutureExt;
use jetstreamer_firehose::firehose::{firehose, BlockData, OnErrorFn, TransactionData};
use tokio::sync::{broadcast, mpsc, mpsc::Sender, oneshot};
use tracing::{debug, error, info};
use yellowstone_grpc_proto::{
    geyser::{subscribe_update::UpdateOneof, SubscribeUpdate, SubscribeUpdateBlock},
    solana::storage::confirmed_block::{BlockHeight, UnixTimestamp},
};
use yellowstone_vixen::{
    sources::{SourceExitStatus, SourceTrait},
    Error as VixenError,
};
use yellowstone_vixen_core::Filters;

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
///     unsafe { yellowstone_vixen_jetstream_source::init_process_env(&config) };
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

struct VixenStreamHandler {
    tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
    skipped_slots_tx: Option<mpsc::Sender<PossibleLeaderSkippedEvent>>,
    // Cache matching filters to avoid iteration per item
    block_matches: Vec<String>,
    transaction_matches: Vec<String>,
    // Counter for progress logging
    blocks_processed: AtomicU64,
}

impl VixenStreamHandler {
    fn new(
        tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
        skipped_slots_tx: Option<mpsc::Sender<PossibleLeaderSkippedEvent>>,
        filters: Filters,
    ) -> Self {
        let (block_matches, transaction_matches) = Self::precalculate_filters(&filters);

        info!(
            block_filters = block_matches.len(),
            transaction_filters = transaction_matches.len(),
            "Initialized VixenStreamHandler with cached filters"
        );

        Self {
            tx,
            skipped_slots_tx,
            block_matches,
            transaction_matches,
            blocks_processed: AtomicU64::new(0),
        }
    }

    fn precalculate_filters(filters: &Filters) -> (Vec<String>, Vec<String>) {
        let mut block_matches = Vec::new();
        let mut transaction_matches = Vec::new();

        for (filter_id, prefilter) in &filters.parsers_filters {
            // 1. Calculate Block Matches
            let mut block_match = false;
            if let Some(block_filter) = &prefilter.block
                && (block_filter.include_transactions
                    || block_filter.include_accounts
                    || block_filter.include_entries)
            {
                block_match = true;
            }
            if prefilter.block_meta.is_some() || prefilter.slot.is_some() {
                block_match = true;
            }

            if block_match {
                block_matches.push(filter_id.clone());
            }

            // 2. Calculate Transaction Matches
            // Instruction parsers need transactions to extract instructions from,
            // so any parser with a transaction filter must receive all transactions.
            // The jetstreamer-firehose API does not support per-account filtering,
            // so we include all transactions whenever a transaction filter is present.
            if prefilter.transaction.is_some() {
                transaction_matches.push(filter_id.clone());
            }
        }

        (block_matches, transaction_matches)
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
                // Use cached matches
                if self.block_matches.is_empty() {
                    debug!(slot, "No filters interested in block, skipping");

                    // Log progress every 10,000 blocks even if skipped
                    let count = self.blocks_processed.fetch_add(1, Ordering::Relaxed);
                    if count.is_multiple_of(10_000) && count > 0 {
                        debug!(slot, count, "Processed blocks (skipping non-matching)");
                    }

                    return Ok(());
                }

                // Log progress every 10,000 blocks
                let count = self.blocks_processed.fetch_add(1, Ordering::Relaxed);
                if count.is_multiple_of(10_000) && count > 0 {
                    debug!(slot, count, "Processed blocks (found matches)");
                }

                let update = SubscribeUpdate {
                    filters: self.block_matches.clone(),
                    update_oneof: Some(UpdateOneof::Block(SubscribeUpdateBlock {
                        slot,
                        blockhash: blockhash.to_string(),
                        rewards: Some(
                            yellowstone_grpc_proto::solana::storage::confirmed_block::Rewards {
                                rewards: vec![],
                                num_partitions: rewards.num_partitions.map(|np| {
                                    yellowstone_grpc_proto::solana::storage::confirmed_block::NumPartitions {
                                        num_partitions: np,
                                    }
                                }),
                            },
                        ),
                        block_time: block_time.map(|bt| UnixTimestamp { timestamp: bt }),
                        block_height: block_height.map(|bh| BlockHeight { block_height: bh }),
                        executed_transaction_count,
                        transactions: vec![],
                        updated_account_count: 0,
                        accounts: vec![],
                        entries: vec![],
                        entries_count: entry_count,
                        parent_slot,
                        parent_blockhash: parent_blockhash.to_string(),
                    })),
                    created_at: Some(yellowstone_grpc_proto::prost_types::Timestamp::from(
                        std::time::SystemTime::now(),
                    )),
                };

                debug!(
                    slot,
                    filters = ?self.block_matches,
                    "Sending block update with {} filter matches",
                    self.block_matches.len()
                );

                self.tx.send(Ok(update)).await.map_err(|e| {
                    let error_msg = format!("Failed to send block update: {}", e);
                    error!("{}", error_msg);
                    Box::new(Error::ChannelSend(error_msg)) as SharedError
                })?;
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

        let update = SubscribeUpdate {
            filters: self.transaction_matches.clone(),
            update_oneof: Some(UpdateOneof::Transaction(
                yellowstone_grpc_proto::geyser::SubscribeUpdateTransaction {
                    slot: tx_data.slot,
                    transaction: transaction_info,
                },
            )),
            created_at: Some(yellowstone_grpc_proto::prost_types::Timestamp::from(
                std::time::SystemTime::now(),
            )),
        };

        debug!(
            slot = tx_data.slot,
            filters = ?self.transaction_matches,
            "Sending transaction update with {} filter matches",
            self.transaction_matches.len()
        );

        self.tx.send(Ok(update)).await.map_err(|e| {
            let error_msg = format!("Failed to send transaction update: {}", e);
            error!("{}", error_msg);
            Box::new(Error::ChannelSend(error_msg)) as SharedError
        })?;

        Ok(())
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

    /// Ripget hot/cold window size in bytes when `sequential` is enabled.
    /// Ignored when `sequential` is `false`. `None` uses the upstream
    /// default (`min(4 GiB, 15% of available RAM)`).
    #[arg(long, env)]
    #[serde(default)]
    pub buffer_window_bytes: Option<u64>,

    /// Optional side channel for `PossibleLeaderSkipped` events.
    #[serde(skip)]
    #[arg(skip)]
    pub possible_leader_skipped_tx: Option<mpsc::Sender<PossibleLeaderSkippedEvent>>,
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

    // TODO: plumb a caller-provided CancellationToken into `connect` once
    // `SourceTrait::connect` exposes a cancellation hook. Upstream
    // `firehose()` accepts an `Option<broadcast::Receiver<()>>` for
    // cooperative shutdown but we currently pass `None`.
    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
    ) -> Result<(), VixenError> {
        let config = self.config.clone();
        let filters = self.filters.clone();

        // jetstreamer-firehose reads configuration exclusively through env vars.
        // The caller must have set them *before* the runtime started via
        // `init_process_env()`. We only validate here — no mutation.
        {
            let expected = ProcessEnvConfig::from_config(&config);
            expected.validate_matches(&ProcessEnvConfig::from_process())?;
        }

        if config.buffer_window_bytes.is_some() && !config.sequential {
            tracing::warn!(
                "`buffer_window_bytes` is set but `sequential` is false; the value will be ignored by jetstreamer-firehose"
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

        let handler = Arc::new(VixenStreamHandler::new(
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

        let result = firehose(
            config.threads as u64,
            config.sequential,
            config.buffer_window_bytes,
            start_slot..end_slot,
            on_block,
            on_tx,
            None::<jetstreamer_firehose::firehose::OnEntryFn>,
            None::<jetstreamer_firehose::firehose::OnRewardFn>,
            None::<OnErrorFn>,
            None::<
                jetstreamer_firehose::firehose::StatsTracking<
                    jetstreamer_firehose::firehose::HandlerFn<
                        jetstreamer_firehose::firehose::Stats,
                    >,
                >,
            >,
            None::<broadcast::Receiver<()>>,
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
}

impl From<Error> for VixenError {
    fn from(e: Error) -> Self {
        match e {
            Error::Io(io_err) => VixenError::Io(io_err),
            // VixenError only exposes an Io variant for generic errors.
            // Wrap with `io::Error::other` but preserve the original error as
            // the source (via `Box<dyn Error>`) so callers can still downcast.
            other => VixenError::Io(std::io::Error::other(other)),
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
            buffer_window_bytes: None,
            possible_leader_skipped_tx: None,
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
buffer-window-bytes = 1073741824

[range]
slot-start = 1000
slot-end = 2000
"#;

        let config: JetstreamSourceConfig =
            toml::from_str(toml_str).expect("valid TOML for JetstreamSourceConfig");

        assert!(config.sequential);
        assert_eq!(config.buffer_window_bytes, Some(1_073_741_824));
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
            config.buffer_window_bytes.is_none(),
            "`buffer_window_bytes` must default to None when absent from TOML"
        );
    }

    #[tokio::test]
    async fn possible_leader_skipped_events_use_side_channel() {
        let (updates_tx, mut updates_rx) = mpsc::channel(4);
        let (skipped_tx, mut skipped_rx) = mpsc::channel(4);
        let handler = VixenStreamHandler::new(
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
}

mod convert {
    use solana_message::VersionedMessage;
    use solana_runtime::bank::RewardType;
    use solana_transaction::versioned::VersionedTransaction;
    use solana_transaction_status::{TransactionStatusMeta, TransactionTokenBalance};
    use yellowstone_grpc_proto::solana::storage::confirmed_block as proto;

    pub fn transaction(tx: VersionedTransaction) -> proto::Transaction {
        let signatures = tx.signatures.iter().map(|s| s.as_ref().to_vec()).collect();

        let message = {
            let (
                header,
                account_keys,
                recent_blockhash,
                instructions,
                versioned,
                address_table_lookups,
            ) = match tx.message {
                VersionedMessage::Legacy(msg) => (
                    msg.header,
                    msg.account_keys,
                    msg.recent_blockhash,
                    msg.instructions,
                    false,
                    vec![],
                ),
                VersionedMessage::V0(msg) => (
                    msg.header,
                    msg.account_keys,
                    msg.recent_blockhash,
                    msg.instructions,
                    true,
                    msg.address_table_lookups
                        .into_iter()
                        .map(|l| proto::MessageAddressTableLookup {
                            account_key: l.account_key.as_ref().to_vec(),
                            writable_indexes: l.writable_indexes,
                            readonly_indexes: l.readonly_indexes,
                        })
                        .collect(),
                ),
            };

            proto::Message {
                header: Some(proto::MessageHeader {
                    num_required_signatures: header.num_required_signatures as u32,
                    num_readonly_signed_accounts: header.num_readonly_signed_accounts as u32,
                    num_readonly_unsigned_accounts: header.num_readonly_unsigned_accounts as u32,
                }),
                account_keys: account_keys.iter().map(|k| k.as_ref().to_vec()).collect(),
                recent_blockhash: recent_blockhash.as_ref().to_vec(),
                instructions: instructions
                    .into_iter()
                    .map(|ix| proto::CompiledInstruction {
                        program_id_index: ix.program_id_index as u32,
                        accounts: ix.accounts,
                        data: ix.data,
                    })
                    .collect(),
                versioned,
                address_table_lookups,
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
                        _ => proto::RewardType::Unspecified as i32,
                    },
                    commission: r.commission.map(|c| c.to_string()).unwrap_or_default(),
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
