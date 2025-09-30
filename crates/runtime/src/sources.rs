//! Sources for Vixen.
//!
//! A `SourceTrait` is a trait that defines the behavior for data sources that can be used to connect to it and
//! send updates to a channel. This trait is implemented by various modules, including the `yellowstone_grpc` module.

use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use vixen_core::Filters;
use yellowstone_grpc_proto::{geyser::SubscribeUpdate, tonic::Status};

/// # SourceTrait
///
/// This trait defines the behavior for data sources that can be used to connect to it and
/// send updates to a channel. Users can implement this trait to create their own sources.
///
/// The `SourceTrait` provides a standardized way to:
/// * Connect to external data sources
/// * Stream updates through a channel
/// * Configure filters for data processing
/// * Manage source-specific configuration
///
/// # Examples
///
/// ```rust,no_run
/// use async_trait::async_trait;
/// use tokio::sync::mpsc::Sender;
/// use yellowstone_vixen::sources::SourceTrait;
/// use yellowstone_vixen_core::Filters;
/// use yellowstone_grpc_proto::prelude::SubscribeUpdate;
/// use yellowstone_grpc_proto::tonic::Status;
/// use std::error::Error as StdError;
///
/// // Example configuration type
/// #[derive(Debug, clap::Args, serde::Deserialize)]
/// struct MyConfig {
///     endpoint: String,
/// }
///
/// #[derive(Debug)]
/// struct MyCustomSource {
///     filters: Filters,
///     config: MyConfig,
/// }
///
/// #[async_trait]
/// impl SourceTrait for MyCustomSource {
///     type Config = MyConfig;
///
///     fn new(config: Self::Config, filters: Filters) -> Self {
///         MyCustomSource { filters, config }
///     }
///
///     async fn connect(
///         &self,
///         _tx: Sender<Result<SubscribeUpdate, Status>>,
///     ) -> Result<(), yellowstone_vixen::Error> {
///         // Implementation for connecting to your data source
///         // and sending updates through the channel
///         todo!()
///     }
/// }
/// ```
///
/// **Then Vixen clients can use this source by adding it to the runtime**:
///
/// ```rust,no_run
/// // Example of how a custom source would be integrated
/// // let runtime = Runtime::<_, MyCustomSource>::builder()
/// //     .build(config)
/// //     .run_async().await;
/// ```
///
/// ---
/// # Required Methods
///
/// * `connect` - Establishes connection to the data source and streams updates
/// * `new` - Creates a new instance of the source with the given configuration and filters
#[async_trait]
pub trait SourceTrait: std::fmt::Debug + Send + 'static {
    /// The configuration for the source.
    type Config: serde::de::DeserializeOwned + clap::Args + std::fmt::Debug;

    /// Creates a new instance of the source.
    fn new(config: Self::Config, filters: Filters) -> Self;

    /// Connect to the `Source` and send the updates to the `tx` channel.
    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, Status>>,
    ) -> Result<(), crate::Error>;
}
