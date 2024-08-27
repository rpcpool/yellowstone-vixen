use std::{iter::zip, ops::Index, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{Instruction, Pubkey};

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct LoadedAddresses {
    pub writable: Vec<Pubkey>,
    pub readonly: Vec<Pubkey>,
}
#[derive(Debug)]
pub struct TxAccountKeys {
    pub static_keys: Vec<Pubkey>,
    pub dynamic_keys: Option<LoadedAddresses>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ixs {
    pub instructions: Vec<IxWithInnerIxs>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IxWithInnerIxs {
    pub outer_ix: Instruction,
    pub inner_ixs: Vec<Instruction>,
}

#[derive(Clone, Default, Debug, Eq)]
pub struct AccountKeys<'a> {
    static_keys: &'a [Pubkey],
    dynamic_keys: Option<&'a LoadedAddresses>,
}

impl PartialEq for AccountKeys<'_> {
    fn eq(&self, other: &Self) -> bool { zip(self.iter(), other.iter()).all(|(a, b)| a == b) }
}

impl Index<usize> for AccountKeys<'_> {
    type Output = Pubkey;

    fn index(&self, index: usize) -> &Self::Output { self.get(index).expect("index is invalid") }
}

impl<'a> AccountKeys<'a> {
    pub fn new(static_keys: &'a [Pubkey], dynamic_keys: Option<&'a LoadedAddresses>) -> Self {
        Self {
            static_keys,
            dynamic_keys,
        }
    }

    fn key_segment_iter(&self) -> impl Iterator<Item = &'a [Pubkey]> {
        if let Some(dynamic_keys) = self.dynamic_keys {
            [
                self.static_keys,
                &dynamic_keys.writable,
                &dynamic_keys.readonly,
            ]
            .into_iter()
        } else {
            // empty segments added for branch type compatibility
            [self.static_keys, &[], &[]].into_iter()
        }
    }

    pub fn get(&self, mut index: usize) -> Option<&'a Pubkey> {
        for key_segment in self.key_segment_iter() {
            if index < key_segment.len() {
                return Some(&key_segment[index]);
            }
            index = index.saturating_sub(key_segment.len());
        }

        None
    }

    pub fn len(&self) -> usize {
        let mut len = 0usize;
        for key_segment in self.key_segment_iter() {
            len = len.saturating_add(key_segment.len());
        }
        len
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }

    pub fn iter(&self) -> impl Iterator<Item = &'a Pubkey> { self.key_segment_iter().flatten() }
}

pub fn get_account_from_index(index: usize, accounts: &TxAccountKeys) -> Result<Pubkey, String> {
    let tx_account_keys_loaded = AccountKeys::new(
        accounts.static_keys.as_slice(),
        accounts.dynamic_keys.as_ref(),
    );

    if tx_account_keys_loaded.is_empty() {
        return Err("No accounts found".to_string());
    }

    tx_account_keys_loaded.get(index).map_or(
        Err(format!(
            "Account index {} out of bounds for {} accounts",
            index,
            tx_account_keys_loaded.len(),
        )),
        |account| Ok(*account),
    )
}

pub fn get_account_pubkey_from_index(
    index: usize,
    accounts: &Vec<String>,
) -> Result<Pubkey, String> {
    if accounts.is_empty() {
        return Err("No accounts found".to_string());
    }

    accounts.get(index).map_or(
        Err(format!(
            "Account index {} out of bounds for {} accounts",
            index,
            accounts.len(),
        )),
        |account| Ok(Pubkey::from_str(account)?),
    )
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

pub trait BytesToPubkey {
    fn to_pubkey(&self) -> Result<Pubkey, String>;
}

impl BytesToPubkey for Vec<u8> {
    fn to_pubkey(&self) -> Result<Pubkey, String> {
        Pubkey::try_from(self.as_slice()).map_err(|e| e.to_string())
    }
}
