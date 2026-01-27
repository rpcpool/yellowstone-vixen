//! Helpers for parsing transaction updates into instructions.

use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use yellowstone_grpc_proto::{
    geyser::SubscribeUpdateTransactionInfo,
    prelude::MessageHeader,
    solana::storage::confirmed_block::{
        CompiledInstruction, InnerInstruction, InnerInstructions, Message, Reward, TokenBalance,
        Transaction, TransactionError, TransactionStatusMeta,
    },
};

use crate::{Pubkey, TransactionUpdate};

/// Errors that can occur when parsing a transaction update into instructions.
#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum ParseError {
    /// A required field was missing from the transaction update.
    #[error("Transaction update missing {}", .0.name())]
    Missing(Missing),
    /// An inner instruction referenced an out-of-range outer instruction.
    #[error("Invalid inner instruction index {0}")]
    InvalidInnerInstructionIndex(u32),
    /// An error occurred while parsing an account key.
    #[error("Invalid account key in transaction data")]
    AccountKey(#[from] AccountKeyError),
}

/// A required field that was missing from the transaction update.
#[derive(Debug, Clone, Copy)]
pub enum Missing {
    /// The `transaction` field was not present.
    TransactionInfo,
    /// The `transaction.transaction` field was not present.
    Transaction,
    /// The `transaction.meta` field was not present.
    TransactionMeta,
    /// The `transaction.transaction.message` field was not present.
    TransactionMessage,
    /// The `transaction.transaction.message.header` field was not present.
    TransactionMessageHeader,
}

impl Missing {
    #[inline]
    fn name(self) -> &'static str {
        match self {
            Self::TransactionInfo => "transaction info",
            Self::Transaction => "transaction",
            Self::TransactionMeta => "transaction status and metadata",
            Self::TransactionMessage => "transaction message",
            Self::TransactionMessageHeader => "transaction message header",
        }
    }
}

impl From<Missing> for ParseError {
    #[inline]
    fn from(value: Missing) -> Self { Self::Missing(value) }
}

/// Shared data between all instructions in a transaction.
#[derive(Debug, Default)]
pub struct InstructionShared {
    /// The slot in which the transaction was processed.
    pub slot: u64,
    /// The signature of the transaction.
    pub signature: Vec<u8>,
    /// Whether the transaction is a vote transaction.
    pub is_vote: bool,
    /// The index of the transaction in the block.
    pub txn_index: u64,
    /// If the transaction failed, the error that occurred.
    pub err: Option<TransactionError>,
    /// The fee paid by the transaction in lamports.
    pub fee: u64,
    /// The balances of the accounts before the transaction.
    pub pre_balances: Vec<u64>,
    /// The balances of the accounts after the transaction.
    pub post_balances: Vec<u64>,
    /// The token balances of the accounts before the transaction.
    pub pre_token_balances: Vec<TokenBalance>,
    /// The token balances of the accounts after the transaction.
    pub post_token_balances: Vec<TokenBalance>,
    /// The log messages produced during execution of the transaction.
    pub log_messages: Vec<String>,
    /// The rewards produced during execution of the transaction.
    pub rewards: Vec<Reward>,
    /// The number of compute units consumed by the transaction.
    pub compute_units_consumed: Option<u64>,
    /// The recent blockhash submitted with the transaction.
    pub recent_blockhash: Vec<u8>,
    /// The keys of the accounts involved in the transaction.
    pub accounts: AccountKeys,
    /// The header of the transaction.
    pub message_header: MessageHeader,
}

/// A parsed instruction from a transaction update.
#[derive(Debug)]
pub struct InstructionUpdate {
    /// The program ID of the instruction.
    pub program: Pubkey,
    /// The accounts passed to the instruction.
    pub accounts: Vec<Pubkey>,
    /// The serialized binary instruction payload.
    pub data: Vec<u8>,
    /// Shared data between all instructions in this transaction.
    pub shared: Arc<InstructionShared>,
    /// Inner instructions invoked by this instruction.
    pub inner: Vec<InstructionUpdate>,
    /// Indices into `shared.log_messages` for logs emitted by this instruction.
    ///
    /// This includes logs between "Program X invoke" and "Program X success/failed",
    /// excluding logs from inner (CPI) instructions. Use the [`logs`](Self::logs)
    /// method to iterate over the actual log messages.
    pub log_indices: Vec<usize>,
}

/// The keys of the accounts involved in a transaction.
#[derive(Debug, Default)]
pub struct AccountKeys {
    /// Account keys submitted directly with the transaction.
    pub static_keys: Vec<Vec<u8>>,
    /// Resolved writable account keys.
    pub dynamic_rw: Vec<Vec<u8>>,
    /// Resolved readonly account keys.
    pub dynamic_ro: Vec<Vec<u8>>,
}

/// Errors that can occur when parsing an account key.
#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum AccountKeyError {
    /// An error occurred while converting the account key index to a usize.
    #[error("Error converting index to usize")]
    IndexConvert(#[from] std::num::TryFromIntError),
    /// The account key index was out of range.
    #[error("Invalid account key index {0}")]
    InvalidIndex(usize),
    /// The referenced account key was invalid.
    #[error("Invalid account key data")]
    InvalidKey(#[from] std::array::TryFromSliceError),
}

impl AccountKeys {
    /// Get an Account pubkey by index within the Transaction.
    ///
    /// # Errors
    /// Returns an error if the index is invalid.
    pub fn get<I: TryInto<usize>>(&self, idx: I) -> Result<Pubkey, AccountKeyError>
    where I::Error: Into<std::num::TryFromIntError> {
        let idx = idx
            .try_into()
            .map_err(|e| AccountKeyError::IndexConvert(e.into()))?;
        let mut i = idx;
        [&self.static_keys, &self.dynamic_rw, &self.dynamic_ro]
            .into_iter()
            .find_map(|k| {
                k.get(i).map_or_else(
                    || {
                        i = i.saturating_sub(k.len());
                        None
                    },
                    |k| Some(k.as_slice().try_into().map_err(Into::into)),
                )
            })
            .unwrap_or(Err(AccountKeyError::InvalidIndex(idx)))
    }
}

impl InstructionUpdate {
    /// Parse a transaction update into a list of instructions.
    ///
    /// # Errors
    /// Returns an error if the transaction update received is in an unparseable
    /// form.
    pub fn parse_from_txn(txn: &TransactionUpdate) -> Result<Vec<Self>, ParseError> {
        let TransactionUpdate { transaction, slot } = txn.clone();
        let SubscribeUpdateTransactionInfo {
            signature,
            is_vote,
            transaction,
            meta,
            index,
        } = transaction.ok_or(Missing::TransactionInfo)?;
        let Transaction {
            signatures: _,
            message,
        } = transaction.ok_or(Missing::Transaction)?;
        let TransactionStatusMeta {
            err,
            fee,
            pre_balances,
            post_balances,
            inner_instructions,
            inner_instructions_none: _,
            log_messages,
            log_messages_none: _,
            pre_token_balances,
            post_token_balances,
            rewards,
            loaded_writable_addresses,
            loaded_readonly_addresses,
            // TODO: how is this decoded
            return_data: _,
            return_data_none: _,
            compute_units_consumed,
            cost_units: _,
        } = meta.ok_or(Missing::TransactionMeta)?;
        let Message {
            header,
            account_keys,
            recent_blockhash,
            instructions,
            versioned: _,
            address_table_lookups: _,
        } = message.ok_or(Missing::TransactionMessage)?;

        let shared = Arc::new(InstructionShared {
            slot,
            signature,
            is_vote,
            txn_index: index,
            err,
            fee,
            pre_balances,
            post_balances,
            pre_token_balances,
            post_token_balances,
            log_messages,
            rewards,
            compute_units_consumed,
            recent_blockhash,
            accounts: AccountKeys {
                static_keys: account_keys,
                dynamic_rw: loaded_writable_addresses,
                dynamic_ro: loaded_readonly_addresses,
            },
            message_header: header.ok_or(Missing::TransactionMessageHeader)?,
        });

        let mut outer = instructions
            .into_iter()
            .map(|i| Self::parse_one(Arc::clone(&shared), i))
            .collect::<Result<Vec<_>, _>>()?;

        Self::parse_inner(&shared, inner_instructions, &mut outer)?;

        // Assign log indices to all instructions based on program invocation tracking
        LogAssigner::assign(&shared.log_messages, &mut outer);

        Ok(outer)
    }

    fn parse_inner(
        shared: &Arc<InstructionShared>,
        inner_instructions: Vec<InnerInstructions>,
        outer: &mut [Self],
    ) -> Result<(), ParseError> {
        for insn in inner_instructions {
            let InnerInstructions {
                index,
                instructions,
            } = insn;

            let Some(outer) = index.try_into().ok().and_then(|i: usize| outer.get_mut(i)) else {
                return Err(ParseError::InvalidInnerInstructionIndex(index));
            };

            let mut inner = instructions
                .into_iter()
                .map(|i| Self::parse_one_inner(Arc::clone(shared), i))
                .collect::<Result<Vec<_>, _>>()?;

            if let Some(mut i) = inner.len().checked_sub(1) {
                while i > 0 {
                    let parent_idx = i - 1;
                    let Some(height) = inner[parent_idx].1 else {
                        continue;
                    };
                    while inner
                        .get(i)
                        .and_then(|&(_, h)| h)
                        .is_some_and(|h| h > height)
                    {
                        let (child, _) = inner.remove(i);
                        inner[parent_idx].0.inner.push(child);
                    }
                    i -= 1;
                }
            }

            let inner: Vec<_> = inner.into_iter().map(|(i, _)| i).collect();
            if outer.inner.is_empty() {
                outer.inner = inner;
            } else {
                outer.inner.extend(inner);
            }
        }

        Ok(())
    }

    #[inline]
    fn parse_one(
        shared: Arc<InstructionShared>,
        ins: CompiledInstruction,
    ) -> Result<Self, ParseError> {
        let CompiledInstruction {
            program_id_index,
            ref accounts,
            data,
        } = ins;
        Self::parse_from_parts(shared, program_id_index, accounts, data)
    }

    fn parse_one_inner(
        shared: Arc<InstructionShared>,
        ins: InnerInstruction,
    ) -> Result<(Self, Option<u32>), ParseError> {
        let InnerInstruction {
            program_id_index,
            ref accounts,
            data,
            stack_height,
        } = ins;
        Self::parse_from_parts(shared, program_id_index, accounts, data).map(|i| (i, stack_height))
    }

    fn parse_from_parts(
        shared: Arc<InstructionShared>,
        program_id_index: u32,
        accounts: &[u8],
        data: Vec<u8>,
    ) -> Result<Self, ParseError> {
        Ok(Self {
            program: shared.accounts.get(program_id_index)?,
            accounts: accounts
                .iter()
                .map(|&i| shared.accounts.get(i))
                .collect::<Result<_, _>>()?,
            data,
            shared,
            inner: vec![],
            log_indices: vec![],
        })
    }

    /// Iterate over all inner instructions stored in this instruction.
    #[inline]
    pub fn visit_all(&self) -> VisitAll<'_> { VisitAll::new(self) }

    /// Get the log messages for this instruction.
    ///
    /// Returns an iterator over the log messages that were emitted during
    /// this instruction's execution, excluding inner (CPI) instruction logs.
    #[inline]
    pub fn logs(&self) -> impl Iterator<Item = &str> {
        self.log_indices
            .iter()
            .filter_map(|&idx| self.shared.log_messages.get(idx).map(String::as_str))
    }

    /// Get "Program data:" entries for this instruction as decoded bytes.
    ///
    /// These entries typically contain Anchor events emitted via `emit!()`.
    /// The returned Vec contains the raw bytes after base64 decoding.
    #[must_use]
    pub fn program_data(&self) -> Vec<Vec<u8>> {
        use base64::Engine;

        self.logs()
            .filter_map(|log| {
                log.strip_prefix("Program data: ")
                    .and_then(|data| base64::engine::general_purpose::STANDARD.decode(data).ok())
            })
            .collect()
    }

    /// Check if this instruction has any "Program data:" entries.
    #[inline]
    #[must_use]
    pub fn has_program_data(&self) -> bool {
        self.logs().any(|log| log.starts_with("Program data: "))
    }
}

/// Tracks program invocation context to attribute logs to instructions.
///
/// Solana transaction logs follow a pattern:
/// - "Program \<pubkey\> invoke [N]" marks the start of execution at depth N
/// - "Program \<pubkey\> success" or "Program \<pubkey\> failed" marks the end
/// - All logs between these belong to that program invocation
///
/// This assigner maintains a stack to correctly attribute logs during CPI calls,
/// ensuring that logs from inner instructions are not included in outer instruction's
/// log indices.
struct LogAssigner {
    /// Stack tracking (program, depth, start index)
    stack: Vec<(Pubkey, u32, usize)>,
    /// Collected log ranges
    ranges: Vec<LogRange>,
}

/// A range of log messages belonging to a single program invocation.
#[derive(Debug, Clone)]
struct LogRange {
    program: Pubkey,
    depth: u32,
    /// The log index where this invocation started (the "invoke" line)
    start: usize,
    /// The log index where this invocation ended (the "success/failed" line)
    end: usize,
    /// Log indices that belong directly to this invocation (not including CPIs)
    indices: Vec<usize>,
}

impl LogAssigner {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            ranges: Vec::new(),
        }
    }

    /// Assign log indices to all instructions in the transaction.
    ///
    /// This is the main entry point for log assignment. It processes all log
    /// messages and assigns the appropriate indices to each instruction.
    pub fn assign(log_messages: &[String], instructions: &mut [InstructionUpdate]) {
        let mut assigner = Self::new();
        assigner.process_logs(log_messages);
        assigner.apply_to_instructions(instructions, 1);
    }

    /// Process all log messages and build log ranges for each program invocation.
    fn process_logs(&mut self, log_messages: &[String]) {
        for (idx, log) in log_messages.iter().enumerate() {
            // Check for program invoke: "Program <pubkey> invoke [N]"
            if let Some((program, depth)) = Self::parse_invoke(log) {
                self.stack.push((program, depth, idx));
                continue;
            }

            // Check for program end: "Program <pubkey> success" or "Program <pubkey> failed"
            if let Some(program) = Self::parse_program_end(log) {
                // Pop matching entry from stack
                if let Some(pos) = self.stack.iter().rposition(|(p, ..)| *p == program) {
                    let (prog, depth, start) = self.stack.remove(pos);

                    // Collect indices for this range, excluding nested CPIs
                    let mut indices = Vec::new();
                    for i in (start + 1)..idx {
                        // Check if this index is inside a nested CPI by checking
                        // if it falls within the start/end boundaries (inclusive) of any deeper range
                        let in_nested = self
                            .ranges
                            .iter()
                            .any(|r| r.depth > depth && i >= r.start && i <= r.end);
                        if !in_nested {
                            indices.push(i);
                        }
                    }

                    self.ranges.push(LogRange {
                        program: prog,
                        depth,
                        start,
                        end: idx,
                        indices,
                    });
                }
            }
        }
    }

    /// Parse "Program \<pubkey\> invoke [N]"
    fn parse_invoke(log: &str) -> Option<(Pubkey, u32)> {
        // Fast path: check prefix before allocating
        let log = log.strip_prefix("Program ")?;
        if !log.contains(" invoke [") {
            return None;
        }

        let mut parts = log.split_whitespace();
        let pubkey_str = parts.next()?;
        let invoke_word = parts.next()?;
        if invoke_word != "invoke" {
            return None;
        }

        let depth_part = parts.next()?;
        let program = Self::parse_pubkey(pubkey_str)?;
        let depth_str = depth_part.trim_start_matches('[').trim_end_matches(']');
        let depth = depth_str.parse::<u32>().ok()?; // Return None on parse failure instead of defaulting

        Some((program, depth))
    }

    /// Parse "Program \<pubkey\> success" or "Program \<pubkey\> failed: ..."
    fn parse_program_end(log: &str) -> Option<Pubkey> {
        // Fast path: check prefix before any allocation
        let log = log.strip_prefix("Program ")?;

        let mut parts = log.split_whitespace();
        let pubkey_str = parts.next()?;
        let status = parts.next()?;

        if status == "success" || status.starts_with("failed") {
            return Self::parse_pubkey(pubkey_str);
        }
        None
    }

    /// Parse a base58-encoded pubkey string into a Pubkey.
    fn parse_pubkey(s: &str) -> Option<Pubkey> {
        let bytes = bs58::decode(s).into_vec().ok()?;
        if bytes.len() != 32 {
            return None;
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Some(crate::KeyBytes(arr))
    }

    /// Apply log indices to instructions based on collected ranges.
    ///
    /// Uses a pre-grouped map for O(1) lookups instead of O(n) filtering.
    fn apply_to_instructions(&self, instructions: &mut [InstructionUpdate], depth: u32) {
        // Pre-group ranges by (program, depth) for O(1) lookup
        // Key: (program, depth), Value: list of index ranges in order
        let mut grouped: HashMap<(Pubkey, u32), Vec<&Vec<usize>>> = HashMap::new();
        for range in &self.ranges {
            grouped
                .entry((range.program, range.depth))
                .or_default()
                .push(&range.indices);
        }

        Self::apply_to_instructions_recursive(instructions, depth, &grouped, &mut HashMap::new());
    }

    fn apply_to_instructions_recursive(
        instructions: &mut [InstructionUpdate],
        depth: u32,
        grouped: &HashMap<(Pubkey, u32), Vec<&Vec<usize>>>,
        used_indices: &mut HashMap<(Pubkey, u32), usize>,
    ) {
        for ix in instructions.iter_mut() {
            let key = (ix.program, depth);

            // Get the next unused range for this (program, depth) pair
            let used_idx = used_indices.entry(key).or_insert(0);
            if let Some(ranges) = grouped.get(&key)
                && let Some(&indices) = ranges.get(*used_idx)
            {
                ix.log_indices.clone_from(indices);
                *used_idx += 1;
            }

            // Recursively apply to inner instructions at depth + 1
            Self::apply_to_instructions_recursive(&mut ix.inner, depth + 1, grouped, used_indices);
        }
    }
}

/// An iterator over all inner instructions stored in an instruction update.
#[derive(Debug)]
#[must_use = "This type does nothing unless iterated"]
pub struct VisitAll<'a>(VisitAllState<'a>);

#[derive(Debug)]
enum VisitAllState<'a> {
    Init(&'a InstructionUpdate),
    Started(VecDeque<std::slice::Iter<'a, InstructionUpdate>>),
}

impl<'a> VisitAll<'a> {
    #[inline]
    fn new(ixs: &'a InstructionUpdate) -> Self { Self(VisitAllState::Init(ixs)) }
}

impl<'a> Iterator for VisitAll<'a> {
    type Item = &'a InstructionUpdate;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            &mut VisitAllState::Init(i) => {
                let mut d = VecDeque::new();
                d.push_back(i.inner.iter());
                self.0 = VisitAllState::Started(d);
                Some(i)
            },
            VisitAllState::Started(d) => loop {
                let Some(ix) = d.back_mut()?.next() else {
                    let _ = d.pop_back().unwrap_or_else(|| unreachable!());
                    continue;
                };
                d.push_back(ix.inner.iter());
                break Some(ix);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a dummy pubkey from a string for testing
    #[allow(dead_code)]
    fn test_pubkey(s: &str) -> Pubkey {
        let mut arr = [0u8; 32];
        let bytes = s.as_bytes();
        arr[..bytes.len().min(32)].copy_from_slice(&bytes[..bytes.len().min(32)]);
        crate::KeyBytes(arr)
    }

    #[test]
    fn test_parse_invoke() {
        let log = "Program 11111111111111111111111111111111 invoke [1]";
        let result = LogAssigner::parse_invoke(log);
        assert!(result.is_some());
        let (_, depth) = result.unwrap();
        assert_eq!(depth, 1);

        let log2 = "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]";
        let result2 = LogAssigner::parse_invoke(log2);
        assert!(result2.is_some());
        let (_, depth2) = result2.unwrap();
        assert_eq!(depth2, 2);
    }

    #[test]
    fn test_parse_program_end() {
        let success_log = "Program 11111111111111111111111111111111 success";
        assert!(LogAssigner::parse_program_end(success_log).is_some());

        let failed_log = "Program 11111111111111111111111111111111 failed: custom error";
        assert!(LogAssigner::parse_program_end(failed_log).is_some());

        let other_log = "Program log: Hello world";
        assert!(LogAssigner::parse_program_end(other_log).is_none());
    }

    #[test]
    fn test_log_assigner_simple() {
        let logs = vec![
            "Program 11111111111111111111111111111111 invoke [1]".to_string(),
            "Program log: Hello from System Program".to_string(),
            "Program 11111111111111111111111111111111 success".to_string(),
        ];

        let mut assigner = LogAssigner::new();
        assigner.process_logs(&logs);

        assert_eq!(assigner.ranges.len(), 1);
        assert_eq!(assigner.ranges[0].depth, 1);
        // Only the log message (index 1), not invoke/success
        assert_eq!(assigner.ranges[0].indices, vec![1]);
    }

    #[test]
    fn test_log_assigner_with_cpi() {
        // Simulates: Outer program (AAA) calls inner program (BBB)
        // Using valid base58 pubkeys (32 bytes each)
        let outer_pk = "11111111111111111111111111111111";
        let inner_pk = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

        let logs = vec![
            format!("Program {} invoke [1]", outer_pk),
            "Program log: Outer before CPI".to_string(),
            format!("Program {} invoke [2]", inner_pk),
            "Program data: SGVsbG8=".to_string(), // Should belong to inner
            format!("Program {} success", inner_pk),
            "Program log: Outer after CPI".to_string(), // Should belong to outer
            format!("Program {} success", outer_pk),
        ];

        let mut assigner = LogAssigner::new();
        assigner.process_logs(&logs);

        // Should have 2 ranges
        assert_eq!(assigner.ranges.len(), 2);

        // Find the inner program's range (depth 2)
        let inner_range = assigner.ranges.iter().find(|r| r.depth == 2);
        assert!(inner_range.is_some());
        assert_eq!(inner_range.unwrap().indices, vec![3]); // "Program data: SGVsbG8="

        // Find the outer program's range (depth 1)
        let outer_range = assigner.ranges.iter().find(|r| r.depth == 1);
        assert!(outer_range.is_some());
        // Should have indices 1 and 5 (before and after CPI logs)
        let outer_indices = &outer_range.unwrap().indices;
        assert!(outer_indices.contains(&1)); // "Outer before CPI"
        assert!(outer_indices.contains(&5)); // "Outer after CPI"
    }

    #[test]
    fn test_log_assigner_failed_transaction() {
        let logs = vec![
            "Program 11111111111111111111111111111111 invoke [1]".to_string(),
            "Program log: About to fail".to_string(),
            "Program 11111111111111111111111111111111 failed: custom error".to_string(),
        ];

        let mut assigner = LogAssigner::new();
        assigner.process_logs(&logs);

        assert_eq!(assigner.ranges.len(), 1);
        assert_eq!(assigner.ranges[0].indices, vec![1]);
    }

    #[test]
    fn test_log_assigner_cpi_with_no_logs() {
        // Edge case: CPI that emits no logs (just invoke/success)
        let outer_pk = "11111111111111111111111111111111";
        let inner_pk = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

        let logs = vec![
            format!("Program {} invoke [1]", outer_pk),
            "Program log: Before CPI".to_string(),
            format!("Program {} invoke [2]", inner_pk),
            format!("Program {} success", inner_pk), // No logs inside inner
            "Program log: After CPI".to_string(),
            format!("Program {} success", outer_pk),
        ];

        let mut assigner = LogAssigner::new();
        assigner.process_logs(&logs);

        assert_eq!(assigner.ranges.len(), 2);

        // Inner program should have no log indices
        let inner_range = assigner.ranges.iter().find(|r| r.depth == 2).unwrap();
        assert!(inner_range.indices.is_empty());

        // Outer program should have indices 1 and 4 (before/after CPI)
        // NOT indices 2 or 3 (the invoke/success of inner)
        let outer_range = assigner.ranges.iter().find(|r| r.depth == 1).unwrap();
        assert_eq!(outer_range.indices, vec![1, 4]);
    }

    #[test]
    fn test_log_assigner_deeply_nested_cpi() {
        // A -> B -> C (3 levels deep)
        let a_pk = "11111111111111111111111111111111";
        let b_pk = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
        let c_pk = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

        let logs = vec![
            format!("Program {} invoke [1]", a_pk), // 0
            "Program log: A start".to_string(),     // 1 -> A
            format!("Program {} invoke [2]", b_pk), // 2
            "Program log: B start".to_string(),     // 3 -> B
            format!("Program {} invoke [3]", c_pk), // 4
            "Program log: C log".to_string(),       // 5 -> C
            format!("Program {} success", c_pk),    // 6
            "Program log: B end".to_string(),       // 7 -> B
            format!("Program {} success", b_pk),    // 8
            "Program log: A end".to_string(),       // 9 -> A
            format!("Program {} success", a_pk),    // 10
        ];

        let mut assigner = LogAssigner::new();
        assigner.process_logs(&logs);

        assert_eq!(assigner.ranges.len(), 3);

        // C (depth 3) should have index 5
        let c_range = assigner.ranges.iter().find(|r| r.depth == 3).unwrap();
        assert_eq!(c_range.indices, vec![5]);

        // B (depth 2) should have indices 3 and 7
        let b_range = assigner.ranges.iter().find(|r| r.depth == 2).unwrap();
        assert_eq!(b_range.indices, vec![3, 7]);

        // A (depth 1) should have indices 1 and 9
        let a_range = assigner.ranges.iter().find(|r| r.depth == 1).unwrap();
        assert_eq!(a_range.indices, vec![1, 9]);
    }

    #[test]
    fn test_log_assigner_multiple_outer_instructions() {
        // Two separate outer instructions in same transaction
        let pk1 = "11111111111111111111111111111111";
        let pk2 = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

        let logs = vec![
            format!("Program {} invoke [1]", pk1),         // 0
            "Program log: First instruction".to_string(),  // 1 -> pk1
            format!("Program {} success", pk1),            // 2
            format!("Program {} invoke [1]", pk2),         // 3
            "Program log: Second instruction".to_string(), // 4 -> pk2
            format!("Program {} success", pk2),            // 5
        ];

        let mut assigner = LogAssigner::new();
        assigner.process_logs(&logs);

        assert_eq!(assigner.ranges.len(), 2);

        // Both should be at depth 1
        assert!(assigner.ranges.iter().all(|r| r.depth == 1));

        // First range should have index 1
        let first = &assigner.ranges[0];
        assert_eq!(first.indices, vec![1]);

        // Second range should have index 4
        let second = &assigner.ranges[1];
        assert_eq!(second.indices, vec![4]);
    }

    #[test]
    fn test_log_assigner_same_program_multiple_invocations() {
        // Same program invoked twice at same depth (common pattern)
        let pk = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

        let logs = vec![
            format!("Program {} invoke [1]", pk),   // 0
            "Program log: First call".to_string(),  // 1
            format!("Program {} success", pk),      // 2
            format!("Program {} invoke [1]", pk),   // 3
            "Program log: Second call".to_string(), // 4
            format!("Program {} success", pk),      // 5
        ];

        let mut assigner = LogAssigner::new();
        assigner.process_logs(&logs);

        assert_eq!(assigner.ranges.len(), 2);

        // Both ranges should have the same program and depth
        assert!(assigner.ranges.iter().all(|r| r.depth == 1));

        // Ranges should be in order
        assert_eq!(assigner.ranges[0].indices, vec![1]);
        assert_eq!(assigner.ranges[1].indices, vec![4]);
    }

    #[test]
    fn test_log_assigner_empty_logs() {
        let logs: Vec<String> = vec![];

        let mut assigner = LogAssigner::new();
        assigner.process_logs(&logs);

        assert!(assigner.ranges.is_empty());
    }

    #[test]
    fn test_log_assigner_malformed_logs_ignored() {
        // Malformed logs should be gracefully ignored
        let logs = vec![
            "Not a program log".to_string(),
            "Program without invoke".to_string(),
            "Program invoke without depth".to_string(),
            "Program 11111111111111111111111111111111 invoke [1]".to_string(),
            "Program log: Valid log".to_string(),
            "Program 11111111111111111111111111111111 success".to_string(),
        ];

        let mut assigner = LogAssigner::new();
        assigner.process_logs(&logs);

        // Should only have one valid range
        assert_eq!(assigner.ranges.len(), 1);
        assert_eq!(assigner.ranges[0].indices, vec![4]); // Only the valid log
    }

    #[test]
    fn test_parse_invoke_edge_cases() {
        // Valid cases
        assert!(
            LogAssigner::parse_invoke("Program 11111111111111111111111111111111 invoke [1]")
                .is_some()
        );
        assert!(
            LogAssigner::parse_invoke("Program 11111111111111111111111111111111 invoke [99]")
                .is_some()
        );

        // Invalid cases
        assert!(LogAssigner::parse_invoke("Program invoke [1]").is_none()); // Missing pubkey
        assert!(LogAssigner::parse_invoke("Program log: invoke [1]").is_none()); // Not an invoke
        assert!(
            LogAssigner::parse_invoke("program 11111111111111111111111111111111 invoke [1]")
                .is_none()
        ); // Lowercase
        assert!(
            LogAssigner::parse_invoke("Program 11111111111111111111111111111111 invoke []")
                .is_none()
        ); // Empty depth
        assert!(
            LogAssigner::parse_invoke("Program 11111111111111111111111111111111 invoke [abc]")
                .is_none()
        ); // Non-numeric depth
    }
}
