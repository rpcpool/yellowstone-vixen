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
use yellowstone_vixen::{self as vixen, vixen_core};

mod account {
    use yellowstone_vixen::proto::tonic;

    tonic::include_proto!("account");
}

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

#[derive(Debug)]
pub struct Parser;

#[derive(Debug, vixen::thiserror::Error)]
#[error("Missing account input data")]
struct MissingData;

fn id() -> vixen_core::Pubkey {
    "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        .parse()
        .unwrap()
}

impl vixen_core::Parser for Parser {
    type Input = vixen_core::AccountUpdate;
    type Output = account::Account;

    fn id(&self) -> std::borrow::Cow<str> { "test_stream::Parser".into() }

    fn prefilter(&self) -> vixen_core::Prefilter {
        vixen_core::Prefilter::builder()
            .account_owners([id()])
            .build()
            .unwrap()
    }

    async fn parse(&self, value: &Self::Input) -> vixen_core::ParseResult<Self::Output> {
        Ok(account::Account {
            data: value.account.as_ref().ok_or(MissingData)?.data.clone(),
        })
    }
}

impl vixen_core::ProgramParser for Parser {
    fn program_id(&self) -> vixen_core::Pubkey { id() }
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
        .account(Parser)
        .build(config)
        .run();
}
