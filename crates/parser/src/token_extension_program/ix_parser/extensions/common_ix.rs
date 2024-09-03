use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};

use super::helpers::{decode_extension_ix_type, Ix};
use crate::helpers::{check_min_accounts_req, get_multisig_signers};

#[derive(Debug, Clone, Copy)]
pub enum ExtensionWithCommonIxs {
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
pub enum IxsSupported {
    InitAndUpdate,
    EnableAndDisable,
}

impl ExtensionWithCommonIxs {
    pub fn get_ixs_supported(extension: &ExtensionWithCommonIxs) -> IxsSupported {
        match extension {
            ExtensionWithCommonIxs::CpiGuard | ExtensionWithCommonIxs::MemoTransfer => {
                IxsSupported::EnableAndDisable
            },
            ExtensionWithCommonIxs::DefaultAccountState
            | ExtensionWithCommonIxs::InterestBearingMint
            | ExtensionWithCommonIxs::GroupMemberPointer
            | ExtensionWithCommonIxs::MetadataPointer
            | ExtensionWithCommonIxs::TransferHook
            | ExtensionWithCommonIxs::GroupPointer => IxsSupported::InitAndUpdate,
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
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug)]
pub struct EnableAccounts {
    pub account: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug)]
pub struct DisableAccounts {
    pub account: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug)]
pub struct CommonExtensionIxs {
    pub extension: ExtensionWithCommonIxs,
    pub ix: CommonIx,
}

#[derive(Debug)]
pub enum CommonIx {
    Initialize(Ix<ExtInitializeAccounts>),
    Update(Ix<UpdateAccounts>),
    Enable(Ix<EnableAccounts>),
    Disable(Ix<DisableAccounts>),
}

impl CommonExtensionIxs {
    pub fn try_parse_extension_ix(
        extension: ExtensionWithCommonIxs,
        ix: &InstructionUpdate,
    ) -> Result<Self, String> {
        let ix_type = decode_extension_ix_type(&ix.data)?;
        let accounts_len = ix.accounts.len();
        match ExtensionWithCommonIxs::get_ixs_supported(&extension) {
            IxsSupported::InitAndUpdate => match ix_type {
                0 => {
                    check_min_accounts_req(accounts_len, 1)?;
                    Ok(CommonExtensionIxs {
                        extension,
                        ix: CommonIx::Initialize(Ix::from_accounts(ExtInitializeAccounts {
                            mint: ix.accounts[0],
                        })),
                    })
                },
                1 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    Ok(CommonExtensionIxs {
                        extension,
                        ix: CommonIx::Update(Ix::from_accounts(UpdateAccounts {
                            mint: ix.accounts[0],
                            extension_authority: ix.accounts[1],
                            multisig_signers: get_multisig_signers(ix, 2),
                        })),
                    })
                },
                _ => return Err("Invalid instruction".to_string()),
            },
            IxsSupported::EnableAndDisable => match ix_type {
                0 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    Ok(CommonExtensionIxs {
                        extension,
                        ix: CommonIx::Enable(Ix::from_accounts(EnableAccounts {
                            account: ix.accounts[0],
                            owner: ix.accounts[1],
                            multisig_signers: get_multisig_signers(ix, 2),
                        })),
                    })
                },
                1 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    Ok(CommonExtensionIxs {
                        extension,
                        ix: CommonIx::Disable(Ix::from_accounts(DisableAccounts {
                            account: ix.accounts[0],
                            owner: ix.accounts[1],
                            multisig_signers: ix.accounts.get(2..).map(|a| a.to_vec()),
                        })),
                    })
                },
                _ => return Err("Invalid instruction".to_string()),
            },
        }
    }
}
