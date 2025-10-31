//! Prometheus metrics for Jetstream source
//!
//! This module provides counters to track blocks and transactions
//! received from the Jetstream service.
//!
//! # Metrics
//!
//! - `jetstream_blocks_received_total`: Counter for blocks received
//! - `jetstream_transactions_received_total`: Counter for transactions received

use std::sync::LazyLock;

use prometheus::{IntCounter, Opts, Registry};

/// Total number of blocks received from Jetstream
pub(crate) static JETSTREAM_BLOCKS_RECEIVED: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "jetstream_blocks_received",
        "Total blocks received from Jetstream",
    ))
    .unwrap()
});

/// Total number of transactions received from Jetstream
pub(crate) static JETSTREAM_TRANSACTIONS_RECEIVED: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "jetstream_transactions_received",
        "Total transactions received from Jetstream",
    ))
    .unwrap()
});

/// Register all Jetstream metrics with the provided Prometheus registry
pub fn register_metrics(registry: &Registry) {
    let _ = registry.register(Box::new(JETSTREAM_BLOCKS_RECEIVED.clone()));
    let _ = registry.register(Box::new(JETSTREAM_TRANSACTIONS_RECEIVED.clone()));
}
