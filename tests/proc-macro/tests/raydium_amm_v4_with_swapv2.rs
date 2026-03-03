mod common;

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

    let parser = raydium_amm::CustomInstructionParser(RaydiumResolver);

    let ixs = tx_fixture!(
        "3J7xWK1gZTyk6GQJUopbP34Z1GxRSN1dLEmq3PyUrbMVYjkjfvMSLNRDVDggVrneJJGt1cdXsgfRUGYnavP75XQR",
        &parser
    );

    // This transaction has 17 accounts, so the resolver routes to SwapBaseInCompact
    let swap = ixs
        .iter()
        .find_map(|ix| match ix.as_ref()?.instruction.as_ref()? {
            raydium_amm::instruction::Instruction::SwapBaseInCompact(s) => Some(s),
            _ => None,
        })
        .expect("no SwapBaseInCompact found");

    use yellowstone_vixen_core::PublicKey;

    let expected = raydium_amm::instruction::SwapBaseInCompact {
        accounts: Some(raydium_amm::instruction::SwapBaseInCompactAccounts {
            token_program: PublicKey::new(vec![
                6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28,
                180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
            ]),
            amm: PublicKey::new(vec![
                66, 238, 42, 218, 212, 12, 53, 80, 219, 34, 134, 23, 205, 81, 84, 25, 184, 252,
                217, 221, 16, 160, 1, 65, 86, 170, 253, 53, 91, 192, 44, 17,
            ]),
            amm_authority: PublicKey::new(vec![
                65, 87, 176, 88, 15, 49, 197, 252, 228, 74, 98, 88, 45, 188, 249, 215, 142, 231,
                89, 67, 160, 132, 163, 147, 179, 80, 54, 141, 34, 137, 147, 8,
            ]),
            amm_open_orders: PublicKey::new(vec![
                66, 238, 42, 218, 212, 12, 53, 80, 219, 34, 134, 23, 205, 81, 84, 25, 184, 252,
                217, 221, 16, 160, 1, 65, 86, 170, 253, 53, 91, 192, 44, 17,
            ]),
            pool_coin_token_account: PublicKey::new(vec![
                139, 18, 230, 5, 1, 32, 156, 221, 91, 96, 91, 116, 169, 30, 111, 156, 245, 246, 42,
                109, 219, 228, 154, 117, 141, 138, 207, 253, 46, 10, 148, 255,
            ]),
            pool_pc_token_account: PublicKey::new(vec![
                155, 187, 225, 72, 15, 114, 220, 199, 26, 153, 140, 50, 24, 236, 42, 146, 92, 89,
                130, 216, 172, 148, 41, 204, 136, 249, 191, 107, 217, 101, 48, 217,
            ]),
            serum_program: PublicKey::new(vec![
                66, 238, 42, 218, 212, 12, 53, 80, 219, 34, 134, 23, 205, 81, 84, 25, 184, 252,
                217, 221, 16, 160, 1, 65, 86, 170, 253, 53, 91, 192, 44, 17,
            ]),
            serum_market: PublicKey::new(vec![
                66, 238, 42, 218, 212, 12, 53, 80, 219, 34, 134, 23, 205, 81, 84, 25, 184, 252,
                217, 221, 16, 160, 1, 65, 86, 170, 253, 53, 91, 192, 44, 17,
            ]),
            serum_bids: PublicKey::new(vec![
                66, 238, 42, 218, 212, 12, 53, 80, 219, 34, 134, 23, 205, 81, 84, 25, 184, 252,
                217, 221, 16, 160, 1, 65, 86, 170, 253, 53, 91, 192, 44, 17,
            ]),
            serum_asks: PublicKey::new(vec![
                66, 238, 42, 218, 212, 12, 53, 80, 219, 34, 134, 23, 205, 81, 84, 25, 184, 252,
                217, 221, 16, 160, 1, 65, 86, 170, 253, 53, 91, 192, 44, 17,
            ]),
            serum_event_queue: PublicKey::new(vec![
                66, 238, 42, 218, 212, 12, 53, 80, 219, 34, 134, 23, 205, 81, 84, 25, 184, 252,
                217, 221, 16, 160, 1, 65, 86, 170, 253, 53, 91, 192, 44, 17,
            ]),
            serum_coin_vault_account: PublicKey::new(vec![
                66, 238, 42, 218, 212, 12, 53, 80, 219, 34, 134, 23, 205, 81, 84, 25, 184, 252,
                217, 221, 16, 160, 1, 65, 86, 170, 253, 53, 91, 192, 44, 17,
            ]),
            serum_pc_vault_account: PublicKey::new(vec![
                66, 238, 42, 218, 212, 12, 53, 80, 219, 34, 134, 23, 205, 81, 84, 25, 184, 252,
                217, 221, 16, 160, 1, 65, 86, 170, 253, 53, 91, 192, 44, 17,
            ]),
            serum_vault_signer: PublicKey::new(vec![
                66, 238, 42, 218, 212, 12, 53, 80, 219, 34, 134, 23, 205, 81, 84, 25, 184, 252,
                217, 221, 16, 160, 1, 65, 86, 170, 253, 53, 91, 192, 44, 17,
            ]),
            user_source_token_account: PublicKey::new(vec![
                43, 223, 135, 76, 221, 119, 198, 35, 192, 77, 199, 87, 202, 206, 221, 133, 117,
                196, 92, 208, 71, 236, 18, 125, 174, 96, 214, 219, 55, 90, 227, 255,
            ]),
            user_destination_token_account: PublicKey::new(vec![
                156, 224, 117, 207, 36, 231, 36, 251, 254, 56, 85, 246, 136, 129, 177, 173, 32,
                212, 16, 198, 122, 104, 235, 23, 48, 67, 251, 64, 37, 29, 217, 220,
            ]),
            user_source_owner: PublicKey::new(vec![
                176, 95, 246, 195, 86, 53, 120, 198, 18, 165, 226, 150, 82, 137, 102, 66, 10, 246,
                22, 81, 25, 252, 64, 213, 90, 126, 143, 13, 211, 224, 58, 130,
            ]),
        }),
        args: Some(raydium_amm::instruction::SwapBaseInCompactArgs {
            amount_in: 820_106_078_370,
            minimum_amount_out: 0,
        }),
    };

    assert_eq!(swap, &expected);
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
    let parser = raydium_amm::InstructionParser;

    let ixs = tx_fixture!(
        "3J7xWK1gZTyk6GQJUopbP34Z1GxRSN1dLEmq3PyUrbMVYjkjfvMSLNRDVDggVrneJJGt1cdXsgfRUGYnavP75XQR",
        &parser
    );

    // This transaction has 17 accounts — the default resolver should route
    // to SwapBaseInCompact automatically.
    let swap = ixs
        .iter()
        .find_map(|ix| match ix.as_ref()?.instruction.as_ref()? {
            raydium_amm::instruction::Instruction::SwapBaseInCompact(s) => Some(s),
            _ => None,
        })
        .expect("default parser should resolve SwapBaseInCompact by account count");

    assert!(swap.accounts.is_some());
    assert!(swap.args.is_some());
}
