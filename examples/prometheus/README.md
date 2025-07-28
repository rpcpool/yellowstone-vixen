# Example Description

In this example, we use the `yellowstone-vixen-raydium-amm-v4-parser` crate to parse Raydium AMM v4 instructions and accounts. We also test the "shared-data" feature flag of the parser(which exposes info shared by all instructions of a transaction, like the tx signature or slot number), and the `FilterPipeline` which allows to implement transaction accounts gRPC filters through `include_accounts` and `required_accounts`.

We demonstrate the use of the `Prometheus` metrics exporter to export metrics to a Prometheus server.

## Running the example

```bash
cargo run --example prometheus -- --config examples/prometheus/config.toml
```