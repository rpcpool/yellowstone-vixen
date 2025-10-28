# Jetstream Replay Example

This example demonstrates how to use the Jetstream source to replay historical Solana data from the Old Faithful archive.

## Overview

The Jetstream source enables streaming historical Solana ledger data at high throughput (>2.7M TPS) from the Old Faithful archive. This example shows how to:

- Configure the Jetstream source for epoch-based replay
- Set up parsers and handlers for blocks and transactions
- Run a historical data replay pipeline

## Features

- ✅ Historical data replay from Old Faithful archive
- ✅ Support for epoch-based or slot-range queries
- ✅ Multi-threaded streaming with out-of-order handling
- ✅ Block and transaction parsing and handling
- ✅ Configurable threading and buffering

## Usage

### Running the Example

```bash
# From the repository root
cargo run --example jetstream-replay
```

### Configuration

The example uses the following configuration (from `config.toml`):

```toml
[source]
archive_url = "https://api.old-faithful.net"
epoch = 800
threads = 4
reorder_buffer_size = 1000
slot_timeout_secs = 30
```

**Configuration Options:**

- `archive_url`: Old Faithful archive endpoint
- `epoch`: Which epoch to replay (mutually exclusive with `slot_start`/`slot_end`)
- `threads`: Number of parallel streaming threads
- `reorder_buffer_size`: Maximum slots to buffer for reordering
- `slot_timeout_secs`: How long to wait for missing slots before skipping

### Alternative: Slot Range Replay

To replay a specific slot range instead of an entire epoch:

```toml
[source]
archive_url = "https://api.old-faithful.net"
slot_start = 250000000
slot_end = 250100000
threads = 8
reorder_buffer_size = 2000
slot_timeout_secs = 60
```

## Output

The example will output information about each block and transaction as they are processed:

```
Block #1: Slot 345600000, Hash: abc123..., Transactions: 1500, Time: Some(1635000000)
Transaction #1: Slot 345600000, Signature: sig123..., Vote: false
Transaction #2: Slot 345600000, Signature: sig456..., Vote: true
...
```

## Performance Tuning

### Thread Count
- **2-4 threads**: Good for smaller ranges or testing
- **4-8 threads**: Recommended for most use cases
- **8+ threads**: For very large ranges, but may increase out-of-order delivery

### Buffer Size
- **500-1000**: Small ranges (<1M slots)
- **1000-2000**: Medium ranges (1M-10M slots)
- **2000+**: Large ranges (>10M slots)

### Timeout
- **30 seconds**: Default, balances responsiveness and missing slot tolerance
- **60+ seconds**: For slower networks or when missing slots are expected

## Limitations

1. **No account updates**: Jetstreamer provides blocks/transactions/rewards only
2. **Network dependent**: Requires connection to Old Faithful archive
3. **Historical only**: Not suitable for real-time streaming
4. **Memory usage**: Large reorder buffers consume RAM

## Troubleshooting

### High memory usage
- Reduce `reorder_buffer_size`
- Reduce `threads`
- Process smaller ranges

### Missing slots in output
- Increase `slot_timeout_secs`
- Check Old Faithful archive coverage
- Verify network connectivity

### Slow performance
- Increase `threads` for more parallelism
- Check network bandwidth
- Optimize downstream handlers

### Connection errors
- Verify `archive_url` is correct and accessible
- Check firewall settings
- Try different Old Faithful endpoint

## API Documentation

For more details on the Jetstream source API:

- [Jetstreamer Crate](https://docs.rs/jetstreamer)
- [Yellowstone Vixen](https://github.com/rpcpool/yellowstone-vixen)
- [Old Faithful Archive](https://anza.xyz/old-faithful)

## Related Examples

- `stream`: Basic streaming server example
- `stream-parser`: Streaming with program-specific parsers
