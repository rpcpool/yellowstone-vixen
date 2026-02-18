//! Kafka sink builder that accepts Vixen parsers as configuration.
//!
//! This module provides a clean API for configuring kafka-sink with Vixen parsers.
//! Users pass their Vixen parser implementations, and kafka-sink handles the rest.
//!
//! All parsed instructions are serialized using protobuf (prost::Message::encode).

use std::{
    collections::{BTreeSet, HashMap},
    future::Future,
    pin::Pin,
    sync::Arc,
};

use prost::Message;
use yellowstone_vixen_core::{
    bs58,
    instruction::{InstructionUpdate, Path},
    ParseError, Parser,
};

use crate::{
    events::{PreparedRecord, RecordHeader},
    schema_registry::{wrap_payload_with_confluent_wire_format, RegisteredSchema},
    utils::make_record_key,
};

/// Parsed instruction result with protobuf-encoded bytes.
#[derive(Debug, Clone)]
pub struct ParsedInstruction {
    /// Human-readable instruction name (e.g., "TransferChecked").
    pub instruction_name: String,
    /// Discriminant/variant identifier.
    pub instruction_type: String,
    /// Protobuf-encoded bytes (via prost::Message::encode_to_vec).
    pub data: Vec<u8>,
}

impl ParsedInstruction {
    /// Create from any prost::Message type.
    pub fn from_proto<T: Message + std::fmt::Debug>(output: &T) -> Self {
        let debug_str = format!("{:?}", output);
        Self {
            instruction_name: extract_type_name_from_debug(&debug_str).to_string(),
            instruction_type: format!("{:?}", std::mem::discriminant(output)),
            data: output.encode_to_vec(),
        }
    }
}

/// Extract the type name from a Debug-formatted string.
///
/// Handles both struct-like `TypeName { .. }` and tuple-like `TypeName(..)` formats.
// TODO: replace with a proper name method on the proto types.
fn extract_type_name_from_debug(debug_str: &str) -> &str {
    debug_str
        .split_once('{')
        .map(|(name, _)| name.trim())
        .or_else(|| debug_str.split_once('(').map(|(name, _)| name.trim()))
        .unwrap_or(debug_str)
}

/// Type-erased instruction parser trait.
/// This allows storing different parser types in a collection.
pub trait DynInstructionParser: Send + Sync {
    /// Try to parse an instruction. Returns None if this parser doesn't handle it.
    fn try_parse<'a>(
        &'a self,
        ix: &'a InstructionUpdate,
    ) -> Pin<Box<dyn Future<Output = Option<ParsedInstruction>> + Send + 'a>>;

    fn topic(&self) -> &str;

    fn program_name(&self) -> &str;
}

/// Secondary filter that can emit additional records for instructions.
/// Runs after the main parser, allowing the same instruction to be routed
/// to multiple topics without modifying the primary flow.
pub trait SecondaryFilter: Send + Sync {
    fn filter<'a>(
        &'a self,
        ix: &'a InstructionUpdate,
        primary_parsed: Option<&'a ParsedInstruction>,
    ) -> Pin<Box<dyn Future<Output = Option<ParsedInstruction>> + Send + 'a>>;

    fn topic(&self) -> &str;

    fn label(&self) -> &str;
}

struct ParserWrapper<P> {
    parser: P,
    topic: String,
    program_name: String,
}

impl<P, O> DynInstructionParser for ParserWrapper<P>
where
    P: Parser<Input = InstructionUpdate, Output = O> + Send + Sync,
    O: Message + std::fmt::Debug + Send + Sync,
{
    fn try_parse<'a>(
        &'a self,
        ix: &'a InstructionUpdate,
    ) -> Pin<Box<dyn Future<Output = Option<ParsedInstruction>> + Send + 'a>> {
        Box::pin(async move {
            match self.parser.parse(ix).await {
                Ok(output) => Some(ParsedInstruction::from_proto(&output)),
                Err(ParseError::Filtered) => None,
                Err(e) => {
                    tracing::warn!(?e, program = %self.program_name, "Error parsing instruction");
                    None
                },
            }
        })
    }

    fn topic(&self) -> &str { &self.topic }

    fn program_name(&self) -> &str { &self.program_name }
}

pub struct KafkaSinkBuilder {
    parsers: Vec<Arc<dyn DynInstructionParser>>,
    secondary_filters: Vec<Arc<dyn SecondaryFilter>>,
    fallback_topic: String,
}

impl Default for KafkaSinkBuilder {
    fn default() -> Self { Self::new() }
}

impl KafkaSinkBuilder {
    pub fn new() -> Self {
        Self {
            parsers: Vec::new(),
            secondary_filters: Vec::new(),
            fallback_topic: "unknown.instructions".to_string(),
        }
    }

    /// Add a Vixen parser with its program name and Kafka topic.
    ///
    /// # Arguments
    /// * `parser` - A Vixen `Parser<Input=InstructionUpdate>` implementation
    /// * `program_name` - Name of the program (e.g., "spl-token")
    /// * `topic` - Kafka topic for this parser's output
    pub fn parser<P, O>(mut self, parser: P, program_name: &str, topic: &str) -> Self
    where
        P: Parser<Input = InstructionUpdate, Output = O> + Send + Sync + 'static,
        O: Message + std::fmt::Debug + Send + Sync + 'static,
    {
        self.parsers.push(Arc::new(ParserWrapper {
            parser,
            topic: topic.to_string(),
            program_name: program_name.to_string(),
        }));
        self
    }

    /// Set the fallback topic for instructions that no parser handles.
    pub fn fallback_topic(mut self, topic: &str) -> Self {
        self.fallback_topic = topic.to_string();
        self
    }

    /// Add a secondary filter that can emit additional records.
    /// Secondary filters run after the main parser and can route
    /// matching instructions to additional topics.
    pub fn secondary_filter<F>(mut self, filter: F) -> Self
    where F: SecondaryFilter + 'static {
        self.secondary_filters.push(Arc::new(filter));
        self
    }

    pub fn build(self) -> ConfiguredParsers {
        ConfiguredParsers {
            parsers: self.parsers,
            secondary_filters: self.secondary_filters,
            fallback_topic: self.fallback_topic,
            schema_ids: HashMap::new(),
        }
    }

    pub fn topics(&self) -> Vec<&str> {
        self.parsers
            .iter()
            .map(|p| p.topic())
            .chain(self.secondary_filters.iter().map(|f| f.topic()))
            .chain(std::iter::once(self.fallback_topic.as_str()))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
}

#[derive(Clone)]
pub struct ConfiguredParsers {
    parsers: Vec<Arc<dyn DynInstructionParser>>,
    secondary_filters: Vec<Arc<dyn SecondaryFilter>>,
    fallback_topic: String,
    /// Map of topic -> schema info for encoding with Confluent wire format.
    schema_ids: HashMap<String, RegisteredSchema>,
}

impl Default for ConfiguredParsers {
    fn default() -> Self {
        Self {
            parsers: Vec::new(),
            secondary_filters: Vec::new(),
            fallback_topic: "unknown.instructions".to_string(),
            schema_ids: HashMap::new(),
        }
    }
}

impl ConfiguredParsers {
    pub fn topics(&self) -> Vec<&str> {
        self.parsers
            .iter()
            .map(|p| p.topic())
            .chain(self.secondary_filters.iter().map(|f| f.topic()))
            .chain(std::iter::once(self.fallback_topic.as_str()))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    pub fn fallback_topic(&self) -> &str { &self.fallback_topic }

    /// Set schema IDs for encoding messages with Confluent wire format.
    /// The key should be the subject name (e.g., "spl-token.instructions-value").
    pub fn set_schema_ids(&mut self, schemas: HashMap<String, RegisteredSchema>) {
        for (subject, schema) in &schemas {
            tracing::info!(
                subject,
                schema_id = schema.schema_id,
                message_index = schema.message_index,
                "Registered schema for encoding"
            );
        }
        self.schema_ids = schemas;
    }

    /// Get schema ID for a topic (looks up "<topic>-value" subject).
    fn get_schema_for_topic(&self, topic: &str) -> Option<&RegisteredSchema> {
        let subject = format!("{}-value", topic);
        let result = self.schema_ids.get(&subject);
        if result.is_none() {
            tracing::warn!(
                topic,
                subject,
                available_subjects = ?self.schema_ids.keys().collect::<Vec<_>>(),
                "No schema found for topic"
            );
        }
        result
    }

    async fn try_parse(&self, ix: &InstructionUpdate) -> Option<(ParsedInstruction, &str, &str)> {
        for parser in &self.parsers {
            if let Some(parsed) = parser.try_parse(ix).await {
                return Some((parsed, parser.program_name(), parser.topic()));
            }
        }
        None
    }

    /// Parse an instruction and prepare a Kafka record.
    ///
    /// Tries each registered parser in order. If one matches, builds a decoded record;
    /// otherwise builds a fallback record with raw instruction data.
    /// Returns the record and the parsed result (if any) for secondary filters.
    pub async fn parse_instruction(
        &self,
        slot: u64,
        signature: &[u8],
        path: &Path,
        ix: &InstructionUpdate,
    ) -> (PreparedRecord, Option<ParsedInstruction>) {
        match self.try_parse(ix).await {
            Some((parsed, program_name, topic)) => {
                let record = self.prepare_decoded_record(
                    slot,
                    signature,
                    path,
                    parsed.clone(),
                    program_name,
                    topic,
                );
                (record, Some(parsed))
            },
            None => {
                let record = self.prepare_fallback_record(slot, signature, path, ix);
                (record, None)
            },
        }
    }

    pub fn secondary_filters(&self) -> &[Arc<dyn SecondaryFilter>] { &self.secondary_filters }

    /// Build the base headers and key shared by all record types.
    fn base_record(slot: u64, signature: &[u8], path: &Path) -> (String, Vec<RecordHeader>) {
        let sig_str = bs58::encode(signature).into_string();
        let path_str = format!("{path:?}");
        let key = make_record_key(slot, &sig_str, &path_str);
        let headers = vec![
            RecordHeader {
                key: "slot",
                value: slot.to_string(),
            },
            RecordHeader {
                key: "signature",
                value: sig_str,
            },
            RecordHeader {
                key: "ix_index",
                value: path_str,
            },
        ];
        (key, headers)
    }

    /// Encode payload with Confluent wire format if a schema is registered for the topic,
    /// otherwise return the raw protobuf bytes.
    fn encode_payload_for_topic(&self, topic: &str, raw_data: Vec<u8>) -> Vec<u8> {
        match self.get_schema_for_topic(topic) {
            Some(schema) => {
                let indices: &[i32] = if schema.message_index == 0 {
                    &[] // First message: empty array per Confluent spec
                } else {
                    &[schema.message_index]
                };
                wrap_payload_with_confluent_wire_format(schema.schema_id, indices, &raw_data)
            },
            None => raw_data,
        }
    }

    /// Prepare a record for a successfully decoded instruction.
    /// Payload is protobuf-encoded with Confluent wire format, metadata goes in Kafka headers.
    pub fn prepare_decoded_record(
        &self,
        slot: u64,
        signature: &[u8],
        path: &Path,
        parsed: ParsedInstruction,
        program_name: &str,
        topic: &str,
    ) -> PreparedRecord {
        let (key, mut headers) = Self::base_record(slot, signature, path);
        let payload = self.encode_payload_for_topic(topic, parsed.data);

        headers.extend([
            RecordHeader {
                key: "program",
                value: program_name.to_string(),
            },
            RecordHeader {
                key: "instruction_type",
                value: parsed.instruction_type,
            },
            RecordHeader {
                key: "instruction_name",
                value: parsed.instruction_name.clone(),
            },
        ]);

        PreparedRecord {
            topic: topic.to_string(),
            payload,
            key,
            headers,
            label: parsed.instruction_name,
            is_decoded: true,
        }
    }

    /// Prepare a fallback record for unrecognized instructions.
    /// Payload is the raw instruction data, metadata in headers.
    pub fn prepare_fallback_record(
        &self,
        slot: u64,
        signature: &[u8],
        path: &Path,
        ix: &InstructionUpdate,
    ) -> PreparedRecord {
        let (key, mut headers) = Self::base_record(slot, signature, path);
        let program_id = bs58::encode(ix.program).into_string();

        headers.push(RecordHeader {
            key: "program_id",
            value: program_id.clone(),
        });

        PreparedRecord {
            topic: self.fallback_topic.clone(),
            payload: ix.data.clone(),
            key,
            headers,
            label: program_id,
            is_decoded: false,
        }
    }
}
