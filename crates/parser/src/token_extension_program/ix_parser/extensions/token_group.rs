use spl_token_group_interface::instruction::{
    InitializeGroup, InitializeMember, TokenGroupInstruction, UpdateGroupAuthority,
    UpdateGroupMaxSize,
};
use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};

use super::helpers::ExtensionIxParser;
use crate::{helpers::check_min_accounts_req, Result, ResultExt};

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
pub enum TokenGroupIx {
    InitializeGroup(InitializeGroupAccounts, InitializeGroup),
    UpdateGroupMaxSize(UpdateGroupMaxSizeAccounts, UpdateGroupMaxSize),
    UpdateGroupAuthority(UpdateGroupAuthorityAccounts, UpdateGroupAuthority),
    InitializeMember(InitializeMemberAccounts, InitializeMember),
}

impl ExtensionIxParser for TokenGroupIx {
    fn try_parse_extension_ix(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();

        let ix_type = TokenGroupInstruction::unpack(&ix.data)
            .parse_err("Error unpacking token group instruction data")?;

        match ix_type {
            TokenGroupInstruction::InitializeGroup(data) => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenGroupIx::InitializeGroup(
                    InitializeGroupAccounts {
                        group: ix.accounts[0],
                        mint: ix.accounts[1],
                        mint_authority: ix.accounts[2],
                    },
                    data,
                ))
            },
            TokenGroupInstruction::UpdateGroupMaxSize(data) => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenGroupIx::UpdateGroupMaxSize(
                    UpdateGroupMaxSizeAccounts {
                        group: ix.accounts[0],
                        update_authority: ix.accounts[1],
                    },
                    data,
                ))
            },

            TokenGroupInstruction::UpdateGroupAuthority(data) => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenGroupIx::UpdateGroupAuthority(
                    UpdateGroupAuthorityAccounts {
                        group: ix.accounts[0],
                        current_authority: ix.accounts[1],
                    },
                    data,
                ))
            },

            TokenGroupInstruction::InitializeMember(data) => {
                check_min_accounts_req(accounts_len, 5)?;
                Ok(TokenGroupIx::InitializeMember(
                    InitializeMemberAccounts {
                        member: ix.accounts[0],
                        member_mint: ix.accounts[1],
                        member_mint_authority: ix.accounts[2],
                        group: ix.accounts[3],
                        group_update_authority: ix.accounts[4],
                    },
                    data,
                ))
            },
        }
    }
}

#[cfg(feature = "proto")]
mod proto_parser {
    use yellowstone_vixen_proto::parser::*;

    use super::*;
    use crate::helpers::IntoProtoData;

    impl IntoProtoData<InitializeGroupAccountsProto> for InitializeGroupAccounts {
        fn into_proto_data(self) -> InitializeGroupAccountsProto {
            InitializeGroupAccountsProto {
                group: self.group.to_string(),
                mint: self.mint.to_string(),
                mint_authority: self.mint_authority.to_string(),
            }
        }
    }

    impl IntoProtoData<UpdateGroupMaxSizeAccountsProto> for UpdateGroupMaxSizeAccounts {
        fn into_proto_data(self) -> UpdateGroupMaxSizeAccountsProto {
            UpdateGroupMaxSizeAccountsProto {
                group: self.group.to_string(),
                update_authority: self.update_authority.to_string(),
            }
        }
    }

    impl IntoProtoData<UpdateGroupAuthorityAccountsProto> for UpdateGroupAuthorityAccounts {
        fn into_proto_data(self) -> UpdateGroupAuthorityAccountsProto {
            UpdateGroupAuthorityAccountsProto {
                group: self.group.to_string(),
                current_authority: self.current_authority.to_string(),
            }
        }
    }

    impl IntoProtoData<InitializeMemberAccountsProto> for InitializeMemberAccounts {
        fn into_proto_data(self) -> InitializeMemberAccountsProto {
            InitializeMemberAccountsProto {
                member: self.member.to_string(),
                member_mint: self.member_mint.to_string(),
                member_mint_authority: self.member_mint_authority.to_string(),
                group: self.group.to_string(),
                group_update_authority: self.group_update_authority.to_string(),
            }
        }
    }

    impl IntoProtoData<InitializeGroupDataProto>
        for spl_token_group_interface::instruction::InitializeGroup
    {
        fn into_proto_data(self) -> InitializeGroupDataProto {
            InitializeGroupDataProto {
                update_authority: Some(self.update_authority.0.to_string()),
                max_size: Into::<u32>::into(self.max_size).into(),
            }
        }
    }

    impl IntoProtoData<UpdateGroupMaxSizeDataProto>
        for spl_token_group_interface::instruction::UpdateGroupMaxSize
    {
        fn into_proto_data(self) -> UpdateGroupMaxSizeDataProto {
            UpdateGroupMaxSizeDataProto {
                max_size: Into::<u32>::into(self.max_size).into(),
            }
        }
    }

    impl IntoProtoData<UpdateGroupAuthorityDataProto>
        for spl_token_group_interface::instruction::UpdateGroupAuthority
    {
        fn into_proto_data(self) -> UpdateGroupAuthorityDataProto {
            UpdateGroupAuthorityDataProto {
                new_authority: self.new_authority.0.to_string(),
            }
        }
    }

    impl IntoProtoData<TokenGroupIxProto> for TokenGroupIx {
        fn into_proto_data(self) -> TokenGroupIxProto {
            match self {
                TokenGroupIx::InitializeGroup(ri) => TokenGroupIxProto {
                    ix_oneof: Some(token_group_ix_proto::IxOneof::InitializeGroupIx(
                        InitializeGroupIxProto {
                            accounts: Some(ri.accounts.into_proto_data()),
                            data: Some(ri.data.unwrap().into_proto_data()),
                        },
                    )),
                },

                TokenGroupIx::UpdateGroupMaxSize(ri) => TokenGroupIxProto {
                    ix_oneof: Some(token_group_ix_proto::IxOneof::UpdateGroupMaxSizeIx(
                        UpdateGroupMaxSizeIxProto {
                            accounts: Some(ri.accounts.into_proto_data()),
                            data: Some(ri.data.unwrap().into_proto_data()),
                        },
                    )),
                },

                TokenGroupIx::UpdateGroupAuthority(ri) => TokenGroupIxProto {
                    ix_oneof: Some(token_group_ix_proto::IxOneof::UpdateGroupAuthorityIx(
                        UpdateGroupAuthorityIxProto {
                            accounts: Some(ri.accounts.into_proto_data()),
                            data: Some(ri.data.unwrap().into_proto_data()),
                        },
                    )),
                },

                TokenGroupIx::InitializeMember(ri) => TokenGroupIxProto {
                    ix_oneof: Some(token_group_ix_proto::IxOneof::InitializeMemberIx(
                        InitializeMemberIxProto {
                            accounts: Some(ri.accounts.into_proto_data()),
                        },
                    )),
                },
            }
        }
    }
}
