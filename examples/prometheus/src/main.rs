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
use solana_accounts_rpc_source::SolanaAccountsRpcSource;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use yellowstone_grpc_source::YellowstoneGrpcSource;
use yellowstone_vixen::{self as vixen, Pipeline};
use yellowstone_vixen_parser::block_meta::BlockMetaParser;
use yellowstone_vixen_raydium_amm_v4_parser::{
    accounts_parser::AccountParser as RaydiumAmmV4AccParser,
    instructions_parser::InstructionParser as RaydiumAmmV4IxParser,
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
        .source(SolanaAccountsRpcSource::new())
        .account(Pipeline::new(RaydiumAmmV4AccParser, [Logger]))
        .instruction(Pipeline::new(RaydiumAmmV4IxParser, [Logger]))
        .block_meta(Pipeline::new(BlockMetaParser, [Logger]))
        .metrics(vixen::metrics::Prometheus)
        .commitment_level(yellowstone_vixen::CommitmentLevel::Confirmed)
        // .from_slot(slot_number)
        .build(config)
        .run();
}
