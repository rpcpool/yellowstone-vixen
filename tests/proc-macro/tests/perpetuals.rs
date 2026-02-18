mod common;

use insta;
use yellowstone_vixen_core::Parser;
use yellowstone_vixen_mock::tx_fixture;
use yellowstone_vixen_proc_macro::include_vixen_parser;

include_vixen_parser!("idls/perp_idl.json");

#[tokio::test]
async fn check_protobuf_schema() {
    common::check_protobuf_format(perpetuals::PROTOBUF_SCHEMA);

    insta::assert_snapshot!(perpetuals::PROTOBUF_SCHEMA);
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
                perpetuals::Instruction::DecreasePositionWithTpsl(s) => Some(s),
                _ => None,
            })
            .expect("no decrease position ix found");

        let expected = perpetuals::DecreasePositionWithTpslInstruction {
            accounts: Some(perpetuals::DecreasePositionWithTpslAccounts {
                keeper: vec![
                    238, 103, 24, 154, 146, 36, 183, 11, 249, 126, 171, 22, 248, 91, 126, 66, 80,
                    130, 214, 35, 46, 153, 237, 255, 229, 32, 219, 75, 135, 121, 46, 21,
                ],
                owner: vec![
                    138, 2, 154, 132, 201, 110, 60, 65, 124, 72, 228, 105, 180, 198, 154, 69, 177,
                    138, 239, 252, 160, 169, 123, 64, 90, 105, 103, 236, 9, 62, 69, 162,
                ],
                transfer_authority: vec![
                    141, 38, 83, 12, 155, 36, 127, 146, 136, 234, 206, 55, 84, 75, 38, 56, 128,
                    192, 44, 173, 4, 211, 33, 80, 237, 29, 1, 248, 251, 221, 35, 134,
                ],
                perpetuals: vec![
                    238, 151, 183, 0, 48, 24, 63, 163, 2, 12, 13, 6, 188, 207, 70, 162, 238, 235,
                    177, 159, 189, 77, 24, 177, 204, 63, 21, 61, 126, 170, 228, 30,
                ],
                pool: vec![
                    62, 30, 36, 115, 199, 52, 6, 84, 235, 135, 41, 0, 53, 21, 28, 64, 43, 208, 227,
                    201, 124, 180, 36, 72, 134, 231, 32, 52, 179, 11, 77, 252,
                ],
                position_request: vec![
                    233, 115, 77, 143, 129, 244, 145, 190, 189, 103, 5, 29, 76, 62, 201, 162, 249,
                    94, 14, 84, 101, 136, 146, 56, 83, 158, 180, 120, 90, 69, 12, 176,
                ],
                position_request_ata: vec![
                    29, 156, 57, 200, 92, 59, 244, 71, 93, 107, 142, 243, 97, 105, 117, 15, 130,
                    93, 162, 16, 233, 61, 59, 152, 138, 96, 254, 171, 26, 93, 233, 251,
                ],
                position: vec![
                    21, 27, 48, 86, 199, 205, 70, 175, 49, 243, 96, 1, 44, 35, 84, 21, 63, 30, 153,
                    23, 190, 180, 203, 32, 246, 28, 255, 218, 234, 227, 11, 255,
                ],
                custody: vec![
                    103, 89, 93, 216, 70, 192, 7, 242, 104, 150, 242, 174, 211, 27, 167, 181, 95,
                    209, 44, 204, 21, 142, 11, 0, 122, 157, 143, 232, 70, 182, 159, 233,
                ],
                custody_doves_price_account: vec![
                    216, 42, 235, 57, 188, 70, 146, 145, 46, 181, 242, 170, 224, 18, 127, 36, 173,
                    176, 246, 182, 107, 253, 118, 9, 80, 73, 48, 236, 108, 178, 99, 136,
                ],
                collateral_custody: vec![
                    103, 89, 93, 216, 70, 192, 7, 242, 104, 150, 242, 174, 211, 27, 167, 181, 95,
                    209, 44, 204, 21, 142, 11, 0, 122, 157, 143, 232, 70, 182, 159, 233,
                ],
                collateral_custody_doves_price_account: vec![
                    216, 42, 235, 57, 188, 70, 146, 145, 46, 181, 242, 170, 224, 18, 127, 36, 173,
                    176, 246, 182, 107, 253, 118, 9, 80, 73, 48, 236, 108, 178, 99, 136,
                ],
                collateral_custody_token_account: vec![
                    155, 188, 50, 161, 141, 135, 28, 7, 53, 93, 210, 81, 97, 36, 21, 196, 32, 76,
                    171, 128, 29, 185, 238, 194, 146, 101, 3, 81, 177, 102, 210, 110,
                ],
                token_program: vec![
                    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172,
                    28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
                ],
                event_authority: vec![
                    31, 110, 107, 244, 132, 55, 71, 222, 35, 151, 202, 112, 75, 230, 84, 146, 147,
                    148, 134, 231, 7, 116, 227, 98, 240, 124, 71, 211, 219, 57, 210, 145,
                ],
                program: perpetuals::PROGRAM_ID.to_vec(),
            }),
            args: Some(perpetuals::DecreasePositionWithTpslArgs {}),
        };

        assert_eq!(decrease, &expected);
    }

    {
        let close = ixs
            .iter()
            .find_map(|ix| match ix.as_ref()?.instruction.as_ref()? {
                perpetuals::Instruction::ClosePositionRequest2(s) => Some(s),
                _ => None,
            })
            .expect("no close position request 2 ix found");

        let expected = perpetuals::ClosePositionRequest2Instruction {
            accounts: Some(perpetuals::ClosePositionRequest2Accounts {
                keeper: vec![
                    238, 103, 24, 154, 146, 36, 183, 11, 249, 126, 171, 22, 248, 91, 126, 66, 80,
                    130, 214, 35, 46, 153, 237, 255, 229, 32, 219, 75, 135, 121, 46, 21,
                ],
                owner: vec![
                    138, 2, 154, 132, 201, 110, 60, 65, 124, 72, 228, 105, 180, 198, 154, 69, 177,
                    138, 239, 252, 160, 169, 123, 64, 90, 105, 103, 236, 9, 62, 69, 162,
                ],
                owner_ata: vec![
                    154, 83, 171, 15, 56, 48, 208, 249, 99, 113, 152, 52, 252, 22, 248, 73, 105,
                    132, 234, 10, 31, 166, 147, 20, 195, 195, 240, 66, 213, 229, 199, 171,
                ],
                pool: vec![
                    62, 30, 36, 115, 199, 52, 6, 84, 235, 135, 41, 0, 53, 21, 28, 64, 43, 208, 227,
                    201, 124, 180, 36, 72, 134, 231, 32, 52, 179, 11, 77, 252,
                ],
                position_request: vec![
                    233, 115, 77, 143, 129, 244, 145, 190, 189, 103, 5, 29, 76, 62, 201, 162, 249,
                    94, 14, 84, 101, 136, 146, 56, 83, 158, 180, 120, 90, 69, 12, 176,
                ],
                position_request_ata: vec![
                    29, 156, 57, 200, 92, 59, 244, 71, 93, 107, 142, 243, 97, 105, 117, 15, 130,
                    93, 162, 16, 233, 61, 59, 152, 138, 96, 254, 171, 26, 93, 233, 251,
                ],
                position: vec![
                    21, 27, 48, 86, 199, 205, 70, 175, 49, 243, 96, 1, 44, 35, 84, 21, 63, 30, 153,
                    23, 190, 180, 203, 32, 246, 28, 255, 218, 234, 227, 11, 255,
                ],
                mint: vec![
                    6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53, 218,
                    196, 57, 220, 26, 235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1,
                ],
                token_program: vec![
                    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172,
                    28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
                ],
                system_program: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ],
                associated_token_program: vec![
                    140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142, 13, 131, 11, 90,
                    19, 153, 218, 255, 16, 132, 4, 142, 123, 216, 219, 233, 248, 89,
                ],
                event_authority: vec![
                    31, 110, 107, 244, 132, 55, 71, 222, 35, 151, 202, 112, 75, 230, 84, 146, 147,
                    148, 134, 231, 7, 116, 227, 98, 240, 124, 71, 211, 219, 57, 210, 145,
                ],
                program: perpetuals::PROGRAM_ID.to_vec(),
            }),
            args: Some(perpetuals::ClosePositionRequest2Args {}),
        };

        assert_eq!(close, &expected);
    }
}
