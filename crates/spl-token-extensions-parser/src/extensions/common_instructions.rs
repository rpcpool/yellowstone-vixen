use shipstern_core::instruction::InstructionUpdate;
use shipstern_parser::{check_min_accounts_req, Error, Result};
use shipstern_proc_macro::shipstern;

use super::extension::decode_extension_ix_type;
use crate::Pubkey;

#[shipstern(enumeration)]
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

#[shipstern]
#[derive(Clone, PartialEq)]
pub struct ExtInitializeAccounts {
    pub mint: Pubkey,
}

#[shipstern]
#[derive(Clone, PartialEq)]
pub struct UpdateAccounts {
    pub mint: Pubkey,
    pub extension_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[shipstern]
#[derive(Clone, PartialEq)]
pub struct EnableAccounts {
    pub account: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[shipstern]
#[derive(Clone, PartialEq)]
pub struct DisableAccounts {
    pub account: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[shipstern]
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
    use super::shipstern;

    #[shipstern]
    #[derive(Clone, PartialEq)]
    pub struct Initialize {
        pub accounts: super::ExtInitializeAccounts,
    }

    #[shipstern]
    #[derive(Clone, PartialEq)]
    pub struct Update {
        pub accounts: super::UpdateAccounts,
    }

    #[shipstern]
    #[derive(Clone, PartialEq)]
    pub struct Enable {
        pub accounts: super::EnableAccounts,
    }

    #[shipstern]
    #[derive(Clone, PartialEq)]
    pub struct Disable {
        pub accounts: super::DisableAccounts,
    }

    #[shipstern(oneof)]
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
                            accounts: ExtInitializeAccounts {
                                mint: crate::Pubkey::new(ix.accounts[0].0),
                            },
                        })),
                    }
                },
                1 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    CommonExtensionInstructions {
                        extension: extension as i32,
                        instruction: Some(oneof::Instruction::Update(oneof::Update {
                            accounts: UpdateAccounts {
                                mint: crate::Pubkey::new(ix.accounts[0].0),
                                extension_authority: crate::Pubkey::new(ix.accounts[1].0),
                                multisig_signers: ix.accounts[2..]
                                    .iter()
                                    .map(|a| crate::Pubkey::new(a.0))
                                    .collect(),
                            },
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
                            accounts: EnableAccounts {
                                account: crate::Pubkey::new(ix.accounts[0].0),
                                owner: crate::Pubkey::new(ix.accounts[1].0),
                                multisig_signers: ix.accounts[2..]
                                    .iter()
                                    .map(|a| crate::Pubkey::new(a.0))
                                    .collect(),
                            },
                        })),
                    }
                },
                1 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    CommonExtensionInstructions {
                        extension: extension as i32,
                        instruction: Some(oneof::Instruction::Disable(oneof::Disable {
                            accounts: DisableAccounts {
                                account: crate::Pubkey::new(ix.accounts[0].0),
                                owner: crate::Pubkey::new(ix.accounts[1].0),
                                multisig_signers: ix.accounts[2..]
                                    .iter()
                                    .map(|a| crate::Pubkey::new(a.0))
                                    .collect(),
                            },
                        })),
                    }
                },
                _ => return Err(Error::new("Invalid instruction")),
            },
        };

        Ok(msg)
    }
}
