use std::{collections::HashMap, fmt::Debug};

use tokio::sync::broadcast;
use vixen_core::{
    instruction::InstructionUpdate, AccountUpdate, Parser, ProgramParser, Pubkey, TransactionUpdate,
};
use yellowstone_vixen_proto::{
    prost::{Message, Name},
    prost_types::Any,
};

use super::{
    config::StreamConfig,
    grpc::{Channels, GrpcHandler, Receiver},
    Server,
};
use crate::{
    builder::{Builder, BuilderKind, RuntimeBuilder, RuntimeKind},
    handler::{BoxPipeline, Pipeline},
    metrics::{MetricsFactory, NullMetrics},
    util,
};

/// An error thrown by the Vixen stream server builder.
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    /// Two program parsers were registered with the same program ID.
    #[error("Parser with duplicate ID {1:?} and duplicate program ID {0} registered")]
    DuplicateId(Pubkey, String),
    /// An error occurred while building the underlying Vixen runtime.
    #[error("Error building Vixen runtime")]
    Runtime(#[from] crate::builder::BuilderError),
}

/// Marker type for the [`StreamBuilder`] type.
#[derive(Debug, Default)]
pub struct StreamKind<'a>(Vec<&'a [u8]>, Channels<HashMap<String, Receiver>>);
/// A builder for the [`Server`] type.
pub type StreamBuilder<'a, M = NullMetrics> = Builder<StreamKind<'a>, M>;

impl<'a> BuilderKind for StreamKind<'a> {
    type Error = BuilderError;
}

fn wrap_parser<P: Debug + Parser + Send + Sync + 'static>(
    parser: P,
    tx: broadcast::Sender<Any>,
) -> BoxPipeline<'static, P::Input>
where
    P::Input: Sync,
    P::Output: Message + Name + Send + Sync,
{
    Box::new(Pipeline::new(parser, [GrpcHandler(tx)]))
}

impl<'a, M: MetricsFactory> StreamBuilder<'a, M> {
    fn insert<
        P: Debug + ProgramParser + Send + Sync + 'static,
        F: FnOnce(&mut Self) -> &mut Vec<BoxPipeline<'static, P::Input>>,
    >(
        self,
        parser: P,
        f: F,
    ) -> Self
    where
        P::Input: Sync,
        P::Output: Message + Name + Send + Sync,
    {
        self.try_mutate(|s| {
            use std::collections::hash_map::Entry;

            // TODO: configure channel size
            let (tx, rx) = broadcast::channel(64);

            match s.extra.1.entry(parser.program_id()) {
                Entry::Vacant(v) => {
                    v.insert([(parser.id().into_owned(), rx)].into_iter().collect());
                },
                Entry::Occupied(o) => match o.into_mut().entry(parser.id().into_owned()) {
                    Entry::Vacant(v) => {
                        v.insert(rx);
                    },
                    Entry::Occupied(_) => {
                        return Err(BuilderError::DuplicateId(
                            parser.program_id(),
                            parser.id().into_owned(),
                        ));
                    },
                },
            }
            f(s).push(wrap_parser(parser, tx));
            Ok(())
        })
    }

    pub fn descriptor_set(self, desc: &'a [u8]) -> Self {
        self.mutate(|s| s.extra.0.push(desc))
    }

    /// Add a new account parser to the builder.
    pub fn account<A: Debug + ProgramParser<Input = AccountUpdate> + Send + Sync + 'static>(
        self,
        account: A,
    ) -> Self
    where
        A::Input: Sync,
        A::Output: Message + Name + Send + Sync,
    {
        self.insert(account, |s| &mut s.account)
    }

    /// Add a new transaction parser to the builder.
    pub fn transaction<
        T: Debug + ProgramParser<Input = TransactionUpdate> + Send + Sync + 'static,
    >(
        self,
        transaction: T,
    ) -> Self
    where
        T::Input: Sync,
        T::Output: Message + Name + Send + Sync,
    {
        self.insert(transaction, |s| &mut s.transaction)
    }

    /// Add a new instruction parser to the builder.
    pub fn instruction<
        T: Debug + ProgramParser<Input = InstructionUpdate> + Send + Sync + 'static,
    >(
        self,
        instruction: T,
    ) -> Self
    where
        T::Input: Sync,
        T::Output: Message + Name + Send + Sync,
    {
        self.insert(instruction, |s| &mut s.instruction)
    }

    /// Attempt to build a new [`Server`] instance from the current builder
    /// state and the provided configuration.
    ///
    /// # Errors
    /// This function returns an error if the builder or configuration are
    /// invalid.
    pub fn try_build(self, config: StreamConfig<M::Config>) -> Result<Server<'a, M>, BuilderError> {
        let Self {
            err,
            account,
            transaction,
            instruction,
            metrics,
            extra: StreamKind(desc_sets, channels),
        } = self;
        let () = err?;

        let StreamConfig {
            grpc: grpc_cfg,
            runtime: runtime_cfg,
        } = config;

        let channels = channels
            .into_iter()
            .map(|(k, v)| (k, v.into_values().collect()))
            .collect();

        let runtime = RuntimeBuilder {
            err: Ok(()),
            account,
            transaction,
            instruction,
            metrics,
            extra: RuntimeKind,
        }
        .try_build(runtime_cfg)?;

        Ok(Server {
            grpc_cfg,
            desc_sets,
            channels,
            runtime,
        })
    }

    /// Build a new [`Server`] instance from the current builder state and the
    /// provided configuration, terminating the current process if an error
    /// occurs.
    #[inline]
    pub fn build(self, config: StreamConfig<M::Config>) -> Server<'a, M> {
        util::handle_fatal_msg(self.try_build(config), "Error building Vixen stream server")
    }
}
