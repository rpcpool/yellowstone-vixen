use std::{str::FromStr, vec};

use serde_json::Value;
use spl_pod::solana_program::{
    instruction::CompiledInstruction,
    message::{v0::LoadedAddresses, AccountKeys},
    pubkey,
    pubkey::Pubkey,
};
use yellowstone_grpc_proto::prelude::{
    InnerInstruction, InnerInstructions, ReturnData, Reward, TokenBalance,
};
use yellowstone_vixen_core::{ParseResult, TransactionUpdate};

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
pub struct ReadableInstructionData {
    pub name: String,
    pub params: Vec<String>,
}

#[derive(Debug)]
pub struct ReadableInstructions {
    pub index: u32,
    pub instructions: Vec<ReadableInstruction>,
}

impl ReadableInstructions {
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
    pub stack_height: Option<u32>, // CPI call depth
    pub readable_data: ReadableInstructionData,
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

        Ok(ReadableInstruction {
            index: index as u32,
            program: parsed_ix.program,
            program_id,
            parsed: parsed_ix.parsed,
            stack_height: parsed_ix.stack_height,
            readable_data: ReadableInstructionData {
                name: "".to_string(),
                params: vec![],
            },
        })
    }
}

#[derive(Debug)]
pub struct AddressLookupTableKeys {
    static_keys: Vec<Pubkey>,
    dynamic_keys: Option<LoadedAddresses>,
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
            log_messages: tx_meta.log_messages.check_and_return_vec(),
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
        println!("Vixen Tx{:#?}", vixen_tx);
        Ok(vixen_tx)
    }
}

pub fn get_address_lookup_table_keys<'a>(
    tx_account_pubkeys: &Vec<String>,
    loaded_readonly_addresses: &Vec<String>,
    loaded_writeable_addresses: &Vec<String>,
) -> Result<AddressLookupTableKeys, String> {
    let static_keys = tx_account_pubkeys
        .iter()
        .map(|key| key.to_pubkey())
        .collect::<Result<Vec<Pubkey>, String>>()?;

    let dynamic_keys: Option<LoadedAddresses> =
        if loaded_readonly_addresses.is_empty() || loaded_writeable_addresses.is_empty() {
            None
        } else {
            let readonly_keys = loaded_readonly_addresses
                .iter()
                .map(|key| key.to_pubkey())
                .collect::<Result<Vec<Pubkey>, String>>()?;

            let writeable_keys = loaded_writeable_addresses
                .iter()
                .map(|key| key.to_pubkey())
                .collect::<Result<Vec<Pubkey>, String>>()?;

            Some(LoadedAddresses {
                readonly: readonly_keys,
                writable: writeable_keys,
            })
        };

    Ok(AddressLookupTableKeys {
        static_keys,
        dynamic_keys,
    })
}

pub fn get_account_from_index(index: usize, tx: &Vec<String>) -> Result<String, String> {
    tx.get(index)
        .map_or(Err("Account not found".to_string()), |account| {
            Ok(account.clone())
        })
}

trait CheckVec {
    fn check_and_return_vec(self) -> Option<Self>
    where Self: Sized;
}

impl<T> CheckVec for Vec<T> {
    fn check_and_return_vec(self) -> Option<Self> {
        if self.is_empty() { None } else { Some(self) }
    }
}

trait ToPubkeyVecString {
    fn to_pubkey_vec(&self) -> Result<Vec<String>, String>;
}

impl ToPubkeyVecString for Vec<Vec<u8>> {
    fn to_pubkey_vec(&self) -> Result<Vec<String>, String> {
        self.iter()
            .map(|key| key.to_pubkey_string())
            .collect::<Result<Vec<String>, String>>()
    }
}
trait StringToPubkey {
    fn to_pubkey(&self) -> Result<Pubkey, String>;
}

impl StringToPubkey for String {
    fn to_pubkey(&self) -> Result<Pubkey, String> {
        Pubkey::from_str(self).map_err(|e| e.to_string())
    }
}

trait BytesToPubkey {
    fn to_pubkey(&self) -> Result<Pubkey, String>;
    fn to_pubkey_string(&self) -> Result<String, String>;
}

impl BytesToPubkey for Vec<u8> {
    fn to_pubkey(&self) -> Result<Pubkey, String> {
        pubkey::Pubkey::try_from(self.as_slice()).map_err(|e| e.to_string())
    }

    fn to_pubkey_string(&self) -> Result<String, String> {
        self.to_pubkey().map(|key| key.to_string())
    }
}

trait ToBase58 {
    fn to_base58(&self) -> String;
}

impl ToBase58 for Vec<u8> {
    fn to_base58(&self) -> String { bs58::encode(self).into_string() }
}
