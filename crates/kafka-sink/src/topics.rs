use std::time::Duration;

use rdkafka::{
    consumer::{BaseConsumer, Consumer},
    ClientConfig, Message, TopicPartitionList,
};

use crate::{config::KafkaSinkConfig, events::TransactionSlotCommitEvent};

/// Last committed block info from a slots topic.
#[derive(Debug, Clone, Copy)]
pub struct LastCommitted {
    pub slot: u64,
}

/// Read the latest committed transaction slot from a topic for resumption.
/// Returns None if the topic is empty or doesn't exist.
pub fn read_last_committed_transaction_block(config: &KafkaSinkConfig) -> Option<LastCommitted> {
    read_last_slot_from_topic(config, &config.transaction_slots_topic)
}

/// Read the latest committed account slot for resumption.
/// Returns None if the topic is empty or doesn't exist.
pub fn read_last_committed_account_block(config: &KafkaSinkConfig) -> Option<LastCommitted> {
    read_last_slot_from_topic(config, &config.account_slots_topic)
}

/// Read the last committed slot from a Kafka topic by scanning the highest offset.
/// Tries to parse as a slot commit event, falls back to any JSON with a "slot" field.
fn read_last_slot_from_topic(config: &KafkaSinkConfig, topic: &str) -> Option<LastCommitted> {
    let mut client_config = ClientConfig::new();
    client_config
        .set("bootstrap.servers", &config.brokers)
        .set("group.id", "vixen-startup-reader")
        .set("enable.auto.commit", "false")
        .set("isolation.level", "read_committed"); // read only committed kafka transactions

    config.apply_sasl_if_configured(&mut client_config);

    let consumer: BaseConsumer = match client_config.create() {
        Ok(consumer) => consumer,
        Err(e) => {
            tracing::warn!(
                ?e,
                topic,
                "Failed to create startup Kafka consumer — starting fresh"
            );
            return None;
        },
    };

    let metadata = match consumer.fetch_metadata(Some(topic), Duration::from_secs(5)) {
        Ok(m) => m,
        Err(e) => {
            tracing::warn!(?e, topic, "Failed to fetch metadata — starting fresh");
            return None;
        },
    };

    let topic_metadata = metadata.topics().iter().find(|t| t.name() == topic)?;

    if topic_metadata.partitions().is_empty() {
        tracing::info!(topic, "Topic has no partitions — starting fresh");
        return None;
    }

    let mut latest: Option<LastCommitted> = None;

    for partition in topic_metadata.partitions() {
        let partition_id = partition.id();

        let (low, high) = consumer
            .fetch_watermarks(topic, partition_id, Duration::from_secs(5))
            .ok()?;

        if high == 0 {
            continue;
        }

        let mut tpl = TopicPartitionList::new();
        tpl.add_partition_offset(topic, partition_id, rdkafka::Offset::Offset(low))
            .ok()?;
        consumer.assign(&tpl).ok()?;

        // Transactional topics may end with control records that consume offsets
        // but do not deserialize into user checkpoint payloads. Walk backward
        // from the last stable offset until we find the newest committed slot record.
        let mut candidate = None;
        for offset in (low..high).rev() {
            consumer
                .seek(
                    topic,
                    partition_id,
                    rdkafka::Offset::Offset(offset),
                    Duration::from_secs(5),
                )
                .ok()?;

            candidate = consumer
                .poll(Duration::from_millis(200))
                .and_then(|r| r.ok())
                .and_then(|msg| msg.payload().map(|payload| payload.to_vec()))
                .and_then(|payload| parse_last_committed_payload(&payload));

            if candidate.is_some() {
                break;
            }
        }

        if let Some(c) = candidate
            && latest.is_none_or(|l| c.slot > l.slot)
        {
            latest = Some(c);
        }
    }

    latest
}

fn parse_last_committed_payload(payload: &[u8]) -> Option<LastCommitted> {
    serde_json::from_slice::<TransactionSlotCommitEvent>(payload)
        .ok()
        .map(|e| LastCommitted { slot: e.slot })
        .or_else(|| {
            serde_json::from_slice::<serde_json::Value>(payload)
                .ok()
                .and_then(|v| v.get("slot")?.as_u64())
                .map(|slot| LastCommitted { slot })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_last_committed_payload_reads_transaction_commit_event() {
        let payload = br#"{"slot":42,"blockhash":"abc","transaction_count":1,"decoded_instruction_count":1,"decode_filtered_instruction_count":0,"decode_error_instruction_count":0,"fallback_instruction_count":0,"transaction_status_failed_count":0,"transaction_status_succeeded_count":1}"#;

        let parsed = parse_last_committed_payload(payload);

        assert!(matches!(parsed, Some(LastCommitted { slot: 42 })));
    }

    #[test]
    fn parse_last_committed_payload_falls_back_to_generic_slot_json() {
        let payload = br#"{"slot":99,"marker_type":"Watermark"}"#;

        let parsed = parse_last_committed_payload(payload);

        assert!(matches!(parsed, Some(LastCommitted { slot: 99 })));
    }
}
