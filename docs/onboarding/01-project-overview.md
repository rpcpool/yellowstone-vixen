# Yellowstone Vixen - Project Overview

## Introduction
Yellowstone Vixen is a high-performance Rust-based project that provides a gRPC service for parsing Solana transactions. It's designed to work seamlessly with the Yellowstone ecosystem, offering efficient and reliable transaction parsing capabilities.

## Architecture Overview

```mermaid
graph TB
    Client[Client Applications] -->|gRPC Requests| Vixen[Yellowstone Vixen Service]
    Vixen -->|Parse| Transactions[Solana Transactions]
    Vixen -->|Metrics| Prometheus[Prometheus Monitoring]
    Vixen -->|Tracing| OpenTelemetry[OpenTelemetry]
    
    subgraph "Vixen Service Components"
        Parser[Transaction Parser]
        ProtoGen[Protocol Buffer Definitions]
        Metrics[Metrics Collection]
        Config[Configuration Management]
    end

    Vixen --> Parser
    Vixen --> ProtoGen
    Vixen --> Metrics
    Vixen --> Config
```

## Key Components

### 1. gRPC Service
- Provides a high-performance interface for transaction parsing
- Uses Protocol Buffers for efficient data serialization
- Supports bi-directional streaming capabilities
- Configurable connection pooling and resource management

### 2. Parser System
```mermaid
flowchart LR
    TX[Raw Transaction] --> Parser[Transaction Parser]
    Parser --> Accounts[Account Parser]
    Parser --> Instructions[Instruction Parser]
    Parser --> |Optional| PreFilter[Account PreFilter]
    
    Accounts --> |Proto| ParsedAccounts[Parsed Accounts]
    Instructions --> |Proto| ParsedInstructions[Parsed Instructions]
```

### 3. Monitoring & Observability
- Prometheus integration for metrics
- OpenTelemetry support for distributed tracing
- Configurable logging levels
- Performance monitoring dashboards

## Technology Stack
- **Language**: Rust (nightly-2024-02-01)
- **Framework**: gRPC with Tonic
- **Serialization**: Protocol Buffers
- **Monitoring**: Prometheus & OpenTelemetry
- **Configuration**: TOML-based with dynamic reloading

## Development Environment
```mermaid
graph LR
    Dev[Development] -->|Rust Toolchain| Build[Build Process]
    Build --> Tests[Testing]
    Build --> Docker[Docker Image]
    Docker --> Deploy[Deployment]
    
    subgraph "Development Tools"
        Clippy[Clippy Linter]
        Fmt[Rustfmt]
        Proto[Protobuf Compiler]
        Test[Test Framework]
    end
```

## Project Structure
```
yellowstone-vixen/
├── crates/                    # Workspace crates
│   ├── core/                  # Core types and traits
│   ├── parser/               # Transaction parsers
│   ├── proto/                # Protocol buffer definitions
│   └── test/                 # Testing utilities
├── docs/                     # Documentation
│   └── onboarding/          # Onboarding guides
├── examples/                 # Usage examples
└── scripts/                  # Development scripts
```

## Getting Started
For detailed setup instructions, see [Getting Started Guide](04-getting-started.md).