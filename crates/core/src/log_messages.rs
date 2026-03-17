//! Utilities for splitting and assigning Solana transaction log messages
//! to individual instructions.

use super::instruction::InstructionUpdate;

///
/// Walk the transaction log messages and assign each instruction's log slice.
///
/// Solana logs follow a strict nesting structure:
///
/// ```text
/// Program ABC invoke [1]      ← outer instruction 0
///   Program log: hello
///   Program DEF invoke [2]    ← inner instruction 0.0
///     Program log: inner
///   Program DEF success
/// Program ABC success
/// ```
///
/// Each instruction receives all log lines from its `invoke` through its
/// `success`/`failed` (inclusive), including lines from inner instructions.
///
pub(crate) fn assign_log_messages(logs: &[String], outer: &mut [InstructionUpdate]) {
    if logs.is_empty() || outer.is_empty() {
        return;
    }

    let mut cursor = 0;

    for ix in outer.iter_mut() {
        cursor = assign_logs_recursive(logs, cursor, ix);
    }
}

/// Recursively assign log lines to an instruction and its inner instructions.
///
/// Returns the new cursor position (index of the first unconsumed log line).
fn assign_logs_recursive(logs: &[String], start: usize, ix: &mut InstructionUpdate) -> usize {
    // Find the invoke line for this instruction.
    let Some(invoke_pos) = find_invoke(logs, start) else {
        return start;
    };

    // Walk forward from the invoke line, tracking depth to find the matching
    // success/failed line that closes this instruction.
    let mut depth: u32 = 1;
    let mut pos = invoke_pos + 1;
    let mut inner_idx = 0;

    while pos < logs.len() && depth > 0 {
        let line = &logs[pos];

        if line.starts_with("Program ") {
            if line.contains(" invoke [") {
                // Entering a nested instruction — assign logs to the
                // corresponding inner instruction if one exists.
                if inner_idx < ix.inner.len() {
                    pos = assign_logs_recursive(logs, pos, &mut ix.inner[inner_idx]);

                    inner_idx += 1;

                    continue;
                }

                depth += 1;
            } else if line.ends_with(" success") || line.contains(" failed:") {
                depth -= 1;

                if depth == 0 {
                    // This is the closing line for the current instruction.
                    ix.log_messages = logs[invoke_pos..=pos].to_vec();

                    return pos + 1;
                }
            }
        }

        pos += 1;
    }

    // Fallback: if we never found a matching close, take everything from invoke to end.
    ix.log_messages = logs[invoke_pos..pos].to_vec();

    pos
}

/// Find the next `Program ... invoke [N]` line at or after `start`.
fn find_invoke(logs: &[String], start: usize) -> Option<usize> {
    logs.iter()
        .enumerate()
        .skip(start)
        .find(|(_, line)| line.starts_with("Program ") && line.contains(" invoke ["))
        .map(|(i, _)| i)
}

///
/// Split transaction logs by outer instruction index.
///
/// Returns a vec where entry `i` contains the log lines for outer instruction `i`
/// (from its `Program ... invoke [1]` through its `Program ... success`/`failed`).
///
#[must_use]
pub fn split_logs_by_outer_ix(logs: &[String]) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    let mut depth: u32 = 0;

    for line in logs {
        if line.starts_with("Program ") && line.contains(" invoke [1]") && depth == 0 {
            if !current.is_empty() {
                result.push(std::mem::take(&mut current));
            }

            depth = 1;

            current.push(line.clone());
        } else if depth > 0 {
            if line.starts_with("Program ") && line.contains(" invoke [") {
                depth += 1;
            } else if line.starts_with("Program ")
                && (line.ends_with(" success") || line.contains(" failed:"))
            {
                depth -= 1;
            }

            current.push(line.clone());
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{
        instruction::{InstructionShared, Path},
        KeyBytes,
    };

    #[test]
    fn test_assign_log_messages() {
        let shared = Arc::new(InstructionShared::default());

        // Build a simple instruction tree: outer ix with one inner ix
        let mut outer = vec![InstructionUpdate {
            program: KeyBytes::new([1; 32]),
            accounts: vec![],
            data: vec![],
            shared: Arc::clone(&shared),
            inner: vec![InstructionUpdate {
                program: KeyBytes::new([2; 32]),
                accounts: vec![],
                data: vec![],
                shared: Arc::clone(&shared),
                inner: vec![],
                path: Path::from(vec![0, 0]),
                log_messages: vec![],
            }],
            path: Path::new_single(0),
            log_messages: vec![],
        }];

        let logs: Vec<String> = vec![
            "Program ABC invoke [1]",
            "Program log: outer hello",
            "Program DEF invoke [2]",
            "Program log: inner hello",
            "Program DEF success",
            "Program ABC consumed 5000 units",
            "Program ABC success",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assign_log_messages(&logs, &mut outer);

        // Outer instruction gets all logs (invoke through success, inclusive)
        assert_eq!(outer[0].log_messages.len(), 7);
        assert_eq!(outer[0].log_messages[0], "Program ABC invoke [1]");
        assert_eq!(outer[0].log_messages[6], "Program ABC success");

        // Inner instruction gets its own slice
        assert_eq!(outer[0].inner[0].log_messages.len(), 3);
        assert_eq!(outer[0].inner[0].log_messages[0], "Program DEF invoke [2]");
        assert_eq!(outer[0].inner[0].log_messages[2], "Program DEF success");
    }

    #[test]
    fn test_assign_log_messages_multiple_outer() {
        let shared = Arc::new(InstructionShared::default());

        let mut outer = vec![
            InstructionUpdate {
                program: KeyBytes::new([1; 32]),
                accounts: vec![],
                data: vec![],
                shared: Arc::clone(&shared),
                inner: vec![],
                path: Path::new_single(0),
                log_messages: vec![],
            },
            InstructionUpdate {
                program: KeyBytes::new([2; 32]),
                accounts: vec![],
                data: vec![],
                shared: Arc::clone(&shared),
                inner: vec![],
                path: Path::new_single(1),
                log_messages: vec![],
            },
        ];

        let logs: Vec<String> = vec![
            "Program ABC invoke [1]",
            "Program log: first",
            "Program ABC success",
            "Program DEF invoke [1]",
            "Program log: second",
            "Program DEF success",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assign_log_messages(&logs, &mut outer);

        assert_eq!(outer[0].log_messages.len(), 3);
        assert_eq!(outer[0].log_messages[1], "Program log: first");

        assert_eq!(outer[1].log_messages.len(), 3);
        assert_eq!(outer[1].log_messages[1], "Program log: second");
    }
}
