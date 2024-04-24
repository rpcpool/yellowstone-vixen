use buffer::BufferOpts;
use yellowstone::YellowstoneOpts;

mod buffer;
mod yellowstone;

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

pub fn run(opts: IndexerOpts) {
    match try_run(opts) {
        Ok(()) => (),
        Err(e) => {
            tracing::error!(err = ?anyhow::Error::new(e), "Fatal error encountered");
            std::process::exit(1);
        },
    }
}

pub fn try_run(opts: IndexerOpts) -> Result<(), Error> {
    let IndexerOpts { yellowstone, buffer } = opts;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?.block_on(async move {
            let client = yellowstone::connect(yellowstone).await?;
            let buf = buffer::run_yellowstone(buffer, client);

            Ok(())
        })
}
