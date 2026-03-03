use std::borrow::Cow;

use yellowstone_vixen_core::{KeyBytes, ParseResult, Parser, Prefilter, ProgramParser, SlotUpdate};

#[derive(Debug, Clone, Copy)]
pub struct SlotParser;

impl Parser for SlotParser {
    type Input = SlotUpdate;
    type Output = SlotUpdate;

    fn id(&self) -> Cow<'static, str> { "yellowstone::SlotParser".into() }

    fn prefilter(&self) -> Prefilter { Prefilter::builder().slots().build().unwrap() }

    async fn parse(&self, slot: &SlotUpdate) -> ParseResult<Self::Output> { Ok(slot.to_owned()) }
}

impl ProgramParser for SlotParser {
    /// "S111111111111111111111111111111111111111112"
    #[inline]
    fn program_id(&self) -> KeyBytes<32> {
        KeyBytes::<32>::new([
            83, 144, 207, 224, 10, 18, 54, 164, 34, 33, 252, 0, 121, 244, 178, 50, 161, 204, 101,
            119, 224, 0, 123, 66, 92, 89, 105, 1, 1, 1, 1, 2,
        ])
    }
}
