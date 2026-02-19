use rdkafka::{producer::FutureProducer, ClientConfig};

use crate::config::KafkaSinkConfig;

pub fn create_producer(config: &KafkaSinkConfig) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", &config.brokers)
        .set("message.timeout.ms", config.message_timeout_ms.to_string())
        .set(
            "queue.buffering.max.messages",
            config.queue_buffering_max_messages.to_string(),
        )
        .set("batch.num.messages", config.batch_num_messages.to_string())
        .set("enable.idempotence", "true")
        .create()
        .expect("Failed to create Kafka producer")
}
