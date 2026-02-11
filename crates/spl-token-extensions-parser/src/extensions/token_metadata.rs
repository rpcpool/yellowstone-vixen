use prost::alloc::vec::Vec;
use spl_token_metadata_interface::instruction::TokenMetadataInstruction as SplTokenMetadataInstruction;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};

use super::extension::ExtensionInstructionParser;
use crate::PubkeyBytes;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub metadata: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub update_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub mint_authority: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFieldAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub metadata: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub update_authority: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveKeyAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub metadata: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub update_authority: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAuthorityAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub metadata: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub current_update_authority: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub metadata: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeArgs {
    #[prost(bytes = "vec", tag = "1")]
    pub raw: Vec<u8>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFieldArgs {
    #[prost(bytes = "vec", tag = "1")]
    pub raw: Vec<u8>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveKeyArgs {
    #[prost(bytes = "vec", tag = "1")]
    pub raw: Vec<u8>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAuthorityArgs {
    #[prost(bytes = "vec", tag = "1")]
    pub raw: Vec<u8>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitArgs {
    #[prost(bytes = "vec", tag = "1")]
    pub raw: Vec<u8>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenMetadataInstruction {
    #[prost(oneof = "token_metadata_instruction::Ix", tags = "1, 2, 3, 4, 5")]
    pub ix: Option<token_metadata_instruction::Ix>,
}

pub mod token_metadata_instruction {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Initialize {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::InitializeAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::InitializeArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateField {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::UpdateFieldAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::UpdateFieldArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RemoveKey {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::RemoveKeyAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::RemoveKeyArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateAuthority {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::UpdateAuthorityAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::UpdateAuthorityArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Emit {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::EmitAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::EmitArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ix {
        #[prost(message, tag = "1")]
        Initialize(Initialize),
        #[prost(message, tag = "2")]
        UpdateField(UpdateField),
        #[prost(message, tag = "3")]
        RemoveKey(RemoveKey),
        #[prost(message, tag = "4")]
        UpdateAuthority(UpdateAuthority),
        #[prost(message, tag = "5")]
        Emit(Emit),
    }
}

impl ExtensionInstructionParser for TokenMetadataInstruction {
    fn try_parse(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();

        let ix_type = SplTokenMetadataInstruction::unpack(&ix.data)
            .parse_err("Error unpacking token metadata instruction data")?;

        use token_metadata_instruction as oneof;

        let msg = match ix_type {
            SplTokenMetadataInstruction::Initialize(_args) => {
                check_min_accounts_req(accounts_len, 4)?;

                oneof::Ix::Initialize(oneof::Initialize {
                    accounts: Some(InitializeAccounts {
                        metadata: ix.accounts[0].to_vec(),
                        update_authority: ix.accounts[1].to_vec(),
                        mint: ix.accounts[2].to_vec(),
                        mint_authority: ix.accounts[3].to_vec(),
                    }),
                    args: Some(InitializeArgs {
                        raw: ix.data.clone(),
                    }),
                })
            },
            SplTokenMetadataInstruction::UpdateField(_args) => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Ix::UpdateField(oneof::UpdateField {
                    accounts: Some(UpdateFieldAccounts {
                        metadata: ix.accounts[0].to_vec(),
                        update_authority: ix.accounts[1].to_vec(),
                    }),
                    args: Some(UpdateFieldArgs {
                        raw: ix.data.clone(),
                    }),
                })
            },
            SplTokenMetadataInstruction::RemoveKey(_args) => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Ix::RemoveKey(oneof::RemoveKey {
                    accounts: Some(RemoveKeyAccounts {
                        metadata: ix.accounts[0].to_vec(),
                        update_authority: ix.accounts[1].to_vec(),
                    }),
                    args: Some(RemoveKeyArgs {
                        raw: ix.data.clone(),
                    }),
                })
            },
            SplTokenMetadataInstruction::UpdateAuthority(_args) => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Ix::UpdateAuthority(oneof::UpdateAuthority {
                    accounts: Some(UpdateAuthorityAccounts {
                        metadata: ix.accounts[0].to_vec(),
                        current_update_authority: ix.accounts[1].to_vec(),
                    }),
                    args: Some(UpdateAuthorityArgs {
                        raw: ix.data.clone(),
                    }),
                })
            },
            SplTokenMetadataInstruction::Emit(_args) => {
                check_min_accounts_req(accounts_len, 1)?;

                oneof::Ix::Emit(oneof::Emit {
                    accounts: Some(EmitAccounts {
                        metadata: ix.accounts[0].to_vec(),
                    }),
                    args: Some(EmitArgs {
                        raw: ix.data.clone(),
                    }),
                })
            },
        };

        Ok(TokenMetadataInstruction { ix: Some(msg) })
    }
}
