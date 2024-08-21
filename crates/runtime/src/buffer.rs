use std::{net::SocketAddr, pin::pin, sync::Arc};

use futures_util::{Stream, StreamExt};
use tokio::sync::oneshot;
use topograph::{
    executor::{Executor, Nonblock, Tokio},
    prelude::*,
};
use tracing::{error, warn};
use vixen_core::{
    Instruction, InstructionsUpdate, UpdateType, VixenSubscribeUpdate, VixenUpdateOneOf,
};
use warp::Filter;
use yellowstone_grpc_proto::{
    geyser::{subscribe_update::UpdateOneof, SubscribeUpdate},
    tonic::Status,
};
use yellowstone_vixen_core::{AccountUpdate, TransactionUpdate};

use crate::{
    handler::DynHandlerPack,
    metrics::{Metrics, MetricsBackend},
    yellowstone, Error, HandlerManagers,
};

#[derive(Default, Debug, Clone, Copy, clap::Args, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BufferOpts {
    pub jobs: Option<usize>,
}

pub struct Buffer(oneshot::Receiver<crate::Error>);

impl Buffer {
    // TODO: use never
    #[inline]
    pub async fn wait_for_stop(self) -> Result<std::convert::Infallible, crate::Error> {
        self.0
            .await
            .map_err(|_| crate::Error::ClientHangup)
            .and_then(Err)
    }
}

pub fn run_yellowstone<
    I,
    T,
    S: Stream<Item = Result<SubscribeUpdate, Status>> + 'static,
    A: DynHandlerPack<AccountUpdate> + Send + Sync + 'static,
    X: DynHandlerPack<InstructionsUpdate> + Send + Sync + 'static,
    M: MetricsBackend,
>(
    opts: BufferOpts,
    client: yellowstone::YellowstoneStream<I, T, S>,
    manager: HandlerManagers<A, X>,
    metrics: Metrics<M>,
) -> Buffer {
    let BufferOpts { jobs } = opts;

    let manager = Arc::new(manager);
    let metrics = Arc::new(metrics);
    #[cfg(feature = "prometheus")]
    let metrics_clone = Arc::clone(&metrics);
    #[cfg(feature = "prometheus")]
    tokio::task::spawn_local(async {
        use prometheus::{Encoder, TextEncoder};
        let route = warp::path("metrics").map(move || {
            let encoder = TextEncoder::new();
            let response = metrics_clone
                .gather_metrics_data()
                .unwrap_or(String::from("no metrics data available"));
            warp::reply::with_header(response, "Content-Type", encoder.format_type())
        });

        // Serve the route
        println!("Prometheus Metrics server running on port 3030");
        let addr: SocketAddr = ([0, 0, 0, 0], 3030).into();
        warp::serve(route).run(addr).await;
    });

    let metrics_clone = Arc::clone(&metrics);

    let exec = Executor::builder(Nonblock(Tokio))
        .num_threads(jobs)
        .build(move |update, _| {
            let manager = Arc::clone(&manager);
            let metrics = Arc::clone(&metrics_clone);
            async move {
                let VixenSubscribeUpdate {
                    filters,
                    update_oneof,
                } = update;
                let Some(update) = update_oneof else { return };

                match update {
                    // UpdateOneof::Account(a) => {
                    //     manager
                    //         .account
                    //         .get_handlers(&filters)
                    //         .run(&a, &metrics)
                    //         .await;
                    // },
                    // UpdateOneof::Transaction(t) => {
                    //     match Instructions::try_from(&t) {
                    //         // Ok(ix) => {
                    //         //     manager
                    //         //         .instruction
                    //         //         .get_handlers(&filters)
                    //         //         .run(&ix, &metrics)
                    //         //         .await;
                    //         // },
                    //         // Err(e) => {
                    //         //     error!(%e, "Error parsing transaction update");
                    //         // },
                    //         Ok(ix) => {
                    //             manager
                    //                 .instruction
                    //                 .get_handlers(&filters)
                    //                 .run(&ix, &metrics)
                    //                 .await;
                    //         },
                    //         Err(e) => {
                    //             error!(%e, "Error parsing transaction update");
                    //         },
                    //     }

                    //     // manager
                    //     //     .transaction
                    //     //     .get_handlers(&filters)
                    //     //     .run(&t, &metrics)
                    //     //     .await;
                    // },
                    // var => warn!(?var, "Unknown update variant"),
                    VixenUpdateOneOf::Account(update) => {
                        manager
                            .account
                            .get_handlers(&filters)
                            .run(&update, &metrics)
                            .await;
                    },
                    VixenUpdateOneOf::Instructions(ixs) => {
                        manager
                            .instructions
                            .get_handlers(&filters)
                            .run(&ixs, &metrics)
                            .await;
                    },
                }
            }
        })
        .unwrap_or_else(|i| match i {});

    let (tx, rx) = oneshot::channel();

    tokio::task::spawn_local(async move {
        let mut stream = pin!(client.stream);
        while let Some(update) = stream.next().await {
            match update {
                //TODO add conversions here
                Ok(u) => {
                    let vixen_update = VixenSubscribeUpdate::try_from(u);
                    match vixen_update {
                        Ok(vixen_update) => {
                            if let Some(ty) = UpdateType::get(&vixen_update.update_oneof) {
                                metrics.inc_received(ty);
                            }
                            exec.push(vixen_update);
                        },
                        Err(e) => {
                            warn!(%e, "Error converting update to vixen update");
                        },
                    }
                    // if let Some(ty) = UpdateType::get(&u.update_oneof) {
                    //     metrics.inc_received(ty);
                    // }
                    // exec.push(u);
                },
                Err(e) => {
                    tx.send(e.into()).unwrap_or_else(|err| {
                        warn!(%err, "Yellowstone stream returned an error after stop requested");
                    });
                    return;
                },
            }
        }

        tx.send(crate::Error::ServerHangup).unwrap_or_else(|_| {
            warn!("Yellowstone client and server both hung up");
        });
    });

    Buffer(rx)
}
