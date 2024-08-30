use yellowstone_vixen_core::{Parser, ProgramParser};
use yellowstone_vixen_proto::prost::{Message, Name};

pub trait IntoProto: Parser {
    type Proto: Message + Name + Send + Sync;

    fn into_proto(value: Self::Output) -> Self::Proto;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Proto<T>(T);

impl<T> Proto<T> {
    #[inline]
    pub fn new(value: T) -> Self { Self(value) }

    #[inline]
    pub fn into_inner(self) -> T { self.0 }
}

impl<T> From<T> for Proto<T> {
    #[inline]
    fn from(value: T) -> Self { Self(value) }
}

impl<T> std::ops::Deref for Proto<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> std::ops::DerefMut for Proto<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<T: ProgramParser + IntoProto + Sync> ProgramParser for Proto<T>
where T::Input: Sync
{
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { self.0.program_id() }
}

impl<T: Parser + IntoProto + Sync> Parser for Proto<T>
where T::Input: Sync
{
    type Input = T::Input;
    type Output = T::Proto;

    #[inline]
    fn id(&self) -> std::borrow::Cow<str> { self.0.id() }

    #[inline]
    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter { self.0.prefilter() }

    #[inline]
    async fn parse(
        &self,
        value: &Self::Input,
    ) -> yellowstone_vixen_core::ParseResult<Self::Output> {
        self.0.parse(value).await.map(T::into_proto)
    }
}
