# Yellowstone Vixen Mock

This crate provides a mock implementation of the Yellowstone Vixen Parser. It is intended to be used for testing purposes.
the load_account_fixtures function takes pubkey, cluster as inputs and fetches the account on-chain and stores it as a jsom file inside fixtures folder and loads the account from the json file for testing purposes. Devs can use the loaded account to test their custom parsers and verify Parser is parsing the account data correctly.

## Example

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

#[cfg(test)]
mod tests {
    const account_pubkey: &str = "DhEP4nTn6DdQA12PZoWN9paKTviPGFf6JzeneB4hGVb2";

    #[tokio::test]
    async fn mock() {
        let account = load_account_fixtures(account_pubkey, ClusterType::Devnet).await.unwrap();
        let parser = TokenExtensionProgramParser;

        let data = parser.parse(&sub_account_update).await.unwrap();

        let data = data.unwrap();
        println!("Parsed Data:{:?}", data);
    }
}

```
