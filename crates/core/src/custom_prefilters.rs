use std::str::FromStr;

use crate::{ParseResult, Parser, Prefilter, ProgramParser, Pubkey, TransactionPrefilter};

/// A marker trait that enables custom prefilters to be applied to a parser.
pub trait CustomPrefilters: Parser + Sized {
    /// Create a new `InstructionParserWithCustomPrefilters` with the given parser.
    ///  This allows to set custom transaction filters
    ///
    /// # Example
    ///
    /// ```rust
    /// use yellowstone_vixen_core::custom_prefilters::CustomPrefilters;
    ///
    /// impl CustomPrefilters for MyParser {}
    ///
    /// let parser = MyParser.filter()
    ///     .include_accounts([TokenProgramIxParser]) // Optional  
    ///     .required_accounts([PumpAmmAccParser]); //Optional
    /// ```
    ///
    /// ---
    ///
    /// ### Note: You can also use strings or `Pubkey` directly:
    /// ```rust
    /// let parser = MyParser.filter()
    ///  .include_accounts(["TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"])
    ///  .required_accounts([Pubkey::from_str("").unwrap()]);
    /// ```
    fn filter(self) -> InstructionParserWithCustomPrefilters<Self> {
        InstructionParserWithCustomPrefilters {
            parser: self,
            include_accounts: vec![],
            required_accounts: vec![],
        }
    }
}

/// A wrapper around a parser that allows for custom prefilters to be applied to the transaction.
#[derive(Debug)]
pub struct InstructionParserWithCustomPrefilters<T: Parser> {
    parser: T,
    include_accounts: Vec<Pubkey>,
    required_accounts: Vec<Pubkey>,
}

impl<T> Parser for InstructionParserWithCustomPrefilters<T>
where
    T: Parser + Send + Sync,
    T::Input: Send + Sync,
{
    type Input = T::Input;
    type Output = T::Output;

    fn id(&self) -> std::borrow::Cow<str> { self.parser.id() }

    fn prefilter(&self) -> Prefilter {
        let mut prefilter = self.parser.prefilter();
        let mut tx_prefilter = prefilter
            .transaction
            .expect("Instruction Parser must have a transaction prefilter");

        tx_prefilter.merge(TransactionPrefilter {
            accounts_include: self.include_accounts.iter().copied().collect(),
            accounts_required: self.required_accounts.iter().copied().collect(),
        });

        prefilter.transaction = Some(tx_prefilter);

        prefilter
    }

    async fn parse(&self, input: &Self::Input) -> ParseResult<Self::Output> {
        self.parser.parse(input).await
    }
}

impl<T> InstructionParserWithCustomPrefilters<T>
where
    T: Parser + Send + Sync,
    T::Input: Send + Sync,
{
    /// Set the included accounts for this transaction prefilter.
    ///
    /// **Note:** If the transaction does not include at least ONE of the accounts set here, the
    /// transaction will not be retrieved.
    #[must_use]
    #[allow(private_bounds)] // CustomPrefiltersAccount is meant to be used internally
    pub fn include_accounts<F: CustomPrefiltersAccount>(
        mut self,
        include_accounts: impl IntoIterator<Item = F>,
    ) -> Self {
        self.include_accounts
            .extend(include_accounts.into_iter().map(|f| f.get_pubkey()));

        self
    }

    /// Set the required accounts for this transaction prefilter.
    ///  The accounts set here **must** be present in the transaction.
    ///
    /// **Note:** If the transaction does not include ALL of the accounts set here, the
    /// transaction will not be retrieved.
    ///
    /// **The Program ID of the Parser program will always be included in this list
    #[must_use]
    #[allow(private_bounds)] // CustomPrefiltersAccount is meant to be used internally
    pub fn required_accounts<F: CustomPrefiltersAccount>(
        mut self,
        required_accounts: impl IntoIterator<Item = F>,
    ) -> Self {
        self.required_accounts
            .extend(required_accounts.into_iter().map(|f| f.get_pubkey()));

        self
    }
}

pub(crate) trait CustomPrefiltersAccount {
    fn get_pubkey(&self) -> Pubkey;
}

impl CustomPrefiltersAccount for Pubkey {
    fn get_pubkey(&self) -> Pubkey { *self }
}

impl<T: ProgramParser> CustomPrefiltersAccount for T {
    fn get_pubkey(&self) -> Pubkey { self.program_id() }
}

impl CustomPrefiltersAccount for &'static str {
    fn get_pubkey(&self) -> Pubkey { Pubkey::from_str(self).unwrap() }
}
