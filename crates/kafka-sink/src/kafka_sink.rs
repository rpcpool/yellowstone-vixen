//! Downstream Kafka writers that consume slots from the coordinator.
//!
//! `TransactionSlotSink` — receives `InstructionSlot<PreparedRecord>`, writes to `transaction.slots`.
//! `AccountSink` — two modes:
//!   - `run_buffered`: receives `AccountSlot<PreparedRecord>` from coordinator, writes to `account.slots`.
//!   - `run_passthrough`: receives `AccountMsg` directly, produces to Kafka immediately.

use std::{future::Future, sync::Arc, time::Duration};

use rdkafka::{
    message::OwnedHeaders,
    producer::{FutureProducer, FutureRecord},
};
use tokio::{sync::mpsc, time::sleep};
use yellowstone_vixen_block_coordinator::{
    AccountCommitAt, AccountMode, AccountSlot, InstructionSlot,
};

type SinkError = Box<dyn std::error::Error + Send + Sync>;

fn kafka_send_error(context: &str, slot: u64, err: impl std::fmt::Display) -> SinkError {
    format!("{context} for slot {slot}: {err}").into()
}

async fn with_retry<F, Fut, R>(
    max_attempts: u32,
    backoff: Duration,
    context: &str,
    slot: u64,
    mut f: F,
    mut on_retry: R,
) -> Result<(), SinkError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<(), SinkError>>,
    R: FnMut(&SinkError, u32, u32),
{
    let max_attempts = max_attempts.max(1);
    for attempt in 1..=max_attempts {
        match f().await {
            Ok(()) => return Ok(()),
            Err(e) if attempt < max_attempts => {
                on_retry(&e, attempt, max_attempts);
                sleep(backoff).await;
            },
            Err(e) => {
                return Err(format!(
                    "{context} for slot {slot} failed after {attempt} attempts: {e}"
                )
                .into());
            },
        }
    }
    unreachable!("max_attempts >= 1")
}

use crate::{
    config::KafkaSinkConfig,
    events::{
        AccountSlotCommitEvent, CommitScope, MarkerType, PreparedRecord, RecordHeader, RecordKind,
        TransactionSlotCommitEvent,
    },
};

fn to_kafka_headers(headers: &[RecordHeader]) -> OwnedHeaders {
    headers.iter().fold(OwnedHeaders::new(), |acc, h| {
        acc.insert(rdkafka::message::Header {
            key: h.key,
            value: Some(h.value.as_bytes()),
        })
    })
}

async fn batch_publish_records(
    producer: &FutureProducer,
    slot: u64,
    records: &[PreparedRecord],
    timeout: Duration,
    error_prefix: &'static str,
) -> Result<(), SinkError> {
    let futures: Vec<_> = records
        .iter()
        .map(|record| {
            let headers = to_kafka_headers(&record.headers);
            producer.send(
                FutureRecord::to(&record.topic)
                    .payload(&record.payload)
                    .key(&record.key)
                    .headers(headers),
                timeout,
            )
        })
        .collect();

    let results = futures::future::join_all(futures).await;
    for (i, result) in results.into_iter().enumerate() {
        result.map_err(|(e, _)| {
            tracing::error!(
                ?e,
                slot,
                topic = %records[i].topic,
                error_prefix,
                "Kafka write failed"
            );
            kafka_send_error(error_prefix, slot, e)
        })?;
    }

    Ok(())
}

/// Consumes instruction slots from the coordinator and writes them to Kafka.
pub struct TransactionSlotSink {
    config: KafkaSinkConfig,
    producer: Arc<FutureProducer>,
}

impl TransactionSlotSink {
    pub fn new(config: KafkaSinkConfig, producer: Arc<FutureProducer>) -> Self {
        Self { config, producer }
    }

    pub async fn run(
        self,
        mut rx: mpsc::Receiver<InstructionSlot<PreparedRecord>>,
    ) -> Result<(), SinkError> {
        tracing::info!("TransactionSlotSink started, waiting for instruction slots...");
        let max_attempts = self.config.kafka_write_max_attempts.max(1);
        let retry_backoff = Duration::from_millis(self.config.kafka_retry_backoff_ms);

        while let Some(ix_slot) = rx.recv().await {
            with_retry(
                max_attempts,
                retry_backoff,
                "Kafka write",
                ix_slot.slot,
                || self.write_instruction_slot(&ix_slot),
                |e, attempt, max| {
                    tracing::warn!(
                        ?e,
                        slot = ix_slot.slot,
                        attempt,
                        max_attempts = max,
                        "Kafka write failed, retrying"
                    );
                },
            )
            .await?;
        }

        tracing::warn!("TransactionSlotSink channel closed, shutting down");
        Ok(())
    }

    async fn write_instruction_slot(
        &self,
        ix_slot: &InstructionSlot<PreparedRecord>,
    ) -> Result<(), SinkError> {
        batch_publish_records(
            self.producer.as_ref(),
            ix_slot.slot,
            &ix_slot.records,
            Duration::ZERO,
            "Kafka write failed",
        )
        .await?;
        self.commit_transaction_slot_checkpoint(ix_slot).await
    }

    async fn commit_transaction_slot_checkpoint(
        &self,
        ix_slot: &InstructionSlot<PreparedRecord>,
    ) -> Result<(), SinkError> {
        let slot = ix_slot.slot;
        let record_count = ix_slot.records.len();
        let event = build_transaction_slot_commit_event(ix_slot);

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
            .map_err(|(e, _)| kafka_send_error("Failed to commit slot", slot, e))?;

        tracing::debug!(
            slot,
            decoded_instruction_count = event.decoded_instruction_count,
            decode_filtered_instruction_count = ix_slot.filtered_instruction_count,
            decode_error_instruction_count = ix_slot.failed_instruction_count,
            fallback_instruction_count = event.fallback_instruction_count,
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
    account_mode: AccountMode,
    max_attempts: u32,
    retry_backoff: Duration,
}

impl AccountSink {
    pub fn new(
        producer: Arc<FutureProducer>,
        account_slots_topic: String,
        account_mode: AccountMode,
        max_attempts: u32,
        retry_backoff_ms: u64,
    ) -> Self {
        Self {
            producer,
            account_slots_topic,
            account_mode,
            max_attempts: max_attempts.max(1),
            retry_backoff: Duration::from_millis(retry_backoff_ms),
        }
    }

    /// Mode A: receives pre-batched `AccountSlot` from coordinator.
    pub async fn run_buffered(
        self,
        mut rx: mpsc::Receiver<AccountSlot<PreparedRecord>>,
    ) -> Result<(), SinkError> {
        tracing::info!("AccountSink (buffered) started");

        while let Some(acct_slot) = rx.recv().await {
            with_retry(
                self.max_attempts,
                self.retry_backoff,
                "Account slot write",
                acct_slot.slot,
                || self.write_account_slot(&acct_slot),
                |e, attempt, max| {
                    tracing::warn!(
                        ?e,
                        slot = acct_slot.slot,
                        attempt,
                        max_attempts = max,
                        "Account slot write failed, retrying"
                    );
                },
            )
            .await?;
        }

        tracing::warn!("AccountSink (buffered) channel closed, shutting down");
        Ok(())
    }

    /// Mode B: passthrough — produce to Kafka immediately, emit watermark markers.
    pub async fn run_passthrough(
        self,
        mut rx: mpsc::Receiver<AccountMsg>,
    ) -> Result<(), SinkError> {
        tracing::info!("AccountSink (passthrough) started");

        let mut current_slot: Option<u64> = None;

        while let Some(msg) = rx.recv().await {
            match msg {
                AccountMsg::Record { slot, record, .. } => {
                    // Produce to Kafka immediately if we have a record.
                    if let Some(record) = record {
                        with_retry(
                            self.max_attempts,
                            self.retry_backoff,
                            "Passthrough account write",
                            slot,
                            || self.publish_passthrough_record(slot, &record),
                            |e, attempt, max| {
                                tracing::warn!(?e, slot, attempt, max_attempts = max,
                                    topic = %record.topic,
                                    "Passthrough account write failed, retrying");
                            },
                        )
                        .await?;
                    }

                    // Emit marker when slot advances (monotonic).
                    if let Some(prev) = current_slot {
                        if slot > prev {
                            self.emit_watermark_with_retry(prev).await?;
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
            self.emit_watermark_with_retry(slot).await?;
        }

        tracing::warn!("AccountSink (passthrough) channel closed, shutting down");
        Ok(())
    }

    async fn write_account_slot(
        &self,
        acct_slot: &AccountSlot<PreparedRecord>,
    ) -> Result<(), SinkError> {
        let slot = acct_slot.slot;

        batch_publish_records(
            self.producer.as_ref(),
            slot,
            &acct_slot.records,
            // Do not retry in send; QueueFull bubbles up so the slot retries as a unit.
            Duration::ZERO,
            "Account Kafka write failed",
        )
        .await?;

        // Commit account slot marker.
        let event = build_account_slot_commit_event(acct_slot, &self.account_mode);

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
            .map_err(|(e, _)| kafka_send_error("Failed to commit account slot", slot, e))?;

        tracing::debug!(
            slot,
            marker_type = %event.marker_type,
            account_commit_at = %event.account_commit_at,
            decoded_account_count = event.decoded_account_count.unwrap_or(0),
            decode_filtered_account_count = event.decode_filtered_account_count.unwrap_or(0),
            decode_error_account_count = event.decode_error_account_count.unwrap_or(0),
            fallback_account_count = event.fallback_account_count.unwrap_or(0),
            record_count = acct_slot.records.len(),
            "Kafka: committed account slot"
        );
        Ok(())
    }

    async fn publish_passthrough_record(
        &self,
        slot: u64,
        record: &PreparedRecord,
    ) -> Result<(), SinkError> {
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
            .map_err(|(e, _)| kafka_send_error("Passthrough Kafka write failed", slot, e))?;
        Ok(())
    }

    async fn emit_watermark_with_retry(&self, slot: u64) -> Result<(), SinkError> {
        with_retry(
            self.max_attempts,
            self.retry_backoff,
            "Emit account watermark",
            slot,
            || self.emit_watermark(slot),
            |e, attempt, max| {
                tracing::warn!(
                    ?e,
                    slot,
                    attempt,
                    max_attempts = max,
                    "Failed to emit account watermark, retrying"
                );
            },
        )
        .await
    }

    async fn emit_watermark(&self, slot: u64) -> Result<(), SinkError> {
        let event = AccountSlotCommitEvent {
            slot,
            marker_type: MarkerType::Watermark,
            account_commit_at: CommitScope::Stream,
            decoded_account_count: None,
            decode_filtered_account_count: None,
            decode_error_account_count: None,
            fallback_account_count: None,
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
            .map_err(|(e, _)| kafka_send_error("Failed to emit watermark", slot, e))?;

        tracing::debug!(slot, "Emitted account watermark");
        Ok(())
    }
}

fn build_transaction_slot_commit_event(
    ix_slot: &InstructionSlot<PreparedRecord>,
) -> TransactionSlotCommitEvent {
    let decoded_instruction_count = ix_slot
        .records
        .iter()
        .filter(|r| r.is_decoded && r.kind == RecordKind::Instruction)
        .count() as u64;
    let fallback_instruction_count = ix_slot
        .records
        .iter()
        .filter(|r| !r.is_decoded && r.kind == RecordKind::Instruction)
        .count() as u64;

    TransactionSlotCommitEvent {
        slot: ix_slot.slot,
        blockhash: ix_slot.blockhash.to_string(),
        transaction_count: ix_slot.executed_transaction_count,
        decoded_instruction_count,
        decode_filtered_instruction_count: ix_slot.filtered_instruction_count,
        decode_error_instruction_count: ix_slot.failed_instruction_count,
        fallback_instruction_count,
        transaction_status_failed_count: ix_slot.transaction_status_failed_count,
        transaction_status_succeeded_count: ix_slot.transaction_status_succeeded_count,
    }
}

fn build_account_slot_commit_event(
    acct_slot: &AccountSlot<PreparedRecord>,
    account_mode: &AccountMode,
) -> AccountSlotCommitEvent {
    let decoded_account_count = acct_slot
        .records
        .iter()
        .filter(|r| r.is_decoded && r.kind == RecordKind::Account)
        .count() as u64;
    let fallback_account_count = acct_slot
        .records
        .iter()
        .filter(|r| !r.is_decoded && r.kind == RecordKind::Account)
        .count() as u64;

    AccountSlotCommitEvent {
        slot: acct_slot.slot,
        marker_type: MarkerType::Completed,
        account_commit_at: CommitScope::from(account_mode),
        decoded_account_count: Some(decoded_account_count),
        decode_filtered_account_count: Some(acct_slot.filtered_account_count),
        decode_error_account_count: Some(acct_slot.failed_account_count),
        fallback_account_count: Some(fallback_account_count),
    }
}

impl From<&AccountMode> for CommitScope {
    fn from(mode: &AccountMode) -> Self {
        match mode {
            AccountMode::Processed { commit_at } => match commit_at {
                AccountCommitAt::Confirmed => Self::Confirmed,
                AccountCommitAt::Finalized => Self::Finalized,
            },
            AccountMode::FinalizedPassthrough => Self::Finalized,
        }
    }
}

#[cfg(test)]
mod tests {
    use yellowstone_vixen_block_coordinator::{
        AccountCommitAt, AccountMode, AccountSlot, InstructionSlot,
    };

    use super::*;

    fn record(topic: &str, kind: RecordKind, is_decoded: bool) -> PreparedRecord {
        PreparedRecord {
            topic: topic.to_string(),
            payload: vec![],
            key: "k".to_string(),
            headers: vec![],
            is_decoded,
            kind,
        }
    }

    #[test]
    fn transaction_commit_event_counts_decode_and_fallback_explicitly() {
        let ix_slot = InstructionSlot {
            slot: 42,
            parent_slot: 41,
            blockhash: Default::default(),
            executed_transaction_count: 10,
            records: vec![
                record("decoded.instructions", RecordKind::Instruction, true),
                record("failed.instructions", RecordKind::Instruction, false),
                // Defensive: non-instruction record must not affect instruction counters.
                record("decoded.accounts", RecordKind::Account, true),
            ],
            filtered_instruction_count: 7,
            failed_instruction_count: 3,
            transaction_status_failed_count: 2,
            transaction_status_succeeded_count: 8,
        };

        let event = build_transaction_slot_commit_event(&ix_slot);
        assert_eq!(event.slot, 42);
        assert_eq!(event.transaction_count, 10);
        assert_eq!(event.decoded_instruction_count, 1);
        assert_eq!(event.decode_filtered_instruction_count, 7);
        assert_eq!(event.decode_error_instruction_count, 3);
        assert_eq!(event.fallback_instruction_count, 1);
        assert_eq!(event.transaction_status_failed_count, 2);
        assert_eq!(event.transaction_status_succeeded_count, 8);
    }

    #[test]
    fn account_commit_event_counts_decode_and_fallback_explicitly() {
        let acct_slot = AccountSlot {
            slot: 55,
            records: vec![
                record("decoded.accounts", RecordKind::Account, true),
                record("failed.accounts", RecordKind::Account, false),
                // Defensive: non-account record must not affect account counters.
                record("decoded.instructions", RecordKind::Instruction, true),
            ],
            decoded_account_count: 0, // event computes this from records, not this field.
            filtered_account_count: 4,
            failed_account_count: 2,
        };

        let event = build_account_slot_commit_event(&acct_slot, &AccountMode::Processed {
            commit_at: AccountCommitAt::Confirmed,
        });
        assert_eq!(event.slot, 55);
        assert_eq!(event.marker_type, MarkerType::Completed);
        assert_eq!(event.account_commit_at, CommitScope::Confirmed);
        assert_eq!(event.decoded_account_count, Some(1));
        assert_eq!(event.decode_filtered_account_count, Some(4));
        assert_eq!(event.decode_error_account_count, Some(2));
        assert_eq!(event.fallback_account_count, Some(1));
    }

    #[test]
    fn passthrough_mode_commits_as_finalized() {
        let acct_slot = AccountSlot {
            slot: 99,
            records: vec![],
            decoded_account_count: 0,
            filtered_account_count: 0,
            failed_account_count: 0,
        };
        let event = build_account_slot_commit_event(&acct_slot, &AccountMode::FinalizedPassthrough);
        assert_eq!(event.marker_type, MarkerType::Completed);
        assert_eq!(event.account_commit_at, CommitScope::Finalized);
    }

    #[test]
    fn watermark_json_omits_account_count_fields() {
        let event = AccountSlotCommitEvent {
            slot: 77,
            marker_type: MarkerType::Watermark,
            account_commit_at: CommitScope::Stream,
            decoded_account_count: None,
            decode_filtered_account_count: None,
            decode_error_account_count: None,
            fallback_account_count: None,
        };

        let value = serde_json::to_value(&event).expect("serialize watermark");
        let obj = value.as_object().expect("object");
        assert!(!obj.contains_key("decoded_account_count"));
        assert!(!obj.contains_key("decode_filtered_account_count"));
        assert!(!obj.contains_key("decode_error_account_count"));
        assert!(!obj.contains_key("fallback_account_count"));
    }

    #[test]
    fn completed_json_includes_account_count_fields() {
        let acct_slot = AccountSlot {
            slot: 88,
            records: vec![
                record("decoded.accounts", RecordKind::Account, true),
                record("failed.accounts", RecordKind::Account, false),
            ],
            decoded_account_count: 0,
            filtered_account_count: 9,
            failed_account_count: 3,
        };
        let event = build_account_slot_commit_event(&acct_slot, &AccountMode::Processed {
            commit_at: AccountCommitAt::Confirmed,
        });

        let value = serde_json::to_value(&event).expect("serialize completed marker");
        let obj = value.as_object().expect("object");
        assert_eq!(
            obj.get("decoded_account_count").and_then(|v| v.as_u64()),
            Some(1)
        );
        assert_eq!(
            obj.get("decode_filtered_account_count")
                .and_then(|v| v.as_u64()),
            Some(9)
        );
        assert_eq!(
            obj.get("decode_error_account_count")
                .and_then(|v| v.as_u64()),
            Some(3)
        );
        assert_eq!(
            obj.get("fallback_account_count").and_then(|v| v.as_u64()),
            Some(1)
        );
    }
}
