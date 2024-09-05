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
    use yellowstone_vixen_mock::{account_fixture, run_account_parse, tx_fixture, run_ix_parse, FixtureData};
    // using token program and token extension program parsers
    use yellowstone_vixen_parser::{
        token_extension_program::{
        ix_parser::{TokenExtensionProgramIxParser,TokenExtensionProgramIx}
        },
        token_program::{account_parser::{TokenProgramAccParser,TokenProgramState}, ix_parser::{TokenProgramIxParser,TokenProgramIx}},
    };

    #[tokio::test]
    async fn test_mint_parsing() {
        let parser = TokenProgramParser;

        let fixture_data = account_fixture!("3SmPYPvZfEmroktLiJsgaNENuPEud3Z52zSfLQ1zJdkK");

        if let FixtureData::Account(account) = fixture_data {
            let state = run_account_parse!(parser, account);

            if let TokenProgramState::Mint(mint) = state {
                assert_eq!(mint.decimals, 10);
            } else {
                panic!("Invalid Account");
            }
        } else {
            panic!("Invalid Fixture Data");
        }
    }

    // Transaction fixture
    #[tokio::test]
    async fn test_mint_to_checked_ix_parsing() {
        let parser = TokenExtensionProgramIxParser;

        let fixture_data = tx_fixture!("44gWEyKUkeUabtJr4eT3CQEkFGrD4jMdwUV6Ew5MR5K3RGizs9iwbkb5Q4T3gnAaSgHxn3ERQ8g5YTXuLP1FrWnt");

        if let FixtureData::Instructions(ixs) = fixture_data {
            let ix = run_ix_parse!(parser, &ixs[0]);
            match ix {
                TokenExtensionProgramIx::TokenProgramIx(ix) => {
                    if let TokenProgramIx::MintToChecked(ix) = ix {
                        assert!(ix.data.is_some());
                        let data = ix.data.as_ref().unwrap();
                        assert_eq!(data.decimals, 9);
                        assert_eq!(data.amount, 100.mul(10u64.pow(data.decimals as u32)));
                    } else {
                        panic!("Invalid Instruction")
                    }
                },
                _ => panic!("Invalid Instruction"),
            }
        } else {
            panic!("Invalid Fixture Data")
        }
    }

}
```
