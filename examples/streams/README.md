# Simple Example

This section provides a simple example to help you get started with setting up Vixen for parsing and handling data from the token extension program on Solana. The example demonstrates how to implement a custom parser and handler, which are essential components in the Vixen framework.

## Overview

In this example, we will create a custom parser to transform raw Solana account data into a structured format and a custom handler to log the parsed data. These components will work together to process and handle data in a streamlined manner.

## Custom Parser for Token Extension Program

The custom parser is responsible for filtering and transforming raw account data into a specific format that the application can use. The `prefilter` method sets up filters for the accounts owned by the target program, while the `parse` method contains the logic to transform raw account data into the desired structure.

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

### Overview

- **prefilter**: This method defines the filters used to identify relevant accounts based on their owners. In this case, it filters for accounts owned by the `spl_token_2022::ID`.
- **parse**: This asynchronous method processes the raw account data, unpacks it, and converts it into a structured `spl_token_2022::state::Account` format.

## Handler for Logging Parsed Accounts

The custom handler is responsible for processing the parsed data. In this example, the handler logs the parsed account information.

```rust
pub struct Handler;

impl<H: std::fmt::Debug + Sync> vixen::Handler<H> for Handler {
    async fn handle(&self, value: &H) -> vixen::HandlerResult<()> {
        tracing::info!(?value);
        Ok(())
    }
}
```

### Overview

- **handle**: This asynchronous method takes the parsed data and logs it using the `tracing` crate. The `tracing::info!` macro is used to log the data in a structured and readable format.

## How the Parser and Handler Work Together

The parser and handler components work together to process and manage data within the Vixen framework. Here’s how they interact:

1. **Prefiltering**: The parser’s `prefilter` method sets up the criteria for filtering relevant accounts. This ensures that only the accounts owned by the target program are processed.
2. **Parsing**: When an account update matches the prefilter criteria, the parser’s `parse` method is called. This method transforms the raw account data into a structured format.
3. **Handling**: Once the data is parsed, the handler’s `handle` method is invoked. This method processes the structured data, which in this example involves logging the parsed account information.

By using these components, Vixen allows developers to create a robust and flexible data processing pipeline tailored to their specific needs. This example provides a foundational understanding of how to implement and integrate custom parsers and handlers within the Vixen framework.
