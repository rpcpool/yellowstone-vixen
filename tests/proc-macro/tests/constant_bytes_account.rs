use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/constant_bytes_account.json");

#[test]
fn parses_account_after_byte_constant_discriminator() {
    let mut data = vec![1];
    data.extend_from_slice(&42_u64.to_le_bytes());

    let parsed = constant_bytes_account::ConstantBytesAccountAccount::try_unpack(&data)
        .expect("byte-valued constant discriminator should match");

    let constant_bytes_account::account::Account::Vault(vault) = parsed.account;
    assert_eq!(vault.amount, 42);
}

#[test]
fn rejects_an_unknown_byte_constant_discriminator() {
    let mut data = vec![2];
    data.extend_from_slice(&42_u64.to_le_bytes());

    assert!(constant_bytes_account::ConstantBytesAccountAccount::try_unpack(&data).is_err());
}

#[test]
fn reports_a_truncated_matching_account() {
    assert!(constant_bytes_account::ConstantBytesAccountAccount::try_unpack(&[1, 42]).is_err());
}
