#![deny(
    clippy::disallowed_methods,
    clippy::suspicious,
    clippy::style,
    clippy::clone_on_ref_ptr
)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use yellowstone_vixen::{
    self as vixen,
    filter_pipeline::FilterPipeline,
    vixen_core::{Prefilter, Pubkey},
    Pipeline,
};
use yellowstone_vixen_raydium_amm_v4_parser::{
    accounts_parser::AccountParser as RaydiumAmmV4AccParser,
    instructions_parser::InstructionParser as RaydiumAmmV4IxParser,
};
use yellowstone_vixen_yellowstone_grpc_source::YellowstoneGrpcSource;

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

#[derive(Debug)]
pub struct Logger;

impl<V: std::fmt::Debug + Sync, R: Sync> vixen::Handler<V, R> for Logger {
    async fn handle(&self, _value: &V, _raw: &R) -> vixen::HandlerResult<()> {
        Ok(())
    }
}
fn main() {
    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    vixen::Runtime::<YellowstoneGrpcSource>::builder()
        .account(Pipeline::new(RaydiumAmmV4AccParser, [Logger]))
        .instruction(Pipeline::new(
            yellowstone_vixen_meteora_amm_parser::instructions_parser::InstructionParser,
            [Logger],
        ))
        .instruction(FilterPipeline::new(
            RaydiumAmmV4IxParser,
            [Logger],
            Prefilter::builder()
                .transaction_accounts_include([
                    Pubkey::from_str("GH8Ers4yzKR3UKDvgVu8cqJfGzU4cU62mTeg9bcJ7ug6").unwrap(),
                    Pubkey::from_str("4xxM4cdb6MEsCxM52xvYqkNbzvdeWWsPDZrBcTqVGUar").unwrap(),
                ])
                .transaction_accounts([Pubkey::from_str(
                    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                )
                .unwrap()]),
        ))
        .build(config)
        .run();
}
