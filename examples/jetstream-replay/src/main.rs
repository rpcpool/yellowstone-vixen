use std::time::Instant;

use anyhow::Result;
use tracing::{error, info};
use tracing_subscriber::fmt;
use warp::Filter;
use yellowstone_vixen::{
    config::{BufferConfig, VixenConfig},
    Handler, HandlerResult, Pipeline, Runtime,
};
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_jetstream_source::{JetstreamSource, JetstreamSourceConfig, SlotRangeConfig};
use yellowstone_vixen_parser::token_program::InstructionParser as TokenProgramIxParser;

// Handler for token program instructions
#[derive(Debug)]
struct TokenInstructionLogger;

impl Handler<yellowstone_vixen_parser::token_program::TokenProgramIx, InstructionUpdate>
    for TokenInstructionLogger
{
    async fn handle(
        &self,
        value: &yellowstone_vixen_parser::token_program::TokenProgramIx,
        _raw: &InstructionUpdate,
    ) -> HandlerResult<()> {
        match value {
            yellowstone_vixen_parser::token_program::TokenProgramIx::Transfer(accounts, data) => {
                info!(
                    instruction = "Transfer",
                    source = %accounts.source,
                    destination = %accounts.destination,
                    amount = data.amount,
                    "Token transfer instruction"
                );
            },
            yellowstone_vixen_parser::token_program::TokenProgramIx::TransferChecked(
                accounts,
                data,
            ) => {
                info!(
                    instruction = "TransferChecked",
                    source = %accounts.source,
                    destination = %accounts.destination,
                    mint = %accounts.mint,
                    amount = data.amount,
                    decimals = data.decimals,
                    "Token transfer checked instruction"
                );
            },
            yellowstone_vixen_parser::token_program::TokenProgramIx::MintTo(accounts, data) => {
                info!(
                    instruction = "MintTo",
                    mint = %accounts.mint,
                    account = %accounts.account,
                    amount = data.amount,
                    "Token mint instruction"
                );
            },
            yellowstone_vixen_parser::token_program::TokenProgramIx::Burn(accounts, data) => {
                info!(
                    instruction = "Burn",
                    account = %accounts.account,
                    mint = %accounts.mint,
                    amount = data.amount,
                    "Token burn instruction"
                );
            },
            yellowstone_vixen_parser::token_program::TokenProgramIx::InitializeMint(
                accounts,
                data,
            ) => {
                info!(
                    instruction = "InitializeMint",
                    mint = %accounts.mint,
                    decimals = data.decimals,
                    "Token mint initialization"
                );
            },
            yellowstone_vixen_parser::token_program::TokenProgramIx::InitializeAccount(
                accounts,
            ) => {
                info!(
                    instruction = "InitializeAccount",
                    account = %accounts.account,
                    mint = %accounts.mint,
                    "Token account initialization"
                );
            },
            yellowstone_vixen_parser::token_program::TokenProgramIx::Approve(accounts, data) => {
                info!(
                    instruction = "Approve",
                    source = %accounts.source,
                    delegate = %accounts.delegate,
                    amount = data.amount,
                    "Token approval instruction"
                );
            },
            yellowstone_vixen_parser::token_program::TokenProgramIx::CloseAccount(accounts) => {
                info!(
                    instruction = "CloseAccount",
                    account = %accounts.account,
                    destination = %accounts.destination,
                    "Token account close instruction"
                );
            },
            other => {
                info!(instruction = ?other, "Other token program instruction");
            },
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    fmt::init();

    // Configuration using epoch 800 (epoch 0 and 1 have corrupted data in old-faithful.net)
    // Epoch 800 corresponds to slots 345,600,000 - 346,031,999 (more recent, likely uncorrupted data)
    let config = JetstreamSourceConfig {
        archive_url: "https://api.old-faithful.net".to_string(),
        range: SlotRangeConfig {
            slot_start: None,
            slot_end: None,
            epoch: Some(800), // Use epoch 800 instead of corrupted early epochs
        },
        threads: 1, // Single thread to reduce chance of hitting corruption
        network: "mainnet".to_string(),
        compact_index_base_url: "https://files.old-faithful.net".to_string(),
        network_capacity_mb: 100000,
        reorder_buffer_size: 1000,
        slot_timeout_secs: 30,
        permissive_transaction_filtering: true,
    };

    //OPTION: if you want to play with different epochs or slot ranges, you can uncomment the following code

    // NOTE: Early epochs (0-~100) have corrupted CAR files in old-faithful.net
    // "Incomplete frame" errors indicate damaged historical data files
    // Current config uses epoch 800 which should have working data

    // Option 1: Try different epochs if 800 doesn't work
    // let config = JetstreamSourceConfig {
    //     archive_url: "https://api.old-faithful.net".to_string(),
    //     range: SlotRangeConfig {
    //         slot_start: None,
    //         slot_end: None,
    //         epoch: Some(500),  // Try different epochs
    //     },
    //     threads: 1,
    //     network: "mainnet".to_string(),
    //     compact_index_base_url: "https://files.old-faithful.net".to_string(),
    //     network_capacity_mb: 100000,
    //     reorder_buffer_size: 1000,
    //     slot_timeout_secs: 30,
    //     permissive_transaction_filtering: true,
    // };

    // Option 2: Use specific slot range instead of epoch
    // let config = JetstreamSourceConfig {
    //     archive_url: "https://api.old-faithful.net".to_string(),
    //     range: SlotRangeConfig {
    //         slot_start: Some(200000000),  // Known working slot range
    //         slot_end: Some(200010000),
    //         epoch: None,
    //     },
    //     threads: 1,
    //     network: "mainnet".to_string(),
    //     compact_index_base_url: "https://files.old-faithful.net".to_string(),
    //     network_capacity_mb: 100000,
    //     reorder_buffer_size: 1000,
    //     slot_timeout_secs: 30,
    //     permissive_transaction_filtering: true,
    // };

    // Option 3: Use Solana RPC API for genesis/recent data instead of CAR files
    // Uncomment and modify the code below to fetch genesis block via RPC:
    /*
    // Add to Cargo.toml dependencies:
    // solana-client = "1.14"
    // solana-sdk = "1.14"

    use solana_client::rpc_client::RpcClient;
    use solana_sdk::commitment_config::CommitmentConfig;

    // In main function, replace jetstream config with:
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    // Fetch genesis block (slot 0)
    match rpc_client.get_confirmed_block_with_encoding(0, solana_client::rpc_request::Encoding::JsonParsed) {
        Ok(block) => {
            info!("Genesis block fetched successfully");
            info!("Blockhash: {}", block.blockhash);
            info!("Transactions: {}", block.transactions.len());
            // Process genesis transactions here
        }
        Err(e) => error!("Failed to fetch genesis block: {}", e),
    }
    */

    info!("Starting Jetstream replay example with Vixen token program parsing");
    info!(archive_url = %config.archive_url, "Configuration");
    info!(
        slot_start = ?config.range.slot_start,
        slot_end = ?config.range.slot_end,
        epoch = ?config.range.epoch,
        threads = config.threads,
        "Source configuration details"
    );

    // Resolve slot range to verify it's valid

    let (start_slot, end_slot) = config
        .range
        .to_slot_range()
        .map_err(|e| anyhow::anyhow!("Failed to resolve slot range: {}", e))?;
    info!(start_slot, end_slot, "Resolved slot range");

    let prometheus_registry = prometheus::Registry::new();
    let prometheus_registry_clone = prometheus_registry.clone();

    let metrics_route = warp::path!("metrics").map(move || {
        use prometheus::Encoder;
        let encoder = prometheus::TextEncoder::new();

        let mut buffer = Vec::new();
        if let Err(e) = encoder.encode(&prometheus_registry_clone.gather(), &mut buffer) {
            error!("Could not encode metrics: {}", e);
            return warp::reply::with_status(
                "".to_string(),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            );
        };

        let response = String::from_utf8(buffer).unwrap_or_default();
        warp::reply::with_status(response, warp::http::StatusCode::OK)
    });

    info!("Starting metrics server on http://localhost:9090/metrics");
    tokio::spawn(async move {
        warp::serve(metrics_route).run(([127, 0, 0, 1], 9090)).await;
    });

    // Create Vixen configuration
    let vixen_config = VixenConfig {
        source: config,
        buffer: BufferConfig::default(),
    };

    info!("Building Vixen runtime with token program instruction parser");
    let runtime = Runtime::<JetstreamSource>::builder()
        .instruction(Pipeline::new(
            TokenProgramIxParser,
            [TokenInstructionLogger],
        ))
        .metrics(prometheus_registry)
        .build(vixen_config);

    let start_time = Instant::now();

    info!("Starting Vixen runtime...");
    runtime.try_run_async().await?;

    let processing_time = start_time.elapsed().as_secs_f64();
    info!(
        processing_time_secs = processing_time,
        "Jetstream replay with Vixen completed"
    );
    //FEATURE: check the metrics at this endpoint -> http://localhost:9090/metrics
    info!("SUCCESS - Vixen integration with token program parsing works!");
    info!("Metrics available at: http://localhost:9090/metrics");

    Ok(())
}
