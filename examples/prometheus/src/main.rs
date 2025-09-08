#![deny(
    clippy::disallowed_methods,
    clippy::suspicious,
    clippy::style,
    clippy::clone_on_ref_ptr
)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::{path::PathBuf, time::Duration};

use clap::Parser;
use yellowstone_vixen::Pipeline;
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

impl<V: std::fmt::Debug + Sync> yellowstone_vixen::Handler<V> for Logger {
    async fn handle(&self, _value: &V) -> yellowstone_vixen::HandlerResult<()> { Ok(()) }
}

#[tokio::main]
async fn main() {
    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    let prometheus_registry = prometheus::Registry::new();
    let prometheus_registry_pushgateway = prometheus_registry.clone();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));

        loop {
            interval.tick().await;

            let metrics = prometheus_registry_pushgateway.gather();

            let _ = tokio::task::spawn_blocking(move || {
                if let Err(e) = prometheus::push_metrics(
                    "vixen",
                    prometheus::labels! {},
                    "http://localhost:9091",
                    metrics,
                    None,
                ) {
                    tracing::error!("Failed to push metrics: {e:?}");
                }
            })
            .await;
        }
    });

    yellowstone_vixen::Runtime::<YellowstoneGrpcSource>::builder()
        .instruction(Pipeline::new(InstructionParser, [Logger]))
        .account(Pipeline::new(AccountParser, [Logger]))
        .metrics(prometheus_registry)
        .build(config)
        .run_async()
        .await;
}
