use spl_token_metadata_interface::instruction::{
    Emit, Initialize, RemoveKey, TokenMetadataInstruction, UpdateAuthority, UpdateField,
};
use yellowstone_vixen_core::{Instruction, Pubkey, ReadableInstruction};

use super::helpers::ExtensionIxParser;
use crate::ix_parser::helpers::check_min_accounts_req;

#[derive(Debug)]
pub struct InitializeAccounts {
    pub metadata: Pubkey,
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
}

#[derive(Debug)]
pub struct UpdateFieldAccounts {
    pub metadata: Pubkey,
    pub update_authority: Pubkey,
}

#[derive(Debug)]
pub struct RmoveKeyAccounts {
    pub metadata: Pubkey,
    pub update_authority: Pubkey,
}

#[derive(Debug)]
pub struct UpdateAuthorityAccounts {
    pub metadata: Pubkey,
    pub current_update_authority: Pubkey,
}

#[derive(Debug)]
pub struct EmitAccounts {
    pub metadata: Pubkey,
}

#[derive(Debug)]
pub enum TokenMetadataIx {
    Initialize(ReadableInstruction<InitializeAccounts, Initialize>),
    UpdateField(ReadableInstruction<UpdateFieldAccounts, UpdateField>),
    RemoveKey(ReadableInstruction<RmoveKeyAccounts, RemoveKey>),
    UpdateAuthority(ReadableInstruction<UpdateAuthorityAccounts, UpdateAuthority>),
    Emit(ReadableInstruction<EmitAccounts, Emit>),
}

impl ExtensionIxParser for TokenMetadataIx {
    fn try_parse_extension_ix(ix: &Instruction) -> Result<Self, String> {
        let accounts_len = ix.accounts.len();

        let ix_type = TokenMetadataInstruction::unpack(&ix.data).map_err(|e| e.to_string())?;

        match ix_type {
            TokenMetadataInstruction::Initialize(data) => {
                check_min_accounts_req(accounts_len, 4)?;

                Ok(TokenMetadataIx::Initialize(ReadableInstruction {
                    accounts: InitializeAccounts {
                        metadata: ix.accounts[0],
                        update_authority: ix.accounts[1],
                        mint: ix.accounts[2],
                        mint_authority: ix.accounts[3],
                    },
                    data: Some(data),
                }))
            },
            TokenMetadataInstruction::UpdateField(data) => {
                check_min_accounts_req(accounts_len, 2)?;

                Ok(TokenMetadataIx::UpdateField(ReadableInstruction {
                    accounts: UpdateFieldAccounts {
                        metadata: ix.accounts[0],
                        update_authority: ix.accounts[1],
                    },
                    data: Some(data),
                }))
            },

            TokenMetadataInstruction::RemoveKey(data) => {
                check_min_accounts_req(accounts_len, 2)?;

                Ok(TokenMetadataIx::RemoveKey(ReadableInstruction {
                    accounts: RmoveKeyAccounts {
                        metadata: ix.accounts[0],
                        update_authority: ix.accounts[1],
                    },
                    data: Some(data),
                }))
            },

            TokenMetadataInstruction::UpdateAuthority(data) => {
                check_min_accounts_req(accounts_len, 2)?;

                Ok(TokenMetadataIx::UpdateAuthority(ReadableInstruction {
                    accounts: UpdateAuthorityAccounts {
                        metadata: ix.accounts[0],
                        current_update_authority: ix.accounts[1],
                    },
                    data: Some(data),
                }))
            },

            TokenMetadataInstruction::Emit(data) => {
                check_min_accounts_req(accounts_len, 1)?;

                Ok(TokenMetadataIx::Emit(ReadableInstruction {
                    accounts: EmitAccounts {
                        metadata: ix.accounts[0],
                    },
                    data: Some(data),
                }))
            },
        }
    }
}
