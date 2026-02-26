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
    AccountSlotCommitEvent, DecodedInstructionEvent, PreparedRecord, RawAccountEvent,
    RawInstructionEvent,
    RecordHeader, RecordKind, TransactionSlotCommitEvent,
};
pub use handler::{BufferingHandler, PassthroughAccountHandler};
pub use kafka_sink::{AccountMsg, AccountSink, TransactionSlotSink};
pub use parsers::{AccountSubscription, TransactionSubscription};
pub use producer::create_producer;
// Re-export rdkafka types for convenience
pub use rdkafka::producer::FutureProducer;
pub use schema_registry::{
    ensure_schemas_registered, wrap_payload_with_confluent_wire_format, RegisteredSchema,
    SchemaDefinition,
};
pub use sink::{DynAccountParser, KafkaSink, KafkaSinkBuilder, ParsedOutput};
pub use topics::{read_last_committed_account_block, read_last_committed_transaction_block, LastCommitted};
pub use utils::{make_account_record_key, make_instruction_record_key};
