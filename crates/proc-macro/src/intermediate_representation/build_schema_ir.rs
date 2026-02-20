use codama_nodes::RootNode;

use crate::intermediate_representation::SchemaIr;

/// Build an intermediate representation of the IDL schema that we can use to render .proto and rust code.
///
/// Why use IR? So we can unify the parsing. Then we have one source of truth for both the .proto renderer and Rust code.
pub fn build_schema_ir(idl: &RootNode) -> SchemaIr {
    let mut ir = SchemaIr {
        types: Vec::new(),
        oneofs: Vec::new(),
    };

    crate::intermediate_representation::build_defined_types(&idl.program.defined_types, &mut ir);

    crate::intermediate_representation::build_accounts(&idl.program.accounts, &mut ir);

    crate::intermediate_representation::build_instructions_schema(
        &idl.program.instructions,
        &mut ir,
    );

    ir
}
