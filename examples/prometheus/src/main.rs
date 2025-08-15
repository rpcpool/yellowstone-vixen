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
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use yellowstone_vixen::{
    self as vixen,
    filter_pipeline::FilterPipeline,
    vixen_core::{InstructionUpdateOutput, Prefilter, Pubkey},
    Pipeline,
};
use yellowstone_vixen_parser::block_meta::BlockMetaParser;
use yellowstone_vixen_raydium_amm_v4_parser::{
    accounts_parser::AccountParser as RaydiumAmmV4AccParser,
    instructions_parser::{InstructionParser as RaydiumAmmV4IxParser, RaydiumAmmV4ProgramIx},
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

#[derive(Debug)]
pub struct RaydiumAmmV4IxLogger;

impl<V: std::fmt::Debug + Sync> vixen::Handler<V> for Logger {
    async fn handle(&self, _value: &V) -> vixen::HandlerResult<()> { Ok(()) }
}

impl vixen::Handler<InstructionUpdateOutput<RaydiumAmmV4ProgramIx>> for RaydiumAmmV4IxLogger {
    async fn handle(
        &self,
        value: &InstructionUpdateOutput<RaydiumAmmV4ProgramIx>,
    ) -> vixen::HandlerResult<()> {
        match &value.parsed_ix {
            RaydiumAmmV4ProgramIx::SwapBaseIn(accounts, _data) => {
                let accounts_expected = [
                    "GH8Ers4yzKR3UKDvgVu8cqJfGzU4cU62mTeg9bcJ7ug6",
                    "4xxM4cdb6MEsCxM52xvYqkNbzvdeWWsPDZrBcTqVGUar",
                ];
                if !accounts_expected.contains(&accounts.amm.to_string().as_str()) {
                    tracing::info!(
                        "Not expected tx sig: {}",
                        vixen::bs58::encode(&value.shared_data.signature).into_string()
                    );
                }
            },
            RaydiumAmmV4ProgramIx::SwapBaseOut(accounts, _data) => {
                let accounts_expected = [
                    "GH8Ers4yzKR3UKDvgVu8cqJfGzU4cU62mTeg9bcJ7ug6",
                    "4xxM4cdb6MEsCxM52xvYqkNbzvdeWWsPDZrBcTqVGUar",
                ];
                if !accounts_expected.contains(&accounts.amm.to_string().as_str()) {
                    tracing::info!(
                        "Not expected tx sig: {}",
                        vixen::bs58::encode(&value.shared_data.signature).into_string()
                    );
                }
            },
            _ => {},
        }

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
        .source(YellowstoneGrpcSource)
        .account(Pipeline::new(RaydiumAmmV4AccParser, [Logger]))
        .instruction(Pipeline::new(yellowstone_vixen_meteora_amm_parser::instructions_parser::InstructionParser, [Logger]))
        .instruction(FilterPipeline::new(RaydiumAmmV4IxParser, [RaydiumAmmV4IxLogger], Prefilter::builder()
            .transaction_accounts_include([
                Pubkey::from_str("GH8Ers4yzKR3UKDvgVu8cqJfGzU4cU62mTeg9bcJ7ug6").unwrap(),
                Pubkey::from_str("4xxM4cdb6MEsCxM52xvYqkNbzvdeWWsPDZrBcTqVGUar").unwrap()
                ]) // At least one of these accounts must be in the transaction
            .transaction_accounts([Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap()]), // All of these accounts must be in the transaction plus the Parser programId
        ))
        .block_meta(Pipeline::new(BlockMetaParser, [Logger]))
        .metrics(vixen::metrics::Prometheus)
        .commitment_level(yellowstone_vixen::CommitmentLevel::Confirmed)
        // .from_slot(slot_number)
        .build(config)
        .run();
}
