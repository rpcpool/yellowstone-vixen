use rdkafka::{producer::FutureProducer, ClientConfig};

use crate::config::KafkaSinkConfig;

pub fn create_producer(config: &KafkaSinkConfig) -> FutureProducer {
    let mut client_config = ClientConfig::new();
    client_config
        .set("bootstrap.servers", &config.brokers)
        .set("message.timeout.ms", config.message_timeout_ms.to_string())
        .set(
            "queue.buffering.max.messages",
            config.queue_buffering_max_messages.to_string(),
        )
        .set("batch.num.messages", config.batch_num_messages.to_string())
        .set("compression.type", config.compression_type.as_str())
        .set("enable.idempotence", "true");

    if let Some(transactional_id) = &config.transactional_id {
        client_config.set("transactional.id", transactional_id).set(
            "transaction.timeout.ms",
            config.transaction_timeout_ms.to_string(),
        );
    }

    config.apply_sasl_if_configured(&mut client_config);

    client_config
        .create()
        .expect("Failed to create Kafka producer")
}
