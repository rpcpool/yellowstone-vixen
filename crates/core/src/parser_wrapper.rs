#![allow(dead_code)]
use std::borrow::Cow;
use crate::{ParseError, ParseResult, Parser, Prefilter};

#[derive(Debug, Copy, Clone)]
pub struct IgnoringErrorParserWrapper<P: Parser, I: Send + Sync, O> {
    inner: P,
    _phantom_input: std::marker::PhantomData<I>,
    _phantom_output: std::marker::PhantomData<O>,
}

impl<P: Parser<Input = I, Output = O> + Sync, I: Send + Sync, O: Sync>
IgnoringErrorParserWrapper<P, I, O>
{
    pub fn new(inner: P) -> Self {
        Self {
            inner,
            _phantom_input: std::marker::PhantomData,
            _phantom_output: std::marker::PhantomData,
        }
    }
}

impl<P: Parser<Input = I, Output = O> + Sync, I: Send + Sync, O: Sync> Parser
for IgnoringErrorParserWrapper<P, I, O>
{
    type Input = I;
    type Output = O;

    fn id(&self) -> Cow<'static, str> {
        Cow::Owned(format!("Parser Wrapper for ... {}", self.inner.id()))
    }

    fn prefilter(&self) -> Prefilter {
        self.inner.prefilter()
    }

    async fn parse(&self, ix_update: &I) -> ParseResult<O> {
        let parse_result = self.inner.parse(ix_update).await;

        if let Err(ParseError::Other(_err)) = &parse_result {
            return Err(ParseError::Filtered);
        }

        parse_result
    }
}
