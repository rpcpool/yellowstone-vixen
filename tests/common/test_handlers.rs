use parking_lot::Mutex;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::info;
use yellowstone_vixen::{Handler, HandlerResult};

/// Statistics for Jupiter swap parser
#[derive(Debug, Default, Clone)]
pub struct JupiterStats {
    pub swap_count: usize,
    pub route_count: usize,
    pub total_volume: u64,
    pub has_events: bool,
}

/// Statistics for OKX DEX parser  
#[derive(Debug, Default, Clone)]
pub struct OkxStats {
    pub swap_count: usize,
    pub aggregation_count: usize,
    pub total_volume: u64,
    pub has_events: bool,
}

/// Combined statistics for both parsers
#[derive(Debug, Clone)]
pub struct DualParserStats {
    pub jupiter_stats: JupiterStats,
    pub okx_stats: OkxStats,
    pub start_time: std::time::Instant,
}

impl DualParserStats {
    pub fn new() -> Self {
        Self {
            jupiter_stats: JupiterStats::default(),
            okx_stats: OkxStats::default(),
            start_time: std::time::Instant::now(),
        }
    }

    pub fn print_summary(&self) {
        let elapsed = self.start_time.elapsed();
        info!("=== Dual Parser Integration Test Summary ===");
        info!("Test duration: {:.2}s", elapsed.as_secs_f64());
        info!("");
        info!("Jupiter Parser:");
        info!("  - Swaps processed: {}", self.jupiter_stats.swap_count);
        info!("  - Routes processed: {}", self.jupiter_stats.route_count);
        info!("  - Total volume: {}", self.jupiter_stats.total_volume);
        info!("");
        info!("OKX DEX Parser:");
        info!("  - Swaps processed: {}", self.okx_stats.swap_count);
        info!(
            "  - Aggregations processed: {}",
            self.okx_stats.aggregation_count
        );
        info!("  - Total volume: {}", self.okx_stats.total_volume);
        info!("");
        info!("Combined totals:");
        info!(
            "  - Total events: {}",
            self.jupiter_stats.swap_count
                + self.jupiter_stats.route_count
                + self.okx_stats.swap_count
                + self.okx_stats.aggregation_count
        );
    }
}

/// Test handler for Jupiter swap events
#[derive(Debug, Clone)]
pub struct JupiterTestHandler {
    stats: Arc<Mutex<JupiterStats>>,
    shutdown_tx: broadcast::Sender<()>,
}

impl JupiterTestHandler {
    pub fn new() -> (Self, broadcast::Receiver<()>) {
        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);
        let handler = Self {
            stats: Arc::new(Mutex::new(JupiterStats::default())),
            shutdown_tx,
        };
        (handler, shutdown_rx)
    }

    pub fn get_stats(&self) -> JupiterStats {
        self.stats.lock().clone()
    }
}

// Generic handler for Jupiter parser output - we'll use Debug trait for logging
impl<T: std::fmt::Debug + Sync> Handler<T> for JupiterTestHandler {
    async fn handle(&self, parsed: &T) -> HandlerResult<()> {
        let mut stats = self.stats.lock();
        stats.route_count += 1;
        stats.has_events = true;

        info!("Jupiter Parser Event: {:?}", parsed);

        let _ = self.shutdown_tx.send(());

        Ok(())
    }
}

/// Test handler for OKX DEX events
#[derive(Debug, Clone)]
pub struct OkxTestHandler {
    stats: Arc<Mutex<OkxStats>>,
    shutdown_tx: broadcast::Sender<()>,
}

impl OkxTestHandler {
    pub fn new() -> (Self, broadcast::Receiver<()>) {
        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);
        let handler = Self {
            stats: Arc::new(Mutex::new(OkxStats::default())),
            shutdown_tx,
        };
        (handler, shutdown_rx)
    }

    pub fn get_stats(&self) -> OkxStats {
        self.stats.lock().clone()
    }
}

// Generic handler for OKX parser output - we'll use Debug trait for logging
impl<T: std::fmt::Debug + Sync> Handler<T> for OkxTestHandler {
    async fn handle(&self, parsed: &T) -> HandlerResult<()> {
        let mut stats = self.stats.lock();
        stats.swap_count += 1;
        stats.has_events = true;

        info!("OKX Parser Event: {:?}", parsed);

        let _ = self.shutdown_tx.send(());

        Ok(())
    }
}
