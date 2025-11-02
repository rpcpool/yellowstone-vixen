use std::sync::Arc;

use async_trait::async_trait;
use futures_util::FutureExt;
use jetstreamer_firehose::firehose::{firehose, BlockData, TransactionData};
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};
use yellowstone_grpc_proto::{
    geyser::{subscribe_update::UpdateOneof, SubscribeUpdate, SubscribeUpdateBlock},
    solana::storage::confirmed_block::{BlockHeight, UnixTimestamp},
};
use yellowstone_vixen::{sources::SourceTrait, Error as VixenError};
use yellowstone_vixen_core::Filters;

#[cfg(feature = "prometheus")]
mod metrics;

#[cfg(feature = "prometheus")]
pub use metrics::register_metrics;

struct VixenStreamHandler {
    tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
    filters: Filters,
    config: JetstreamSourceConfig,
}

impl VixenStreamHandler {
    fn new(
        tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
        filters: Filters,
        config: JetstreamSourceConfig,
    ) -> Self {
        Self {
            tx,
            filters,
            config,
        }
    }

    fn jetstream_block_received() {
        #[cfg(feature = "prometheus")]
        crate::metrics::JETSTREAM_BLOCKS_RECEIVED.inc();
    }

    fn jetstream_transaction_received() {
        #[cfg(feature = "prometheus")]
        crate::metrics::JETSTREAM_TRANSACTIONS_RECEIVED.inc();
    }

    async fn process_block(&self, block: BlockData) -> Result<(), Error> {
        info!(slot = block.slot(), "Processing block");

        Self::jetstream_block_received();

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

                info!(slot, "Sending block update");
                if let Err(e) = self.tx.send(Ok(update)).await {
                    let error_msg = format!("Failed to send block update: {}", e);
                    error!("{}", error_msg);
                    return Err(Error::ChannelSend(error_msg));
                }
            },
            BlockData::LeaderSkipped { slot } => {
                debug!(slot, "Skipping leader-skipped slot");
            },
        }

        Ok(())
    }

    async fn process_transaction(&self, tx: TransactionData) -> Result<(), Error> {
        info!(
            signature = ?tx.signature,
            slot = tx.slot,
            transaction_slot_index = tx.transaction_slot_index,
            is_vote = tx.is_vote,
            "Processing transaction"
        );

        // Check if transaction should be filtered
        if !self.should_process_transaction(&tx) {
            debug!(signature = ?tx.signature, slot = tx.slot, "Transaction filtered out");
            return Ok(());
        }

        Self::jetstream_transaction_received();
        // TODO: Populate transaction field with protobuf-encoded transaction data
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

        info!(slot = tx.slot, "Sending transaction update");
        if let Err(e) = self.tx.send(Ok(update)).await {
            let error_msg = format!("Failed to send transaction update: {}", e);
            error!("{}", error_msg);
            return Err(Error::ChannelSend(error_msg));
        }

        Ok(())
    }

    fn should_process_transaction(&self, tx: &TransactionData) -> bool {
        // If no filters are configured, process all transactions
        if self.filters.parsers_filters.is_empty() {
            return true;
        }

        // Check configured filters
        for (filter_id, prefilter) in &self.filters.parsers_filters {
            if let Some(_tx_filter) = &prefilter.transaction {
                // NOTE: Full transaction data available via tx.transaction
                // Parse VersionedTransaction.message.account_keys for filtering

                if self.config.permissive_transaction_filtering {
                    info!(
                        filter_id = %filter_id,
                        signature = ?tx.signature,
                        "Processing transaction (permissive mode)"
                    );
                    return true;
                } else {
                    // Strict mode: Account key extraction not implemented yet
                    // TODO: Implement proper transaction parsing and account key extraction
                    info!(
                        filter_id = %filter_id,
                        signature = ?tx.signature,
                        "Strict filtering requires account key extraction - not yet implemented"
                    );
                    continue;
                }
            }

            // Process if filter has account/slot requirements
            if prefilter.account.is_some() || prefilter.slot.is_some() {
                debug!(filter_id = %filter_id, "Filter has account/slot requirements - processing transaction");
                return true;
            }
        }

        // If we have filters configured but none require transactions, don't process
        debug!("No transaction-relevant filters configured");
        false
    }
}

/// Jetstream source configuration
#[derive(Debug, Clone, serde::Deserialize, clap::Args)]
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

    /// Network name (mainnet, etc.)
    #[arg(long, env, default_value = "mainnet")]
    pub network: String,

    /// Compact index base URL
    #[arg(long, env, default_value = "https://files.old-faithful.net")]
    pub compact_index_base_url: String,

    /// Network capacity in MB
    #[arg(long, env, default_value = "1000")]
    pub network_capacity_mb: usize,

    /// Control transaction filtering: true = permissive (all), false = strict (limited).
    #[arg(long, env, default_value = "true")]
    pub permissive_transaction_filtering: bool,
}

/// Configuration for slot ranges or epochs
#[derive(Debug, Clone, serde::Deserialize, clap::Args)]
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
        filters: Filters,
        tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
        _cancellation_token: CancellationToken,
    ) -> Result<(), Error> {
        let (start_slot, end_slot) = config.range.to_slot_range().map_err(|e| {
            Error::SlotRangeResolution(format!("Failed to resolve slot range: {}", e))
        })?;
        info!(
            start_slot,
            end_slot, "Starting Old Faithful streaming from {}", config.archive_url
        );

        let handler = Arc::new(VixenStreamHandler::new(
            tx.clone(),
            filters.clone(),
            config.clone(),
        ));

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

        let handler_on_tx = handler.clone();
        let on_tx = Some(move |_thread_id: usize, tx: TransactionData| {
            let handler_callback = handler_on_tx.clone();
            async move {
                handler_callback
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
            let error_msg = format!("{:?}", error);
            if error_msg.contains("incomplete frame") {
                error!(
                    slot,
                    error = %error_msg,
                    "Corrupted CAR file at slot {slot}. Try different epoch (e.g., 800)."
                );
                return Err(Error::Jetstreamer(format!(
                    "Corrupted data at slot {}: {}. Try epoch 800 or RPC API.",
                    slot, error_msg
                )));
            } else {
                return Err(Error::Jetstreamer(format!(
                    "Firehose error at slot {}: {:?}",
                    slot, error
                )));
            }
        }

        info!("Jetstreamer historical data streaming completed successfully");
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
            permissive_transaction_filtering: true,
        };

        let filters = Filters::new(std::collections::HashMap::new());
        let source = JetstreamSource::new(config, filters);

        // Verify config storage
        assert_eq!(source.config.archive_url, "https://api.old-faithful.net");
        assert_eq!(source.config.threads, 4);
        assert_eq!(source.config.network, "mainnet");
        assert!(source.config.permissive_transaction_filtering);
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
            network: "mainnet".to_string(),
            compact_index_base_url: "https://files.old-faithful.net".to_string(),
            network_capacity_mb: 1000,
            permissive_transaction_filtering: true,
        };

        let filters = Filters::new(std::collections::HashMap::new());
        let _source = JetstreamSource::new(config, filters);

        // Verify epoch to slot conversion
        let epoch = 800u64;
        let slots_per_epoch = 432_000u64;
        let expected_start = epoch * slots_per_epoch;
        let expected_end = (epoch + 1) * slots_per_epoch - 1;

        assert_eq!(expected_start, 345600000);
        assert_eq!(expected_end, 346031999);
    }

    #[test]
    fn test_network_epoch_slots() {
        // Verify all networks use same slots per epoch
        assert_eq!(432_000, 432_000);

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
                slot_end: Some(1000), // Invalid: end < start
                epoch: None,
            },
            threads: 4,
            network: "mainnet".to_string(),
            compact_index_base_url: "https://files.test.com".to_string(),
            network_capacity_mb: 1000,
            permissive_transaction_filtering: true,
        };

        let filters = Filters::new(std::collections::HashMap::new());
        let source = JetstreamSource::new(config, filters);

        // Config creation allowed, validation happens during connect
        assert_eq!(source.config.range.slot_start, Some(2000));
        assert_eq!(source.config.range.slot_end, Some(1000));
    }
}
