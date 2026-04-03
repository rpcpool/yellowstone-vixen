use std::{fs, path::Path};

use codama_nodes::{EventNode, RootNode};

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
/// Events are tried first from `program.events` as `EventNode`s (codama v0.8+).
/// If that fails (e.g. old IDLs with `kind: "instructionNode"` events), the
/// events field is ignored and an empty vec is returned — the IDL still loads
/// for instruction/account parsing.
pub fn load_codama_idl<P: AsRef<Path>>(path: P) -> Result<(RootNode, Vec<EventNode>), IdlError> {
    let data = fs::read_to_string(&path).map_err(IdlError::ReadFile)?;

    let mut value: serde_json::Value = serde_json::from_str(&data).map_err(IdlError::ParseFile)?;

    // Some codama generators emit error codes as strings ("6000") instead of
    // integers (6000). Coerce them before deserialization so `ErrorNode.code`
    // (which expects `usize`) doesn't fail.
    fix_string_error_codes(&mut value);

    // Try parsing directly — works when events use `kind: "eventNode"` or are absent.
    if let Ok(root) = serde_json::from_value::<RootNode>(value.clone()) {
        let events = root.program.events.clone();
        return Ok((root, events));
    }

    // Fallback: strip the events field and parse without it.
    // This handles IDLs with old-format events (`kind: "instructionNode"`).
    if let Some(program) = value.get_mut("program") {
        program.as_object_mut().map(|obj| obj.remove("events"));
    }

    let root = serde_json::from_value::<RootNode>(value).map_err(IdlError::ParseFile)?;

    Ok((root, Vec::new()))
}

/// Coerce `"code": "6000"` → `"code": 6000` in `program.errors[]`.
fn fix_string_error_codes(value: &mut serde_json::Value) {
    let errors = value
        .get_mut("program")
        .and_then(|p| p.get_mut("errors"))
        .and_then(|e| e.as_array_mut());

    let Some(errors) = errors else { return };

    for error in errors {
        if let Some(code) = error.get_mut("code")
            && let Some(s) = code.as_str().and_then(|s| s.parse::<u64>().ok())
        {
            *code = serde_json::Value::Number(s.into());
        }
    }
}
