use buffer::BufferOpts;
use yellowstone::YellowstoneOpts;

mod buffer;
mod parser;
mod parser_manager;
mod yellowstone;

pub use parser::{Parser, BoxedParser, DynParser};
pub use parser_manager::ParserManager;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Yellowstone gRPC error")]
    Yellowstone(#[from] yellowstone::Error),
}

#[derive(Debug, clap::Args, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IndexerOpts {
    #[command(flatten)]
    yellowstone: YellowstoneOpts,

    #[command(flatten)]
    #[serde(default)]
    buffer: BufferOpts,
}

pub fn run<P: Parser + Send + Sync + 'static>(opts: IndexerOpts, manager: ParserManager<P>) {
    match try_run(opts, manager) {
        Ok(()) => (),
        Err(e) => {
            tracing::error!(err = ?anyhow::Error::new(e), "Fatal error encountered");
            std::process::exit(1);
        },
    }
}

pub fn try_run<P: Parser + Send + Sync + 'static>(
    opts: IndexerOpts,
    manager: ParserManager<P>,
) -> Result<(), Error> {
    let IndexerOpts {
        yellowstone,
        buffer,
    } = opts;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let client = yellowstone::connect(yellowstone, &manager).await?;
            let buf = buffer::run_yellowstone(buffer, client, manager);

            Ok(())
        })
}
