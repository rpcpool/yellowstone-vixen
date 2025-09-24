# Yellowstone Vixen Integration Tests

## Configuration

There are two ways to configure the integration tests:

### Option 1: Configuration File (Recommended)

1. **Create your test configuration file**:
   ```bash
   cp tests/Vixen.example.toml tests/Vixen.test.toml
   ```

2. **Edit the configuration file** with your actual credentials:
   ```toml
   [yellowstone]
   endpoint = "https://your-actual-endpoint.rpcpool.com"
   x-token = "your-actual-token"
   timeout = 30
   ```

### Option 2: Environment Variables

Set these environment variables:
```bash
export GRPC_URL="your-grpc-endpoint"           # Required
export GRPC_AUTH_TOKEN="your-auth-token"      # Optional
export GRPC_TIMEOUT="30"                      # Optional, defaults to 30 seconds
```

## Running Tests

**All Integration Tests:**
```bash
# Using custom config file
cargo test --test integration_test -- --ignored -- --config tests/Vixen.my-test.toml

# Using default test config (tests/Vixen.test.toml)
cargo test --test integration_test -- --ignored
```
