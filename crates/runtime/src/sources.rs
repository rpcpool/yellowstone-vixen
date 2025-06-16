//! Sources for Vixen.
//!
//! A `Source` is a trait that defines the behavior for data `Sources` that can be used to connect to it and
//!  send the updates to a channel.
//!
//! The `Source` trait is implemented by the `yellowstone_grpc` module.

use async_trait::async_trait;
use tokio::{sync::mpsc::Sender, task::JoinSet};
use vixen_core::Filters;
use yellowstone_grpc_proto::{geyser::SubscribeUpdate, tonic::Status};

use crate::config::YellowstoneConfig;

/// This trait defines the behavior for data `Sources` that can be used to connect to it and
///  send the updates to a channel.
#[async_trait]
pub trait Source: std::fmt::Debug + Send + 'static {
    /// Connect to the `Source` and send the updates to the `tx` channel.
    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, Status>>,
    ) -> Result<JoinSet<()>, crate::Error>;

    /// Set the filters to use for the `Source`. In general you define the implementations here but then the method
    ///  the users call is `filters`, which has a check to not override the filters if they are already set on top of this method.
    fn set_filters_unchecked(&mut self, filters: Filters);

    /// Set the config to use for the `Source`. In general you define the implementations here but then the method
    ///  the users call is `config`, which has a check to not override the config if they are already set on top of this method.
    fn set_config_unchecked(&mut self, config: YellowstoneConfig);

    /// Should return the filters to use for the `Source`.
    fn get_filters(&self) -> &Option<Filters>;

    /// Should return the config to use for the `Source`.
    fn get_config(&self) -> Option<YellowstoneConfig>;

    /// Optional method, the default behavior is only set the filters with `set_filters` if no filters
    ///  are already set.
    fn filters(&mut self, filters: Filters) {
        if self.get_filters().is_none() {
            self.set_filters_unchecked(filters);
        }
    }

    /// Optional method, the default behavior is only set the config with `set_config` if no config
    ///  is already set.
    fn config(&mut self, config: YellowstoneConfig) {
        if self.get_config().is_none() {
            self.set_config_unchecked(config);
        }
    }
}
