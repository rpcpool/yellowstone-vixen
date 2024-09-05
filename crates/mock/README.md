# Yellowstone Vixen Mock

This crate provides a mock implementation of the Yellowstone Vixen Parser. It is intended to be used for testing purposes. The `load_fixture` function takes a fixture name as input, fetches the account on-chain if not already present, stores it as a JSON file inside the fixtures folder, and loads the account from the JSON file for testing purposes. Developers can use the loaded account to test their custom parsers and verify that the parser is correctly parsing the account data.

## Installation

```bash
cargo add yellowstone-vixen-mock
```

## Example

```rust
#[cfg(test)]
mod tests {
    use yellowstone_vixen_mock::{account_fixture, run_parse};

    use super::*;

    #[tokio::test]
    async fn test_mint_parsing() {
        let parser = TokenProgramParser;

        let account = account_fixture!("3SmPYPvZfEmroktLiJsgaNENuPEud3Z52zSfLQ1zJdkK");

        let state = run_parse!(parser, account);

        if let TokenProgramState::Mint(mint) = state {
            assert_eq!(mint.decimals, 10);
        } else {
            panic!("Invalid Mint Account");
        }
    }
}
```
