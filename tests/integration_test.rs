use std::time::Duration;

use tracing::{error, info, warn};
use yellowstone_vixen::{vixen_core::Parser, Pipeline, Runtime};
use yellowstone_vixen_mock::{
    create_mock_transaction_update_with_cache, parse_instructions_from_txn_update,
};
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

/// Helper function to test specific transaction signatures with a given parser
async fn test_specific_signatures<P>(
    parser_name: &str,
    parser: &P,
    signatures: &[&str],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    P: yellowstone_vixen::vixen_core::Parser<
            Input = yellowstone_vixen::vixen_core::instruction::InstructionUpdate,
        > + yellowstone_vixen::vixen_core::ProgramParser
        + Sync,
    P::Output: std::fmt::Debug,
{
    info!("Starting {} specific signatures test", parser_name);
    info!("Testing {} signatures", signatures.len());
    info!(
        "Parser ID: {}",
        yellowstone_vixen::vixen_core::Parser::id(parser)
    );
    info!(
        "Parser Program ID: {}",
        yellowstone_vixen::vixen_core::ProgramParser::program_id(parser)
    );

    let mut success_count = 0;
    let mut filtered_count = 0;
    let mut error_count = 0;
    let mut error_details = Vec::new();

    for (i, signature) in signatures.iter().enumerate() {
        info!("\n=== Testing signature {}/{} ===", i + 1, signatures.len());
        info!("Signature: {}", signature);

        match create_mock_transaction_update_with_cache(signature).await {
            Ok(transaction_update) => {
                match parse_instructions_from_txn_update(&transaction_update) {
                    Ok(instruction_updates) => {
                        info!(
                            "Found {} instructions in transaction",
                            instruction_updates.len()
                        );

                        let mut parsed_any = false;
                        let parser_program_id =
                            yellowstone_vixen::vixen_core::ProgramParser::program_id(parser);

                        // Log all top-level instructions
                        for (ix_idx, ix) in instruction_updates.iter().enumerate() {
                            info!("Instruction {}: program = {}", ix_idx, ix.program);
                        }

                        // Iterate through all instructions including inner instructions
                        // This matches the pattern used in runtime/src/instruction.rs
                        for ix in instruction_updates.iter().flat_map(|i| i.visit_all()) {
                            if ix.program == parser_program_id {
                                let ix_type = if ix.parent_program.is_none() {
                                    "top-level"
                                } else {
                                    "inner"
                                };
                                info!(
                                    "Found {} {} instruction (ix_index: {}, program: {})",
                                    parser_name, ix_type, ix.ix_index, ix.program
                                );

                                match parser.parse(ix).await {
                                    Ok(parsed) => {
                                        info!(
                                            "✓ Successfully parsed {} instruction {}: {:?}",
                                            ix_type, ix.ix_index, parsed
                                        );
                                        success_count += 1;
                                        parsed_any = true;
                                    },
                                    Err(yellowstone_vixen::vixen_core::ParseError::Filtered) => {
                                        // Like runtime, ignore Filtered - this is expected behavior
                                        // (e.g., Pancake swaps called by Jupiter aggregator are filtered to avoid double counting)
                                        info!(
                                            "ℹ {} instruction {} was filtered (expected behavior)",
                                            ix_type, ix.ix_index
                                        );
                                        filtered_count += 1;
                                    },
                                    Err(e) => {
                                        // CPI event logs will produce "Invalid Instruction discriminator" errors
                                        // This is expected behavior as they are not actual instructions
                                        let error_msg = format!("{e:?}");
                                        if error_msg.contains("Invalid Instruction discriminator") {
                                            info!(
                                                "ℹ {} instruction {} is likely a CPI event log \
                                                 (filtered)",
                                                ix_type, ix.ix_index
                                            );
                                            filtered_count += 1;
                                        } else {
                                            error!(
                                                "✗ Failed to parse {} instruction {}: {:?}",
                                                ix_type, ix.ix_index, e
                                            );
                                            error_count += 1;
                                            error_details.push(format!(
                                                "Signature: {}, {} instruction ix_index {}: {:?}",
                                                signature, ix_type, ix.ix_index, e
                                            ));
                                        }
                                    },
                                }
                            }
                        }

                        if !parsed_any {
                            warn!("No {} instructions found in this transaction", parser_name);
                        }
                    },
                    Err(e) => {
                        error!("Failed to parse instructions from transaction: {:?}", e);
                        error_count += 1;
                        error_details.push(format!(
                            "Signature: {signature}: Failed to parse instructions: {e:?}"
                        ));
                    },
                }
            },
            Err(e) => {
                error!("Failed to fetch transaction {}: {:?}", signature, e);
                error_count += 1;
                error_details.push(format!("Signature: {signature}: Failed to fetch: {e:?}"));
            },
        }
    }

    info!("\n=== {} Specific Signatures Test Summary ===", parser_name);
    info!("Total signatures tested: {}", signatures.len());
    info!("Successfully parsed instructions: {}", success_count);
    info!("Filtered instructions: {}", filtered_count);
    info!("Failed to parse instructions: {}", error_count);

    if !error_details.is_empty() {
        error!("\nError Details:");
        for (i, detail) in error_details.iter().enumerate() {
            error!("  {}. {}", i + 1, detail);
        }
    }

    // Assertions to ensure parsing was successful or explicitly filtered
    // Allow the case where all instructions are filtered (e.g., aggregator-invoked swaps)
    assert!(
        success_count > 0 || filtered_count > 0,
        "Expected at least one instruction to be successfully parsed or filtered, but got 0 of \
         each"
    );
    assert_eq!(
        error_count, 0,
        "Expected no parsing errors, but got {error_count} errors"
    );

    if error_count > 0 {
        Err(format!("Failed to parse {error_count} instructions").into())
    } else {
        Ok(())
    }
}

// Import parsers
use kryptogo_vixen_okx_dex_parser::instructions_parser::InstructionParser as OkxInstructionParser;
use yellowstone_vixen_boop_parser::instructions_parser::InstructionParser as BoopInstructionParser;
use yellowstone_vixen_jupiter_swap_parser::instructions_parser::InstructionParser as JupiterInstructionParser;
use yellowstone_vixen_meteora_amm_parser::instructions_parser::InstructionParser as MeteoraAmmInstructionParser;
use yellowstone_vixen_meteora_dbc_parser::instructions_parser::InstructionParser as MeteoraDbcInstructionParser;
use yellowstone_vixen_meteora_parser::instructions_parser::InstructionParser as MeteoraDlmmInstructionParser;
use yellowstone_vixen_meteora_pools_parser::instructions_parser::InstructionParser as MeteoraPoolsInstructionParser;
use yellowstone_vixen_moonshot_parser::instructions_parser::InstructionParser as MoonshotInstructionParser;
use yellowstone_vixen_orca_whirlpool_parser::instructions_parser::InstructionParser as OrcaWhirlpoolInstructionParser;
use yellowstone_vixen_pancake_parser::instructions_parser::InstructionParser as PancakeInstructionParser;
use yellowstone_vixen_pump_swaps_parser::instructions_parser::InstructionParser as PumpSwapsInstructionParser;
use yellowstone_vixen_pumpfun_parser::instructions_parser::InstructionParser as PumpfunInstructionParser;
use yellowstone_vixen_raydium_amm_v4_parser::instructions_parser::InstructionParser as RaydiumAmmV4InstructionParser;
use yellowstone_vixen_raydium_clmm_parser::instructions_parser::InstructionParser as RaydiumClmmInstructionParser;
use yellowstone_vixen_raydium_cpmm_parser::instructions_parser::InstructionParser as RaydiumCpmmInstructionParser;
use yellowstone_vixen_raydium_launchpad_parser::instructions_parser::InstructionParser as RaydiumLaunchpadInstructionParser;

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

#[tokio::test]
async fn test_okx_specific_signatures() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();

    let signatures = &[
        "5gtu6ARVv16QjWi2QR3CZwwPJAEL3TNqvX5HKBz39uB9ky53E2Q7MBjxLqW1a3BFUoXkEEgfmAdsedSeHjpCEau3",
    ];

    let parser = OkxInstructionParser;
    test_specific_signatures("OKX", &parser, signatures).await
}

#[tokio::test]
async fn test_jupiter_specific_signatures() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    init_tracing();

    let signatures = &[
        "vRYNRDqsLW7Kk6GHPzxYytqxHDzDMTGfD2SD3fYsUZgA7o7yhDp97orn9uVoZKjWXYYoNMnGb4jzz2GxZuD2UV1",
    ];

    let parser = JupiterInstructionParser;
    test_specific_signatures("Jupiter", &parser, signatures).await
}

#[tokio::test]
async fn test_boop_specific_signatures() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();

    let signatures = &[
        "53mugqLKTaQquv8nVNqKFFgTyEyf3HfZ468wDtNRmZpZPh8UwvsmLj9uuyu5gQVjB2PFC7EaECGdPNVyv9fW7zcN",
    ];

    let parser = BoopInstructionParser;
    test_specific_signatures("Boop", &parser, signatures).await
}

#[tokio::test]
async fn test_meteora_amm_specific_signatures(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();

    let signatures = &[
        "N2jtQtjjoa6xP7DT2pSEno4umH8GUH8y94raw1qd32hUkaV2rRtDgvD3kMZxYALg63pdELiBhhoYUyZBFT8Tm3k",
    ];

    let parser = MeteoraAmmInstructionParser;
    test_specific_signatures("Meteora AMM", &parser, signatures).await
}

#[tokio::test]
async fn test_meteora_dbc_specific_signatures(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();

    let signatures = &[
        "23BXcBmnM3S3kFxcR49poVKADhGyfdN8GtSGj4tjffFHAYRZHjLpZYWL2McmEMWQT2WcVDwa88CbzvU6N9NjCZCS",
    ];

    let parser = MeteoraDbcInstructionParser;
    test_specific_signatures("Meteora DBC", &parser, signatures).await
}

#[tokio::test]
async fn test_meteora_dlmm_specific_signatures(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();

    let signatures = &[
        "2DfsmTYvMqKwXDBEicEtqLeFfyJ43LLPeVbg8NSjzsQZuhzKzUmZP9XeQLm8C9z8pu3z5paHdJKcnQrw3PA8s4hs",
    ];

    let parser = MeteoraDlmmInstructionParser;
    test_specific_signatures("Meteora DLMM", &parser, signatures).await
}

#[tokio::test]
async fn test_meteora_pools_specific_signatures(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();

    let signatures = &[
        "2mHGPXMzxs6NtaHtbVqku9iKCBy1uAbohMk1yB1it6gku9xXnkQt7TaCh5seb66n7wsADf13MsYYutnYRNrkzbSX",
    ];

    let parser = MeteoraPoolsInstructionParser;
    test_specific_signatures("Meteora Pools", &parser, signatures).await
}

#[tokio::test]
async fn test_moonshot_specific_signatures() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    init_tracing();

    let signatures = &[
        "5UWcde33J3rxFusKri4UCihzq2YatSoYbVjEhm5PRbYxx7VGxh2DPAMixkfnZ5wVyoE4wZNhwMLeJCULkufRd5cn",
    ];

    if signatures.is_empty() {
        warn!("No Moonshot signatures to test");
        return Ok(());
    }

    let parser = MoonshotInstructionParser;
    test_specific_signatures("Moonshot", &parser, signatures).await
}

#[tokio::test]
async fn test_orca_whirlpool_specific_signatures(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();

    let signatures = &[
        "N5qR3DcvdJfwk4kcCCDBMPgJdGmm8mVoXn32QxNrQovaDQCACWaDxJYVBaoUcP7gE342jvJGU2NPcu7mr9qFD9T",
    ];

    let parser = OrcaWhirlpoolInstructionParser;
    test_specific_signatures("Orca Whirlpool", &parser, signatures).await
}

#[tokio::test]
async fn test_pancake_specific_signatures() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    init_tracing();

    let signatures = &[
        "5DJFLgNiZD73caNFPsbJekW9fjsyL3ZnCyZWnEJADVq7VrKak1GMhxpUiUqD2orkoKrhbcmVgsvWXTGeJBL6sTFt",
    ];

    if signatures.is_empty() {
        warn!("No Pancake signatures to test");
        return Ok(());
    }

    let parser = PancakeInstructionParser;
    test_specific_signatures("Pancake", &parser, signatures).await
}

#[tokio::test]
async fn test_pump_swaps_specific_signatures(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();

    let signatures = &[
        "4pCgZ28WbXfpq93VuwbJwjWTvfgesEds9PKXkXmMHUGWmYduKbieMpbckHUwQ8ugVdvrT3CYQDC3n9j4BzPLyJCz",
    ];

    let parser = PumpSwapsInstructionParser;
    test_specific_signatures("Pump Swaps", &parser, signatures).await
}

#[tokio::test]
async fn test_pumpfun_specific_signatures() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    init_tracing();

    let signatures = &[
        "22K6ixTV6Hk9mk9dBqbTcixYw2LXNYEDyiENzLMTs4S8z9i3WRjYLpXDM2mE75nP36moUZ5MeH1ahTvUvYP9L8jH",
    ];

    let parser = PumpfunInstructionParser;
    test_specific_signatures("PumpFun", &parser, signatures).await
}

#[tokio::test]
async fn test_raydium_amm_v4_specific_signatures(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();

    let signatures = &[
        "54MFrVcfzQEnfMCQo2KtRJErGBnr2rgJ7ShAQ8mpr61FdyiQsc8vuxBYqz8xGmM4C23sYcm1Wic3gJTjUf5u9Pkr",
    ];

    let parser = RaydiumAmmV4InstructionParser;
    test_specific_signatures("Raydium AMM V4", &parser, signatures).await
}

#[tokio::test]
async fn test_raydium_clmm_specific_signatures(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();

    let signatures = &[
        "nexzRp8Z5abE2pfaySm7bft7PqnTAQG64Y11gBHvzqdLUYspc84dTtQY9P6BiAMMDNYBTEBLhMDtbHoYYNgUvxS",
    ];

    let parser = RaydiumClmmInstructionParser;
    test_specific_signatures("Raydium CLMM", &parser, signatures).await
}

#[tokio::test]
async fn test_raydium_cpmm_specific_signatures(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();

    let signatures = &[
        "4RoVbE9HB9GSQN1wyBRW7TJCq4ovvWyMfegQAM1Lvd3UgYWGGgJcW3GYruAi7j1poKboPCS2bK71J4iM5EUwxD6R",
    ];

    let parser = RaydiumCpmmInstructionParser;
    test_specific_signatures("Raydium CPMM", &parser, signatures).await
}

#[tokio::test]
async fn test_raydium_launchpad_specific_signatures(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing();

    let signatures = &[
        "5LzhiGZB462sKGLhav13z3KUjjN6EhT9y21F4e4hKpsggFj6bseibkmPP7qXMnac6uDuKZZpdwDcbDqP3YHUXRVp",
    ];

    let parser = RaydiumLaunchpadInstructionParser;
    test_specific_signatures("Raydium Launchpad", &parser, signatures).await
}
