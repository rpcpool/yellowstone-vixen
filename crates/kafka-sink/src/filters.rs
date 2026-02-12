//! Secondary filters for routing instructions to additional topics.

use std::{future::Future, pin::Pin, sync::LazyLock};

use yellowstone_vixen_core::{bs58, instruction::InstructionUpdate};

use crate::sink::{ParsedInstruction, SecondaryFilter};

/// JUP token mint address
const JUP_MINT: &str = "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN";

/// SPL Token program ID (decoded to raw bytes for zero-allocation comparison).
static TOKEN_PROGRAM_BYTES: LazyLock<[u8; 32]> = LazyLock::new(|| {
    let mut buf = [0u8; 32];
    bs58::decode("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
        .onto(&mut buf)
        .expect("valid base58");
    buf
});

/// SPL Token instruction discriminants
const SPL_TOKEN_TRANSFER: u8 = 3;
const SPL_TOKEN_TRANSFER_CHECKED: u8 = 12;

/// Filter that routes JUP token transfers to a dedicated topic.
/// Checks pre_token_balances to identify transfers involving JUP mint.
pub struct JupTransferFilter {
    topic: String,
}

impl JupTransferFilter {
    pub fn new(topic: &str) -> Self {
        Self {
            topic: topic.to_string(),
        }
    }

    /// Check if any token balance in the transaction involves JUP mint
    /// Checks both pre and post balances to catch all cases:
    /// - pre: source account had JUP before transfer
    /// - post: destination account has JUP after (covers newly created accounts)
    fn has_jup_token_balance(ix: &InstructionUpdate) -> bool {
        ix.shared
            .pre_token_balances
            .iter()
            .chain(ix.shared.post_token_balances.iter())
            .any(|tb| tb.mint == JUP_MINT)
    }

    /// Check if this is an SPL Token program instruction
    fn is_token_program(ix: &InstructionUpdate) -> bool {
        AsRef::<[u8; 32]>::as_ref(&ix.program) == TOKEN_PROGRAM_BYTES.as_ref()
    }

    /// Check if instruction is a Transfer or TransferChecked
    fn is_transfer_instruction(ix: &InstructionUpdate) -> bool {
        if ix.data.is_empty() {
            return false;
        }
        let discriminant = ix.data[0];
        discriminant == SPL_TOKEN_TRANSFER || discriminant == SPL_TOKEN_TRANSFER_CHECKED
    }
}

impl SecondaryFilter for JupTransferFilter {
    fn filter<'a>(
        &'a self,
        ix: &'a InstructionUpdate,
        primary_parsed: Option<&'a ParsedInstruction>,
    ) -> Pin<Box<dyn Future<Output = Option<ParsedInstruction>> + Send + 'a>> {
        Box::pin(async move {
            // Only process if primary parser decoded it
            let parsed = primary_parsed?;

            // Check if instruction is from Token program
            if !Self::is_token_program(ix) {
                return None;
            }

            // Check if it's a transfer instruction (discriminant 3 or 12)
            if !Self::is_transfer_instruction(ix) {
                return None;
            }

            // Check if JUP is involved in this transaction's token balances
            if !Self::has_jup_token_balance(ix) {
                return None;
            }

            // Reuse the already-parsed instruction data
            Some(parsed.clone())
        })
    }

    fn topic(&self) -> &str { &self.topic }

    fn label(&self) -> &str { "jup-transfer" }
}
