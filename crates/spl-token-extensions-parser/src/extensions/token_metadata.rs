use spl_token_metadata_interface::instruction::TokenMetadataInstruction as SplTokenMetadataInstruction;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};
use yellowstone_vixen_proc_macro::vixen;

use super::extension::ExtensionInstructionParser;
use crate::Pubkey;

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeAccounts {
    pub metadata: Pubkey,
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateFieldAccounts {
    pub metadata: Pubkey,
    pub update_authority: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct RemoveKeyAccounts {
    pub metadata: Pubkey,
    pub update_authority: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateAuthorityAccounts {
    pub metadata: Pubkey,
    pub current_update_authority: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct EmitAccounts {
    pub metadata: Pubkey,
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
        pub accounts: super::InitializeAccounts,
        pub args: super::InitializeArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct UpdateField {
        pub accounts: super::UpdateFieldAccounts,
        pub args: super::UpdateFieldArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct RemoveKey {
        pub accounts: super::RemoveKeyAccounts,
        pub args: super::RemoveKeyArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct UpdateAuthority {
        pub accounts: super::UpdateAuthorityAccounts,
        pub args: super::UpdateAuthorityArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Emit {
        pub accounts: super::EmitAccounts,
        pub args: super::EmitArgs,
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
                    accounts: InitializeAccounts {
                        metadata: crate::Pubkey::new(ix.accounts[0].0),
                        update_authority: crate::Pubkey::new(ix.accounts[1].0),
                        mint: crate::Pubkey::new(ix.accounts[2].0),
                        mint_authority: crate::Pubkey::new(ix.accounts[3].0),
                    },
                    args: InitializeArgs {
                        raw: ix.data.clone(),
                    },
                })
            },
            SplTokenMetadataInstruction::UpdateField(_args) => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::UpdateField(oneof::UpdateField {
                    accounts: UpdateFieldAccounts {
                        metadata: crate::Pubkey::new(ix.accounts[0].0),
                        update_authority: crate::Pubkey::new(ix.accounts[1].0),
                    },
                    args: UpdateFieldArgs {
                        raw: ix.data.clone(),
                    },
                })
            },
            SplTokenMetadataInstruction::RemoveKey(_args) => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::RemoveKey(oneof::RemoveKey {
                    accounts: RemoveKeyAccounts {
                        metadata: crate::Pubkey::new(ix.accounts[0].0),
                        update_authority: crate::Pubkey::new(ix.accounts[1].0),
                    },
                    args: RemoveKeyArgs {
                        raw: ix.data.clone(),
                    },
                })
            },
            SplTokenMetadataInstruction::UpdateAuthority(_args) => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::UpdateAuthority(oneof::UpdateAuthority {
                    accounts: UpdateAuthorityAccounts {
                        metadata: crate::Pubkey::new(ix.accounts[0].0),
                        current_update_authority: crate::Pubkey::new(ix.accounts[1].0),
                    },
                    args: UpdateAuthorityArgs {
                        raw: ix.data.clone(),
                    },
                })
            },
            SplTokenMetadataInstruction::Emit(_args) => {
                check_min_accounts_req(accounts_len, 1)?;

                oneof::Instruction::Emit(oneof::Emit {
                    accounts: EmitAccounts {
                        metadata: crate::Pubkey::new(ix.accounts[0].0),
                    },
                    args: EmitArgs {
                        raw: ix.data.clone(),
                    },
                })
            },
        };

        Ok(TokenMetadataIx {
            instruction: Some(msg),
        })
    }
}
