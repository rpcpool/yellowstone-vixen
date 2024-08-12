use std::str::FromStr;

use serde_json::Value;
use spl_pod::solana_program::{
    message::v0::LoadedAddresses,
    pubkey::{self, Pubkey},
};

// use super::structure::{IxsInfo, ReadableInstructions, TransactionIxData};
// use crate::tx_parser::token_program::parse_token_program_ix;

#[derive(Debug)]
pub struct TxAccountKeys {
    pub static_keys: Vec<Pubkey>,
    pub dynamic_keys: Option<LoadedAddresses>,
}

pub fn get_tx_account_keys(
    tx_account_pubkeys: Vec<Pubkey>,
    loaded_readonly_addresses: Vec<Pubkey>,
    loaded_writeable_addresses: Vec<Pubkey>,
) -> Result<TxAccountKeys, String> {
    let dynamic_keys: Option<LoadedAddresses> =
        if loaded_readonly_addresses.is_empty() || loaded_writeable_addresses.is_empty() {
            None
        } else {
            Some(LoadedAddresses {
                readonly: loaded_readonly_addresses,
                writable: loaded_writeable_addresses,
            })
        };

    Ok(TxAccountKeys {
        static_keys: tx_account_pubkeys.clone(),
        dynamic_keys,
    })
}

pub fn get_account_from_index(index: usize, tx: &Vec<String>) -> Result<String, String> {
    tx.get(index)
        .map_or(Err("Account not found".to_string()), |account| {
            Ok(account.clone())
        })
}

pub trait CheckVec {
    fn check_and_return_vec(self) -> Option<Self>
    where Self: Sized;
}

impl<T> CheckVec for Vec<T> {
    fn check_and_return_vec(self) -> Option<Self> {
        if self.is_empty() { None } else { Some(self) }
    }
}

pub trait ToPubkeyVecString {
    fn to_pubkey_vec(&self) -> Result<Vec<Pubkey>, String>;
}

impl ToPubkeyVecString for Vec<Vec<u8>> {
    fn to_pubkey_vec(&self) -> Result<Vec<Pubkey>, String> {
        self.iter()
            .map(|key| key.to_pubkey())
            .collect::<Result<Vec<Pubkey>, String>>()
    }
}
pub trait StringToPubkey {
    fn to_pubkey(&self) -> Result<Pubkey, String>;
}

impl StringToPubkey for String {
    fn to_pubkey(&self) -> Result<Pubkey, String> {
        Pubkey::from_str(self).map_err(|e| e.to_string())
    }
}

pub trait BytesToPubkey {
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

pub trait ToBase58 {
    fn to_base58(&self) -> String;
}

impl ToBase58 for Vec<u8> {
    fn to_base58(&self) -> String { bs58::encode(self).into_string() }
}

pub trait ToSnakeCase {
    fn to_snake_case(self) -> String;
}

impl ToSnakeCase for String {
    fn to_snake_case(self) -> String {
        let mut result = String::new();
        for (i, c) in self.chars().enumerate() {
            if c.is_uppercase() {
                if i != 0 {
                    result.push('_');
                }
                result.push(c.to_lowercase().next().unwrap());
            } else {
                result.push(c);
            }
        }
        result
    }
}

pub fn get_all_ixs_in_tx(logs: &Vec<String>) -> Option<Vec<String>> {
    let mut ix_logs: Vec<&String> = Vec::new();
    for log in logs.iter() {
        if log.contains("Instruction:") {
            ix_logs.push(log);
        }
    }
    let ix_names = ix_logs
        .iter()
        .map(|log| {
            let ix_name = log.split("Instruction: ").last()?;
            Some(ix_name.to_string().to_snake_case())
        })
        .collect::<Option<Vec<String>>>()?;
    Some(ix_names)
}

#[derive(Debug)]
pub enum IxType {
    Outer, //Outer ixs
    Cpi,   //nested ixs
}

#[derive(Debug)]
pub struct ParsedIx {
    pub calling_program: String,
    pub name: String,
    pub ix_type: IxType,
    // params:Vec<String> //TODO
}

// pub fn parse_ixs(readables_ixs: &Vec<ReadableInstructions>) -> Vec<ParsedIx> {
//     let mut parsed_ixs: Vec<ParsedIx> = Vec::new();

//     for ix in readables_ixs.iter() {
//         for inner_ix in ix.instructions.iter() {
//             let parsed = parse_ix_from_json(inner_ix.parsed.clone());
//             if parsed.is_none() {
//                 continue;
//             }

//             let ix_name = parsed.unwrap();

//             let parsed_ix = ParsedIx {
//                 calling_program: inner_ix.program_id.clone(),
//                 name: ix_name.to_snake_case(),
//                 ix_type: IxType::Cpi, //these calls are always CPI
//             };

//             parsed_ixs.push(parsed_ix);
//         }
//     }

//     parsed_ixs
// }

// pub fn parse_ix_from_json(parsed: Value) -> Option<String> {
//     parsed
//         .as_object()
//         .and_then(|data| data.get("type"))
//         .and_then(|ix_name| ix_name.as_str())
//         .map(|name| name.to_string())
// }

// pub fn get_ix_info(
//     logs: &Option<Vec<String>>,
//     readables_ixs: &Vec<ReadableInstructions>,
// ) -> Option<IxsInfo> {
//     let all_ixs = get_all_ixs_in_tx(logs.as_ref()?)?;
//     let parsed_ixs = parse_ixs(readables_ixs);
//     Some(IxsInfo {
//         all_ixs,
//         filtered_parsed_ixs: parsed_ixs,
//     })
// }

// pub fn get_ix_data<'i>(program_id: &String, json_data: &Value) -> Option<TransactionIxData> {
//     let spl_program_id = Pubkey::from_str(program_id).ok()?;
//     if program_id.eq(&spl_program_id.to_string()) {
//         return parse_token_program_ix(json_data)
//             .map_or(None, |ix| Some(TransactionIxData::TokenProgramIx(ix)));
//     }

//     return None;
// }
