use std::{collections::HashSet, fs, path::Path};

use codama_nodes::RootNode;

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

pub struct ParsedIdl {
    pub root: RootNode,
    /// Instruction names (camelCase, as they appear in the IDL) that have `"isEvent": true`.
    pub event_names: HashSet<String>,
}

pub fn load_codama_idl<P: AsRef<Path>>(path: P) -> Result<ParsedIdl, IdlError> {
    let data = fs::read_to_string(&path).map_err(IdlError::ReadFile)?;

    // Pre-pass: extract instruction names with `isEvent: true` from raw JSON.
    // codama-nodes doesn't have the `isEvent` field, so we grab it ourselves.
    let event_names = {
        let raw: serde_json::Value =
            serde_json::from_str(&data).map_err(IdlError::ParseFile)?;

        raw.pointer("/program/instructions")
            .and_then(|v| v.as_array())
            .map(|instructions| {
                instructions
                    .iter()
                    .filter(|ix| ix.get("isEvent").and_then(|v| v.as_bool()).unwrap_or(false))
                    .filter_map(|ix| ix.get("name").and_then(|v| v.as_str()).map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    };

    let root = serde_json::from_str::<RootNode>(&data).map_err(IdlError::ParseFile)?;

    Ok(ParsedIdl { root, event_names })
}
