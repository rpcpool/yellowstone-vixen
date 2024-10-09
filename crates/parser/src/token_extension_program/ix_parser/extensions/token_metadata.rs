use spl_token_metadata_interface::instruction::{
    Emit, Initialize, RemoveKey, TokenMetadataInstruction, UpdateAuthority, UpdateField,
};
use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};

use super::helpers::ExtensionIxParser;
use crate::{helpers::check_min_accounts_req, Result, ResultExt};

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
pub enum TokenMetadataIx {
    Initialize(InitializeAccounts, Initialize),
    UpdateField(UpdateFieldAccounts, UpdateField),
    RemoveKey(RmoveKeyAccounts, RemoveKey),
    UpdateAuthority(UpdateAuthorityAccounts, UpdateAuthority),
    Emit(EmitAccounts, Emit),
}

impl ExtensionIxParser for TokenMetadataIx {
    fn try_parse_extension_ix(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();

        let ix_type = TokenMetadataInstruction::unpack(&ix.data)
            .parse_err("Error unpacking token metadata instruction data")?;

        match ix_type {
            TokenMetadataInstruction::Initialize(data) => {
                check_min_accounts_req(accounts_len, 4)?;

                Ok(TokenMetadataIx::Initialize(
                    InitializeAccounts {
                        metadata: ix.accounts[0],
                        update_authority: ix.accounts[1],
                        mint: ix.accounts[2],
                        mint_authority: ix.accounts[3],
                    },
                    data,
                ))
            },
            TokenMetadataInstruction::UpdateField(data) => {
                check_min_accounts_req(accounts_len, 2)?;

                Ok(TokenMetadataIx::UpdateField(
                    UpdateFieldAccounts {
                        metadata: ix.accounts[0],
                        update_authority: ix.accounts[1],
                    },
                    data,
                ))
            },

            TokenMetadataInstruction::RemoveKey(data) => {
                check_min_accounts_req(accounts_len, 2)?;

                Ok(TokenMetadataIx::RemoveKey(
                    RmoveKeyAccounts {
                        metadata: ix.accounts[0],
                        update_authority: ix.accounts[1],
                    },
                    data,
                ))
            },

            TokenMetadataInstruction::UpdateAuthority(data) => {
                check_min_accounts_req(accounts_len, 2)?;

                Ok(TokenMetadataIx::UpdateAuthority(
                    UpdateAuthorityAccounts {
                        metadata: ix.accounts[0],
                        current_update_authority: ix.accounts[1],
                    },
                    data,
                ))
            },

            TokenMetadataInstruction::Emit(data) => {
                check_min_accounts_req(accounts_len, 1)?;

                Ok(TokenMetadataIx::Emit(
                    EmitAccounts {
                        metadata: ix.accounts[0],
                    },
                    data,
                ))
            },
        }
    }
}

#[cfg(feature = "proto")]
mod proto_parser {
    use token_metadata_ix_proto::IxOneof;
    use yellowstone_vixen_proto::parser::{
        token_metadata_ix_proto, EmitAccountsProto, EmitDataProto, EmitIxProto,
        InitializeAccountsProto, InitializeDataProto, InitializeIxProto, RemoveKeyDataProto,
        RemoveKeyIxProto, RmoveKeyAccountsProto, TokenMetadataIxProto,
        UpdateAuthorityAccountsProto, UpdateAuthorityDataProto, UpdateAuthorityIxProto,
        UpdateFieldAccountsProto, UpdateFieldDataProto, UpdateFieldIxProto,
    };

    use super::{
        EmitAccounts, InitializeAccounts, RmoveKeyAccounts, TokenMetadataIx,
        UpdateAuthorityAccounts, UpdateFieldAccounts,
    };
    use crate::helpers::IntoProto;

    impl IntoProto<InitializeAccountsProto> for InitializeAccounts {
        fn into_proto(self) -> InitializeAccountsProto {
            InitializeAccountsProto {
                metadata: self.metadata.to_string(),
                update_authority: self.update_authority.to_string(),
                mint: self.mint.to_string(),
                mint_authority: self.mint_authority.to_string(),
            }
        }
    }

    impl IntoProto<UpdateFieldAccountsProto> for UpdateFieldAccounts {
        fn into_proto(self) -> UpdateFieldAccountsProto {
            UpdateFieldAccountsProto {
                metadata: self.metadata.to_string(),
                update_authority: self.update_authority.to_string(),
            }
        }
    }

    impl IntoProto<RmoveKeyAccountsProto> for RmoveKeyAccounts {
        fn into_proto(self) -> RmoveKeyAccountsProto {
            RmoveKeyAccountsProto {
                metadata: self.metadata.to_string(),
                update_authority: self.update_authority.to_string(),
            }
        }
    }

    impl IntoProto<UpdateAuthorityAccountsProto> for UpdateAuthorityAccounts {
        fn into_proto(self) -> UpdateAuthorityAccountsProto {
            UpdateAuthorityAccountsProto {
                metadata: self.metadata.to_string(),
                current_update_authority: self.current_update_authority.to_string(),
            }
        }
    }

    impl IntoProto<EmitAccountsProto> for EmitAccounts {
        fn into_proto(self) -> EmitAccountsProto {
            EmitAccountsProto {
                metadata: self.metadata.to_string(),
            }
        }
    }

    impl IntoProto<InitializeDataProto> for spl_token_metadata_interface::instruction::Initialize {
        fn into_proto(self) -> InitializeDataProto {
            InitializeDataProto {
                name: self.name,
                symbol: self.symbol,
                uri: self.uri,
            }
        }
    }

    fn from_field_to_string(field: spl_token_metadata_interface::state::Field) -> String {
        match field {
            spl_token_metadata_interface::state::Field::Name => "Name".to_string(),
            spl_token_metadata_interface::state::Field::Symbol => "Symbol".to_string(),
            spl_token_metadata_interface::state::Field::Uri => "Uri".to_string(),
            spl_token_metadata_interface::state::Field::Key(key) => key,
        }
    }

    impl IntoProto<UpdateFieldDataProto> for spl_token_metadata_interface::instruction::UpdateField {
        fn into_proto(self) -> UpdateFieldDataProto {
            UpdateFieldDataProto {
                key: from_field_to_string(self.field),
                value: self.value,
            }
        }
    }

    impl IntoProto<RemoveKeyDataProto> for spl_token_metadata_interface::instruction::RemoveKey {
        fn into_proto(self) -> RemoveKeyDataProto {
            RemoveKeyDataProto {
                idempotent: self.idempotent,
                key: self.key,
            }
        }
    }

    impl IntoProto<UpdateAuthorityDataProto>
        for spl_token_metadata_interface::instruction::UpdateAuthority
    {
        fn into_proto(self) -> UpdateAuthorityDataProto {
            UpdateAuthorityDataProto {
                new_authority: self.new_authority.0.to_string(),
            }
        }
    }

    impl IntoProto<EmitDataProto> for spl_token_metadata_interface::instruction::Emit {
        fn into_proto(self) -> EmitDataProto {
            EmitDataProto {
                start: self.start,
                end: self.end,
            }
        }
    }

    impl IntoProto<TokenMetadataIxProto> for TokenMetadataIx {
        fn into_proto(self) -> TokenMetadataIxProto {
            match self {
                TokenMetadataIx::Initialize(acc, data) => TokenMetadataIxProto {
                    ix_oneof: Some(IxOneof::InitializeIx(InitializeIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },
                TokenMetadataIx::UpdateField(acc, data) => TokenMetadataIxProto {
                    ix_oneof: Some(IxOneof::UpdateFieldsIx(UpdateFieldIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenMetadataIx::RemoveKey(acc, data) => TokenMetadataIxProto {
                    ix_oneof: Some(IxOneof::RemoveKeyIx(RemoveKeyIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenMetadataIx::UpdateAuthority(acc, data) => TokenMetadataIxProto {
                    ix_oneof: Some(IxOneof::UpdateAuthorityIx(UpdateAuthorityIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenMetadataIx::Emit(acc, data) => TokenMetadataIxProto {
                    ix_oneof: Some(IxOneof::EmitIx(EmitIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },
            }
        }
    }
}
