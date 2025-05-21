# Yellowstone Vixen

Yellowstone Vixen is a framework for building program-aware, real-time Solana data pipelines. It provides the core components—runtime, parser definitions, and handler interfaces—needed to transform raw on-chain events into structured, actionable data.

Vixen consumes Dragon’s Mouth gRPC streams and routes program-specific change events through pluggable parsers, enabling developers to log, store, or stream enriched data for indexing, analytics, and downstream consumption.

## Table of Contents

- [Yellowstone Vixen](#yellowstone-vixen)
  - [Table of Contents](#table-of-contents)
  - [Problem Solving](#problem-solving)
  - [Features](#features)
  - [Quick Start](#quick-start)
  - [Supported Programs](#supported-programs)
  - [Dragon's Mouth](#dragons-mouth)
  - [Developer Resources](#developer-resources)
  - [Maintainers](#maintainers)

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
        .instruction(Pipeline::new(InstructionParser, [Logger]))
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

## Supported Programs

there are **16** 3rd-party programs:

| Address                                        | Public Name                        | Parser                                                                                                                                   |
| ---------------------------------------------- | ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4`  | **Boop.fun**                       | [yellowstone-vixen-boop-parser](https://github.com/rpcpool/yellowstone-vixen/tree/main/crates/boop-parser)                               |
| `JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4`  | **Jupiter Aggregator v6**          | [yellowstone-vixen-jupiter-swap-parser](https://github.com/rpcpool/yellowstone-vixen/tree/main/crates/jupiter-swap-parser)               |
| `LiMoM9rMhrdYrfzUCxQppvxCSG1FcrUK9G8uLq4A1GF`  | **Kamino Limit Order**             | [yellowstone-vixen-kamino-limit-orders-parser](https://github.com/rpcpool/yellowstone-vixen/blob/main/crates/kamino-limit-orders-parser) |
| `cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG`  | **Meteora DAMM v2**                | [yellowstone-vixen-meteora-amm-parser](https://github.com/rpcpool/yellowstone-vixen/tree/main/crates/meteora-amm-parser)                 |
| `dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN`  | **Meteora Dynamic Bonding Curve**  | [yellowstone-vixen-meteora-dbc-parser](https://github.com/rpcpool/yellowstone-vixen/blob/main/crates/meteora-dbc-parser)                 |
| `LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo`  | **Meteora DLMM**                   | [yellowstone-vixen-meteora-parser](https://github.com/rpcpool/yellowstone-vixen/blob/main/crates/meteora-parser)                         |
| `Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB` | **Meteora Pools**                  | [yellowstone-vixen-meteora-pools-parser](https://github.com/rpcpool/yellowstone-vixen/blob/main/crates/meteora-pools-parser)             |
| `MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG`  | **Moonshot**                       | [yellowstone-vixen-moonshot-parser](https://github.com/rpcpool/yellowstone-vixen/blob/main/crates/moonshot-parser)                       |
| `whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc`  | **Whirlpools**                     | [yellowstone-vixen-orca-whirlpool-parser](https://github.com/rpcpool/yellowstone-vixen/blob/main/crates/orca-whirlpool-parser)           |
| `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA`  | **Pump.fun AMM**                   | [yellowstone-vixen-pump-swaps-parser](https://github.com/rpcpool/yellowstone-vixen/blob/main/crates/pump-swaps-parser)                   |
| `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P`  | **Pump.fun**                       | [yellowstone-vixen-pumpfun-parser](https://github.com/rpcpool/yellowstone-vixen/blob/main/crates/pumpfun-parser)                         |
| `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8` | **Raydium Liquidity Pool V4**      | [yellowstone-vixen-raydium-amm-v4-parser](https://github.com/rpcpool/yellowstone-vixen/blob/main/crates/raydium-amm-v4-parser)           |
| `CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK` | **Raydium Concentrated Liquidity** | [yellowstone-vixen-raydium-clmm-parser](https://github.com/rpcpool/yellowstone-vixen/blob/main/crates/raydium-clmm-parser)               |
| `CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C` | **Raydium CPMM**                   | [yellowstone-vixen-raydium-cpmm-parser](https://github.com/rpcpool/yellowstone-vixen/blob/main/crates/raydium-cpmm-parser)               |
| `LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj`  | **Raydium Launchpad**              | [yellowstone-vixen-raydium-launchpad-parser](https://github.com/rpcpool/yellowstone-vixen/blob/main/crates/raydium-launchpad-parser)     |
| `5U3EU2ubXtK84QcRjWVmYt9RaDyA8gKxdUrPFXmZyaki` | **Virtuals**                       | [yellowstone-vixen-virtuals-parser](https://github.com/rpcpool/yellowstone-vixen/blob/main/crates/virtuals-parser)                       |

## Dragon's Mouth

Dragon's Mouth can be self-hosted as a Geyser plugin or used via a commercial vendor. For more details, refer to the [Yellowstone Dragon's Mouth documentation](https://docs.triton.one/project-yellowstone/dragons-mouth-grpc-subscriptions) and [Yellowstone repository](https://github.com/rpcpool/yellowstone-grpc).

## Developer Resources

- [**Mock Testing for Parsers**](./crates/mock/README.md): Load and replay devnet accounts or transactions offline.
- [**Usage Examples**](./examples/): A variety of example projects that demonstrate how to use the features.
- [**Example Vixen Configuration**](./Vixen.example.toml): Starter TOML file for pipeline configuration.

## Maintainers

This project is developed by [ABK Labs](https://abklabs.com/) and [Triton One](https://triton.one/).
