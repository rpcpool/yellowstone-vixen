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
    use yellowstone_vixen_proto::parser::{
        token_group_ix_proto, InitializeGroupAccountsProto, InitializeGroupDataProto,
        InitializeGroupIxProto, InitializeMemberAccountsProto, InitializeMemberIxProto,
        TokenGroupIxProto, UpdateGroupAuthorityAccountsProto, UpdateGroupAuthorityDataProto,
        UpdateGroupAuthorityIxProto, UpdateGroupMaxSizeAccountsProto, UpdateGroupMaxSizeDataProto,
        UpdateGroupMaxSizeIxProto,
    };

    use super::{
        InitializeGroupAccounts, InitializeMemberAccounts, TokenGroupIx,
        UpdateGroupAuthorityAccounts, UpdateGroupMaxSizeAccounts,
    };
    use crate::helpers::IntoProto;

    impl IntoProto<InitializeGroupAccountsProto> for InitializeGroupAccounts {
        fn into_proto(self) -> InitializeGroupAccountsProto {
            InitializeGroupAccountsProto {
                group: self.group.to_string(),
                mint: self.mint.to_string(),
                mint_authority: self.mint_authority.to_string(),
            }
        }
    }

    impl IntoProto<UpdateGroupMaxSizeAccountsProto> for UpdateGroupMaxSizeAccounts {
        fn into_proto(self) -> UpdateGroupMaxSizeAccountsProto {
            UpdateGroupMaxSizeAccountsProto {
                group: self.group.to_string(),
                update_authority: self.update_authority.to_string(),
            }
        }
    }

    impl IntoProto<UpdateGroupAuthorityAccountsProto> for UpdateGroupAuthorityAccounts {
        fn into_proto(self) -> UpdateGroupAuthorityAccountsProto {
            UpdateGroupAuthorityAccountsProto {
                group: self.group.to_string(),
                current_authority: self.current_authority.to_string(),
            }
        }
    }

    impl IntoProto<InitializeMemberAccountsProto> for InitializeMemberAccounts {
        fn into_proto(self) -> InitializeMemberAccountsProto {
            InitializeMemberAccountsProto {
                member: self.member.to_string(),
                member_mint: self.member_mint.to_string(),
                member_mint_authority: self.member_mint_authority.to_string(),
                group: self.group.to_string(),
                group_update_authority: self.group_update_authority.to_string(),
            }
        }
    }

    impl IntoProto<InitializeGroupDataProto>
        for spl_token_group_interface::instruction::InitializeGroup
    {
        fn into_proto(self) -> InitializeGroupDataProto {
            InitializeGroupDataProto {
                update_authority: Some(self.update_authority.0.to_string()),
                max_size: Into::<u32>::into(self.max_size).into(),
            }
        }
    }

    impl IntoProto<UpdateGroupMaxSizeDataProto>
        for spl_token_group_interface::instruction::UpdateGroupMaxSize
    {
        fn into_proto(self) -> UpdateGroupMaxSizeDataProto {
            UpdateGroupMaxSizeDataProto {
                max_size: Into::<u32>::into(self.max_size).into(),
            }
        }
    }

    impl IntoProto<UpdateGroupAuthorityDataProto>
        for spl_token_group_interface::instruction::UpdateGroupAuthority
    {
        fn into_proto(self) -> UpdateGroupAuthorityDataProto {
            UpdateGroupAuthorityDataProto {
                new_authority: self.new_authority.0.to_string(),
            }
        }
    }

    impl IntoProto<TokenGroupIxProto> for TokenGroupIx {
        fn into_proto(self) -> TokenGroupIxProto {
            match self {
                TokenGroupIx::InitializeGroup(acc, data) => TokenGroupIxProto {
                    ix_oneof: Some(token_group_ix_proto::IxOneof::InitializeGroupIx(
                        InitializeGroupIxProto {
                            accounts: Some(acc.into_proto()),
                            data: Some(data.into_proto()),
                        },
                    )),
                },

                TokenGroupIx::UpdateGroupMaxSize(acc, data) => TokenGroupIxProto {
                    ix_oneof: Some(token_group_ix_proto::IxOneof::UpdateGroupMaxSizeIx(
                        UpdateGroupMaxSizeIxProto {
                            accounts: Some(acc.into_proto()),
                            data: Some(data.into_proto()),
                        },
                    )),
                },

                TokenGroupIx::UpdateGroupAuthority(acc, data) => TokenGroupIxProto {
                    ix_oneof: Some(token_group_ix_proto::IxOneof::UpdateGroupAuthorityIx(
                        UpdateGroupAuthorityIxProto {
                            accounts: Some(acc.into_proto()),
                            data: Some(data.into_proto()),
                        },
                    )),
                },

                TokenGroupIx::InitializeMember(acc, _) => TokenGroupIxProto {
                    ix_oneof: Some(token_group_ix_proto::IxOneof::InitializeMemberIx(
                        InitializeMemberIxProto {
                            accounts: Some(acc.into_proto()),
                        },
                    )),
                },
            }
        }
    }
}
