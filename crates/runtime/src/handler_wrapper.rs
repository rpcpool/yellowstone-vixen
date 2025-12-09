#![allow(dead_code)]

use crate::{Handler, HandlerResult};

#[derive(Debug)]
pub struct WrappingHandler<T: Sync, R: Sync, H: Handler<T, R>> {
    inner: H,
    _phantom_t: std::marker::PhantomData<T>,
    _phantom_r: std::marker::PhantomData<R>,
}

impl<T: Sync, R: Sync, H: Handler<T, R>> WrappingHandler<T, R, H> {
    pub fn new(handler: H) -> Self {
        Self {
            inner: handler,
            _phantom_t: std::marker::PhantomData,
            _phantom_r: std::marker::PhantomData,
        }
    }
}

impl<T: Sync, R: Sync, H: Handler<T, R> + Sync> Handler<T, R> for WrappingHandler<T, R, H> {
    async fn handle(
        &self,
        value: &T,
        input: &R,
    ) -> HandlerResult<()> {
        self.inner.handle(value, input).await
    }
}
