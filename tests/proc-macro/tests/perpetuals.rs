mod common;

use common::{pubkey, pubkey_bytes};
use yellowstone_vixen_core::Parser;
use yellowstone_vixen_mock::{account_fixture, tx_fixture};
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("idls/perpetuals.json");

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
        pool: pubkey("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq"),
        mint: pubkey("So11111111111111111111111111111111111111112"),
        token_account: pubkey("BUvduFTd2sWFagCunBPLupG8fBTJqweLw9DuhruNFSCm"),
        decimals: 9,
        is_stable: false,
        oracle: perpetuals::OracleParams {
            oracle_account: pubkey("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE"),
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
            cumulative_interest_rate: vec![151, 179, 251, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            last_update: 1_771_834_760,
            hourly_funding_dbps: 0,
        },
        bump: 253,
        token_account_bump: 255,
        increase_position_bps: 6,
        decrease_position_bps: 6,
        max_position_size_usd: 10_000_000_000_000,
        doves_oracle: pubkey("39cWjvHrpHNz2SbXv6ME4NPhqBDBd4KsjUYv5JkHEAJU"),
        jump_rate_state: perpetuals::JumpRateState {
            min_rate_bps: 1000,
            max_rate_bps: 15000,
            target_rate_bps: 3500,
            target_utilization_rate: 800_000_000,
        },
        doves_ag_oracle: pubkey("FYq2BWQ1V5P1WFBqr3qB2Kb5yHVvSv7upzKodgQE5zXh"),
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
            cumulative_interest_rate: vec![
                0, 0, 100, 167, 179, 182, 224, 13, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            last_update: 1_771_834_760,
            hourly_funding_dbps: 0,
        },
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
        perpetuals::account::Account::Pool(pool) => pool,
        _ => panic!("Unexpected account state"),
    };

    let expected = perpetuals::Pool {
        name: "Pool".to_string(),
        custodies: vec![
            pubkey("7xS2gz2bTp3fwCC7knJvUWTEU9Tycczu6VhJYKgi1wdz"),
            pubkey("AQCGyheWPLeo6Qp9WpYS9m3Qj479t7R636N9ey1rEjEn"),
            pubkey("5Pv3gM9JrFFH883SWAhvJC9RPYmo8UNxuFtv5bMMALkm"),
            pubkey("G18jKKXQwBbrHeiK3C9MRXhkHsLHf7XgCSisykV46EZa"),
            pubkey("4vkNeXiYEUizLdrpdPS1eC2mccyM4NUPRtERrk6ZETkk"),
        ],
        aum_usd: vec![42, 148, 219, 152, 238, 6, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        limit: perpetuals::Limit {
            max_aum_usd: vec![0, 128, 83, 238, 123, 168, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            token_weightage_buffer_bps: vec![232, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
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
    let parser = perpetuals::InstructionParser.with_raw_logs();

    let ixs = tx_fixture!(
        "3MnCnWjL83RBYGe8ADwhxFHTvbXmDEPyi2mxHQ2211kmQ4NAqBFvhjPexgiCy8o6mGPztnZwjbcxRi4WMGZDeah8",
        &parser
    );

    {
        let decrease_ix = ixs
            .iter()
            .find_map(|ix| {
                let ix = ix.as_ref()?;
                matches!(
                    &ix.instruction,
                    perpetuals::instruction::Instruction::DecreasePositionWithTpsl { .. }
                )
                .then_some(ix)
            })
            .expect("no decrease position ix found");

        let expected = perpetuals::Instructions {
            instruction: perpetuals::instruction::Instruction::DecreasePositionWithTpsl {
                accounts: perpetuals::instruction::DecreasePositionWithTpslAccounts {
                    keeper: pubkey("H3dDE6K6uqBp5kBKctH6w8hzpV5eAmxUiqvcdHxuE9i4"),
                    owner: pubkey("AHjZmPJ3swFKz6nVhoeV3Y2zB94rd2QFbAoEPvNo6DXX"),
                    transfer_authority: pubkey("AVzP2GeRmqGphJsMxWoqjpUifPpCret7LqWhD8NWQK49"),
                    perpetuals: pubkey("H4ND9aYttUVLFmNypZqLjZ52FYiGvdEB45GmwNoKEjTj"),
                    pool: pubkey("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq"),
                    position_request: pubkey("GiHy9ixvdkzGQVoTVXkppg7FCMxx1gpwnB7Df9HEUuBM"),
                    position_request_ata: pubkey("2zazRXFzzDHUJF4UBuGcmp3wbUh7zuhpjEuEHG1APisc"),
                    position: pubkey("2RPcEvVpLwVVxAPYn89np27pTC6keUGQ1Yf77qa7i1ok"),
                    custody: pubkey("7xS2gz2bTp3fwCC7knJvUWTEU9Tycczu6VhJYKgi1wdz"),
                    custody_doves_price_account: pubkey(
                        "FYq2BWQ1V5P1WFBqr3qB2Kb5yHVvSv7upzKodgQE5zXh",
                    ),
                    collateral_custody: pubkey("7xS2gz2bTp3fwCC7knJvUWTEU9Tycczu6VhJYKgi1wdz"),
                    collateral_custody_doves_price_account: pubkey(
                        "FYq2BWQ1V5P1WFBqr3qB2Kb5yHVvSv7upzKodgQE5zXh",
                    ),
                    collateral_custody_token_account: pubkey(
                        "BUvduFTd2sWFagCunBPLupG8fBTJqweLw9DuhruNFSCm",
                    ),
                    token_program: pubkey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                    event_authority: pubkey("37hJBDnntwqhGbK7L6M1bLyvccj4u55CCUiLPdYkiqBN"),
                    program: pubkey_bytes(&perpetuals::PROGRAM_ID),
                    remaining_accounts: vec![],
                },
                args: perpetuals::instruction::DecreasePositionWithTpslArgs {},
            },
            raw_logs: vec![
                "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu invoke [1]".into(),
                "Program log: Instruction: DecreasePositionWithTpsl".into(),
                "Program log: Check permissions".into(),
                "Program log: doves ag price: 8464693189, publish time: 1770650409".into(),
                "Program log: doves ag price: 8464693189, publish time: 1770650409".into(),
                "Program log: Exit price: 84646931".into(),
                "Program log: Trigger order price: 84650000".into(),
                "Program log: has_profit: true, pnl_delta: 5832000".into(),
                "Program log: Collected fee: 2323578".into(),
                "Program log: Transfer tokens".into(),
                "Program log: Amount out: 1012455751".into(),
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]".into(),
                "Program log: Instruction: Transfer".into(),
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4736 of 134675 \
                 compute units"
                    .into(),
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".into(),
                "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu invoke [2]".into(),
                "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu consumed 5134 of 123856 \
                 compute units"
                    .into(),
                "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu success".into(),
                "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu consumed 104302 of 202306 \
                 compute units"
                    .into(),
                "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu success".into(),
            ],
            anchor_log_events: vec![],
        };

        assert_eq!(decrease_ix, &expected);
    }

    {
        let close_ix = ixs
            .iter()
            .find_map(|ix| {
                let ix = ix.as_ref()?;
                matches!(
                    &ix.instruction,
                    perpetuals::instruction::Instruction::ClosePositionRequest2 { .. }
                )
                .then_some(ix)
            })
            .expect("no close position request 2 ix found");

        let expected = perpetuals::Instructions {
            instruction: perpetuals::instruction::Instruction::ClosePositionRequest2 {
                accounts: perpetuals::instruction::ClosePositionRequest2Accounts {
                    keeper: pubkey("H3dDE6K6uqBp5kBKctH6w8hzpV5eAmxUiqvcdHxuE9i4"),
                    owner: pubkey("AHjZmPJ3swFKz6nVhoeV3Y2zB94rd2QFbAoEPvNo6DXX"),
                    owner_ata: pubkey("BPRnMmfxF7QYepU9ypvBMA4eXwbHGST79bUgKvoZ6Jsk"),
                    pool: pubkey("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq"),
                    position_request: pubkey("GiHy9ixvdkzGQVoTVXkppg7FCMxx1gpwnB7Df9HEUuBM"),
                    position_request_ata: pubkey("2zazRXFzzDHUJF4UBuGcmp3wbUh7zuhpjEuEHG1APisc"),
                    position: pubkey("2RPcEvVpLwVVxAPYn89np27pTC6keUGQ1Yf77qa7i1ok"),
                    mint: pubkey("So11111111111111111111111111111111111111112"),
                    token_program: pubkey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                    system_program: pubkey("11111111111111111111111111111111"),
                    associated_token_program: pubkey(
                        "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
                    ),
                    event_authority: pubkey("37hJBDnntwqhGbK7L6M1bLyvccj4u55CCUiLPdYkiqBN"),
                    program: pubkey_bytes(&perpetuals::PROGRAM_ID),
                    remaining_accounts: vec![],
                },
                args: perpetuals::instruction::ClosePositionRequest2Args {},
            },
            raw_logs: vec![
                "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu invoke [1]".into(),
                "Program log: Instruction: ClosePositionRequest2".into(),
                "Program log: Check permissions".into(),
                "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu invoke [2]".into(),
                "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu consumed 5134 of 70488 \
                 compute units"
                    .into(),
                "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu success".into(),
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]".into(),
                "Program log: Instruction: CloseAccount".into(),
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 3014 of 62491 \
                 compute units"
                    .into(),
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".into(),
                "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu consumed 41954 of 98004 \
                 compute units"
                    .into(),
                "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu success".into(),
            ],
            anchor_log_events: vec![],
        };

        assert_eq!(close_ix, &expected);
    }
}

#[tokio::test]
async fn parse_borrow_from_custody_ix() {
    let parser = perpetuals::InstructionParser.with_raw_logs();

    let ixs = tx_fixture!(
        "5mYEUYXCZisS8CChCG8mL8N3NEWHUA81Rr7kLA28P5upSzDStLq1f4QKhFLY7R8GsRNB27gM6YzvKerxejtLQxCj",
        &parser
    );

    let borrow_ix = ixs
        .iter()
        .find_map(|ix| {
            let ix = ix.as_ref()?;
            matches!(
                &ix.instruction,
                perpetuals::instruction::Instruction::BorrowFromCustody { .. }
            )
            .then_some(ix)
        })
        .expect("no borrow from custody ix found");

    let expected = perpetuals::Instructions {
        instruction: perpetuals::instruction::Instruction::BorrowFromCustody {
            accounts: perpetuals::instruction::BorrowFromCustodyAccounts {
                owner: pubkey("E2Z5ggFhABjC5tSZYouMgfgUpgNsvDpWrR6YTFt7D4YC"),
                perpetuals: pubkey("H4ND9aYttUVLFmNypZqLjZ52FYiGvdEB45GmwNoKEjTj"),
                pool: pubkey("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq"),
                custody: pubkey("G18jKKXQwBbrHeiK3C9MRXhkHsLHf7XgCSisykV46EZa"),
                transfer_authority: pubkey("AVzP2GeRmqGphJsMxWoqjpUifPpCret7LqWhD8NWQK49"),
                borrow_position: pubkey("iUzDVme5Mc21GdULKK2JFuvjNWY4TaULF2kNGTcoXf9"),
                custody_token_account: pubkey("WzWUoCmtVv7eqAbU3BfKPU3fhLP6CXR8NCJH78UK9VS"),
                user_token_account: pubkey("Av9zpU3ZtdfYMHdW9ombpaarS9bQhsK4Rxvt6piUTkmH"),
                lp_token_mint: pubkey("27G8MtK7VtTcCHkpASjSDdkWWYfoqT6ggEuKidVJidD4"),
                token_program: pubkey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                event_authority: pubkey("37hJBDnntwqhGbK7L6M1bLyvccj4u55CCUiLPdYkiqBN"),
                program: pubkey("PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu"),
                remaining_accounts: vec![
                    pubkey("7xS2gz2bTp3fwCC7knJvUWTEU9Tycczu6VhJYKgi1wdz"),
                    pubkey("AQCGyheWPLeo6Qp9WpYS9m3Qj479t7R636N9ey1rEjEn"),
                    pubkey("5Pv3gM9JrFFH883SWAhvJC9RPYmo8UNxuFtv5bMMALkm"),
                    pubkey("G18jKKXQwBbrHeiK3C9MRXhkHsLHf7XgCSisykV46EZa"),
                    pubkey("4vkNeXiYEUizLdrpdPS1eC2mccyM4NUPRtERrk6ZETkk"),
                    pubkey("FYq2BWQ1V5P1WFBqr3qB2Kb5yHVvSv7upzKodgQE5zXh"),
                    pubkey("AFZnHPzy4mvVCffrVwhewHbFc93uTHvDSFrVH7GtfXF1"),
                    pubkey("hUqAT1KQ7eW1i6Csp9CXYtpPfSAvi835V7wKi5fRfmC"),
                    pubkey("6Jp2xZUTWdDD2ZyUPRzeMdc6AFQ5K3pFgZxk2EijfjnM"),
                    pubkey("Fgc93D641F8N2d1xLjQ4jmShuD3GE3BsCXA56KBQbF5u"),
                ],
            },
            args: perpetuals::instruction::BorrowFromCustodyArgs {
                amount: 8_463_144_037,
            },
        },
        raw_logs: vec![
            "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu invoke [1]".into(),
            "Program log: Instruction: BorrowFromCustody".into(),
            "Program log: Borrow from custody at 1772150417".into(),
            "Program log: doves ag price: 8585898945, publish time: 1772150415".into(),
            "Program log: doves ag price: 202710309285, publish time: 1772150416".into(),
            "Program log: doves ag price: 6746042743950, publish time: 1772150415".into(),
            "Program log: doves ag price: 99988000, publish time: 1772150415".into(),
            "Program log: doves ag price: 99993000, publish time: 1772150415".into(),
            "Program log: doves ag price: 99988000, publish time: 1772150415".into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]".into(),
            "Program log: Instruction: Transfer".into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4690 of 305138 compute \
             units"
                .into(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".into(),
            "Program log: new amount borrowed in usd: 6813955776162".into(),
            "Program log: total borrow amount usd:6813955776162 collateral amount \
             usd:11148464949892 maintainance margin usd: 10033618454902"
                .into(),
            "Program log: doves ag price: 8585898945, publish time: 1772150415".into(),
            "Program log: doves ag price: 202710309285, publish time: 1772150416".into(),
            "Program log: doves ag price: 6746042743950, publish time: 1772150415".into(),
            "Program log: doves ag price: 99988000, publish time: 1772150415".into(),
            "Program log: doves ag price: 99993000, publish time: 1772150415".into(),
            "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu invoke [2]".into(),
            "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu consumed 5134 of 253644 compute \
             units"
                .into(),
            "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu success".into(),
            "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu consumed 155036 of 393986 \
             compute units"
                .into(),
            "Program PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu success".into(),
        ],
        anchor_log_events: vec![],
    };

    assert_eq!(borrow_ix, &expected);

    // The same transaction also contains a CPI event instruction (borrowFromCustodyEvent).
    {
        let event_args = ixs
            .iter()
            .find_map(|ix| match &ix.as_ref()?.instruction {
                perpetuals::instruction::Instruction::BorrowFromCustodyEvent {
                    accounts: _,
                    args,
                } => Some(args),
                _ => None,
            })
            .expect("no borrow from custody event ix found");

        let expected_event_args = perpetuals::instruction::BorrowFromCustodyEventArgs {
            owner: pubkey("E2Z5ggFhABjC5tSZYouMgfgUpgNsvDpWrR6YTFt7D4YC"),
            pool: pubkey("5BUwFW4nRbftYTDMbgxykoFWqWHPzahFSNAaaaJtVKsq"),
            position_key: pubkey("iUzDVme5Mc21GdULKK2JFuvjNWY4TaULF2kNGTcoXf9"),
            position_mint: pubkey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
            position_custody: pubkey("G18jKKXQwBbrHeiK3C9MRXhkHsLHf7XgCSisykV46EZa"),
            size_custody_token: 8_463_144_037,
            collateral_amount: 3_000_000_000_000,
            collateral_amount_usd: 11_148_464_949_892,
            margin_usd: 10_033_618_454_902,
            update_time: 1_772_150_417,
        };

        assert_eq!(event_args, &expected_event_args);
    }
}
