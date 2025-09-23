//! Helpers for parsing transaction updates into instructions.

use std::{collections::VecDeque, sync::Arc};

use regex::Regex;
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
    fn from(value: Missing) -> Self {
        Self::Missing(value)
    }
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
    /// The unique index of this instruction within the transaction
    pub ix_index: u16,
    /// The program pubkey of the parent instruction (None for top-level instructions)
    pub parent_program: Option<Pubkey>,
    /// Program logs generated during execution of this instruction.
    pub parsed_logs: Vec<String>,
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
    where
        I::Error: Into<std::num::TryFromIntError>,
    {
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
            .enumerate()
            .map(|(idx, i)| Self::parse_one(Arc::clone(&shared), i, idx as u16))
            .collect::<Result<Vec<_>, _>>()?;

        let mut next_idx = outer.len() as u16;
        Self::parse_inner(&shared, inner_instructions, &mut outer, &mut next_idx)?;

        // Assign logs to instructions based on invoke/success patterns
        Self::assign_logs_to_instructions(&mut outer, &shared.log_messages)?;

        Ok(outer)
    }

    fn parse_inner(
        shared: &Arc<InstructionShared>,
        inner_instructions: Vec<InnerInstructions>,
        outer: &mut [Self],
        next_idx: &mut u16,
    ) -> Result<(), ParseError> {
        for insn in inner_instructions {
            let InnerInstructions {
                index,
                instructions,
            } = insn;

            let Some(outer) = index.try_into().ok().and_then(|i: usize| outer.get_mut(i)) else {
                return Err(ParseError::InvalidInnerInstructionIndex(index));
            };

            let parent_program = outer.program;
            let mut inner = instructions
                .into_iter()
                .map(|i| {
                    let idx = *next_idx;
                    *next_idx += 1;
                    Self::parse_one_inner(Arc::clone(shared), i, idx, parent_program)
                })
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
                        let (mut child, _) = inner.remove(i);
                        child.parent_program = Some(inner[parent_idx].0.program);
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

    fn assign_logs_to_instructions(
        outer: &mut [Self],
        log_messages: &[String],
    ) -> Result<(), ParseError> {
        let invoke_regex =
            Regex::new(r"Program ([1-9A-HJ-NP-Za-km-z]{32,44}) invoke \[(\d+)\]").unwrap();
        let success_regex = Regex::new(r"Program ([1-9A-HJ-NP-Za-km-z]{32,44}) success").unwrap();
        let failed_regex = Regex::new(r"Program ([1-9A-HJ-NP-Za-km-z]{32,44}) failed:").unwrap();
        let consumed_regex =
            Regex::new(r"Program ([1-9A-HJ-NP-Za-km-z]{32,44}) consumed \d+ of \d+ compute units")
                .unwrap();

        // create list of instruction paths in depth-first order for existing instructions data structure
        let mut instruction_paths = Vec::new();
        Self::collect_instruction_paths(&mut instruction_paths, outer, Vec::new());

        // maintain current execution stack (store paths)
        let mut execution_stack: Vec<Vec<usize>> = Vec::new();
        let mut path_iter = instruction_paths.iter();

        for log in log_messages {
            // NOTE: invoke 給要開始的 program, 其餘的給 stack top
            if let Some(captures) = invoke_regex.captures(log) {
                let program_id = captures.get(1).unwrap().as_str();
                let depth: usize = captures.get(2).unwrap().as_str().parse().unwrap();

                execution_stack.truncate(depth - 1);

                // find next matching instruction
                // NOTE: 這假設指令樹順序跟 Log 樹順序一致
                while let Some(path) = path_iter.next() {
                    let instruction = Self::get_instruction_at_path_mut(outer, path)?;
                    if instruction.program.to_string() == program_id {
                        instruction.parsed_logs.push(log.clone());
                        execution_stack.push(path.clone());
                        break;
                    }
                }
            } else if success_regex.is_match(log) {
                if let Some(current_path) = execution_stack.last() {
                    let current = Self::get_instruction_at_path_mut(outer, current_path)?;
                    current.parsed_logs.push(log.clone());
                    execution_stack.pop();
                }
            } else if failed_regex.is_match(log) {
                // Handle failed instructions - same as success but indicates failure
                if let Some(current_path) = execution_stack.last() {
                    let current = Self::get_instruction_at_path_mut(outer, current_path)?;
                    current.parsed_logs.push(log.clone());
                    execution_stack.pop();
                }
            } else if let Some(captures) = consumed_regex.captures(log) {
                let program_id = captures.get(1).unwrap().as_str();

                // find matching program in execution stack
                if let Some(matching_path) = execution_stack.iter().rev().find(|path| {
                    Self::get_instruction_at_path(outer, path)
                        .map(|instr| instr.program.to_string() == program_id)
                        .unwrap_or(false)
                }) {
                    let instruction = Self::get_instruction_at_path_mut(outer, matching_path)?;
                    instruction.parsed_logs.push(log.clone());
                }
            } else if let Some(current_path) = execution_stack.last() {
                // other logs assigned to current instruction
                let current = Self::get_instruction_at_path_mut(outer, current_path)?;
                current.parsed_logs.push(log.clone());
            }
        }

        Ok(())
    }

    // auxiliary function: collect instruction paths in depth-first order
    fn collect_instruction_paths(
        paths: &mut Vec<Vec<usize>>,
        instructions: &[Self],
        current_path: Vec<usize>,
    ) {
        for (i, instruction) in instructions.iter().enumerate() {
            let mut path = current_path.clone();
            path.push(i);
            paths.push(path.clone());

            // recursively process inner instructions
            Self::collect_instruction_paths(paths, &instruction.inner, path);
        }
    }

    // auxiliary function: get mutable reference to instruction at path
    fn get_instruction_at_path_mut<'a>(
        outer: &'a mut [Self],
        path: &[usize],
    ) -> Result<&'a mut Self, ParseError> {
        if path.is_empty() {
            return Err(ParseError::InvalidInnerInstructionIndex(0));
        }

        let mut current = outer
            .get_mut(path[0])
            .ok_or(ParseError::InvalidInnerInstructionIndex(path[0] as u32))?;

        for &index in &path[1..] {
            current = current
                .inner
                .get_mut(index)
                .ok_or(ParseError::InvalidInnerInstructionIndex(index as u32))?;
        }

        Ok(current)
    }

    // auxiliary function: get immutable reference to instruction at path
    fn get_instruction_at_path<'a>(outer: &'a [Self], path: &[usize]) -> Option<&'a Self> {
        if path.is_empty() {
            return None;
        }

        let mut current = outer.get(path[0])?;

        for &index in &path[1..] {
            current = current.inner.get(index)?;
        }

        Some(current)
    }

    #[inline]
    fn parse_one(
        shared: Arc<InstructionShared>,
        ins: CompiledInstruction,
        ix_index: u16,
    ) -> Result<Self, ParseError> {
        let CompiledInstruction {
            program_id_index,
            ref accounts,
            data,
        } = ins;
        Self::parse_from_parts(shared, program_id_index, accounts, data, ix_index, None)
    }

    fn parse_one_inner(
        shared: Arc<InstructionShared>,
        ins: InnerInstruction,
        ix_index: u16,
        parent_program: Pubkey,
    ) -> Result<(Self, Option<u32>), ParseError> {
        let InnerInstruction {
            program_id_index,
            ref accounts,
            data,
            stack_height,
        } = ins;
        Self::parse_from_parts(
            shared,
            program_id_index,
            accounts,
            data,
            ix_index,
            Some(parent_program),
        )
        .map(|i| (i, stack_height))
    }

    fn parse_from_parts(
        shared: Arc<InstructionShared>,
        program_id_index: u32,
        accounts: &[u8],
        data: Vec<u8>,
        ix_index: u16,
        parent_program: Option<Pubkey>,
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
            ix_index,
            parent_program,
            parsed_logs: vec![],
        })
    }

    /// Iterate over all inner instructions stored in this instruction.
    #[inline]
    pub fn visit_all(&self) -> VisitAll<'_> {
        VisitAll::new(self)
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
    fn new(ixs: &'a InstructionUpdate) -> Self {
        Self(VisitAllState::Init(ixs))
    }
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

    #[tokio::test]
    async fn test_instruction_logs_assignment() {
        use yellowstone_vixen_mock::{
            create_mock_transaction_update, parse_instructions_from_txn_update,
        };

        let fixture_signature = "3VChbqC1CpbiN6seqo7aGvRaPPkvxh5c4W1y7NNRt23mk8cfX4fac6FPB9Z47APLnTtgZJ3eWEnnhfa2kJdnSbUr";

        // Create a mock transaction update using the utility
        let transaction_update = create_mock_transaction_update(fixture_signature)
            .await
            .expect("Failed to create mock transaction update");

        // Parse instructions using the core parse_from_txn logic
        let instruction_updates = parse_instructions_from_txn_update(&transaction_update)
            .expect("Failed to parse instructions from transaction update");

        println!(
            "Testing log assignment on {} instructions",
            instruction_updates.len()
        );

        for (i, ix) in instruction_updates.iter().enumerate() {
            println!("\n=== Instruction {} ===", i);
            println!(
                "Program: {}, Parsed logs count: {}, Raw logs count: {}",
                ix.program.to_string(),
                ix.parsed_logs.len(),
                ix.shared.log_messages.len()
            );

            // Print all parsed logs for this instruction
            println!("Parsed logs:");
            for (j, parsed_log) in ix.parsed_logs.iter().enumerate() {
                println!("  Parsed log {}: {}", j, parsed_log);
            }

            // Test inner instructions
            for (inner_i, inner_ix) in ix.inner.iter().enumerate() {
                println!("\n  === Inner Instruction {} ===", inner_i);
                println!(
                    "  Program: {}, Parsed logs count: {}",
                    inner_ix.program.to_string(),
                    inner_ix.parsed_logs.len()
                );

                // Print all parsed logs for inner instructions
                println!("  Inner parsed logs:");
                for (j, log) in inner_ix.parsed_logs.iter().enumerate() {
                    println!("    Inner parsed log {}: {}", j, log);
                }
            }
        }

        // Verify that we have some instructions
        assert!(
            !instruction_updates.is_empty(),
            "Should have parsed some instructions"
        );

        // Verify that logs are properly assigned to instructions
        let instructions_with_logs = instruction_updates
            .iter()
            .filter(|ix| !ix.parsed_logs.is_empty())
            .count();

        // At least some instructions should have logs assigned
        assert!(
            instructions_with_logs > 0,
            "At least some instructions should have logs assigned"
        );

        println!("✓ Instruction log assignment test completed successfully");
        println!(
            "✓ {} out of {} instructions have assigned logs",
            instructions_with_logs,
            instruction_updates.len()
        );
    }

    #[tokio::test]
    async fn test_inner_instruction_parent_relationships() {
        use yellowstone_vixen_mock::create_mock_transaction_update;

        let fixture_signature = "3VChbqC1CpbiN6seqo7aGvRaPPkvxh5c4W1y7NNRt23mk8cfX4fac6FPB9Z47APLnTtgZJ3eWEnnhfa2kJdnSbUr";

        // Create a mock transaction update using the utility
        let transaction_update = create_mock_transaction_update(fixture_signature)
            .await
            .expect("Failed to create mock transaction update");

        // Parse instructions using the core parse_from_txn logic directly
        let instruction_updates = super::InstructionUpdate::parse_from_txn(&transaction_update)
            .expect("Failed to parse instructions from transaction update");

        println!(
            "Testing parent-child relationships on {} instructions",
            instruction_updates.len()
        );

        // Helper function to recursively print instruction hierarchy
        fn print_instruction_hierarchy(
            ix: &super::InstructionUpdate,
            depth: usize,
            instruction_path: &str,
        ) {
            let indent = "  ".repeat(depth);
            let parent_info = if let Some(parent_program) = ix.parent_program {
                format!("parent: {}", parent_program.to_string())
            } else {
                "TOP-LEVEL".to_string()
            };

            println!(
                "{}{}Instruction {}: ix_index={}, {}, program={}",
                indent,
                instruction_path,
                "",
                ix.ix_index,
                parent_info,
                ix.program.to_string()
            );

            // Recursively print inner instructions
            for (inner_i, inner_ix) in ix.inner.iter().enumerate() {
                let inner_path = format!("{}[inner-{}] ", instruction_path, inner_i);
                print_instruction_hierarchy(inner_ix, depth + 1, &inner_path);
            }
        }

        // Print hierarchy for all top-level instructions
        for (i, ix) in instruction_updates.iter().enumerate() {
            println!("\n=== Top-level Instruction {} ===", i);
            print_instruction_hierarchy(ix, 0, "");
        }

        // Verify parent-child relationships
        let mut total_instructions = 0;
        let mut top_level_count = 0;
        let mut inner_instructions_count = 0;

        // Helper function to recursively count and verify instructions
        fn verify_instruction_hierarchy(
            ix: &super::InstructionUpdate,
            expected_parent: Option<super::Pubkey>,
        ) -> (usize, usize, usize) {
            let mut total = 1;
            let mut top_level = 0;
            let mut inner = 0;

            // Verify parent relationship
            if ix.parent_program.is_none() {
                top_level = 1;
                assert!(
                    expected_parent.is_none(),
                    "Top-level instruction should not have a parent, but parent_program is {:?}",
                    ix.parent_program
                );
            } else {
                inner = 1;
                if let Some(expected) = expected_parent {
                    assert_eq!(
                        ix.parent_program, Some(expected),
                        "Inner instruction parent_program {:?} doesn't match expected parent {}",
                        ix.parent_program, expected
                    );
                }
            }

            // Recursively verify inner instructions
            for inner_ix in &ix.inner {
                let (inner_total, inner_top_level, inner_inner) =
                    verify_instruction_hierarchy(inner_ix, Some(ix.program));
                total += inner_total;
                top_level += inner_top_level;
                inner += inner_inner;
            }

            (total, top_level, inner)
        }

        // Count and verify all instructions
        for ix in &instruction_updates {
            let (count, top, inner_count) = verify_instruction_hierarchy(ix, None);
            total_instructions += count;
            top_level_count += top;
            inner_instructions_count += inner_count;
        }

        println!("\n=== Parent-Child Relationship Summary ===");
        println!("Total instructions: {}", total_instructions);
        println!("Top-level instructions: {}", top_level_count);
        println!("Inner instructions: {}", inner_instructions_count);

        // Verify that we have some instructions
        assert!(
            !instruction_updates.is_empty(),
            "Should have parsed some instructions"
        );

        // Verify counts make sense
        assert_eq!(
            top_level_count,
            instruction_updates.len(),
            "Top-level count should match outer instruction count"
        );

        println!("✓ Parent-child relationship test completed successfully");
        println!("✓ All parent relationships verified correctly");
    }
}
