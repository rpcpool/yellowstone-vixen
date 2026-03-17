use spl_token_group_interface::instruction::TokenGroupInstruction as SplTokenGroupInstruction;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};
use yellowstone_vixen_proc_macro::vixen;

use crate::{ExtensionInstructionParser, Pubkey};

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeGroupAccounts {
    pub group: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeGroupArgs {
    pub max_size: u64,
    pub update_authority: ::core::option::Option<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupMaxSizeAccounts {
    pub group: Pubkey,
    pub update_authority: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupMaxSizeArgs {
    pub max_size: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupAuthorityAccounts {
    pub group: Pubkey,
    pub current_authority: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupAuthorityArgs {
    pub new_authority: Option<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeMemberAccounts {
    pub member: Pubkey,
    pub member_mint: Pubkey,
    pub member_mint_authority: Pubkey,
    pub group: Pubkey,
    pub group_update_authority: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeMemberArgs {
    // empty
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct TokenGroupIx {
    #[hint(oneof = "token_group_instruction::Instruction", tags = "1, 2, 3, 4")]
    pub instruction: ::core::option::Option<token_group_instruction::Instruction>,
}

pub mod token_group_instruction {
    use super::vixen;

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeGroup {
        pub accounts: super::InitializeGroupAccounts,
        pub args: super::InitializeGroupArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct UpdateGroupMaxSize {
        pub accounts: super::UpdateGroupMaxSizeAccounts,
        pub args: super::UpdateGroupMaxSizeArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct UpdateGroupAuthority {
        pub accounts: super::UpdateGroupAuthorityAccounts,
        pub args: super::UpdateGroupAuthorityArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeMember {
        pub accounts: super::InitializeMemberAccounts,
        pub args: super::InitializeMemberArgs,
    }

    #[vixen(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Instruction {
        InitializeGroup(InitializeGroup),
        UpdateGroupMaxSize(UpdateGroupMaxSize),
        UpdateGroupAuthority(UpdateGroupAuthority),
        InitializeMember(InitializeMember),
    }
}

#[inline]
fn pod_u64_to_u64(v: spl_pod::primitives::PodU64) -> u64 {
    // PodU64 is a little-endian wrapper
    u64::from_le_bytes(v.0)
}

#[inline]
fn opt_nonzero_pubkey_to_bytes(
    v: spl_pod::optional_keys::OptionalNonZeroPubkey,
) -> ::core::option::Option<Pubkey> {
    let bytes: [u8; 32] = v.0.to_bytes();

    if bytes == [0u8; 32] {
        None
    } else {
        Some(Pubkey::new(bytes))
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
                        max_size: pod_u64_to_u64(args.max_size),
                        update_authority: opt_nonzero_pubkey_to_bytes(args.update_authority),
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
                        max_size: pod_u64_to_u64(args.max_size),
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
                        new_authority: Some(crate::Pubkey::new(args.new_authority.0.to_bytes())),
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
