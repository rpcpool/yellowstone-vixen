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
use solana_signature::Signature;
use yellowstone_vixen::{self as vixen, filter_pipeline::FilterPipeline, vixen_core::{instruction::InstructionUpdate, KeyBytes, Prefilter}, HandlerResult};
use yellowstone_vixen::config::VixenConfig;
use yellowstone_vixen::vixen_core::instruction::Path;
use yellowstone_vixen::vixen_core::TransactionUpdate;
use yellowstone_vixen_spl_token_parser::InstructionParser;
use yellowstone_vixen_yellowstone_grpc_source::{YellowstoneGrpcConfig, YellowstoneGrpcSource};

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

#[derive(Debug)]
pub struct Logger;

impl<V: std::fmt::Debug + Sync> vixen::Handler<V, InstructionUpdate> for Logger {

    async fn handle_tx_start(&self, txn: &TransactionUpdate) -> HandlerResult<()> {
        let sig = Signature::try_from(txn.transaction.as_ref().unwrap().signature.as_slice()).unwrap();
        println!("--- starttx {}", sig);
        Ok(())
    }

    async fn handle(&self, value: &V, input: &InstructionUpdate) -> vixen::HandlerResult<()> {
        // TODO
        // println!("ix {:?} - {value:?}", input.path);
        let sig = Signature::try_from(input.shared.signature.as_slice()).unwrap();
        println!("{} > {:?} tx {}", indent(&input.path), input.path, sig);
        Ok(())
    }

    async fn handle_cpi_return(&self, caller_cpi_path: &Path) -> HandlerResult<()> {
        println!("{} <<< {:?}", indent(&caller_cpi_path), caller_cpi_path);
        Ok(())
    }

    async fn handle_tx_end(&self, txn: &TransactionUpdate) -> HandlerResult<()> {
        let sig = Signature::try_from(txn.transaction.as_ref().unwrap().signature.as_slice()).unwrap();
        println!("=== endtx {}", sig);
        println!();
        Ok(())
    }

}

fn indent(cpi_path: &Path) -> String {
    "  ".repeat(cpi_path.len())
}

fn main() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config: VixenConfig<YellowstoneGrpcConfig> = toml::from_str(&config).expect("Error parsing config");
    if config.buffer.jobs != Some(1) {
        println!("This example works best with buffer.jobs to be set to 1 but was {:?}", config.buffer.jobs);
    }


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
