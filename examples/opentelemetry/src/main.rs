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
use yellowstone_vixen::{
    self as vixen,
    opentelemetry::{global, trace::TracerProvider as _},
    Pipeline,
};
use yellowstone_vixen_parser::{
    token_extensions::TokenExtensionProgramParser, token_program::TokenProgramParser,
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
    global::set_tracer_provider(tracer_provider);

    let metrics_exporter = opentelemetry_stdout::MetricsExporter::default();
    let reader = PeriodicReader::builder(metrics_exporter, opentelemetry_sdk::runtime::Tokio)
        .with_interval(Duration::from_secs(30))
        .build();
    let meter_provider = SdkMeterProvider::builder().with_reader(reader).build();
    global::set_meter_provider(meter_provider);

    tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    let _root = tracing::error_span!("service_start").entered();

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    vixen::Runtime::builder()
        .account(Pipeline::new(TokenExtensionProgramParser, [Handler]))
        .account(Pipeline::new(TokenProgramParser, [Handler]))
        .metrics(vixen::metrics::OpenTelemetry)
        .build(config)
        .run_async()
        .await;
}
