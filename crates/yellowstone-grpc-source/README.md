# Shipstern Sources

## Overview

Shipstern introduces a flexible `Source` system that allows you to connect to various data sources and stream their updates through a standardized interface. This feature is designed to be extensible, allowing you to create custom sources while maintaining a consistent API.

## Key Features

- 🔌 **Standardized Connection Interface**: Connect to any data source using a unified API
- 📡 **Asynchronous Updates**: Stream data updates through channels
- ⚙️ **Configurable**: Set up filters and source-specific configurations
- 🔄 **Extensible**: Create your own custom sources

## How It Works

The [`Source`](https://github.com/solana-rpc/shipstern/blob/main/crates/runtime/src/sources.rs) trait provides a standardized way to:
- Connect to external data sources
- Stream updates through a channel to the Shipstern runtime for processing
- Configure filters for data processing
- Manage source-specific configuration

## Runtime filter updates

The gRPC source keeps the `SubscribeRequest` sink that `yellowstone-grpc-client`
returns next to the update stream, so a caller can change the subscription
without tearing down the connection and losing messages during the reconnect.

Take the sender off the runtime before running it, because `run` and
`run_async` both consume the runtime:

```rust
let mut runtime = Runtime::<YellowstoneGrpcSource>::builder()
    .instruction(Pipeline::new(TokenProgramIxParser, [Handler]))
    .try_build(config)?;

if let Some(filter_updates) = runtime.filter_updates() {
    tokio::spawn(async move {
        filter_updates.send(new_filters).await.ok();
    });
}

runtime.run_async().await;
```

Each `Filters` sent replaces the whole subscription rather than adding to it,
which is what the server does with a mid-stream request, so send the complete
set every time.

The map keys are parser IDs. A key that matches no registered pipeline still
changes what the server sends, and the runtime then discards all of it at trace
level, so take the keys from the parsers you registered. Delivery is best
effort: a set rejected while the source is between connections is retried once
the stream recovers, but the sender is not told either way.

A set the server refuses, by exceeding its configured filter limits for
example, comes back on the stream with a code the client does not retry, which
ends the run. An update the provider will not accept stops the runtime rather
than leaving the previous subscription in place, so validate against the
provider's limits before sending one.

`Runtime::filter_updates` returns `None` for sources that do not implement
this, which today is every source except gRPC. Whether an update takes effect
also depends on the provider. Both `yellowstone-grpc-geyser` and `richat` apply
mid-stream requests to a live subscription, but a deployment can sit behind
infrastructure that does not forward them.

## Creating a Custom Source

Here's a step-by-step guide to creating your own source:

```rust
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use shipstern::sources::Source;
use shipstern::config::YellowstoneConfig;
use shipstern_core::Filters;

#[derive(Debug)]
struct MyCustomSource {
    filters: Option<Filters>,
    config: Option<YellowstoneConfig>,
}

#[async_trait]
impl Source for MyCustomSource {
    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, Status>>,
    ) -> Result<JoinSet<()>, crate::Error> {
        // Your connection logic here
        todo!()
    }

    fn name(&self) -> String {
        "my-custom-source".to_string()
    }

    // ... other required methods
}
```

## Required Methods

| Method | Description |
|--------|-------------|
| `connect` | Establishes connection to the data source and streams updates |
| `name` | Returns a unique identifier for the source |
| `set_filters_unchecked` | Sets filters for data processing |
| `set_config_unchecked` | Sets source-specific configuration |
| `get_filters` | Retrieves current filters |
| `get_config` | Retrieves current configuration |

## Optional Methods

The trait provides two optional methods with safe default implementations:

- `filters`: Safely sets filters if none are currently set
- `config`: Safely sets configuration if none is currently set

## Best Practices

1. **Naming**: Choose clear, descriptive names for your sources
2. **Error Handling**: Implement proper error handling in your `connect` method
3. **Resource Management**: Ensure proper cleanup of resources when the source is dropped
4. **Configuration**: Use the configuration system to make your source flexible
5. **Filtering**: Implement efficient filtering to reduce unnecessary data transfer

## Example Use Case

Here's a practical example of how to use a source:

```rust
shipstern::Runtime::builder()
    // Add the source to the runtime
    .source(YellowstoneGrpcSource::new())
    // We could call this multiple times to add concurrent Sources
    // .source(SolanaAccountsRpcSource::new())
    .account(Pipeline::new(TokenProgramAccParser, [Handler]))
    .account(Pipeline::new(TokenExtensionProgramAccParser, [Handler]))
    .instruction(Pipeline::new(TokenExtensionProgramIxParser, [Handler]))
    .instruction(Pipeline::new(TokenProgramIxParser, [Handler]))
    .build(config)
    .run();
```

## 🔮 Roadmap

### 📅 Planned Features
| Feature | Priority | Description |
|---------|----------|-------------|
| Source Testing Harness | High | Make it easy to test `Source` implementations and speed up contributions |
| Space for cleanup logic | Medium | Expose a method that can be used for Sources that need to cleanup resources |
| Support additional data sources | Medium | Add support for additional data sources |



## Contributing

We welcome contributions to expand the ecosystem of sources! When creating a new source:

1. Follow the trait implementation guidelines
2. Include comprehensive documentation
3. Consider adding example usage

## Support

If you need help or have questions, please open an issue on [GitHub](https://github.com/solana-rpc/shipstern) or also check other sources implementations in the repository.

---
