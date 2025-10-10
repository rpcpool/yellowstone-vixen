//! Helpers for parsing transaction updates into instructions.

use std::{
    collections::VecDeque,
    sync::{Arc, LazyLock},
};

use regex::Regex;
use yellowstone_grpc_proto::{
    geyser::SubscribeUpdateTransactionInfo,
    prelude::MessageHeader,
    solana::storage::confirmed_block::{
        CompiledInstruction, InnerInstruction, InnerInstructions, Message, Reward, TokenBalance,
        Transaction, TransactionError, TransactionStatusMeta,
    },
};

use crate::{KeyBytes, Pubkey, TransactionUpdate};

// Static regex patterns for log parsing
static INVOKE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Program ([1-9A-HJ-NP-Za-km-z]{32,44}) invoke \[(\d+)\]").unwrap()
});

static SUCCESS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Program ([1-9A-HJ-NP-Za-km-z]{32,44}) success").unwrap());

static FAILED_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Program ([1-9A-HJ-NP-Za-km-z]{32,44}) failed:").unwrap());

static CONSUMED_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Program ([1-9A-HJ-NP-Za-km-z]{32,44}) consumed \d+ of \d+ compute units").unwrap()
});

/// Information about a token account created during transaction execution
#[derive(Debug, Clone, Copy)]
pub struct CreatedTokenAccount {
    /// The pubkey of the created token account
    pub account: KeyBytes<32>,
    /// The mint of the token
    pub mint: KeyBytes<32>,
    /// The owner of the token account
    pub owner: KeyBytes<32>,
}

/// Pre-parsed log message representation
#[derive(Debug, Clone)]
enum ParsedLog {
    Invoke {
        program_id: Pubkey,
        depth: usize,
        original_idx: usize,
    },
    Success {
        program_id: Pubkey,
        original_idx: usize,
    },
    Failed {
        program_id: Pubkey,
        original_idx: usize,
    },
    Consumed {
        #[allow(dead_code)]
        program_id: Pubkey,
        original_idx: usize,
    },
    Other {
        original_idx: usize,
    },
}

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
    /// Token accounts created during transaction execution (parsed from inner instructions)
    pub created_token_accounts: Vec<CreatedTokenAccount>,
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
    /// Indices into `shared.log_messages` for logs generated during execution of this instruction.
    pub parsed_logs: Vec<usize>,
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
    /// Parse created token accounts from both outer and inner instructions
    ///
    /// Searches through all outer and inner instructions for SPL Token InitializeAccount/InitializeAccount2/InitializeAccount3
    /// instructions and extracts the account, mint, and owner information.
    fn parse_created_token_accounts(
        outer_instructions: &[CompiledInstruction],
        inner_instructions: &[InnerInstructions],
        account_keys: &[Vec<u8>],
        loaded_writable_addresses: &[Vec<u8>],
        loaded_readonly_addresses: &[Vec<u8>],
    ) -> Vec<CreatedTokenAccount> {
        use spl_token::instruction::TokenInstruction;

        let mut created_accounts = Vec::new();

        // Helper to get account by index
        let get_account = |idx: usize| -> Option<KeyBytes<32>> {
            let mut i = idx;
            for keys in [
                account_keys,
                loaded_writable_addresses,
                loaded_readonly_addresses,
            ] {
                if i < keys.len() {
                    return keys[i].as_slice().try_into().ok();
                }
                i = i.saturating_sub(keys.len());
            }
            None
        };

        // SPL Token program IDs
        let spl_token_program: KeyBytes<32> = spl_token::ID.to_bytes().into();
        let spl_token_2022_program: KeyBytes<32> = spl_token_2022::ID.to_bytes().into();

        // Helper to process a compiled instruction
        let mut process_instruction = |program_id_index: u32, accounts: &[u8], data: &[u8]| {
            // Check if this is a SPL Token or Token-2022 program instruction
            let Some(program_id) = get_account(program_id_index as usize) else {
                return;
            };

            if program_id != spl_token_program && program_id != spl_token_2022_program {
                return;
            }

            // Try to parse the instruction
            if let Ok(token_ix) = TokenInstruction::unpack(data) {
                match token_ix {
                    // InitializeAccount: accounts = [account, mint, owner, rent_sysvar]
                    TokenInstruction::InitializeAccount => {
                        if accounts.len() >= 3 {
                            if let (Some(account), Some(mint), Some(owner)) = (
                                get_account(accounts[0] as usize),
                                get_account(accounts[1] as usize),
                                get_account(accounts[2] as usize),
                            ) {
                                created_accounts.push(CreatedTokenAccount {
                                    account,
                                    mint,
                                    owner,
                                });
                            }
                        }
                    },
                    // InitializeAccount2: accounts = [account, mint]
                    // InitializeAccount3: accounts = [account, mint]
                    // owner is extracted from instruction data, not accounts
                    TokenInstruction::InitializeAccount2 { owner }
                    | TokenInstruction::InitializeAccount3 { owner } => {
                        if accounts.len() >= 2 {
                            if let (Some(account), Some(mint)) = (
                                get_account(accounts[0] as usize),
                                get_account(accounts[1] as usize),
                            ) {
                                created_accounts.push(CreatedTokenAccount {
                                    account,
                                    mint,
                                    owner: owner.to_bytes().into(),
                                });
                            }
                        }
                    },
                    _ => {},
                }
            }
        };

        // Parse outer instructions
        for outer_ix in outer_instructions {
            process_instruction(
                outer_ix.program_id_index,
                &outer_ix.accounts,
                &outer_ix.data,
            );
        }

        // Parse inner instructions
        for inner_ix_set in inner_instructions {
            for inner_ix in &inner_ix_set.instructions {
                process_instruction(
                    inner_ix.program_id_index,
                    &inner_ix.accounts,
                    &inner_ix.data,
                );
            }
        }

        created_accounts
    }

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

        // Parse created token accounts from both outer and inner instructions
        let created_token_accounts = Self::parse_created_token_accounts(
            &instructions,
            &inner_instructions,
            &account_keys,
            &loaded_writable_addresses,
            &loaded_readonly_addresses,
        );

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
            created_token_accounts,
        });

        let mut outer = instructions
            .into_iter()
            .enumerate()
            .map(|(idx, i)| {
                Self::parse_one(
                    Arc::clone(&shared),
                    i,
                    u16::try_from(idx).unwrap_or(u16::MAX),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut next_idx = u16::try_from(outer.len()).unwrap_or(u16::MAX);
        Self::parse_inner(&shared, inner_instructions, &mut outer, &mut next_idx)?;

        // Assign logs to instructions based on invoke/success patterns
        Self::assign_logs_to_instructions(&mut outer, &shared.log_messages);

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

    fn assign_logs_to_instructions(outer: &mut [Self], log_messages: &[String]) {
        // Pre-parse all logs into structured representation
        let parsed_logs = Self::pre_parse_logs(log_messages);

        // Two-pointer algorithm: traverse instruction tree and logs simultaneously
        let mut log_idx = 0;
        let _orphaned_logs = Self::assign_logs_recursive(outer, &parsed_logs, &mut log_idx, 1);
        // Note: orphaned_logs at depth 1 means logs that couldn't be assigned to any instruction
        // This shouldn't happen in well-formed transactions, but we discard them here
    }

    /// Pre-parse log messages into structured representation
    fn pre_parse_logs(log_messages: &[String]) -> Vec<ParsedLog> {
        log_messages
            .iter()
            .enumerate()
            .map(|(idx, log)| {
                if let Some(captures) = INVOKE_REGEX.captures(log) {
                    let program_id_str = captures.get(1).unwrap().as_str();
                    let depth: usize = captures.get(2).unwrap().as_str().parse().unwrap();

                    // Parse program_id to Pubkey (avoid repeated to_string() later)
                    if let Ok(program_id) = program_id_str.parse::<Pubkey>() {
                        return ParsedLog::Invoke {
                            program_id,
                            depth,
                            original_idx: idx,
                        };
                    }
                } else if let Some(captures) = SUCCESS_REGEX.captures(log) {
                    let program_id_str = captures.get(1).unwrap().as_str();
                    if let Ok(program_id) = program_id_str.parse::<Pubkey>() {
                        return ParsedLog::Success {
                            program_id,
                            original_idx: idx,
                        };
                    }
                } else if let Some(captures) = FAILED_REGEX.captures(log) {
                    let program_id_str = captures.get(1).unwrap().as_str();
                    if let Ok(program_id) = program_id_str.parse::<Pubkey>() {
                        return ParsedLog::Failed {
                            program_id,
                            original_idx: idx,
                        };
                    }
                } else if let Some(captures) = CONSUMED_REGEX.captures(log) {
                    let program_id_str = captures.get(1).unwrap().as_str();
                    if let Ok(program_id) = program_id_str.parse::<Pubkey>() {
                        return ParsedLog::Consumed {
                            program_id,
                            original_idx: idx,
                        };
                    }
                }

                ParsedLog::Other { original_idx: idx }
            })
            .collect()
    }

    /// Recursively assign logs to instructions using two-pointer algorithm
    fn assign_logs_recursive(
        instructions: &mut [Self],
        parsed_logs: &[ParsedLog],
        log_idx: &mut usize,
        current_depth: usize,
    ) -> Vec<usize> {
        let mut parent_logs = Vec::new();

        for instruction in instructions {
            // Find this instruction's invoke log at current_depth
            let mut found_invoke = false;
            while *log_idx < parsed_logs.len() {
                match &parsed_logs[*log_idx] {
                    ParsedLog::Invoke {
                        program_id,
                        depth,
                        original_idx,
                    } if *depth == current_depth && *program_id == instruction.program => {
                        instruction.parsed_logs.push(*original_idx);
                        *log_idx += 1;
                        found_invoke = true;
                        break;
                    },
                    // If we encounter logs that don't match, they might belong to parent
                    ParsedLog::Other { original_idx }
                    | ParsedLog::Consumed { original_idx, .. } => {
                        parent_logs.push(*original_idx);
                        *log_idx += 1;
                    },
                    // If we hit an invoke at shallower depth, stop searching
                    ParsedLog::Invoke { depth, .. } if *depth < current_depth => {
                        break;
                    },
                    // Otherwise keep searching
                    _ => {
                        *log_idx += 1;
                    },
                }
            }

            if !found_invoke {
                // Log mismatch - skip this instruction
                continue;
            }

            // Consume logs until we return to current_depth
            loop {
                if *log_idx >= parsed_logs.len() {
                    break;
                }

                match &parsed_logs[*log_idx] {
                    // Child invoke at deeper depth → recurse
                    ParsedLog::Invoke { depth, .. } if *depth == current_depth + 1 => {
                        if instruction.inner.is_empty() {
                            // No inner instructions but found deeper invoke - skip
                            *log_idx += 1;
                        } else {
                            let returned_logs = Self::assign_logs_recursive(
                                &mut instruction.inner,
                                parsed_logs,
                                log_idx,
                                current_depth + 1,
                            );
                            // Assign the returned logs (that belonged to this parent) to this instruction
                            instruction.parsed_logs.extend(returned_logs);
                        }
                    },

                    // Success/failed at current depth for this program → done with this instruction
                    ParsedLog::Success {
                        program_id,
                        original_idx,
                    }
                    | ParsedLog::Failed {
                        program_id,
                        original_idx,
                    } if *program_id == instruction.program => {
                        instruction.parsed_logs.push(*original_idx);
                        *log_idx += 1;
                        break;
                    },

                    // Other logs or consumed → assign to current instruction
                    ParsedLog::Other { original_idx }
                    | ParsedLog::Consumed { original_idx, .. } => {
                        instruction.parsed_logs.push(*original_idx);
                        *log_idx += 1;
                    },

                    // Invoke at same or shallower depth → we're done with this instruction
                    ParsedLog::Invoke { depth, .. } if *depth <= current_depth => {
                        break;
                    },

                    // Success/failed for different program → assign and continue
                    _ => {
                        if let Some(idx) = match &parsed_logs[*log_idx] {
                            ParsedLog::Success { original_idx, .. }
                            | ParsedLog::Failed { original_idx, .. } => Some(*original_idx),
                            _ => None,
                        } {
                            instruction.parsed_logs.push(idx);
                        }
                        *log_idx += 1;
                    },
                }
            }
        }

        parent_logs
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
    pub fn visit_all(&self) -> VisitAll<'_> { VisitAll::new(self) }
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

    #[tokio::test]
    #[allow(clippy::too_many_lines)]
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
            println!("\n=== Instruction {i} ===");
            println!(
                "Program: {}, Parsed logs count: {}, Raw logs count: {}",
                ix.program,
                ix.parsed_logs.len(),
                ix.shared.log_messages.len()
            );

            // Print all parsed logs for this instruction
            println!("Parsed logs:");
            for (j, log_idx) in ix.parsed_logs.iter().enumerate() {
                if let Some(log) = ix.shared.log_messages.get(*log_idx) {
                    println!("  Parsed log {j} (idx={log_idx}): {log}");
                }
            }

            // Test inner instructions
            for (inner_i, inner_ix) in ix.inner.iter().enumerate() {
                println!("\n  === Inner Instruction {inner_i} ===");
                println!(
                    "  Program: {}, Parsed logs count: {}",
                    inner_ix.program,
                    inner_ix.parsed_logs.len()
                );

                // Print all parsed logs for inner instructions
                println!("  Inner parsed logs:");
                for (j, log_idx) in inner_ix.parsed_logs.iter().enumerate() {
                    if let Some(log) = inner_ix.shared.log_messages.get(*log_idx) {
                        println!("    Inner parsed log {j} (idx={log_idx}): {log}");
                    }
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

        // Verify log indices are valid and in ascending order
        for (i, ix) in instruction_updates.iter().enumerate() {
            for (j, &log_idx) in ix.parsed_logs.iter().enumerate() {
                assert!(
                    log_idx < ix.shared.log_messages.len(),
                    "Instruction {i}: log index {log_idx} at position {j} is out of bounds (total \
                     logs: {})",
                    ix.shared.log_messages.len()
                );

                // Verify logs are in ascending order (chronological)
                if j > 0 {
                    assert!(
                        log_idx > ix.parsed_logs[j - 1],
                        "Instruction {i}: logs out of order at position {j}: {} <= {}",
                        log_idx,
                        ix.parsed_logs[j - 1]
                    );
                }
            }

            // Recursively check inner instructions
            for (inner_i, inner_ix) in ix.inner.iter().enumerate() {
                for (j, &log_idx) in inner_ix.parsed_logs.iter().enumerate() {
                    assert!(
                        log_idx < inner_ix.shared.log_messages.len(),
                        "Instruction {i}.{inner_i}: log index {log_idx} at position {j} is out of \
                         bounds"
                    );

                    // Verify logs are in ascending order
                    if j > 0 {
                        assert!(
                            log_idx > inner_ix.parsed_logs[j - 1],
                            "Instruction {i}.{inner_i}: logs out of order at position {j}"
                        );
                    }
                }
            }
        }

        // Verify specific instruction log patterns
        // Instruction 0: ComputeBudget should have invoke + success
        assert_eq!(
            instruction_updates[0].parsed_logs.len(),
            2,
            "ComputeBudget instruction should have 2 logs (invoke + success)"
        );
        assert!(
            instruction_updates[0].shared.log_messages[instruction_updates[0].parsed_logs[0]]
                .contains("invoke [1]"),
            "First log should be invoke at depth 1"
        );
        assert!(
            instruction_updates[0].shared.log_messages[instruction_updates[0].parsed_logs[1]]
                .contains("success"),
            "Last log should be success"
        );

        // Instruction 2: JUP6 should have inner instructions with logs
        assert!(
            !instruction_updates[2].inner.is_empty(),
            "JUP6 instruction should have inner instructions"
        );
        assert!(
            !instruction_updates[2].inner[0].parsed_logs.is_empty(),
            "Inner instruction 0 should have logs assigned"
        );

        // Verify that inner instruction logs are within parent's log range
        let parent_first_log = instruction_updates[2].parsed_logs[0];
        let parent_last_log = *instruction_updates[2].parsed_logs.last().unwrap();
        let inner_first_log = instruction_updates[2].inner[0].parsed_logs[0];

        assert!(
            inner_first_log > parent_first_log && inner_first_log < parent_last_log,
            "Inner instruction logs should be within parent's log range: parent \
             [{parent_first_log}..{parent_last_log}], inner starts at {inner_first_log}"
        );

        // Verify depth markers in logs
        for ix in &instruction_updates {
            if !ix.parsed_logs.is_empty() {
                let first_log = &ix.shared.log_messages[ix.parsed_logs[0]];
                assert!(
                    first_log.contains("invoke [1]"),
                    "Top-level instruction should start with 'invoke [1]': {first_log}"
                );
            }

            for inner_ix in &ix.inner {
                if !inner_ix.parsed_logs.is_empty() {
                    let first_log = &inner_ix.shared.log_messages[inner_ix.parsed_logs[0]];
                    assert!(
                        first_log.contains("invoke [2]") || first_log.contains("invoke [3]"),
                        "Inner instruction should start with 'invoke [2]' or deeper: {first_log}"
                    );
                }
            }
        }

        println!("✓ Instruction log assignment test completed successfully");
        println!(
            "✓ {} out of {} instructions have assigned logs",
            instructions_with_logs,
            instruction_updates.len()
        );
        println!("✓ All log indices are valid and chronologically ordered");
        println!("✓ Log depth markers match instruction hierarchy");
    }

    // Helper function to recursively print instruction hierarchy
    fn print_instruction_hierarchy(
        ix: &super::InstructionUpdate,
        depth: usize,
        instruction_path: &str,
    ) {
        let indent = "  ".repeat(depth);
        let parent_info = if let Some(parent_program) = ix.parent_program {
            format!("parent: {parent_program}")
        } else {
            "TOP-LEVEL".to_string()
        };

        println!(
            "{indent}{instruction_path}Instruction : ix_index={}, {parent_info}, program={}",
            ix.ix_index, ix.program
        );

        // Recursively print inner instructions
        for (inner_i, inner_ix) in ix.inner.iter().enumerate() {
            let inner_path = format!("{instruction_path}[inner-{inner_i}] ");
            print_instruction_hierarchy(inner_ix, depth + 1, &inner_path);
        }
    }

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
                    ix.parent_program,
                    Some(expected),
                    "Inner instruction parent_program {:?} doesn't match expected parent {}",
                    ix.parent_program,
                    expected
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

    #[tokio::test]
    #[allow(clippy::too_many_lines, clippy::items_after_statements)]
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

        // Print hierarchy for all top-level instructions
        for (i, ix) in instruction_updates.iter().enumerate() {
            println!("\n=== Top-level Instruction {i} ===");
            print_instruction_hierarchy(ix, 0, "");
        }

        // Verify parent-child relationships
        let mut total_instructions = 0;
        let mut top_level_count = 0;
        let mut inner_instructions_count = 0;

        // Count and verify all instructions
        for ix in &instruction_updates {
            let (count, top, inner_count) = verify_instruction_hierarchy(ix, None);
            total_instructions += count;
            top_level_count += top;
            inner_instructions_count += inner_count;
        }

        println!("\n=== Parent-Child Relationship Summary ===");
        println!("Total instructions: {total_instructions}");
        println!("Top-level instructions: {top_level_count}");
        println!("Inner instructions: {inner_instructions_count}");

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

        assert_eq!(
            total_instructions,
            top_level_count + inner_instructions_count,
            "Total instructions should equal top-level + inner instructions"
        );

        // Verify we have the expected structure from the fixture
        assert_eq!(
            instruction_updates.len(),
            4,
            "Should have 4 top-level instructions"
        );

        assert_eq!(
            total_instructions, 13,
            "Should have 13 total instructions (4 top-level + 9 inner)"
        );

        assert_eq!(
            inner_instructions_count, 9,
            "Should have 9 inner instructions"
        );

        // Verify specific instruction has inner instructions
        let jup_instruction = &instruction_updates[2]; // JUP6 instruction
        assert!(
            !jup_instruction.inner.is_empty(),
            "JUP6 instruction should have inner instructions"
        );
        assert_eq!(
            jup_instruction.inner.len(),
            2,
            "JUP6 instruction should have 2 direct inner instructions"
        );

        // Verify first inner instruction (Eo7WjKq...) has correct parent and nested structure
        let first_inner = &jup_instruction.inner[0];
        assert_eq!(
            first_inner.parent_program,
            Some(jup_instruction.program),
            "First inner instruction should have JUP6 as parent"
        );
        assert_eq!(
            first_inner.inner.len(),
            3,
            "First inner instruction (Eo7WjKq) should have 3 nested inner instructions"
        );

        // Verify nested inner instructions have correct parent chain
        let nested_inner = &first_inner.inner[1]; // 24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi
        assert_eq!(
            nested_inner.parent_program,
            Some(first_inner.program),
            "Nested inner instruction should have Eo7WjKq as parent"
        );
        assert_eq!(
            nested_inner.inner.len(),
            2,
            "Nested inner instruction (24Uqj9) should have 2 children"
        );

        // Verify second inner instruction (recursive CPI to JUP6)
        let second_inner = &jup_instruction.inner[1];
        assert_eq!(
            second_inner.parent_program,
            Some(jup_instruction.program),
            "Second inner instruction should have JUP6 as parent"
        );
        assert_eq!(
            second_inner.program, jup_instruction.program,
            "Second inner instruction is a recursive call to JUP6"
        );

        // Verify ix_index uniqueness and sequential ordering
        let mut seen_indices = std::collections::HashSet::new();
        let mut max_idx = 0u16;

        fn collect_indices(
            ix: &super::InstructionUpdate,
            seen: &mut std::collections::HashSet<u16>,
            max: &mut u16,
        ) {
            assert!(
                seen.insert(ix.ix_index),
                "Duplicate ix_index found: {}",
                ix.ix_index
            );
            *max = (*max).max(ix.ix_index);

            for inner_ix in &ix.inner {
                collect_indices(inner_ix, seen, max);
            }
        }

        for ix in &instruction_updates {
            collect_indices(ix, &mut seen_indices, &mut max_idx);
        }

        assert_eq!(
            seen_indices.len(),
            total_instructions,
            "All instructions should have unique ix_index"
        );

        // Verify indices are sequential from 0
        #[allow(clippy::cast_possible_truncation)]
        for i in 0..total_instructions {
            assert!(seen_indices.contains(&(i as u16)), "Missing ix_index: {i}");
        }

        // Verify parent programs exist in the transaction (recursively collect all programs)
        fn collect_all_programs(
            ix: &super::InstructionUpdate,
            programs: &mut std::collections::HashSet<super::Pubkey>,
        ) {
            programs.insert(ix.program);
            for inner_ix in &ix.inner {
                collect_all_programs(inner_ix, programs);
            }
        }

        fn verify_parent_exists(
            ix: &super::InstructionUpdate,
            all_programs: &std::collections::HashSet<super::Pubkey>,
        ) {
            if let Some(parent) = ix.parent_program {
                assert!(
                    all_programs.contains(&parent),
                    "Parent program {parent} not found in transaction"
                );
            }

            for inner_ix in &ix.inner {
                verify_parent_exists(inner_ix, all_programs);
            }
        }

        let mut all_programs = std::collections::HashSet::new();
        for ix in &instruction_updates {
            collect_all_programs(ix, &mut all_programs);
        }

        for ix in &instruction_updates {
            verify_parent_exists(ix, &all_programs);
        }

        println!("✓ Parent-child relationship test completed successfully");
        println!("✓ All parent relationships verified correctly");
        println!("✓ ix_index values are unique and sequential");
        println!("✓ All parent programs exist in the transaction");
    }

    #[tokio::test]
    async fn test_parse_created_token_accounts() {
        use yellowstone_vixen_mock::create_mock_transaction_update;

        let fixture_signature = "3BjJ4XxXi1ttr58wbT8Je3z75YwsH6mhQUcC46HSJUW69ANNeVmcixsx3FTJK459JiSr5jNMijtjcN6nQ2kR5ucG";
        let expected_token_account = "26JJhFPRFNbH7Q9gdbbsyzb8TdQuA7CYJ1Zn4cg4PebD";

        // Create a mock transaction update using the utility
        let transaction_update = create_mock_transaction_update(fixture_signature)
            .await
            .expect("Failed to create mock transaction update");

        // Parse instructions using the core parse_from_txn logic
        let instruction_updates = super::InstructionUpdate::parse_from_txn(&transaction_update)
            .expect("Failed to parse instructions from transaction update");

        // Verify that we have some instructions
        assert!(
            !instruction_updates.is_empty(),
            "Should have parsed some instructions"
        );

        // Get the created token accounts from the shared data
        let created_token_accounts = &instruction_updates[0].shared.created_token_accounts;

        println!(
            "Found {} created token accounts",
            created_token_accounts.len()
        );
        for (i, account) in created_token_accounts.iter().enumerate() {
            let account_str = bs58::encode(&account.account.0).into_string();
            let mint_str = bs58::encode(&account.mint.0).into_string();
            let owner_str = bs58::encode(&account.owner.0).into_string();
            println!(
                "Created token account {i}: account={account_str}, mint={mint_str}, \
                 owner={owner_str}"
            );
        }

        // Parse the expected token account pubkey
        let expected_account_bytes: [u8; 32] = bs58::decode(expected_token_account)
            .into_vec()
            .expect("Failed to decode expected token account")
            .try_into()
            .expect("Invalid pubkey length");

        // Check if the expected token account exists in created_token_accounts
        let found = created_token_accounts
            .iter()
            .any(|account| account.account.0 == expected_account_bytes);

        assert!(
            found,
            "Expected token account {expected_token_account} should exist in \
             created_token_accounts"
        );

        println!("✓ Token account {expected_token_account} found in created_token_accounts");
    }
}
