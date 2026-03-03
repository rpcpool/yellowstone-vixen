use spl_token_group_interface::instruction::TokenGroupInstruction as SplTokenGroupInstruction;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};
use yellowstone_vixen_proc_macro::vixen;

use crate::{ExtensionInstructionParser, PublicKey};

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeGroupAccounts {
    pub group: PublicKey,
    pub mint: PublicKey,
    pub mint_authority: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeGroupArgs {
    pub max_size: u64,
    pub update_authority: ::core::option::Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupMaxSizeAccounts {
    pub group: PublicKey,
    pub update_authority: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupMaxSizeArgs {
    pub max_size: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupAuthorityAccounts {
    pub group: PublicKey,
    pub current_authority: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupAuthorityArgs {
    pub new_authority: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeMemberAccounts {
    pub member: PublicKey,
    pub member_mint: PublicKey,
    pub member_mint_authority: PublicKey,
    pub group: PublicKey,
    pub group_update_authority: PublicKey,
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
        pub accounts: ::core::option::Option<super::InitializeGroupAccounts>,
        pub args: ::core::option::Option<super::InitializeGroupArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct UpdateGroupMaxSize {
        pub accounts: ::core::option::Option<super::UpdateGroupMaxSizeAccounts>,
        pub args: ::core::option::Option<super::UpdateGroupMaxSizeArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct UpdateGroupAuthority {
        pub accounts: ::core::option::Option<super::UpdateGroupAuthorityAccounts>,
        pub args: ::core::option::Option<super::UpdateGroupAuthorityArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeMember {
        pub accounts: ::core::option::Option<super::InitializeMemberAccounts>,
        pub args: ::core::option::Option<super::InitializeMemberArgs>,
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
) -> ::core::option::Option<PublicKey> {
    let bytes: [u8; 32] = v.0.to_bytes();

    if bytes == [0u8; 32] {
        None
    } else {
        Some(PublicKey::new(bytes))
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
                    accounts: Some(InitializeGroupAccounts {
                        group: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        mint: crate::PublicKey::new(ix.accounts[1].to_vec()),
                        mint_authority: crate::PublicKey::new(ix.accounts[2].to_vec()),
                    }),
                    args: Some(InitializeGroupArgs {
                        max_size: pod_u64_to_u64(args.max_size),
                        update_authority: opt_nonzero_pubkey_to_bytes(args.update_authority),
                    }),
                })
            },
            SplTokenGroupInstruction::UpdateGroupMaxSize(args) => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::UpdateGroupMaxSize(oneof::UpdateGroupMaxSize {
                    accounts: Some(UpdateGroupMaxSizeAccounts {
                        group: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        update_authority: crate::PublicKey::new(ix.accounts[1].to_vec()),
                    }),
                    args: Some(UpdateGroupMaxSizeArgs {
                        max_size: pod_u64_to_u64(args.max_size),
                    }),
                })
            },
            SplTokenGroupInstruction::UpdateGroupAuthority(args) => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::UpdateGroupAuthority(oneof::UpdateGroupAuthority {
                    accounts: Some(UpdateGroupAuthorityAccounts {
                        group: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        current_authority: crate::PublicKey::new(ix.accounts[1].to_vec()),
                    }),
                    args: Some(UpdateGroupAuthorityArgs {
                        new_authority: Some(crate::PublicKey::new(args.new_authority.0.to_bytes())),
                    }),
                })
            },
            SplTokenGroupInstruction::InitializeMember(_args) => {
                check_min_accounts_req(accounts_len, 5)?;

                oneof::Instruction::InitializeMember(oneof::InitializeMember {
                    accounts: Some(InitializeMemberAccounts {
                        member: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        member_mint: crate::PublicKey::new(ix.accounts[1].to_vec()),
                        member_mint_authority: crate::PublicKey::new(ix.accounts[2].to_vec()),
                        group: crate::PublicKey::new(ix.accounts[3].to_vec()),
                        group_update_authority: crate::PublicKey::new(ix.accounts[4].to_vec()),
                    }),
                    args: Some(InitializeMemberArgs {}),
                })
            },
        };

        Ok(TokenGroupIx {
            instruction: Some(msg),
        })
    }
}
