use spl_token_metadata_interface::instruction::TokenMetadataInstruction as SplTokenMetadataInstruction;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};
use yellowstone_vixen_proc_macro::vixen;

use super::extension::ExtensionInstructionParser;
use crate::PublicKey;

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeAccounts {
    pub metadata: PublicKey,
    pub update_authority: PublicKey,
    pub mint: PublicKey,
    pub mint_authority: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateFieldAccounts {
    pub metadata: PublicKey,
    pub update_authority: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct RemoveKeyAccounts {
    pub metadata: PublicKey,
    pub update_authority: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateAuthorityAccounts {
    pub metadata: PublicKey,
    pub current_update_authority: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct EmitAccounts {
    pub metadata: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeArgs {
    pub raw: Vec<u8>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateFieldArgs {
    pub raw: Vec<u8>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct RemoveKeyArgs {
    pub raw: Vec<u8>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateAuthorityArgs {
    pub raw: Vec<u8>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct EmitArgs {
    pub raw: Vec<u8>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct TokenMetadataIx {
    #[hint(
        oneof = "token_metadata_instruction::Instruction",
        tags = "1, 2, 3, 4, 5"
    )]
    pub instruction: Option<token_metadata_instruction::Instruction>,
}

pub mod token_metadata_instruction {
    use super::vixen;

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Initialize {
        pub accounts: Option<super::InitializeAccounts>,
        pub args: Option<super::InitializeArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct UpdateField {
        pub accounts: Option<super::UpdateFieldAccounts>,
        pub args: Option<super::UpdateFieldArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct RemoveKey {
        pub accounts: Option<super::RemoveKeyAccounts>,
        pub args: Option<super::RemoveKeyArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct UpdateAuthority {
        pub accounts: Option<super::UpdateAuthorityAccounts>,
        pub args: Option<super::UpdateAuthorityArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Emit {
        pub accounts: Option<super::EmitAccounts>,
        pub args: Option<super::EmitArgs>,
    }

    #[vixen(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Instruction {
        Initialize(Initialize),
        UpdateField(UpdateField),
        RemoveKey(RemoveKey),
        UpdateAuthority(UpdateAuthority),
        Emit(Emit),
    }
}

impl ExtensionInstructionParser for TokenMetadataIx {
    fn try_parse(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();

        let ix_type = SplTokenMetadataInstruction::unpack(&ix.data)
            .parse_err("Error unpacking token metadata instruction data")?;

        use token_metadata_instruction as oneof;

        let msg = match ix_type {
            SplTokenMetadataInstruction::Initialize(_args) => {
                check_min_accounts_req(accounts_len, 4)?;

                oneof::Instruction::Initialize(oneof::Initialize {
                    accounts: Some(InitializeAccounts {
                        metadata: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        update_authority: crate::PublicKey::new(ix.accounts[1].to_vec()),
                        mint: crate::PublicKey::new(ix.accounts[2].to_vec()),
                        mint_authority: crate::PublicKey::new(ix.accounts[3].to_vec()),
                    }),
                    args: Some(InitializeArgs {
                        raw: ix.data.clone(),
                    }),
                })
            },
            SplTokenMetadataInstruction::UpdateField(_args) => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::UpdateField(oneof::UpdateField {
                    accounts: Some(UpdateFieldAccounts {
                        metadata: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        update_authority: crate::PublicKey::new(ix.accounts[1].to_vec()),
                    }),
                    args: Some(UpdateFieldArgs {
                        raw: ix.data.clone(),
                    }),
                })
            },
            SplTokenMetadataInstruction::RemoveKey(_args) => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::RemoveKey(oneof::RemoveKey {
                    accounts: Some(RemoveKeyAccounts {
                        metadata: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        update_authority: crate::PublicKey::new(ix.accounts[1].to_vec()),
                    }),
                    args: Some(RemoveKeyArgs {
                        raw: ix.data.clone(),
                    }),
                })
            },
            SplTokenMetadataInstruction::UpdateAuthority(_args) => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::UpdateAuthority(oneof::UpdateAuthority {
                    accounts: Some(UpdateAuthorityAccounts {
                        metadata: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        current_update_authority: crate::PublicKey::new(ix.accounts[1].to_vec()),
                    }),
                    args: Some(UpdateAuthorityArgs {
                        raw: ix.data.clone(),
                    }),
                })
            },
            SplTokenMetadataInstruction::Emit(_args) => {
                check_min_accounts_req(accounts_len, 1)?;

                oneof::Instruction::Emit(oneof::Emit {
                    accounts: Some(EmitAccounts {
                        metadata: crate::PublicKey::new(ix.accounts[0].to_vec()),
                    }),
                    args: Some(EmitArgs {
                        raw: ix.data.clone(),
                    }),
                })
            },
        };

        Ok(TokenMetadataIx {
            instruction: Some(msg),
        })
    }
}
