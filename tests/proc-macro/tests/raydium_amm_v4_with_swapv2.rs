mod common;

use common::pubkey;
use yellowstone_vixen_core::Parser;
use yellowstone_vixen_mock::tx_fixture;
use yellowstone_vixen_proc_macro::include_vixen_parser;

// This IDL have non-even length hex strings for discriminators, which caused `hex::decode` to fail before we added padding logic. This test ensures that the padding logic works correctly.

include_vixen_parser!("idls/raydium_amm_v4_with_swapv2.json");

#[test]
fn check_protobuf_schema() {
    common::check_protobuf_format(raydium_amm::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(raydium_amm::PROTOBUF_SCHEMA);
}

///
/// Demonstrates using CustomInstructionParser with a resolver that handles
/// the 17-account SwapBaseIn variant (where ammTargetOrdersDeprecated is omitted).
///
/// The IDL defines two variants:
///   - `swapBaseIn` (18 accounts, includes ammTargetOrdersDeprecated)
///   - `swapBaseInCompact` (17 accounts, without the deprecated account)
///
/// Both share the same discriminator (0x09). The resolver disambiguates by account count.
///
#[tokio::test]
async fn parse_swap_base_in_with_custom_resolver() {
    use yellowstone_vixen_core::ParseError;

    #[derive(Debug, Copy, Clone)]
    struct RaydiumResolver;

    impl raydium_amm::InstructionResolver for RaydiumResolver {
        fn resolve(
            &self,
            accounts: &[yellowstone_vixen_core::KeyBytes<32>],
            data: &[u8],
        ) -> Result<raydium_amm::Instructions, ParseError> {
            // SwapBaseIn discriminator is 0x09 — both variants share it.
            // Disambiguate by account count.
            if data.first() == Some(&0x09) {
                return if accounts.len() >= 18 {
                    raydium_amm::parse_swap_base_in(accounts, data)
                } else {
                    raydium_amm::parse_swap_base_in_compact(accounts, data)
                };
            }

            // Everything else: default resolution
            raydium_amm::resolve_instruction_default(accounts, data)
        }
    }

    let parser = raydium_amm::CustomInstructionParser::new(RaydiumResolver).with_raw_logs();

    let ixs = tx_fixture!(
        "3J7xWK1gZTyk6GQJUopbP34Z1GxRSN1dLEmq3PyUrbMVYjkjfvMSLNRDVDggVrneJJGt1cdXsgfRUGYnavP75XQR",
        &parser
    );

    // This transaction has 17 accounts, so the resolver routes to SwapBaseInCompact
    let swap_ix = ixs
        .iter()
        .find_map(|ix| {
            let ix = ix.as_ref()?;
            matches!(
                &ix.instruction,
                raydium_amm::instruction::Instruction::SwapBaseInCompact { .. }
            )
            .then_some(ix)
        })
        .expect("no SwapBaseInCompact found");

    let expected = raydium_amm::Instructions {
        instruction: raydium_amm::instruction::Instruction::SwapBaseInCompact {
            accounts: raydium_amm::instruction::SwapBaseInCompactAccounts {
                token_program: pubkey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                amm: pubkey("5WGYajM1xtLy3QrLHGSX4YPwsso3jrjEsbU1VivUErzk"),
                amm_authority: pubkey("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1"),
                amm_open_orders: pubkey("5WGYajM1xtLy3QrLHGSX4YPwsso3jrjEsbU1VivUErzk"),
                pool_coin_token_account: pubkey("AMtPGYQS873njs35mD9MAAMKoospEuzNHPy7LQuuKo4A"),
                pool_pc_token_account: pubkey("BUvMbqP311JDU4ZGnf1rSZJLjutTU9VpNLEos393TYyW"),
                serum_program: pubkey("5WGYajM1xtLy3QrLHGSX4YPwsso3jrjEsbU1VivUErzk"),
                serum_market: pubkey("5WGYajM1xtLy3QrLHGSX4YPwsso3jrjEsbU1VivUErzk"),
                serum_bids: pubkey("5WGYajM1xtLy3QrLHGSX4YPwsso3jrjEsbU1VivUErzk"),
                serum_asks: pubkey("5WGYajM1xtLy3QrLHGSX4YPwsso3jrjEsbU1VivUErzk"),
                serum_event_queue: pubkey("5WGYajM1xtLy3QrLHGSX4YPwsso3jrjEsbU1VivUErzk"),
                serum_coin_vault_account: pubkey("5WGYajM1xtLy3QrLHGSX4YPwsso3jrjEsbU1VivUErzk"),
                serum_pc_vault_account: pubkey("5WGYajM1xtLy3QrLHGSX4YPwsso3jrjEsbU1VivUErzk"),
                serum_vault_signer: pubkey("5WGYajM1xtLy3QrLHGSX4YPwsso3jrjEsbU1VivUErzk"),
                user_source_token_account: pubkey("3xGDodLWEbjPde4JedHAuGSKbM2VbqgpzK76pHpinJhL"),
                user_destination_token_account: pubkey(
                    "BZP7eTQqi4M7XXWci3rVcM3kJsTCHqdpBewLa6aCyZVu",
                ),
                user_source_owner: pubkey("CsVdJ8WH8Q9eHSTRpwtwN3TYApm24QnLKYUMNxJ3DaED"),
                remaining_accounts: vec![],
            },
            args: raydium_amm::instruction::SwapBaseInCompactArgs {
                amount_in: 820_106_078_370,
                minimum_amount_out: 0,
            },
        },
        raw_logs: vec![
            "Program BoobsBSMpFRBA91sNwKLYShRRQPH5GjoCH4NhLUt4yRo invoke [1]".into(),
            "Program log: Instruction: SwapRaydiumV4".into(),
            "Program 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 invoke [2]".into(),
            "Program log: ray_log: \
             A6KoIfK+AAAAAAAAAAAAAAABAAAAAAAAADdfVHZYuQAAA7EJc2IAAAAQglcYrmszAeP+PAAAAAAA"
                .into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]".into(),
            "Program log: Instruction: Transfer".into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 20988 compute \
             units"
                .into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]".into(),
            "Program log: Instruction: Transfer".into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 13874 compute \
             units"
                .into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".into(),
            "Program 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 consumed 26537 of 35072 compute \
             units"
                .into(),
            "Program 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 success".into(),
            "Program BoobsBSMpFRBA91sNwKLYShRRQPH5GjoCH4NhLUt4yRo consumed 46413 of 54550 compute \
             units"
                .into(),
            "Program BoobsBSMpFRBA91sNwKLYShRRQPH5GjoCH4NhLUt4yRo success".into(),
        ],
    };

    assert_eq!(swap_ix, &expected);
}

///
/// The default InstructionParser now automatically disambiguates instructions
/// sharing a discriminator when they have different account counts.
///
/// For raydium_amm, `swapBaseIn` (18 accounts) and `swapBaseInCompact` (17 accounts)
/// both share discriminator 0x09. The default resolver picks the right one by checking
/// `accounts.len() >= 18` first, then falling back to `swapBaseInCompact`.
///
#[tokio::test]
async fn parse_swap_base_in_with_default_parser() {
    let parser = raydium_amm::InstructionParser.with_raw_logs();

    let ixs = tx_fixture!(
        "3J7xWK1gZTyk6GQJUopbP34Z1GxRSN1dLEmq3PyUrbMVYjkjfvMSLNRDVDggVrneJJGt1cdXsgfRUGYnavP75XQR",
        &parser
    );

    // This transaction has 17 accounts — the default resolver should route
    // to SwapBaseInCompact automatically.
    let swap_ix = ixs
        .iter()
        .find_map(|ix| {
            let ix = ix.as_ref()?;
            matches!(
                &ix.instruction,
                raydium_amm::instruction::Instruction::SwapBaseInCompact { .. }
            )
            .then_some(ix)
        })
        .expect("default parser should resolve SwapBaseInCompact by account count");

    assert_eq!(swap_ix.raw_logs, vec![
        "Program BoobsBSMpFRBA91sNwKLYShRRQPH5GjoCH4NhLUt4yRo invoke [1]",
        "Program log: Instruction: SwapRaydiumV4",
        "Program 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 invoke [2]",
        "Program log: ray_log: \
         A6KoIfK+AAAAAAAAAAAAAAABAAAAAAAAADdfVHZYuQAAA7EJc2IAAAAQglcYrmszAeP+PAAAAAAA",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
        "Program log: Instruction: Transfer",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 20988 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
        "Program log: Instruction: Transfer",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 13874 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 consumed 26537 of 35072 compute \
         units",
        "Program 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 success",
        "Program BoobsBSMpFRBA91sNwKLYShRRQPH5GjoCH4NhLUt4yRo consumed 46413 of 54550 compute \
         units",
        "Program BoobsBSMpFRBA91sNwKLYShRRQPH5GjoCH4NhLUt4yRo success",
    ]);
}
