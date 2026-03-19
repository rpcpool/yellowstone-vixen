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
use yellowstone_vixen::{
    self as vixen,
    config::VixenConfig,
    filter_pipeline::FilterPipeline,
    handler::LifecycleEvent,
    vixen_core::{
        instruction::{InstructionShared, InstructionUpdate, Path},
        KeyBytes, Prefilter, TransactionUpdate,
    },
    HandlerResult,
};
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
    async fn handle(&self, _value: &V, input: &InstructionUpdate) -> vixen::HandlerResult<()> {
        let sig = Signature::try_from(input.shared.signature.as_slice()).unwrap();
        println!("{} > {:?} tx {}", indent(&input.path), input.path, sig);
        Ok(())
    }

    async fn handle_lifecycle(
        &self,
        txn: &TransactionUpdate,
        _instruction_shared: &InstructionShared,
        event: &LifecycleEvent<'_>,
    ) -> HandlerResult<()> {
        match event {
            LifecycleEvent::TxStart => {
                let sig =
                    Signature::try_from(txn.transaction.as_ref().unwrap().signature.as_slice())
                        .unwrap();
                println!("--- starttx {sig}");
            },
            LifecycleEvent::TxEnd => {
                let sig =
                    Signature::try_from(txn.transaction.as_ref().unwrap().signature.as_slice())
                        .unwrap();
                println!("=== endtx {sig}");
                println!();
            },
            LifecycleEvent::CpiEnter {
                caller_cpi_path: caller,
            } => {
                println!("{} >>> {:?} ENTER", indent(caller), caller);
            },
            LifecycleEvent::CpiReturn {
                caller_cpi_path: caller,
            } => {
                println!("{} <<< {:?} RETURN", indent(caller), caller);
            },
        }
        Ok(())
    }
}

fn indent(cpi_path: &Path) -> String { "  ".repeat(cpi_path.len()) }

fn main() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config: VixenConfig<YellowstoneGrpcConfig> =
        toml::from_str(&config).expect("Error parsing config");
    if config.buffer.jobs != Some(1) {
        println!(
            "This example works best with buffer.jobs to be set to 1 but was {:?}",
            config.buffer.jobs
        );
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
