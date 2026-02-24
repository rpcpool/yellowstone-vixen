//! CoordinatorSource — transparent tap between geyser stream and Vixen Runtime.
//!
//! Implements Vixen's `SourceTrait` to forward raw geyser events to the coordinator
//! while also forwarding transaction events to the Vixen Runtime.
//!

use std::{path::PathBuf, time::Duration};

use async_trait::async_trait;
use futures_util::StreamExt;
use tokio::sync::{mpsc::Sender, oneshot};
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::{
    geyser::{
        subscribe_update::UpdateOneof, SubscribeRequest, SubscribeRequestFilterBlocksMeta,
        SubscribeRequestFilterEntry, SubscribeUpdate,
    },
    tonic::{transport::ClientTlsConfig, Status},
};
use yellowstone_vixen::{
    sources::{SourceExitStatus, SourceTrait},
    Error as VixenError,
};
use yellowstone_vixen_core::{CommitmentLevel, Filters};
use yellowstone_vixen_yellowstone_grpc_source::YellowstoneGrpcConfig;

use crate::{fixtures::FixtureWriter, types::CoordinatorInput};

/// Config for CoordinatorSource.
///
/// Wraps the real source config plus channels for coordinator inputs.
/// The channels are set programmatically after deserialization — they can't
/// come from a config file, hence `#[serde(skip)]` + `#[arg(skip)]`.
#[derive(Debug, serde::Deserialize, clap::Args)]
pub struct CoordinatorSourceConfig {
    #[command(flatten)]
    #[serde(flatten)]
    pub source: YellowstoneGrpcConfig,

    /// Channel to send CoordinatorInput events to the coordinator.
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
    /// - entries (block reconstruction)
    /// - blocks_meta (BlockSummary)
    /// - slot lifecycle (FirstShredReceived, CreatedBank, Completed, Dead).
    fn with_coordinator_subscriptions(self) -> Self;

    /// Set the starting slot for the gRPC stream.
    fn with_from_slot(self, from_slot: Option<u64>) -> Self;

    /// Set commitment to Processed — required for the two-gate flush pattern
    /// so the coordinator sees all transactions before confirmation.
    fn with_commitment_processed(self) -> Self;
}

impl CoordinatorSubscription for SubscribeRequest {
    fn with_coordinator_subscriptions(mut self) -> Self {
        self.entry
            .insert("coordinator".to_string(), SubscribeRequestFilterEntry {});
        self.blocks_meta.insert(
            "coordinator".to_string(),
            SubscribeRequestFilterBlocksMeta {},
        );
        self.slots.entry("coordinator".to_string()).or_default();
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

    fn with_commitment_processed(mut self) -> Self {
        self.commitment = Some(CommitmentLevel::Processed as i32);
        self
    }
}

/// Vixen source that taps the geyser stream for the coordinator.
///
/// On each `SubscribeUpdate`:
/// 1. Forward the raw event to the coordinator (clone for BlockSM-relevant events)
/// 2. Forward Account/Transaction events to the Vixen Runtime (move, no clone)
#[derive(Debug)]
pub struct CoordinatorSource {
    config: CoordinatorSourceConfig,
    filters: Filters,
}

#[async_trait]
impl SourceTrait for CoordinatorSource {
    type Config = CoordinatorSourceConfig;

    fn new(config: Self::Config, filters: Filters) -> Self { Self { config, filters } }

    async fn connect(
        &self,
        tx: Sender<Result<SubscribeUpdate, Status>>,
        status_tx: oneshot::Sender<SourceExitStatus>,
    ) -> Result<(), VixenError> {
        let coordinator_tx = self.config.coordinator_input_tx.as_ref().ok_or_else(|| {
            VixenError::Io(std::io::Error::other(
                "coordinator_input_tx must be set before connect",
            ))
        })?;

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
            .with_from_slot(config.from_slot)
            .with_commitment_processed();

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

        let mut account_seq: u64 = 0;

        let exit_status = 'stream: loop {
            let Some(mut update) = stream.next().await else {
                break SourceExitStatus::StreamEnded;
            };

            match &update {
                Ok(subscribe_update) => {
                    // Capture raw protobuf to fixture file if enabled.
                    if let Some(ref mut writer) = fixture_writer {
                        match writer.write(subscribe_update) {
                            Ok(true) => {},
                            Ok(false) => {
                                tracing::info!(path = ?self.config.fixture_path, "Fixture capture complete");
                                break SourceExitStatus::Completed;
                            },
                            Err(e) => {
                                tracing::error!(?e, "Fixture write failed");
                                break SourceExitStatus::Error(e.to_string());
                            },
                        }
                    }
                }
                _ => {}
            }

            // Repurpose write_version as ingress sequence number for deterministic ordering
            if let Ok(su) = &mut update {
                if let Some(UpdateOneof::Account(acct)) = su.update_oneof.as_mut() {
                    if let Some(info) = acct.account.as_mut() {
                        info.write_version = account_seq;
                        account_seq = account_seq.wrapping_add(1);
                    }
                }
            }

            match &update {
                Ok(subscribe_update) => {
                    // Send lightweight AccountEventSeen for each Account event.
                    if let Some(UpdateOneof::Account(acct)) = &subscribe_update.update_oneof {
                        if coordinator_tx
                            .send(CoordinatorInput::AccountEventSeen { slot: acct.slot })
                            .await
                            .is_err()
                        {
                            tracing::error!("Coordinator input channel closed");
                            break 'stream SourceExitStatus::Error(
                                "Coordinator input channel closed".to_string(),
                            );
                        }
                    }

                    // Forward BlockSM-relevant events to the coordinator.
                    // Entry, Slot, and BlockMeta events are needed for block reconstruction.
                    let is_block_sm_event = matches!(
                        subscribe_update.update_oneof,
                        Some(
                            UpdateOneof::Entry(_)
                                | UpdateOneof::Slot(_)
                                | UpdateOneof::BlockMeta(_)
                        )
                    );

                    if is_block_sm_event {
                        // Clone for coordinator (Account/Transaction events go to Runtime uncloned)
                        if coordinator_tx
                            .send(CoordinatorInput::GeyserUpdate(subscribe_update.clone()))
                            .await
                            .is_err()
                        {
                            tracing::error!("Coordinator input channel closed");
                            break 'stream SourceExitStatus::Error(
                                "Coordinator input channel closed".to_string(),
                            );
                        }
                    }

                    // Only forward Account and Transaction events to the Runtime.
                    if !matches!(
                        subscribe_update.update_oneof,
                        Some(UpdateOneof::Account(_) | UpdateOneof::Transaction(_))
                    ) {
                        continue;
                    }
                },
                Err(status) => {
                    tracing::warn!(code = ?status.code(), message = %status.message(), "Received error status from stream");
                    let code = status.code();
                    let message = status.message().to_string();
                    let _ = tx.send(Err(status.clone())).await;
                    break SourceExitStatus::StreamError { code, message };
                },
            }

            if tx.send(update).await.is_err() {
                tracing::error!("Runtime channel closed");
                break SourceExitStatus::ReceiverDropped;
            }
        };

        tracing::debug!("CoordinatorSource gRPC stream ended");

        let _ = status_tx.send(exit_status);

        Ok(())
    }
}
