use yellowstone_vixen_bpf_loader_parser::{account, AccountParser, BpfLoaderState};
use yellowstone_vixen_core::{Parser, Pubkey};
use yellowstone_vixen_mock::account_fixture;

#[tokio::test]
async fn parse_program_data_account() {
    let parser = AccountParser;

    let account = account_fixture!("C258LjtN8Q928yz2nJe5kwQycckGfWBSrhZBP2mTpLQt", &parser);

    let BpfLoaderState {
        state: Some(account::State::ProgramData(pd)),
    } = account
    else {
        panic!("Invalid Account");
    };

    assert!(pd.slot > 0);
    assert!(pd.upgrade_authority.is_some());
}

#[tokio::test]
async fn parse_program_account() {
    let parser = AccountParser;

    let account = account_fixture!("PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu", &parser);

    let BpfLoaderState {
        state: Some(account::State::Program(prog)),
    } = account
    else {
        panic!("Invalid Account");
    };

    assert_ne!(prog.programdata_address, Pubkey::default());
}
