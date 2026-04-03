use vixen_test_utils::{check_protobuf_format, p};
use yellowstone_vixen_core::Parser;
use yellowstone_vixen_mock::tx_fixture;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/limit_order_v2.json");

#[test]
fn check_protobuf_schema() {
    check_protobuf_format(limit_order2::PROTOBUF_SCHEMA);
    insta::assert_snapshot!(limit_order2::PROTOBUF_SCHEMA);
}

///
/// Parse a fill-order transaction.
///
/// The fixture contains a `FillOrder` instruction with a binary `swapData`
/// bytes field (routed through Jupiter) and a CPI self-invocation `TradeEvent`.
///
#[tokio::test]
async fn parse_fill_order_transaction() {
    let parser = limit_order2::InstructionParser;

    let ixs = tx_fixture!(
        "VLzYfuPuxFcAV841yKgyvuYeJK9VgBKf9YSH4jHGLAJhc84jJZzYbrh97XJRQh3tfsLEAEkws3CRcmhsGeDEdFk",
        &parser
    );

    let outputs: Vec<_> = ixs.iter().filter_map(|out| out.as_ref()).collect();

    let expected = vec![limit_order2::ProgramEventOutput {
        instruction: Some(limit_order2::Instructions {
            instruction: limit_order2::instruction::Instruction::FillOrder {
                accounts: limit_order2::instruction::FillOrderAccounts {
                    taker: p("j1oxqtEHFn7rUkdABJLmtVtz5fFmHFs4tCG3fWJnkHX"),
                    maker: p("5Lm2xAx8Y4VfxVpa1JrrcxZ4UMqvYzzCr9iHLHz6CgMG"),
                    order: p("Dme3Cn7UZ7gRt2LjkRXNPenXYrk65fxUs1eVqw6teUPw"),
                    taker_input_mint_account: p("6WMgSeD8qx2DDqVDXWjxtG5mTPQ2jmgwNqpVxvURHAus"),
                    taker_output_mint_account: p("9ydcYkLagRQeA1QiXndauAwA3kDpr85beLesmfNcUcSw"),
                    maker_output_mint_account: None,
                    fee_account: p("BTMWnyZf9ypca3mckpoiQqZcGHbKwRbDhVXqGDhDTs4N"),
                    order_input_mint_account: p("B1ESrH2j5i4TWVDYQojaqHUAMS7ZzQ9b94ZwDEynF3ik"),
                    input_mint: p("J8PSdNP3QewKq2Z1JJJFDMaqF7KcaiJhR7gbr5KZpump"),
                    input_token_program: p("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                    output_mint: p("So11111111111111111111111111111111111111112"),
                    output_token_program: p("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                    jupiter_program: p("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"),
                    system_program: p("11111111111111111111111111111111"),
                    event_authority: p("ArsDfE54RTkC3zhtzPdtvTtKw9XSV5w1PCTBFVGiLd52"),
                    program: p("j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X"),
                    remaining_accounts: vec![
                        p("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                        p("j1oxqtEHFn7rUkdABJLmtVtz5fFmHFs4tCG3fWJnkHX"),
                        p("6WMgSeD8qx2DDqVDXWjxtG5mTPQ2jmgwNqpVxvURHAus"),
                        p("9ydcYkLagRQeA1QiXndauAwA3kDpr85beLesmfNcUcSw"),
                        p("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"),
                        p("So11111111111111111111111111111111111111112"),
                        p("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"),
                        p("D8cy77BBepLMngZx6ZukaTff5hCt1HrWyKk3Hnd9oitf"),
                        p("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"),
                        p("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA"),
                        p("3KFCgJ5R3zshW8hTDbzjSrrKSRYmKvsMfhc4Vo4iddxD"),
                        p("j1oxqtEHFn7rUkdABJLmtVtz5fFmHFs4tCG3fWJnkHX"),
                        p("ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw"),
                        p("J8PSdNP3QewKq2Z1JJJFDMaqF7KcaiJhR7gbr5KZpump"),
                        p("So11111111111111111111111111111111111111112"),
                        p("6WMgSeD8qx2DDqVDXWjxtG5mTPQ2jmgwNqpVxvURHAus"),
                        p("9ydcYkLagRQeA1QiXndauAwA3kDpr85beLesmfNcUcSw"),
                        p("D3HR3UmnpbFcryUW1xAF1tectFc9zmypJxZAo4CvyG74"),
                        p("2rDB8Srhx2DPCCAxgqkS13mu9FeamCuGWgua1KfwQLXX"),
                        p("AVmoTthdrX6tKt4nDjco2D775W2YK3sDhxPcMmzUAmTY"),
                        p("FGptqdxjahafaCzpZ1T6EDtCzYMv7Dyn5MgBLyB3VUFW"),
                        p("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                        p("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                        p("11111111111111111111111111111111"),
                        p("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                        p("GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR"),
                        p("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA"),
                        p("7cvRLG2KdkE1P35ZXuTjCBLDJVakrN4fytQcrJq8iD45"),
                        p("E1pXBuY8ubXN3AYrBbUErLPwZkvNYHG9TRU9TXXn7YNL"),
                        p("5PHirr8joyTMp9JMm6nW7hNDVyEYdkzDqazxPD7RaTjx"),
                        p("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ"),
                        p("j1oxqtEHFn7rUkdABJLmtVtz5fFmHFs4tCG3fWJnkHX"),
                        p("BHCi57CyXSsScqWavQaqUqoZfbdA8Afx6VPTuJ7fjuaT"),
                        p("9xiBcrxAJaCE5xZYPiLHpnnizQjuYMsvdhsrod3iFFZQ"),
                        p("B7WAWtxoY46s9TTEofCbt2UyX1dhiRZRe4ptkVJRgqCm"),
                        p("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"),
                    ],
                },
                args: limit_order2::instruction::FillOrderArgs {
                    input_amount: 111_914_692_932,
                    swap_data: vec![
                        229, 23, 203, 151, 122, 227, 173, 42, 1, 0, 0, 0, 100, 100, 0, 1, 68, 177,
                        162, 14, 26, 0, 0, 0, 210, 167, 213, 175, 0, 0, 0, 0, 50, 0, 0,
                    ],
                },
            },
        }),
        program_events: vec![limit_order2::Events {
            event: limit_order2::event::Event::TradeEvent {
                accounts: limit_order2::event::TradeEventAccounts {
                    remaining_accounts: vec![],
                },
                args: limit_order2::event::TradeEventArgs {
                    order_key: p("Dme3Cn7UZ7gRt2LjkRXNPenXYrk65fxUs1eVqw6teUPw"),
                    taker: p("j1oxqtEHFn7rUkdABJLmtVtz5fFmHFs4tCG3fWJnkHX"),
                    remaining_making_amount: 0,
                    remaining_taking_amount: 0,
                    making_amount: 111_914_692_932,
                    taking_amount: 2_950_014_930,
                },
            },
        }],
    }];

    let expected_refs: Vec<_> = expected.iter().collect();
    assert_eq!(outputs, expected_refs);
}
