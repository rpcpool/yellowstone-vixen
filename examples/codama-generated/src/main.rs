#![deny(
    clippy::disallowed_methods,
    clippy::suspicious,
    clippy::style,
    clippy::clone_on_ref_ptr
)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::{io::Write, path::PathBuf};

use clap::Parser as _;
use yellowstone_vixen::{self as vixen, Pipeline};
use yellowstone_vixen_meteora_parser::accounts_parser::{
    AccountParser as LbClmmAccParser, LbClmmProgramState,
};

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

#[derive(Debug)]
pub struct Handler;

impl vixen::Handler<LbClmmProgramState> for Handler {
    async fn handle(&self, value: &LbClmmProgramState) -> vixen::HandlerResult<()> {
        match value {
            LbClmmProgramState::LbPair(lb_pair) => {
                println!(
                    "lb_pair: {:?}, {:?}",
                    lb_pair.activation_point, lb_pair.token_x_mint
                );
            },
            LbClmmProgramState::Oracle(_oracle) => (),
            _ => {
                print!("#");
                std::io::stdout().flush().unwrap();
            },
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    vixen::Runtime::builder()
        .account(Pipeline::new(LbClmmAccParser, [Handler]))
        .build(config)
        .run_async()
        .await;
}
