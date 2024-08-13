use spl_pod::solana_program::pubkey::Pubkey;

use super::helpers::{decode_extension_ix_type, Ix};
use crate::ix_parser::vixen_ix::{helpers::check_min_accounts_req, structure::InstructionUpdate};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct InitializeAccounts {
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
pub struct CommonExtIxs {
    pub extension: ExtensionWithCommonIxs,
    pub ix: CommonIx,
}

#[derive(Debug)]
pub enum CommonIx {
    Initialize(Ix<InitializeAccounts>),
    Update(Ix<UpdateAccounts>),
    Enable(Ix<EnableAccounts>),
    Disable(Ix<DisableAccounts>),
}

impl CommonExtIxs {
    pub fn try_parse_extension_ix(
        extension: ExtensionWithCommonIxs,
        ix_data: &InstructionUpdate,
    ) -> Result<Self, String> {
        let ix_type = decode_extension_ix_type(&ix_data.data)?;
        match ExtensionWithCommonIxs::get_ixs_supported(&extension) {
            IxsSupported::InitAndUpdate => match ix_type {
                0 => {
                    check_min_accounts_req(ix_data.accounts.len(), 1)?;
                    Ok(CommonExtIxs {
                        extension,
                        ix: CommonIx::Initialize(Ix::from_accounts(InitializeAccounts {
                            mint: ix_data.accounts[0],
                        })),
                    })
                },
                1 => {
                    check_min_accounts_req(ix_data.accounts.len(), 2)?;
                    Ok(CommonExtIxs {
                        extension,
                        ix: CommonIx::Update(Ix::from_accounts(UpdateAccounts {
                            mint: ix_data.accounts[0],
                            extension_authority: ix_data.accounts[1],
                            multisig_signers: ix_data.accounts.get(2..).map(|a| a.to_vec()),
                        })),
                    })
                },
                _ => return Err("Invalid instruction".to_string()),
            },
            IxsSupported::EnableAndDisable => match ix_type {
                0 => {
                    check_min_accounts_req(ix_data.accounts.len(), 2)?;
                    Ok(CommonExtIxs {
                        extension,
                        ix: CommonIx::Enable(Ix::from_accounts(EnableAccounts {
                            account: ix_data.accounts[0],
                            owner: ix_data.accounts[1],
                            multisig_signers: None,
                        })),
                    })
                },
                1 => {
                    check_min_accounts_req(ix_data.accounts.len(), 2)?;
                    Ok(CommonExtIxs {
                        extension,
                        ix: CommonIx::Disable(Ix::from_accounts(DisableAccounts {
                            account: ix_data.accounts[0],
                            owner: ix_data.accounts[1],
                            multisig_signers: ix_data.accounts.get(2..).map(|a| a.to_vec()),
                        })),
                    })
                },
                _ => return Err("Invalid instruction".to_string()),
            },
        }
    }
}
