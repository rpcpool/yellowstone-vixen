use prost::alloc::vec::Vec;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Error, Result};

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

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtInitializeAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub extension_authority: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ::prost::Enumeration)]
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

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonExtensionInstructions {
    #[prost(enumeration = "ExtensionWithCommonInstructionProto", tag = "1")]
    pub extension: i32,

    #[prost(oneof = "common_extension_instructions::Ix", tags = "2, 3, 4, 5")]
    pub ix: Option<common_extension_instructions::Ix>,
}

pub mod common_extension_instructions {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Initialize {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::ExtInitializeAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Update {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::UpdateAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Enable {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::EnableAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Disable {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::DisableAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ix {
        #[prost(message, tag = "2")]
        Initialize(Initialize),
        #[prost(message, tag = "3")]
        Update(Update),
        #[prost(message, tag = "4")]
        Enable(Enable),
        #[prost(message, tag = "5")]
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
