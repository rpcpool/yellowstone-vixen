# Yellowstone Vixen Sources

## Overview

Yellowstone Vixen introduces a flexible `Source` system that allows you to connect to various data sources and stream their updates through a standardized interface. This feature is designed to be extensible, allowing you to create custom sources while maintaining a consistent API.

## Key Features

- 🔌 **Standardized Connection Interface**: Connect to any data source using a unified API
- 📡 **Asynchronous Updates**: Stream data updates through channels
- ⚙️ **Configurable**: Set up filters and source-specific configurations
- 🔄 **Extensible**: Create your own custom sources

## How It Works

The [`Source`](https://github.com/rpcpool/yellowstone-vixen/blob/main/crates/runtime/src/sources.rs) trait provides a standardized way to:
- Connect to external data sources
- Stream updates through a channel to the Vixen runtime for processing
- Configure filters for data processing
- Manage source-specific configuration

## Creating a Custom Source

Here's a step-by-step guide to creating your own source:

```rust
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use yellowstone_vixen::sources::Source;
use yellowstone_vixen::config::YellowstoneConfig;
use vixen_core::Filters;

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
vixen::Runtime::builder()
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

If you need help or have questions, please open an issue on [GitHub](https://github.com/rpcpool/yellowstone-vixen) or also check other sources implementations in the repository.

---
