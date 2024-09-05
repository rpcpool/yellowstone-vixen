use std::fmt::Debug;

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
    grpc::{Channels, GrpcHandler},
    Server,
};
use crate::{
    builder::{Builder, BuilderKind, RuntimeBuilder, RuntimeKind},
    handler::{BoxPipeline, Pipeline},
    metrics::MetricsFactory,
    util,
};

#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("Duplicate program ID {0} registered")]
    DuplicateId(Pubkey),
    #[error("Error building Vixen runtime")]
    Runtime(#[from] crate::builder::BuilderError),
}

#[derive(Debug, Default)]
pub struct StreamKind(Channels);
pub type StreamBuilder<M> = Builder<StreamKind, M>;

impl BuilderKind for StreamKind {
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

impl<M: MetricsFactory> StreamBuilder<M> {
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

            match s.extra.0.entry(parser.program_id()) {
                Entry::Vacant(v) => {
                    v.insert(rx);
                },
                Entry::Occupied(_) => return Err(BuilderError::DuplicateId(parser.program_id())),
            }
            f(s).push(wrap_parser(parser, tx));
            Ok(())
        })
    }

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

    pub fn try_build(self, config: StreamConfig<M::Config>) -> Result<Server<M>, BuilderError> {
        let Self {
            err,
            account,
            transaction,
            instruction,
            metrics,
            extra: StreamKind(channels),
        } = self;
        let () = err?;

        let StreamConfig {
            grpc: grpc_cfg,
            runtime: runtime_cfg,
        } = config;

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
            channels,
            runtime,
        })
    }

    #[inline]
    pub fn build(self, config: StreamConfig<M::Config>) -> Server<M> {
        util::handle_fatal_msg(self.try_build(config), "Error building Vixen stream server")
    }
}
