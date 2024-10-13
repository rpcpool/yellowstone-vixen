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
use yellowstone_vixen_parser::{
    token_extension_program::{
        account_parser::TokenExtensionProgramAccParser, ix_parser::TokenExtensionProgramIxParser,
    },
    token_program::{account_parser::TokenProgramAccParser, ix_parser::TokenProgramIxParser},
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
        .descriptor_set(parser::DESCRIPTOR_SET)
        .account(Proto::new(TokenExtensionProgramAccParser))
        .account(Proto::new(TokenProgramAccParser))
        .instruction(Proto::new(TokenProgramIxParser))
        .instruction(Proto::new(TokenExtensionProgramIxParser))
        .build(config)
        .run();
}
