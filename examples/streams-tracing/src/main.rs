#![deny(
    clippy::disallowed_methods,
    clippy::suspicious,
    clippy::style,
    clippy::clone_on_ref_ptr
)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::{path::PathBuf, time::Duration};

use clap::Parser as _;
use opentelemetry::trace::TracerProvider;
use tracing_subscriber::layer::SubscriberExt;
use yellowstone_vixen::{self as vixen, vixen_core::proto::Proto, CommitmentLevel};
use yellowstone_vixen_parser::{
    token_extension_program::{
        AccountParser as TokenExtensionProgramAccParser,
        InstructionParser as TokenExtensionProgramIxParser,
    },
    token_program::{
        AccountParser as TokenProgramAccParser, InstructionParser as TokenProgramIxParser,
    },
};
use yellowstone_vixen_yellowstone_grpc_source::YellowstoneGrpcSource;

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

#[tokio::main]
#[rustfmt::skip]
async fn main() {
    let span_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .expect("Error building span exporter!");
    // let span_exporter = opentelemetry_stdout::SpanExporter::default();

    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(span_exporter)
        .build();

    let log_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .build()
        .expect("Error building log exporter!");
    // let log_exporter = opentelemetry_stdout::LogExporter::default();

    let log_provider = opentelemetry_sdk::logs::SdkLoggerProvider::builder()
        .with_batch_exporter(log_exporter)
        .build();

    let tracer = tracer_provider.tracer("vixen_tracer");
    let _ = opentelemetry::global::set_tracer_provider(tracer_provider);

    let traces_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    let log_layer = opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(&log_provider);

    let value_filter = tracing_subscriber::filter::filter_fn(|metadata| {
        let name_matches = matches!(
            metadata.name(),
            "incorrectly_parsed_account"
                | "incorrectly_parsed_instruction"
                | "correctly_parsed_instruction"
                | "correctly_parsed_account"
                | "non_zeroed_end_bytes"
        );

        metadata.is_event() && name_matches
    });

    let subscriber = tracing_subscriber::Registry::default()
        .with(tracing_subscriber::filter::Filtered::new(traces_layer, value_filter.clone()))
        .with(tracing_subscriber::filter::Filtered::new(log_layer, value_filter));
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    let prometheus_registry = prometheus::Registry::new();
    let registry_clone = prometheus_registry.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;

            let metrics = registry_clone.gather();
            let _ = tokio::task::spawn_blocking(move || {
                if let Err(e) = prometheus::push_metrics(
                    "vixen",
                    prometheus::labels! {},
                    "http://localhost:9091",
                    metrics,
                    None,
                ) {
                    tracing::error!("Failed to push Fumarole metrics: {e:?}");
                }
            })
            .await;
        }
    });

    vixen::stream::Server::builder()
        .source(YellowstoneGrpcSource::new())
        .account(Proto::new(yellowstone_vixen_boop_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_jupiter_swap_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_kamino_limit_orders_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_meteora_amm_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_meteora_dbc_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_meteora_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_meteora_pools_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_meteora_vault_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_moonshot_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_orca_whirlpool_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_pump_swaps_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_pumpfun_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_raydium_amm_v4_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_raydium_clmm_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_raydium_cpmm_parser::accounts_parser::AccountParser))
        .account(Proto::new(yellowstone_vixen_raydium_launchpad_parser::accounts_parser::AccountParser))
        .account(Proto::new(TokenExtensionProgramAccParser))
        .account(Proto::new(TokenProgramAccParser))
        .instruction(Proto::new(yellowstone_vixen_boop_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_jupiter_swap_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_kamino_limit_orders_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_meteora_amm_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_meteora_dbc_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_meteora_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_meteora_pools_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_meteora_vault_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_moonshot_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_orca_whirlpool_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_pump_swaps_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_pumpfun_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_raydium_amm_v4_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_raydium_clmm_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_raydium_cpmm_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(yellowstone_vixen_raydium_launchpad_parser::instructions_parser::InstructionParser))
        .instruction(Proto::new(TokenProgramIxParser))
        .instruction(Proto::new(TokenExtensionProgramIxParser))
        .metrics(prometheus_registry)
        .commitment_level(CommitmentLevel::Confirmed)
        .build(config)
        .run_async()
        .await;
}
