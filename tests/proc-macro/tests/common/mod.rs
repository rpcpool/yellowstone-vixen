mod check_protobuf_format;

pub use check_protobuf_format::check_protobuf_format;

pub fn pubkey(s: &str) -> yellowstone_vixen_core::PublicKey {
    yellowstone_vixen_core::PublicKey::new(
        yellowstone_vixen_core::bs58::decode(s).into_vec().unwrap(),
    )
}

pub fn pubkey_bytes(bytes: &[u8]) -> yellowstone_vixen_core::PublicKey {
    yellowstone_vixen_core::PublicKey::new(bytes.to_vec())
}
