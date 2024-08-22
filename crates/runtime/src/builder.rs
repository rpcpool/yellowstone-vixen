use tracing::error;

use crate::{
    config::{MaybeDefault, VixenConfig},
    metrics::{Counters, Metrics, MetricsFactory, NullMetrics},
    Chain, HandlerManagers, Runtime,
};

#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("Missing field {0:?}")]
    MissingField(&'static str),
    #[error("Missing config section {0:?}")]
    MissingConfig(&'static str),
    #[error("Error instantiating metrics backend")]
    Metrics(#[source] Box<dyn std::error::Error>),
}

#[derive(Debug)]
pub struct RuntimeBuilder<A, X, M> {
    err: Result<(), BuilderError>,
    manager: Option<HandlerManagers<A, X>>,
    metrics: M,
}

impl<A, X> Default for RuntimeBuilder<A, X, NullMetrics> {
    fn default() -> Self {
        Self {
            err: Ok(()),
            manager: None,
            metrics: NullMetrics,
        }
    }
}

#[inline]
fn unwrap<T>(name: &'static str, val: Option<T>) -> Result<T, BuilderError> {
    val.ok_or(BuilderError::MissingField(name))
}

#[inline]
fn unwrap_cfg<T>(name: &'static str, val: Option<T>) -> Result<T, BuilderError> {
    val.ok_or(BuilderError::MissingConfig(name))
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

    pub fn metrics<N>(self, metrics: N) -> RuntimeBuilder<A, X, N> {
        let Self {
            err,
            manager,
            metrics: _,
        } = self;

        RuntimeBuilder {
            err,
            manager,
            metrics,
        }
    }

    pub fn manager(self, manager: HandlerManagers<A, X>) -> Self {
        self.mutate(|s| s.manager = Some(manager))
    }
}

impl<A, X, M: MetricsFactory> RuntimeBuilder<A, X, M> {
    pub fn try_build(
        self,
        config: VixenConfig<M::Config>,
    ) -> Result<Runtime<A, X, M>, BuilderError> {
        let Self {
            err,
            manager,
            metrics,
        } = self;
        let () = err?;

        let VixenConfig {
            yellowstone: yellowstone_cfg,
            buffer: buffer_cfg,
            metrics: metrics_cfg,
        } = config;

        let metrics_cfg = unwrap_cfg(
            "metrics",
            metrics_cfg.opt().or_else(MaybeDefault::default_opt),
        )?;

        let Metrics(instrumenter, exporter) = metrics
            .create(metrics_cfg, "vixen")
            .map_err(|e| BuilderError::Metrics(e.into()))?;

        Ok(Runtime {
            yellowstone_cfg,
            buffer_cfg,
            manager: unwrap("manager", manager)?,
            counters: Counters::new(&instrumenter),
            exporter,
        })
    }

    #[inline]
    pub fn build(self, config: VixenConfig<M::Config>) -> Runtime<A, X, M> {
        self.try_build(config).unwrap_or_else(|e| {
            error!(e = %Chain(&e), "Error building Vixen runtime");
            panic!("Error building Vixen runtime: {e}")
        })
    }
}
