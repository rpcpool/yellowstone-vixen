mod common;

use yellowstone_vixen_core::Parser;
use yellowstone_vixen_mock::{account_fixture, tx_fixture};
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("idls/perp_idl.json");

#[test]
fn check_protobuf_schema() {
    common::check_protobuf_format(perpetuals::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(perpetuals::PROTOBUF_SCHEMA);
}

#[tokio::test]
async fn parse_custody_account() {
    let parser = perpetuals::AccountParser;
    let account = account_fixture!("7xS2gz2bTp3fwCC7knJvUWTEU9Tycczu6VhJYKgi1wdz", &parser);

    let custody = match account.account {
        Some(perpetuals::account::Account::Custody(c)) => c,
        _ => panic!("Unexpected account state"),
    };

    let expected = perpetuals::Custody {
        pool: perpetuals::PublicKey::new(vec![
            62, 30, 36, 115, 199, 52, 6, 84, 235, 135, 41, 0, 53, 21, 28, 64, 43, 208, 227, 201,
            124, 180, 36, 72, 134, 231, 32, 52, 179, 11, 77, 252,
        ]),
        mint: perpetuals::PublicKey::new(vec![
            6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53, 218, 196, 57,
            220, 26, 235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1,
        ]),
        token_account: perpetuals::PublicKey::new(vec![
            155, 188, 50, 161, 141, 135, 28, 7, 53, 93, 210, 81, 97, 36, 21, 196, 32, 76, 171, 128,
            29, 185, 238, 194, 146, 101, 3, 81, 177, 102, 210, 110,
        ]),
        decimals: 9,
        is_stable: false,
        oracle: Some(perpetuals::OracleParams {
            oracle_account: perpetuals::PublicKey::new(vec![
                96, 49, 71, 4, 52, 13, 237, 223, 55, 31, 212, 36, 114, 20, 143, 36, 142, 157, 26,
                109, 26, 94, 178, 172, 58, 205, 139, 127, 213, 214, 178, 67,
            ]),
            oracle_type: Some(perpetuals::OracleType {
                kind: Some(perpetuals::oracle_type::Kind::Pyth(
                    perpetuals::OracleTypePyth {},
                )),
            }),
            buffer: 50,
            max_price_age_sec: 5,
        }),
        pricing: Some(perpetuals::PricingParams {
            trade_impact_fee_scalar: 3_750_000_000_000_000,
            buffer: 0,
            swap_spread: 0,
            max_leverage: 5_000_000,
            max_global_long_sizes: 410_000_000_000_000,
            max_global_short_sizes: 112_293_947_657_720,
        }),
        permissions: Some(perpetuals::Permissions {
            allow_swap: true,
            allow_add_liquidity: true,
            allow_remove_liquidity: true,
            allow_increase_position: true,
            allow_decrease_position: true,
            allow_collateral_withdrawal: true,
            allow_liquidate_position: true,
        }),
        target_ratio_bps: 4700,
        assets: Some(perpetuals::Assets {
            fees_reserves: 466_896_904,
            owned: 6_368_829_939_225_389,
            locked: 398_908_550_717_247,
            guaranteed_usd: 24_953_150_512_329,
            global_short_sizes: 10_560_791_260_461,
            global_short_average_prices: 83_486_086,
        }),
        funding_rate_state: Some(perpetuals::FundingRateState {
            cumulative_interest_rate: vec![151, 179, 251, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            last_update: 1_771_834_760,
            hourly_funding_dbps: 0,
        }),
        bump: 253,
        token_account_bump: 255,
        increase_position_bps: 6,
        decrease_position_bps: 6,
        max_position_size_usd: 10_000_000_000_000,
        doves_oracle: perpetuals::PublicKey::new(vec![
            31, 236, 44, 187, 11, 175, 112, 110, 143, 105, 186, 138, 53, 218, 38, 247, 99, 111, 71,
            194, 119, 3, 29, 140, 62, 4, 86, 16, 146, 217, 76, 97,
        ]),
        jump_rate_state: Some(perpetuals::JumpRateState {
            min_rate_bps: 1000,
            max_rate_bps: 15000,
            target_rate_bps: 3500,
            target_utilization_rate: 800_000_000,
        }),
        doves_ag_oracle: perpetuals::PublicKey::new(vec![
            216, 42, 235, 57, 188, 70, 146, 145, 46, 181, 242, 170, 224, 18, 127, 36, 173, 176,
            246, 182, 107, 253, 118, 9, 80, 73, 48, 236, 108, 178, 99, 136,
        ]),
        price_impact_buffer: Some(perpetuals::PriceImpactBuffer {
            open_interest: vec![
                0,
                0,
                -3_691_644_661,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                620_487_212,
                0,
                0,
                0,
                0,
                1_936_686_312,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                2_385_457_659,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                7_440_976_097,
                0,
                0,
                0,
            ],
            last_updated: 1_771_834_760,
            fee_factor: 1,
            exponent: 1.0,
            delta_imbalance_threshold_decimal: 1_500_000_000_000,
            max_fee_bps: 50,
        }),
        borrow_lend_parameters: Some(perpetuals::BorrowLendParams {
            borrows_limit_in_bps: 0,
            maintainance_margin_bps: 0,
            protocol_fee_bps: 0,
            liquidation_margin: 0,
            liquidation_fee_bps: 0,
        }),
        borrows_funding_rate_state: Some(perpetuals::FundingRateState {
            cumulative_interest_rate: vec![
                0, 0, 100, 167, 179, 182, 224, 13, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            last_update: 1_771_834_760,
            hourly_funding_dbps: 0,
        }),
        debt: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        borrow_lend_interests_accured: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        borrow_limit_in_token_amount: 0,
        min_interest_fee_bps: 0,
        min_interest_fee_grace_period_seconds: 0,
        total_staked_amount_lamports: 5_620_789_931_843_821,
        max_total_staked_amount_lamports: 6_000_000_000_000_000,
        external_swap_fee_multiplier_bps: 20000,
        disable_close_position_request: false,
        withdrawal_limit_token_amount: 0,
        withdrawal_token_amount_accumulated: 0,
        withdrawal_limit_last_reset_at: 0,
        withdrawal_limit_interval_seconds: 3600,
    };

    assert_eq!(custody, expected);
}

#[tokio::test]
async fn parse_pool_account() {
    let parser = perpetuals::AccountParser;
    let account = account_fixture!("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq", &parser);

    let pool = match account.account {
        Some(perpetuals::account::Account::Pool(pool)) => pool,
        _ => panic!("Unexpected account state"),
    };

    let expected = perpetuals::Pool {
        name: "Pool".to_string(),
        custodies: vec![
            perpetuals::PublicKey::new(vec![
                103, 89, 93, 216, 70, 192, 7, 242, 104, 150, 242, 174, 211, 27, 167, 181, 95, 209,
                44, 204, 21, 142, 11, 0, 122, 157, 143, 232, 70, 182, 159, 233,
            ]),
            perpetuals::PublicKey::new(vec![
                139, 170, 74, 72, 100, 34, 108, 192, 34, 72, 77, 200, 40, 30, 26, 22, 143, 92, 244,
                232, 5, 63, 26, 229, 6, 109, 145, 106, 136, 185, 223, 159,
            ]),
            perpetuals::PublicKey::new(vec![
                65, 77, 129, 72, 106, 241, 62, 110, 236, 158, 45, 91, 207, 69, 145, 50, 227, 164,
                102, 71, 9, 182, 109, 56, 208, 100, 119, 145, 36, 198, 206, 62,
            ]),
            perpetuals::PublicKey::new(vec![
                222, 232, 11, 52, 19, 162, 211, 16, 128, 98, 107, 146, 108, 56, 175, 52, 190, 209,
                134, 219, 54, 142, 151, 127, 58, 191, 75, 213, 69, 68, 154, 109,
            ]),
            perpetuals::PublicKey::new(vec![
                58, 87, 226, 203, 202, 208, 40, 131, 80, 35, 204, 171, 74, 216, 123, 210, 118, 78,
                143, 193, 50, 214, 142, 152, 102, 9, 235, 19, 75, 215, 134, 249,
            ]),
        ],
        aum_usd: vec![42, 148, 219, 152, 238, 6, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        limit: Some(perpetuals::Limit {
            max_aum_usd: vec![0, 128, 83, 238, 123, 168, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            token_weightage_buffer_bps: vec![232, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            buffer: 0,
        }),
        fees: Some(perpetuals::Fees {
            swap_multiplier: 100,
            stable_swap_multiplier: 100,
            add_remove_liquidity_bps: 20,
            swap_bps: 10,
            tax_bps: 100,
            stable_swap_bps: 2,
            stable_swap_tax_bps: 5,
            liquidation_reward_bps: 150,
            protocol_share_bps: 2500,
        }),
        pool_apr: Some(perpetuals::PoolApr {
            last_updated: 1771002900,
            fee_apr_bps: 1093,
            realized_fee_usd: 1250427617755,
        }),
        max_request_execution_sec: 45,
        bump: 252,
        lp_token_bump: 254,
        inception_time: 1689677832,
        parameter_update_oracle: Some(perpetuals::Secp256k1Pubkey {
            prefix: 2,
            key: vec![
                81, 186, 187, 169, 123, 155, 81, 29, 179, 83, 117, 14, 44, 57, 230, 19, 66, 46,
                133, 49, 33, 164, 138, 181, 32, 132, 12, 0, 39, 52, 40, 92,
            ],
        }),
        aum_usd_updated_at: 1771515604,
        max_trigger_price_diff_bps: 500,
        disable_close_position_request: false,
        max_lp_token_price_change_bps: 100,
        aum_usd_refreshed_at_slot: 401338012,
    };

    assert_eq!(pool, expected);
}

#[tokio::test]
async fn parse_decrease_position_with_tpsl_and_close_position_request_2_ix() {
    let parser = perpetuals::InstructionParser;

    let ixs = tx_fixture!(
        "3MnCnWjL83RBYGe8ADwhxFHTvbXmDEPyi2mxHQ2211kmQ4NAqBFvhjPexgiCy8o6mGPztnZwjbcxRi4WMGZDeah8",
        &parser
    );

    {
        let decrease = ixs
            .iter()
            .find_map(|ix| match ix.as_ref()?.instruction.as_ref()? {
                perpetuals::instruction::Instruction::DecreasePositionWithTpsl(s) => Some(s),
                _ => None,
            })
            .expect("no decrease position ix found");

        let expected = perpetuals::instruction::DecreasePositionWithTpsl {
            accounts: Some(perpetuals::instruction::DecreasePositionWithTpslAccounts {
                keeper: perpetuals::PublicKey::new(vec![
                    238, 103, 24, 154, 146, 36, 183, 11, 249, 126, 171, 22, 248, 91, 126, 66, 80,
                    130, 214, 35, 46, 153, 237, 255, 229, 32, 219, 75, 135, 121, 46, 21,
                ]),
                owner: perpetuals::PublicKey::new(vec![
                    138, 2, 154, 132, 201, 110, 60, 65, 124, 72, 228, 105, 180, 198, 154, 69, 177,
                    138, 239, 252, 160, 169, 123, 64, 90, 105, 103, 236, 9, 62, 69, 162,
                ]),
                transfer_authority: perpetuals::PublicKey::new(vec![
                    141, 38, 83, 12, 155, 36, 127, 146, 136, 234, 206, 55, 84, 75, 38, 56, 128,
                    192, 44, 173, 4, 211, 33, 80, 237, 29, 1, 248, 251, 221, 35, 134,
                ]),
                perpetuals: perpetuals::PublicKey::new(vec![
                    238, 151, 183, 0, 48, 24, 63, 163, 2, 12, 13, 6, 188, 207, 70, 162, 238, 235,
                    177, 159, 189, 77, 24, 177, 204, 63, 21, 61, 126, 170, 228, 30,
                ]),
                pool: perpetuals::PublicKey::new(vec![
                    62, 30, 36, 115, 199, 52, 6, 84, 235, 135, 41, 0, 53, 21, 28, 64, 43, 208, 227,
                    201, 124, 180, 36, 72, 134, 231, 32, 52, 179, 11, 77, 252,
                ]),
                position_request: perpetuals::PublicKey::new(vec![
                    233, 115, 77, 143, 129, 244, 145, 190, 189, 103, 5, 29, 76, 62, 201, 162, 249,
                    94, 14, 84, 101, 136, 146, 56, 83, 158, 180, 120, 90, 69, 12, 176,
                ]),
                position_request_ata: perpetuals::PublicKey::new(vec![
                    29, 156, 57, 200, 92, 59, 244, 71, 93, 107, 142, 243, 97, 105, 117, 15, 130,
                    93, 162, 16, 233, 61, 59, 152, 138, 96, 254, 171, 26, 93, 233, 251,
                ]),
                position: perpetuals::PublicKey::new(vec![
                    21, 27, 48, 86, 199, 205, 70, 175, 49, 243, 96, 1, 44, 35, 84, 21, 63, 30, 153,
                    23, 190, 180, 203, 32, 246, 28, 255, 218, 234, 227, 11, 255,
                ]),
                custody: perpetuals::PublicKey::new(vec![
                    103, 89, 93, 216, 70, 192, 7, 242, 104, 150, 242, 174, 211, 27, 167, 181, 95,
                    209, 44, 204, 21, 142, 11, 0, 122, 157, 143, 232, 70, 182, 159, 233,
                ]),
                custody_doves_price_account: perpetuals::PublicKey::new(vec![
                    216, 42, 235, 57, 188, 70, 146, 145, 46, 181, 242, 170, 224, 18, 127, 36, 173,
                    176, 246, 182, 107, 253, 118, 9, 80, 73, 48, 236, 108, 178, 99, 136,
                ]),
                collateral_custody: perpetuals::PublicKey::new(vec![
                    103, 89, 93, 216, 70, 192, 7, 242, 104, 150, 242, 174, 211, 27, 167, 181, 95,
                    209, 44, 204, 21, 142, 11, 0, 122, 157, 143, 232, 70, 182, 159, 233,
                ]),
                collateral_custody_doves_price_account: perpetuals::PublicKey::new(vec![
                    216, 42, 235, 57, 188, 70, 146, 145, 46, 181, 242, 170, 224, 18, 127, 36, 173,
                    176, 246, 182, 107, 253, 118, 9, 80, 73, 48, 236, 108, 178, 99, 136,
                ]),
                collateral_custody_token_account: perpetuals::PublicKey::new(vec![
                    155, 188, 50, 161, 141, 135, 28, 7, 53, 93, 210, 81, 97, 36, 21, 196, 32, 76,
                    171, 128, 29, 185, 238, 194, 146, 101, 3, 81, 177, 102, 210, 110,
                ]),
                token_program: perpetuals::PublicKey::new(vec![
                    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172,
                    28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
                ]),
                event_authority: perpetuals::PublicKey::new(vec![
                    31, 110, 107, 244, 132, 55, 71, 222, 35, 151, 202, 112, 75, 230, 84, 146, 147,
                    148, 134, 231, 7, 116, 227, 98, 240, 124, 71, 211, 219, 57, 210, 145,
                ]),
                program: perpetuals::PublicKey::new(perpetuals::PROGRAM_ID.to_vec()),
            }),
            args: Some(perpetuals::instruction::DecreasePositionWithTpslArgs {}),
        };

        assert_eq!(decrease, &expected);
    }

    {
        let close = ixs
            .iter()
            .find_map(|ix| match ix.as_ref()?.instruction.as_ref()? {
                perpetuals::instruction::Instruction::ClosePositionRequest2(s) => Some(s),
                _ => None,
            })
            .expect("no close position request 2 ix found");

        let expected = perpetuals::instruction::ClosePositionRequest2 {
            accounts: Some(perpetuals::instruction::ClosePositionRequest2Accounts {
                keeper: perpetuals::PublicKey::new(vec![
                    238, 103, 24, 154, 146, 36, 183, 11, 249, 126, 171, 22, 248, 91, 126, 66, 80,
                    130, 214, 35, 46, 153, 237, 255, 229, 32, 219, 75, 135, 121, 46, 21,
                ]),
                owner: perpetuals::PublicKey::new(vec![
                    138, 2, 154, 132, 201, 110, 60, 65, 124, 72, 228, 105, 180, 198, 154, 69, 177,
                    138, 239, 252, 160, 169, 123, 64, 90, 105, 103, 236, 9, 62, 69, 162,
                ]),
                owner_ata: perpetuals::PublicKey::new(vec![
                    154, 83, 171, 15, 56, 48, 208, 249, 99, 113, 152, 52, 252, 22, 248, 73, 105,
                    132, 234, 10, 31, 166, 147, 20, 195, 195, 240, 66, 213, 229, 199, 171,
                ]),
                pool: perpetuals::PublicKey::new(vec![
                    62, 30, 36, 115, 199, 52, 6, 84, 235, 135, 41, 0, 53, 21, 28, 64, 43, 208, 227,
                    201, 124, 180, 36, 72, 134, 231, 32, 52, 179, 11, 77, 252,
                ]),
                position_request: perpetuals::PublicKey::new(vec![
                    233, 115, 77, 143, 129, 244, 145, 190, 189, 103, 5, 29, 76, 62, 201, 162, 249,
                    94, 14, 84, 101, 136, 146, 56, 83, 158, 180, 120, 90, 69, 12, 176,
                ]),
                position_request_ata: perpetuals::PublicKey::new(vec![
                    29, 156, 57, 200, 92, 59, 244, 71, 93, 107, 142, 243, 97, 105, 117, 15, 130,
                    93, 162, 16, 233, 61, 59, 152, 138, 96, 254, 171, 26, 93, 233, 251,
                ]),
                position: perpetuals::PublicKey::new(vec![
                    21, 27, 48, 86, 199, 205, 70, 175, 49, 243, 96, 1, 44, 35, 84, 21, 63, 30, 153,
                    23, 190, 180, 203, 32, 246, 28, 255, 218, 234, 227, 11, 255,
                ]),
                mint: perpetuals::PublicKey::new(vec![
                    6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53, 218,
                    196, 57, 220, 26, 235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1,
                ]),
                token_program: perpetuals::PublicKey::new(vec![
                    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172,
                    28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
                ]),
                system_program: perpetuals::PublicKey::new(vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
                associated_token_program: perpetuals::PublicKey::new(vec![
                    140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142, 13, 131, 11, 90,
                    19, 153, 218, 255, 16, 132, 4, 142, 123, 216, 219, 233, 248, 89,
                ]),
                event_authority: perpetuals::PublicKey::new(vec![
                    31, 110, 107, 244, 132, 55, 71, 222, 35, 151, 202, 112, 75, 230, 84, 146, 147,
                    148, 134, 231, 7, 116, 227, 98, 240, 124, 71, 211, 219, 57, 210, 145,
                ]),
                program: perpetuals::PublicKey::new(perpetuals::PROGRAM_ID.to_vec()),
            }),
            args: Some(perpetuals::instruction::ClosePositionRequest2Args {}),
        };

        assert_eq!(close, &expected);
    }
}
