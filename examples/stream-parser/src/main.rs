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
use yellowstone_vixen::{self as vixen, proto::parser, vixen_core::proto::Proto};
use yellowstone_vixen_meteora_parser::{
    accounts_parser::AccountParser as MeteoraAccParser,
    instructions_parser::InstructionParser as MeteoraIxParser,
    proto_def::DESCRIPTOR_SET as METEORA_DESCRIPTOR_SET,
};
use yellowstone_vixen_parser::{
    orca::{AccountParser as OrcaAccParser, InstructionParser as OrcaIxParser},
    raydium::{AccountParser as RaydiumAccParser, InstructionParser as RaydiumIxParser},
    token_extension_program::{
        AccountParser as TokenExtensionProgramAccParser,
        InstructionParser as TokenExtensionProgramIxParser,
    },
    token_program::{
        AccountParser as TokenProgramAccParser, InstructionParser as TokenProgramIxParser,
    },
};
use yellowstone_vixen_pumpfun_parser::{
    accounts_parser::AccountParser as PumpfunAccParser,
    instructions_parser::InstructionParser as PumpfunIxParser,
    proto_def::DESCRIPTOR_SET as PUMP_DESCRIPTOR_SET,
};

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    vixen::stream::Server::builder()
        .descriptor_set(parser::orca::DESCRIPTOR_SET)
        .descriptor_set(parser::raydium::DESCRIPTOR_SET)
        .descriptor_set(parser::token::DESCRIPTOR_SET)
        .descriptor_set(parser::token_extensions::DESCRIPTOR_SET)
        .descriptor_set(METEORA_DESCRIPTOR_SET)
        .descriptor_set(PUMP_DESCRIPTOR_SET)
        .account(Proto::new(MeteoraAccParser))
        .account(Proto::new(PumpfunAccParser))
        .account(Proto::new(TokenExtensionProgramAccParser))
        .account(Proto::new(TokenProgramAccParser))
        .account(Proto::new(OrcaAccParser))
        .account(Proto::new(RaydiumAccParser))
        .instruction(Proto::new(MeteoraIxParser))
        .instruction(Proto::new(PumpfunIxParser))
        .instruction(Proto::new(TokenProgramIxParser))
        .instruction(Proto::new(TokenExtensionProgramIxParser))
        .instruction(Proto::new(OrcaIxParser))
        .instruction(Proto::new(RaydiumIxParser))
        .build(config)
        .run();
}
