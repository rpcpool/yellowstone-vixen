use std::str::FromStr;

use solana_zk_token_sdk::encryption::elgamal::ELGAMAL_KEYPAIR_LEN;
use spl_pod::solana_program::{program_option::COption, pubkey::Pubkey as SolanaPubkey};
use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};

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

pub fn get_multisig_signers(ix: &InstructionUpdate, from_idx: usize) -> Option<Vec<Pubkey>> {
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

#[derive(Debug)]
pub struct ReadableInstruction<A, D> {
    pub accounts: A,
    pub data: Option<D>,
}

impl<A, D> ReadableInstruction<A, D> {
    pub fn new(accounts: A, data: Option<D>) -> Self { Self { accounts, data } }

    pub fn from_accounts(accounts: A) -> Self {
        Self {
            accounts,
            data: None,
        }
    }
}

pub trait InstructionParser<C> {
    fn parse_ix(_: &InstructionUpdate) -> Result<C, String>;
}

pub trait IntoProtoData<O> {
    fn into_proto_data(self) -> O;
}

#[derive(Debug, Clone, Copy)]
pub struct ElGamalPubkeyBytes(pub [u8; ELGAMAL_KEYPAIR_LEN / 2]);

impl From<ElGamalPubkeyBytes> for String {
    fn from(bytes: ElGamalPubkeyBytes) -> Self { bs58::encode(bytes.0).into_string() }
}
