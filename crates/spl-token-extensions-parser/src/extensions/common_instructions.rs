use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Error, Result};
use yellowstone_vixen_proc_macro::vixen_proto;

use super::extension::decode_extension_ix_type;
use crate::PubkeyBytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionWithCommonInstruction {
    CpiGuard,
    DefaultAccountState,
    InterestBearingMint,
    MemoTransfer,
    GroupMemberPointer,
    GroupPointer,
    MetadataPointer,
    TransferHook,
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

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ExtInitializeAccounts {
    pub mint: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UpdateAccounts {
    pub mint: PubkeyBytes,
    pub extension_authority: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct EnableAccounts {
    pub account: PubkeyBytes,
    pub owner: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DisableAccounts {
    pub account: PubkeyBytes,
    pub owner: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto(enumeration)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ExtensionWithCommonInstructionProto {
    CpiGuard = 0,
    DefaultAccountState = 1,
    InterestBearingMint = 2,
    MemoTransfer = 3,
    GroupMemberPointer = 4,
    GroupPointer = 5,
    MetadataPointer = 6,
    TransferHook = 7,
}

#[inline]
fn extension_to_proto(e: ExtensionWithCommonInstruction) -> i32 {
    use ExtensionWithCommonInstruction as E;
    use ExtensionWithCommonInstructionProto as P;

    match e {
        E::CpiGuard => P::CpiGuard as i32,
        E::DefaultAccountState => P::DefaultAccountState as i32,
        E::InterestBearingMint => P::InterestBearingMint as i32,
        E::MemoTransfer => P::MemoTransfer as i32,
        E::GroupMemberPointer => P::GroupMemberPointer as i32,
        E::GroupPointer => P::GroupPointer as i32,
        E::MetadataPointer => P::MetadataPointer as i32,
        E::TransferHook => P::TransferHook as i32,
    }
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct CommonExtensionInstructions {
    #[vixen_proto_hint(enumeration = "ExtensionWithCommonInstructionProto")]
    pub extension: i32,

    #[vixen_proto_hint(oneof = "common_extension_instructions::Ix", tags = "2, 3, 4, 5")]
    pub ix: Option<common_extension_instructions::Ix>,
}

pub mod common_extension_instructions {
    use super::vixen_proto;

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct Initialize {
        pub accounts: Option<super::ExtInitializeAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct Update {
        pub accounts: Option<super::UpdateAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct Enable {
        pub accounts: Option<super::EnableAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct Disable {
        pub accounts: Option<super::DisableAccounts>,
    }

    #[vixen_proto(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Ix {
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
                        extension: extension_to_proto(extension),
                        ix: Some(oneof::Ix::Initialize(oneof::Initialize {
                            accounts: Some(ExtInitializeAccounts {
                                mint: ix.accounts[0].to_vec(),
                            }),
                        })),
                    }
                },
                1 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    CommonExtensionInstructions {
                        extension: extension_to_proto(extension),
                        ix: Some(oneof::Ix::Update(oneof::Update {
                            accounts: Some(UpdateAccounts {
                                mint: ix.accounts[0].to_vec(),
                                extension_authority: ix.accounts[1].to_vec(),
                                multisig_signers: ix.accounts[2..]
                                    .iter()
                                    .map(|pk| pk.to_vec())
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
                        extension: extension_to_proto(extension),
                        ix: Some(oneof::Ix::Enable(oneof::Enable {
                            accounts: Some(EnableAccounts {
                                account: ix.accounts[0].to_vec(),
                                owner: ix.accounts[1].to_vec(),
                                multisig_signers: ix.accounts[2..]
                                    .iter()
                                    .map(|pk| pk.to_vec())
                                    .collect(),
                            }),
                        })),
                    }
                },
                1 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    CommonExtensionInstructions {
                        extension: extension_to_proto(extension),
                        ix: Some(oneof::Ix::Disable(oneof::Disable {
                            accounts: Some(DisableAccounts {
                                account: ix.accounts[0].to_vec(),
                                owner: ix.accounts[1].to_vec(),
                                multisig_signers: ix.accounts[2..]
                                    .iter()
                                    .map(|pk| pk.to_vec())
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
