use std::time::Instant;

use anyhow::Result;
use clap::Parser as _;
use tracing::info;
use yellowstone_vixen::{
    config::{BufferConfig, VixenConfig},
    Handler, HandlerResult, Pipeline, Runtime,
};
use yellowstone_vixen_core::{instruction::InstructionUpdate, ParserId};
use yellowstone_vixen_jetstream_source::{JetstreamSource, JetstreamSourceConfig, SlotRangeConfig};
use yellowstone_vixen_spl_token_parser::{InstructionParser, TokenProgramInstruction};

/// Handler for SPL Token program instructions
#[derive(Debug)]
struct TokenInstructionLogger;

impl Handler<TokenProgramInstruction, InstructionUpdate> for TokenInstructionLogger {
    async fn handle(
        &self,
        value: &TokenProgramInstruction,
        _raw: &InstructionUpdate,
    ) -> HandlerResult<()> {
        match value {
            TokenProgramInstruction::Transfer { accounts, args } => {
                info!(
                    instruction = "Transfer",
                    source = %accounts.source,
                    destination = %accounts.destination,
                    amount = args.amount,
                    "Token transfer instruction"
                );
            },
            TokenProgramInstruction::TransferChecked { accounts, args } => {
                info!(
                    instruction = "TransferChecked",
                    source = %accounts.source,
                    destination = %accounts.destination,
                    mint = %accounts.mint,
                    amount = args.amount,
                    decimals = args.decimals,
                    "Token transfer checked instruction"
                );
            },
            TokenProgramInstruction::MintTo { accounts, args } => {
                info!(
                    instruction = "MintTo",
                    mint = %accounts.mint,
                    account = %accounts.account,
                    amount = args.amount,
                    "Token mint instruction"
                );
            },
            TokenProgramInstruction::Burn { accounts, args } => {
                info!(
                    instruction = "Burn",
                    account = %accounts.account,
                    mint = %accounts.mint,
                    amount = args.amount,
                    "Token burn instruction"
                );
            },
            TokenProgramInstruction::InitializeMint { accounts, args } => {
                info!(
                    instruction = "InitializeMint",
                    mint = %accounts.mint,
                    decimals = args.decimals,
                    "Token mint initialization"
                );
            },
            TokenProgramInstruction::InitializeAccount { accounts } => {
                info!(
                    instruction = "InitializeAccount",
                    account = %accounts.account,
                    mint = %accounts.mint,
                    "Token account initialization"
                );
            },
            TokenProgramInstruction::Approve { accounts, args } => {
                info!(
                    instruction = "Approve",
                    source = %accounts.source,
                    delegate = %accounts.delegate,
                    amount = args.amount,
                    "Token approval instruction"
                );
            },
            TokenProgramInstruction::CloseAccount { accounts } => {
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

#[derive(clap::Parser)]
#[command(version, about = "Jetstream historical replay with SPL Token parsing")]
struct Opts {
    /// Epoch to replay (e.g., 885)
    #[arg(long)]
    epoch: Option<u64>,

    /// Start slot (alternative to epoch)
    #[arg(long, conflicts_with = "epoch")]
    slot_start: Option<u64>,

    /// End slot (requires slot_start)
    #[arg(long, requires = "slot_start")]
    slot_end: Option<u64>,

    /// Number of parallel threads
    #[arg(long, default_value = "3")]
    threads: usize,

    /// Archive URL
    #[arg(long, default_value = "https://api.old-faithful.net")]
    archive_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let opts = Opts::parse();

    // Build slot range config from CLI args
    let range = SlotRangeConfig {
        slot_start: opts.slot_start,
        slot_end: opts.slot_end,
        epoch: opts.epoch.or(Some(885)), // Default to epoch 885 if nothing specified
    };

    let config = JetstreamSourceConfig {
        archive_url: opts.archive_url,
        range,
        threads: opts.threads,
        network: "mainnet".to_string(),
        compact_index_base_url: "https://files.old-faithful.net".to_string(),
        network_capacity_mb: 100000,
    };

    info!("Starting Jetstream replay with SPL Token parsing");
    info!(archive_url = %config.archive_url, "Configuration");
    info!(
        slot_start = ?config.range.slot_start,
        slot_end = ?config.range.slot_end,
        epoch = ?config.range.epoch,
        threads = config.threads,
        "Source configuration"
    );

    let (start_slot, end_slot) = config
        .range
        .to_slot_range()
        .map_err(|e| anyhow::anyhow!("Failed to resolve slot range: {}", e))?;
    info!(start_slot, end_slot, "Resolved slot range");

    let vixen_config = VixenConfig {
        source: config,
        buffer: BufferConfig::default(),
    };

    info!("Building Vixen runtime with SPL Token instruction parser");
    let pipeline = Pipeline::new(InstructionParser, [TokenInstructionLogger]);
    info!("Created pipeline with ID: {}", pipeline.id());

    let runtime = Runtime::<JetstreamSource>::builder()
        .instruction(pipeline)
        .build(vixen_config);

    let start_time = Instant::now();

    info!("Starting Vixen runtime...");
    runtime.try_run_async().await?;

    let processing_time = start_time.elapsed().as_secs_f64();
    info!(
        processing_time_secs = processing_time,
        "Jetstream replay completed"
    );

    Ok(())
}
