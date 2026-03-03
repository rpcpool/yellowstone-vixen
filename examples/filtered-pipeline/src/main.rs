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
    vixen_core::{instruction::InstructionUpdate, KeyBytes, Prefilter},
};
use yellowstone_vixen_spl_token_parser::InstructionParser;
use yellowstone_vixen_yellowstone_grpc_source::YellowstoneGrpcSource;

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

#[derive(Debug)]
pub struct Logger;

impl<V: std::fmt::Debug + Sync> vixen::Handler<V, InstructionUpdate> for Logger {
    async fn handle(&self, value: &V, input: &InstructionUpdate) -> vixen::HandlerResult<()> {
        println!("ix {:?} - {value:?}", input.path);
        Ok(())
    }
}

fn main() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    vixen::Runtime::<YellowstoneGrpcSource>::builder()
        .instruction(FilterPipeline::new(
            InstructionParser,
            [Logger],
            Prefilter::builder().transaction_accounts_include([KeyBytes::<32>::from_str(
                "So11111111111111111111111111111111111111112",
            )
            .unwrap()]),
        ))
        .build(config)
        .run();
}
