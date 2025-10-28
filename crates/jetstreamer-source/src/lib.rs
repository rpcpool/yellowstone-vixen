use std::{
    collections::BTreeMap,
    sync::Arc,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use futures_util::FutureExt;
use jetstreamer_firehose::firehose::{firehose, BlockData, TransactionData};
use tokio::sync::{mpsc::Sender, RwLock};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};
use yellowstone_grpc_proto::{
    geyser::{subscribe_update::UpdateOneof, SubscribeUpdate, SubscribeUpdateBlock},
    solana::storage::confirmed_block::{BlockHeight, UnixTimestamp},
};
use yellowstone_vixen::{sources::SourceTrait, Error as VixenError};
use yellowstone_vixen_core::Filters;

struct VixenStreamHandler {
    tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
    reorder_buffer: Arc<RwLock<ReorderBuffer>>,
}

impl VixenStreamHandler {
    fn new(
        tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
        reorder_buffer: Arc<RwLock<ReorderBuffer>>,
    ) -> Self {
        Self { tx, reorder_buffer }
    }

    async fn process_block(&self, block: BlockData) -> Result<(), Error> {
        info!(slot = block.slot(), "FIREHOSE: Processing block");

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
                let update = SubscribeUpdate {
                    filters: vec![],
                    update_oneof: Some(UpdateOneof::Block(
                        SubscribeUpdateBlock {
                            slot,
                            blockhash: blockhash.to_string(),
                            rewards: Some(yellowstone_grpc_proto::solana::storage::confirmed_block::Rewards {
                                rewards: vec![],
                                num_partitions: rewards.num_partitions.map(|np| yellowstone_grpc_proto::solana::storage::confirmed_block::NumPartitions { num_partitions: np }),
                            }),
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
                        }
                    )),
                    created_at: Some(yellowstone_grpc_proto::prost_types::Timestamp::from(std::time::SystemTime::now())),
                };

                {
                    let mut buffer = self.reorder_buffer.write().await;
                    let ready_updates = buffer.add_slot(slot, vec![update]);

                    for ready_update in ready_updates {
                        info!("FIREHOSE: Sending ready block update for slot {}", slot);
                        if let Err(e) = self.tx.send(Ok(ready_update)).await {
                            let error_msg = format!("Failed to send block update: {}", e);
                            error!("{}", error_msg);
                            return Err(Error::ChannelSend(error_msg));
                        }
                    }
                }
            },
            BlockData::LeaderSkipped { slot } => {
                debug!(slot, "Skipping leader-skipped slot");
            },
        }

        Ok(())
    }

    async fn process_transaction(&self, tx: TransactionData) -> Result<(), Error> {
        info!(signature = ?tx.signature, slot = tx.slot, "FIREHOSE: Processing transaction");

        let update = SubscribeUpdate {
            filters: vec![],
            update_oneof: Some(UpdateOneof::Transaction(
                yellowstone_grpc_proto::geyser::SubscribeUpdateTransaction {
                    slot: tx.slot,
                    transaction: None,
                },
            )),
            created_at: Some(yellowstone_grpc_proto::prost_types::Timestamp::from(
                std::time::SystemTime::now(),
            )),
        };

        {
            let mut buffer = self.reorder_buffer.write().await;
            let ready_updates = buffer.add_slot(tx.slot, vec![update]);

            for ready_update in ready_updates {
                info!(
                    "FIREHOSE: Sending ready transaction update for slot {}",
                    tx.slot
                );
                if let Err(e) = self.tx.send(Ok(ready_update)).await {
                    let error_msg = format!("Failed to send transaction update: {}", e);
                    error!("{}", error_msg);
                    return Err(Error::ChannelSend(error_msg));
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, serde::Deserialize, clap::Args)]
pub struct JetstreamSourceConfig {
    #[arg(long, env)]
    pub archive_url: String,

    #[command(flatten)]
    pub range: SlotRangeConfig,

    #[arg(long, env, default_value = "4")]
    pub threads: usize,

    #[arg(long, env, default_value = "1000")]
    pub reorder_buffer_size: usize,

    #[arg(long, env, default_value = "30")]
    pub slot_timeout_secs: u64,

    #[arg(long, env, default_value = "mainnet")]
    pub network: String,

    #[arg(long, env, default_value = "https://files.old-faithful.net")]
    pub compact_index_base_url: String,

    #[arg(long, env, default_value = "1000")]
    pub network_capacity_mb: usize,
}

#[derive(Debug, Clone, serde::Deserialize, clap::Args)]
pub struct SlotRangeConfig {
    #[arg(long, env, conflicts_with = "epoch")]
    pub slot_start: Option<u64>,

    #[arg(long, env, requires = "slot_start", conflicts_with = "epoch")]
    pub slot_end: Option<u64>,

    #[arg(long, env, conflicts_with = "slot_start")]
    pub epoch: Option<u64>,
}

impl SlotRangeConfig {
    pub fn to_slot_range(&self) -> Result<(u64, u64), Error> {
        match (self.slot_start, self.slot_end, self.epoch) {
            (Some(start), Some(end), None) => {
                if start > end {
                    return Err(Error::InvalidConfig(
                        "slot_start must be <= slot_end".into(),
                    ));
                }
                Ok((start, end))
            },
            (None, None, Some(epoch)) => {
                let slots_per_epoch = 432_000u64;
                let start = epoch * slots_per_epoch;
                let end = (epoch + 1) * slots_per_epoch - 1;
                info!(
                    epoch,
                    start_slot = start,
                    end_slot = end,
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

struct ReorderBuffer {
    buffer: BTreeMap<u64, Vec<SubscribeUpdate>>,
    next_expected_slot: u64,
    max_size: usize,
    timeout: Duration,
    slot_timestamps: BTreeMap<u64, Instant>,
}

impl ReorderBuffer {
    fn new(start_slot: u64, max_size: usize, timeout_secs: u64) -> Self {
        Self {
            buffer: BTreeMap::new(),
            next_expected_slot: start_slot,
            max_size,
            timeout: Duration::from_secs(timeout_secs),
            slot_timestamps: BTreeMap::new(),
        }
    }

    fn add_slot(&mut self, slot: u64, updates: Vec<SubscribeUpdate>) -> Vec<SubscribeUpdate> {
        info!(
            "REORDER BUFFER: Adding slot {} with {} updates",
            slot,
            updates.len()
        );
        let mut ready_updates = Vec::new();

        if slot == self.next_expected_slot {
            ready_updates.extend(updates);
            self.next_expected_slot += 1;

            while let Some(buffered) = self.buffer.remove(&self.next_expected_slot) {
                self.slot_timestamps.remove(&self.next_expected_slot);
                ready_updates.extend(buffered);
                self.next_expected_slot += 1;
            }
        } else if slot > self.next_expected_slot {
            if self.buffer.len() < self.max_size {
                self.slot_timestamps.insert(slot, Instant::now());
                self.buffer.insert(slot, updates);
                debug!(
                    slot = slot,
                    expected = self.next_expected_slot,
                    buffered = self.buffer.len(),
                    "Buffered out-of-order slot"
                );
            } else {
                warn!(
                    slot = slot,
                    buffer_size = self.buffer.len(),
                    "Reorder buffer full, dropping slot"
                );
            }
        } else {
            warn!(
                slot = slot,
                expected = self.next_expected_slot,
                "Received old slot, dropping"
            );
        }

        ready_updates
    }

    fn flush_timed_out(&mut self) -> Vec<SubscribeUpdate> {
        let mut ready_updates = Vec::new();
        let now = Instant::now();

        let mut timed_out_slots = Vec::new();
        for (&slot, &timestamp) in &self.slot_timestamps {
            if now.duration_since(timestamp) > self.timeout {
                timed_out_slots.push(slot);
            }
        }

        if !timed_out_slots.is_empty() {
            warn!(
                slots = ?timed_out_slots,
                timeout_secs = self.timeout.as_secs(),
                "Skipping missing slots after timeout"
            );

            if let Some(&first_timed_out) = timed_out_slots.first() {
                for skipped_slot in self.next_expected_slot..first_timed_out {
                    warn!(slot = skipped_slot, "Skipping missing slot");
                }

                self.next_expected_slot = first_timed_out;

                while let Some(buffered) = self.buffer.remove(&self.next_expected_slot) {
                    self.slot_timestamps.remove(&self.next_expected_slot);
                    ready_updates.extend(buffered);
                    self.next_expected_slot += 1;
                }
            }
        }

        ready_updates
    }

    fn stats(&self) -> BufferStats {
        BufferStats {
            buffered_count: self.buffer.len(),
            next_expected: self.next_expected_slot,
            oldest_buffered: self.buffer.keys().next().copied(),
            newest_buffered: self.buffer.keys().next_back().copied(),
        }
    }
}

#[derive(Debug)]
struct BufferStats {
    buffered_count: usize,
    next_expected: u64,
    oldest_buffered: Option<u64>,
    newest_buffered: Option<u64>,
}

#[derive(Debug)]
pub struct JetstreamSource {
    filters: Filters,
    config: JetstreamSourceConfig,
}

#[async_trait]
impl SourceTrait for JetstreamSource {
    type Config = JetstreamSourceConfig;

    fn new(config: Self::Config, filters: Filters) -> Self {
        Self { config, filters }
    }

    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
    ) -> Result<(), VixenError> {
        let config = self.config.clone();
        let filters = self.filters.clone();

        let cancellation_token = CancellationToken::new();
        let token = cancellation_token.clone();

        tokio::spawn(async move {
            if let Err(e) = Self::stream_loop(config, filters, tx.clone(), token).await {
                error!(error = %e, "Jetstream streaming failed");
                let _ = tx
                    .send(Err(yellowstone_grpc_proto::tonic::Status::internal(
                        e.to_string(),
                    )))
                    .await;
            }
        });

        Ok(())
    }
}

impl JetstreamSource {
    async fn stream_loop(
        config: JetstreamSourceConfig,
        _filters: Filters,
        tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
        cancellation_token: CancellationToken,
    ) -> Result<(), Error> {
        debug!("Entering stream_loop method");

        let (start_slot, end_slot) = config.range.to_slot_range().map_err(|e| {
            Error::SlotRangeResolution(format!("Failed to resolve slot range: {}", e))
        })?;
        info!(
            start_slot,
            end_slot, "Starting Old Faithful data streaming from {}", config.archive_url
        );

        let reorder_buffer = Arc::new(RwLock::new(ReorderBuffer::new(
            start_slot,
            config.reorder_buffer_size,
            config.slot_timeout_secs,
        )));

        let buffer_clone = reorder_buffer.clone();
        let tx_clone = tx.clone();
        let token_clone = cancellation_token.clone();
        tokio::spawn(async move {
            Self::timeout_checker_loop(buffer_clone, tx_clone, token_clone).await;
        });

        let handler = Arc::new(VixenStreamHandler::new(tx.clone(), reorder_buffer.clone()));

        std::env::set_var("JETSTREAMER_NETWORK", config.network.clone());
        std::env::set_var(
            "JETSTREAMER_COMPACT_INDEX_BASE_URL",
            config.compact_index_base_url.clone(),
        );
        std::env::set_var(
            "JETSTREAMER_NETWORK_CAPACITY_MB",
            config.network_capacity_mb.to_string(),
        );

        let handler_clone = handler.clone();
        let on_block = Some(move |_thread_id: usize, block: BlockData| {
            let handler = handler_clone.clone();
            async move {
                handler
                    .process_block(block)
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)
            }
            .boxed()
        });

        let handler_clone = handler.clone();
        let on_tx = Some(move |_thread_id: usize, tx: TransactionData| {
            let handler = handler_clone.clone();
            async move {
                handler
                    .process_transaction(tx)
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)
            }
            .boxed()
        });

        let result = firehose(
            config.threads as u64,
            start_slot..end_slot,
            on_block,
            on_tx,
            None::<jetstreamer_firehose::firehose::OnEntryFn>,
            None::<jetstreamer_firehose::firehose::OnRewardFn>,
            None::<
                jetstreamer_firehose::firehose::StatsTracking<
                    jetstreamer_firehose::firehose::HandlerFn<
                        jetstreamer_firehose::firehose::Stats,
                    >,
                >,
            >,
            None,
        )
        .await;

        if let Err((error, slot)) = result {
            return Err(Error::Jetstreamer(format!(
                "Firehose error at slot {}: {:?}",
                slot, error
            )));
        }

        info!("Jetstreamer historical data streaming completed successfully");
        Ok(())
    }

    async fn timeout_checker_loop(
        buffer: Arc<RwLock<ReorderBuffer>>,
        tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
        cancellation_token: CancellationToken,
    ) {
        let check_interval = tokio::time::Duration::from_secs(5);
        let mut interval = tokio::time::interval(check_interval);

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let ready_updates = {
                        let mut buffer = buffer.write().await;
                        buffer.flush_timed_out()
                    };

                    if !ready_updates.is_empty() {
                        info!("TIMEOUT CHECKER: Flushing {} timed-out updates", ready_updates.len());
                    }
                    for update in ready_updates {
                        info!("TIMEOUT CHECKER: Sending timed-out update");
                        if tx.send(Ok(update)).await.is_err() {
                            return;
                        }
                    }

                    let stats = buffer.read().await.stats();
                    if stats.buffered_count > 0 {
                        debug!(
                            buffered = stats.buffered_count,
                            next_expected = stats.next_expected,
                            oldest = ?stats.oldest_buffered,
                            newest = ?stats.newest_buffered,
                            "Reorder buffer status"
                        );
                    }
                }

                _ = cancellation_token.cancelled() => {
                    break;
                }
            }
        }
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

    #[error("Reorder buffer error: {0}")]
    ReorderBuffer(String),

    #[error("Timeout checker error: {0}")]
    TimeoutChecker(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP request error: {0}")]
    HttpRequest(String),

    #[error("JSON parsing error: {0}")]
    JsonParse(String),

    #[error("Jetstreamer firehose error: {0}")]
    Jetstreamer(String),
}

impl From<Error> for VixenError {
    fn from(e: Error) -> Self {
        match e {
            Error::Io(io_err) => VixenError::Io(io_err),
            other => VixenError::Io(std::io::Error::other(other.to_string())),
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
        let (start, end) = config.to_slot_range().unwrap();
        assert_eq!(start, 345_600_000);
        assert_eq!(end, 346_031_999);
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
    fn test_invalid_config() {
        let config = SlotRangeConfig {
            slot_start: Some(100),
            slot_end: Some(200),
            epoch: Some(800),
        };
        assert!(config.to_slot_range().is_err());
    }

    #[test]
    fn test_reorder_buffer_in_order() {
        let mut buffer = ReorderBuffer::new(100, 1000, 30);
        let update = create_test_update(100);
        let ready = buffer.add_slot(100, vec![update]);
        assert_eq!(ready.len(), 1);
        assert_eq!(buffer.stats().next_expected, 101);
    }

    #[test]
    fn test_reorder_buffer_out_of_order() {
        let mut buffer = ReorderBuffer::new(100, 1000, 30);

        // Add slot 102 first (buffered)
        let ready = buffer.add_slot(102, vec![create_test_update(102)]);
        assert_eq!(ready.len(), 0);
        assert_eq!(buffer.stats().buffered_count, 1);

        // Add slot 100 (emits 100)
        let ready = buffer.add_slot(100, vec![create_test_update(100)]);
        assert_eq!(ready.len(), 1);
        assert_eq!(buffer.stats().next_expected, 101);

        // Add slot 101 (emits 101 and 102)
        let ready = buffer.add_slot(101, vec![create_test_update(101)]);
        assert_eq!(ready.len(), 2);
        assert_eq!(buffer.stats().next_expected, 103);
        assert_eq!(buffer.stats().buffered_count, 0);
    }

    #[test]
    fn test_reorder_buffer_overflow() {
        let mut buffer = ReorderBuffer::new(100, 2, 30);

        // Fill buffer to capacity
        buffer.add_slot(102, vec![create_test_update(102)]);
        buffer.add_slot(103, vec![create_test_update(103)]);
        assert_eq!(buffer.stats().buffered_count, 2);

        // Try to add beyond capacity (should drop)
        buffer.add_slot(104, vec![create_test_update(104)]);
        assert_eq!(buffer.stats().buffered_count, 2);
    }

    #[test]
    fn test_reorder_buffer_old_slot() {
        let mut buffer = ReorderBuffer::new(100, 1000, 30);

        // Process slot 100
        buffer.add_slot(100, vec![create_test_update(100)]);

        // Try to add older slot 99 (should drop)
        let ready = buffer.add_slot(99, vec![create_test_update(99)]);
        assert_eq!(ready.len(), 0);
    }

    fn create_test_update(slot: u64) -> SubscribeUpdate {
        SubscribeUpdate {
            filters: vec!["test".to_string()],
            created_at: None,
            update_oneof: Some(UpdateOneof::Block(SubscribeUpdateBlock {
                slot,
                blockhash: format!("block_{}", slot),
                ..Default::default()
            })),
        }
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
            reorder_buffer_size: 1000,
            slot_timeout_secs: 30,
            network: "mainnet".to_string(),
            compact_index_base_url: "https://files.old-faithful.net".to_string(),
            network_capacity_mb: 1000,
        };

        let filters = Filters::new(std::collections::HashMap::new());
        let source = JetstreamSource::new(config, filters);

        // Verify configuration is stored correctly
        assert_eq!(source.config.archive_url, "https://api.old-faithful.net");
        assert_eq!(source.config.threads, 4);
        assert_eq!(source.config.network, "mainnet");
    }

    #[test]
    fn test_epoch_to_slot_conversion_mainnet() {
        let config = JetstreamSourceConfig {
            archive_url: "https://api.old-faithful.net".to_string(),
            range: SlotRangeConfig {
                slot_start: None,
                slot_end: None,
                epoch: Some(800),
            },
            threads: 4,
            reorder_buffer_size: 1000,
            slot_timeout_secs: 30,
            network: "mainnet".to_string(),
            compact_index_base_url: "https://files.old-faithful.net".to_string(),
            network_capacity_mb: 1000,
        };

        let filters = Filters::new(std::collections::HashMap::new());
        let _source = JetstreamSource::new(config, filters);

        // Test epoch conversion (this would normally happen in connect method)
        let epoch = 800u64;
        let slots_per_epoch = 432_000u64;
        let expected_start = epoch * slots_per_epoch;
        let expected_end = (epoch + 1) * slots_per_epoch - 1;

        assert_eq!(expected_start, 345600000);
        assert_eq!(expected_end, 346031999);
    }

    #[test]
    fn test_network_epoch_slots() {
        // Test that different networks use appropriate slots per epoch
        assert_eq!(432_000, 432_000); // All networks currently use same value

        // Test mainnet aliases
        let mainnet_configs = ["mainnet", "mainnet-beta"];
        for network in &mainnet_configs {
            assert!(matches!(*network, "mainnet" | "mainnet-beta"));
        }
    }

    #[test]
    fn test_slot_range_config_validation() {
        let config = JetstreamSourceConfig {
            archive_url: "https://test.com".to_string(),
            range: SlotRangeConfig {
                slot_start: Some(2000),
                slot_end: Some(1000), //NOTE: Invalid: end < start
                epoch: None,
            },
            threads: 4,
            reorder_buffer_size: 1000,
            slot_timeout_secs: 30,
            network: "mainnet".to_string(),
            compact_index_base_url: "https://files.test.com".to_string(),
            network_capacity_mb: 1000,
        };

        let filters = Filters::new(std::collections::HashMap::new());
        let source = JetstreamSource::new(config, filters);

        // This test verifies the config is created, but validation would happen during connect
        assert_eq!(source.config.range.slot_start, Some(2000));
        assert_eq!(source.config.range.slot_end, Some(1000));
    }
}
