use std::collections::HashMap;

use crate::{config::YellowstoneConfig, datasources::DataSource, yellowstone};
use async_trait::async_trait;
use futures_util::StreamExt;
use tokio::sync::mpsc::Sender;
use vixen_core::{Filters, Parser};
use yellowstone_grpc_proto::{
    geyser::{CommitmentLevel, SubscribeUpdate},
    tonic::Status,
};

#[derive(Debug)]
pub struct YellowstoneGrpcDataSource {
    config: YellowstoneConfig,
    filters: Option<Filters<'static>>,
    commitment_filter: Option<CommitmentLevel>,
}

impl YellowstoneGrpcDataSource {
    pub fn new(config: YellowstoneConfig) -> Self {
        Self {
            config,
            filters: None,
            commitment_filter: None,
        }
    }

    // TODO: fix lifetimes & Filters impl (maybe we need DerefMut impl for Filters)
    pub fn add_filter<T: Parser>(mut self, parser: T) -> Self {
        match &mut self.filters {
            Some(_filters) => {
                // filters.insert("new_filter", parser.prefilter());
            },
            None => {
                let mut filters = HashMap::new();
                filters.insert("new_filter", parser.prefilter());
                self.filters = Some(Filters::new(filters));
            },
        }

        self
    }
}

#[async_trait]
impl DataSource for YellowstoneGrpcDataSource {
    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, Status>>,
    ) -> Result<(), crate::Error> {
        let filters = match self.filters.clone() {
            Some(filters) => filters,
            None => return Err(crate::Error::ConfigError),
        };

        let client =
            yellowstone::connect(self.config.clone(), filters, self.commitment_filter).await?;

        let mut stream = std::pin::pin!(client.stream);

        while let Some(update) = stream.next().await {
            let res = tx.send(update).await;
            if res.is_err() {
                tracing::warn!("Failed to send update to buffer");
            }
        }

        Ok(())
    }

    fn has_filters(&self) -> bool {
        self.filters.is_some()
    }

    fn filters(&mut self, filters: Filters<'static>) {
        if !self.has_filters() {
            self.filters = Some(filters);
        }
    }

    fn commitment_filter(&mut self, commitment_filter: Option<CommitmentLevel>) {
        self.commitment_filter = commitment_filter;
    }
}
