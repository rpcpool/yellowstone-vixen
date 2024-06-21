# Yellowstone Vixen (Formerly Solana Indexer Stack)

Yellowstone Vixen allows dApp developers to build program-aware change event streams for Solana. It provides the building blocks, such as a runtime, parser specification, and handler specification, to create custom indexes for specific programs, accounts, and transactions. Vixen enables developers to assemble program parsers to process real-time change events from Dragonmouth, converting them into program-aware structures. These structures can then be stored in a database or used in other data pipelines.

## Table of Contents

1. [Objectives](#objectives)
2. [Requirements](#requirements)
3. [Example](#example)
4. [Dragonmouth](#dragonmouth)
5. [Developers](#developers)

## Objectives

1. **Cost Efficiency**: Utilizing Dragonmouth, multiple Vixen instances can share a single geyser stream. With various filter options, storage costs focus on what's essential for the dApp.
2. **Operational Simplicity**: Vixen requires minimal configuration and dependency on other systems, making it easy to operate.
3. **Recovery**: In the event of a crash or cold start, operators can designate a starting slot. Vixen, in conjunction with Dragonmouth, replays all transactions and accounts from the specified slot until reaching the active slot, then switches to real-time processing.
4. **Auditability**: Operators can conduct verifiable audits to check the accounts and transactions processed by the index and at which slot.
5. **Observability**: Operators can monitor the health of their installation, gaining insights into lag, throughput, and error rates.
6. **Composability**: Program parsers are developed as separate modules (cargo crates), enabling programs to include other parsers needed to deserialize cross-program invocations (CPI).

## Requirements

1. **Parser**: A module responsible for transforming raw Solana data into a program-specific format.
2. **Handler**: A module that processes the parsed data, performing tasks such as logging, storing in a database, or triggering other actions.
3. **HandlerManager**: Manages multiple handlers for different types of data (e.g., accounts, transactions).
4. **Configuration**: A TOML file specifying the settings and parameters for Vixen.

## Example

This example demonstrates how a developer can implement a generic parsing pipeline with Vixen. The example is located in the [`/crates/test`](/crates/test) directory.

To run the example, navigate to the test directory and execute the following command:

```
cd crates/test
RUST_LOG=info cargo run -- --config "$(pwd)/Vixen.toml"
```

You can find an example configuration file at [`Vixen.toml`](/Vixen.toml).

### Explanation

In this example, you need to implement specific components to create a functional parsing pipeline:

- **CustomParser Struct**: Defines the parsing logic for the specific program. The `prefilter` method sets up filters for the accounts owned by the target program, which are used to build the underlying Dragonmouth subscription. The `parse` method contains the logic to transform raw account data into the desired structure.

```rust
pub struct CustomParser;

impl vixen_core::Parser for CustomParser {
    type Input = AccountUpdate;
    type Output = CustomParsedData; // Replace with the actual data type

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([CUSTOM_PROGRAM_ID]) // Replace with the actual program ID
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        // Implement parsing logic here
        // Example: Ok(CustomParsedData::from(acct))
        unimplemented!()
    }
}
```

- **CustomHandler Struct**: Defines how the parsed data should be handled. This could involve logging the data, storing it in a database, or triggering other actions.

```rust
pub struct CustomHandler;

impl<H: std::fmt::Debug + Sync> vixen::Handler<H> for CustomHandler {
    async fn handle(&self, value: &H) -> vixen::HandlerResult<()> {
        // Implement handling logic here
        // Example: tracing::info!(?value);
        unimplemented!()
    }
}
```

- **Main Function**: Sets up the tracing subscriber, reads the configuration file, and runs the Vixen framework with the specified handlers and managers.

```rust
fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Opts { config } = Opts::parse();
    let config = std::fs::read_to_string(config).expect("Error reading config file");
    let config = toml::from_str(&config).expect("Error parsing config");

    vixen::run(config, HandlerManagers {
        account: HandlerManager::new([handler::boxed(vixen::HandlerPack::new(CustomParser, [CustomHandler]))]),
        transaction: HandlerManager::empty(),
    });
}
```

### Token Extensions Example

To illustrate, here's how you might implement the `CustomParser` and `CustomHandler` for parsing the token extension program:

- **Parser for Token Extension Program**:

```rust
pub struct Parser;

impl vixen_core::Parser for Parser {
    type Input = AccountUpdate;
    type Output = spl_token_2022::state::Account;

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([spl_token_2022::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acct.account.as_ref().ok_or(ProgramError::InvalidArgument)?;

        let acct = spl_token_2022::state::Account::unpack(
            inner
                .data
                .get(..spl_token_2022::state::Account::LEN)
                .ok_or(ProgramError::InvalidArgument)?,
        )?;

        Ok(acct)
    }
}
```

- **Handler for Logging Parsed Accounts**:

```rust
pub struct Handler;

impl<H: std::fmt::Debug + Sync> vixen::Handler<H> for Handler {
    async fn handle(&self, value: &H) -> vixen::HandlerResult<()> {
        tracing::info!(?value);
        Ok(())
    }
}
```

This setup shows how to use Vixen to create an efficient indexing solution for specific needs on the Solana blockchain. By following this pattern, developers can build their custom parsers and handlers for various Solana programs and data pipelines.

## Dragonmouth

Dragonmouth can be self-hosted as a Geyser plugin or used via a commercial vendor. For more details, refer to the [Yellowstone Dragonmouth documentation](https://docs.triton.one/project-yellowstone/dragons-mouth-grpc-subscriptions) and [Yellow Stone repository](https://github.com/rpcpool/yellowstone-grpc).

## Developers

This project is developed by [ABK Labs](https://abklabs.com/) and [Triton One](https://triton.one/).
