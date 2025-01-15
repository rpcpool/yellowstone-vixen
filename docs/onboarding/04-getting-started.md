# Getting Started with Yellowstone Vixen

## Prerequisites

Before you begin, ensure you have the following installed:
- Rust (nightly-2024-02-01)
- Protocol Buffer compiler
- OpenSSL development libraries
- Git

<details>
<summary>macOS Installation</summary>

```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Protocol Buffers
brew install protobuf

# Install OpenSSL
brew install openssl@3

# Set required environment variables
export OPENSSL_DIR=$(brew --prefix openssl@3)
```
</details>

<details>
<summary>Linux Installation</summary>

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y protobuf-compiler libssl-dev pkg-config

# Fedora
sudo dnf install -y protobuf-compiler openssl-devel pkg-config

# Arch Linux
sudo pacman -S protobuf openssl pkg-config
```
</details>

<details>
<summary>Windows Installation</summary>

```powershell
# Install Rust
Invoke-WebRequest https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
.\rustup-init.exe

# Install Chocolatey if not already installed
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Install Protocol Buffers
choco install protoc

# Install OpenSSL
choco install openssl
```
</details>

## Setting Up the Development Environment

1. **Clone the Repository**
```bash
git clone https://github.com/rpcpool/yellowstone-vixen.git
cd yellowstone-vixen
```

2. **Set Rust Toolchain**
```bash
# Install nightly toolchain
rustup toolchain install nightly-2024-02-01
rustup override set nightly-2024-02-01

# Install required components
rustup component add rustfmt
rustup component add clippy
```

3. **Build the Project**
```bash
# Build all crates
cargo build

# Run tests
cargo test

# Run with features
cargo build --features "proto token-program"
```

## Project Configuration

1. **Create Configuration File**
```bash
# Copy example configuration
cp Vixen.example.toml Vixen.toml

# Edit configuration as needed
$EDITOR Vixen.toml
```

2. **Environment Setup**
```bash
# Set development environment variables
export RUST_LOG=debug
export RUST_BACKTRACE=1
```

## Running the Service

1. **Start the Service**
```bash
# Run in development mode
cargo run --bin yellowstone-vixen

# Run with specific features
cargo run --bin yellowstone-vixen --features "proto token-program"
```

2. **Development Tools**
```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests with coverage
cargo tarpaulin
```

## Common Development Tasks

### Adding a New Parser

1. Create a new module in `crates/parser/src/`
2. Implement the `Parser` trait
3. Add tests in the module's test directory
4. Register the parser in `lib.rs`

Example:
```rust
use yellowstone_vixen_core::{Parser, ParseResult};

pub struct MyParser;

impl Parser for MyParser {
    type Input = InstructionUpdate;
    type Output = InstructionUpdate;

    fn id(&self) -> std::borrow::Cow<str> {
        "my_parser::MyParser".into()
    }

    async fn parse(&self, instruction: &Self::Input) -> ParseResult<Self::Output> {
        // Implementation here
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_my_parser() {
        // Test implementation
    }
}
```

### Running Integration Tests

```bash
# Run all integration tests
cargo test --test '*'

# Run specific test suite
cargo test --test integration_test
```

### Debugging

1. **Enable Logging**
```bash
export RUST_LOG=debug
export RUST_BACKTRACE=1
```

2. **Use VS Code Debug Configuration**
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug Vixen",
            "cargo": {
                "args": ["build", "--bin=yellowstone-vixen"]
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

## Monitoring & Metrics

1. **Start Prometheus**
```bash
docker-compose up -d prometheus
```

2. **Start Grafana**
```bash
docker-compose up -d grafana
```

3. **Access Dashboards**
- Prometheus: http://localhost:9090
- Grafana: http://localhost:3000

## Contributing

1. Create a new branch
```bash
git checkout -b feature/my-feature
```

2. Make changes and commit
```bash
git add .
git commit -m "feat(parser): add new feature"
```

3. Run checks
```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test
```

4. Push changes and create PR
```bash
git push origin feature/my-feature
```

## Troubleshooting

### Common Issues

1. **Build Failures**
- Check Rust toolchain version
- Verify protobuf compiler installation
- Ensure OpenSSL is properly configured

2. **Runtime Errors**
- Check configuration file
- Verify environment variables
- Review log output

3. **Test Failures**
- Run tests with verbose output
- Check test dependencies
- Verify mock data

### Getting Help

- Check existing issues on GitHub
- Join our Discord community
- Review documentation in `/docs`
- Contact maintainers

## Next Steps

- Review [Technical Decisions](02-technical-decisions.md)
- Explore [Project Overview](01-project-overview.md)
- Check [Technical Debt](03-technical-debt.md)
- Join development discussions