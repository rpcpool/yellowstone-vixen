//! Utilities for splitting and assigning Solana transaction log messages
//! to individual instructions.

use super::instruction::InstructionUpdate;

/// Classification of a single Solana transaction log line.
///
/// Solana's runtime emits log lines in three structural shapes the
/// instruction-attribution code cares about:
///
/// - `Program <pubkey> invoke [<n>]` — opens a program invocation at depth `n`.
/// - `Program <pubkey> success` / `Program <pubkey> failed: <reason>` — closes one.
/// - everything else (`Program log: ...`, `Program data: ...`,
///   `Program return: ...`, `Program <pubkey> consumed N of M compute units`,
///   user payloads, etc.) — depth is unchanged.
///
/// Classification is structural, not substring-based: a `Program log:` payload
/// containing the literal text `" invoke [2]"` is `Other`, not `Invoke`.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LogLineKind {
    /// `Program <pubkey> invoke [<n>]`
    Invoke,
    /// `Program <pubkey> success` or `Program <pubkey> failed: ...`
    Close,
    /// Anything else.
    Other,
}

/// Classify a Solana log line.
///
/// A line is only an `Invoke` or `Close` if its second whitespace-separated
/// token is a real pubkey-shaped string — i.e. *not* one of the reserved
/// runtime tags `log:`, `data:`, `return:`. This rejects adversarial
/// `Program log: ... invoke [2]` payloads from being misclassified.
pub(crate) fn classify_log_line(line: &str) -> LogLineKind {
    let Some(rest) = line.strip_prefix("Program ") else {
        return LogLineKind::Other;
    };

    // Second token must be a pubkey, not a runtime-reserved tag.
    let Some((second, tail)) = rest.split_once(' ') else {
        return LogLineKind::Other;
    };
    if matches!(second, "log:" | "data:" | "return:") {
        return LogLineKind::Other;
    }

    // Invoke shape: `invoke [<digits>]`.
    if let Some(after_invoke) = tail.strip_prefix("invoke [")
        && let Some(digits) = after_invoke.split(']').next()
        && !digits.is_empty()
        && digits.chars().all(|c| c.is_ascii_digit())
    {
        return LogLineKind::Invoke;
    }

    // Close shape: literal `success` or starts with `failed:`.
    if tail == "success" || tail.starts_with("failed:") {
        return LogLineKind::Close;
    }

    LogLineKind::Other
}

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
        match classify_log_line(&logs[pos]) {
            LogLineKind::Invoke => {
                // Entering a nested instruction — assign logs to the
                // corresponding inner instruction if one exists.
                if inner_idx < ix.inner.len() {
                    pos = assign_logs_recursive(logs, pos, &mut ix.inner[inner_idx]);
                    inner_idx += 1;
                    continue;
                }
                depth += 1;
            },
            LogLineKind::Close => {
                depth -= 1;
                if depth == 0 {
                    ix.log_range = invoke_pos..(pos + 1);
                    return pos + 1;
                }
            },
            LogLineKind::Other => {},
        }

        pos += 1;
    }

    // Fallback: if we never found a matching close, take everything from invoke to end.
    ix.log_range = invoke_pos..pos;

    pos
}

/// Find the next real `Program <pubkey> invoke [N]` line at or after `start`,
/// ignoring adversarial `Program log:` payloads that contain that substring.
fn find_invoke(logs: &[String], start: usize) -> Option<usize> {
    logs.iter()
        .enumerate()
        .skip(start)
        .find(|(_, line)| classify_log_line(line) == LogLineKind::Invoke)
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
        let kind = classify_log_line(line);
        let is_outer_invoke = matches!(kind, LogLineKind::Invoke) && line.contains(" invoke [1]");

        if is_outer_invoke && depth == 0 {
            if !current.is_empty() {
                result.push(std::mem::take(&mut current));
            }

            depth = 1;

            current.push(line.clone());
        } else if depth > 0 {
            match kind {
                LogLineKind::Invoke => depth += 1,
                LogLineKind::Close => depth -= 1,
                LogLineKind::Other => {},
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

        let shared = Arc::new(InstructionShared {
            log_messages: logs.clone(),
            ..InstructionShared::default()
        });

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
                log_range: 0..0,
            }],
            path: Path::new_single(0),
            log_range: 0..0,
        }];

        assign_log_messages(&shared.log_messages, &mut outer);

        // Outer instruction gets all logs (invoke through success, inclusive)
        assert_eq!(outer[0].log_messages().len(), 7);
        assert_eq!(outer[0].log_messages()[0], "Program ABC invoke [1]");
        assert_eq!(outer[0].log_messages()[6], "Program ABC success");

        // Inner instruction gets its own slice
        assert_eq!(outer[0].inner[0].log_messages().len(), 3);
        assert_eq!(
            outer[0].inner[0].log_messages()[0],
            "Program DEF invoke [2]"
        );
        assert_eq!(outer[0].inner[0].log_messages()[2], "Program DEF success");
    }

    #[test]
    fn test_assign_log_messages_multiple_outer() {
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

        let shared = Arc::new(InstructionShared {
            log_messages: logs.clone(),
            ..InstructionShared::default()
        });

        let mut outer = vec![
            InstructionUpdate {
                program: KeyBytes::new([1; 32]),
                accounts: vec![],
                data: vec![],
                shared: Arc::clone(&shared),
                inner: vec![],
                path: Path::new_single(0),
                log_range: 0..0,
            },
            InstructionUpdate {
                program: KeyBytes::new([2; 32]),
                accounts: vec![],
                data: vec![],
                shared: Arc::clone(&shared),
                inner: vec![],
                path: Path::new_single(1),
                log_range: 0..0,
            },
        ];

        assign_log_messages(&shared.log_messages, &mut outer);

        assert_eq!(outer[0].log_messages().len(), 3);
        assert_eq!(outer[0].log_messages()[1], "Program log: first");

        assert_eq!(outer[1].log_messages().len(), 3);
        assert_eq!(outer[1].log_messages()[1], "Program log: second");
    }

    /// Regression for issue #186 / PR #187 follow-up: a handler asking for
    /// "logs my program emitted" must not see lines from inner CPI invocations.
    ///
    /// `log_messages()` returns the full nested range (invoke through success
    /// inclusive, including inner CPI lines). `direct_log_messages()` returns
    /// only the lines emitted while *this* program is at the top of the
    /// invocation stack — the CPI-aware view that DEX handlers (Raydium,
    /// Boop, Pump) need to scrape `Program log:` trade results without
    /// pulling in token-program transfer logs.
    #[test]
    fn direct_log_messages_excludes_inner_cpi_lines() {
        let logs: Vec<String> = vec![
            "Program RAY invoke [1]",
            "Program log: ray_log: swap-base-in",
            "Program TOKEN invoke [2]",
            "Program log: Instruction: Transfer",
            "Program TOKEN success",
            "Program log: ray_log: swap done",
            "Program RAY success",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let shared = Arc::new(InstructionShared {
            log_messages: logs.clone(),
            ..InstructionShared::default()
        });

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
                log_range: 0..0,
            }],
            path: Path::new_single(0),
            log_range: 0..0,
        }];

        assign_log_messages(&shared.log_messages, &mut outer);

        let direct: Vec<&str> = outer[0].direct_log_messages().collect();

        assert_eq!(
            direct,
            vec![
                "Program RAY invoke [1]",
                "Program log: ray_log: swap-base-in",
                "Program log: ray_log: swap done",
                "Program RAY success",
            ],
            "direct_log_messages must skip inner CPI lines"
        );

        let inner_direct: Vec<&str> = outer[0].inner[0].direct_log_messages().collect();
        assert_eq!(
            inner_direct,
            vec![
                "Program TOKEN invoke [2]",
                "Program log: Instruction: Transfer",
                "Program TOKEN success",
            ],
            "inner ix direct_log_messages contains only its own lines"
        );
    }

    /// Edge case: inner CPI ends with `failed:` rather than `success`. The
    /// closing line still pops depth, so the outer program's trailing line
    /// must be visible.
    #[test]
    fn direct_log_messages_handles_failed_inner_cpi() {
        let logs: Vec<String> = vec![
            "Program RAY invoke [1]",
            "Program TOKEN invoke [2]",
            "Program TOKEN failed: custom error",
            "Program log: ray_log: recovering",
            "Program RAY success",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let shared = Arc::new(InstructionShared {
            log_messages: logs.clone(),
            ..InstructionShared::default()
        });

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
                log_range: 0..0,
            }],
            path: Path::new_single(0),
            log_range: 0..0,
        }];

        assign_log_messages(&shared.log_messages, &mut outer);

        let direct: Vec<&str> = outer[0].direct_log_messages().collect();
        assert_eq!(direct, vec![
            "Program RAY invoke [1]",
            "Program log: ray_log: recovering",
            "Program RAY success",
        ]);
    }

    /// Iter 3: when an outer ix has no inner CPIs, `direct_log_messages()`
    /// must equal `log_messages()` exactly.
    #[test]
    fn direct_log_messages_flat_ix_equals_log_messages() {
        let logs: Vec<String> = vec![
            "Program SOLO invoke [1]",
            "Program log: nothing nested here",
            "Program log: another line",
            "Program SOLO success",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let shared = Arc::new(InstructionShared {
            log_messages: logs.clone(),
            ..InstructionShared::default()
        });

        let mut outer = vec![InstructionUpdate {
            program: KeyBytes::new([1; 32]),
            accounts: vec![],
            data: vec![],
            shared: Arc::clone(&shared),
            inner: vec![],
            path: Path::new_single(0),
            log_range: 0..0,
        }];

        assign_log_messages(&shared.log_messages, &mut outer);

        let full: Vec<&str> = outer[0].log_messages().iter().map(String::as_str).collect();
        let direct: Vec<&str> = outer[0].direct_log_messages().collect();
        assert_eq!(direct, full, "flat ix: direct must equal full");
    }

    /// Iter 4: A → B → C three-level CPI. Outer A's direct view must skip
    /// every line emitted while B or C are on the stack.
    #[test]
    fn direct_log_messages_filters_three_deep_cpi() {
        let logs: Vec<String> = vec![
            "Program A invoke [1]",
            "Program log: A start",
            "Program B invoke [2]",
            "Program log: B start",
            "Program C invoke [3]",
            "Program log: C only",
            "Program C success",
            "Program log: B middle",
            "Program B success",
            "Program log: A end",
            "Program A success",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let shared = Arc::new(InstructionShared {
            log_messages: logs.clone(),
            ..InstructionShared::default()
        });

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
                inner: vec![InstructionUpdate {
                    program: KeyBytes::new([3; 32]),
                    accounts: vec![],
                    data: vec![],
                    shared: Arc::clone(&shared),
                    inner: vec![],
                    path: Path::from(vec![0, 0, 0]),
                    log_range: 0..0,
                }],
                path: Path::from(vec![0, 0]),
                log_range: 0..0,
            }],
            path: Path::new_single(0),
            log_range: 0..0,
        }];

        assign_log_messages(&shared.log_messages, &mut outer);

        let direct: Vec<&str> = outer[0].direct_log_messages().collect();
        assert_eq!(direct, vec![
            "Program A invoke [1]",
            "Program log: A start",
            "Program log: A end",
            "Program A success",
        ]);
    }

    /// Iter 5: an outer ix's own `Program data:` line is preserved; an inner
    /// CPI's `Program data:` line is dropped. This is the core guarantee for
    /// Anchor-event handlers that decode payloads from depth-1 lines.
    #[test]
    fn direct_log_messages_keeps_own_program_data_drops_inner() {
        let logs: Vec<String> = vec![
            "Program OUTER invoke [1]",
            "Program data: OUTER_PAYLOAD_BASE64==",
            "Program INNER invoke [2]",
            "Program data: INNER_PAYLOAD_BASE64==",
            "Program INNER success",
            "Program OUTER success",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let shared = Arc::new(InstructionShared {
            log_messages: logs.clone(),
            ..InstructionShared::default()
        });

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
                log_range: 0..0,
            }],
            path: Path::new_single(0),
            log_range: 0..0,
        }];

        assign_log_messages(&shared.log_messages, &mut outer);

        let direct: Vec<&str> = outer[0].direct_log_messages().collect();
        assert!(
            direct.contains(&"Program data: OUTER_PAYLOAD_BASE64=="),
            "outer Program data: must be preserved"
        );
        assert!(
            !direct.contains(&"Program data: INNER_PAYLOAD_BASE64=="),
            "inner Program data: must be filtered out"
        );
    }

    /// Iter 6: `Program return: <id> <data>` lines (real txns place these
    /// just before `success`) are at depth 1 and must be kept. They aren't
    /// invokes and aren't closes.
    #[test]
    fn direct_log_messages_keeps_program_return() {
        let logs: Vec<String> = vec![
            "Program OUTER invoke [1]",
            "Program return: OUTER 8fo2SxUAAAA=",
            "Program OUTER success",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let shared = Arc::new(InstructionShared {
            log_messages: logs.clone(),
            ..InstructionShared::default()
        });

        let mut outer = vec![InstructionUpdate {
            program: KeyBytes::new([1; 32]),
            accounts: vec![],
            data: vec![],
            shared: Arc::clone(&shared),
            inner: vec![],
            path: Path::new_single(0),
            log_range: 0..0,
        }];

        assign_log_messages(&shared.log_messages, &mut outer);

        let direct: Vec<&str> = outer[0].direct_log_messages().collect();
        assert_eq!(direct, vec![
            "Program OUTER invoke [1]",
            "Program return: OUTER 8fo2SxUAAAA=",
            "Program OUTER success",
        ]);
    }

    /// Iter 7: adversarial input — a `Program log:` payload that contains
    /// the substring `" invoke ["`. The depth tracker must not be fooled
    /// by user-supplied log content.
    #[test]
    fn direct_log_messages_ignores_invoke_in_program_log_payload() {
        let logs: Vec<String> = vec![
            "Program OUTER invoke [1]",
            "Program log: before",
            "Program log: this looks like invoke [2] but is not",
            "Program log: after",
            "Program OUTER success",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let shared = Arc::new(InstructionShared {
            log_messages: logs.clone(),
            ..InstructionShared::default()
        });

        let mut outer = vec![InstructionUpdate {
            program: KeyBytes::new([1; 32]),
            accounts: vec![],
            data: vec![],
            shared: Arc::clone(&shared),
            inner: vec![],
            path: Path::new_single(0),
            log_range: 0..0,
        }];

        assign_log_messages(&shared.log_messages, &mut outer);

        let direct: Vec<&str> = outer[0].direct_log_messages().collect();
        assert_eq!(
            direct,
            vec![
                "Program OUTER invoke [1]",
                "Program log: before",
                "Program log: this looks like invoke [2] but is not",
                "Program log: after",
                "Program OUTER success",
            ],
            "Program log: payload must not be misclassified as an invoke"
        );
    }

    /// Iter 9: same class of bug — `Program log:` payload containing
    /// `" success"` must not be misclassified as a close.
    #[test]
    fn direct_log_messages_ignores_success_in_program_log_payload() {
        let logs: Vec<String> = vec![
            "Program OUTER invoke [1]",
            "Program log: oh what a success",
            "Program log: still going",
            "Program OUTER success",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let shared = Arc::new(InstructionShared {
            log_messages: logs.clone(),
            ..InstructionShared::default()
        });

        let mut outer = vec![InstructionUpdate {
            program: KeyBytes::new([1; 32]),
            accounts: vec![],
            data: vec![],
            shared: Arc::clone(&shared),
            inner: vec![],
            path: Path::new_single(0),
            log_range: 0..0,
        }];

        assign_log_messages(&shared.log_messages, &mut outer);

        let direct: Vec<&str> = outer[0].direct_log_messages().collect();
        assert_eq!(
            direct,
            vec![
                "Program OUTER invoke [1]",
                "Program log: oh what a success",
                "Program log: still going",
                "Program OUTER success",
            ],
            "Program log: payload must not be misclassified as a close"
        );
    }

    /// Direct unit coverage of the line classifier. Pins the structural
    /// rules independent of the higher-level filtering logic.
    #[test]
    fn classify_log_line_covers_all_shapes() {
        // Real invokes
        assert_eq!(
            classify_log_line("Program ABC invoke [1]"),
            LogLineKind::Invoke
        );
        assert_eq!(
            classify_log_line("Program ABC invoke [2]"),
            LogLineKind::Invoke
        );
        assert_eq!(
            classify_log_line("Program ABC invoke [99]"),
            LogLineKind::Invoke
        );

        // Real closes
        assert_eq!(classify_log_line("Program ABC success"), LogLineKind::Close);
        assert_eq!(
            classify_log_line("Program ABC failed: custom error: 0x1"),
            LogLineKind::Close
        );

        // Runtime-tagged lines are Other regardless of payload
        assert_eq!(classify_log_line("Program log: hello"), LogLineKind::Other);
        assert_eq!(
            classify_log_line("Program log: invoke [2]"),
            LogLineKind::Other
        );
        assert_eq!(
            classify_log_line("Program log: success"),
            LogLineKind::Other
        );
        assert_eq!(
            classify_log_line("Program log: failed: nope"),
            LogLineKind::Other
        );
        assert_eq!(
            classify_log_line("Program data: BASE64=="),
            LogLineKind::Other
        );
        assert_eq!(
            classify_log_line("Program return: ABC abc="),
            LogLineKind::Other
        );

        // Compute-units consumption line is Other (not a close)
        assert_eq!(
            classify_log_line("Program ABC consumed 1234 of 200000 compute units"),
            LogLineKind::Other,
        );

        // Non-Program lines are Other
        assert_eq!(classify_log_line("just a random log"), LogLineKind::Other);
        assert_eq!(classify_log_line(""), LogLineKind::Other);
        assert_eq!(classify_log_line("Program "), LogLineKind::Other);

        // Malformed invoke variants are Other
        assert_eq!(
            classify_log_line("Program ABC invoke []"),
            LogLineKind::Other
        );
        assert_eq!(
            classify_log_line("Program ABC invoke [abc]"),
            LogLineKind::Other
        );
        assert_eq!(
            classify_log_line("Program ABC invoke 1"),
            LogLineKind::Other
        );
    }
}
