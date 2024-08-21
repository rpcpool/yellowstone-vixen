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
use vixen::{handler, HandlerManager, HandlerManagers};
use yellowstone_vixen::{self as vixen, metrics::MetricsFactory};
use yellowstone_vixen_parser::{
    account_parser::{
        token_extensions::TokenExtensionProgramParser, token_program::TokenProgramParser,
    },
    ix_parser::{
        token_extensions::TokenExtensionProgramIxParser, token_program::TokenProgramIxParser,
    },
};
#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

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
        .opts(config)
        .manager(HandlerManagers {
            account: 
            // HandlerManager::new([
            //     handler::boxed(vixen::HandlerPack::new(TokenExtensionProgramParser, [
            //         Handler,
            //     ])),
            //     handler::boxed(vixen::HandlerPack::new(TokenProgramParser, [Handler])),
            // ]),
            HandlerManager::empty(),
            instructions: HandlerManager::new([
                // handler::boxed(vixen::HandlerPack::new(TokenExtensionProgramIxParser, [
                //     Handler,
                // ])),
                handler::boxed(vixen::HandlerPack::new(TokenProgramIxParser, [Handler])),
            ]),
        })
        .metrics(vixen::metrics::prometheus_mod::Prometheus::create().unwrap())
        .build()
        .run();
}
