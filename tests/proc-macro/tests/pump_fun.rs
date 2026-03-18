use vixen_test_utils::{check_protobuf_format, p};
use yellowstone_vixen_core::{Parser, Pubkey};
use yellowstone_vixen_mock::tx_fixture;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("idls/pump_fun.json");

#[test]
fn check_protobuf_schema() {
    check_protobuf_format(pump_fun::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(pump_fun::PROTOBUF_SCHEMA);
}

#[tokio::test]
async fn parse_sell_ix() {
    let parser = pump_fun::InstructionParser;

    let ixs = tx_fixture!(
        "4GtxzHLTW8ZqhhHLqXPK2DSqtqaMXBvknpkXqgtfGjxaPvBC6AdHnzvX6X8EgMo9V4Ua4osroEpk7Q3f3oernYTQ",
        &parser
    );

    let (sell_accounts, sell_args) = ixs
        .iter()
        .find_map(|ix| match &ix.as_ref()?.instruction {
            pump_fun::instruction::Instruction::Sell { accounts, args } => Some((accounts, args)),
            _ => None,
        })
        .expect("no Sell found");

    let expected = pump_fun::instruction::Sell {
        accounts: pump_fun::instruction::SellAccounts {
            global: p("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            fee_recipient: p("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
            mint: p("FXLjNRSQXuFM8fhKLEg2QeuUxs4Eu2hhc28tZhzrpump"),
            bonding_curve: p("9srUo8cgvHKkAZYNdjwgiYm8vK8PNRhBbzjh8RuWZtp3"),
            associated_bonding_curve: p("Ccpc35bJPjubzYbfkUbg7kK7Arkp4Q8so7ne6rLpAGXv"),
            associated_user: p("3K6Mgvd9jEvXRM7DTeNGjBZc2xi4QCqyg1mqVwaCWP76"),
            user: p("GEk94udrX63hAvNzNE1eC8zNvMDZ2cpV9xTgKuVP7kAw"),
            system_program: p("11111111111111111111111111111111"),
            creator_vault: p("7YbeWL1XuV6FCgYZbFSk7pcqk4UdM2G9YWuZKYHWmpfJ"),
            token_program: p("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            event_authority: p("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: Pubkey::new(pump_fun::PROGRAM_ID),
            fee_config: p("8Wf5TiAheLUqBrKXeYg2JtAFFMWtKdG2BSFgqUcPVwTt"),
            fee_program: p("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ"),
            remaining_accounts: vec![],
        },
        args: pump_fun::instruction::SellArgs {
            amount: 3_878_351_170_692,
            min_sol_output: 522_403_143,
        },
    };

    assert_eq!(sell_accounts, &expected.accounts);
    assert_eq!(sell_args, &expected.args);
}

#[tokio::test]
async fn parse_buy_ix() {
    let parser = pump_fun::InstructionParser;

    let ixs = tx_fixture!(
        "3tkxRjNDfth6NxXpYbbLKmPkPYyAD4jjXfNnDCYtCKSPN2zSpJXT29reowKtFKz1puY1fHmBFVAskkK2A7o8cZgJ",
        &parser
    );

    let (buy_accounts, buy_args) = ixs
        .iter()
        .find_map(|ix| match &ix.as_ref()?.instruction {
            pump_fun::instruction::Instruction::Buy { accounts, args } => Some((accounts, args)),
            _ => None,
        })
        .expect("no Buy found");

    let expected = pump_fun::instruction::Buy {
        accounts: pump_fun::instruction::BuyAccounts {
            global: p("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            fee_recipient: p("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
            mint: p("BaWrqmzh9nqUkZRKB9p9WNaM8xKRrmcW9ztgLJrWpump"),
            bonding_curve: p("8DZu2LNtTKph9iShWpWdFjHPFAqGkrafpSUyaFZrd6ip"),
            associated_bonding_curve: p("FW755HTHAZweRrzoyZWe6iQdHwtvh82Z6HKcymWCUwCQ"),
            associated_user: p("GYNLNMVdaw8CtrgsiJhrwG5vmTXhei8Kc768FqBEEEsq"),
            user: p("DG6aWRd9ft47v9MZxeoHrxaJfVfLcXRTwC4RZ23DkokX"),
            system_program: p("11111111111111111111111111111111"),
            token_program: p("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            creator_vault: p("BM11VKvie7n5CA6crAasQDeorhpvLJ3Ghu8BHkTpwRSq"),
            event_authority: p("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: Pubkey::new(pump_fun::PROGRAM_ID),
            global_volume_accumulator: p("Hq2wp8uJ9jCPsYgNHex8RtqdvMPfVGoYwjvF1ATiwn2Y"),
            user_volume_accumulator: p("Guka6uenDqtwCT8xmGpirKWeD5KU3r5L21qCSBVPHHPY"),
            fee_config: p("8Wf5TiAheLUqBrKXeYg2JtAFFMWtKdG2BSFgqUcPVwTt"),
            fee_program: p("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ"),
            remaining_accounts: vec![],
        },
        args: pump_fun::instruction::BuyArgs {
            amount: 693_868_985_905,
            max_sol_cost: 55_000_000,
            track_volume: false,
        },
    };

    assert_eq!(buy_accounts, &expected.accounts);
    assert_eq!(buy_args, &expected.args);
}
