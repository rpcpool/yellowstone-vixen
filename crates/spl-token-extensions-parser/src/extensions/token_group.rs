use spl_token_group_interface::instruction::TokenGroupInstruction as SplTokenGroupInstruction;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};
use yellowstone_vixen_proc_macro::vixen_proto;

use crate::{ExtensionInstructionParser, PubkeyBytes};

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeGroupAccounts {
    pub group: PubkeyBytes,
    pub mint: PubkeyBytes,
    pub mint_authority: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeGroupArgs {
    pub max_size: u64,
    pub update_authority: ::core::option::Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupMaxSizeAccounts {
    pub group: PubkeyBytes,
    pub update_authority: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupMaxSizeArgs {
    pub max_size: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupAuthorityAccounts {
    pub group: PubkeyBytes,
    pub current_authority: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateGroupAuthorityArgs {
    pub new_authority: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeMemberAccounts {
    pub member: PubkeyBytes,
    pub member_mint: PubkeyBytes,
    pub member_mint_authority: PubkeyBytes,
    pub group: PubkeyBytes,
    pub group_update_authority: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeMemberArgs {
    // empty
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TokenGroupIx {
    #[vixen_proto_hint(oneof = "token_group_instruction::Instruction", tags = "1, 2, 3, 4")]
    pub instruction: ::core::option::Option<token_group_instruction::Instruction>,
}

pub mod token_group_instruction {
    use super::vixen_proto;

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct InitializeGroup {
        pub accounts: ::core::option::Option<super::InitializeGroupAccounts>,
        pub args: ::core::option::Option<super::InitializeGroupArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct UpdateGroupMaxSize {
        pub accounts: ::core::option::Option<super::UpdateGroupMaxSizeAccounts>,
        pub args: ::core::option::Option<super::UpdateGroupMaxSizeArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct UpdateGroupAuthority {
        pub accounts: ::core::option::Option<super::UpdateGroupAuthorityAccounts>,
        pub args: ::core::option::Option<super::UpdateGroupAuthorityArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct InitializeMember {
        pub accounts: ::core::option::Option<super::InitializeMemberAccounts>,
        pub args: ::core::option::Option<super::InitializeMemberArgs>,
    }

    #[vixen_proto(oneof)]
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
) -> ::core::option::Option<PubkeyBytes> {
    let bytes: [u8; 32] = v.0.to_bytes();

    if bytes == [0u8; 32] {
        None
    } else {
        Some(bytes.to_vec())
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
                        group: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        mint_authority: ix.accounts[2].to_vec(),
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
                        group: ix.accounts[0].to_vec(),
                        update_authority: ix.accounts[1].to_vec(),
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
                        group: ix.accounts[0].to_vec(),
                        current_authority: ix.accounts[1].to_vec(),
                    }),
                    args: Some(UpdateGroupAuthorityArgs {
                        new_authority: args.new_authority.0.to_bytes().to_vec(),
                    }),
                })
            },
            SplTokenGroupInstruction::InitializeMember(_args) => {
                check_min_accounts_req(accounts_len, 5)?;

                oneof::Instruction::InitializeMember(oneof::InitializeMember {
                    accounts: Some(InitializeMemberAccounts {
                        member: ix.accounts[0].to_vec(),
                        member_mint: ix.accounts[1].to_vec(),
                        member_mint_authority: ix.accounts[2].to_vec(),
                        group: ix.accounts[3].to_vec(),
                        group_update_authority: ix.accounts[4].to_vec(),
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
