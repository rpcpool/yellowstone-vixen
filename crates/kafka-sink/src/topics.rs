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
    read_last_slot_from_topic(&config.brokers, &config.transaction_slots_topic)
}

/// Read the latest committed account slot for resumption.
/// Returns None if the topic is empty or doesn't exist.
pub fn read_last_committed_account_block(config: &KafkaSinkConfig) -> Option<LastCommitted> {
    read_last_slot_from_topic(&config.brokers, &config.account_slots_topic)
}

/// Read the last committed slot from a Kafka topic by scanning the highest offset.
/// Tries to parse as a slot commit event, falls back to any JSON with a "slot" field.
fn read_last_slot_from_topic(brokers: &str, topic: &str) -> Option<LastCommitted> {
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", "vixen-startup-reader")
        .set("enable.auto.commit", "false")
        .create()
        .expect("Failed to create Kafka consumer for startup");

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

        let (_, high) = consumer
            .fetch_watermarks(topic, partition_id, Duration::from_secs(5))
            .ok()?;

        if high == 0 {
            continue;
        }

        let mut tpl = TopicPartitionList::new();
        tpl.add_partition_offset(topic, partition_id, rdkafka::Offset::Offset(high - 1))
            .ok()?;
        consumer.assign(&tpl).ok()?;

        let candidate = consumer
            .poll(Duration::from_secs(5))
            .and_then(|r| r.ok())
            .and_then(|msg| msg.payload().map(|p| p.to_vec()))
            .and_then(|payload| {
                // Try to parse as TransactionSlotCommitEvent first, then as a generic JSON with a "slot" field.
                serde_json::from_slice::<TransactionSlotCommitEvent>(&payload)
                    .ok()
                    .map(|e| LastCommitted { slot: e.slot })
                    .or_else(|| {
                        serde_json::from_slice::<serde_json::Value>(&payload)
                            .ok()
                            .and_then(|v| v.get("slot")?.as_u64())
                            .map(|slot| LastCommitted { slot })
                    })
            });

        if let Some(c) = candidate
            && latest.is_none_or(|l| c.slot > l.slot)
        {
            latest = Some(c);
        }
    }

    latest
}
