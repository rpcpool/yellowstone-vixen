use std::{fs, path::Path};

use codama_nodes::{InstructionNode, RootNode};

#[derive(Debug)]
pub enum IdlError {
    ReadFile(std::io::Error),
    ParseFile(serde_json::Error),
}

impl std::fmt::Display for IdlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdlError::ReadFile(e) => write!(f, "Failed to read file: {}", e),
            IdlError::ParseFile(e) => write!(f, "Failed to parse JSON: {}", e),
        }
    }
}

/// Load a codama IDL file, returning the root node and any events.
///
/// The `events` array is extracted separately because `codama_nodes::ProgramNode`
/// (v0.6) does not have an `events` field. We parse the JSON as `serde_json::Value`
/// first to extract `program.events`, then deserialize the rest as `RootNode`
/// (which silently ignores unknown fields).
pub fn load_codama_idl<P: AsRef<Path>>(
    path: P,
) -> Result<(RootNode, Vec<InstructionNode>), IdlError> {
    let data = fs::read_to_string(&path).map_err(IdlError::ReadFile)?;

    let value: serde_json::Value =
        serde_json::from_str(&data).map_err(IdlError::ParseFile)?;

    let events = value
        .get("program")
        .and_then(|p| p.get("events"))
        .map(|e| serde_json::from_value::<Vec<InstructionNode>>(e.clone()))
        .transpose()
        .map_err(IdlError::ParseFile)?
        .unwrap_or_default();

    let root = serde_json::from_value::<RootNode>(value).map_err(IdlError::ParseFile)?;

    Ok((root, events))
}
