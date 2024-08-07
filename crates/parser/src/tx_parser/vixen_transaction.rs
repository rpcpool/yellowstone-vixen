use spl_token::solana_program::pubkey::Pubkey;
use yellowstone_grpc_proto::prelude::{ReturnData, Reward, TokenBalance};
use yellowstone_vixen_core::TransactionUpdate;

#[derive(Debug)]
pub struct TxReturnData {
    program_id: String,
    data: String,
}

impl From<ReturnData> for TxReturnData {
    fn from(data: ReturnData) -> Self {
        TxReturnData {
            program_id: bytes_to_base58(&data.program_id),
            data: bytes_to_base58(&data.data),
        }
    }
}

#[derive(Debug)]
pub struct VixenTransaction {
    pub signature: String, // Signature is a base58 encoded string
    pub is_versioned: bool,
    pub recent_blockhash: String,
    pub log_messages: Option<Vec<String>>,
    // pub err: Option<String>,
    pub fee: u64,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    // pub inner_instructions: Option<Vec<UiInnerInstructions>>,
    pub pre_token_balances: Option<Vec<TokenBalance>>,
    pub post_token_balances: Option<Vec<TokenBalance>>,
    pub rewards: Vec<Reward>,
    pub loaded_readonly_addresses: Vec<String>,
    pub loaded_writeable_addresses: Vec<String>,
    pub return_data: Option<TxReturnData>,
    pub compute_units_consumed: Option<u64>,
    pub tx_account_pubkeys: Vec<String>,
}

impl TryFrom<TransactionUpdate> for VixenTransaction {
    type Error = String;

    fn try_from(update: TransactionUpdate) -> Result<Self, Self::Error> {
        let outer_tx = update.transaction.ok_or("Transaction not found")?;
        let inner_tx = outer_tx.transaction.ok_or("Inner transaction not found")?;
        let tx_message_data = inner_tx.message.ok_or("Transaction message not found")?;
        let tx_meta = outer_tx.meta.ok_or("Transaction meta not found")?;
        let tx_account_pubkeys = byte_vec_to_pubkey_vec(tx_message_data.account_keys)?;
        let tx_return_data = tx_meta.return_data.map_or(None, |data| Some(data.into()));
        // let inner_instructions: Vec<UiInnerInstructions> =
        //     Vec::with_capacity(tx_meta.inner_instructions.len());

        let vixen_tx = VixenTransaction {
            signature: bytes_to_base58(&outer_tx.signature),
            fee: tx_meta.fee,
            pre_balances: tx_meta.pre_balances,
            post_balances: tx_meta.post_balances,
            log_messages: check_and_return_vec(tx_meta.log_messages),
            pre_token_balances: check_and_return_vec(tx_meta.pre_token_balances),
            post_token_balances: check_and_return_vec(tx_meta.post_token_balances),
            rewards: tx_meta.rewards,
            loaded_writeable_addresses: byte_vec_to_pubkey_vec(tx_meta.loaded_writable_addresses)?,
            loaded_readonly_addresses: byte_vec_to_pubkey_vec(tx_meta.loaded_readonly_addresses)?,
            return_data: tx_return_data,
            compute_units_consumed: tx_meta.compute_units_consumed,
            is_versioned: tx_message_data.versioned,
            recent_blockhash: bytes_to_base58(&tx_message_data.recent_blockhash),
            tx_account_pubkeys,
        };

        Ok(vixen_tx)
    }
}

pub fn check_and_return_vec<V>(vec: Vec<V>) -> Option<Vec<V>> {
    if vec.is_empty() { None } else { Some(vec) }
}

pub fn byte_vec_to_pubkey_vec(bytes_vec: Vec<Vec<u8>>) -> Result<Vec<String>, String> {
    bytes_vec
        .iter()
        .map(|bytes| bytes_to_pubkey(bytes))
        .collect()
}

pub fn bytes_to_pubkey(bytes: &[u8]) -> Result<String, String> {
    Pubkey::try_from(bytes)
        .map(|p| p.to_string())
        .map_err(|e| e.to_string())
}

pub fn bytes_to_base58(bytes: &[u8]) -> String { bs58::encode(bytes).into_string() }
