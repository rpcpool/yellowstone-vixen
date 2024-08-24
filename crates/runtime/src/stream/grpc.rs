use std::{collections::HashMap, future::Future, pin::Pin};

use tokio::{
    sync::broadcast,
    task::{JoinError, JoinHandle},
};
use vixen_core::Pubkey;
use yellowstone_vixen_proto::{
    prost::{Message, Name},
    prost_types::Any,
    stream::{
        self,
        vixen_server::{Vixen, VixenServer},
        SubscribeRequest, SubscribeUpdate,
    },
    tonic::{self, transport, Request, Response, Status},
    tonic_reflection,
};

use super::config::GrpcConfig;
use crate::{stop, Handler, HandlerResult};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("gRPC transport error")]
    Transport(#[from] transport::Error),
    #[error("Server task panicked")]
    Panic(#[from] JoinError),
}

#[derive(Debug)]
pub struct GrpcHandler(pub(super) broadcast::Sender<Any>);

impl<T: Message + Name + Sync> Handler<T> for GrpcHandler {
    async fn handle(&self, value: &T) -> HandlerResult<()> {
        self.0.send(Any::from_msg(value)?).ok();
        Ok(())
    }
}

pub type Channels = HashMap<Pubkey, broadcast::Receiver<Any>>;

pub(super) struct Service(Channels);

#[tonic::async_trait]
impl Vixen for Service {
    // TODO: Box<dyn Stream> is tacky but any alternatives are not worth thinking about right now
    type SubscribeStream = Pin<
        Box<
            dyn futures_util::Stream<Item = Result<SubscribeUpdate, Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let pubkey =
            request
                .into_inner()
                .program
                .parse()
                .map_err(|e: vixen_core::PubkeyFromStrError| {
                    Status::new(tonic::Code::InvalidArgument, e.to_string())
                })?;

        let stream = futures_util::stream::unfold(
            self.0
                .get(&pubkey)
                .map_or_else(|| broadcast::channel(1).1, broadcast::Receiver::resubscribe),
            |mut rx| async move {
                for _ in 0..8 {
                    match rx.recv().await {
                        Ok(m) => return Some((Ok(SubscribeUpdate { parsed: Some(m) }), rx)),
                        Err(broadcast::error::RecvError::Closed) => return None,
                        Err(broadcast::error::RecvError::Lagged(_)) => (),
                    }
                }
                panic!("gRPC channel lagged too many times!")
            },
        );

        Ok(Response::new(Box::pin(stream)))
    }
}

#[derive(Debug)]
pub struct Server(stop::StopTx, JoinHandle<Result<(), transport::Error>>);

#[inline]
fn flatten_error<T>(res: Result<Result<T, transport::Error>, JoinError>) -> Result<T, Error> {
    Ok(res??)
}

impl Future for Server {
    type Output = Result<(), Error>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Pin::new(&mut self.1).poll(cx).map(flatten_error)
    }
}

impl Server {
    pub fn run(config: GrpcConfig, channels: HashMap<Pubkey, broadcast::Receiver<Any>>) -> Self {
        let GrpcConfig { address } = config;

        let reflection = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(stream::DESCRIPTOR_SET)
            .build()
            .unwrap();

        let (stop, rx) = stop::channel();
        Self(
            stop,
            tokio::task::spawn(async move {
                transport::Server::builder()
                    .add_service(reflection)
                    .add_service(VixenServer::new(Service(channels)))
                    .serve_with_shutdown(address, rx.as_unit())
                    .await
            }),
        )
    }

    pub async fn stop(self) -> Result<(), Error> {
        self.0.maybe_send();
        flatten_error(self.1.await)
    }
}
