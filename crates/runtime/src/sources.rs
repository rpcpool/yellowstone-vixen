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

use crate::config::YellowstoneConfig;

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
/// use yellowstone_vixen::config::YellowstoneConfig;
/// use vixen_core::Filters;
///
/// #[derive(Debug)]
/// struct MyCustomSource {
///     filters: Option<Filters>,
///     config: Option<YellowstoneConfig>,
/// }
///
/// #[async_trait]
/// impl Source for MyCustomSource {
///     async fn connect(
///         &self,
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
///
///     fn set_filters_unchecked(&mut self, filters: Filters) {
///         self.filters = Some(filters);
///     }
///
///     fn set_config_unchecked(&mut self, config: YellowstoneConfig) {
///         self.config = Some(config);
///     }
///
///     fn get_filters(&self) -> &Option<Filters> {
///         &self.filters
///     }
///
///     fn get_config(&self) -> Option<YellowstoneConfig> {
///         self.config.clone()
///     }
/// }
/// ```
///
/// **Then Vixen clients can use this source by adding it to the runtime**:
///
/// ```rust
/// vixen::Runtime::builder()
///     .source(MyCustomSource::new())
///     .build(config)
///     .run();
/// ```
///
/// ---
/// # Required Methods
///
/// * `connect` - Establishes connection to the data source and streams updates
/// * `name` - Returns a unique identifier for the source
/// * `set_filters_unchecked` - Sets filters for data processing
/// * `set_config_unchecked` - Sets source-specific configuration
/// * `get_filters` - Retrieves current filters
/// * `get_config` - Retrieves current configuration
#[async_trait]
pub trait Source: std::fmt::Debug + Send + 'static {
    /// Connect to the `Source` and send the updates to the `tx` channel.
    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, Status>>,
    ) -> Result<(), crate::Error>;

    /// Should return the name of the `Source`.
    fn name(&self) -> String;

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
