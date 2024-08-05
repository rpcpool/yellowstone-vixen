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
use spl_token_2022::solana_program::{program_error::ProgramError, program_pack::Pack};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use vixen::vixen_core::{self, ParseResult};
use vixen_core::{AccountUpdate, Prefilter};
use yellowstone_vixen::handler::{HandlerManager, HandlerManagers};
use yellowstone_vixen::metrics::MetricsFactory;
use yellowstone_vixen::{self as vixen, handler};

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

pub struct Parser;

impl vixen_core::Parser for Parser {
    type Input = AccountUpdate;
    type Output = spl_token_2022::state::Account;

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([spl_token_2022::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acct.account.as_ref().ok_or(ProgramError::InvalidArgument)?;

        let acct = spl_token_2022::state::Account::unpack(
            inner
                .data
                .get(..spl_token_2022::state::Account::LEN)
                .ok_or(ProgramError::InvalidArgument)?,
        )?;

        Ok(acct)
    }
}

pub struct Handler;

impl<H: std::fmt::Debug + Sync> vixen::Handler<H> for Handler {
    async fn handle(&self, value: &H) -> vixen::HandlerResult<()> {
        tracing::info!(?value);
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
        .opts(config)
        .manager(HandlerManagers {
            account: HandlerManager::new([handler::boxed(vixen::HandlerPack::new(
                Parser,
                [Handler],
            ))]),
            transaction: HandlerManager::empty(),
        })
        .metrics(vixen::metrics::prometheus_mod::Prometheus::create().unwrap())
        .build()
        .run();
}
