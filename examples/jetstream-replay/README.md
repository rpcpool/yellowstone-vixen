# Jetstream Replay Example

This example demonstrates how to use the Jetstream source with Vixen to replay historical Solana data from the Old Faithful archive and parse token program instructions.

## Overview

The Jetstream source enables streaming historical Solana ledger data at high throughput (>2.7M TPS) from the Old Faithful archive. This example integrates with the Vixen framework to:

- Configure the Jetstream source for epoch-based replay through Vixen
- Set up token program instruction parsers and handlers
- Showcase parsing and logging of SPL Token program instructions
- Run a historical data replay pipeline with instruction-level parsing

## Features

- ✅ Historical data replay from Old Faithful archive
- ✅ Support for epoch-based or slot-range queries
- ✅ Multi-threaded streaming with simplified direct processing
- ✅ Vixen integration with token program instruction parsing
- ✅ Structured logging of SPL Token instructions (Transfer, Mint, Burn, etc.)
- ✅ Configurable threading and buffering
- ✅ Prometheus metrics via Vixen runtime (blocks, transactions, parsing stats)

## Setup

Jetstreamer requires **Clang 16** (not 17) due to RocksDB dependencies. Install dependencies and set environment variables:

### Linux (Ubuntu/Debian)

```bash
# Install Clang 16
wget -qO- https://apt.llvm.org/llvm.sh | sudo bash -s -- 16
sudo apt update && sudo apt install -y gcc-13 g++-13 zlib1g-dev libssl-dev libtool

# Set as default
sudo update-alternatives --install /usr/bin/clang clang /usr/bin/clang-16 100
sudo update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-16 100

# Environment variables (add to ~/.bashrc)
export CC=clang
export CXX=clang++
export LIBCLANG_PATH=/usr/lib/llvm16/lib/libclang.so
```

### Linux (Arch)

```bash
sudo pacman -S clang16 llvm16 zlib openssl libtool
yay -S gcc13  # or use system gcc

# Environment variables (add to ~/.bashrc)
export CC=clang-16
export CXX=clang++-16
export LIBCLANG_PATH=/usr/lib/llvm16/lib/libclang.so
export LD_LIBRARY_PATH=/usr/lib/llvm16/lib:$LD_LIBRARY_PATH
```

### macOS

```bash
brew install llvm@16 zlib openssl libtool

# Environment variables (add to ~/.zshrc)
export CC=/opt/homebrew/opt/llvm@16/bin/clang
export CXX=/opt/homebrew/opt/llvm@16/bin/clang++
export LIBCLANG_PATH=/opt/homebrew/opt/llvm@16/lib/libclang.dylib
export LDFLAGS="-L/opt/homebrew/opt/llvm@16/lib"
export CPPFLAGS="-I/opt/homebrew/opt/llvm@16/include"
```

**Troubleshooting**: If you get RocksDB compilation errors, ensure you're using Clang 16 (not 17) and `LIBCLANG_PATH` is correctly set.

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
```

**Configuration Options:**

- `archive_url`: Old Faithful archive endpoint
- `epoch`: Which epoch to replay (mutually exclusive with `slot_start`/`slot_end`)
- `threads`: Number of parallel streaming threads

### Alternative: Slot Range Replay

To replay a specific slot range instead of an entire epoch:

```toml
[source]
archive_url = "https://api.old-faithful.net"
slot_start = 250000000
slot_end = 250100000
threads = 8
```

## Output

The example will output information about parsed token program instructions as they are processed:

```
Building Vixen runtime with token program instruction parser
Starting Vixen runtime...
instruction=Transfer source=11111111111111111111111111111112 destination=11111111111111111111111111111113 amount=1000000
instruction=MintTo mint=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v account=11111111111111111111111111111112 amount=50000000
instruction=TransferChecked source=11111111111111111111111111111112 destination=11111111111111111111111111111113 mint=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v amount=2500000 decimals=6
instruction=Burn account=11111111111111111111111111111112 mint=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v amount=100000
...
Jetstream replay with Vixen completed
SUCCESS - Vixen integration with token program parsing works!
```

## Performance Tuning

### Thread Count
- **2-4 threads**: Good for smaller ranges or testing
- **4-8 threads**: Recommended for most use cases
- **8+ threads**: For very large ranges with higher parallelism

## Limitations

1. **Token program focus**: Only parses SPL Token program instructions, other programs are not handled
2. **No account updates**: Jetstreamer provides blocks/transactions/rewards only
3. **Network dependent**: Requires connection to Old Faithful archive
4. **Historical only**: Not suitable for real-time streaming

## Troubleshooting

### High memory usage
- Reduce `threads`
- Process smaller ranges

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
- `filtered-pipeline`: Vixen pipeline with custom filters and parsers
- `prometheus`: Metrics collection example
