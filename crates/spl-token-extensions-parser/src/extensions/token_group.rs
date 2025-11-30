use spl_token_group_interface::instruction::{
    InitializeGroup, InitializeMember, TokenGroupInstruction as SplTokenGroupInstruction,
    UpdateGroupAuthority, UpdateGroupMaxSize,
};
use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};

use crate::ExtensionInstructionParser;

#[derive(Debug, Clone, Copy)]
pub struct InitializeGroupAccounts {
    pub group: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct UpdateGroupMaxSizeAccounts {
    pub group: Pubkey,
    pub update_authority: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct UpdateGroupAuthorityAccounts {
    pub group: Pubkey,
    pub current_authority: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeMemberAccounts {
    pub member: Pubkey,
    pub member_mint: Pubkey,
    pub member_mint_authority: Pubkey,
    pub group: Pubkey,
    pub group_update_authority: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenGroupInstruction {
    InitializeGroup {
        accounts: InitializeGroupAccounts,
        args: InitializeGroup,
    },
    UpdateGroupMaxSize {
        accounts: UpdateGroupMaxSizeAccounts,
        args: UpdateGroupMaxSize,
    },
    UpdateGroupAuthority {
        accounts: UpdateGroupAuthorityAccounts,
        args: UpdateGroupAuthority,
    },
    InitializeMember {
        accounts: InitializeMemberAccounts,
        args: InitializeMember,
    },
}

impl ExtensionInstructionParser for TokenGroupInstruction {
    fn try_parse(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();

        let ix_type = SplTokenGroupInstruction::unpack(&ix.data)
            .parse_err("Error unpacking token group instruction data")?;

        Ok(match ix_type {
            SplTokenGroupInstruction::InitializeGroup(args) => {
                check_min_accounts_req(accounts_len, 3)?;
                TokenGroupInstruction::InitializeGroup {
                    accounts: InitializeGroupAccounts {
                        group: ix.accounts[0],
                        mint: ix.accounts[1],
                        mint_authority: ix.accounts[2],
                    },
                    args,
                }
            },
            SplTokenGroupInstruction::UpdateGroupMaxSize(args) => {
                check_min_accounts_req(accounts_len, 2)?;
                TokenGroupInstruction::UpdateGroupMaxSize {
                    accounts: UpdateGroupMaxSizeAccounts {
                        group: ix.accounts[0],
                        update_authority: ix.accounts[1],
                    },
                    args,
                }
            },
            SplTokenGroupInstruction::UpdateGroupAuthority(args) => {
                check_min_accounts_req(accounts_len, 2)?;
                TokenGroupInstruction::UpdateGroupAuthority {
                    accounts: UpdateGroupAuthorityAccounts {
                        group: ix.accounts[0],
                        current_authority: ix.accounts[1],
                    },
                    args,
                }
            },
            SplTokenGroupInstruction::InitializeMember(args) => {
                check_min_accounts_req(accounts_len, 5)?;
                TokenGroupInstruction::InitializeMember {
                    accounts: InitializeMemberAccounts {
                        member: ix.accounts[0],
                        member_mint: ix.accounts[1],
                        member_mint_authority: ix.accounts[2],
                        group: ix.accounts[3],
                        group_update_authority: ix.accounts[4],
                    },
                    args,
                }
            },
        })
    }
}
