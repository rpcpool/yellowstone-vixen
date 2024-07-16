use crate::{
    metrics::{Metrics, MetricsBackend, NullMetrics},
    HandlerManagers, IndexerOpts, Runtime,
};

#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum BuilderError {
    #[error("Missing field {0}")]
    MissingField(&'static str),
}

#[derive(Debug)]
pub struct RuntimeBuilder<A, X, M> {
    err: Result<(), BuilderError>,
    opts: Option<IndexerOpts>,
    manager: Option<HandlerManagers<A, X>>,
    metrics: M,
}

impl<A, X> Default for RuntimeBuilder<A, X, NullMetrics> {
    fn default() -> Self {
        Self {
            err: Ok(()),
            opts: None,
            manager: None,
            metrics: NullMetrics,
        }
    }
}

#[inline]
fn unwrap<T>(name: &'static str, val: Option<T>) -> Result<T, BuilderError> {
    val.ok_or(BuilderError::MissingField(name))
}

impl<A, X, M> RuntimeBuilder<A, X, M> {
    #[inline]
    fn mutate(self, mutate: impl FnOnce(&mut Self)) -> Self {
        self.try_mutate(|s| {
            mutate(s);
            Ok(())
        })
    }

    #[inline]
    fn try_mutate(mut self, mutate: impl FnOnce(&mut Self) -> Result<(), BuilderError>) -> Self {
        if let Ok(()) = self.err {
            self.err = mutate(&mut self);
        }
        self
    }

    pub fn opts(self, opts: IndexerOpts) -> Self { self.mutate(|s| s.opts = Some(opts)) }

    pub fn metrics<N>(self, metrics: N) -> RuntimeBuilder<A, X, N> {
        let Self {
            err,
            opts,
            manager,
            metrics: _,
        } = self;

        RuntimeBuilder {
            err,
            opts,
            manager,
            metrics,
        }
    }

    pub fn manager(self, manager: HandlerManagers<A, X>) -> Self {
        self.mutate(|s| s.manager = Some(manager))
    }
}

impl<A, X, M: MetricsBackend> RuntimeBuilder<A, X, M> {
    pub fn try_build(self) -> Result<Runtime<A, X, M>, BuilderError> {
        let Self {
            err,
            opts,
            manager,
            metrics,
        } = self;
        let () = err?;

        Ok(Runtime {
            opts: unwrap("opts", opts)?,
            manager: unwrap("manager", manager)?,
            metrics: Metrics::new(&metrics),
        })
    }

    #[inline]
    pub fn build(self) -> Runtime<A, X, M> { self.try_build().unwrap() }
}
