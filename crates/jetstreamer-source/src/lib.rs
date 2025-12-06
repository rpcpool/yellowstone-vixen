use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use async_trait::async_trait;
use futures_util::FutureExt;
use jetstreamer_firehose::firehose::{firehose, BlockData, OnErrorFn, TransactionData};
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};
use yellowstone_grpc_proto::{
    geyser::{subscribe_update::UpdateOneof, SubscribeUpdate, SubscribeUpdateBlock},
    solana::storage::confirmed_block::{BlockHeight, UnixTimestamp},
};
use yellowstone_vixen::{sources::SourceTrait, Error as VixenError};
use yellowstone_vixen_core::Filters;

type SharedError = Box<dyn std::error::Error + Send + Sync + 'static>;

struct VixenStreamHandler {
    tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
    // Cache matching filters to avoid iteration per item
    block_matches: Vec<String>,
    transaction_matches: Vec<String>,
    // Counter for progress logging
    blocks_processed: AtomicU64,
}

impl VixenStreamHandler {
    fn new(
        tx: Sender<Result<SubscribeUpdate, yellowstone_grpc_proto::tonic::Status>>,
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

            //BUG: 2. Calculate Transaction Matches
            // NOTE: Instruction parsers NEED transactions to extract instructions from!
            // We should NOT skip them here, otherwise they won't receive any transactions.

            let mut tx_match = false;
            if prefilter.transaction.is_some() {
                // Note: We cannot check account keys with current jetstreamer-firehose API
                // so we include all transactions when transaction filters are configured
                tx_match = true;
            }
            if let Some(tx_filter) = &prefilter.transaction
                && (!tx_filter.accounts_include.is_empty()
                    || !tx_filter.accounts_required.is_empty())
            {
                tx_match = true;
            }

            if tx_match {
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
                debug!(slot, "Skipping possibly leader-skipped slot");
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
                transaction: Some(convert::transaction(&tx_data.transaction)),
                meta: Some(convert::transaction_status_meta(
                    &tx_data.transaction_status_meta,
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
    /// Convert configuration to slot range
    /// Returns (start_slot, end_slot)
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
                // Mainnet/testnet use 432,000 slots per epoch
                const SLOTS_PER_EPOCH: u64 = 432_000;
                let start = epoch * SLOTS_PER_EPOCH;
                let end = (epoch + 1) * SLOTS_PER_EPOCH - 1;
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

        let handler = Arc::new(VixenStreamHandler::new(tx.clone(), filters.clone()));

        // Set environment variables for jetstreamer
        unsafe {
            std::env::set_var("JETSTREAMER_NETWORK", &config.network);
            std::env::set_var(
                "JETSTREAMER_COMPACT_INDEX_BASE_URL",
                &config.compact_index_base_url,
            );
            std::env::set_var(
                "JETSTREAMER_NETWORK_CAPACITY_MB",
                config.network_capacity_mb.to_string(),
            );
        }

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
            None,
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
        };

        let filters = Filters::new(std::collections::HashMap::new());
        let source = JetstreamSource::new(config, filters);

        assert_eq!(source.config.archive_url, "https://api.old-faithful.net");
        assert_eq!(source.config.threads, 4);
        assert_eq!(source.config.network, "mainnet");
    }

    #[test]
    fn test_multiple_epochs() {
        for epoch in [800, 801, 802] {
            let config = SlotRangeConfig {
                slot_start: None,
                slot_end: None,
                epoch: Some(epoch),
            };
            let (start, end) = config.to_slot_range().unwrap();
            assert_eq!(start, epoch * 432_000);
            assert_eq!(end, (epoch + 1) * 432_000 - 1);
        }
    }
}

mod convert {
    use solana_message::VersionedMessage;
    use solana_runtime::bank::RewardType;
    use solana_transaction::versioned::VersionedTransaction;
    use solana_transaction_status::{TransactionStatusMeta, TransactionTokenBalance};
    use yellowstone_grpc_proto::solana::storage::confirmed_block as proto;

    pub fn transaction(tx: &VersionedTransaction) -> proto::Transaction {
        proto::Transaction {
            signatures: tx.signatures.iter().map(|s| s.as_ref().to_vec()).collect(),
            message: Some(match &tx.message {
                VersionedMessage::Legacy(msg) => proto::Message {
                    header: Some(proto::MessageHeader {
                        num_required_signatures: msg.header.num_required_signatures as u32,
                        num_readonly_signed_accounts: msg.header.num_readonly_signed_accounts
                            as u32,
                        num_readonly_unsigned_accounts: msg.header.num_readonly_unsigned_accounts
                            as u32,
                    }),
                    account_keys: msg
                        .account_keys
                        .iter()
                        .map(|k| k.as_ref().to_vec())
                        .collect(),
                    recent_blockhash: msg.recent_blockhash.as_ref().to_vec(),
                    instructions: msg
                        .instructions
                        .iter()
                        .map(|ix| proto::CompiledInstruction {
                            program_id_index: ix.program_id_index as u32,
                            accounts: ix.accounts.clone(),
                            data: ix.data.clone(),
                        })
                        .collect(),
                    versioned: false,
                    address_table_lookups: vec![],
                },
                VersionedMessage::V0(msg) => proto::Message {
                    header: Some(proto::MessageHeader {
                        num_required_signatures: msg.header.num_required_signatures as u32,
                        num_readonly_signed_accounts: msg.header.num_readonly_signed_accounts
                            as u32,
                        num_readonly_unsigned_accounts: msg.header.num_readonly_unsigned_accounts
                            as u32,
                    }),
                    account_keys: msg
                        .account_keys
                        .iter()
                        .map(|k| k.as_ref().to_vec())
                        .collect(),
                    recent_blockhash: msg.recent_blockhash.as_ref().to_vec(),
                    instructions: msg
                        .instructions
                        .iter()
                        .map(|ix| proto::CompiledInstruction {
                            program_id_index: ix.program_id_index as u32,
                            accounts: ix.accounts.clone(),
                            data: ix.data.clone(),
                        })
                        .collect(),
                    versioned: true,
                    address_table_lookups: msg
                        .address_table_lookups
                        .iter()
                        .map(|l| proto::MessageAddressTableLookup {
                            account_key: l.account_key.as_ref().to_vec(),
                            writable_indexes: l.writable_indexes.clone(),
                            readonly_indexes: l.readonly_indexes.clone(),
                        })
                        .collect(),
                },
            }),
        }
    }

    pub fn transaction_status_meta(meta: &TransactionStatusMeta) -> proto::TransactionStatusMeta {
        proto::TransactionStatusMeta {
            err: meta.status.clone().err().map(|e| proto::TransactionError {
                err: bincode::serialize(&e).unwrap_or_default(),
            }),
            fee: meta.fee,
            pre_balances: meta.pre_balances.clone(),
            post_balances: meta.post_balances.clone(),
            inner_instructions: meta
                .inner_instructions
                .as_ref()
                .map(|ixs| {
                    ixs.iter()
                        .map(|ix| proto::InnerInstructions {
                            index: ix.index as u32,
                            instructions: ix
                                .instructions
                                .iter()
                                .map(|i| proto::InnerInstruction {
                                    program_id_index: i.instruction.program_id_index as u32,
                                    accounts: i.instruction.accounts.clone(),
                                    data: i.instruction.data.clone(),
                                    stack_height: i.stack_height,
                                })
                                .collect(),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            inner_instructions_none: meta.inner_instructions.is_none(),
            log_messages: meta.log_messages.clone().unwrap_or_default(),
            log_messages_none: meta.log_messages.is_none(),
            pre_token_balances: meta
                .pre_token_balances
                .as_ref()
                .map(|bs| bs.iter().map(convert_token_balance).collect())
                .unwrap_or_default(),
            post_token_balances: meta
                .post_token_balances
                .as_ref()
                .map(|bs| bs.iter().map(convert_token_balance).collect())
                .unwrap_or_default(),
            rewards: meta
                .rewards
                .as_ref()
                .map(|rs| {
                    rs.iter()
                        .map(|r| proto::Reward {
                            pubkey: r.pubkey.clone(),
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
                        .collect()
                })
                .unwrap_or_default(),
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
            return_data: meta.return_data.as_ref().map(|r| proto::ReturnData {
                program_id: r.program_id.as_ref().to_vec(),
                data: r.data.clone(),
            }),
            return_data_none: meta.return_data.is_none(),
            compute_units_consumed: meta.compute_units_consumed,
            cost_units: None,
        }
    }

    fn convert_token_balance(tb: &TransactionTokenBalance) -> proto::TokenBalance {
        proto::TokenBalance {
            account_index: tb.account_index as u32,
            mint: tb.mint.clone(),
            ui_token_amount: Some(proto::UiTokenAmount {
                ui_amount: tb.ui_token_amount.ui_amount.unwrap_or_default(),
                decimals: tb.ui_token_amount.decimals as u32,
                amount: tb.ui_token_amount.amount.clone(),
                ui_amount_string: tb.ui_token_amount.ui_amount_string.clone(),
            }),
            owner: tb.owner.clone(),
            program_id: tb.program_id.clone(),
        }
    }
}
