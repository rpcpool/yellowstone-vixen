//! Sources for Vixen.
//!
//! A `Source` is a trait that defines the behavior for data `Sources` that can be used to connect to it and
//!  send the updates to a channel.
//!
//! The `Source` trait is implemented by the `yellowstone_grpc` module.

use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use vixen_core::Filters;
use yellowstone_grpc_proto::{geyser::SubscribeUpdate, tonic::Status};

/// # Sources trait
///
/// This trait defines the behavior for data `Sources` that can be used to connect to it and
///  send the updates to a channel.
///
/// Users can implement this trait to create their own `Source`s.
///
/// The `Source` trait provides a standardized way to:
/// * Connect to external data sources
/// * Stream updates through a channel
/// * Configure filters for data processing
/// * Manage source-specific configuration
///
/// # Examples
///
/// ```rust
/// use async_trait::async_trait;
/// use tokio::sync::mpsc::Sender;
/// use yellowstone_vixen::sources::Source;
/// use vixen_core::Filters;
///
/// #[derive(Debug)]
/// struct MyCustomSource;
///
/// #[async_trait]
/// impl Source for MyCustomSource {
///     type Config = MyConfig;
///
///     async fn connect(
///         &self,
///         config: Self::Config,
///         filters: Filters,
///         tx: Sender<Result<SubscribeUpdate, Status>>,
///     ) -> Result<(), crate::Error> {
///         // Implementation for connecting to your data source
///         // and sending updates through the channel
///         todo!()
///     }
///
///     fn name(&self) -> String {
///         "my-custom-source".to_string()
///     }
/// }
/// ```
///
/// **Then Vixen clients can use this source by adding it to the runtime**:
///
/// ```rust
/// vixen::Runtime::builder()
///     .source(MyCustomSource)
///     .build(config)
///     .run();
/// ```
///
/// ---
/// # Required Methods
///
/// * `connect` - Establishes connection to the data source and streams updates
/// * `name` - Returns a unique identifier for the source
#[async_trait]
pub trait SourceTrait: std::fmt::Debug + Send + 'static {
    /// The configuration type for the source.
    type Config: serde::de::DeserializeOwned
        + Default
        + std::fmt::Debug
        + Clone
        + Send
        + Sync
        + 'static;

    /// The name of the source.
    fn name() -> String;

    /// Connect to the `Source` and send the updates to the `tx` channel.
    async fn connect(
        config: Self::Config,
        filters: Filters,
        tx: Sender<Result<SubscribeUpdate, Status>>,
    ) -> Result<(), crate::Error>;
}
