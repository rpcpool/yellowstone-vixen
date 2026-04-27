use codama_nodes::{EventNode, RootNode};

use crate::intermediate_representation::SchemaIr;

/// Build an intermediate representation of the IDL schema that we can use to render .proto and rust code.
///
/// Why use IR? So we can unify the parsing. Then we have one source of truth for both the .proto renderer and Rust code.
pub fn build_schema_ir(idl: &RootNode, events: &[EventNode]) -> SchemaIr {
    let mut ir = SchemaIr {
        types: Vec::new(),
        oneofs: Vec::new(),
        type_aliases: std::collections::HashMap::new(),
    };

    crate::intermediate_representation::build_defined_types(&idl.program.defined_types, &mut ir);

    crate::intermediate_representation::build_accounts(&idl.program.accounts, &mut ir);

    crate::intermediate_representation::build_instructions_schema(
        &idl.program.instructions,
        &mut ir,
    );

    if cfg!(feature = "program-events") && !events.is_empty() {
        crate::intermediate_representation::build_events_schema(events, &mut ir);
    }

    ir
}
