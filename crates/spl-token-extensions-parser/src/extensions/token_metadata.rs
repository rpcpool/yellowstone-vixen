use spl_token_metadata_interface::instruction::{
    Emit, Initialize, RemoveKey, TokenMetadataInstruction as SplTokenMetadataInstruction,
    UpdateAuthority, UpdateField,
};
use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};

use super::extension::ExtensionInstructionParser;

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
pub enum TokenMetadataInstruction {
    Initialize {
        accounts: InitializeAccounts,
        args: Initialize,
    },
    UpdateField {
        accounts: UpdateFieldAccounts,
        args: UpdateField,
    },
    RemoveKey {
        accounts: RmoveKeyAccounts,
        args: RemoveKey,
    },
    UpdateAuthority {
        accounts: UpdateAuthorityAccounts,
        args: UpdateAuthority,
    },
    Emit {
        accounts: EmitAccounts,
        args: Emit,
    },
}

impl ExtensionInstructionParser for TokenMetadataInstruction {
    fn try_parse(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();

        let ix_type = SplTokenMetadataInstruction::unpack(&ix.data)
            .parse_err("Error unpacking token metadata instruction data")?;

        match ix_type {
            SplTokenMetadataInstruction::Initialize(args) => {
                check_min_accounts_req(accounts_len, 4)?;

                Ok(TokenMetadataInstruction::Initialize {
                    accounts: InitializeAccounts {
                        metadata: ix.accounts[0],
                        update_authority: ix.accounts[1],
                        mint: ix.accounts[2],
                        mint_authority: ix.accounts[3],
                    },
                    args,
                })
            },
            SplTokenMetadataInstruction::UpdateField(args) => {
                check_min_accounts_req(accounts_len, 2)?;

                Ok(TokenMetadataInstruction::UpdateField {
                    accounts: UpdateFieldAccounts {
                        metadata: ix.accounts[0],
                        update_authority: ix.accounts[1],
                    },
                    args,
                })
            },
            SplTokenMetadataInstruction::RemoveKey(args) => {
                check_min_accounts_req(accounts_len, 2)?;

                Ok(TokenMetadataInstruction::RemoveKey {
                    accounts: RmoveKeyAccounts {
                        metadata: ix.accounts[0],
                        update_authority: ix.accounts[1],
                    },
                    args,
                })
            },
            SplTokenMetadataInstruction::UpdateAuthority(args) => {
                check_min_accounts_req(accounts_len, 2)?;

                Ok(TokenMetadataInstruction::UpdateAuthority {
                    accounts: UpdateAuthorityAccounts {
                        metadata: ix.accounts[0],
                        current_update_authority: ix.accounts[1],
                    },
                    args,
                })
            },
            SplTokenMetadataInstruction::Emit(args) => {
                check_min_accounts_req(accounts_len, 1)?;

                Ok(TokenMetadataInstruction::Emit {
                    accounts: EmitAccounts {
                        metadata: ix.accounts[0],
                    },
                    args,
                })
            },
        }
    }
}
