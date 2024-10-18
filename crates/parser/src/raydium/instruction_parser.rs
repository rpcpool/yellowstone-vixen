use std::borrow::Cow;

use borsh::BorshDeserialize;
use yellowstone_vixen_core::{
    instruction::InstructionUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};

use super::instruction_helpers::{
    RaydiumProgramIx, SwapAccounts, SwapIxData, SwapV2Accounts, SWAP_IX_DISC, SWAP_V2_IX_DISC,
};
use crate::{
    helpers::{check_min_accounts_req, IX_DISCRIMINATOR_SIZE},
    raydium::RADIUM_V3_PROGRAM_ID,
};

#[derive(Debug, Clone, Copy)]
pub struct InstructionParser;

impl Parser for InstructionParser {
    type Input = InstructionUpdate;
    type Output = RaydiumProgramIx;

    fn id(&self) -> Cow<str> { "yellowstone_vixen_parser::jup_programs::InstructionParser".into() }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([RADIUM_V3_PROGRAM_ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, ix_update: &InstructionUpdate) -> ParseResult<Self::Output> {
        if ix_update.program.equals_ref(RADIUM_V3_PROGRAM_ID) {
            return InstructionParser::parse_impl(ix_update);
        }
        Err(ParseError::Filtered)
    }
}

impl ProgramParser for InstructionParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        RADIUM_V3_PROGRAM_ID.to_bytes().into()
    }
}

impl InstructionParser {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn parse_impl(ix: &InstructionUpdate) -> Result<RaydiumProgramIx, ParseError> {
        let accounts_len = ix.accounts.len();
        let ix_discriminator: [u8; 8] = ix.data[0..IX_DISCRIMINATOR_SIZE].try_into()?;
        let mut ix_data = &ix.data[IX_DISCRIMINATOR_SIZE..];
        let swap_single_ix_data: SwapIxData = BorshDeserialize::deserialize(&mut ix_data)?;
        match ix_discriminator {
            SWAP_IX_DISC => {
                check_min_accounts_req(accounts_len, 10)?;
                Ok(RaydiumProgramIx::Swap(
                    SwapAccounts {
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
                    SwapIxData {
                        amount: swap_single_ix_data.amount,
                        other_amount_threshold: swap_single_ix_data.other_amount_threshold,
                        is_base_input: swap_single_ix_data.is_base_input,
                        sqrt_price_limit_x64: swap_single_ix_data.sqrt_price_limit_x64,
                    },
                ))
            },
            SWAP_V2_IX_DISC => {
                check_min_accounts_req(accounts_len, 14)?;
                Ok(RaydiumProgramIx::SwapV2(
                    SwapV2Accounts {
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
                    SwapIxData {
                        amount: swap_single_ix_data.amount,
                        other_amount_threshold: swap_single_ix_data.other_amount_threshold,
                        is_base_input: swap_single_ix_data.is_base_input,
                        sqrt_price_limit_x64: swap_single_ix_data.sqrt_price_limit_x64,
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

        let ixs = tx_fixture!("MKJhpfmz9ji2HbuP8Fk8s4XCQinWXBY7wcwgzn5PyibcV99RfmCPJt671jtqPFFEXDByYCzrdPh6AKjKgUA4HPY"
        ,&parser);

        if let RaydiumProgramIx::Swap(accounts, data) = ixs[0] {
            assert_eq!(
                accounts.amm_config.to_string(),
                "9iFER3bpjf1PTTCQCfTRu17EJgvsxo9pVyA9QWwEuX4x".to_string()
            );
            assert_eq!(data.amount, 553_573_055);
        } else {
            panic!("Invalid Instruction");
        }
    }
}
