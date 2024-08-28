use std::str::FromStr;

use spl_pod::solana_program::{program_option::COption, pubkey::Pubkey as SolanaPubkey};
use yellowstone_vixen_core::{Instruction, Pubkey};

pub fn check_min_accounts_req(
    accounts_len: usize,
    expected_no_of_accounts: usize,
) -> Result<(), String> {
    if accounts_len < expected_no_of_accounts {
        return Err(format!(
            "Expected {} accounts, found {}",
            expected_no_of_accounts, accounts_len
        ));
    }
    Ok(())
}

pub fn get_multisig_signers(ix: &Instruction, from_idx: usize) -> Option<Vec<Pubkey>> {
    if from_idx >= ix.accounts.len() {
        return None;
    }
    ix.accounts.get(from_idx..).map(|p| p.to_vec())
}

pub fn to_supported_coption_pubkey(sol_coption: COption<SolanaPubkey>) -> COption<Pubkey> {
    match sol_coption {
        COption::Some(pubkey) => COption::Some(to_supported_pubkey(pubkey)),
        COption::None => COption::None,
    }
}

pub fn to_supported_pubkey(sol_pubkey: SolanaPubkey) -> Pubkey {
    Pubkey::from_str(&sol_pubkey.to_string()).unwrap()
}

pub fn check_pubkeys_match<T: ToString, S: ToString>(pubkey1: &T, pubkey2: &S) -> bool {
    pubkey1.to_string().eq(&pubkey2.to_string())
}
