use std::borrow::Cow;

use borsh::BorshDeserialize;
use yellowstone_vixen_core::{
    instruction::InstructionUpdate, ParseError, ParseResult, Parser, Prefilter,
};

use crate::{
    helpers::{
        check_min_accounts_req, InstructionParser, ReadableInstruction, IX_DISCRIMINATOR_SIZE,
    },
    jup_programs::raydium::RADIUM_V3_PROGRAM_ID,
};

use super::{
    RaydiumProgramIx, SwapAccounts, SwapIxData, SwapV2Accounts, SWAP_IX_DISC, SWAP_V2_IX_DISC,
};
use crate::helpers::check_pubkeys_match;

#[derive(Debug, Clone, Copy)]
pub struct RaydiumProgramIxParser;

impl Parser for RaydiumProgramIxParser {
    type Input = InstructionUpdate;
    type Output = RaydiumProgramIx;

    fn id(&self) -> Cow<str> {
        "yellowstone_vixen_parser::token_program::RaydiumProgramIxParser".into()
    }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([RADIUM_V3_PROGRAM_ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, ix_update: &InstructionUpdate) -> ParseResult<Self::Output> {
        if check_pubkeys_match(&ix_update.program, &RADIUM_V3_PROGRAM_ID) {
            return RaydiumProgramIxParser::parse_ix(ix_update);
        }
        Err(ParseError::Filtered)
    }
}

impl InstructionParser<RaydiumProgramIx> for RaydiumProgramIxParser {
    fn parse_ix(ix: &InstructionUpdate) -> Result<RaydiumProgramIx, ParseError> {
        let accounts_len = ix.accounts.len();
        let ix_discriminator: [u8; 8] = ix.data[0..IX_DISCRIMINATOR_SIZE].try_into()?;
        let mut ix_data = &ix.data[IX_DISCRIMINATOR_SIZE..];
        let swap_single_ix_data: SwapIxData = BorshDeserialize::deserialize(&mut ix_data)?;
        match ix_discriminator {
            SWAP_IX_DISC => {
                check_min_accounts_req(accounts_len, 10)?;
                Ok(RaydiumProgramIx::Swap(ReadableInstruction {
                    accounts: SwapAccounts {
                        payer: ix.accounts[0],
                        amm_config: ix.accounts[1],
                        pool_state: ix.accounts[2],
                        input_token_account: ix.accounts[3],
                        output_token_account: ix.accounts[4],
                        input_vault: ix.accounts[5],
                        output_vault: ix.accounts[6],
                        observation_state: ix.accounts[7],
                        token_program: ix.accounts[8],
                        tick_array: ix.accounts[9],
                    },
                    data: Some(swap_single_ix_data),
                }))
            },
            SWAP_V2_IX_DISC => {
                check_min_accounts_req(accounts_len, 14)?;
                Ok(RaydiumProgramIx::SwapV2(ReadableInstruction {
                    accounts: SwapV2Accounts {
                        payer: ix.accounts[0],
                        amm_config: ix.accounts[1],
                        pool_state: ix.accounts[2],
                        input_token_account: ix.accounts[3],
                        output_token_account: ix.accounts[4],
                        input_vault: ix.accounts[5],
                        output_vault: ix.accounts[6],
                        observation_state: ix.accounts[7],
                        token_program: ix.accounts[8],
                        token_2022_program: ix.accounts[9],
                        memo_program: ix.accounts[10],
                        input_vault_mint: ix.accounts[11],
                        output_vault_mint: ix.accounts[12],
                        tick_array: ix.accounts[13],
                    },
                    data: Some(swap_single_ix_data),
                }))
            },

            _ => Err(ParseError::from("Unknown instruction")),
        }
    }
}

#[cfg(test)]
mod tests {
    use yellowstone_vixen_mock::{run_ix_parse, tx_fixture, FixtureData, LoadFixtureFilters};

    use super::*;

    #[tokio::test]
    async fn test_swap_ix_parsing() {
        let parser = RaydiumProgramIxParser;

        let filters = LoadFixtureFilters {
            outer_ixs_programs: vec![
                "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string(),
                RADIUM_V3_PROGRAM_ID.to_string(),
            ],
            inner_ixs_discriminators: vec![SWAP_IX_DISC],
        };

        let ixs = tx_fixture!("MKJhpfmz9ji2HbuP8Fk8s4XCQinWXBY7wcwgzn5PyibcV99RfmCPJt671jtqPFFEXDByYCzrdPh6AKjKgUA4HPY",
            Some(filters));

        let parsed = run_ix_parse!(parser, &ixs[0].inner[0]);
        if let RaydiumProgramIx::Swap(ReadableInstruction { accounts, data }) = parsed {
            assert_eq!(
                accounts.amm_config.to_string(),
                "9iFER3bpjf1PTTCQCfTRu17EJgvsxo9pVyA9QWwEuX4x".to_string()
            );
            assert!(data.is_some());
            let swap_ix_data = data.unwrap();
            assert_eq!(swap_ix_data.amount, 553_573_055);
        } else {
            panic!("Invalid Instruction");
        }
    }
}
