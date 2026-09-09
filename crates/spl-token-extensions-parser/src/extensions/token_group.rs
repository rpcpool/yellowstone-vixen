use shipstern_core::instruction::InstructionUpdate;
use shipstern_parser::{check_min_accounts_req, Result, ResultExt};
use shipstern_proc_macro::shipstern;
use spl_token_group_interface::instruction::TokenGroupInstruction as SplTokenGroupInstruction;

use crate::{ExtensionInstructionParser, Pubkey};

#[shipstern]
#[derive(Clone, PartialEq)]
pub struct InitializeGroupAccounts {
    pub group: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
}

#[shipstern]
#[derive(Clone, PartialEq)]
pub struct InitializeGroupArgs {
    pub max_size: u64,
    pub update_authority: ::core::option::Option<Pubkey>,
}

#[shipstern]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupMaxSizeAccounts {
    pub group: Pubkey,
    pub update_authority: Pubkey,
}

#[shipstern]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupMaxSizeArgs {
    pub max_size: u64,
}

#[shipstern]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupAuthorityAccounts {
    pub group: Pubkey,
    pub current_authority: Pubkey,
}

#[shipstern]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupAuthorityArgs {
    pub new_authority: Option<Pubkey>,
}

#[shipstern]
#[derive(Clone, PartialEq)]
pub struct InitializeMemberAccounts {
    pub member: Pubkey,
    pub member_mint: Pubkey,
    pub member_mint_authority: Pubkey,
    pub group: Pubkey,
    pub group_update_authority: Pubkey,
}

#[shipstern]
#[derive(Clone, PartialEq)]
pub struct InitializeMemberArgs {
    // empty
}

#[shipstern]
#[derive(Clone, PartialEq)]
pub struct TokenGroupIx {
    #[hint(oneof = "token_group_instruction::Instruction", tags = "1, 2, 3, 4")]
    pub instruction: ::core::option::Option<token_group_instruction::Instruction>,
}

pub mod token_group_instruction {
    use super::shipstern;

    #[shipstern]
    #[derive(Clone, PartialEq)]
    pub struct InitializeGroup {
        pub accounts: super::InitializeGroupAccounts,
        pub args: super::InitializeGroupArgs,
    }

    #[shipstern]
    #[derive(Clone, PartialEq)]
    pub struct UpdateGroupMaxSize {
        pub accounts: super::UpdateGroupMaxSizeAccounts,
        pub args: super::UpdateGroupMaxSizeArgs,
    }

    #[shipstern]
    #[derive(Clone, PartialEq)]
    pub struct UpdateGroupAuthority {
        pub accounts: super::UpdateGroupAuthorityAccounts,
        pub args: super::UpdateGroupAuthorityArgs,
    }

    #[shipstern]
    #[derive(Clone, PartialEq)]
    pub struct InitializeMember {
        pub accounts: super::InitializeMemberAccounts,
        pub args: super::InitializeMemberArgs,
    }

    #[shipstern(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Instruction {
        InitializeGroup(InitializeGroup),
        UpdateGroupMaxSize(UpdateGroupMaxSize),
        UpdateGroupAuthority(UpdateGroupAuthority),
        InitializeMember(InitializeMember),
    }
}

impl ExtensionInstructionParser for TokenGroupIx {
    fn try_parse(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();

        let ix_type = SplTokenGroupInstruction::unpack(&ix.data)
            .parse_err("Error unpacking token group instruction data")?;

        use token_group_instruction as oneof;

        let msg = match ix_type {
            SplTokenGroupInstruction::InitializeGroup(args) => {
                check_min_accounts_req(accounts_len, 3)?;

                oneof::Instruction::InitializeGroup(oneof::InitializeGroup {
                    accounts: InitializeGroupAccounts {
                        group: crate::Pubkey::new(ix.accounts[0].0),
                        mint: crate::Pubkey::new(ix.accounts[1].0),
                        mint_authority: crate::Pubkey::new(ix.accounts[2].0),
                    },
                    args: InitializeGroupArgs {
                        max_size: u64::from(args.max_size),
                        update_authority: args
                            .update_authority
                            .get()
                            .map(|key| Pubkey::new(key.to_bytes())),
                    },
                })
            },
            SplTokenGroupInstruction::UpdateGroupMaxSize(args) => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::UpdateGroupMaxSize(oneof::UpdateGroupMaxSize {
                    accounts: UpdateGroupMaxSizeAccounts {
                        group: crate::Pubkey::new(ix.accounts[0].0),
                        update_authority: crate::Pubkey::new(ix.accounts[1].0),
                    },
                    args: UpdateGroupMaxSizeArgs {
                        max_size: u64::from(args.max_size),
                    },
                })
            },
            SplTokenGroupInstruction::UpdateGroupAuthority(args) => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::UpdateGroupAuthority(oneof::UpdateGroupAuthority {
                    accounts: UpdateGroupAuthorityAccounts {
                        group: crate::Pubkey::new(ix.accounts[0].0),
                        current_authority: crate::Pubkey::new(ix.accounts[1].0),
                    },
                    args: UpdateGroupAuthorityArgs {
                        new_authority: args
                            .new_authority
                            .get()
                            .map(|key| Pubkey::new(key.to_bytes())),
                    },
                })
            },
            SplTokenGroupInstruction::InitializeMember(_args) => {
                check_min_accounts_req(accounts_len, 5)?;

                oneof::Instruction::InitializeMember(oneof::InitializeMember {
                    accounts: InitializeMemberAccounts {
                        member: crate::Pubkey::new(ix.accounts[0].0),
                        member_mint: crate::Pubkey::new(ix.accounts[1].0),
                        member_mint_authority: crate::Pubkey::new(ix.accounts[2].0),
                        group: crate::Pubkey::new(ix.accounts[3].0),
                        group_update_authority: crate::Pubkey::new(ix.accounts[4].0),
                    },
                    args: InitializeMemberArgs {},
                })
            },
        };

        Ok(TokenGroupIx {
            instruction: Some(msg),
        })
    }
}
