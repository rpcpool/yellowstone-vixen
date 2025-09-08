#![deny(
    clippy::disallowed_methods,
    clippy::suspicious,
    clippy::style,
    clippy::clone_on_ref_ptr
)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::path::PathBuf;

use clap::Parser as _;
use opentelemetry::trace::TracerProvider;
use tracing_subscriber::layer::SubscriberExt;
use yellowstone_vixen::{Handler, HandlerResult, Pipeline, Runtime};
use yellowstone_vixen_parser::token_program::{AccountParser, InstructionParser};
use yellowstone_vixen_yellowstone_grpc_source::YellowstoneGrpcSource;

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

#[derive(Debug)]
pub struct Logger;

impl<V: std::fmt::Debug + Sync> Handler<V> for Logger {
    async fn handle(&self, _value: &V) -> HandlerResult<()> { Ok(()) }
}

#[rustfmt::skip]
#[allow(clippy::too_many_lines)]
fn main() {
    let span_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .expect("Error building span exporter!");

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

    Runtime::<YellowstoneGrpcSource>::builder()
        .instruction(Pipeline::new(InstructionParser, [Logger]))
        .account(Pipeline::new(AccountParser, [Logger]))
        .build(config)
        .run();
}
