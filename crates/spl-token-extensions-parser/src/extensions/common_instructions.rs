use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};
use yellowstone_vixen_parser::{check_min_accounts_req, Error, Result};

use super::extension::decode_extension_ix_type;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct ExtInitializeAccounts {
    pub mint: Pubkey,
}

#[derive(Debug)]
pub struct UpdateAccounts {
    pub mint: Pubkey,
    pub extension_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct EnableAccounts {
    pub account: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct DisableAccounts {
    pub account: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct CommonExtensionInstructions {
    pub extension: ExtensionWithCommonInstruction,
    pub instruction: CommonInstruction,
}

#[derive(Debug)]
pub enum CommonInstruction {
    Initialize { accounts: ExtInitializeAccounts },
    Update { accounts: UpdateAccounts },
    Enable { accounts: EnableAccounts },
    Disable { accounts: DisableAccounts },
}

impl CommonExtensionInstructions {
    pub fn try_parse_extension_instruction(
        extension: ExtensionWithCommonInstruction,
        ix: &InstructionUpdate,
    ) -> Result<Self> {
        let ix_type: u8 = decode_extension_ix_type(&ix.data[1..])?;
        let accounts_len = ix.accounts.len();
        match ExtensionWithCommonInstruction::get_instructions_supported(&extension) {
            InstructionSupported::InitAndUpdate => match ix_type {
                0 => {
                    check_min_accounts_req(accounts_len, 1)?;
                    Ok(CommonExtensionInstructions {
                        extension,
                        instruction: CommonInstruction::Initialize {
                            accounts: ExtInitializeAccounts {
                                mint: ix.accounts[0],
                            },
                        },
                    })
                },
                1 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    Ok(CommonExtensionInstructions {
                        extension,
                        instruction: CommonInstruction::Update {
                            accounts: UpdateAccounts {
                                mint: ix.accounts[0],
                                extension_authority: ix.accounts[1],
                                multisig_signers: ix.accounts[2..].to_vec(),
                            },
                        },
                    })
                },
                _ => Err(Error::new("Invalid instruction")),
            },
            InstructionSupported::EnableAndDisable => match ix_type {
                0 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    Ok(CommonExtensionInstructions {
                        extension,
                        instruction: CommonInstruction::Enable {
                            accounts: EnableAccounts {
                                account: ix.accounts[0],
                                owner: ix.accounts[1],
                                multisig_signers: ix.accounts[2..].to_vec(),
                            },
                        },
                    })
                },
                1 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    Ok(CommonExtensionInstructions {
                        extension,
                        instruction: CommonInstruction::Disable {
                            accounts: DisableAccounts {
                                account: ix.accounts[0],
                                owner: ix.accounts[1],
                                multisig_signers: ix.accounts[2..].to_vec(),
                            },
                        },
                    })
                },
                _ => Err(Error::new("Invalid instruction")),
            },
        }
    }
}
