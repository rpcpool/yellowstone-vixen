use vixen_test_utils::{check_protobuf_format, p};
use yellowstone_vixen_core::Parser;
use yellowstone_vixen_mock::tx_fixture;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/limit_order_v1.json");

// ---------------------------------------------------------------------------
// Proto schemas
// ---------------------------------------------------------------------------

#[test]
fn check_protobuf_schema() {
    check_protobuf_format(limit_order::PROTOBUF_SCHEMA);
    insta::assert_snapshot!(limit_order::PROTOBUF_SCHEMA);
}

// ---------------------------------------------------------------------------
// Parsing: instructions with log-based anchor events
// ---------------------------------------------------------------------------

///
/// Parse the flash-fill transaction using `program_event_parser()`.
///
/// The fixture contains two limit_order instructions:
/// 1. PreFlashFillOrder — no anchor events
/// 2. FlashFillOrder — with a TradeEvent emitted via "Program data:" log
///
/// Both are asserted together against the full parsed output.
///
#[tokio::test]
async fn parse_flash_fill_transaction() {
    let parser = limit_order::InstructionParser;

    let ixs = tx_fixture!(
        "3jaLYNHZBxPxAcXUYdsMRhrDL19YpS7jDgkq5p4GDcsfUK82sLAuDv9gnRhw5KkAh8yWgomZnHn8Lbz3uvbqKpAC",
        &parser
    );

    let outputs: Vec<_> = ixs.iter().filter_map(|out| out.as_ref()).collect();

    let expected = vec![
        // 1. PreFlashFillOrder — instruction only, no events
        limit_order::ProgramEventOutput {
            instruction: Some(limit_order::Instructions {
                instruction: limit_order::instruction::Instruction::PreFlashFillOrder {
                    accounts: limit_order::instruction::PreFlashFillOrderAccounts {
                        order: p("HzKfs1qTtpvV9u9yh2imBh7aKtFBeuffAFbn3L1pA6Qw"),
                        reserve: p("DzNK4xxbthzVRJVnPCHhBYAFki24n9B2McPmPUtiTLkc"),
                        taker: p("71WDyyCsZwyEYDV91Qrb212rdg6woCHYQhFnmZUBxiJ6"),
                        taker_output_account: p("71WDyyCsZwyEYDV91Qrb212rdg6woCHYQhFnmZUBxiJ6"),
                        input_mint: p("So11111111111111111111111111111111111111112"),
                        input_mint_token_program: p("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                        instruction: p("Sysvar1nstructions1111111111111111111111111"),
                        system_program: p("11111111111111111111111111111111"),
                        remaining_accounts: vec![],
                    },
                    args: limit_order::instruction::PreFlashFillOrderArgs {
                        making_amount: 151_188_480,
                    },
                },
            }),
            program_events: vec![],
        },
        // 2. FlashFillOrder — instruction + TradeEvent from "Program data:" log
        limit_order::ProgramEventOutput {
            instruction: Some(limit_order::Instructions {
                instruction: limit_order::instruction::Instruction::FlashFillOrder {
                    accounts: limit_order::instruction::FlashFillOrderAccounts {
                        order: p("HzKfs1qTtpvV9u9yh2imBh7aKtFBeuffAFbn3L1pA6Qw"),
                        reserve: p("DzNK4xxbthzVRJVnPCHhBYAFki24n9B2McPmPUtiTLkc"),
                        maker: p("7c49JgubU1Q8mdBUt1kgdSkP2ezgCSgenknV4KMyhSJQ"),
                        taker: p("71WDyyCsZwyEYDV91Qrb212rdg6woCHYQhFnmZUBxiJ6"),
                        maker_output_account: p("7x9pfoBP1syJPzN4YqBNWX1VbQQkaVPdRuxBrWab5GuC"),
                        taker_input_account: p("CGhH4cVn7zdkwGEWojhFcqfyt7zPLi3AVDuhWeHA3WiQ"),
                        fee_authority: p("H3vkQqNVWySTD4c1Y91wtoT5iwxKSVtVLfC2rD8SgwTN"),
                        program_fee_account: p("GXYDRDTopqAuJVi9F24t3uVDjQ41wQvYa6qdDcNQSCvm"),
                        referral: None,
                        input_mint: p("So11111111111111111111111111111111111111112"),
                        input_mint_token_program: p("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                        output_mint: p("WENWENvqqNya429ubCdR81ZmD69brwQaaBYY6p3LCpk"),
                        output_mint_token_program: p("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                        system_program: p("11111111111111111111111111111111"),
                        remaining_accounts: vec![],
                    },
                    args: limit_order::instruction::FlashFillOrderArgs {
                        max_taking_amount: 91_407_969_193,
                    },
                },
            }),
            program_events: vec![limit_order::Events {
                event: limit_order::event::Event::TradeEvent {
                    accounts: limit_order::event::TradeEventAccounts {
                        remaining_accounts: vec![],
                    },
                    args: limit_order::event::TradeEventArgs {
                        order_key: p("HzKfs1qTtpvV9u9yh2imBh7aKtFBeuffAFbn3L1pA6Qw"),
                        taker: p("71WDyyCsZwyEYDV91Qrb212rdg6woCHYQhFnmZUBxiJ6"),
                        remaining_in_amount: 0,
                        remaining_out_amount: 0,
                        in_amount: 151_188_480,
                        out_amount: 91_407_969_193,
                    },
                },
            }],
        },
    ];

    let expected_refs: Vec<_> = expected.iter().collect();
    assert_eq!(outputs, expected_refs);
}
