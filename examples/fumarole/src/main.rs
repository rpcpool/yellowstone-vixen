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
use yellowstone_vixen::Pipeline;
use yellowstone_vixen_spl_token_parser::{AccountParser, InstructionParser};
use yellowstone_vixen_yellowstone_fumarole_source::YellowstoneFumaroleSource;

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

#[derive(Debug)]
pub struct Logger;

impl<V: std::fmt::Debug + Sync, R: Sync> yellowstone_vixen::Handler<V, R> for Logger {
    async fn handle(&self, value: &V, _raw: &R) -> yellowstone_vixen::HandlerResult<()> {
        println!("{value:?}");
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Fialed to install rustls crypto provider");

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    yellowstone_vixen::Runtime::<YellowstoneFumaroleSource>::builder()
        .instruction(Pipeline::new(InstructionParser, [Logger]))
        .account(Pipeline::new(AccountParser, [Logger]))
        .build(config)
        .run_async()
        .await;
}
