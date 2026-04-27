use base64::Engine;
use borsh::to_vec;
use vixen_test_utils::p;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/jupiter_log_scope_regression.json");

const JUPITER_PROGRAM_ID: &str = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";
const GOOSEFX_PROGRAM_ID: &str = "GAMMA7meSFWaBXF25oSUgmGRwaW6sCMFLmBNiMSdbHVT";
const SWAP_EVENT_DISCRIMINATOR: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];

fn swap_event_log_payload() -> String {
    let args = jupiter_regression::event::SwapEventArgs {
        amm: p("GAMMA7meSFWaBXF25oSUgmGRwaW6sCMFLmBNiMSdbHVT"),
        input_mint: p("So11111111111111111111111111111111111111112"),
        input_amount: 42,
        output_mint: p("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
        output_amount: 1337,
    };
    let mut payload = SWAP_EVENT_DISCRIMINATOR.to_vec();
    payload.extend(to_vec(&args).expect("serialize swap event args"));
    base64::engine::general_purpose::STANDARD.encode(payload)
}

/// Regression for DFlow tx
/// 4VBS4Uy26RrYvzCbDcyG8eh7YXdy6tZxja8AK4EWYCmqbTfqFNykLt1KyZfQ2xyYqdzSMLkGmb9aC3RiKLi9Vdtn
///
/// The bug was that a nested GooseFX `"Program data:"` line inside a Jupiter
/// invoke could be decoded as Jupiter's `swapEvent` because the discriminator
/// bytes collided.
#[test]
fn ignores_nested_goosefx_program_data_for_jupiter_swap_event() {
    let encoded = swap_event_log_payload();
    let logs = vec![
        format!("Program {JUPITER_PROGRAM_ID} invoke [1]"),
        "Program log: Instruction: Route".to_string(),
        format!("Program {GOOSEFX_PROGRAM_ID} invoke [2]"),
        format!("Program data: {encoded}"),
        format!("Program {GOOSEFX_PROGRAM_ID} success"),
        format!("Program {JUPITER_PROGRAM_ID} success"),
    ];

    let events = jupiter_regression::resolve_events_from_logs(&logs);
    assert!(
        events.is_empty(),
        "nested GooseFX program data must not be decoded as Jupiter swap events"
    );
}

#[test]
fn parses_top_level_jupiter_program_data_for_swap_event() {
    let encoded = swap_event_log_payload();
    let logs = vec![
        format!("Program {JUPITER_PROGRAM_ID} invoke [1]"),
        format!("Program data: {encoded}"),
        format!("Program {JUPITER_PROGRAM_ID} success"),
    ];

    let events = jupiter_regression::resolve_events_from_logs(&logs);
    assert_eq!(
        events.len(),
        1,
        "top-level Jupiter program data should parse"
    );

    let (accounts, args) = match &events[0].event {
        jupiter_regression::event::Event::SwapEvent { accounts, args } => (accounts, args),
    };

    assert_eq!(
        accounts,
        &jupiter_regression::event::SwapEventAccounts {
            remaining_accounts: vec![],
        }
    );
    assert_eq!(args.amm, p(GOOSEFX_PROGRAM_ID));
    assert_eq!(
        args.input_mint,
        p("So11111111111111111111111111111111111111112")
    );
    assert_eq!(args.input_amount, 42);
    assert_eq!(
        args.output_mint,
        p("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
    );
    assert_eq!(args.output_amount, 1337);
}
