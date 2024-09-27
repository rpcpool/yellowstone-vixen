use solana_zk_token_sdk::encryption::elgamal::ELGAMAL_KEYPAIR_LEN;
use yellowstone_vixen_core::bs58;

pub fn check_min_accounts_req(
    accounts_len: usize,
    expected_no_of_accounts: usize,
) -> Result<(), String> {
    if accounts_len < expected_no_of_accounts {
        return Err(format!(
            "Expected {expected_no_of_accounts} accounts, found {accounts_len}"
        ));
    }
    Ok(())
}

yellowstone_vixen_core::pubkey_convert_helpers!(spl_pod::solana_program::pubkey::Pubkey);
#[cfg(feature = "proto")]
yellowstone_vixen_core::proto_helper_traits!();

#[derive(Debug, Clone, Copy)]
pub struct ElGamalPubkeyBytes(pub [u8; ELGAMAL_KEYPAIR_LEN / 2]);

impl From<ElGamalPubkeyBytes> for String {
    fn from(bytes: ElGamalPubkeyBytes) -> Self { bs58::encode(bytes.0).into_string() }
}
