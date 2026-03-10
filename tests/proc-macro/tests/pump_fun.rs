mod common;

use common::{pubkey, pubkey_bytes};
use yellowstone_vixen_core::Parser;
use yellowstone_vixen_mock::tx_fixture;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("idls/pump_fun.json");

#[test]
fn check_protobuf_schema() {
    common::check_protobuf_format(pump_fun::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(pump_fun::PROTOBUF_SCHEMA);
}

#[tokio::test]
async fn parse_sell_ix() {
    let parser = pump_fun::InstructionParser;

    let ixs = tx_fixture!(
        "4GtxzHLTW8ZqhhHLqXPK2DSqtqaMXBvknpkXqgtfGjxaPvBC6AdHnzvX6X8EgMo9V4Ua4osroEpk7Q3f3oernYTQ",
        &parser
    );

    let sell_ix = ixs
        .iter()
        .find_map(|ix| {
            let ix = ix.as_ref()?;
            matches!(
                &ix.instruction,
                pump_fun::instruction::Instruction::Sell { .. }
            )
            .then_some(ix)
        })
        .expect("no Sell found");

    let expected = pump_fun::Instructions {
        instruction: pump_fun::instruction::Instruction::Sell {
            accounts: pump_fun::instruction::SellAccounts {
                global: pubkey("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                fee_recipient: pubkey("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
                mint: pubkey("FXLjNRSQXuFM8fhKLEg2QeuUxs4Eu2hhc28tZhzrpump"),
                bonding_curve: pubkey("9srUo8cgvHKkAZYNdjwgiYm8vK8PNRhBbzjh8RuWZtp3"),
                associated_bonding_curve: pubkey("Ccpc35bJPjubzYbfkUbg7kK7Arkp4Q8so7ne6rLpAGXv"),
                associated_user: pubkey("3K6Mgvd9jEvXRM7DTeNGjBZc2xi4QCqyg1mqVwaCWP76"),
                user: pubkey("GEk94udrX63hAvNzNE1eC8zNvMDZ2cpV9xTgKuVP7kAw"),
                system_program: pubkey("11111111111111111111111111111111"),
                creator_vault: pubkey("7YbeWL1XuV6FCgYZbFSk7pcqk4UdM2G9YWuZKYHWmpfJ"),
                token_program: pubkey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                event_authority: pubkey("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                program: pubkey_bytes(&pump_fun::PROGRAM_ID),
                fee_config: pubkey("8Wf5TiAheLUqBrKXeYg2JtAFFMWtKdG2BSFgqUcPVwTt"),
                fee_program: pubkey("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ"),
                remaining_accounts: vec![],
            },
            args: pump_fun::instruction::SellArgs {
                amount: 3_878_351_170_692,
                min_sol_output: 522_403_143,
            },
        },
        raw_logs: vec![
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [1]".into(),
            "Program log: Instruction: Sell".into(),
            "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ invoke [2]".into(),
            "Program log: Instruction: GetFees".into(),
            "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ consumed 3136 of 57992 \
             compute units"
                .into(),
            "Program return: pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ \
             AAAAAAAAAABfAAAAAAAAAB4AAAAAAAAA"
                .into(),
            "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ success".into(),
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb invoke [2]".into(),
            "Program log: Instruction: TransferChecked".into(),
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb consumed 2475 of 51092 \
             compute units"
                .into(),
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb success".into(),
            "Program data: vdt/007mYe7XyVnAmrsmNQEqjfzZIfKiwqiQCYeIdMrC6UxxN4hCf9DrkCUAAAAA\
             hGy+/4YDAAAA4mShxt7KFvq6T6dmjb9yEzSgbpRBKEVTmLNy5reQWhyA9IlpAAAAABJjOsQQAAAA\
             ZMje4I2WAQAStxbICQAAAGQwzJT8lwAASsL40N1cvJfjKJwZfLUGKlTz2Va5zm5RFfllZ6pcs+Zf\
             AAAAAAAAAFtcWwAAAAAALUNGTC+hWGN5OXP1S7ZMJLKc62q0tXQFvA5WBZ2I/18eAAAAAAAAAMzZ\
             HAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAHNlbGwA"
                .into(),
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [2]".into(),
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 2054 of 42010 \
             compute units"
                .into(),
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success".into(),
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 47690 of 86731 \
             compute units"
                .into(),
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success".into(),
        ],
    };

    assert_eq!(sell_ix, &expected);
}

#[tokio::test]
async fn parse_buy_ix() {
    let parser = pump_fun::InstructionParser;

    let ixs = tx_fixture!(
        "3tkxRjNDfth6NxXpYbbLKmPkPYyAD4jjXfNnDCYtCKSPN2zSpJXT29reowKtFKz1puY1fHmBFVAskkK2A7o8cZgJ",
        &parser
    );

    let buy_ix = ixs
        .iter()
        .find_map(|ix| {
            let ix = ix.as_ref()?;
            matches!(
                &ix.instruction,
                pump_fun::instruction::Instruction::Buy { .. }
            )
            .then_some(ix)
        })
        .expect("no Buy found");

    let expected = pump_fun::Instructions {
        instruction: pump_fun::instruction::Instruction::Buy {
            accounts: pump_fun::instruction::BuyAccounts {
                global: pubkey("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                fee_recipient: pubkey("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
                mint: pubkey("BaWrqmzh9nqUkZRKB9p9WNaM8xKRrmcW9ztgLJrWpump"),
                bonding_curve: pubkey("8DZu2LNtTKph9iShWpWdFjHPFAqGkrafpSUyaFZrd6ip"),
                associated_bonding_curve: pubkey("FW755HTHAZweRrzoyZWe6iQdHwtvh82Z6HKcymWCUwCQ"),
                associated_user: pubkey("GYNLNMVdaw8CtrgsiJhrwG5vmTXhei8Kc768FqBEEEsq"),
                user: pubkey("DG6aWRd9ft47v9MZxeoHrxaJfVfLcXRTwC4RZ23DkokX"),
                system_program: pubkey("11111111111111111111111111111111"),
                token_program: pubkey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                creator_vault: pubkey("BM11VKvie7n5CA6crAasQDeorhpvLJ3Ghu8BHkTpwRSq"),
                event_authority: pubkey("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                program: pubkey_bytes(&pump_fun::PROGRAM_ID),
                global_volume_accumulator: pubkey("Hq2wp8uJ9jCPsYgNHex8RtqdvMPfVGoYwjvF1ATiwn2Y"),
                user_volume_accumulator: pubkey("Guka6uenDqtwCT8xmGpirKWeD5KU3r5L21qCSBVPHHPY"),
                fee_config: pubkey("8Wf5TiAheLUqBrKXeYg2JtAFFMWtKdG2BSFgqUcPVwTt"),
                fee_program: pubkey("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ"),
                remaining_accounts: vec![],
            },
            args: pump_fun::instruction::BuyArgs {
                amount: 693_868_985_905,
                max_sol_cost: 55_000_000,
                track_volume: pump_fun::OptionBool { item_0: false },
            },
        },
        raw_logs: vec![
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [1]".into(),
            "Program log: Instruction: Buy".into(),
            "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ invoke [2]".into(),
            "Program log: Instruction: GetFees".into(),
            "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ consumed 3136 of 136942 \
             compute units"
                .into(),
            "Program return: pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ \
             AAAAAAAAAABfAAAAAAAAAB4AAAAAAAAA"
                .into(),
            "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ success".into(),
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb invoke [2]".into(),
            "Program log: Instruction: TransferChecked".into(),
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb consumed 2475 of 130003 \
             compute units"
                .into(),
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb success".into(),
            "Program 11111111111111111111111111111111 invoke [2]".into(),
            "Program 11111111111111111111111111111111 success".into(),
            "Program 11111111111111111111111111111111 invoke [2]".into(),
            "Program 11111111111111111111111111111111 success".into(),
            "Program 11111111111111111111111111111111 invoke [2]".into(),
            "Program 11111111111111111111111111111111 success".into(),
            "Program data: vdt/007mYe6dKsxKfMbYbJAUjzVU6hwpUo/NqZtQ0u8EpfI1QaN7/5i4+wIAAAAA\
             MX7QjaEAAAABtipDEeZapMxXx0Wp0Funx+qS849Mz3skrt28g1HUC0yp9IlpAAAAAMnkpzkLAAAA\
             LNhpoj9fAgDJOIQ9BAAAACxAV1auYAEASsL40N1cvJfjKJwZfLUGKlTz2Va5zm5RFfllZ6pcs+Zf\
             AAAAAAAAAF9BBwAAAAAApZaRFOGWYW3wyoOUf3uyF/i/3KfhW/3tp0juGw55TDEeAAAAAAAAAIpK\
             AgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAwAAAGJ1eQA="
                .into(),
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [2]".into(),
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 2054 of 114862 \
             compute units"
                .into(),
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success".into(),
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 68600 of 179974 \
             compute units"
                .into(),
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success".into(),
        ],
    };

    assert_eq!(buy_ix, &expected);
}
