mod common;

use prost::Message;
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
        perpetuals::account::Account::Custody(c) => c,
        _ => panic!("Unexpected account state"),
    };

    let expected = perpetuals::Custody {
        pool: perpetuals::Pubkey::new([
            62, 30, 36, 115, 199, 52, 6, 84, 235, 135, 41, 0, 53, 21, 28, 64, 43, 208, 227, 201,
            124, 180, 36, 72, 134, 231, 32, 52, 179, 11, 77, 252,
        ]),
        mint: perpetuals::Pubkey::new([
            6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53, 218, 196, 57,
            220, 26, 235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1,
        ]),
        token_account: perpetuals::Pubkey::new([
            155, 188, 50, 161, 141, 135, 28, 7, 53, 93, 210, 81, 97, 36, 21, 196, 32, 76, 171, 128,
            29, 185, 238, 194, 146, 101, 3, 81, 177, 102, 210, 110,
        ]),
        decimals: 9,
        is_stable: false,
        oracle: perpetuals::OracleParams {
            oracle_account: perpetuals::Pubkey::new([
                96, 49, 71, 4, 52, 13, 237, 223, 55, 31, 212, 36, 114, 20, 143, 36, 142, 157, 26,
                109, 26, 94, 178, 172, 58, 205, 139, 127, 213, 214, 178, 67,
            ]),
            oracle_type: perpetuals::OracleType {
                kind: perpetuals::oracle_type::Kind::Pyth(perpetuals::OracleTypePyth {}),
            },
            buffer: 50,
            max_price_age_sec: 5,
        },
        pricing: perpetuals::PricingParams {
            trade_impact_fee_scalar: 3_750_000_000_000_000,
            buffer: 0,
            swap_spread: 0,
            max_leverage: 5_000_000,
            max_global_long_sizes: 410_000_000_000_000,
            max_global_short_sizes: 112_293_947_657_720,
        },
        permissions: perpetuals::Permissions {
            allow_swap: true,
            allow_add_liquidity: true,
            allow_remove_liquidity: true,
            allow_increase_position: true,
            allow_decrease_position: true,
            allow_collateral_withdrawal: true,
            allow_liquidate_position: true,
        },
        target_ratio_bps: 4700,
        assets: perpetuals::Assets {
            fees_reserves: 466_896_904,
            owned: 6_368_829_939_225_389,
            locked: 398_908_550_717_247,
            guaranteed_usd: 24_953_150_512_329,
            global_short_sizes: 10_560_791_260_461,
            global_short_average_prices: 83_486_086,
        },
        funding_rate_state: perpetuals::FundingRateState {
            cumulative_interest_rate: u128::from_le_bytes([151, 179, 251, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            last_update: 1_771_834_760,
            hourly_funding_dbps: 0,
        },
        bump: 253,
        token_account_bump: 255,
        increase_position_bps: 6,
        decrease_position_bps: 6,
        max_position_size_usd: 10_000_000_000_000,
        doves_oracle: perpetuals::Pubkey::new([
            31, 236, 44, 187, 11, 175, 112, 110, 143, 105, 186, 138, 53, 218, 38, 247, 99, 111, 71,
            194, 119, 3, 29, 140, 62, 4, 86, 16, 146, 217, 76, 97,
        ]),
        jump_rate_state: perpetuals::JumpRateState {
            min_rate_bps: 1000,
            max_rate_bps: 15000,
            target_rate_bps: 3500,
            target_utilization_rate: 800_000_000,
        },
        doves_ag_oracle: perpetuals::Pubkey::new([
            216, 42, 235, 57, 188, 70, 146, 145, 46, 181, 242, 170, 224, 18, 127, 36, 173, 176,
            246, 182, 107, 253, 118, 9, 80, 73, 48, 236, 108, 178, 99, 136,
        ]),
        price_impact_buffer: perpetuals::PriceImpactBuffer {
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
        },
        borrow_lend_parameters: perpetuals::BorrowLendParams {
            borrows_limit_in_bps: 0,
            maintainance_margin_bps: 0,
            protocol_fee_bps: 0,
            liquidation_margin: 0,
            liquidation_fee_bps: 0,
        },
        borrows_funding_rate_state: perpetuals::FundingRateState {
            cumulative_interest_rate: u128::from_le_bytes([
                0, 0, 100, 167, 179, 182, 224, 13, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
            last_update: 1_771_834_760,
            hourly_funding_dbps: 0,
        },
        debt: 0u128,
        borrow_lend_interests_accured: 0u128,
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
        perpetuals::account::Account::Pool(pool) => pool,
        _ => panic!("Unexpected account state"),
    };

    let expected = perpetuals::Pool {
        name: "Pool".to_string(),
        custodies: vec![
            perpetuals::Pubkey::new([
                103, 89, 93, 216, 70, 192, 7, 242, 104, 150, 242, 174, 211, 27, 167, 181, 95, 209,
                44, 204, 21, 142, 11, 0, 122, 157, 143, 232, 70, 182, 159, 233,
            ]),
            perpetuals::Pubkey::new([
                139, 170, 74, 72, 100, 34, 108, 192, 34, 72, 77, 200, 40, 30, 26, 22, 143, 92, 244,
                232, 5, 63, 26, 229, 6, 109, 145, 106, 136, 185, 223, 159,
            ]),
            perpetuals::Pubkey::new([
                65, 77, 129, 72, 106, 241, 62, 110, 236, 158, 45, 91, 207, 69, 145, 50, 227, 164,
                102, 71, 9, 182, 109, 56, 208, 100, 119, 145, 36, 198, 206, 62,
            ]),
            perpetuals::Pubkey::new([
                222, 232, 11, 52, 19, 162, 211, 16, 128, 98, 107, 146, 108, 56, 175, 52, 190, 209,
                134, 219, 54, 142, 151, 127, 58, 191, 75, 213, 69, 68, 154, 109,
            ]),
            perpetuals::Pubkey::new([
                58, 87, 226, 203, 202, 208, 40, 131, 80, 35, 204, 171, 74, 216, 123, 210, 118, 78,
                143, 193, 50, 214, 142, 152, 102, 9, 235, 19, 75, 215, 134, 249,
            ]),
        ],
        aum_usd: u128::from_le_bytes([42, 148, 219, 152, 238, 6, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        limit: perpetuals::Limit {
            max_aum_usd: u128::from_le_bytes([0, 128, 83, 238, 123, 168, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            token_weightage_buffer_bps: u128::from_le_bytes([232, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            buffer: 0,
        },
        fees: perpetuals::Fees {
            swap_multiplier: 100,
            stable_swap_multiplier: 100,
            add_remove_liquidity_bps: 20,
            swap_bps: 10,
            tax_bps: 100,
            stable_swap_bps: 2,
            stable_swap_tax_bps: 5,
            liquidation_reward_bps: 150,
            protocol_share_bps: 2500,
        },
        pool_apr: perpetuals::PoolApr {
            last_updated: 1771002900,
            fee_apr_bps: 1093,
            realized_fee_usd: 1250427617755,
        },
        max_request_execution_sec: 45,
        bump: 252,
        lp_token_bump: 254,
        inception_time: 1689677832,
        parameter_update_oracle: perpetuals::Secp256k1Pubkey {
            prefix: 2,
            key: vec![
                81, 186, 187, 169, 123, 155, 81, 29, 179, 83, 117, 14, 44, 57, 230, 19, 66, 46,
                133, 49, 33, 164, 138, 181, 32, 132, 12, 0, 39, 52, 40, 92,
            ],
        },
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
        let (decrease_accounts, decrease_args) = ixs
            .iter()
            .find_map(|ix| match &ix.as_ref()?.instruction {
                perpetuals::instruction::Instruction::DecreasePositionWithTpsl {
                    accounts,
                    args,
                } => Some((accounts, args)),
                _ => None,
            })
            .expect("no decrease position ix found");

        let expected = perpetuals::instruction::DecreasePositionWithTpsl {
            accounts: perpetuals::instruction::DecreasePositionWithTpslAccounts {
                keeper: perpetuals::Pubkey::new([
                    238, 103, 24, 154, 146, 36, 183, 11, 249, 126, 171, 22, 248, 91, 126, 66, 80,
                    130, 214, 35, 46, 153, 237, 255, 229, 32, 219, 75, 135, 121, 46, 21,
                ]),
                owner: perpetuals::Pubkey::new([
                    138, 2, 154, 132, 201, 110, 60, 65, 124, 72, 228, 105, 180, 198, 154, 69, 177,
                    138, 239, 252, 160, 169, 123, 64, 90, 105, 103, 236, 9, 62, 69, 162,
                ]),
                transfer_authority: perpetuals::Pubkey::new([
                    141, 38, 83, 12, 155, 36, 127, 146, 136, 234, 206, 55, 84, 75, 38, 56, 128,
                    192, 44, 173, 4, 211, 33, 80, 237, 29, 1, 248, 251, 221, 35, 134,
                ]),
                perpetuals: perpetuals::Pubkey::new([
                    238, 151, 183, 0, 48, 24, 63, 163, 2, 12, 13, 6, 188, 207, 70, 162, 238, 235,
                    177, 159, 189, 77, 24, 177, 204, 63, 21, 61, 126, 170, 228, 30,
                ]),
                pool: perpetuals::Pubkey::new([
                    62, 30, 36, 115, 199, 52, 6, 84, 235, 135, 41, 0, 53, 21, 28, 64, 43, 208, 227,
                    201, 124, 180, 36, 72, 134, 231, 32, 52, 179, 11, 77, 252,
                ]),
                position_request: perpetuals::Pubkey::new([
                    233, 115, 77, 143, 129, 244, 145, 190, 189, 103, 5, 29, 76, 62, 201, 162, 249,
                    94, 14, 84, 101, 136, 146, 56, 83, 158, 180, 120, 90, 69, 12, 176,
                ]),
                position_request_ata: perpetuals::Pubkey::new([
                    29, 156, 57, 200, 92, 59, 244, 71, 93, 107, 142, 243, 97, 105, 117, 15, 130,
                    93, 162, 16, 233, 61, 59, 152, 138, 96, 254, 171, 26, 93, 233, 251,
                ]),
                position: perpetuals::Pubkey::new([
                    21, 27, 48, 86, 199, 205, 70, 175, 49, 243, 96, 1, 44, 35, 84, 21, 63, 30, 153,
                    23, 190, 180, 203, 32, 246, 28, 255, 218, 234, 227, 11, 255,
                ]),
                custody: perpetuals::Pubkey::new([
                    103, 89, 93, 216, 70, 192, 7, 242, 104, 150, 242, 174, 211, 27, 167, 181, 95,
                    209, 44, 204, 21, 142, 11, 0, 122, 157, 143, 232, 70, 182, 159, 233,
                ]),
                custody_doves_price_account: perpetuals::Pubkey::new([
                    216, 42, 235, 57, 188, 70, 146, 145, 46, 181, 242, 170, 224, 18, 127, 36, 173,
                    176, 246, 182, 107, 253, 118, 9, 80, 73, 48, 236, 108, 178, 99, 136,
                ]),
                collateral_custody: perpetuals::Pubkey::new([
                    103, 89, 93, 216, 70, 192, 7, 242, 104, 150, 242, 174, 211, 27, 167, 181, 95,
                    209, 44, 204, 21, 142, 11, 0, 122, 157, 143, 232, 70, 182, 159, 233,
                ]),
                collateral_custody_doves_price_account: perpetuals::Pubkey::new([
                    216, 42, 235, 57, 188, 70, 146, 145, 46, 181, 242, 170, 224, 18, 127, 36, 173,
                    176, 246, 182, 107, 253, 118, 9, 80, 73, 48, 236, 108, 178, 99, 136,
                ]),
                collateral_custody_token_account: perpetuals::Pubkey::new([
                    155, 188, 50, 161, 141, 135, 28, 7, 53, 93, 210, 81, 97, 36, 21, 196, 32, 76,
                    171, 128, 29, 185, 238, 194, 146, 101, 3, 81, 177, 102, 210, 110,
                ]),
                token_program: perpetuals::Pubkey::new([
                    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172,
                    28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
                ]),
                event_authority: perpetuals::Pubkey::new([
                    31, 110, 107, 244, 132, 55, 71, 222, 35, 151, 202, 112, 75, 230, 84, 146, 147,
                    148, 134, 231, 7, 116, 227, 98, 240, 124, 71, 211, 219, 57, 210, 145,
                ]),
                program: perpetuals::Pubkey::new(perpetuals::PROGRAM_ID),
                remaining_accounts: vec![],
            },
            args: perpetuals::instruction::DecreasePositionWithTpslArgs {},
        };

        assert_eq!(decrease_accounts, &expected.accounts);
        assert_eq!(decrease_args, &expected.args);
    }

    {
        let (close_accounts, close_args) = ixs
            .iter()
            .find_map(|ix| match &ix.as_ref()?.instruction {
                perpetuals::instruction::Instruction::ClosePositionRequest2 { accounts, args } => {
                    Some((accounts, args))
                },
                _ => None,
            })
            .expect("no close position request 2 ix found");

        let expected = perpetuals::instruction::ClosePositionRequest2 {
            accounts: perpetuals::instruction::ClosePositionRequest2Accounts {
                keeper: perpetuals::Pubkey::new([
                    238, 103, 24, 154, 146, 36, 183, 11, 249, 126, 171, 22, 248, 91, 126, 66, 80,
                    130, 214, 35, 46, 153, 237, 255, 229, 32, 219, 75, 135, 121, 46, 21,
                ]),
                owner: perpetuals::Pubkey::new([
                    138, 2, 154, 132, 201, 110, 60, 65, 124, 72, 228, 105, 180, 198, 154, 69, 177,
                    138, 239, 252, 160, 169, 123, 64, 90, 105, 103, 236, 9, 62, 69, 162,
                ]),
                owner_ata: perpetuals::Pubkey::new([
                    154, 83, 171, 15, 56, 48, 208, 249, 99, 113, 152, 52, 252, 22, 248, 73, 105,
                    132, 234, 10, 31, 166, 147, 20, 195, 195, 240, 66, 213, 229, 199, 171,
                ]),
                pool: perpetuals::Pubkey::new([
                    62, 30, 36, 115, 199, 52, 6, 84, 235, 135, 41, 0, 53, 21, 28, 64, 43, 208, 227,
                    201, 124, 180, 36, 72, 134, 231, 32, 52, 179, 11, 77, 252,
                ]),
                position_request: perpetuals::Pubkey::new([
                    233, 115, 77, 143, 129, 244, 145, 190, 189, 103, 5, 29, 76, 62, 201, 162, 249,
                    94, 14, 84, 101, 136, 146, 56, 83, 158, 180, 120, 90, 69, 12, 176,
                ]),
                position_request_ata: perpetuals::Pubkey::new([
                    29, 156, 57, 200, 92, 59, 244, 71, 93, 107, 142, 243, 97, 105, 117, 15, 130,
                    93, 162, 16, 233, 61, 59, 152, 138, 96, 254, 171, 26, 93, 233, 251,
                ]),
                position: perpetuals::Pubkey::new([
                    21, 27, 48, 86, 199, 205, 70, 175, 49, 243, 96, 1, 44, 35, 84, 21, 63, 30, 153,
                    23, 190, 180, 203, 32, 246, 28, 255, 218, 234, 227, 11, 255,
                ]),
                mint: perpetuals::Pubkey::new([
                    6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53, 218,
                    196, 57, 220, 26, 235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1,
                ]),
                token_program: perpetuals::Pubkey::new([
                    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172,
                    28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
                ]),
                system_program: perpetuals::Pubkey::new([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
                associated_token_program: perpetuals::Pubkey::new([
                    140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142, 13, 131, 11, 90,
                    19, 153, 218, 255, 16, 132, 4, 142, 123, 216, 219, 233, 248, 89,
                ]),
                event_authority: perpetuals::Pubkey::new([
                    31, 110, 107, 244, 132, 55, 71, 222, 35, 151, 202, 112, 75, 230, 84, 146, 147,
                    148, 134, 231, 7, 116, 227, 98, 240, 124, 71, 211, 219, 57, 210, 145,
                ]),
                program: perpetuals::Pubkey::new(perpetuals::PROGRAM_ID),
                remaining_accounts: vec![],
            },
            args: perpetuals::instruction::ClosePositionRequest2Args {},
        };

        assert_eq!(close_accounts, &expected.accounts);
        assert_eq!(close_args, &expected.args);
    }
}

#[tokio::test]
async fn parse_borrow_from_custody_ix() {
    let parser = perpetuals::InstructionParser;

    let ixs = tx_fixture!(
        "5mYEUYXCZisS8CChCG8mL8N3NEWHUA81Rr7kLA28P5upSzDStLq1f4QKhFLY7R8GsRNB27gM6YzvKerxejtLQxCj",
        &parser
    );

    let (borrow_accounts, borrow_args) = ixs
        .iter()
        .find_map(|ix| match &ix.as_ref()?.instruction {
            perpetuals::instruction::Instruction::BorrowFromCustody { accounts, args } => {
                Some((accounts, args))
            },
            _ => None,
        })
        .expect("no borrow from custody ix found");

    use perpetuals::Pubkey;

    let expected = perpetuals::instruction::BorrowFromCustody {
        accounts: perpetuals::instruction::BorrowFromCustodyAccounts {
            owner: Pubkey::new([
                193, 141, 200, 224, 246, 201, 5, 150, 208, 94, 178, 61, 237, 45, 230, 117, 221,
                127, 66, 219, 18, 153, 140, 155, 9, 12, 15, 17, 113, 249, 167, 29,
            ]),
            perpetuals: Pubkey::new([
                238, 151, 183, 0, 48, 24, 63, 163, 2, 12, 13, 6, 188, 207, 70, 162, 238, 235, 177,
                159, 189, 77, 24, 177, 204, 63, 21, 61, 126, 170, 228, 30,
            ]),
            pool: Pubkey::new([
                62, 30, 36, 115, 199, 52, 6, 84, 235, 135, 41, 0, 53, 21, 28, 64, 43, 208, 227,
                201, 124, 180, 36, 72, 134, 231, 32, 52, 179, 11, 77, 252,
            ]),
            custody: Pubkey::new([
                222, 232, 11, 52, 19, 162, 211, 16, 128, 98, 107, 146, 108, 56, 175, 52, 190, 209,
                134, 219, 54, 142, 151, 127, 58, 191, 75, 213, 69, 68, 154, 109,
            ]),
            transfer_authority: Pubkey::new([
                141, 38, 83, 12, 155, 36, 127, 146, 136, 234, 206, 55, 84, 75, 38, 56, 128, 192,
                44, 173, 4, 211, 33, 80, 237, 29, 1, 248, 251, 221, 35, 134,
            ]),
            borrow_position: Pubkey::new([
                10, 160, 117, 35, 217, 119, 94, 32, 82, 76, 97, 73, 104, 24, 124, 173, 193, 25, 42,
                175, 214, 77, 58, 21, 186, 216, 174, 34, 147, 19, 160, 188,
            ]),
            custody_token_account: Pubkey::new([
                7, 174, 222, 70, 130, 196, 60, 168, 189, 35, 161, 245, 194, 169, 200, 148, 174,
                246, 253, 87, 63, 45, 82, 20, 43, 171, 167, 194, 196, 86, 70, 241,
            ]),
            user_token_account: Pubkey::new([
                147, 87, 35, 84, 159, 192, 21, 171, 32, 132, 213, 96, 177, 150, 71, 166, 115, 202,
                122, 55, 70, 4, 210, 126, 106, 102, 181, 28, 158, 43, 25, 180,
            ]),
            lp_token_mint: Pubkey::new([
                16, 118, 70, 156, 16, 65, 217, 233, 179, 159, 194, 237, 225, 19, 51, 151, 59, 62,
                149, 115, 42, 68, 57, 32, 113, 147, 166, 28, 196, 16, 141, 67,
            ]),
            token_program: Pubkey::new([
                6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28,
                180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
            ]),
            event_authority: Pubkey::new([
                31, 110, 107, 244, 132, 55, 71, 222, 35, 151, 202, 112, 75, 230, 84, 146, 147, 148,
                134, 231, 7, 116, 227, 98, 240, 124, 71, 211, 219, 57, 210, 145,
            ]),
            program: Pubkey::new([
                5, 177, 243, 202, 241, 148, 98, 239, 135, 96, 240, 171, 222, 117, 205, 61, 158,
                227, 27, 58, 50, 198, 32, 232, 148, 18, 46, 156, 155, 129, 69, 250,
            ]),
            remaining_accounts: vec![
                Pubkey::new([
                    103, 89, 93, 216, 70, 192, 7, 242, 104, 150, 242, 174, 211, 27, 167, 181, 95,
                    209, 44, 204, 21, 142, 11, 0, 122, 157, 143, 232, 70, 182, 159, 233,
                ]),
                Pubkey::new([
                    139, 170, 74, 72, 100, 34, 108, 192, 34, 72, 77, 200, 40, 30, 26, 22, 143, 92,
                    244, 232, 5, 63, 26, 229, 6, 109, 145, 106, 136, 185, 223, 159,
                ]),
                Pubkey::new([
                    65, 77, 129, 72, 106, 241, 62, 110, 236, 158, 45, 91, 207, 69, 145, 50, 227,
                    164, 102, 71, 9, 182, 109, 56, 208, 100, 119, 145, 36, 198, 206, 62,
                ]),
                Pubkey::new([
                    222, 232, 11, 52, 19, 162, 211, 16, 128, 98, 107, 146, 108, 56, 175, 52, 190,
                    209, 134, 219, 54, 142, 151, 127, 58, 191, 75, 213, 69, 68, 154, 109,
                ]),
                Pubkey::new([
                    58, 87, 226, 203, 202, 208, 40, 131, 80, 35, 204, 171, 74, 216, 123, 210, 118,
                    78, 143, 193, 50, 214, 142, 152, 102, 9, 235, 19, 75, 215, 134, 249,
                ]),
                Pubkey::new([
                    216, 42, 235, 57, 188, 70, 146, 145, 46, 181, 242, 170, 224, 18, 127, 36, 173,
                    176, 246, 182, 107, 253, 118, 9, 80, 73, 48, 236, 108, 178, 99, 136,
                ]),
                Pubkey::new([
                    137, 116, 97, 3, 147, 40, 78, 117, 140, 2, 181, 4, 182, 212, 160, 110, 102, 27,
                    143, 113, 202, 107, 86, 209, 96, 133, 161, 98, 37, 143, 145, 100,
                ]),
                Pubkey::new([
                    10, 94, 179, 70, 66, 54, 72, 77, 64, 8, 213, 215, 209, 243, 166, 61, 96, 115,
                    62, 141, 88, 106, 109, 125, 75, 165, 76, 227, 23, 48, 183, 59,
                ]),
                Pubkey::new([
                    78, 218, 125, 88, 17, 150, 201, 61, 3, 86, 34, 155, 16, 119, 115, 97, 205, 50,
                    105, 36, 150, 113, 51, 95, 56, 96, 94, 195, 58, 201, 132, 30,
                ]),
                Pubkey::new([
                    218, 40, 255, 246, 50, 152, 165, 14, 79, 147, 195, 1, 198, 238, 170, 100, 214,
                    237, 61, 60, 80, 25, 129, 231, 40, 254, 0, 125, 174, 143, 186, 20,
                ]),
            ],
        },
        args: perpetuals::instruction::BorrowFromCustodyArgs {
            amount: 8_463_144_037,
        },
    };

    assert_eq!(borrow_accounts, &expected.accounts);
    assert_eq!(borrow_args, &expected.args);
}

///
/// Proto encode → decode round-trip for structs containing native u8 fields.
///
/// Verifies that the manual `prost::Message` impl correctly widens u8→u32
/// on encode and narrows u32→u8 on decode, preserving the original values.
///
#[tokio::test]
async fn proto_round_trip_native_u8_fields() {
    let parser = perpetuals::AccountParser;
    let account = account_fixture!("7xS2gz2bTp3fwCC7knJvUWTEU9Tycczu6VhJYKgi1wdz", &parser);

    let custody = match account.account {
        perpetuals::account::Account::Custody(c) => c,
        _ => panic!("Unexpected account state"),
    };

    // Verify native u8 fields have their original values before round-trip.
    assert_eq!(custody.decimals, 9u8);
    assert_eq!(custody.bump, 253u8);
    assert_eq!(custody.token_account_bump, 255u8);

    // Encode to proto bytes.
    let mut buf = Vec::new();

    custody.encode(&mut buf).expect("proto encode failed");

    // Decode back from proto bytes.
    let decoded = perpetuals::Custody::decode(buf.as_slice()).expect("proto decode failed");

    // All fields must survive the round-trip, including the u8 ones.
    assert_eq!(decoded, custody);

    // Double-check the u8 fields explicitly — 253 and 255 exercise the
    // high end of the range where truncation bugs would show.
    assert_eq!(decoded.decimals, 9u8);
    assert_eq!(decoded.bump, 253u8);
    assert_eq!(decoded.token_account_bump, 255u8);
}

///
/// Proto encode → decode round-trip for instruction dispatch (oneof wrapper).
///
/// The `Instructions` struct uses a manual `prost::Message` impl that delegates
/// to the manual `prost::Oneof`-style impl on the enum. This test verifies the
/// full chain works correctly.
///
#[tokio::test]
async fn proto_round_trip_instruction() {
    let parser = perpetuals::InstructionParser;

    let ixs = tx_fixture!(
        "5mYEUYXCZisS8CChCG8mL8N3NEWHUA81Rr7kLA28P5upSzDStLq1f4QKhFLY7R8GsRNB27gM6YzvKerxejtLQxCj",
        &parser
    );

    let original = ixs
        .iter()
        .find_map(|ix| ix.as_ref())
        .expect("no instruction found");

    // Encode to proto bytes.
    let mut buf = Vec::new();

    original.encode(&mut buf).expect("proto encode failed");

    assert!(!buf.is_empty(), "encoded bytes should not be empty");

    // Verify encoded_len matches actual output size.
    assert_eq!(original.encoded_len(), buf.len());
}
