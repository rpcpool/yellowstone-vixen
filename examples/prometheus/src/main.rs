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
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use yellowstone_vixen::{self as vixen, Pipeline};
use yellowstone_vixen_parser::{
    token_extension_program::{
        account_parser::TokenExtensionProgramAccParser, ix_parser::TokenExtensionProgramIxParser,
    },
    token_program::{account_parser::TokenProgramAccParser, ix_parser::TokenProgramIxParser},
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

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    vixen::Runtime::builder()
        .account(Pipeline::new(TokenExtensionProgramAccParser, [Handler]))
        .account(Pipeline::new(TokenProgramAccParser, [Handler]))
        .instruction(Pipeline::new(TokenExtensionProgramIxParser, [Handler]))
        .instruction(Pipeline::new(TokenProgramIxParser, [Handler]))
        .metrics(vixen::metrics::Prometheus)
        .build(config)
        .run();
}
