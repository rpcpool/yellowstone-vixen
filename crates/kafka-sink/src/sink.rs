//! Kafka sink builder that accepts Vixen parsers as configuration.
//!
//! This module provides a clean API for configuring kafka-sink with Vixen parsers.
//! Users pass their Vixen parser implementations, and kafka-sink handles the rest.
//!
//! All parsed outputs are serialized using protobuf (prost::Message::encode).

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
    AccountUpdate, ParseError, Parser,
};

use crate::{
    events::{PreparedRecord, RecordHeader, RecordKind},
    schema_registry::{wrap_payload_with_confluent_wire_format, RegisteredSchema},
    utils::{make_account_record_key, make_instruction_record_key},
};

/// Parsed output result with protobuf-encoded bytes.
#[derive(Debug, Clone)]
pub struct ParsedOutput {
    /// Protobuf-encoded bytes (via prost::Message::encode_to_vec).
    pub data: Vec<u8>,
}

impl ParsedOutput {
    /// Create from any prost::Message type.
    pub fn from_proto<T: Message>(output: &T) -> Self {
        Self {
            data: output.encode_to_vec(),
        }
    }
}

// --- DynInstructionParser ---

/// Type-erased instruction parser trait.
/// This allows storing different parser types in a collection.
pub trait DynInstructionParser: Send + Sync {
    /// Try to parse an instruction. Returns None if this parser doesn't handle it.
    fn try_parse<'a>(
        &'a self,
        ix: &'a InstructionUpdate,
    ) -> Pin<Box<dyn Future<Output = Option<ParsedOutput>> + Send + 'a>>;

    fn topic(&self) -> &str;

    fn program_name(&self) -> &str;

    fn fallback_topic(&self) -> Option<&str>;
}

/// Secondary filter that can emit additional records for instructions.
/// Runs after the main parser, allowing the same instruction to be routed
/// to multiple topics without modifying the primary flow.
pub trait SecondaryFilter: Send + Sync {
    fn filter<'a>(
        &'a self,
        ix: &'a InstructionUpdate,
        primary_parsed: Option<&'a ParsedOutput>,
    ) -> Pin<Box<dyn Future<Output = Option<ParsedOutput>> + Send + 'a>>;

    fn topic(&self) -> &str;
}

struct InstructionParserWrapper<P> {
    parser: P,
    topic: String,
    program_name: String,
    fallback_topic: Option<String>,
}

impl<P, O> DynInstructionParser for InstructionParserWrapper<P>
where
    P: Parser<Input = InstructionUpdate, Output = O> + Send + Sync,
    O: Message + Send + Sync,
{
    fn try_parse<'a>(
        &'a self,
        ix: &'a InstructionUpdate,
    ) -> Pin<Box<dyn Future<Output = Option<ParsedOutput>> + Send + 'a>> {
        Box::pin(async move {
            match self.parser.parse(ix).await {
                Ok(output) => Some(ParsedOutput::from_proto(&output)),
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

    fn fallback_topic(&self) -> Option<&str> { self.fallback_topic.as_deref() }
}

// --- DynAccountParser ---

/// Type-erased account parser trait.
/// This allows storing different account parser types in a collection.
pub trait DynAccountParser: Send + Sync {
    /// Try to parse an account. Returns None if this parser doesn't handle it.
    fn try_parse<'a>(
        &'a self,
        acct: &'a AccountUpdate,
    ) -> Pin<Box<dyn Future<Output = Option<ParsedOutput>> + Send + 'a>>;

    fn topic(&self) -> &str;

    fn program_name(&self) -> &str;

    fn fallback_topic(&self) -> Option<&str>;
}

struct AccountParserWrapper<P> {
    parser: P,
    topic: String,
    program_name: String,
    fallback_topic: Option<String>,
}

impl<P, O> DynAccountParser for AccountParserWrapper<P>
where
    P: Parser<Input = AccountUpdate, Output = O> + Send + Sync,
    O: Message + Send + Sync,
{
    fn try_parse<'a>(
        &'a self,
        acct: &'a AccountUpdate,
    ) -> Pin<Box<dyn Future<Output = Option<ParsedOutput>> + Send + 'a>> {
        Box::pin(async move {
            match self.parser.parse(acct).await {
                Ok(output) => Some(ParsedOutput::from_proto(&output)),
                Err(ParseError::Filtered) => None,
                Err(e) => {
                    tracing::warn!(?e, program = %self.program_name, "Error parsing account");
                    None
                },
            }
        })
    }

    fn topic(&self) -> &str { &self.topic }

    fn program_name(&self) -> &str { &self.program_name }

    fn fallback_topic(&self) -> Option<&str> { self.fallback_topic.as_deref() }
}

// --- Sealed SinkInput trait ---

mod sealed {
    pub trait Sealed {}
}

/// Trait that dispatches parser registration to the right internal vec.
/// Sealed so only `InstructionUpdate` and `AccountUpdate` can implement it.
pub trait SinkInput: sealed::Sealed {
    #[doc(hidden)]
    fn add_parser<P, O>(
        parser: P,
        name: String,
        topic: String,
        fallback_topic: Option<String>,
        builder: &mut KafkaSinkBuilder,
    ) where
        P: Parser<Input = Self, Output = O> + Send + Sync + 'static,
        O: Message + Send + Sync + 'static;
}

impl sealed::Sealed for InstructionUpdate {}

impl SinkInput for InstructionUpdate {
    fn add_parser<P, O>(
        parser: P,
        name: String,
        topic: String,
        fallback_topic: Option<String>,
        builder: &mut KafkaSinkBuilder,
    ) where
        P: Parser<Input = Self, Output = O> + Send + Sync + 'static,
        O: Message + Send + Sync + 'static,
    {
        builder
            .instruction_parsers
            .push(Arc::new(InstructionParserWrapper {
                parser,
                topic,
                program_name: name,
                fallback_topic,
            }));
    }
}

impl sealed::Sealed for AccountUpdate {}

impl SinkInput for AccountUpdate {
    fn add_parser<P, O>(
        parser: P,
        name: String,
        topic: String,
        fallback_topic: Option<String>,
        builder: &mut KafkaSinkBuilder,
    ) where
        P: Parser<Input = Self, Output = O> + Send + Sync + 'static,
        O: Message + Send + Sync + 'static,
    {
        builder
            .account_parsers
            .push(Arc::new(AccountParserWrapper {
                parser,
                topic,
                program_name: name,
                fallback_topic,
            }));
    }
}

// --- KafkaSinkBuilder ---

pub struct KafkaSinkBuilder {
    instruction_parsers: Vec<Arc<dyn DynInstructionParser>>,
    account_parsers: Vec<Arc<dyn DynAccountParser>>,
    secondary_filters: Vec<Arc<dyn SecondaryFilter>>,
    fallback_topic: Option<String>,
}

impl Default for KafkaSinkBuilder {
    fn default() -> Self { Self::new() }
}

impl KafkaSinkBuilder {
    pub fn new() -> Self {
        Self {
            instruction_parsers: Vec::new(),
            account_parsers: Vec::new(),
            secondary_filters: Vec::new(),
            fallback_topic: None,
        }
    }

    /// Add a Vixen parser with its program name and Kafka topic.
    ///
    /// Accepts both `Parser<Input=InstructionUpdate>` and `Parser<Input=AccountUpdate>`
    /// via the sealed `SinkInput` trait, dispatching to the right internal collection.
    pub fn parser<P, O>(mut self, parser: P, program_name: &str, topic: &str) -> Self
    where
        P: Parser + Send + Sync + 'static,
        P::Input: SinkInput,
        O: Message + Send + Sync + 'static,
        P: Parser<Output = O>,
    {
        P::Input::add_parser(
            parser,
            program_name.to_string(),
            topic.to_string(),
            None,
            &mut self,
        );
        self
    }

    /// Add a Vixen parser with a per-parser fallback topic.
    ///
    /// When parsing returns `Filtered`, the raw data is sent to the fallback topic
    /// instead of being silently dropped.
    pub fn parser_with_fallback<P, O>(
        mut self,
        parser: P,
        program_name: &str,
        topic: &str,
        fallback_topic: &str,
    ) -> Self
    where
        P: Parser + Send + Sync + 'static,
        P::Input: SinkInput,
        O: Message + Send + Sync + 'static,
        P: Parser<Output = O>,
    {
        P::Input::add_parser(
            parser,
            program_name.to_string(),
            topic.to_string(),
            Some(fallback_topic.to_string()),
            &mut self,
        );
        self
    }

    /// Set the fallback topic for instructions that no parser handles.
    /// If not set, unmatched instructions are silently dropped.
    pub fn fallback_topic(mut self, topic: &str) -> Self {
        self.fallback_topic = Some(topic.to_string());
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

    pub fn build(self) -> KafkaSink {
        KafkaSink {
            instruction_parsers: self.instruction_parsers,
            account_parsers: self.account_parsers,
            secondary_filters: self.secondary_filters,
            fallback_topic: self.fallback_topic,
            schema_ids: HashMap::new(),
        }
    }

    pub fn topics(&self) -> Vec<&str> {
        self.instruction_parsers
            .iter()
            .flat_map(|p| {
                std::iter::once(p.topic()).chain(p.fallback_topic())
            })
            .chain(
                self.account_parsers
                    .iter()
                    .flat_map(|p| {
                        std::iter::once(p.topic()).chain(p.fallback_topic())
                    }),
            )
            .chain(self.secondary_filters.iter().map(|f| f.topic()))
            .chain(self.fallback_topic.as_deref())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
}

// --- KafkaSink (formerly ConfiguredParsers) ---

#[derive(Clone)]
pub struct KafkaSink {
    instruction_parsers: Vec<Arc<dyn DynInstructionParser>>,
    account_parsers: Vec<Arc<dyn DynAccountParser>>,
    secondary_filters: Vec<Arc<dyn SecondaryFilter>>,
    fallback_topic: Option<String>,
    /// Map of topic -> schema info for encoding with Confluent wire format.
    schema_ids: HashMap<String, RegisteredSchema>,
}

impl Default for KafkaSink {
    fn default() -> Self {
        Self {
            instruction_parsers: Vec::new(),
            account_parsers: Vec::new(),
            secondary_filters: Vec::new(),
            fallback_topic: None,
            schema_ids: HashMap::new(),
        }
    }
}

impl KafkaSink {
    pub fn topics(&self) -> Vec<&str> {
        self.instruction_parsers
            .iter()
            .flat_map(|p| {
                std::iter::once(p.topic()).chain(p.fallback_topic())
            })
            .chain(
                self.account_parsers
                    .iter()
                    .flat_map(|p| {
                        std::iter::once(p.topic()).chain(p.fallback_topic())
                    }),
            )
            .chain(self.secondary_filters.iter().map(|f| f.topic()))
            .chain(self.fallback_topic.as_deref())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    pub fn fallback_topic(&self) -> Option<&str> { self.fallback_topic.as_deref() }

    /// Returns true if any account parsers are registered.
    pub fn has_account_parsers(&self) -> bool { !self.account_parsers.is_empty() }

    /// Returns the account parsers (used by AccountSubscription to build prefilter).
    pub fn account_parsers(&self) -> &[Arc<dyn DynAccountParser>] { &self.account_parsers }

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

    // --- Instruction parsing ---

    async fn try_parse_instruction(
        &self,
        ix: &InstructionUpdate,
    ) -> Option<(ParsedOutput, &str)> {
        for parser in &self.instruction_parsers {
            if let Some(parsed) = parser.try_parse(ix).await {
                return Some((parsed, parser.topic()));
            }
        }
        None
    }

    /// Parse an instruction and prepare a Kafka record.
    ///
    /// Tries each registered parser in order. If one matches, builds a decoded record;
    /// if none match and a fallback topic is configured, builds a fallback record.
    /// Returns `None` if no parser matched and no fallback topic is set.
    pub async fn parse_instruction(
        &self,
        slot: u64,
        signature: &[u8],
        path: &Path,
        ix: &InstructionUpdate,
    ) -> Option<(PreparedRecord, Option<ParsedOutput>)> {
        match self.try_parse_instruction(ix).await {
            Some((parsed, topic)) => {
                let record = self.prepare_decoded_instruction_record(
                    slot,
                    signature,
                    path,
                    parsed.clone(),
                    topic,
                );
                Some((record, Some(parsed)))
            },
            None => {
                let fallback = self.fallback_topic.as_deref()?;
                let record =
                    self.prepare_fallback_instruction_record(slot, signature, path, ix, fallback);
                Some((record, None))
            },
        }
    }

    pub fn secondary_filters(&self) -> &[Arc<dyn SecondaryFilter>] { &self.secondary_filters }

    /// Build the base headers and key shared by all instruction record types.
    fn instruction_base_record(
        slot: u64,
        signature: &[u8],
        path: &Path,
    ) -> (String, Vec<RecordHeader>) {
        let sig_str = bs58::encode(signature).into_string();
        let path_str = format!("{path:?}");
        let key = make_instruction_record_key(slot, &sig_str, &path_str);
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
    pub fn prepare_decoded_instruction_record(
        &self,
        slot: u64,
        signature: &[u8],
        path: &Path,
        parsed: ParsedOutput,
        topic: &str,
    ) -> PreparedRecord {
        let (key, headers) = Self::instruction_base_record(slot, signature, path);
        let payload = self.encode_payload_for_topic(topic, parsed.data);

        PreparedRecord {
            topic: topic.to_string(),
            payload,
            key,
            headers,
            is_decoded: true,
            kind: RecordKind::Instruction,
        }
    }

    /// Prepare a fallback record for unrecognized instructions.
    /// Payload is the raw instruction data, metadata in headers.
    pub fn prepare_fallback_instruction_record(
        &self,
        slot: u64,
        signature: &[u8],
        path: &Path,
        ix: &InstructionUpdate,
        fallback_topic: &str,
    ) -> PreparedRecord {
        let (key, mut headers) = Self::instruction_base_record(slot, signature, path);
        let program_id = bs58::encode(ix.program).into_string();

        headers.push(RecordHeader {
            key: "program_id",
            value: program_id,
        });

        PreparedRecord {
            topic: fallback_topic.to_string(),
            payload: ix.data.clone(),
            key,
            headers,
            is_decoded: false,
            kind: RecordKind::Instruction,
        }
    }

    // --- Subscription constructors ---

    /// Create a TransactionSubscription for this sink.
    pub fn transaction_subscription(&self) -> crate::parsers::TransactionSubscription {
        crate::parsers::TransactionSubscription
    }

    /// Create an AccountSubscription for this sink, if any account parsers are registered.
    /// Returns `None` if no account parsers were configured.
    pub fn account_subscription(&self) -> Option<crate::parsers::AccountSubscription> {
        crate::parsers::AccountSubscription::new(self)
    }

    // --- Account parsing ---

    /// Parse an account update and prepare a Kafka record.
    ///
    /// Tries each registered account parser. On match, builds a decoded account record.
    /// On `Filtered` (no match from a specific parser), checks if that parser has a
    /// fallback_topic. If no parser matches at all, returns `None` (silently skip).
    pub async fn parse_account(
        &self,
        slot: u64,
        acct: &AccountUpdate,
    ) -> Option<PreparedRecord> {
        let inner = acct.account.as_ref()?;
        let pubkey_str = bs58::encode(&inner.pubkey).into_string();
        let owner_str = bs58::encode(&inner.owner).into_string();

        for parser in &self.account_parsers {
            match parser.try_parse(acct).await {
                Some(parsed) => {
                    return Some(self.prepare_decoded_account_record(
                        slot,
                        &pubkey_str,
                        &owner_str,
                        parsed,
                        parser.topic(),
                    ));
                },
                None => {
                    // Parser filtered this account — check for per-parser fallback
                    if let Some(fallback) = parser.fallback_topic() {
                        return Some(self.prepare_fallback_account_record(
                            slot,
                            &pubkey_str,
                            &owner_str,
                            &inner.data,
                            fallback,
                        ));
                    }
                },
            }
        }
        // No parser matched and no fallback configured — silently skip.
        None
    }

    /// Prepare a record for a successfully decoded account.
    fn prepare_decoded_account_record(
        &self,
        slot: u64,
        pubkey: &str,
        owner: &str,
        parsed: ParsedOutput,
        topic: &str,
    ) -> PreparedRecord {
        let key = make_account_record_key(slot, pubkey);
        let payload = self.encode_payload_for_topic(topic, parsed.data);

        let headers = vec![
            RecordHeader {
                key: "slot",
                value: slot.to_string(),
            },
            RecordHeader {
                key: "pubkey",
                value: pubkey.to_string(),
            },
            RecordHeader {
                key: "owner",
                value: owner.to_string(),
            },
        ];

        PreparedRecord {
            topic: topic.to_string(),
            payload,
            key,
            headers,
            is_decoded: true,
            kind: RecordKind::Account,
        }
    }

    /// Prepare a fallback record for accounts that a parser filtered out.
    fn prepare_fallback_account_record(
        &self,
        slot: u64,
        pubkey: &str,
        owner: &str,
        data: &[u8],
        fallback_topic: &str,
    ) -> PreparedRecord {
        let key = make_account_record_key(slot, pubkey);

        let headers = vec![
            RecordHeader {
                key: "slot",
                value: slot.to_string(),
            },
            RecordHeader {
                key: "pubkey",
                value: pubkey.to_string(),
            },
            RecordHeader {
                key: "owner",
                value: owner.to_string(),
            },
        ];

        PreparedRecord {
            topic: fallback_topic.to_string(),
            payload: data.to_vec(),
            key,
            headers,
            is_decoded: false,
            kind: RecordKind::Account,
        }
    }
}
