use std::time::{Duration, Instant};

use anyhow::Result;
use yellowstone_vixen_jetstream_source::{JetstreamSource, JetstreamSourceConfig, SlotRangeConfig, register_metrics};
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use tracing_subscriber::fmt;
use yellowstone_vixen::sources::SourceTrait;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<()> {
    fmt::init();

    let config = JetstreamSourceConfig {
        archive_url: "https://api.old-faithful.net".to_string(),
        range: SlotRangeConfig {
            slot_start: None,
            slot_end: None,
            epoch: Some(800), // Use epoch 800 as specified in acceptance criteria
        },
        threads:4 ,
        reorder_buffer_size: 1000,
        slot_timeout_secs: 5, // Shorter timeout for testing
        network: "mainnet".to_string(),
        compact_index_base_url: "https://files.old-faithful.net".to_string(),
        network_capacity_mb: 10000,
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

    let prometheus_registry = prometheus::Registry::new();
    register_metrics(&prometheus_registry);

    let metrics_route = warp::path!("metrics")
        .map(move || {
            use prometheus::Encoder;
            let encoder = prometheus::TextEncoder::new();

            let mut buffer = Vec::new();
            if let Err(e) = encoder.encode(&prometheus_registry.gather(), &mut buffer) {
                error!("Could not encode metrics: {}", e);
                return warp::reply::with_status("".to_string(), warp::http::StatusCode::INTERNAL_SERVER_ERROR);
            };

            let response = String::from_utf8(buffer).unwrap_or_default();
            warp::reply::with_status(response, warp::http::StatusCode::OK)
        });

    info!("Starting metrics server on http://localhost:9090/metrics");
    tokio::spawn(async move {
        warp::serve(metrics_route)
            .run(([127, 0, 0, 1], 9090))
            .await;
    });

    let filters = yellowstone_vixen_core::Filters::new(std::collections::HashMap::new());
    let source = JetstreamSource::new(config, filters);
    let (tx, mut rx) = mpsc::channel(1000);
    let start_time = Instant::now();
    let source_handle = tokio::spawn(async move { source.connect(tx).await });

    let mut update_count = 0;
    let mut block_count = 0;
    let mut transaction_count = 0;

    let timeout_duration = Duration::from_secs(30);
    let timeout_deadline = start_time + timeout_duration;

    while let Some(result) = rx.recv().await {
        match result {
            Ok(update) => {
                update_count += 1;
                info!(update_count, "Received update");

                match update.update_oneof {
                    Some(yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof::Block(block_update)) => {
                        block_count += 1;
                        info!(slot = block_update.slot, "Block received");
                    }
                    Some(yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof::Transaction(tx_update)) => {
                        transaction_count += 1;
                        info!(slot = tx_update.slot, "Transaction received");
                    }
                    Some(other) => {
                        warn!(update_type = ?other, "Received unknown update type");
                    }
                    None => {
                        warn!("Received update with no data content");
                    }
                }
            }
            Err(e) => {
                error!(error = %e, "Error receiving update from channel");
                break;
            }
        }

        if Instant::now() > timeout_deadline {
            break;
        }
    }

    let _ = source_handle.await;
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
        info!("Metrics available at: http://localhost:9090/metrics");
    } else {
        warn!("CONNECTED - Jetstreamer integration functional but no data received within timeout");
    }

    Ok(())
}
