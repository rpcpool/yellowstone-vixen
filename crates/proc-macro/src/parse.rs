use std::{fs, path::Path};

use codama_nodes::RootNode;
use hex::FromHexError;

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

pub fn load_codama_idl<P: AsRef<Path>>(path: P) -> Result<RootNode, IdlError> {
    let data = fs::read_to_string(&path).map_err(IdlError::ReadFile)?;
    serde_json::from_str::<RootNode>(&data).map_err(IdlError::ParseFile)
}

pub fn hexdecode_relaxed(input: &str) -> Result<Vec<u8>, FromHexError> {
    if input.len() % 2 != 0 {
        let mut fixed = input.to_string();
        fixed.insert(0, '0');
        hex::decode(fixed)
    } else {
        hex::decode(input)
    }
}
