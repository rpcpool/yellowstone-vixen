//! Downstream Kafka writer that consumes `ConfirmedSlot<PreparedRecord>` from the coordinator.
//!
//! Replaces `BlockProcessor` — all ordering is handled by the coordinator,
//! so this module just writes records and commits the slot.

use std::{sync::Arc, time::Duration};

use futures::future::join_all;
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
    pub fn new(config: KafkaSinkConfig, producer: Arc<FutureProducer>) -> Self {
        Self { config, producer }
    }

    pub async fn run(self, mut rx: mpsc::Receiver<ConfirmedSlot<PreparedRecord>>) {
        tracing::info!("ConfirmedSlotSink started, waiting for confirmed slots...");

        while let Some(confirmed) = rx.recv().await {
            if let Err(e) = self.write_confirmed_slot(&confirmed).await {
                tracing::error!(
                    ?e,
                    slot = confirmed.slot,
                    "Error writing confirmed slot to Kafka"
                );
            }
        }

        tracing::warn!("ConfirmedSlotSink channel closed, shutting down");
    }

    async fn write_confirmed_slot(
        &self,
        confirmed: &ConfirmedSlot<PreparedRecord>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.batch_publish_records(confirmed.slot, &confirmed.records)
            .await;
        self.commit_slot_checkpoint(confirmed).await
    }

    /// Publish all records to their respective Kafka topics in parallel.
    async fn batch_publish_records(&self, slot: u64, records: &[PreparedRecord]) {
        let futures: Vec<_> = records
            .iter()
            .map(|r| {
                let headers = to_kafka_headers(&r.headers);
                self.producer.send(
                    FutureRecord::to(&r.topic)
                        .payload(&r.payload)
                        .key(&r.key)
                        .headers(headers),
                    Duration::from_secs(5),
                )
            })
            .collect();

        let results = join_all(futures).await;

        let failure_count = results
            .into_iter()
            .zip(records.iter())
            .filter(|(result, record)| {
                if let Err((e, _)) = result {
                    tracing::error!(?e, slot, topic = %record.topic, "Kafka write failed");
                    true
                } else {
                    false
                }
            })
            .count();

        if failure_count > 0 {
            tracing::warn!(
                slot,
                failure_count,
                record_count = records.len(),
                "Partial write failures"
            );
        }
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

        match self
            .producer
            .send(
                FutureRecord::to(&self.config.slots_topic)
                    .payload(&payload)
                    .key(&slot_key),
                Duration::from_secs(5),
            )
            .await
        {
            Ok(_) => tracing::info!(slot, decoded_count, record_count, "Kafka: committed slot"),
            Err((e, _)) => tracing::error!(?e, slot, "Kafka: failed to commit slot"),
        }

        Ok(())
    }
}
