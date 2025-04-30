# Yellowstone Vixen Mock

Yellowstone Vixen Mock provides tools for testing Vixen parsers without needing a live Solana node.
It supports offline fixtures, account replay, and instruction replay — helping you validate parsing logic quickly and reliably using devnet data.

## Features

-     🔌 Offline Testing
  Run parser unit tests without connecting to a live Solana node.
- 🗂 Fixture Management
  Load real Solana devnet accounts or transactions as JSON fixtures, reusable across tests.
- 🧪 Replay Support
  Replay devnet account updates and transaction instructions into your custom parsers.
- 🚀 Faster Development
  Build and debug parsing pipelines locally, with repeatable fixture-based tests.

Fixtures are fetched from Solana Devnet, not Mainnet.
This ensures safe and reproducible testing environments.

## Installation

```
cargo add yellowstone-vixen-mock
```

Example Usage

```rust
#[cfg(test)]
mod tests {
    use yellowstone_vixen_mock::{account_fixture, tx_fixture};
    use yellowstone_vixen_parser::{
        token_extension_program::InstructionParser as TokenExtensionProgramIxParser,
        token_program::{AccountParser as TokenProgramAccParser, TokenProgramState},
    };

    #[tokio::test]
    async fn test_account_parsing() {
        let parser = TokenProgramAccParser;
        let account = account_fixture!("3SmPYPvZfEmroktLiJsgaNENuPEud3Z52zSfLQ1zJdkK", &parser);

        let TokenProgramState::Mint(mint) = account else {
            panic!("Unexpected account state");
        };

        assert_eq!(mint.decimals, 10);
    }

    #[tokio::test]
    async fn test_instruction_parsing() {
        let parser = TokenExtensionProgramIxParser;
        let ixs = tx_fixture!(
            "44gWEyKUkeUabtJr4eT3CQEkFGrD4jMdwUV6Ew5MR5K3RGizs9iwbkb5Q4T3gnAaSgHxn3ERQ8g5YTXuLP1FrWnt",
            &parser
        );

        let Some(first_ix) = ixs.get(0) else {
            panic!("No instructions found");
        };

        tracing::info!("Parsed instruction: {:?}", first_ix);
    }
}
```
