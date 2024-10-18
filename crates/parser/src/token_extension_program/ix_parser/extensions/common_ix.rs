use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};

use super::helpers::decode_extension_ix_type;
use crate::{helpers::check_min_accounts_req, Error, Result};

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
    #[must_use]
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
pub struct CommonExtensionIxs {
    pub extension: ExtensionWithCommonIxs,
    pub ix: CommonIx,
}

#[derive(Debug)]
pub enum CommonIx {
    Initialize(ExtInitializeAccounts),
    Update(UpdateAccounts),
    Enable(EnableAccounts),
    Disable(DisableAccounts),
}

impl CommonExtensionIxs {
    pub fn try_parse_extension_ix(
        extension: ExtensionWithCommonIxs,
        ix: &InstructionUpdate,
    ) -> Result<Self> {
        let ix_type = decode_extension_ix_type(&ix.data)?;
        let accounts_len = ix.accounts.len();
        match ExtensionWithCommonIxs::get_ixs_supported(&extension) {
            IxsSupported::InitAndUpdate => match ix_type {
                0 => {
                    check_min_accounts_req(accounts_len, 1)?;
                    Ok(CommonExtensionIxs {
                        extension,
                        ix: CommonIx::Initialize(ExtInitializeAccounts {
                            mint: ix.accounts[0],
                        }),
                    })
                },
                1 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    Ok(CommonExtensionIxs {
                        extension,
                        ix: CommonIx::Update(UpdateAccounts {
                            mint: ix.accounts[0],
                            extension_authority: ix.accounts[1],
                            multisig_signers: ix.accounts[2..].to_vec(),
                        }),
                    })
                },
                _ => Err(Error::new("Invalid instruction")),
            },
            IxsSupported::EnableAndDisable => match ix_type {
                0 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    Ok(CommonExtensionIxs {
                        extension,
                        ix: CommonIx::Enable(EnableAccounts {
                            account: ix.accounts[0],
                            owner: ix.accounts[1],
                            multisig_signers: ix.accounts[2..].to_vec(),
                        }),
                    })
                },
                1 => {
                    check_min_accounts_req(accounts_len, 2)?;
                    Ok(CommonExtensionIxs {
                        extension,
                        ix: CommonIx::Disable(DisableAccounts {
                            account: ix.accounts[0],
                            owner: ix.accounts[1],
                            multisig_signers: ix.accounts[2..].to_vec(),
                        }),
                    })
                },
                _ => Err(Error::new("Invalid instruction")),
            },
        }
    }
}

#[cfg(feature = "proto")]
mod proto_parser {

    use common_extension_ix_proto::IxOneof;
    use yellowstone_vixen_proto::parser::{
        common_extension_ix_proto, CommonExtensionIxProto, DisableAccountsProto, DisableIxProto,
        EnableAccountsProto, EnableIxProto, ExtInitializeAccountsProto, ExtInitializeIxProto,
        UpdateAccountsProto, UpdateIxProto,
    };

    use super::{CommonIx, DisableAccounts, EnableAccounts, ExtInitializeAccounts, UpdateAccounts};
    use crate::helpers::{proto::FromVecPubkeyToVecString, IntoProto};

    impl IntoProto<ExtInitializeAccountsProto> for ExtInitializeAccounts {
        fn into_proto(self) -> ExtInitializeAccountsProto {
            ExtInitializeAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProto<UpdateAccountsProto> for UpdateAccounts {
        fn into_proto(self) -> UpdateAccountsProto {
            UpdateAccountsProto {
                mint: self.mint.to_string(),
                extension_authority: self.extension_authority.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<EnableAccountsProto> for EnableAccounts {
        fn into_proto(self) -> EnableAccountsProto {
            EnableAccountsProto {
                account: self.account.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<DisableAccountsProto> for DisableAccounts {
        fn into_proto(self) -> DisableAccountsProto {
            DisableAccountsProto {
                account: self.account.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<CommonExtensionIxProto> for CommonIx {
        fn into_proto(self) -> CommonExtensionIxProto {
            match self {
                CommonIx::Initialize(acc) => CommonExtensionIxProto {
                    ix_oneof: Some(IxOneof::ExtInitializeIx(ExtInitializeIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                CommonIx::Update(acc) => CommonExtensionIxProto {
                    ix_oneof: Some(IxOneof::UpdateIx(UpdateIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                CommonIx::Enable(acc) => CommonExtensionIxProto {
                    ix_oneof: Some(IxOneof::EnableIx(EnableIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                CommonIx::Disable(acc) => CommonExtensionIxProto {
                    ix_oneof: Some(IxOneof::DisableIx(DisableIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },
            }
        }
    }
}
