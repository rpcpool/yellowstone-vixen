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
use opentelemetry_sdk::{
    metrics::{PeriodicReader, SdkMeterProvider},
    trace::TracerProvider,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use yellowstone_vixen::{self as vixen, opentelemetry::trace::TracerProvider as _, Pipeline};
use yellowstone_vixen_parser::{
    token_extension_program::account_parser::TokenExtensionProgramAccParser,
    token_program::account_parser::TokenProgramAccParser,
};

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

#[derive(Debug)]
pub struct Handler;

impl<V: std::fmt::Debug + Sync> vixen::Handler<V> for Handler {
    async fn handle(&self, value: &V) -> vixen::HandlerResult<()> {
        tracing::info!(?value);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let span_exporter = opentelemetry_stdout::SpanExporter::default();
    let tracer_provider = TracerProvider::builder()
        .with_simple_exporter(span_exporter)
        .build();
    let tracer = tracer_provider.tracer("stdout");

    let metrics_exporter = opentelemetry_stdout::MetricsExporter::default();
    let reader = PeriodicReader::builder(metrics_exporter, opentelemetry_sdk::runtime::Tokio)
        .with_interval(Duration::from_secs(30))
        .build();
    let meter_provider = SdkMeterProvider::builder().with_reader(reader).build();

    tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    let _root = tracing::error_span!("service_start").entered();

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    vixen::Runtime::builder()
        .account(Pipeline::new(TokenExtensionProgramAccParser, [Handler]))
        .account(Pipeline::new(TokenProgramAccParser, [Handler]))
        .metrics(vixen::metrics::OpenTelemetry::new(meter_provider))
        .build(config)
        .run_async()
        .await;
}
