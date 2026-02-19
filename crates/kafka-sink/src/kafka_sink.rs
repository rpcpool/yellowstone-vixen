//! Downstream Kafka writer that consumes `ConfirmedSlot<PreparedRecord>` from the coordinator.
//!
//! Replaces `BlockProcessor` — all ordering is handled by the coordinator,
//! so this module just writes records and commits the slot.

use std::{sync::Arc, time::Duration};

use rdkafka::{
    message::OwnedHeaders,
    producer::{FutureProducer, FutureRecord},
};
use tokio::sync::mpsc;
use yellowstone_vixen_block_coordinator::ConfirmedSlot;

use crate::{
    config::KafkaSinkConfig,
    events::{PreparedRecord, RecordHeader, SlotCommitEvent},
};

fn to_kafka_headers(headers: &[RecordHeader]) -> OwnedHeaders {
    headers.iter().fold(OwnedHeaders::new(), |acc, h| {
        acc.insert(rdkafka::message::Header {
            key: h.key,
            value: Some(h.value.as_bytes()),
        })
    })
}

/// Consumes confirmed slots from the coordinator and writes them to Kafka.
///
/// For each confirmed slot:
/// 1. Batch-publish all `PreparedRecord`s to their respective topics
/// 2. Commit the slot to the slots topic (atomic checkpoint)
pub struct ConfirmedSlotSink {
    config: KafkaSinkConfig,
    producer: Arc<FutureProducer>,
}

impl ConfirmedSlotSink {
    const MAX_ATTEMPTS: u32 = 3;

    pub fn new(config: KafkaSinkConfig, producer: Arc<FutureProducer>) -> Self {
        Self { config, producer }
    }

    pub async fn run(
        self,
        mut rx: mpsc::Receiver<ConfirmedSlot<PreparedRecord>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!("ConfirmedSlotSink started, waiting for confirmed slots...");

        while let Some(confirmed) = rx.recv().await {
            for attempt in 1..=Self::MAX_ATTEMPTS {
                match self.write_confirmed_slot(&confirmed).await {
                    Ok(()) => break,
                    Err(e) if attempt < Self::MAX_ATTEMPTS => {
                        tracing::warn!(
                            ?e,
                            slot = confirmed.slot,
                            attempt,
                            "Kafka write failed, retrying"
                        );
                    },
                    Err(e) => {
                        return Err(format!(
                            "Slot {} failed after {attempt} attempts: {e}",
                            confirmed.slot,
                        )
                        .into());
                    },
                }
            }
        }

        tracing::warn!("ConfirmedSlotSink channel closed, shutting down");
        Ok(())
    }

    async fn write_confirmed_slot(
        &self,
        confirmed: &ConfirmedSlot<PreparedRecord>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.batch_publish_records(confirmed.slot, &confirmed.records)
            .await?;
        self.commit_slot_checkpoint(confirmed).await
    }

    /// Publish all records to Kafka, preserving transaction ordering within each topic.
    /// Records arrive pre-sorted by (tx_index, ix_path) from the coordinator.
    ///
    /// `FutureProducer::send()` is async only because it contains an internal sleep-retry
    /// loop for queue-full conditions — the actual enqueue into librdkafka's internal queue
    /// is synchronous.
    ///
    /// Uses `join_all` to await all delivery acks concurrently. Ordering is preserved
    /// because: (1) `join_all` polls futures in index order, (2) each `send()` enqueues
    /// synchronously on its first poll before yielding at the ack-wait, and
    /// (3) `enable.idempotence=true` prevents retry reordering at the network layer.
    ///
    /// `queue_timeout` is set to `Duration::ZERO` so that a queue-full condition
    /// immediately returns an error (failing the slot for retry) rather than sleeping
    /// before enqueue, which could allow a later future to enqueue first.
    ///
    /// Fails the entire slot on any write error so the caller can replay it.
    async fn batch_publish_records(
        &self,
        slot: u64,
        records: &[PreparedRecord],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let futures: Vec<_> = records
            .iter()
            .map(|record| {
                let headers = to_kafka_headers(&record.headers);
                self.producer.send(
                    FutureRecord::to(&record.topic)
                        .payload(&record.payload)
                        .key(&record.key)
                        .headers(headers),
                    Duration::ZERO,
                )
            })
            .collect();

        let results = futures::future::join_all(futures).await;

        for (i, result) in results.into_iter().enumerate() {
            result.map_err(|(e, _)| -> Box<dyn std::error::Error + Send + Sync> {
                tracing::error!(?e, slot, topic = %records[i].topic, "Kafka write failed");
                format!("Kafka write failed for slot {slot}: {e}").into()
            })?;
        }

        Ok(())
    }

    /// Commit a slot checkpoint to the slots topic (atomic marker for resumption).
    async fn commit_slot_checkpoint(
        &self,
        confirmed: &ConfirmedSlot<PreparedRecord>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let slot = confirmed.slot;
        let record_count = confirmed.records.len();
        let decoded_count = confirmed.records.iter().filter(|r| r.is_decoded).count() as u64;

        let event = SlotCommitEvent {
            slot,
            blockhash: confirmed.blockhash.to_string(),
            transaction_count: confirmed.executed_transaction_count,
            decoded_instruction_count: decoded_count,
        };

        let payload = serde_json::to_string(&event)?;
        let slot_key = slot.to_string();

        self.producer
            .send(
                FutureRecord::to(&self.config.slots_topic)
                    .payload(&payload)
                    .key(&slot_key),
                Duration::from_secs(5),
            )
            .await
            .map_err(|(e, _)| -> Box<dyn std::error::Error + Send + Sync> {
                format!("Kafka: failed to commit slot {slot}: {e}").into()
            })?;

        tracing::info!(slot, decoded_count, record_count, "Kafka: committed slot");
        Ok(())
    }
}
