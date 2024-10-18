use std::borrow::Cow;

use borsh::BorshDeserialize;
use yellowstone_vixen_core::{
    instruction::InstructionUpdate, ParseError, ParseResult, Parser, ProgramParser,
};

use super::instruction_helpers::{
    OrcaProgramIx, SwapAccounts, SwapIxData, SwapV2Accounts, SwapV2IxData, SWAP_IX_DISC,
    SWAP_V2_IX_DISC,
};
use crate::helpers::{check_min_accounts_req, IX_DISCRIMINATOR_SIZE};

#[derive(Debug, Clone, Copy)]
pub struct InstructionParser;

impl Parser for InstructionParser {
    type Input = InstructionUpdate;
    type Output = OrcaProgramIx;

    fn id(&self) -> Cow<str> {
        "yellowstone_vixen_parser::jup_programs::orca::InstructionParser".into()
    }

    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
        yellowstone_vixen_core::Prefilter::builder()
            .account_owners([orca_whirlpools_client::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, ix_update: &InstructionUpdate) -> ParseResult<Self::Output> {
        if ix_update.program.equals_ref(orca_whirlpools_client::ID) {
            InstructionParser::parse_impl(ix_update)
        } else {
            Err(ParseError::Filtered)
        }
    }
}

impl ProgramParser for InstructionParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        orca_whirlpools_client::ID.to_bytes().into()
    }
}

impl InstructionParser {
    pub(crate) fn parse_impl(ix: &InstructionUpdate) -> Result<OrcaProgramIx, ParseError> {
        let accounts_len = ix.accounts.len();
        let ix_discriminator: [u8; 8] = ix.data[0..IX_DISCRIMINATOR_SIZE].try_into()?;
        let mut ix_data = &ix.data[IX_DISCRIMINATOR_SIZE..];

        match ix_discriminator {
            SWAP_IX_DISC => {
                check_min_accounts_req(accounts_len, 11)?;
                let swap_ix_data: SwapIxData = BorshDeserialize::deserialize(&mut ix_data).unwrap();
                Ok(OrcaProgramIx::Swap(
                    SwapAccounts {
                        token_program: ix.accounts[0],
                        token_authority: ix.accounts[1],
                        whirlpool: ix.accounts[2],
                        token_owner_account_a: ix.accounts[3],
                        token_vault_a: ix.accounts[4],
                        token_owner_account_b: ix.accounts[5],
                        token_vault_b: ix.accounts[6],
                        tick_array0: ix.accounts[7],
                        tick_array1: ix.accounts[8],
                        tick_array2: ix.accounts[9],
                        oracle: ix.accounts[10],
                    },
                    SwapIxData {
                        a_to_b: swap_ix_data.a_to_b,
                        amount: swap_ix_data.amount,
                        other_amount_threshold: swap_ix_data.other_amount_threshold,
                        sqrt_price_limit: swap_ix_data.sqrt_price_limit,
                        amount_specified_is_input: swap_ix_data.amount_specified_is_input,
                    },
                ))
            },
            SWAP_V2_IX_DISC => {
                check_min_accounts_req(accounts_len, 15)?;
                let swap_ix_v2_data: SwapV2IxData =
                    BorshDeserialize::deserialize(&mut ix_data).unwrap();
                Ok(OrcaProgramIx::SwapV2(
                    SwapV2Accounts {
                        token_program_a: ix.accounts[0],
                        token_program_b: ix.accounts[1],
                        memo_program: ix.accounts[2],
                        token_authority: ix.accounts[3],
                        whirlpool: ix.accounts[4],
                        token_mint_a: ix.accounts[5],
                        token_mint_b: ix.accounts[6],
                        token_owner_account_a: ix.accounts[7],
                        token_vault_a: ix.accounts[8],
                        token_owner_account_b: ix.accounts[9],
                        token_vault_b: ix.accounts[10],
                        tick_array0: ix.accounts[11],
                        tick_array1: ix.accounts[12],
                        tick_array2: ix.accounts[13],
                        oracle: ix.accounts[14],
                    },
                    SwapV2IxData {
                        a_to_b: swap_ix_v2_data.a_to_b,
                        amount: swap_ix_v2_data.amount,
                        other_amount_threshold: swap_ix_v2_data.other_amount_threshold,
                        sqrt_price_limit: swap_ix_v2_data.sqrt_price_limit,
                        amount_specified_is_input: swap_ix_v2_data.amount_specified_is_input,
                    },
                ))
            },
            _ => Err(ParseError::from("Unknown instruction")),
        }
    }
}

#[cfg(test)]
mod tests {
    use yellowstone_vixen_mock::tx_fixture;

    use super::*;

    #[tokio::test]
    async fn test_swap_ix_parsing() {
        let parser = InstructionParser;

        let ixs = tx_fixture!("3WC8LGHHs3wYzWef1YmLsRS96G1s5BV4XJYhzvypgWp1uGG16SxepFCd7FhHaTieW66Yn9JFR4tUPA1HYArgFZaA",&parser);

        if let OrcaProgramIx::Swap(accounts, data) = ixs[0] {
            assert_eq!(
                accounts.whirlpool.to_string(),
                "ENYEHSyduTbFN1xoSEGaLu7c1F8AqKucdscMuV5Yypy2".to_string()
            );
            assert_eq!(data.a_to_b, false);
        } else {
            panic!("Invalid Instruction");
        }
    }
}
