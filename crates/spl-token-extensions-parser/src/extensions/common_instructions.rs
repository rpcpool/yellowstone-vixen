use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Error, Result};
use yellowstone_vixen_proc_macro::vixen;

use super::extension::decode_extension_ix_type;
use crate::PublicKey;

#[vixen(enumeration)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ExtensionWithCommonInstruction {
    CpiGuard = 0,
    DefaultAccountState = 1,
    InterestBearingMint = 2,
    MemoTransfer = 3,
    GroupMemberPointer = 4,
    GroupPointer = 5,
    MetadataPointer = 6,
    TransferHook = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionSupported {
    InitAndUpdate,
    EnableAndDisable,
}

impl ExtensionWithCommonInstruction {
    #[must_use]
    pub fn get_instructions_supported(extension: &Self) -> InstructionSupported {
        match extension {
            ExtensionWithCommonInstruction::CpiGuard
            | ExtensionWithCommonInstruction::MemoTransfer => {
                InstructionSupported::EnableAndDisable
            },
            ExtensionWithCommonInstruction::DefaultAccountState
            | ExtensionWithCommonInstruction::InterestBearingMint
            | ExtensionWithCommonInstruction::GroupMemberPointer
            | ExtensionWithCommonInstruction::MetadataPointer
            | ExtensionWithCommonInstruction::TransferHook
            | ExtensionWithCommonInstruction::GroupPointer => InstructionSupported::InitAndUpdate,
        }
    }
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ExtInitializeAccounts {
    pub mint: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateAccounts {
    pub mint: PublicKey,
    pub extension_authority: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct EnableAccounts {
    pub account: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DisableAccounts {
    pub account: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct CommonExtensionInstructions {
    #[hint(enumeration = "ExtensionWithCommonInstruction")]
    pub extension: i32,

    #[hint(
        oneof = "common_extension_instructions::Instruction",
        tags = "2, 3, 4, 5"
    )]
    pub instruction: Option<common_extension_instructions::Instruction>,
}

pub mod common_extension_instructions {
    use super::vixen;

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Initialize {
        pub accounts: Option<super::ExtInitializeAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Update {
        pub accounts: Option<super::UpdateAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Enable {
        pub accounts: Option<super::EnableAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Disable {
        pub accounts: Option<super::DisableAccounts>,
    }

    #[vixen(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Instruction {
        Initialize(Initialize),
        Update(Update),
        Enable(Enable),
        Disable(Disable),
    }
}

impl CommonExtensionInstructions {
    pub fn try_parse_extension_instruction(
        extension: ExtensionWithCommonInstruction,
        ix: &InstructionUpdate,
    ) -> Result<Self> {
        let ix_type: u8 = decode_extension_ix_type(&ix.data[1..])?;
        let accounts_len = ix.accounts.len();

        use common_extension_instructions as oneof;

        let msg = match ExtensionWithCommonInstruction::get_instructions_supported(&extension) {
            InstructionSupported::InitAndUpdate => match ix_type {
                0 => {
                    check_min_accounts_req(accounts_len, 1)?;
                    CommonExtensionInstructions {
                        extension: extension as i32,
                        instruction: Some(oneof::Instruction::Initialize(oneof::Initialize {
                            accounts: Some(ExtInitializeAccounts {
                                mint: crate::PublicKey::new(ix.accounts[0].to_vec()),
                            }),
                        })),
                    }
                },
                1 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    CommonExtensionInstructions {
                        extension: extension as i32,
                        instruction: Some(oneof::Instruction::Update(oneof::Update {
                            accounts: Some(UpdateAccounts {
                                mint: crate::PublicKey::new(ix.accounts[0].to_vec()),
                                extension_authority: crate::PublicKey::new(ix.accounts[1].to_vec()),
                                multisig_signers: ix.accounts[2..]
                                    .iter()
                                    .map(|a| crate::PublicKey::new(a.to_vec()))
                                    .collect(),
                            }),
                        })),
                    }
                },
                _ => return Err(Error::new("Invalid instruction")),
            },

            InstructionSupported::EnableAndDisable => match ix_type {
                0 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    CommonExtensionInstructions {
                        extension: extension as i32,
                        instruction: Some(oneof::Instruction::Enable(oneof::Enable {
                            accounts: Some(EnableAccounts {
                                account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                                owner: crate::PublicKey::new(ix.accounts[1].to_vec()),
                                multisig_signers: ix.accounts[2..]
                                    .iter()
                                    .map(|a| crate::PublicKey::new(a.to_vec()))
                                    .collect(),
                            }),
                        })),
                    }
                },
                1 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    CommonExtensionInstructions {
                        extension: extension as i32,
                        instruction: Some(oneof::Instruction::Disable(oneof::Disable {
                            accounts: Some(DisableAccounts {
                                account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                                owner: crate::PublicKey::new(ix.accounts[1].to_vec()),
                                multisig_signers: ix.accounts[2..]
                                    .iter()
                                    .map(|a| crate::PublicKey::new(a.to_vec()))
                                    .collect(),
                            }),
                        })),
                    }
                },
                _ => return Err(Error::new("Invalid instruction")),
            },
        };

        Ok(msg)
    }
}
