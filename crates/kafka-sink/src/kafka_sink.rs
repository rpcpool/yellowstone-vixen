//! Downstream Kafka writers that consume slots from the coordinator.
//!
//! `TransactionSlotSink` — receives `InstructionSlot<PreparedRecord>`, writes to `transaction.slots`.
//! `AccountSink` — two modes:
//!   - `run_buffered`: receives `AccountSlot<PreparedRecord>` from coordinator, writes to `account.slots`.
//!   - `run_passthrough`: receives `AccountMsg` directly, produces to Kafka immediately.

use std::{sync::Arc, time::Duration};

use rdkafka::{
    message::OwnedHeaders,
    producer::{FutureProducer, FutureRecord},
};
use tokio::sync::mpsc;
use yellowstone_vixen_block_coordinator::{AccountSlot, InstructionSlot};

use crate::{
    config::KafkaSinkConfig,
    events::{AccountSlotCommitEvent, PreparedRecord, RecordHeader, RecordKind, TransactionSlotCommitEvent},
};

fn to_kafka_headers(headers: &[RecordHeader]) -> OwnedHeaders {
    headers.iter().fold(OwnedHeaders::new(), |acc, h| {
        acc.insert(rdkafka::message::Header {
            key: h.key,
            value: Some(h.value.as_bytes()),
        })
    })
}

/// Consumes instruction slots from the coordinator and writes them to Kafka.
pub struct TransactionSlotSink {
    config: KafkaSinkConfig,
    producer: Arc<FutureProducer>,
}

impl TransactionSlotSink {
    const MAX_ATTEMPTS: u32 = 3;

    pub fn new(config: KafkaSinkConfig, producer: Arc<FutureProducer>) -> Self {
        Self { config, producer }
    }

    pub async fn run(
        self,
        mut rx: mpsc::Receiver<InstructionSlot<PreparedRecord>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!("TransactionSlotSink started, waiting for instruction slots...");

        while let Some(ix_slot) = rx.recv().await {
            for attempt in 1..=Self::MAX_ATTEMPTS {
                match self.write_instruction_slot(&ix_slot).await {
                    Ok(()) => break,
                    Err(e) if attempt < Self::MAX_ATTEMPTS => {
                        tracing::warn!(
                            ?e,
                            slot = ix_slot.slot,
                            attempt,
                            "Kafka write failed, retrying"
                        );
                    },
                    Err(e) => {
                        return Err(format!(
                            "Slot {} failed after {attempt} attempts: {e}",
                            ix_slot.slot,
                        )
                        .into());
                    },
                }
            }
        }

        tracing::warn!("TransactionSlotSink channel closed, shutting down");
        Ok(())
    }

    async fn write_instruction_slot(
        &self,
        ix_slot: &InstructionSlot<PreparedRecord>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.batch_publish_records(ix_slot.slot, &ix_slot.records)
            .await?;
        self.commit_transaction_slot_checkpoint(ix_slot).await
    }

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
                    // Do not retry when a send fail. Returns an RDKafkaErrorCode::QueueFull error instead so the slot retries, ensuring we never lose the ordering
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

    async fn commit_transaction_slot_checkpoint(
        &self,
        ix_slot: &InstructionSlot<PreparedRecord>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let slot = ix_slot.slot;
        let record_count = ix_slot.records.len();
        let decoded_instruction_count = ix_slot
            .records
            .iter()
            .filter(|r| r.is_decoded && r.kind == RecordKind::Instruction)
            .count() as u64;

        let event = TransactionSlotCommitEvent {
            slot,
            blockhash: ix_slot.blockhash.to_string(),
            transaction_count: ix_slot.executed_transaction_count,
            decoded_instruction_count,
            filtered_instruction_count: ix_slot.filtered_instruction_count,
            failed_instruction_count: ix_slot.failed_instruction_count,
            transaction_status_failed_count: ix_slot.transaction_status_failed_count,
            transaction_status_succeeded_count: ix_slot.transaction_status_succeeded_count,
        };

        let payload = serde_json::to_string(&event)?;
        let slot_key = slot.to_string();

        self.producer
            .send(
                FutureRecord::to(&self.config.transaction_slots_topic)
                    .payload(&payload)
                    .key(&slot_key),
                Duration::from_secs(5),
            )
            .await
            .map_err(|(e, _)| -> Box<dyn std::error::Error + Send + Sync> {
                format!("Kafka: failed to commit slot {slot}: {e}").into()
            })?;

        tracing::info!(
            slot,
            decoded_instruction_count,
            filtered_instruction_count = ix_slot.filtered_instruction_count,
            failed_instruction_count = ix_slot.failed_instruction_count,
            transaction_status_failed_count = ix_slot.transaction_status_failed_count,
            transaction_status_succeeded_count = ix_slot.transaction_status_succeeded_count,
            record_count,
            "Kafka: committed instruction slot"
        );
        Ok(())
    }
}

/// Messages from the passthrough account pipeline.
pub enum AccountMsg {
    Record {
        slot: u64,
        record: Option<PreparedRecord>,
        had_error: bool,
    },
}

/// Writes account records to Kafka and commits to the account.slots topic.
pub struct AccountSink {
    producer: Arc<FutureProducer>,
    account_slots_topic: String,
}

impl AccountSink {
    pub fn new(producer: Arc<FutureProducer>, account_slots_topic: String) -> Self {
        Self {
            producer,
            account_slots_topic,
        }
    }

    /// Mode A: receives pre-batched `AccountSlot` from coordinator.
    pub async fn run_buffered(
        self,
        mut rx: mpsc::Receiver<AccountSlot<PreparedRecord>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!("AccountSink (buffered) started");

        while let Some(acct_slot) = rx.recv().await {
            let slot = acct_slot.slot;

            // Batch-publish all account records.
            let futures: Vec<_> = acct_slot
                .records
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
                    tracing::error!(?e, slot, topic = %acct_slot.records[i].topic, "Account Kafka write failed");
                    format!("Account Kafka write failed for slot {slot}: {e}").into()
                })?;
            }

            // Commit account slot marker.
            let decoded_account_count = acct_slot
                .records
                .iter()
                .filter(|r| r.is_decoded && r.kind == RecordKind::Account)
                .count() as u64;

            let event = AccountSlotCommitEvent {
                slot,
                decoded_account_count,
                filtered_account_count: acct_slot.filtered_account_count,
                failed_account_count: acct_slot.failed_account_count,
            };

            let payload = serde_json::to_string(&event)?;
            let slot_key = slot.to_string();

            self.producer
                .send(
                    FutureRecord::to(&self.account_slots_topic)
                        .payload(&payload)
                        .key(&slot_key),
                    Duration::from_secs(5),
                )
                .await
                .map_err(|(e, _)| -> Box<dyn std::error::Error + Send + Sync> {
                    format!("Kafka: failed to commit account slot {slot}: {e}").into()
                })?;

            tracing::info!(
                slot,
                decoded_account_count,
                filtered_account_count = acct_slot.filtered_account_count,
                failed_account_count = acct_slot.failed_account_count,
                record_count = acct_slot.records.len(),
                "Kafka: committed account slot"
            );
        }

        tracing::warn!("AccountSink (buffered) channel closed, shutting down");
        Ok(())
    }

    /// Mode B: passthrough — produce to Kafka immediately, emit watermark markers.
    pub async fn run_passthrough(
        self,
        mut rx: mpsc::Receiver<AccountMsg>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!("AccountSink (passthrough) started");

        let mut current_slot: Option<u64> = None;

        while let Some(msg) = rx.recv().await {
            match msg {
                AccountMsg::Record { slot, record, .. } => {
                    // Produce to Kafka immediately if we have a record.
                    if let Some(record) = record {
                        let headers = to_kafka_headers(&record.headers);
                        self.producer
                            .send(
                                FutureRecord::to(&record.topic)
                                    .payload(&record.payload)
                                    .key(&record.key)
                                    .headers(headers),
                                Duration::from_secs(5),
                            )
                            .await
                            .map_err(|(e, _)| -> Box<dyn std::error::Error + Send + Sync> {
                                format!("Passthrough Kafka write failed for slot {slot}: {e}")
                                    .into()
                            })?;
                    }

                    // Emit marker when slot advances (monotonic).
                    if let Some(prev) = current_slot {
                        if slot > prev {
                            self.emit_watermark(prev).await?;
                            current_slot = Some(slot);
                        }
                        // Straggler for old slot — record written, marker NOT moved backward.
                    } else {
                        current_slot = Some(slot);
                    }
                },
            }
        }

        // Emit final marker on channel close.
        if let Some(slot) = current_slot {
            self.emit_watermark(slot).await?;
        }

        tracing::warn!("AccountSink (passthrough) channel closed, shutting down");
        Ok(())
    }

    async fn emit_watermark(
        &self,
        slot: u64,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let event = AccountSlotCommitEvent {
            slot,
            decoded_account_count: 0,
            filtered_account_count: 0,
            failed_account_count: 0,
        };
        let payload = serde_json::to_string(&event)?;
        let slot_key = slot.to_string();

        self.producer
            .send(
                FutureRecord::to(&self.account_slots_topic)
                    .payload(&payload)
                    .key(&slot_key),
                Duration::from_secs(5),
            )
            .await
            .map_err(|(e, _)| -> Box<dyn std::error::Error + Send + Sync> {
                format!("Kafka: failed to emit watermark for slot {slot}: {e}").into()
            })?;

        tracing::debug!(slot, "Emitted account watermark");
        Ok(())
    }
}
