use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use vixen_core::Filters;
use yellowstone_grpc_proto::{geyser::SubscribeUpdate, tonic::Status};

pub mod yellowstone_grpc;

#[async_trait]
pub trait DataSource: std::fmt::Debug + Send + 'static {
    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, Status>>,
    ) -> Result<(), crate::Error>;

    fn has_filters(&self) -> bool;

    /// Meant to be used as a fallback for Filters, if we don't have custom filters set for the datasource,
    ///  we use the Vixen runtime default filters.
    fn filters(&mut self, filters: Filters<'static>);

    fn commitment_filter(
        &mut self,
        commitment_filter: Option<yellowstone_grpc_proto::geyser::CommitmentLevel>,
    );
}
