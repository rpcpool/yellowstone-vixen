use std::path::PathBuf;

use clap::Parser as _;
use yellowstone_vixen::Pipeline;
use yellowstone_vixen_proc_macro::include_vixen_parser;
use yellowstone_vixen_yellowstone_grpc_source::YellowstoneGrpcSource;

include_vixen_parser!("pump_fun.json");

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

#[derive(Debug)]
pub struct Logger;

impl<V: std::fmt::Debug + Sync, R: Sync> yellowstone_vixen::Handler<V, R> for Logger {
    async fn handle(&self, value: &V, _raw: &R) -> yellowstone_vixen::HandlerResult<()> {
        println!("{value:?}");
        Ok(())
    }
}

fn main() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    yellowstone_vixen::Runtime::<YellowstoneGrpcSource>::builder()
        .account(Pipeline::new(pump_fun::AccountParser, [Logger]))
        .instruction(Pipeline::new(pump_fun::InstructionParser, [Logger]))
        .build(config)
        .run();
}
