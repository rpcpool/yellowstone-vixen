use spl_token_metadata_interface::instruction::{
    Emit, Initialize, RemoveKey, TokenMetadataInstruction, UpdateAuthority, UpdateField,
};
use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};

use super::helpers::ExtensionIxParser;
use crate::helpers::check_min_accounts_req;

#[derive(Debug, Clone, Copy)]
pub struct InitializeAccounts {
    pub metadata: Pubkey,
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct UpdateFieldAccounts {
    pub metadata: Pubkey,
    pub update_authority: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct RmoveKeyAccounts {
    pub metadata: Pubkey,
    pub update_authority: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct UpdateAuthorityAccounts {
    pub metadata: Pubkey,
    pub current_update_authority: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct EmitAccounts {
    pub metadata: Pubkey,
}

#[derive(Debug)]
pub enum TokenMetadataIx {
    Initialize(InitializeAccounts, Initialize),
    UpdateField(UpdateFieldAccounts, UpdateField),
    RemoveKey(RmoveKeyAccounts, RemoveKey),
    UpdateAuthority(UpdateAuthorityAccounts, UpdateAuthority),
    Emit(EmitAccounts, Emit),
}

impl ExtensionIxParser for TokenMetadataIx {
    fn try_parse_extension_ix(ix: &InstructionUpdate) -> Result<Self, String> {
        let accounts_len = ix.accounts.len();

        let ix_type = TokenMetadataInstruction::unpack(&ix.data).map_err(|e| e.to_string())?;

        match ix_type {
            TokenMetadataInstruction::Initialize(data) => {
                check_min_accounts_req(accounts_len, 4)?;

                Ok(TokenMetadataIx::Initialize(
                    InitializeAccounts {
                        metadata: ix.accounts[0],
                        update_authority: ix.accounts[1],
                        mint: ix.accounts[2],
                        mint_authority: ix.accounts[3],
                    },
                    data,
                ))
            },
            TokenMetadataInstruction::UpdateField(data) => {
                check_min_accounts_req(accounts_len, 2)?;

                Ok(TokenMetadataIx::UpdateField(
                    UpdateFieldAccounts {
                        metadata: ix.accounts[0],
                        update_authority: ix.accounts[1],
                    },
                    data,
                ))
            },

            TokenMetadataInstruction::RemoveKey(data) => {
                check_min_accounts_req(accounts_len, 2)?;

                Ok(TokenMetadataIx::RemoveKey(
                    RmoveKeyAccounts {
                        metadata: ix.accounts[0],
                        update_authority: ix.accounts[1],
                    },
                    data,
                ))
            },

            TokenMetadataInstruction::UpdateAuthority(data) => {
                check_min_accounts_req(accounts_len, 2)?;

                Ok(TokenMetadataIx::UpdateAuthority(
                    UpdateAuthorityAccounts {
                        metadata: ix.accounts[0],
                        current_update_authority: ix.accounts[1],
                    },
                    data,
                ))
            },

            TokenMetadataInstruction::Emit(data) => {
                check_min_accounts_req(accounts_len, 1)?;

                Ok(TokenMetadataIx::Emit(
                    EmitAccounts {
                        metadata: ix.accounts[0],
                    },
                    data,
                ))
            },
        }
    }
}
