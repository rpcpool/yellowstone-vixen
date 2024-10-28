use std::fmt::Debug;

use borsh::{BorshDeserialize, BorshSerialize};
use yellowstone_vixen_core::Pubkey;

pub const SWAP_IX_DISC: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];

#[derive(Debug, Clone, Copy)]
pub struct SwapAccounts {
    pub token_program: Pubkey,
    pub token_authority: Pubkey,
    pub whirlpool: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_array0: Pubkey,
    pub tick_array1: Pubkey,
    pub tick_array2: Pubkey,
    pub oracle: Pubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct SwapIxData {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
}

pub const SWAP_V2_IX_DISC: [u8; 8] = [43, 4, 237, 11, 26, 201, 30, 98];

#[derive(Debug, Clone, Copy)]
pub struct SwapV2Accounts {
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub memo_program: Pubkey,
    pub token_authority: Pubkey,
    pub whirlpool: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_array0: Pubkey,
    pub tick_array1: Pubkey,
    pub tick_array2: Pubkey,
    pub oracle: Pubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct SwapV2IxData {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
}

#[derive(Debug)]
pub enum OrcaProgramIx {
    Swap(SwapAccounts, SwapIxData),
    SwapV2(SwapV2Accounts, SwapV2IxData),
}

#[cfg(feature = "proto")]
mod proto_parser {
    use yellowstone_vixen_proto::parser::{
        orca_program_ix_proto::IxOneof, OrcaProgramIxProto, OrcaSwapAccountsProto,
        OrcaSwapInstructionProto, OrcaSwapIxDataProto, OrcaSwapV2AccountsProto,
        OrcaSwapV2InstructionProto, OrcaSwapV2IxDataProto,
    };

    use super::{OrcaProgramIx, SwapAccounts, SwapIxData, SwapV2Accounts, SwapV2IxData};
    use crate::helpers::IntoProto;

    impl IntoProto<OrcaSwapAccountsProto> for SwapAccounts {
        fn into_proto(self) -> OrcaSwapAccountsProto {
            OrcaSwapAccountsProto {
                token_program: self.token_program.to_string(),
                token_authority: self.token_authority.to_string(),
                whirlpool: self.whirlpool.to_string(),
                token_owner_account_a: self.token_owner_account_a.to_string(),
                token_vault_a: self.token_vault_a.to_string(),
                token_owner_account_b: self.token_owner_account_b.to_string(),
                token_vault_b: self.token_vault_b.to_string(),
                tick_array0: self.tick_array0.to_string(),
                tick_array1: self.tick_array1.to_string(),
                tick_array2: self.tick_array2.to_string(),
                oracle: self.oracle.to_string(),
            }
        }
    }

    impl IntoProto<OrcaSwapIxDataProto> for SwapIxData {
        fn into_proto(self) -> OrcaSwapIxDataProto {
            OrcaSwapIxDataProto {
                amount: self.amount,
                other_amount_threshold: self.other_amount_threshold,
                sqrt_price_limit: self.sqrt_price_limit.to_string(),
                amount_specified_is_input: self.amount_specified_is_input,
                a_to_b: self.a_to_b,
            }
        }
    }

    impl IntoProto<OrcaSwapV2AccountsProto> for SwapV2Accounts {
        fn into_proto(self) -> OrcaSwapV2AccountsProto {
            OrcaSwapV2AccountsProto {
                token_program_a: self.token_program_a.to_string(),
                token_program_b: self.token_program_b.to_string(),
                memo_program: self.memo_program.to_string(),
                token_authority: self.token_authority.to_string(),
                whirlpool: self.whirlpool.to_string(),
                token_mint_a: self.token_mint_a.to_string(),
                token_mint_b: self.token_mint_b.to_string(),
                token_owner_account_a: self.token_owner_account_a.to_string(),
                token_vault_a: self.token_vault_a.to_string(),
                token_owner_account_b: self.token_owner_account_b.to_string(),
                token_vault_b: self.token_vault_b.to_string(),
                tick_array0: self.tick_array0.to_string(),
                tick_array1: self.tick_array1.to_string(),
                tick_array2: self.tick_array2.to_string(),
                oracle: self.oracle.to_string(),
            }
        }
    }

    impl IntoProto<OrcaSwapV2IxDataProto> for SwapV2IxData {
        fn into_proto(self) -> OrcaSwapV2IxDataProto {
            OrcaSwapV2IxDataProto {
                amount: self.amount,
                other_amount_threshold: self.other_amount_threshold,
                sqrt_price_limit: self.sqrt_price_limit.to_string(),
                amount_specified_is_input: self.amount_specified_is_input,
                a_to_b: self.a_to_b,
            }
        }
    }

    impl IntoProto<OrcaProgramIxProto> for OrcaProgramIx {
        fn into_proto(self) -> OrcaProgramIxProto {
            match self {
                OrcaProgramIx::Swap(acc, data) => OrcaProgramIxProto {
                    ix_oneof: Some(IxOneof::Swap(OrcaSwapInstructionProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },
                OrcaProgramIx::SwapV2(acc, data) => OrcaProgramIxProto {
                    ix_oneof: Some(IxOneof::SwapV2(OrcaSwapV2InstructionProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },
            }
        }
    }
}
