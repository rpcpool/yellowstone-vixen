# Yellowstone Vixen Parser

This crate provides several account parsers, such as Token and TokenExtension. These parsers can be imported from this crate and used within yellowstone-vixen.

## Installation

```bash

cargo add yellowstone-vixen-parser

```

## Example

```rust
use yellowstone_vixen_parser::{
    token_extensions::TokenExtensionProgramParser, token_program::TokenProgramParser,
};
use yellowstone_vixen::{self as vixen, Pipeline};

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    vixen::Runtime::builder()
        .account(Pipeline::new(TokenExtensionProgramParser, [Handler]))
        .account(Pipeline::new(TokenProgramParser, [Handler]))
        .build(config)
        .run();
}
```
