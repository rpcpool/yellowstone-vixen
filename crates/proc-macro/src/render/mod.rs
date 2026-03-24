/// Anchor CPI event tag: first 8 bytes of `sha256("anchor:event")`.
/// Used in generated code to identify self-CPI event instructions.
const ANCHOR_EVENT_IX_TAG: u64 = 0x1d9a_cb51_2ea5_45e4;

pub mod account_parser;
pub mod event_parser;
pub mod instruction_parser;
mod manual_prost;
pub mod program_pubkey;
pub mod proto_schema_string;
pub mod rust_types_from_ir;
pub mod vixen_parser;

pub use account_parser::account_parser;
pub use event_parser::event_parser;
pub use instruction_parser::instruction_parser;
pub use program_pubkey::program_pubkey;
pub use proto_schema_string::proto_schema_string;
pub use rust_types_from_ir::rust_types_from_ir;
pub use vixen_parser::vixen_parser;
