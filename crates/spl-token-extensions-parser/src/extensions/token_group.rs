use spl_token_group_interface::instruction::TokenGroupInstruction as SplTokenGroupInstruction;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};

use crate::{ExtensionInstructionParser, PubkeyBytes};

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeGroupAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub group: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub mint_authority: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeGroupArgs {
    #[prost(uint64, tag = "1")]
    pub max_size: u64,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub update_authority: ::core::option::Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGroupMaxSizeAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub group: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub update_authority: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGroupMaxSizeArgs {
    #[prost(uint64, tag = "1")]
    pub max_size: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGroupAuthorityAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub group: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub current_authority: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGroupAuthorityArgs {
    #[prost(bytes = "vec", tag = "1")]
    pub new_authority: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeMemberAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub member: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub member_mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub member_mint_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub group: PubkeyBytes,
    #[prost(bytes = "vec", tag = "5")]
    pub group_update_authority: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeMemberArgs {
    // empty
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenGroupInstruction {
    #[prost(oneof = "token_group_instruction::Ix", tags = "1, 2, 3, 4")]
    pub ix: ::core::option::Option<token_group_instruction::Ix>,
}

pub mod token_group_instruction {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitializeGroup {
        #[prost(message, optional, tag = "1")]
        pub accounts: ::core::option::Option<super::InitializeGroupAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: ::core::option::Option<super::InitializeGroupArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateGroupMaxSize {
        #[prost(message, optional, tag = "1")]
        pub accounts: ::core::option::Option<super::UpdateGroupMaxSizeAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: ::core::option::Option<super::UpdateGroupMaxSizeArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateGroupAuthority {
        #[prost(message, optional, tag = "1")]
        pub accounts: ::core::option::Option<super::UpdateGroupAuthorityAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: ::core::option::Option<super::UpdateGroupAuthorityArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitializeMember {
        #[prost(message, optional, tag = "1")]
        pub accounts: ::core::option::Option<super::InitializeMemberAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: ::core::option::Option<super::InitializeMemberArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ix {
        #[prost(message, tag = "1")]
        InitializeGroup(InitializeGroup),
        #[prost(message, tag = "2")]
        UpdateGroupMaxSize(UpdateGroupMaxSize),
        #[prost(message, tag = "3")]
        UpdateGroupAuthority(UpdateGroupAuthority),
        #[prost(message, tag = "4")]
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

impl ExtensionInstructionParser for TokenGroupInstruction {
    fn try_parse(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();

        let ix_type = SplTokenGroupInstruction::unpack(&ix.data)
            .parse_err("Error unpacking token group instruction data")?;

        use token_group_instruction as oneof;

        let msg = match ix_type {
            SplTokenGroupInstruction::InitializeGroup(args) => {
                check_min_accounts_req(accounts_len, 3)?;

                oneof::Ix::InitializeGroup(oneof::InitializeGroup {
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

                oneof::Ix::UpdateGroupMaxSize(oneof::UpdateGroupMaxSize {
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

                oneof::Ix::UpdateGroupAuthority(oneof::UpdateGroupAuthority {
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

                oneof::Ix::InitializeMember(oneof::InitializeMember {
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

        Ok(TokenGroupInstruction { ix: Some(msg) })
    }
}
