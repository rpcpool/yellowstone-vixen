# Yellowstone Vixen

Yellowstone Vixen allows dApp developers to build program-aware change event streams for Solana. It provides the building blocks, such as a runtime, parser specification, and handler specification, to create custom indexes for specific programs, accounts, and transactions. Vixen enables developers to assemble program parsers to process real-time change events from Dragon's Mouth, converting them into program-aware structures. These structures can then be stored in a database or used in other data pipelines.

## Table of Contents

1. [Objectives](#objectives)
2. [Requirements](#requirements)
3. [Example](#example)
4. [Dragon's Mouth](#dragonsmouth)
5. [Developers](#developers)

## Objectives

1. **Cost Efficiency**: Utilizing Dragon's Mouth, multiple Vixen instances can share a single geyser stream. With various filter options, storage costs focus on what's essential for the dApp.
2. **Operational Simplicity**: Vixen requires minimal configuration and dependency on other systems, making it easy to operate.
3. **Observability**: Operators can monitor the health of their installation, gaining insights into lag, throughput, and error rates.
4. **Composability**: Program parsers are developed as separate modules (cargo crates), enabling programs to include other parsers needed to deserialize cross-program invocations (CPI).

## Requirements

1. **Parser**: A module responsible for transforming raw Solana data into a program-specific format.
2. **Handler**: A module that processes the parsed data, performing tasks such as logging, storing in a database, or triggering other actions.
3. **HandlerManager**: Manages multiple handlers for different types of data (e.g., accounts, transactions).
4. **Configuration**: A TOML file specifying the settings and parameters for Vixen.

## Example

This example demonstrates how a developer can implement a generic parsing pipeline with Vixen. The examples are located in the [`/examples`](/examples) directory.

To run the example, navigate to the desired example directory and execute the following command:

```
cd examples/prometheus
RUST_LOG=info cargo run -- --config "$(pwd)/../../Vixen.toml"
```

You can find an example configuration file at [`Vixen.toml`](/Vixen.toml).

### Explanation

In this example, you need to implement specific components to create a functional parsing pipeline:

- **Parser**: Defines the parsing logic for the specific program. The `prefilter` method sets up filters for the accounts owned by the target program, which are used to build the underlying Dragon's Mouth subscription. The `parse` method contains the logic to transform raw account data into the desired structure.

```rust
pub struct CustomParser;

impl vixen_core::Parser for CustomParser {
    type Input = AccountUpdate;
    type Output = CustomParsedData; // Replace with the actual data type

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([CUSTOM_PROGRAM_ID]) // Replace with the actual program ID
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        // Implement parsing logic here
        // Example: Ok(CustomParsedData::from(acct))
        unimplemented!()
    }
}
```

- **Handler**: Defines how the parsed data should be handled. This could involve logging the data, storing it in a database, or triggering other actions.

```rust
pub struct CustomHandler;

impl<H: std::fmt::Debug + Sync> vixen::Handler<H> for CustomHandler {
    async fn handle(&self, value: &H) -> vixen::HandlerResult<()> {
        // Implement handling logic here
        // Example: tracing::info!(?value);
        unimplemented!()
    }
}
```

- **Main**: Sets up the tracing subscriber, reads the configuration file, and runs the Vixen framework with the specified handlers, managers and metrics.

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
        .build(config)
        .run();
}
```

## Yellowstone Vixen Mock

This crate includes a mock feature designed for testing parsers. It is intended solely for testing purposes. For more details, refer to the [mock](crates/mock/README.md) documentation.

## Metrics Support

### Prometheus

Vixen also supports Prometheus for metrics. To enable Prometheus, set the `prometheus` feature in the `Cargo.toml` file:

```toml
[dependencies]
yellowstone-vixen = { version = "0.0.0", features = ["prometheus"] }
```

- **Prometheus Setup**:

```rust
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

Prometheus metrics are served on the `/metrics` endpoint. To collect metrics, we have setup a prometheus server as a docker container. You can access the metrics at `http://localhost:9091` after running the prometheus server using docker-compose.

### Docker Setup for Metrics

To run prometheus, you need to have docker and docker-compose installed on your machine. To start the services, run the following command:

```bash
sudo docker-compose up
```

## Dragon's Mouth

Dragon's Mouth can be self-hosted as a Geyser plugin or used via a commercial vendor. For more details, refer to the [Yellowstone Dragon's Mouth documentation](https://docs.triton.one/project-yellowstone/dragons-mouth-grpc-subscriptions) and [Yellowstone repository](https://github.com/rpcpool/yellowstone-grpc).

## Developers

This project is developed by [ABK Labs](https://abklabs.com/) and [Triton One](https://triton.one/).
