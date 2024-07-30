# Yellowstone Vixen Parser

This crate provides several account parsers, such as Token and TokenExtension. These parsers can be imported from this crate and used within yellowstone-vixen.

## Installation

```bash

cargo add yellowstone-vixen-parser

```

## Example

```rust

use yellowstone_vixen_parser::{TokenProgramParser, TokenExtensionProgramParser};
use yellowstone_vixen as vixen;
use yellowstone_vixen_core::{Handler, HandlerManager, HandlerManagers};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main(){
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");
    vixen::run(
        config,
        HandlerManagers {
            account: HandlerManager::new([handler::boxed(vixen::HandlerPack::new(
                TokenExtensionProgramParser,
                [Handler],
            )),

            handler::boxed(vixen::HandlerPack::new(
                TokenProgramParser,
                [Handler]))
            ]),
            transaction: HandlerManager::empty(),
        },
    );
}

```
