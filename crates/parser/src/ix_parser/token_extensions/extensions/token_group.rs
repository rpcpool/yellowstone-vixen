use spl_token_group_interface::instruction::{
    InitializeGroup, InitializeMember, TokenGroupInstruction, UpdateGroupAuthority,
    UpdateGroupMaxSize,
};
use yellowstone_vixen_core::{Instruction, Pubkey, ReadableInstruction};

use super::helpers::ExtensionIxParser;
use crate::ix_parser::helpers::check_min_accounts_req;

#[derive(Debug)]
pub struct InitializeGroupAccounts {
    pub group: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
}

#[derive(Debug)]
pub struct UpdateGroupMaxSizeAccounts {
    pub group: Pubkey,
    pub update_authority: Pubkey,
}

#[derive(Debug)]
pub struct UpdateGroupAuthorityAccounts {
    pub group: Pubkey,
    pub current_authority: Pubkey,
}

#[derive(Debug)]
pub struct InitializeMemberAccounts {
    pub member: Pubkey,
    pub member_mint: Pubkey,
    pub member_mint_authority: Pubkey,
    pub group: Pubkey,
    pub group_update_authority: Pubkey,
}
#[derive(Debug)]
pub enum TokenGroupIx {
    InitializeGroup(ReadableInstruction<InitializeGroupAccounts, InitializeGroup>),
    UpdateGroupMaxSize(ReadableInstruction<UpdateGroupMaxSizeAccounts, UpdateGroupMaxSize>),
    UpdateGroupAuthority(ReadableInstruction<UpdateGroupAuthorityAccounts, UpdateGroupAuthority>),
    InitializeMember(ReadableInstruction<InitializeMemberAccounts, InitializeMember>),
}

impl ExtensionIxParser for TokenGroupIx {
    fn try_parse_extension_ix(ix: &Instruction) -> Result<Self, String> {
        let accounts_len = ix.accounts.len();

        let ix_type = TokenGroupInstruction::unpack(&ix.data).map_err(|e| e.to_string())?;

        match ix_type {
            TokenGroupInstruction::InitializeGroup(data) => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenGroupIx::InitializeGroup(ReadableInstruction {
                    accounts: InitializeGroupAccounts {
                        group: ix.accounts[0],
                        mint: ix.accounts[1],
                        mint_authority: ix.accounts[2],
                    },
                    data: Some(data),
                }))
            },
            TokenGroupInstruction::UpdateGroupMaxSize(data) => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenGroupIx::UpdateGroupMaxSize(ReadableInstruction {
                    accounts: UpdateGroupMaxSizeAccounts {
                        group: ix.accounts[0],
                        update_authority: ix.accounts[1],
                    },
                    data: Some(data),
                }))
            },

            TokenGroupInstruction::UpdateGroupAuthority(data) => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenGroupIx::UpdateGroupAuthority(ReadableInstruction {
                    accounts: UpdateGroupAuthorityAccounts {
                        group: ix.accounts[0],
                        current_authority: ix.accounts[1],
                    },
                    data: Some(data),
                }))
            },

            TokenGroupInstruction::InitializeMember(data) => {
                check_min_accounts_req(accounts_len, 5)?;
                Ok(TokenGroupIx::InitializeMember(ReadableInstruction {
                    accounts: InitializeMemberAccounts {
                        member: ix.accounts[0],
                        member_mint: ix.accounts[1],
                        member_mint_authority: ix.accounts[2],
                        group: ix.accounts[3],
                        group_update_authority: ix.accounts[4],
                    },
                    data: Some(data),
                }))
            },
        }
    }
}
