pub mod config;
pub mod events;
pub mod handler;
pub mod kafka_sink;
pub mod parsers;
pub mod producer;
pub mod schema_registry;
pub mod sink;
pub mod topics;
pub mod utils;

// Re-export main types
pub use config::{KafkaCompressionType, KafkaSinkConfig};
pub use events::{
    AccountSlotCommitEvent, CommitScope, MarkerType, PreparedRecord, RawAccountEvent,
    RawInstructionEvent, RecordHeader, RecordKind, TransactionSlotCommitEvent,
};
pub use handler::BufferingHandler;
#[cfg(feature = "experimental-account-parser")]
pub use handler::PassthroughAccountHandler;
pub use kafka_sink::TransactionSlotSink;
#[cfg(feature = "experimental-account-parser")]
pub use kafka_sink::{AccountMsg, AccountPassthroughSink, AccountSlotSink};
pub use parsers::{AccountSubscription, TransactionSubscription};
pub use producer::{create_producer, initialize_transactional_producer};
// Re-export rdkafka producer types for convenience
pub use rdkafka::producer::{FutureProducer, FutureRecord};
pub use schema_registry::{
    ensure_schemas_registered, wrap_payload_with_confluent_wire_format, RegisteredSchema,
    SchemaDefinition,
};
pub use sink::{DynAccountParser, KafkaSink, KafkaSinkBuilder, ParsedOutput};
pub use topics::{
    read_last_committed_account_block, read_last_committed_transaction_block, LastCommitted,
};
pub use utils::{make_account_record_key, make_instruction_record_key};
