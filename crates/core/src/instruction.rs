//! Helpers for parsing transaction updates into instructions.

use std::{collections::VecDeque, fmt::Debug, sync::Arc};

use yellowstone_grpc_proto::{
    geyser::SubscribeUpdateTransactionInfo,
    prelude::MessageHeader,
    solana::storage::confirmed_block::{
        CompiledInstruction, InnerInstruction, InnerInstructions, Message, Reward, TokenBalance,
        Transaction, TransactionError, TransactionStatusMeta,
    },
};

use crate::{log_messages::assign_log_messages, Pubkey, TransactionUpdate};

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
#[derive(Debug, Clone)]
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
    /// The path of this instruction within the transaction.
    pub path: Path,
    /// Range into `shared.log_messages` for this instruction's logs.
    pub log_range: std::ops::Range<usize>,
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

#[derive(Clone, PartialEq, Eq, Hash)]
/// 0-based indices representing the path to the instruction
// typically one or two elements long, but can be 3+ levels deep
// empty path makes no sense, but is allowed for completeness
pub struct Path(Vec<u32>);

impl Path {
    /// Create a new empty instruction path.
    #[must_use]
    pub fn new_single(idx: u32) -> Self {
        let mut path_idx = Vec::with_capacity(4);
        path_idx.push(idx);
        Self(path_idx)
    }

    /// Push a new index onto the instruction path.
    #[must_use]
    pub fn push_clone(&self, idx: u32) -> Self {
        let mut path_idx = self.0.clone();
        path_idx.push(idx);
        Self(path_idx)
    }

    /// Get the current instruction path as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[u32] { &self.0 }

    /// Get the length of the instruction path.
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    /// Check if the instruction path is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Check if this instruction path is a (direct) parent of another instruction path.
    #[must_use]
    pub fn is_parent_of(&self, other: &Path) -> bool {
        if self.len() + 1 != other.len() {
            return false;
        }
        let same_prefix_len = self.len();
        other.0[..same_prefix_len] == self.0[..same_prefix_len]
    }

    /// Check if this instruction path is an ancestor of another instruction path.
    #[must_use]
    pub fn is_ancestor_of(&self, other: &Path) -> bool {
        if self.len() >= other.len() {
            return false;
        }
        let same_prefix_len = self.len();
        other.0[..same_prefix_len] == self.0[..same_prefix_len]
    }
}

impl From<Vec<u32>> for Path {
    fn from(value: Vec<u32>) -> Self { Self(value) }
}

impl Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 1.7
        let formatted = self
            .0
            .iter()
            .map(|i| (i + 1).to_string())
            .collect::<Vec<_>>()
            .join(".");

        f.write_str(&formatted)
    }
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
    /// Returns the log messages for this specific instruction.
    ///
    /// This is a zero-copy slice into the shared transaction log messages.
    #[must_use]
    pub fn log_messages(&self) -> &[String] { &self.shared.log_messages[self.log_range.clone()] }

    /// Returns the log messages emitted *directly* by this instruction's
    /// program, excluding lines emitted while an inner CPI is on top of the
    /// invocation stack.
    ///
    /// Unlike [`log_messages`](Self::log_messages), which yields every line
    /// inside this instruction's `invoke`/`success` window (including nested
    /// CPI logs), this iterator skips any line that occurs while a deeper
    /// `Program ... invoke [N+1]` is active.
    ///
    /// The opening `Program <id> invoke [N]` and closing
    /// `Program <id> success`/`failed:` lines for this instruction are
    /// included; the lines between them are filtered to depth-0 only.
    ///
    /// Example output for an outer Raydium ix that CPIs to the token program:
    ///
    /// ```text,ignore
    /// Program RAY invoke [1]
    /// Program log: ray_log: swap-base-in
    /// Program log: ray_log: swap done
    /// Program RAY success
    /// ```
    ///
    /// (Token-program lines emitted at depth 2 are skipped.)
    pub fn direct_log_messages(&self) -> impl Iterator<Item = &str> {
        use crate::log_messages::{classify_log_line, LogLineKind};

        let lines = self.log_messages();
        let mut depth: u32 = 0;

        lines.iter().filter_map(move |line| {
            match classify_log_line(line) {
                LogLineKind::Invoke => {
                    depth += 1;
                    // Only the outermost invoke (this instruction's own) is at
                    // depth 1 after the increment; deeper invokes are skipped.
                    (depth == 1).then_some(line.as_str())
                }
                LogLineKind::Close => {
                    let was_outer = depth == 1;
                    depth = depth.saturating_sub(1);
                    was_outer.then_some(line.as_str())
                }
                LogLineKind::Other => (depth == 1).then_some(line.as_str()),
            }
        })
    }

    /// Build instruction updates from a transaction update.
    ///
    /// # Errors
    /// Returns an error if the transaction update received is in an unbuildable
    /// form.
    #[deprecated(note = "use InstructionUpdate::build_from_txn instead")]
    pub fn parse_from_txn(txn: &TransactionUpdate) -> Result<Vec<Self>, ParseError> {
        Self::build_from_txn(txn)
    }

    /// Build instruction updates from a transaction update.
    ///
    /// # Errors
    /// Returns an error if the transaction update received is in an unbuildable
    /// form.
    pub fn build_from_txn(txn: &TransactionUpdate) -> Result<Vec<Self>, ParseError> {
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

        #[allow(clippy::cast_possible_truncation)] // instruction count never exceeds u32::MAX
        let mut outer = instructions
            .into_iter()
            .enumerate()
            .map(|(idx, i)| {
                Self::build_outer_instruction(Arc::clone(&shared), i, Path::new_single(idx as u32))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Self::attach_inner_instructions(&shared, inner_instructions, &mut outer)?;

        assign_log_messages(&shared.log_messages, &mut outer);

        Ok(outer)
    }

    // Called once per transaction to reconstruct nested CPI instructions and
    // attach them to their outer parent.
    fn attach_inner_instructions(
        shared: &Arc<InstructionShared>,
        inner_instructions: Vec<InnerInstructions>,
        outer: &mut [Self],
    ) -> Result<(), ParseError> {
        for insn in inner_instructions {
            let InnerInstructions {
                index: index_outer,
                instructions,
            } = insn;

            let Some(outer) = index_outer
                .try_into()
                .ok()
                .and_then(|i: usize| outer.get_mut(i))
            else {
                return Err(ParseError::InvalidInnerInstructionIndex(index_outer));
            };

            let heights: Vec<Option<u32>> =
                instructions.iter().map(|ins| ins.stack_height).collect();
            let paths_at_index = derive_paths_from_stackheights(&heights, index_outer);

            let mut inner = instructions
                .into_iter()
                .enumerate()
                .map(|(idx, i)| {
                    Self::build_inner_instruction(
                        Arc::clone(shared),
                        i,
                        paths_at_index[idx].clone(),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;

            if let Some(mut i) = inner.len().checked_sub(1) {
                while i > 0 {
                    let parent_idx = i - 1;
                    let Some(height) = inner[parent_idx].1 else {
                        // stack_height missing for old data
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
    fn build_outer_instruction(
        shared: Arc<InstructionShared>,
        ins: CompiledInstruction,
        path: Path,
    ) -> Result<Self, ParseError> {
        let CompiledInstruction {
            program_id_index,
            ref accounts,
            data,
        } = ins;
        Self::build_instruction(shared, program_id_index, accounts, data, path)
    }

    fn build_inner_instruction(
        shared: Arc<InstructionShared>,
        ins: InnerInstruction,
        path: Path,
    ) -> Result<(Self, Option<u32>), ParseError> {
        let InnerInstruction {
            program_id_index,
            ref accounts,
            data,
            stack_height,
        } = ins;
        Self::build_instruction(shared, program_id_index, accounts, data, path)
            .map(|i| (i, stack_height))
    }

    fn build_instruction(
        shared: Arc<InstructionShared>,
        program_id_index: u32,
        accounts: &[u8],
        data: Vec<u8>,
        path: Path,
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
            path,
            log_range: 0..0,
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

///
/// Derive instruction paths from the flat `stack_heights` array returned by the Solana runtime.
///
/// Each inner instruction carries a stack height indicating its nesting depth
/// (1 = top-level CPI, 2 = CPI called by a CPI, etc.).
/// This function reconstructs the full tree path for every instruction
/// by tracking a virtual stack of child indices.
///
/// ## Example
///
/// Given outer instruction index 3 and these inner instructions:
///
/// ```text
/// stack_heights: [1, 1, 2, 2, 1]
///
/// ix[0]: height=1  →  Path [3, 0]       (first child of outer ix 3)
/// ix[1]: height=1  →  Path [3, 1]       (second child, same level)
/// ix[2]: height=2  →  Path [3, 1, 0]    (first grandchild under ix[1])
/// ix[3]: height=2  →  Path [3, 1, 1]    (second grandchild, same level)
/// ix[4]: height=1  →  Path [3, 2]       (back up to child level)
/// ```
///
fn derive_paths_from_stackheights(stack_heights: &[Option<u32>], outer_index: u32) -> Vec<Path> {
    if stack_heights.is_empty() {
        return Vec::new();
    }

    let mut paths: Vec<Path> = Vec::with_capacity(stack_heights.len());

    // `path_stack` tracks the current position in the call tree as a list of child indices.
    // e.g. [3, 1, 0] means: outer instruction 3 → child 1 → grandchild 0.
    let mut path_stack: Vec<u32> = Vec::with_capacity(4);

    path_stack.push(outer_index);
    path_stack.push(0);
    paths.push(Path(path_stack.clone()));

    for (i, ref current_height) in stack_heights.iter().enumerate().skip(1) {
        let (Some(current_height), Some(prev_height)) = (current_height, stack_heights[i - 1])
        else {
            // Stack height missing — assume same level as previous instruction.
            if let Some(last) = path_stack.last_mut() {
                *last += 1;
            }

            paths.push(Path(path_stack.clone()));

            continue;
        };

        match current_height.cmp(&prev_height) {
            std::cmp::Ordering::Greater => {
                // CPI call: descend one level (stack height always increments by exactly 1).
                debug_assert_eq!(
                    *current_height,
                    prev_height + 1,
                    "invalid stack heights: {stack_heights:?}"
                );

                path_stack.push(0);
            },
            std::cmp::Ordering::Equal => {
                // Sibling: same depth, advance to next child index.
                if let Some(last) = path_stack.last_mut() {
                    *last += 1;
                }
            },
            std::cmp::Ordering::Less => {
                // Return from CPI: may skip multiple levels at once (e.g. height 3 → 1).
                path_stack.truncate(*current_height as usize);

                if let Some(last) = path_stack.last_mut() {
                    *last += 1;
                }
            },
        }

        paths.push(Path(path_stack.clone()));
    }

    debug_assert_eq!(
        paths.len(),
        stack_heights.len(),
        "derived paths failed for {stack_heights:?}"
    );

    paths
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ix_path_parent() {
        use super::Path;

        let p_empty = Path::from(vec![]);
        let p0 = Path::from(vec![0]);
        let p1 = Path::from(vec![0, 1]);
        let p2 = Path::from(vec![0, 1, 2]);
        let p3 = Path::from(vec![0, 2]);
        let p4 = Path::from(vec![0, 1, 2, 3]);

        assert!(p_empty.is_parent_of(&p0));
        assert!(p1.is_parent_of(&p2));
        assert!(!p1.is_parent_of(&p4));
        assert!(!p2.is_parent_of(&p1));
        assert!(!p1.is_parent_of(&p3));
        assert!(!p1.is_parent_of(&p1));
    }

    #[test]
    fn test_ix_path_ancestor() {
        use super::Path;

        let p1 = Path::from(vec![0, 1]);
        let p2 = Path::from(vec![0, 1, 2]);
        let p3 = Path::from(vec![0, 2]);
        let p4 = Path::from(vec![0, 1, 2, 3]);

        assert!(p1.is_ancestor_of(&p2));
        assert!(p1.is_ancestor_of(&p4));
        assert!(!p2.is_ancestor_of(&p1));
        assert!(!p1.is_ancestor_of(&p3));
        assert!(!p1.is_parent_of(&p1));
    }
}
