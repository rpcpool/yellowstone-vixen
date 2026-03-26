use vixen_test_utils::{check_protobuf_format, p};
use yellowstone_vixen_core::Parser;
use yellowstone_vixen_mock::tx_fixture;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("../idls/okx-labs1.json");

#[test]
fn check_protobuf_schema() {
    check_protobuf_format(dex_solana::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(dex_solana::PROTOBUF_SCHEMA);
}

#[tokio::test]
async fn parse_swap_v3_ix() {
    let parser = dex_solana::InstructionParser;

    let ixs = tx_fixture!(
        "2WhQXGYdZrrQrixgsxALHgZUiM5SdHasiX56jJb6QMZHXnYf69zFfUoJLmCjKHzfuA2UV5kaPRfhqPGudJDZeWwk",
        &parser
    );

    let (accounts, args) = ixs
        .iter()
        .find_map(|ix| match &ix.as_ref()?.instruction {
            dex_solana::instruction::Instruction::SwapV3 { accounts, args } => {
                Some((accounts, args))
            },
            _ => None,
        })
        .expect("no SwapV3 found");

    let expected = dex_solana::instruction::SwapV3 {
        accounts: dex_solana::instruction::SwapV3Accounts {
            payer: p("66FZfEpN1ZwYQaEgnZrKohNGaS4T3kzRkdoWZTGMiKTc"),
            source_token_account: p("5mWLMNwyyiyZfvQW3nHfJRjhXyany7aQjNfehr2v96dh"),
            destination_token_account: p("ELvYu6Df3pB1JAueraFGHVYFNWMdxrSnLV1eCK6oMt3L"),
            source_mint: p("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"),
            destination_mint: p("So11111111111111111111111111111111111111112"),
            commission_account: p("6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma"),
            platform_fee_account: p("6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma"),
            sa_authority: p("HV1KXxWFaSeriyFvXyx48FqG9BoFbfinB8njCJonqP7K"),
            source_token_sa: p("D5wjgMadAHGstes5ZTCuw8XnGhxiQdsiMo3D44bei3tu"),
            destination_token_sa: p("2rikd7tzPbmowhUJzPNVtX7fuUGcnBa8jqJnx6HbtHeE"),
            source_token_program: p("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            destination_token_program: p("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: p("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            system_program: p("11111111111111111111111111111111"),
            remaining_accounts: vec![
                p("ALPHAQmeA7bjrVuccPsYPiCvsi428SNwte66Srvs4pHA"),
                p("HV1KXxWFaSeriyFvXyx48FqG9BoFbfinB8njCJonqP7K"),
                p("D5wjgMadAHGstes5ZTCuw8XnGhxiQdsiMo3D44bei3tu"),
                p("2rikd7tzPbmowhUJzPNVtX7fuUGcnBa8jqJnx6HbtHeE"),
                p("hKH9LFREBm3TxTx5Ex6D1nHTKEA8ii3CWfhEkB9s21u"),
                p("HZyb7Gv2pWTRYq8XuaeWBePQ8CDNhxigkNohZU2dLPEC"),
                p("BjVanGXAEmHL677Lj7ywusqE7Uaop3sqYb7kiL2LFiK9"),
                p("4xZYFLR2eyo2vrfsGuJZEPsgZafamE9fFPJ5wYJDW6nU"),
                p("BjVanGXAEmHL677Lj7ywusqE7Uaop3sqYb7kiL2LFiK9"),
                p("4xZYFLR2eyo2vrfsGuJZEPsgZafamE9fFPJ5wYJDW6nU"),
                p("4xZYFLR2eyo2vrfsGuJZEPsgZafamE9fFPJ5wYJDW6nU"),
                p("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                p("Sysvar1nstructions1111111111111111111111111"),
                p("ALPHAQmeA7bjrVuccPsYPiCvsi428SNwte66Srvs4pHA"),
                p("ALPHAQmeA7bjrVuccPsYPiCvsi428SNwte66Srvs4pHA"),
                p("MNFSTqtC93rEfYHB6hF82sKdZpUDFWkViLByLd1k1Ms"),
                p("HV1KXxWFaSeriyFvXyx48FqG9BoFbfinB8njCJonqP7K"),
                p("D5wjgMadAHGstes5ZTCuw8XnGhxiQdsiMo3D44bei3tu"),
                p("2rikd7tzPbmowhUJzPNVtX7fuUGcnBa8jqJnx6HbtHeE"),
                p("7ecvmhGKVcK4SgxeGQJG6yVwVAhbQxLrBuaMoUmpRZ6i"),
                p("11111111111111111111111111111111"),
                p("4ThJXdq1ga5aaZSXRa9gDgEBwkj7iG2tJxJ3XXC2K23u"),
                p("GkXG58JMDWCfoAorj8uCJcKj748uqV7tfYaV67mSW5LH"),
                p("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                p("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"),
                p("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                p("So11111111111111111111111111111111111111112"),
                p("7mR36vj6pvg1U1cRatvUbLG57yqsd1ojLbrgxb6azaQ1"),
                p("E1mBVQyt7BHK8SaBSfME7usYxx94T4DtHEjbUpEBhZx"),
            ],
        },
        args: dex_solana::instruction::SwapV3Args {
            args: dex_solana::SwapArgs {
                amount_in: 38_623_833_337,
                expect_amount_out: 48_981_429_027,
                min_return: 48_491_614_736,
                amounts: vec![38_623_833_337],
                routes: vec![dex_solana::SwapArgsRoutesInner {
                    items: vec![dex_solana::Route {
                        dexes: vec![
                            dex_solana::Dex {
                                kind: dex_solana::dex::Kind::AlphaQ(dex_solana::DexAlphaQ {}),
                            },
                            dex_solana::Dex {
                                kind: dex_solana::dex::Kind::Manifest(dex_solana::DexManifest {}),
                            },
                        ],
                        weights: vec![54, 46],
                    }],
                }],
            },
            commission_info: 0,
            platform_fee_rate: 0,
            order_id: 0,
        },
    };

    assert_eq!(accounts, &expected.accounts);
    assert_eq!(args, &expected.args);
}
