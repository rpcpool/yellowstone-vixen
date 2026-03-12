use yellowstone_vixen_core::{pubkey_convert_helpers, KeyBytes, PublicKey};

pubkey_convert_helpers!(solana_pubkey::Pubkey);

#[test]
fn convert_vixen_keybytes() {

    let solana_pubkey = solana_pubkey::Pubkey::new_unique();

    let vixen_keybytes: KeyBytes<32> = into_vixen_pubkey(solana_pubkey);

    let back = from_vixen_pubkey(vixen_keybytes);

    assert_eq!(solana_pubkey, back);

}

#[test]
fn convert_vixen_publickey() {

    let solana_pubkey = solana_pubkey::Pubkey::new_unique();

    let public_key: PublicKey = into_vixen_publickey(solana_pubkey);

    let back = from_vixen_publickey(public_key).expect("conversion should succeed");

    assert_eq!(solana_pubkey, back);

}


