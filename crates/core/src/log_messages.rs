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

    /// Real-world transaction with 7 outer instructions:
    ///   0: ComputeBudget (2 lines)
    ///   1: ComputeBudget (2 lines)
    ///   2: AToken CreateIdempotent (4 lines)
    ///   3: AToken CreateIdempotent (4 lines)
    ///   4: j1o2q PreFlashFillOrder with 1 inner CPI (8 lines)
    ///   5: JUP SharedAccountsRoute with deep nesting (43 lines)
    ///   6: j1o2q FlashFillOrder with 3 inner CPIs (15 lines)
    #[test]
    fn test_split_logs_by_outer_ix_real_world() {
        let logs: Vec<String> = vec![
            "Program ComputeBudget111111111111111111111111111111 invoke [1]",
            "Program ComputeBudget111111111111111111111111111111 success",
            "Program ComputeBudget111111111111111111111111111111 invoke [1]",
            "Program ComputeBudget111111111111111111111111111111 success",
            "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [1]",
            "Program log: CreateIdempotent",
            "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 4338 of 303918 compute \
             units",
            "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success",
            "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [1]",
            "Program log: CreateIdempotent",
            "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 5937 of 299580 compute \
             units",
            "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success",
            "Program j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X invoke [1]",
            "Program log: Instruction: PreFlashFillOrder",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
            "Program log: Instruction: TransferChecked",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6147 of 278694 compute \
             units",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
            "Program j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X consumed 22619 of 293643 compute \
             units",
            "Program j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X success",
            "Program JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 invoke [1]",
            "Program log: Instruction: SharedAccountsRoute",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
            "Program log: Instruction: Transfer",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4644 of 267311 compute \
             units",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
            "Program 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 invoke [2]",
            "Program log: ray_log: \
             A9cDAhYAAAAAAAAAAAAAAAACAAAAAAAAAIrXBBYAAAAAOAp4Z0kAAAAQzh3GLQAAABdtrA0AAAAA",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
            "Program log: Instruction: Transfer",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 246681 compute \
             units",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
            "Program log: Instruction: Transfer",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4736 of 239620 compute \
             units",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
            "Program 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 consumed 25622 of 259931 \
             compute units",
            "Program 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 success",
            "Program JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 invoke [2]",
            "Program JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 consumed 187 of 232746 compute \
             units",
            "Program JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 success",
            "Program goonuddtQRrWqqn5nFyczVKaie28f3kDkHWkHtURSLE invoke [2]",
            "Program log: 00000000181cff6c-00000000192afc86-0000000000000006-ee",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
            "Program log: Instruction: Transfer",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4736 of 196945 compute \
             units",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
            "Program log: Instruction: Transfer",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 190906 compute \
             units",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
            "Program goonuddtQRrWqqn5nFyczVKaie28f3kDkHWkHtURSLE consumed 82545 of 226760 compute \
             units",
            "Program goonuddtQRrWqqn5nFyczVKaie28f3kDkHWkHtURSLE success",
            "Program JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 invoke [2]",
            "Program JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 consumed 187 of 142649 compute \
             units",
            "Program JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 success",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
            "Program log: Instruction: Transfer",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 139952 compute \
             units",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
            "Program JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 consumed 135847 of 271024 \
             compute units",
            "Program return: JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 ygA2AQAAAAA=",
            "Program JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 success",
            "Program j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X invoke [1]",
            "Program log: Instruction: FlashFillOrder",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
            "Program log: Instruction: TransferChecked",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6200 of 122947 compute \
             units",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
            "Program log: Instruction: TransferChecked",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6200 of 114107 compute \
             units",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
            "Program j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X invoke [2]",
            "Program j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X consumed 2028 of 105058 compute \
             units",
            "Program j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X success",
            "Program j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X consumed 33773 of 135177 compute \
             units",
            "Program j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X success",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let splits = split_logs_by_outer_ix(&logs);

        assert_eq!(splits.len(), 7);

        // ix 0: ComputeBudget
        assert_eq!(splits[0].len(), 2);
        assert!(splits[0][0].contains("ComputeBudget"));
        assert!(splits[0][1].contains("success"));

        // ix 1: ComputeBudget
        assert_eq!(splits[1].len(), 2);
        assert!(splits[1][0].contains("ComputeBudget"));

        // ix 2: AToken CreateIdempotent
        assert_eq!(splits[2].len(), 4);
        assert!(splits[2][0].contains("ATokenGPv"));
        assert_eq!(splits[2][1], "Program log: CreateIdempotent");

        // ix 3: AToken CreateIdempotent
        assert_eq!(splits[3].len(), 4);
        assert!(splits[3][0].contains("ATokenGPv"));

        // ix 4: PreFlashFillOrder (1 inner CPI to Token program)
        assert_eq!(splits[4].len(), 8);
        assert!(splits[4][0].contains("j1o2q"));
        assert!(splits[4][1].contains("PreFlashFillOrder"));
        assert!(splits[4][7].contains("j1o2q") && splits[4][7].contains("success"));

        // ix 5: JUP SharedAccountsRoute (deep nesting: Raydium, JUP self-CPI, Goon, Token)
        assert_eq!(splits[5].len(), 43);
        assert!(splits[5][0].contains("JUP6Lkb"));
        assert!(splits[5][1].contains("SharedAccountsRoute"));
        assert!(splits[5][42].contains("JUP6Lkb") && splits[5][42].contains("success"));

        // ix 6: FlashFillOrder (3 inner CPIs)
        assert_eq!(splits[6].len(), 15);
        assert!(splits[6][0].contains("j1o2q"));
        assert!(splits[6][1].contains("FlashFillOrder"));
        assert!(splits[6][14].contains("j1o2q") && splits[6][14].contains("success"));

        // All log lines accounted for
        let total: usize = splits.iter().map(|s| s.len()).sum();
        assert_eq!(total, logs.len());
    }
}
