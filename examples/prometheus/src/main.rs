#![deny(
    clippy::disallowed_methods,
    clippy::suspicious,
    clippy::style,
    clippy::clone_on_ref_ptr
)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::path::PathBuf;

use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use yellowstone_vixen::{
    self as vixen, sources::yellowstone_grpc::YellowstoneGrpcSource, Pipeline,
};
use yellowstone_vixen_parser::{
    block_meta::BlockMetaParser,
    token_extension_program::{
        AccountParser as TokenExtensionProgramAccParser,
        InstructionParser as TokenExtensionProgramIxParser,
    },
    token_program::{
        AccountParser as TokenProgramAccParser, InstructionParser as TokenProgramIxParser,
    },
};

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

#[derive(Debug)]
pub struct Logger;

impl<V: std::fmt::Debug + Sync> vixen::Handler<V> for Logger {
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
        .source(YellowstoneGrpcSource::new())
        .account(Pipeline::new(TokenProgramAccParser, [Logger]))
        .account(Pipeline::new(TokenExtensionProgramAccParser, [Logger]))
        .instruction(Pipeline::new(TokenExtensionProgramIxParser, [Logger]))
        .instruction(Pipeline::new(TokenProgramIxParser, [Logger]))
        .block_meta(Pipeline::new(BlockMetaParser, [Logger]))
        .metrics(vixen::metrics::Prometheus)
        .commitment_level(yellowstone_vixen::CommitmentLevel::Confirmed)
        .build(config)
        .run();
}
