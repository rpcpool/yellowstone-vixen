use std::time::Duration;

use tracing::{error, info};
use yellowstone_vixen::{vixen_core::Parser, Pipeline, Runtime};
use yellowstone_vixen_yellowstone_grpc_source::YellowstoneGrpcSource;

mod common;
use common::{
    create_test_config, run_integration_test_with_event_completion,
    test_handlers::{JupiterTestHandler, OkxTestHandler},
};

// Initialize tracing once for all tests
fn init_tracing() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .try_init()
            .ok(); // Ignore if already initialized
    });
}

// Import parsers
use kryptogo_vixen_okx_dex_parser::instructions_parser::InstructionParser as OkxInstructionParser;
use yellowstone_vixen_jupiter_swap_parser::instructions_parser::InstructionParser as JupiterInstructionParser;

/// Integration test
///
/// Configuration:
/// - Use --config path/to/config.toml to specify configuration file
/// - Falls back to environment variables for backward compatibility:
///   - GRPC_URL: gRPC service address  
///   - GRPC_AUTH_TOKEN: authentication token
///   - GRPC_TIMEOUT: timeout in seconds
#[tokio::test]
#[ignore]
async fn test_jupiter_parser() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();

    info!("Starting Jupiter-only integration test");

    let config = create_test_config().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        e
    })?;
    let (jupiter_handler, shutdown_rx) = JupiterTestHandler::new();
    let jupiter_parser = JupiterInstructionParser;

    info!("Jupiter Parser ID: {}", Parser::id(&jupiter_parser));

    let vixen_runtime = Runtime::<YellowstoneGrpcSource>::builder()
        .instruction(Pipeline::new(jupiter_parser, [jupiter_handler.clone()]))
        .build(config);

    info!("Starting Jupiter parser runtime...");

    let max_duration = Duration::from_secs(30);

    let result = run_integration_test_with_event_completion(
        || async { vixen_runtime.try_run_async().await.map_err(|e| e.into()) },
        shutdown_rx,
        max_duration,
    )
    .await;

    let stats = jupiter_handler.get_stats();
    info!("Jupiter Parser Statistics:");
    info!("  - Swap events: {}", stats.swap_count);
    info!("  - Route events: {}", stats.route_count);
    info!("  - Total volume: {}", stats.total_volume);

    result
}

#[tokio::test]
#[ignore]
async fn test_okx_parser() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();

    info!("Starting OKX-only integration test");

    let config = create_test_config().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        e
    })?;
    let (okx_handler, shutdown_rx) = OkxTestHandler::new();
    let okx_parser = OkxInstructionParser;

    info!("OKX Parser ID: {}", Parser::id(&okx_parser));

    let vixen_runtime = Runtime::<YellowstoneGrpcSource>::builder()
        .instruction(Pipeline::new(okx_parser, [okx_handler.clone()]))
        .build(config);

    info!("Starting OKX parser runtime...");

    let max_duration = Duration::from_secs(30);

    let result = run_integration_test_with_event_completion(
        || async { vixen_runtime.try_run_async().await.map_err(|e| e.into()) },
        shutdown_rx,
        max_duration,
    )
    .await;

    let stats = okx_handler.get_stats();
    info!("OKX Parser Statistics:");
    info!("  - Swap events: {}", stats.swap_count);
    info!("  - Aggregation events: {}", stats.aggregation_count);
    info!("  - Total volume: {}", stats.total_volume);

    result
}
