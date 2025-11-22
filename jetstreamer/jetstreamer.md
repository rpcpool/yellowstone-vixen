# JetStreamer: Historical Solana Data Streaming

JetStreamer is a powerful data source for Yellowstone Vixen that enables streaming and replaying historical Solana blockchain data from CAR (Content Addressable aRchive) files. It provides access to comprehensive historical blockchain data, making it essential for backtesting, data analysis, protocol research, and trading strategy development.

## Overview

JetStreamer integrates with the [jetstreamer-firehose](https://github.com/rpcpool/jetstreamer) library to stream historical Solana data from [Old Faithful](https://old-faithful.net/) archives. It converts raw firehose data into the Yellowstone gRPC protocol format, allowing seamless integration with Yellowstone Vixen's parser and handler ecosystem.

## Key Features

### 🔄 Historical Data Streaming
- **Complete Blockchain Replay**: Stream blocks and transactions from any historical slot range
- **CAR Archive Integration**: Direct access to compressed, indexed Solana data archives
- **Protocol-Compatible**: Converts firehose data to Yellowstone gRPC SubscribeUpdate format

### ⚡ Performance & Scalability
- **Parallel Processing**: Configurable thread count for optimal performance
- **Intelligent Filtering**: Only processes data matching your parser requirements
- **Network Optimization**: Configurable bandwidth limits and connection pooling
- **Async Architecture**: Non-blocking I/O operations for efficient resource utilization

### 🎯 Flexible Data Selection
- **Slot Range Support**: Specify exact slot ranges (start_slot, end_slot)
- **Epoch-Based Selection**: Use epoch numbers for convenient time-based selection
- **Network Support**: Mainnet, testnet, and devnet configurations

### 🔍 Data Filtering & Processing
- **Parser-Driven Filtering**: Automatically filters blocks/transactions based on registered parsers
- **Account Monitoring**: Process transactions containing specific accounts
- **Instruction Extraction**: Extract and parse program instructions from transactions
- **Block Metadata**: Access block headers, rewards, timestamps, and execution data

### 📊 Observability & Monitoring
- **Prometheus Metrics**: Built-in metrics for throughput, latency, and error tracking
- **Structured Logging**: Comprehensive logging with tracing for debugging and monitoring
- **Progress Tracking**: Real-time progress reporting during long-running replays

## Limitations

### Data Quality Issues
- **Early Epoch Corruption**: Epochs 0-100 contain corrupted CAR files with "incomplete frame" errors
- **Archive Completeness**: Some historical slots may have missing or incomplete data
- **Data Consistency**: Historical data quality depends on archive integrity

### API Constraints
- **Limited Account Filtering**: Cannot pre-filter transactions by account keys (processes all matching transactions)
- **Transaction Metadata**: Limited access to full transaction metadata compared to real-time sources
- **No Real-Time Data**: Strictly historical data only (not suitable for live monitoring)

### Performance Considerations
- **Network Bandwidth**: Large historical replays require significant bandwidth
- **Storage Requirements**: CAR files are compressed but still require substantial disk space for large ranges
- **Processing Overhead**: Parallel processing increases memory usage with higher thread counts

### Operational Limitations
- **Archive Availability**: Dependent on Old Faithful archive accessibility and uptime
- **Rate Limiting**: Subject to archive provider rate limits and capacity constraints
- **Version Compatibility**: Tied to specific jetstreamer-firehose library versions

## Installation & Setup

### Dependencies

Add JetStreamer to your `Cargo.toml`:

```toml
[dependencies]
yellowstone-vixen-jetstream-source = { git = "https://github.com/rpcpool/yellowstone-vixen", features = ["prometheus"] }
```

### Basic Configuration

```toml
[source]
# Old Faithful archive URL
archive-url = "https://api.old-faithful.net"

# Data selection - choose one approach:
# Option 1: Epoch-based (recommended for recent data)
epoch = 800

# Option 2: Slot range (for precise control)
# slot-start = 345600000
# slot-end = 346031999

# Performance tuning
threads = 4
network-capacity-mb = 1000

# Network configuration
network = "mainnet"
compact-index-base-url = "https://files.old-faithful.net"
```

## Usage Examples

### Basic Token Program Analysis

```rust
use yellowstone_vixen::{
    Handler, HandlerResult, Pipeline, Runtime,
};
use yellowstone_vixen_jetstream_source::{JetstreamSource, JetstreamSourceConfig, SlotRangeConfig};
use yellowstone_vixen_parser::token_program::InstructionParser;

#[derive(Debug)]
struct TokenLogger;

impl Handler<token_program::TokenProgramIx, InstructionUpdate> for TokenLogger {
    async fn handle(&self, value: &TokenProgramIx, _raw: &InstructionUpdate) -> HandlerResult<()> {
        println!("Token instruction: {:?}", value);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = JetstreamSourceConfig {
        archive_url: "https://api.old-faithful.net".to_string(),
        range: SlotRangeConfig {
            epoch: Some(800), // Recent epoch with good data quality
            ..Default::default()
        },
        threads: 2,
        network: "mainnet".to_string(),
        compact_index_base_url: "https://files.old-faithful.net".to_string(),
        network_capacity_mb: 500,
    };

    let pipeline = Pipeline::new(InstructionParser, [TokenLogger]);
    let runtime = Runtime::<JetstreamSource>::builder()
        .instruction(pipeline)
        .build(config);

    runtime.run().await?;
    Ok(())
}
```

### Advanced Multi-Program Analysis

```rust
use yellowstone_vixen::{
    config::VixenConfig,
    Handler, Pipeline, Runtime,
};

// Configure for multiple DeFi protocols
let config = JetstreamSourceConfig {
    archive_url: "https://api.old-faithful.net".to_string(),
    range: SlotRangeConfig {
        slot_start: Some(200_000_000),
        slot_end: Some(200_100_000),
        epoch: None,
    },
    threads: 8, // Higher parallelism for large ranges
    network_capacity_mb: 2000, // Increased bandwidth
    ..Default::default()
};

let runtime = Runtime::<JetstreamSource>::builder()
    // Jupiter aggregator
    .instruction(Pipeline::new(JupiterSwapParser, [JupiterHandler]))
    // Raydium AMM
    .instruction(Pipeline::new(RaydiumAmmParser, [RaydiumHandler]))
    // Meteora pools
    .instruction(Pipeline::new(MeteoraParser, [MeteoraHandler]))
    .build(config);
```

## Configuration Reference

### JetstreamSourceConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `archive_url` | `String` | - | URL of the Old Faithful archive API |
| `range` | `SlotRangeConfig` | - | Slot range or epoch specification |
| `threads` | `usize` | `4` | Number of parallel processing threads |
| `network` | `String` | `"mainnet"` | Network name (mainnet/testnet/devnet) |
| `compact_index_base_url` | `String` | `"https://files.old-faithful.net"` | Base URL for compact indexes |
| `network_capacity_mb` | `usize` | `1000` | Network bandwidth limit in MB |

### SlotRangeConfig

| Field | Type | Description |
|-------|------|-------------|
| `slot_start` | `Option<u64>` | Starting slot number |
| `slot_end` | `Option<u64>` | Ending slot number (must be > slot_start) |
| `epoch` | `Option<u64>` | Epoch number (432,000 slots per epoch) |

**Note**: Specify either `epoch` OR (`slot_start` + `slot_end`), not both.

## Best Practices

### Data Quality & Reliability

1. **Avoid Early Epochs**: Start with epoch 100+ to avoid corrupted CAR files
2. **Test Small Ranges**: Validate configuration with small slot ranges first
3. **Monitor for Corruption**: Watch for "incomplete frame" errors indicating data corruption
4. **Use Recent Epochs**: Epoch 800+ generally has better data quality

### Performance Optimization

1. **Thread Tuning**: Start with 2-4 threads, increase based on network capacity
2. **Bandwidth Management**: Adjust `network_capacity_mb` based on your connection
3. **Filtering Efficiency**: Use specific parsers to reduce unnecessary processing
4. **Resource Monitoring**: Monitor memory usage with high thread counts

### Error Handling

```rust
// Handle common JetStreamer errors
match runtime.try_run_async().await {
    Ok(_) => println!("Replay completed successfully"),
    Err(e) if e.to_string().contains("incomplete frame") => {
        eprintln!("Data corruption detected - try a different epoch/slot range");
    }
    Err(e) if e.to_string().contains("slot range") => {
        eprintln!("Invalid slot configuration: {}", e);
    }
    Err(e) => eprintln!("Unexpected error: {}", e),
}
```

## Troubleshooting

### Common Issues

**"Incomplete frame" errors**
- **Cause**: Corrupted CAR files in early epochs
- **Solution**: Use epoch 100+ or recent epochs (800+)

**Slow processing**
- **Cause**: Insufficient threads or bandwidth limits
- **Solution**: Increase `threads` and `network_capacity_mb`

**No data processed**
- **Cause**: Filters not matching any data in the selected range
- **Solution**: Verify parser registration and slot range validity

**Connection timeouts**
- **Cause**: Network issues or archive unavailability
- **Solution**: Check archive URL and network connectivity

### Debugging Tips

1. **Enable Debug Logging**: Use `RUST_LOG=debug` for detailed processing information
2. **Monitor Metrics**: Access Prometheus metrics at `/metrics` endpoint
3. **Test with Small Ranges**: Start with 1000-slot ranges for testing
4. **Verify Archive Access**: Test archive URLs independently

## Integration with Yellowstone Vixen

JetStreamer integrates seamlessly with the Yellowstone Vixen framework:

- **Source Interface**: Implements `SourceTrait` for plug-and-play integration
- **Parser Compatibility**: Works with all Vixen parsers (token, DeFi protocols, etc.)
- **Handler Support**: Compatible with all Vixen handler types
- **Metrics Integration**: Automatic Prometheus metrics collection

## Use Cases

### Backtesting Trading Strategies
- Replay historical DEX transactions for strategy validation
- Analyze price movements and liquidity patterns
- Test arbitrage opportunities across protocols

### Protocol Analysis & Research
- Study DeFi protocol behavior over time
- Analyze user adoption and usage patterns
- Research governance and voting mechanisms

### Data Analytics & Reporting
- Generate comprehensive blockchain analytics
- Build historical dashboards and reports
- Perform statistical analysis on blockchain data

### Development & Testing
- Test parsers against real historical data
- Validate smart contract behavior
- Develop and debug blockchain applications

## Contributing

JetStreamer is part of the Yellowstone Vixen ecosystem. Contributions are welcome:

- Report data quality issues or archive corruption
- Suggest performance improvements
- Add support for new network configurations
- Improve error handling and recovery mechanisms

## License

JetStreamer is licensed under the same terms as Yellowstone Vixen. See the main project LICENSE file for details.
