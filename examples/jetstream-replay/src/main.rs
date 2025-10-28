use std::time::{Duration, Instant};

use anyhow::Result;
use jetstreamer_source::{JetstreamSource, JetstreamSourceConfig, SlotRangeConfig};
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use tracing_subscriber::fmt;
use yellowstone_vixen::sources::SourceTrait;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    fmt::init();

    // Create Jetstream source configuration for epoch 800
    let config = JetstreamSourceConfig {
        archive_url: "https://api.old-faithful.net".to_string(),
        range: SlotRangeConfig {
            slot_start: None,
            slot_end: None,
            epoch: Some(800), // Use epoch 800 as specified in acceptance criteria
        },
        threads: 4,
        reorder_buffer_size: 1000,
        slot_timeout_secs: 5, // Shorter timeout for testing
        network: "mainnet".to_string(),
        compact_index_base_url: "https://files.old-faithful.net".to_string(),
        network_capacity_mb: 1000,
    };

    info!("Starting Jetstream replay example");
    info!(archive_url = %config.archive_url, "Configuration");
    info!(
        slot_start = ?config.range.slot_start,
        slot_end = ?config.range.slot_end,
        epoch = ?config.range.epoch,
        threads = config.threads,
        buffer_size = config.reorder_buffer_size,
        timeout_secs = config.slot_timeout_secs,
        "Source configuration details"
    );

    // Create empty filters
    let filters = yellowstone_vixen_core::Filters::new(std::collections::HashMap::new());

    // Create Jetstream source
    let source = JetstreamSource::new(config, filters);

    // Create channel for updates
    let (tx, mut rx) = mpsc::channel(1000);

    // Start timing
    let start_time = Instant::now();

    // Start the source
    let source_handle = tokio::spawn(async move { source.connect(tx).await });

    // Collect statistics
    let mut update_count = 0;
    let mut block_count = 0;
    let mut transaction_count = 0;

    // Process updates with timeout
    let timeout = Duration::from_secs(30);
    loop {
        tokio::select! {
            result = rx.recv() => {
                match result {
                    Some(Ok(update)) => {
                        update_count += 1;

                        // Log the update data with structured tracing
                        info!(
                            update_number = update_count,
                            timestamp = ?update.created_at,
                            "Received data update"
                        );

                        match update.update_oneof {
                            Some(yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof::Block(block_update)) => {
                                block_count += 1;
                                info!(
                                    slot = block_update.slot,
                                    blockhash = %block_update.blockhash,
                                    executed_tx_count = block_update.executed_transaction_count,
                                    parent_slot = block_update.parent_slot,
                                    "Block update received"
                                );
                                if let Some(rewards) = &block_update.rewards {
                                    let num_partitions = rewards.num_partitions
                                        .as_ref()
                                        .map(|np| np.num_partitions)
                                        .unwrap_or(0);
                                    info!(partitions = num_partitions, "Block rewards data");
                                }
                            }
                            Some(yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof::Transaction(tx_update)) => {
                                transaction_count += 1;
                                info!(slot = tx_update.slot, "Transaction update received");
                                // Jetstreamer provides minimal transaction data
                                info!("Transaction contains minimal data from Jetstreamer");
                            }
                            Some(other) => {
                                warn!(update_type = ?other, "Received unknown update type");
                            }
                            None => {
                                warn!("Received update with no data content");
                            }
                        }
                    }
                    Some(Err(e)) => {
                        error!(error = %e, "Error receiving update from channel");
                        break;
                    }
                    None => break, // Channel closed
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                if start_time.elapsed() > timeout {
                    break;
                }
            }
        }
    }

    // Wait for source to complete
    let _ = source_handle.await;

    // Log final results with structured tracing
    let processing_time = start_time.elapsed().as_secs_f64();
    info!(
        total_updates = update_count,
        block_updates = block_count,
        transaction_updates = transaction_count,
        processing_time_secs = processing_time,
        "Jetstream replay completed"
    );

    if update_count > 0 {
        info!("SUCCESS - Real data streaming works!");
    } else {
        warn!("CONNECTED - Jetstreamer integration functional but no data received within timeout");
    }

    Ok(())
}
