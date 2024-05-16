use std::path::PathBuf;

use clap::Parser as _;
use spl_token_2022::solana_program::{program_error::ProgramError, program_pack::Pack};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use yellowstone_vixen as vixen;
use yellowstone_vixen::parser::{AccountUpdate, Prefilter, TransactionUpdate};

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

pub struct Parser;

impl vixen::Parser for Parser {
    type Error = ProgramError;

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([spl_token_2022::ID])
            .build()
            .unwrap()
    }

    fn filter_account(&self, acct: &AccountUpdate) -> bool { true }

    fn filter_transaction(&self, txn: &TransactionUpdate) -> bool { true }

    async fn process_account<'a>(&'a self, acct: &'a AccountUpdate) -> Result<(), Self::Error> {
        let inner = acct.account.as_ref().ok_or(ProgramError::InvalidArgument)?;

        let acct = spl_token_2022::state::Account::unpack(
            inner
                .data
                .get(..spl_token_2022::state::Account::LEN)
                .ok_or(ProgramError::InvalidArgument)?,
        )?;
        info!(?acct);
        Ok(())
    }

    async fn process_transaction<'a>(
        &'a self,
        txn: &'a TransactionUpdate,
    ) -> Result<(), Self::Error> {
        info!(?txn);
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
    vixen::run(
        config,
        vixen::ParserManager::<vixen::BoxedParser>::new(
            [("my-parser".into(), Box::new(Parser) as vixen::BoxedParser)]
                .into_iter()
                .collect(),
        ),
    );
}
