# Yellowstone Vixen

Yellowstone Vixen is a framework for building program-aware, real-time Solana data pipelines. It provides the core components—runtime, parser definitions, and handler interfaces—needed to transform raw on-chain events into structured, actionable data.

Vixen consumes Dragon’s Mouth gRPC streams and routes program-specific change events through pluggable parsers, enabling developers to log, store, or stream enriched data for indexing, analytics, and downstream consumption.

## Table of Contents

1. [Problem Solving](#problem-solving)
2. [Features](#features)
3. [Quick Start](#quick-start)
4. [Examples](#examples)
5. [Architecture Diagram](#architecture-diagram)
6. [Dragon’s Mouth Integration](#dragons-mouth)
7. [Developer Resources](#developer-resources)
8. [Maintainers](#maintainers)

## Problem Solving

Yellowstone Vixen solves core challenges for Solana dApp developers:

- **Cost Efficiency**: Share Dragon’s Mouth subscriptions and filter only the data you care about.
- **Operational Simplicity**: Lightweight setup, minimal external dependencies.
- **Observability**: Built-in Prometheus metrics for lag, throughput, and error tracking.
- **Composability**: Independent, reusable parser crates that can deserialize complex cross-program interactions (CPI).

## Features

- **🛠 Parser + Handler Architecture**: Build pipelines that transform raw Solana events into structured models and trigger custom logic.
- **🔥 Dragon’s Mouth Integration**: Subscribe to Solana Geyser streams via gRPC with minimal configuration.
- **📈 Metrics Support**: Prometheus /metrics endpoint available out-of-the-box.
- **🧪 Offline Testing with Fixtures**: Test parsers without connecting to live Solana nodes using devnet fixtures.
- **🔄 gRPC Streaming API**: Serve parsed program events directly to external systems or clients.

## Quick Start

A minimal example using Token Program parsers and a Logger handler:

```rust
use std::path::PathBuf;

use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use yellowstone_vixen::Pipeline;
use yellowstone_vixen_parser::token_program::{AccountParser, InstructionParser};

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Opts {
    #[arg(long, short)]
    config: PathBuf,
}

#[derive(Debug)]
pub struct Logger;

impl<V: std::fmt::Debug + Sync> vixen::Handler<V> for Logger {
    async fn handle(&self, value: &V) -> vixen::HandlerResult<()> {
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

    yellowstone_vixen::Runtime::builder()
        .account(Pipeline::new(AccountParser, [Logger]))
        .account(Pipeline::new(InstructionParser, [Logger]))
        .metrics(yellowstone_vixen::metrics::Prometheus)
        .commitment_level(yellowstone_vixen::CommitmentLevel::Confirmed)
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

## Dragon's Mouth

Dragon's Mouth can be self-hosted as a Geyser plugin or used via a commercial vendor. For more details, refer to the [Yellowstone Dragon's Mouth documentation](https://docs.triton.one/project-yellowstone/dragons-mouth-grpc-subscriptions) and [Yellowstone repository](https://github.com/rpcpool/yellowstone-grpc).

## Developer Resources

- [**Mock Testing for Parsers**](./crates/mock/README.md): Load and replay devnet accounts or transactions offline.
- [**Usage Examples**](./examples/): A variety of example projects that demonstrate how to use the features.
- [**Example Vixen Configuration**](./Vixen.example.toml): Starter TOML file for pipeline configuration.

## Maintainers

This project is developed by [ABK Labs](https://abklabs.com/) and [Triton One](https://triton.one/).
