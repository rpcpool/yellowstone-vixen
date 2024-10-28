use borsh::{BorshDeserialize, BorshSerialize};
use yellowstone_vixen_core::Pubkey;

pub const SWAP_IX_DISC: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];

#[derive(Debug, Clone, Copy)]
pub struct SwapAccounts {
    /// The user performing the swap
    pub payer: Pubkey,
    /// The factory state to read protocol fees
    pub amm_config: Pubkey,
    /// The program account of the pool in which the swap will be performed
    pub pool_state: Pubkey,
    /// The user token account for input token
    pub input_token_account: Pubkey,
    /// The user token account for output token
    pub output_token_account: Pubkey,
    /// The vault token account for input token
    pub input_vault: Pubkey,
    /// The vault token account for output token
    pub output_vault: Pubkey,
    /// The program account for the most recent oracle observation
    pub observation_state: Pubkey,
    /// SPL program for token transfers
    pub token_program: Pubkey,
    /// The program account for the tick array
    pub tick_array: Pubkey,
}

pub const SWAP_V2_IX_DISC: [u8; 8] = [43, 4, 237, 11, 26, 201, 30, 98];

#[derive(Debug, Clone, Copy)]
pub struct SwapV2Accounts {
    /// The user performing the swap
    pub payer: Pubkey,
    /// The factory state to read protocol fees
    pub amm_config: Pubkey,
    /// The program account of the pool in which the swap will be performed
    pub pool_state: Pubkey,
    /// The user token account for input token
    pub input_token_account: Pubkey,
    /// The user token account for output token
    pub output_token_account: Pubkey,
    /// The vault token account for input token
    pub input_vault: Pubkey,
    /// The vault token account for output token
    pub output_vault: Pubkey,
    /// The program account for the most recent oracle observation
    pub observation_state: Pubkey,
    /// SPL program for token transfers
    pub token_program: Pubkey,
    /// SPL 2022 Token program
    pub token_2022_program: Pubkey,
    /// Memo program
    pub memo_program: Pubkey,
    /// Input vault mint
    pub input_vault_mint: Pubkey,
    /// Output vault mint
    pub output_vault_mint: Pubkey,
    /// The program account for the tick array
    pub tick_array: Pubkey,
}

#[derive(Debug, Clone, Copy, BorshDeserialize, BorshSerialize)]
pub struct SwapIxData {
    /// The amount of input token to swap
    pub amount: u64,
    /// The minimum amount of output token to receive
    pub other_amount_threshold: u64,
    /// The square root of the maximum price to accept
    pub sqrt_price_limit_x64: u128,
    /// Whether the amount specified is the input or output
    pub is_base_input: bool,
}

#[derive(Debug)]
pub enum RaydiumProgramIx {
    Swap(SwapAccounts, SwapIxData),
    SwapV2(SwapV2Accounts, SwapIxData),
}

#[cfg(feature = "proto")]
mod proto_parser {
    use yellowstone_vixen_proto::parser::{
        raydium_program_ix_proto::IxOneof, RaydiumProgramIxProto, RaydiumSwapAccountsProto,
        RaydiumSwapInstructionProto, RaydiumSwapIxDataProto, RaydiumSwapV2AccountsProto,
        RaydiumSwapV2InstructionProto,
    };

    use super::{RaydiumProgramIx, SwapAccounts, SwapIxData, SwapV2Accounts};
    use crate::helpers::IntoProto;

    impl IntoProto<RaydiumSwapAccountsProto> for SwapAccounts {
        fn into_proto(self) -> RaydiumSwapAccountsProto {
            RaydiumSwapAccountsProto {
                payer: self.payer.to_string(),
                amm_config: self.amm_config.to_string(),
                pool_state: self.pool_state.to_string(),
                input_token_account: self.input_token_account.to_string(),
                output_token_account: self.output_token_account.to_string(),
                input_vault: self.input_vault.to_string(),
                output_vault: self.output_vault.to_string(),
                observation_state: self.observation_state.to_string(),
                token_program: self.token_program.to_string(),
                tick_array: self.tick_array.to_string(),
            }
        }
    }

    impl IntoProto<RaydiumSwapIxDataProto> for SwapIxData {
        fn into_proto(self) -> RaydiumSwapIxDataProto {
            RaydiumSwapIxDataProto {
                amount: self.amount,
                other_amount_threshold: self.other_amount_threshold,
                sqrt_price_limit_x64: self.sqrt_price_limit_x64.to_string(),
                is_base_input: self.is_base_input,
            }
        }
    }

    impl IntoProto<RaydiumSwapV2AccountsProto> for SwapV2Accounts {
        fn into_proto(self) -> RaydiumSwapV2AccountsProto {
            RaydiumSwapV2AccountsProto {
                payer: self.payer.to_string(),
                amm_config: self.amm_config.to_string(),
                pool_state: self.pool_state.to_string(),
                input_token_account: self.input_token_account.to_string(),
                output_token_account: self.output_token_account.to_string(),
                input_vault: self.input_vault.to_string(),
                output_vault: self.output_vault.to_string(),
                observation_state: self.observation_state.to_string(),
                token_program: self.token_program.to_string(),
                token_2022_program: self.token_2022_program.to_string(),
                memo_program: self.memo_program.to_string(),
                input_vault_mint: self.input_vault_mint.to_string(),
                output_vault_mint: self.output_vault_mint.to_string(),
                tick_array: self.tick_array.to_string(),
            }
        }
    }

    impl IntoProto<RaydiumProgramIxProto> for RaydiumProgramIx {
        fn into_proto(self) -> RaydiumProgramIxProto {
            match self {
                RaydiumProgramIx::Swap(accounts, data) => RaydiumProgramIxProto {
                    ix_oneof: Some(IxOneof::Swap(RaydiumSwapInstructionProto {
                        accounts: Some(accounts.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                RaydiumProgramIx::SwapV2(accounts, data) => RaydiumProgramIxProto {
                    ix_oneof: Some(IxOneof::SwapV2(RaydiumSwapV2InstructionProto {
                        accounts: Some(accounts.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },
            }
        }
    }
}
