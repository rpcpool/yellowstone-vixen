use serde_json::Value;
use spl_pod::solana_program::{instruction::CompiledInstruction, message::AccountKeys};
#[cfg(feature = "token-program")]
use spl_token::instruction::TokenInstruction;
#[cfg(feature = "token-extensions")]
use spl_token_2022::instruction::TokenInstruction as TokenExtensionInstruction;
use yellowstone_grpc_proto::prelude::{
    InnerInstruction, InnerInstructions, ReturnData, Reward, TokenBalance,
};
use yellowstone_vixen_core::{ParseResult, TransactionUpdate};

use super::helpers::{get_account_from_index, get_address_lookup_table_keys, *};

#[derive(Debug)]
pub enum TransactionIxData {
    #[cfg(feature = "token-program")]
    TokenProgramIx(TokenInstruction<'static>),
    #[cfg(feature = "token-extensions")]
    TokenExtensionProgramIx(TokenExtensionInstruction<'static>),
    UnknownIx(Value),
}

#[derive(Debug)]
pub struct TxReturnData {
    pub program_id: String,
    pub data: String,
}

impl From<ReturnData> for TxReturnData {
    fn from(data: ReturnData) -> Self {
        TxReturnData {
            program_id: data.program_id.to_base58(),
            data: data.data.to_base58(),
        }
    }
}

#[derive(Debug)]
pub struct IxsInfo {
    pub all_ixs: Vec<String>,
    pub filtered_parsed_ixs: Vec<ParsedIx>,
}

#[derive(Debug)]
pub struct ReadableInstructions {
    pub index: u32,
    pub instructions: Vec<ReadableInstruction>,
}

impl<'i> ReadableInstructions {
    fn try_from_inner_ixs(
        inner_ixs: InnerInstructions,
        tx_account_pubkeys: &Vec<String>,
        loaded_readonly_addresses: &Vec<String>,
        loaded_writeable_addresses: &Vec<String>,
    ) -> Result<ReadableInstructions, String> {
        let instructions = inner_ixs
            .instructions
            .into_iter()
            .enumerate()
            .map(|(idx, ix)| {
                ReadableInstruction::try_from_inner_ix(
                    ix,
                    idx,
                    &tx_account_pubkeys,
                    &loaded_readonly_addresses,
                    &loaded_writeable_addresses,
                )
            })
            .collect::<Result<Vec<ReadableInstruction>, String>>()?;
        Ok(ReadableInstructions {
            index: inner_ixs.index,
            instructions,
        })
    }
}

#[derive(Debug)]
pub struct ReadableInstruction {
    pub index: u32,
    pub program: String,
    pub program_id: String,
    pub parsed: Value,
    pub stack_height: Option<u32>,
    pub ix_data: TransactionIxData,
}

impl ReadableInstruction {
    pub fn try_from_inner_ix(
        inner_ix: InnerInstruction,
        index: usize,
        tx_account_pubkeys: &Vec<String>,
        loaded_readonly_addresses: &Vec<String>,
        loaded_writeable_addresses: &Vec<String>,
    ) -> Result<ReadableInstruction, String> {
        let program_id =
            get_account_from_index(inner_ix.program_id_index as usize, tx_account_pubkeys)?;

        let compiled_ix = CompiledInstruction {
            program_id_index: inner_ix.program_id_index as u8,
            accounts: inner_ix.accounts.clone(),
            data: inner_ix.data,
        };

        let address_lookup_table_keys = get_address_lookup_table_keys(
            tx_account_pubkeys,
            loaded_readonly_addresses,
            loaded_writeable_addresses,
        )?;

        let address_lookup_table_keys: AccountKeys = AccountKeys::new(
            &address_lookup_table_keys.static_keys,
            address_lookup_table_keys.dynamic_keys.as_ref(),
        );

        let parsed_ix = solana_transaction_status::parse_instruction::parse(
            &program_id.to_pubkey()?,
            &compiled_ix,
            &address_lookup_table_keys,
            inner_ix.stack_height,
        )
        .map_err(|e| e.to_string())?;

        let ix_data =
            get_ix_data(&program_id, &parsed_ix.parsed).ok_or("Error in parsing Ix Data")?;

        Ok(ReadableInstruction {
            index: index as u32,
            program: parsed_ix.program,
            program_id,
            parsed: parsed_ix.parsed,
            stack_height: parsed_ix.stack_height,
            ix_data,
        })
    }
}

#[derive(Debug)]
pub struct VixenTransaction {
    pub signature: String, // Signature is a base58 encoded string
    pub is_versioned: bool,
    pub recent_blockhash: String,
    pub log_messages: Option<Vec<String>>,
    pub fee: u64,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    pub readable_instructions: Vec<ReadableInstructions>,
    pub pre_token_balances: Option<Vec<TokenBalance>>,
    pub post_token_balances: Option<Vec<TokenBalance>>,
    pub rewards: Vec<Reward>,
    pub loaded_readonly_addresses: Vec<String>,
    pub loaded_writeable_addresses: Vec<String>,
    pub return_data: Option<TxReturnData>,
    pub compute_units_consumed: Option<u64>,
    pub tx_account_pubkeys: Vec<String>,
}

impl VixenTransaction {
    pub fn try_from_tx_update(update: TransactionUpdate) -> ParseResult<Self> {
        let outer_tx = update.transaction.ok_or("Transaction not found")?;
        let inner_tx = outer_tx.transaction.ok_or("Inner transaction not found")?;
        let tx_message_data = inner_tx.message.ok_or("Transaction message not found")?;
        let tx_meta = outer_tx.meta.ok_or("Transaction meta not found")?;
        let tx_account_pubkeys = tx_message_data.account_keys.to_pubkey_vec()?;
        let tx_return_data = tx_meta.return_data.map_or(None, |data| Some(data.into()));
        let loaded_readonly_addresses = tx_meta.loaded_readonly_addresses.to_pubkey_vec()?;
        let loaded_writeable_addresses = tx_meta.loaded_writable_addresses.to_pubkey_vec()?;
        let log_messages = tx_meta.log_messages.check_and_return_vec();

        let readable_instructions = tx_meta
            .inner_instructions
            .into_iter()
            .map(|ixs| {
                ReadableInstructions::try_from_inner_ixs(
                    ixs,
                    &tx_account_pubkeys,
                    &loaded_readonly_addresses,
                    &loaded_writeable_addresses,
                )
            })
            .collect::<Result<Vec<ReadableInstructions>, String>>()?;

        let vixen_tx = VixenTransaction {
            signature: outer_tx.signature.to_base58(),
            fee: tx_meta.fee,
            pre_balances: tx_meta.pre_balances,
            post_balances: tx_meta.post_balances,
            log_messages,
            pre_token_balances: tx_meta.pre_token_balances.check_and_return_vec(),
            post_token_balances: tx_meta.post_token_balances.check_and_return_vec(),
            rewards: tx_meta.rewards,
            loaded_writeable_addresses: tx_meta.loaded_writable_addresses.to_pubkey_vec()?,
            loaded_readonly_addresses: tx_meta.loaded_readonly_addresses.to_pubkey_vec()?,
            return_data: tx_return_data,
            compute_units_consumed: tx_meta.compute_units_consumed,
            is_versioned: tx_message_data.versioned,
            recent_blockhash: tx_message_data.recent_blockhash.to_base58(),
            tx_account_pubkeys,
            readable_instructions,
        };
        // println!("Vixen Tx {:#?}", vixen_tx);
        Ok(vixen_tx)
    }
}
