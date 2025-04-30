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
use yellowstone_vixen_jupiter_swap_parser::{
    accounts_parser::AccountParser as JupiterSwapAccParser,
    instructions_parser::InstructionParser as JupiterSwapIxParser,
    proto_def::DESCRIPTOR_SET as JUPITER_SWAP_DESCRIPTOR_SET,
};
use yellowstone_vixen_meteora_amm_parser::{
    accounts_parser::AccountParser as MeteoraAmmAccParser,
    instructions_parser::InstructionParser as MeteoraAmmIxParser,
    proto_def::DESCRIPTOR_SET as METEORA_AMM_DESCRIPTOR_SET,
};
use yellowstone_vixen_meteora_parser::{
    accounts_parser::AccountParser as MeteoraAccParser,
    instructions_parser::InstructionParser as MeteoraIxParser,
    proto_def::DESCRIPTOR_SET as METEORA_DESCRIPTOR_SET,
};
use yellowstone_vixen_moonshot_parser::{
    accounts_parser::AccountParser as MoonshotAccParser,
    instructions_parser::InstructionParser as MoonshotIxParser,
    proto_def::DESCRIPTOR_SET as MOONSHOT_DESCRIPTOR_SET,
};
use yellowstone_vixen_orca_whirlpool_parser::{
    accounts_parser::AccountParser as OrcaWhirlpoolAccParser,
    instructions_parser::InstructionParser as OrcaWhirlpoolIxParser,
    proto_def::DESCRIPTOR_SET as ORCA_WHIRLPOOL_DESCRIPTOR_SET,
};
use yellowstone_vixen_parser::{
    raydium::{AccountParser as RaydiumAccParser, InstructionParser as RaydiumIxParser},
    token_extension_program::{
        AccountParser as TokenExtensionProgramAccParser,
        InstructionParser as TokenExtensionProgramIxParser,
    },
    token_program::{
        AccountParser as TokenProgramAccParser, InstructionParser as TokenProgramIxParser,
    },
};
use yellowstone_vixen_pump_swaps_parser::{
    accounts_parser::AccountParser as PumpAmmAccParser,
    instructions_parser::InstructionParser as PumpAmmIxParser,
    proto_def::DESCRIPTOR_SET as PUMP_AMM_DESCRIPTOR_SET,
};
use yellowstone_vixen_pumpfun_parser::{
    accounts_parser::AccountParser as PumpfunAccParser,
    instructions_parser::InstructionParser as PumpfunIxParser,
    proto_def::DESCRIPTOR_SET as PUMP_DESCRIPTOR_SET,
};
use yellowstone_vixen_raydium_amm_v4_parser::{
    accounts_parser::AccountParser as RaydiumAmmV4AccParser,
    instructions_parser::InstructionParser as RaydiumAmmV4IxParser,
    proto_def::DESCRIPTOR_SET as RAYDIUM_AMM_V4_DESCRIPTOR_SET,
};
use yellowstone_vixen_raydium_clmm_parser::{
    accounts_parser::AccountParser as RaydiumClmmAccParser,
    instructions_parser::InstructionParser as RaydiumClmmIxParser,
    proto_def::DESCRIPTOR_SET as RAYDIUM_CLMM_DESCRIPTOR_SET,
};
use yellowstone_vixen_raydium_cpmm_parser::{
    accounts_parser::AccountParser as RaydiumCpmmAccParser,
    instructions_parser::InstructionParser as RaydiumCpmmIxParser,
    proto_def::DESCRIPTOR_SET as RAYDIUM_CPMM_DESCRIPTOR_SET,
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
        .descriptor_set(parser::raydium::DESCRIPTOR_SET)
        .descriptor_set(parser::token::DESCRIPTOR_SET)
        .descriptor_set(parser::token_extensions::DESCRIPTOR_SET)
        .descriptor_set(METEORA_DESCRIPTOR_SET)
        .descriptor_set(PUMP_DESCRIPTOR_SET)
        .descriptor_set(JUPITER_SWAP_DESCRIPTOR_SET)
        .descriptor_set(PUMP_AMM_DESCRIPTOR_SET)
        .descriptor_set(RAYDIUM_CPMM_DESCRIPTOR_SET)
        .descriptor_set(ORCA_WHIRLPOOL_DESCRIPTOR_SET)
        .descriptor_set(MOONSHOT_DESCRIPTOR_SET)
        .descriptor_set(METEORA_AMM_DESCRIPTOR_SET)
        .descriptor_set(RAYDIUM_AMM_V4_DESCRIPTOR_SET)
        .descriptor_set(RAYDIUM_CLMM_DESCRIPTOR_SET)
        .account(Proto::new(MeteoraAccParser))
        .account(Proto::new(PumpfunAccParser))
        .account(Proto::new(TokenExtensionProgramAccParser))
        .account(Proto::new(TokenProgramAccParser))
        .account(Proto::new(RaydiumAccParser))
        .account(Proto::new(JupiterSwapAccParser))
        .account(Proto::new(PumpAmmAccParser))
        .account(Proto::new(RaydiumCpmmAccParser))
        .account(Proto::new(OrcaWhirlpoolAccParser))
        .account(Proto::new(MoonshotAccParser))
        .account(Proto::new(MeteoraAmmAccParser))
        .account(Proto::new(RaydiumAmmV4AccParser))
        .account(Proto::new(RaydiumClmmAccParser))
        .instruction(Proto::new(MeteoraIxParser))
        .instruction(Proto::new(PumpfunIxParser))
        .instruction(Proto::new(TokenProgramIxParser))
        .instruction(Proto::new(TokenExtensionProgramIxParser))
        .instruction(Proto::new(RaydiumIxParser))
        .instruction(Proto::new(JupiterSwapIxParser))
        .instruction(Proto::new(PumpAmmIxParser))
        .instruction(Proto::new(RaydiumCpmmIxParser))
        .instruction(Proto::new(OrcaWhirlpoolIxParser))
        .instruction(Proto::new(MoonshotIxParser))
        .instruction(Proto::new(MeteoraAmmIxParser))
        .instruction(Proto::new(RaydiumAmmV4IxParser))
        .instruction(Proto::new(RaydiumClmmIxParser))
        .build(config)
        .run();
}
