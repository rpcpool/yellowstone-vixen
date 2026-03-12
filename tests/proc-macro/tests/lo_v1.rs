mod common;

use common::pubkey;
use yellowstone_vixen_core::Parser;
use yellowstone_vixen_mock::tx_fixture;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("idls/lo_v1.json");

#[test]
fn check_protobuf_schema() {
    common::check_protobuf_format(limit_order::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(limit_order::PROTOBUF_SCHEMA);
}

#[tokio::test]
async fn parse_pre_flash_fill_order_ix() {
    let parser = limit_order::InstructionParser.with_raw_logs();

    let ixs = tx_fixture!(
        "3jaLYNHZBxPxAcXUYdsMRhrDL19YpS7jDgkq5p4GDcsfUK82sLAuDv9gnRhw5KkAh8yWgomZnHn8Lbz3uvbqKpAC",
        &parser
    );

    let pre_flash_ix = ixs
        .iter()
        .find_map(|ix| {
            let ix = ix.as_ref()?;
            matches!(
                &ix.instruction,
                limit_order::instruction::Instruction::PreFlashFillOrder { .. }
            )
            .then_some(ix)
        })
        .expect("no PreFlashFillOrder found");

    let expected = limit_order::Instructions {
        instruction: limit_order::instruction::Instruction::PreFlashFillOrder {
            accounts: limit_order::instruction::PreFlashFillOrderAccounts {
                order: pubkey("HzKfs1qTtpvV9u9yh2imBh7aKtFBeuffAFbn3L1pA6Qw"),
                reserve: pubkey("DzNK4xxbthzVRJVnPCHhBYAFki24n9B2McPmPUtiTLkc"),
                taker: pubkey("71WDyyCsZwyEYDV91Qrb212rdg6woCHYQhFnmZUBxiJ6"),
                taker_output_account: pubkey("71WDyyCsZwyEYDV91Qrb212rdg6woCHYQhFnmZUBxiJ6"),
                input_mint: pubkey("So11111111111111111111111111111111111111112"),
                input_mint_token_program: pubkey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                instruction: pubkey("Sysvar1nstructions1111111111111111111111111"),
                system_program: pubkey("11111111111111111111111111111111"),
                remaining_accounts: vec![],
            },
            args: limit_order::instruction::PreFlashFillOrderArgs {
                making_amount: 151_188_480,
            },
        },
        raw_logs: vec![
            "Program jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu invoke [1]".into(),
            "Program log: Instruction: PreFlashFillOrder".into(),
            "Program 11111111111111111111111111111111 invoke [2]".into(),
            "Program 11111111111111111111111111111111 success".into(),
            "Program jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu consumed 42871 of 650383 compute \
             units"
                .into(),
            "Program jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu success".into(),
        ],
        anchor_log_events: vec![],
    };

    assert_eq!(pre_flash_ix, &expected);
}

#[tokio::test]
async fn parse_flash_fill_order_ix() {
    let parser = limit_order::InstructionParser.with_raw_logs();

    let ixs = tx_fixture!(
        "3jaLYNHZBxPxAcXUYdsMRhrDL19YpS7jDgkq5p4GDcsfUK82sLAuDv9gnRhw5KkAh8yWgomZnHn8Lbz3uvbqKpAC",
        &parser
    );

    let flash_ix = ixs
        .iter()
        .find_map(|ix| {
            let ix = ix.as_ref()?;
            matches!(
                &ix.instruction,
                limit_order::instruction::Instruction::FlashFillOrder { .. }
            )
            .then_some(ix)
        })
        .expect("no FlashFillOrder found");

    let expected = limit_order::Instructions {
        instruction: limit_order::instruction::Instruction::FlashFillOrder {
            accounts: limit_order::instruction::FlashFillOrderAccounts {
                order: pubkey("HzKfs1qTtpvV9u9yh2imBh7aKtFBeuffAFbn3L1pA6Qw"),
                reserve: pubkey("DzNK4xxbthzVRJVnPCHhBYAFki24n9B2McPmPUtiTLkc"),
                maker: pubkey("7c49JgubU1Q8mdBUt1kgdSkP2ezgCSgenknV4KMyhSJQ"),
                taker: pubkey("71WDyyCsZwyEYDV91Qrb212rdg6woCHYQhFnmZUBxiJ6"),
                maker_output_account: pubkey("7x9pfoBP1syJPzN4YqBNWX1VbQQkaVPdRuxBrWab5GuC"),
                taker_input_account: pubkey("CGhH4cVn7zdkwGEWojhFcqfyt7zPLi3AVDuhWeHA3WiQ"),
                fee_authority: pubkey("H3vkQqNVWySTD4c1Y91wtoT5iwxKSVtVLfC2rD8SgwTN"),
                program_fee_account: pubkey("GXYDRDTopqAuJVi9F24t3uVDjQ41wQvYa6qdDcNQSCvm"),
                referral: pubkey("jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu"),
                input_mint: pubkey("So11111111111111111111111111111111111111112"),
                input_mint_token_program: pubkey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                output_mint: pubkey("WENWENvqqNya429ubCdR81ZmD69brwQaaBYY6p3LCpk"),
                output_mint_token_program: pubkey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                system_program: pubkey("11111111111111111111111111111111"),
                remaining_accounts: vec![],
            },
            args: limit_order::instruction::FlashFillOrderArgs {
                max_taking_amount: 91_407_969_193,
            },
        },
        raw_logs: vec![
            "Program jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu invoke [1]".into(),
            "Program log: Instruction: FlashFillOrder".into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]".into(),
            "Program log: Instruction: TransferChecked".into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6147 of 56613 compute \
             units"
                .into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]".into(),
            "Program log: Instruction: TransferChecked".into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6147 of 46958 compute \
             units"
                .into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".into(),
            "Program 11111111111111111111111111111111 invoke [2]".into(),
            "Program 11111111111111111111111111111111 success".into(),
            "Program data: \
             vdt/007mYe78ajUMZUip7eKpV9ZDGMglVmD2ntOibuEws7ZobgiFOFlHKkCyZDWBtb1Nj5NatQSurNWW\
             y0a2b1mw9O9xbT67AAAAAAAAAAAAAAAAAAAAAAD0AgkAAAAAqetWSBUAAAA="
                .into(),
            "Program jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu consumed 58542 of 94384 compute \
             units"
                .into(),
            "Program jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu success".into(),
        ],
        anchor_log_events: vec![limit_order::AnchorLogEvent::TradeEvent(
            limit_order::anchor_log_event::TradeEvent {
                order_key: pubkey("HzKfs1qTtpvV9u9yh2imBh7aKtFBeuffAFbn3L1pA6Qw"),
                taker: pubkey("71WDyyCsZwyEYDV91Qrb212rdg6woCHYQhFnmZUBxiJ6"),
                remaining_in_amount: 0,
                remaining_out_amount: 0,
                in_amount: 151_188_480,
                out_amount: 91_407_969_193,
            },
        )],
    };

    assert_eq!(flash_ix, &expected);
}
