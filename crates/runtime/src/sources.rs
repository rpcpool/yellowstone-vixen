//! Sources for Vixen.
//!
//! A `Source` is a trait that defines the behavior for data `Sources` that can be used to connect to it and
//!  send the updates to a channel.
//!
//! The `Source` trait is implemented by the `yellowstone_grpc` module.

use std::{fmt::Debug, marker::PhantomData};

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
/// struct MyCustomSource {
///     filters: Option<Filters>,
/// }
///
/// #[async_trait]
/// impl Source for MyCustomSource {
///     async fn connect(
///         &self,
///         tx: Sender<Result<SubscribeUpdate, Status>>,
///         raw_config: toml::Value,
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
///     fn get_filters(&self) -> &Option<Filters> {
///         &self.filters
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
#[async_trait]
pub trait SourceTrait: std::fmt::Debug + Send + 'static {
    type Config: serde::de::DeserializeOwned;

    fn name() -> String;

    fn new(config: Self::Config, filters: Filters) -> Self;

    /// Connect to the `Source` and send the updates to the `tx` channel.
    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, Status>>,
    ) -> Result<(), crate::Error>;
}

#[derive(Debug)]
pub struct Source<S>
where S: SourceTrait + Debug + Send + Sync + 'static
{
    _source: PhantomData<S>,
}

impl<S> Source<S>
where S: SourceTrait + Debug + Send + Sync + 'static
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            _source: PhantomData,
        }
    }
}

impl<S> Default for Source<S>
where S: SourceTrait + Debug + Send + Sync + 'static
{
    fn default() -> Self { Self::new() }
}

pub trait DynSource: std::fmt::Debug {
    fn name(&self) -> String;
    fn connect(
        &self,
        config: toml::Value,
        filters: Filters,
        tx: Sender<Result<SubscribeUpdate, Status>>,
    ) -> tokio::task::JoinHandle<Result<(), crate::Error>>;
}

impl<S> DynSource for Source<S>
where S: SourceTrait + Debug + Send + Sync + 'static
{
    fn name(&self) -> String { S::name() }

    fn connect(
        &self,
        config: toml::Value,
        filters: Filters,
        tx: Sender<Result<SubscribeUpdate, Status>>,
    ) -> tokio::task::JoinHandle<Result<(), crate::Error>> {
        let config: S::Config = serde::Deserialize::deserialize(config)
            .unwrap_or_else(|_| panic!("Failed to deserialize config for source {}", self.name()));
        let source = S::new(config, filters);

        tokio::spawn(async move { source.connect(tx).await })
    }
}
