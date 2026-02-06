//! CoordinatorSource — transparent tap between geyser stream and Vixen Runtime.
//!
//! Implements Vixen's `SourceTrait` to extract lightweight BlockSM inputs
//! into a side channel while forwarding transaction events to the Runtime.
//!

use std::{path::PathBuf, time::Duration};

use async_trait::async_trait;
use futures_util::StreamExt;
use tokio::sync::mpsc::Sender;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::{
    geyser::{
        subscribe_update::UpdateOneof, SubscribeRequest, SubscribeRequestFilterBlocksMeta,
        SubscribeRequestFilterEntry, SubscribeUpdate,
    },
    tonic::{transport::ClientTlsConfig, Status},
};
use yellowstone_vixen::{sources::SourceTrait, Error as VixenError};
use yellowstone_vixen_core::Filters;
use yellowstone_vixen_yellowstone_grpc_source::YellowstoneGrpcConfig;

use crate::{extract_coordinator_inputs, fixtures::FixtureWriter, CoordinatorInput};

/// Config for CoordinatorSource.
///
/// Wraps the real source config plus a channel for BlockSM inputs.
/// The channel is set programmatically after deserialization — it can't
/// come from a config file, hence `#[serde(skip)]` + `#[arg(skip)]`.
#[derive(Debug, serde::Deserialize, clap::Args)]
pub struct CoordinatorSourceConfig {
    #[command(flatten)]
    #[serde(flatten)]
    pub source: YellowstoneGrpcConfig,

    /// Channel to send BlockSM inputs to the coordinator.
    #[serde(skip)]
    #[arg(skip)]
    pub coordinator_input_tx: Option<Sender<CoordinatorInput>>,

    /// Path to write captured fixture data (length-delimited protobuf).
    #[serde(skip)]
    #[arg(skip)]
    pub fixture_path: Option<PathBuf>,

    /// Number of BlockMeta messages to capture before stopping.
    #[serde(skip)]
    #[arg(skip)]
    pub fixture_slots: Option<usize>,
}

trait CoordinatorSubscription {
    /// Add all subscriptions required by the BlockSM coordinator:
    /// entries (block reconstruction), blocks_meta (BlockSummary), and
    /// slot lifecycle (FirstShredReceived, CreatedBank, Completed, Dead).
    fn with_coordinator_subscriptions(self) -> Self;

    /// Set the starting slot for the gRPC stream.
    fn with_from_slot(self, from_slot: Option<u64>) -> Self;
}

impl CoordinatorSubscription for SubscribeRequest {
    fn with_coordinator_subscriptions(mut self) -> Self {
        self.entry
            .insert("coordinator".to_string(), SubscribeRequestFilterEntry {});
        self.blocks_meta.insert(
            "coordinator".to_string(),
            SubscribeRequestFilterBlocksMeta {},
        );
        self.slots
            .entry("coordinator".to_string())
            .or_default();
        for filter in self.slots.values_mut() {
            filter.interslot_updates = Some(true);
            filter.filter_by_commitment = None;
        }
        self
    }

    fn with_from_slot(mut self, from_slot: Option<u64>) -> Self {
        self.from_slot = from_slot.or(self.from_slot);
        self
    }
}

/// Vixen source that taps the geyser stream for BlockSM inputs.
///
/// On each `SubscribeUpdate`:
/// 1. Extract lightweight BlockSM inputs (integers + 32-byte hash, no large allocs)
/// 2. Send them to the coordinator via the side channel
/// 3. Forward the full event to the Vixen Runtime (move, no clone)
#[derive(Debug)]
pub struct CoordinatorSource {
    config: CoordinatorSourceConfig,
    filters: Filters,
}

#[async_trait]
impl SourceTrait for CoordinatorSource {
    type Config = CoordinatorSourceConfig;

    fn new(config: Self::Config, filters: Filters) -> Self { Self { config, filters } }

    async fn connect(&self, tx: Sender<Result<SubscribeUpdate, Status>>) -> Result<(), VixenError> {
        let coordinator_tx = self
            .config
            .coordinator_input_tx
            .as_ref()
            .expect("coordinator_input_tx must be set before connect");

        let mut fixture_writer = match (&self.config.fixture_path, self.config.fixture_slots) {
            (Some(path), Some(slots)) => {
                tracing::info!(?path, slots, "Fixture capture enabled");
                Some(FixtureWriter::new(path, slots).map_err(VixenError::Io)?)
            },
            _ => None,
        };

        let config = &self.config.source;
        let timeout = Duration::from_secs(config.timeout);

        let mut client = GeyserGrpcClient::build_from_shared(config.endpoint.clone())?
            .x_token(config.x_token.clone())?
            .max_decoding_message_size(config.max_decoding_message_size.unwrap_or(usize::MAX))
            .accept_compressed(config.accept_compression.unwrap_or_default().into())
            .connect_timeout(timeout)
            .timeout(timeout)
            .tls_config(ClientTlsConfig::new().with_native_roots())?
            .connect()
            .await?;

        let subscribe_request = SubscribeRequest::from(self.filters.clone())
            .with_coordinator_subscriptions()
            .with_from_slot(config.from_slot);

        tracing::info!(
            has_transactions = !subscribe_request.transactions.is_empty(),
            has_blocks_meta = !subscribe_request.blocks_meta.is_empty(),
            has_slots = !subscribe_request.slots.is_empty(),
            has_entries = !subscribe_request.entry.is_empty(),
            from_slot = ?subscribe_request.from_slot,
            commitment = ?subscribe_request.commitment,
            "CoordinatorSource subscribing to gRPC stream"
        );

        let (_sub_tx, stream) = client
            .subscribe_with_request(Some(subscribe_request))
            .await?;

        let mut stream = std::pin::pin!(stream);

        tracing::info!("CoordinatorSource gRPC stream started");

        while let Some(update) = stream.next().await {
            if let Ok(ref subscribe_update) = update {
                // Capture raw protobuf to fixture file if enabled.
                if let Some(ref mut writer) = fixture_writer {
                    match writer.write(subscribe_update) {
                        Ok(true) => {},
                        Ok(false) => {
                            tracing::info!(path = ?self.config.fixture_path, "Fixture capture complete");
                            return Ok(());
                        },
                        Err(e) => {
                            tracing::error!(?e, "Fixture write failed");
                            return Ok(());
                        },
                    }
                }

                // Lightweight extraction: integers + 32-byte hash only.
                let inputs: Vec<CoordinatorInput> = extract_coordinator_inputs(subscribe_update);
                if !inputs.is_empty() {
                    tracing::debug!(count = inputs.len(), "Extracted coordinator inputs");
                }
                for input in inputs {
                    if coordinator_tx.send(input).await.is_err() {
                        tracing::error!("Coordinator input channel closed");
                        return Ok(());
                    }
                }

                // Only forward to the Runtime Account and Transactions events.
                if !matches!(
                    subscribe_update.update_oneof,
                    Some(UpdateOneof::Account(_) | UpdateOneof::Transaction(_))
                ) {
                    continue;
                }
            }

            if tx.send(update).await.is_err() {
                tracing::error!("Runtime channel closed");
                break;
            }
        }

        tracing::debug!("CoordinatorSource gRPC stream ended");

        Ok(())
    }
}
