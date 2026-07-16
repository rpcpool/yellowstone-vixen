use codama_nodes::{EventNode, RootNode};

use crate::intermediate_representation::SchemaIr;

/// Build an intermediate representation of the IDL schema that we can use to render .proto and rust code.
///
/// Why use IR? So we can unify the parsing. Then we have one source of truth for both the .proto renderer and Rust code.
pub fn build_schema_ir(idl: &RootNode, events: &[EventNode]) -> SchemaIr {
    let mut ir = SchemaIr::default();

    reserve_direct_message_names(idl, events, &mut ir);

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

/// Reserve names that are emitted directly by the account, instruction, and
/// event builders before inline helpers are materialized. Helpers are the only
/// generated messages whose names can be changed without changing parser APIs;
/// direct messages must keep their established names.
fn reserve_direct_message_names(idl: &RootNode, events: &[EventNode], ir: &mut SchemaIr) {
    let program_name = crate::utils::to_pascal_case(&idl.program.name);
    let mut names = vec!["PublicKey".to_string()];

    for account in &idl.program.accounts {
        names.push(crate::utils::to_pascal_case(&account.name));
    }

    if !idl.program.accounts.is_empty() {
        let default_wrapper = format!("{program_name}Account");
        let wrapper_name = if idl
            .program
            .accounts
            .iter()
            .any(|account| crate::utils::to_pascal_case(&account.name) == default_wrapper)
        {
            format!("{program_name}AccountOutput")
        } else {
            default_wrapper
        };

        names.push(wrapper_name);
    }

    for instruction in &idl.program.instructions {
        let name = crate::utils::to_pascal_case(&instruction.name);

        names.extend([
            name.clone(),
            format!("{name}Accounts"),
            format!("{name}Args"),
        ]);
    }

    if !idl.program.instructions.is_empty() {
        names.push("Instructions".to_string());
    }

    if cfg!(feature = "program-events") && !events.is_empty() {
        for event in events {
            let name = crate::utils::to_pascal_case(&event.name);

            names.extend([
                name.clone(),
                format!("{name}Accounts"),
                format!("{name}Args"),
            ]);
        }

        names.push("ProgramEvents".to_string());

        if !idl.program.instructions.is_empty() {
            names.push("ProgramEventOutput".to_string());
        }
    }

    ir.reserve_type_names(names);
}
