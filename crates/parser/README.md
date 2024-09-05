# Yellowstone Vixen Parser

This crate provides several account parsers, such as Token and TokenExtension. These parsers can be imported from this crate and used within yellowstone-vixen.

## Installation

```bash

cargo add yellowstone-vixen-parser

```

## Example

```rust

use std::path::PathBuf;

use clap::Parser as _;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use yellowstone_vixen::{self as vixen, Pipeline};
use yellowstone_vixen_parser::{
    token_extension_program::{
        account_parser::TokenExtensionProgramAccParser, ix_parser::TokenExtensionProgramIxParser,
    },
    token_program::{account_parser::TokenProgramAccParser, ix_parser::TokenProgramIxParser},
};

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    vixen::Runtime::builder()
        .account(Pipeline::new(TokenExtensionProgramAccParser, [Handler]))
        .account(Pipeline::new(TokenProgramAccParser, [Handler]))
        .instruction(Pipeline::new(TokenExtensionProgramIxParser, [Handler]))
        .instruction(Pipeline::new(TokenProgramIxParser, [Handler]))
        .metrics(vixen::metrics::Prometheus)
        .build(config)
        .run();
}
```
