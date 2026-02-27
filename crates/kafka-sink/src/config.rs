use serde::{Deserialize, Serialize};
use yellowstone_vixen_block_coordinator::AccountMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaSinkConfig {
    /// Kafka bootstrap servers (e.g., "localhost:9092").
    pub brokers: String,

    /// Schema Registry URL (e.g., "http://localhost:8081").
    #[serde(default = "default_schema_registry_url")]
    pub schema_registry_url: String,

    /// Topic for instruction/transaction slot commit markers.
    #[serde(default = "default_transaction_slots_topic")]
    pub transaction_slots_topic: String,

    /// Topic for account slot commit markers.
    #[serde(default = "default_account_slots_topic")]
    pub account_slots_topic: String,

    /// How accounts are processed and output.
    #[serde(default)]
    pub account_mode: AccountMode,

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

    /// Max attempts for Kafka writes before surfacing an error.
    #[serde(default = "default_kafka_write_max_attempts")]
    pub kafka_write_max_attempts: u32,

    /// Delay between Kafka write retry attempts.
    #[serde(default = "default_kafka_retry_backoff_ms")]
    pub kafka_retry_backoff_ms: u64,
}

fn default_schema_registry_url() -> String {
    "http://localhost:8081".to_string()
}

fn default_transaction_slots_topic() -> String {
    "transaction.slots".to_string()
}

fn default_account_slots_topic() -> String {
    "account.slots".to_string()
}

fn default_buffer_size() -> usize {
    100
}

fn default_message_timeout_ms() -> u32 {
    5000
}

fn default_queue_buffering_max_messages() -> u32 {
    100000
}

fn default_batch_num_messages() -> u32 {
    1000
}

fn default_kafka_write_max_attempts() -> u32 {
    3
}

fn default_kafka_retry_backoff_ms() -> u64 {
    200
}

impl Default for KafkaSinkConfig {
    fn default() -> Self {
        Self {
            brokers: "localhost:9092".to_string(),
            schema_registry_url: default_schema_registry_url(),
            transaction_slots_topic: default_transaction_slots_topic(),
            account_slots_topic: default_account_slots_topic(),
            account_mode: AccountMode::default(),
            buffer_size: default_buffer_size(),
            message_timeout_ms: default_message_timeout_ms(),
            queue_buffering_max_messages: default_queue_buffering_max_messages(),
            batch_num_messages: default_batch_num_messages(),
            kafka_write_max_attempts: default_kafka_write_max_attempts(),
            kafka_retry_backoff_ms: default_kafka_retry_backoff_ms(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use yellowstone_vixen_block_coordinator::AccountCommitAt;

    #[test]
    fn account_mode_defaults_to_finalized_passthrough() {
        let config = KafkaSinkConfig::default();
        assert!(matches!(
            config.account_mode,
            AccountMode::FinalizedPassthrough
        ));
    }

    #[test]
    fn account_commit_at_defaults_to_confirmed() {
        let commit_at = AccountCommitAt::default();
        assert_eq!(commit_at, AccountCommitAt::Confirmed);
    }

    #[test]
    fn processed_finalized_round_trips_through_serde() {
        let mode = AccountMode::Processed {
            commit_at: AccountCommitAt::Finalized,
        };
        let json = serde_json::to_string(&mode).unwrap();
        let deserialized: AccountMode = serde_json::from_str(&json).unwrap();
        match deserialized {
            AccountMode::Processed { commit_at } => {
                assert_eq!(commit_at, AccountCommitAt::Finalized);
            },
            _ => panic!("Expected Processed, got: {deserialized:?}"),
        }
    }

    #[test]
    fn slots_topic_defaults() {
        let config = KafkaSinkConfig::default();
        assert_eq!(config.transaction_slots_topic, "transaction.slots");
        assert_eq!(config.account_slots_topic, "account.slots");
    }

    #[test]
    fn kafka_retry_defaults() {
        let config = KafkaSinkConfig::default();
        assert_eq!(config.kafka_write_max_attempts, 3);
        assert_eq!(config.kafka_retry_backoff_ms, 200);
    }
}
