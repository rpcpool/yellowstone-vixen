use std::time::Duration;

use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication},
    consumer::{BaseConsumer, Consumer},
    ClientConfig, Message, TopicPartitionList,
};

use crate::{config::KafkaSinkConfig, events::SlotCommitEvent};

// TODO: maybe there is way to configure redpanda to do it automatically then can let him handle topic creation natively
/// Creates topics if they don't exist, skips if they already exist.
pub fn ensure_topics_exist_with_log_compaction(
    config: &KafkaSinkConfig,
    instruction_topics: &[&str],
) {
    let admin: AdminClient<_> = ClientConfig::new()
        .set("bootstrap.servers", &config.brokers)
        .create()
        .expect("Failed to create Kafka admin client");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create runtime for topic creation");

    rt.block_on(async {
        let topics_to_create: Vec<NewTopic> = std::iter::once(config.slots_topic.as_str())
            .chain(instruction_topics.iter().copied())
            .map(|t| {
                NewTopic::new(t, 1, TopicReplication::Fixed(1)).set("cleanup.policy", "compact")
            })
            .collect();

        match admin
            .create_topics(topics_to_create.iter(), &AdminOptions::new())
            .await
        {
            Ok(results) => {
                for result in results {
                    match result {
                        Ok(topic) => tracing::info!(topic, "Topic created with log compaction"),
                        Err((topic, err)) => {
                            if err == rdkafka::types::RDKafkaErrorCode::TopicAlreadyExists {
                                tracing::debug!(topic, "Topic already exists");
                            } else {
                                tracing::warn!(?err, topic, "Failed to create topic");
                            }
                        },
                    }
                }
            },
            Err(e) => tracing::error!(?e, "Failed to create topics"),
        }
    });
}

/// Last committed block info from the slots topic.
#[derive(Debug, Clone, Copy)]
pub struct LastCommitted {
    pub slot: u64,
}

/// Read the latest committed block from the slots topic.
/// Used for resumption (slot for from_slot) and deduplication (block_height for ordering).
/// Returns None if the topic is empty or doesn't exist.
pub fn read_last_committed_block(config: &KafkaSinkConfig) -> Option<LastCommitted> {
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", &config.brokers)
        .set("group.id", "vixen-startup-reader")
        .set("enable.auto.commit", "false")
        .create()
        .expect("Failed to create Kafka consumer for startup");

    let metadata = match consumer.fetch_metadata(Some(&config.slots_topic), Duration::from_secs(5))
    {
        Ok(m) => m,
        Err(e) => {
            tracing::warn!(
                ?e,
                "Failed to fetch metadata for slots topic - starting fresh"
            );
            return None;
        },
    };

    let topic_metadata = metadata
        .topics()
        .iter()
        .find(|t| t.name() == config.slots_topic)?;

    if topic_metadata.partitions().is_empty() {
        tracing::info!("Slots topic has no partitions - starting fresh");
        return None;
    }

    let mut latest: Option<LastCommitted> = None;

    for partition in topic_metadata.partitions() {
        let partition_id = partition.id();

        let (_, high) = consumer
            .fetch_watermarks(&config.slots_topic, partition_id, Duration::from_secs(5))
            .ok()?;

        if high == 0 {
            continue; // Empty partition
        }

        let mut tpl = TopicPartitionList::new();
        tpl.add_partition_offset(
            &config.slots_topic,
            partition_id,
            rdkafka::Offset::Offset(high - 1),
        )
        .ok()?;
        consumer.assign(&tpl).ok()?;

        let candidate = consumer
            .poll(Duration::from_secs(5))
            .and_then(|r| r.ok())
            .and_then(|msg| msg.payload().map(|p| p.to_vec()))
            .and_then(|payload| serde_json::from_slice::<SlotCommitEvent>(&payload).ok())
            .map(|event| LastCommitted { slot: event.slot });

        if let Some(c) = candidate
            && latest.is_none_or(|l| c.slot > l.slot)
        {
            latest = Some(c);
        }
    }

    latest
}
