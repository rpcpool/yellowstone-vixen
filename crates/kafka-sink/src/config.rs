use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaSinkConfig {
    /// Kafka bootstrap servers (e.g., "localhost:9092").
    pub brokers: String,

    /// Schema Registry URL (e.g., "http://localhost:8081").
    #[serde(default = "default_schema_registry_url")]
    pub schema_registry_url: String,

    #[serde(default = "default_slots_topic")]
    pub slots_topic: String,

    /// Buffer size for the processing channel and deduplication tracking.
    /// Used for mpsc channel capacity (handle bursts) and to prune old processed heights.
    #[serde(default = "default_buffer_size")]
    pub buffer_size: usize,

    #[serde(default = "default_message_timeout_ms")]
    pub message_timeout_ms: u32,

    #[serde(default = "default_queue_buffering_max_messages")]
    pub queue_buffering_max_messages: u32,

    #[serde(default = "default_batch_num_messages")]
    pub batch_num_messages: u32,
}

fn default_schema_registry_url() -> String { "http://localhost:8081".to_string() }

fn default_slots_topic() -> String { "solana.slots".to_string() }

fn default_buffer_size() -> usize { 100 }

fn default_message_timeout_ms() -> u32 { 5000 }

fn default_queue_buffering_max_messages() -> u32 { 100000 }

fn default_batch_num_messages() -> u32 { 1000 }

impl Default for KafkaSinkConfig {
    fn default() -> Self {
        Self {
            brokers: "localhost:9092".to_string(),
            schema_registry_url: default_schema_registry_url(),
            slots_topic: default_slots_topic(),
            buffer_size: default_buffer_size(),
            message_timeout_ms: default_message_timeout_ms(),
            queue_buffering_max_messages: default_queue_buffering_max_messages(),
            batch_num_messages: default_batch_num_messages(),
        }
    }
}

impl KafkaSinkConfig {
    pub fn new(brokers: impl Into<String>, schema_registry_url: impl Into<String>) -> Self {
        Self {
            brokers: brokers.into(),
            schema_registry_url: schema_registry_url.into(),
            ..Default::default()
        }
    }
}
