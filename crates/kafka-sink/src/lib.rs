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
pub use config::KafkaSinkConfig;
pub use events::{
    DecodedInstructionEvent, PreparedRecord, RawInstructionEvent, RecordHeader, SlotCommitEvent,
};
pub use handler::BufferingHandler;
pub use kafka_sink::ConfirmedSlotSink;
pub use parsers::TransactionParser;
pub use producer::create_producer;
// Re-export rdkafka types for convenience
pub use rdkafka::producer::FutureProducer;
pub use schema_registry::{
    ensure_schemas_registered, wrap_payload_with_confluent_wire_format, RegisteredSchema,
    SchemaDefinition,
};
pub use sink::{ConfiguredParsers, KafkaSinkBuilder, ParsedInstruction, SecondaryFilter};
pub use topics::{read_last_committed_block, LastCommitted};
pub use utils::make_record_key;
