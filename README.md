# Yellowstone Vixen

Yellowstone Vixen is a comprehensive framework for building program-aware, real-time Solana data pipelines. It provides the core components—runtime, parser definitions, and handler interfaces—needed to transform raw on-chain events into structured, actionable data. Vixen supports the registration of custom data sources, allowing developers to integrate various data streams seamlessly.

Solana change events, following the Yellowstone gRPC specification, are received from the source and routed through pluggable parsers. This enables developers to log, store, or stream enriched data for indexing, analytics, and downstream consumption.

## Table of Contents

- [Yellowstone Vixen](#yellowstone-vixen)
  - [Table of Contents](#table-of-contents)
  - [Problem Solving](#problem-solving)
  - [Features](#features)
  - [Quick Start](#quick-start)
  - [Parsers](#parsers)
    - [Built-in](#built-in)
    - [Codegen Macro](#codegen-macro)
  - [Official Sources](#official-sources)
  - [Running Tests](#running-tests)
  - [Developer Resources](#developer-resources)
  - [Maintainers](#maintainers)

## Problem Solving

Yellowstone Vixen solves core challenges for Solana dApp developers:

- **Cost Efficiency**: Share Dragon's Mouth subscriptions and filter only the data you care about.
- **Operational Simplicity**: Lightweight setup, minimal external dependencies.
- **Observability**: Built-in Prometheus metrics for lag, throughput, and error tracking.
- **Composability**: Independent, reusable parser crates that can deserialize complex cross-program interactions (CPI).

## Features

- **🛠 Parser + Handler Architecture**: Build pipelines that transform raw Solana events into structured models and trigger custom logic.
- **🔥 Flexible Source Integration**: Register custom data sources or use existing ones like Dragon's Mouth for Solana Geyser streams.
- **✨ IDL-Based Code Generation**: Generate type-safe parsers directly from your Solana program's IDL using a procedural macro, eliminating manual deserialization and improving maintainability.
- **📈 Metrics Support**: Register your own Prometheus registry for unified metrics reporting.
- **🧪 Offline Testing with Fixtures**: Test parsers without connecting to live Solana nodes using devnet fixtures.
- **🔄 gRPC Streaming API**: Serve parsed program events directly to external systems or clients.

## Quick Start

A minimal example using Token Program parsers and a Logger handler:

```rust
use std::path::PathBuf;

use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use yellowstone_vixen::Pipeline;
use yellowstone_vixen_spl_token_parser::{AccountParser, InstructionParser};
use yellowstone_vixen_yellowstone_grpc_source::YellowstoneGrpcSource;

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

#[derive(Debug)]
pub struct Logger;

impl<V: std::fmt::Debug + Sync, R: Sync> vixen::Handler<V, R> for Logger {
    async fn handle(&self, value: &V, _raw: &R) -> vixen::HandlerResult<()> {
        tracing::info!(?value);
        Ok(())
    }
}

fn main() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Fialed to install rustls crypto provider");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    yellowstone_vixen::Runtime<YellowstoneGrpcSource>::builder()
        .account(Pipeline::new(AccountParser, [Logger]))
        .instruction(Pipeline::new(InstructionParser, [Logger]))
        .build(config)
        .run();
}
```

```shell
RUST_LOG=info cargo run -- --config "./Vixen.toml"
```

Prometheus metrics are served on the `/metrics` endpoint. To collect metrics, we have setup a prometheus server as a docker container. You can access the metrics at `http://localhost:9090` after running the prometheus server using docker-compose.

To run prometheus, you need to have docker and docker-compose installed on your machine. To start the services, run the following command:

```bash
sudo docker-compose up
```

## Parsers

### Built-in

| Address                                       | Public Name          | Parser                                                                                                                                      |
| --------------------------------------------- | -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` | **Token Program**    | [yellowstone-vixen-spl-token-parser](https://github.com/rpcpool/yellowstone-vixen/tree/main/crates/spl-token-parser)                        |
| `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` | **Token Extensions** | [yellowstone-vixen-spl-token-extensions-parser](https://github.com/rpcpool/yellowstone-vixen/tree/main/crates/spl-token-extensions-program) |

### Codegen Macro

The `yellowstone-vixen-proc-macro` crate provides the `include_vixen_parser!` procedural macro, which generates a Vixen parser from a Codama JSON IDL file.

To use it, add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
borsh = "^1.0.0"
yellowstone-vixen-parser = { version = "0.6.0" }
yellowstone-vixen-proc-macro = { version = "0.6.0" }
```

Then, import and invoke the macro in your code. Specify the path to your Codama JSON IDL file relative to your crate root:

```rust
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("path/to/idl.json");
```

The generated account and instruction parsers will be available under a module named after the program.

### Handling Discriminator Collisions

Some programs define multiple instruction variants that share the same discriminator. When the variants have **different account counts**, the generated `InstructionParser` disambiguates automatically — it tries the variant with the most accounts first and falls back to smaller ones:

```rust
// Just works — no custom resolver needed.
let parser = my_program::InstructionParser;
```

When two or more variants share both the same discriminator **and** the same account count, the default parser cannot disambiguate and returns an error at runtime. In that case, implement `InstructionResolver` and use `CustomInstructionParser`:

```rust
use yellowstone_vixen_core::ParseError;

#[derive(Debug, Copy, Clone)]
struct MyResolver;

impl my_program::InstructionResolver for MyResolver {
    fn resolve(
        &self,
        accounts: &[yellowstone_vixen_core::KeyBytes<32>],
        data: &[u8],
    ) -> Result<my_program::Instructions, ParseError> {
        // Custom disambiguation logic for the ambiguous discriminator.
        if data.first() == Some(&0x09) {
            // ... inspect data or accounts to decide which variant ...
        }

        // Delegate everything else to the default resolver.
        my_program::resolve_instruction_default(accounts, data)
    }
}

let parser = my_program::CustomInstructionParser(MyResolver);
```

## Official Sources

Yellowstone Vixen supports several official data sources for ingesting Solana account and transaction data. Each source is provided as a Rust crate and can be configured in your Vixen pipeline. Below is a summary of the available sources:

| Source Crate                                                          | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| --------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`yellowstone-grpc-source`](./crates/yellowstone-grpc-source)         | **Dragon's Mouth (gRPC Source)**: Scalable and reliable streaming of account and transaction data from Solana nodes via the Dragon's Mouth Geyser plugin. Can be self-hosted or accessed via commercial vendors. See the [Dragon's Mouth documentation](https://docs.triton.one/project-yellowstone/dragons-mouth-grpc-subscriptions) and [Yellowstone repository](https://github.com/rpcpool/yellowstone-grpc) for more details.                                                                              |
| [`yellowstone-fumarole-source`](./crates/yellowstone-fumarole-source) | **Fumarole Reliable Streams**: Scalable, reliable, and persistent streaming of Solana accounts and transactions. Fumarole merges data from multiple nodes for high availability, supports consumer groups for horizontal scalability, and allows clients to resume streams after interruptions. Currently in limited beta—contact Triton support for access. [Learn more](https://blog.triton.one/introducing-yellowstone-fumarole) or see the [GitHub repo](https://github.com/rpcpool/yellowstone-fumarole). |
| [`solana-rpc-source`](./crates/solana-rpc-source)                     | **Solana RPC Source**: Pulls account data directly from a Solana node's JSON-RPC API using `getProgramAccounts`.                                                                                                                                                                                                                                                                                                                                                                                               |
| [`solana-snapshot-source`](./crates/solana-snapshot-source)           | **Solana Snapshot Source**: Loads and processes Solana ledger snapshots for offline or historical analysis.                                                                                                                                                                                                                                                                                                                                                                                                    |

Refer to the crate documentation for setup instructions and configuration options.

## Running Tests

Parser tests use the `RPC_ENDPOINT` environment variable to fetch fixture data from a Solana RPC node. If not set, it defaults to `https://api.devnet.solana.com`.

**Inline (one-off):**

```bash
RPC_ENDPOINT=https://my-rpc.example.com cargo test
```

**Persistent per-project via `.cargo/config.toml`:**

Create `.cargo/config.toml` in the repo root:

```toml
[env]
RPC_ENDPOINT = "https://my-rpc.example.com"
```

This applies to every `cargo` invocation inside this workspace. The file is gitignored so each developer can use their own RPC provider.

## Developer Resources

- [**Mock Testing for Parsers**](./crates/mock/README.md): Load and replay devnet accounts or transactions offline.
- [**Usage Examples**](./examples/): A variety of example projects that demonstrate how to use the features.
- [**Example Vixen Configuration**](./Vixen.example.toml): Starter TOML file for pipeline configuration.
- [**Generate Parsers from IDL**](./docs/codama-parser-generation.md): Use Codama to automatically generate Vixen parsers from Anchor or custom IDL files.

## Maintainers

This project is developed by [ABK Labs](https://abklabs.com/) and [Triton One](https://triton.one/).
