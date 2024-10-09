use spl_token_2022::{extension::ExtensionType, instruction::AuthorityType};
use yellowstone_vixen_core::Pubkey;

use super::extensions::{
    CommonExtensionIxs, ConfidentaltransferFeeIx, ConfidentaltransferIx, TokenGroupIx,
    TokenMetadataIx, TransferFeeIx,
};
use crate::token_program::ix_parser::{SetAuthorityAccounts, TokenProgramIx};

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum TokenExtensionProgramIx {
    TokenProgramIx(TokenProgramIx),
    SetAuthority(SetAuthorityAccounts, SetAuthorityData),
    CreateNativeMint(CreateNativeMintAccounts),
    InitializeMintCloseAuthority(
        InitializeMintCloseAuthorityAccounts,
        InitializeMintCloseAuthorityData,
    ),
    InitializeNonTransferableMint(InitializeNonTransferableMintAccounts),
    Reallocate(ReallocateAccounts, ReallocateData),
    InitializePermanentDelegate(
        InitializePermanentDelegateAccounts,
        InitializePermanentDelegateData,
    ),
    WithdrawExcessLamports(WithdrawExcessLamportsAccounts),
    TransferFeeIx(TransferFeeIx),
    ConfidentialTransferIx(ConfidentaltransferIx),
    ConfidentialtransferFeeIx(ConfidentaltransferFeeIx),
    CpiGuardIx(CommonExtensionIxs),
    DefaultAccountStateIx(CommonExtensionIxs),
    GroupMemberPointerIx(CommonExtensionIxs),
    GroupPointerIx(CommonExtensionIxs),
    InterestBearingMintIx(CommonExtensionIxs),
    MemoTransferIx(CommonExtensionIxs),
    MetadataPointerIx(CommonExtensionIxs),
    TransferHookIx(CommonExtensionIxs),
    TokenMetadataIx(TokenMetadataIx),
    TokenGroupIx(TokenGroupIx),
}

#[derive(Debug, Clone, Copy)]
pub struct CreateNativeMintAccounts {
    pub mint: Pubkey,
    pub funding_account: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeMintCloseAuthorityAccounts {
    pub mint: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeMintCloseAuthorityData {
    pub close_authority: Option<Pubkey>,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeNonTransferableMintAccounts {
    pub mint: Pubkey,
}

#[derive(Debug)]
pub struct ReallocateAccounts {
    pub account: Pubkey,
    pub payer: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct ReallocateData {
    pub extension_types: Vec<ExtensionType>,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializePermanentDelegateAccounts {
    pub account: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializePermanentDelegateData {
    pub delegate: Pubkey,
}

#[derive(Debug)]
pub struct WithdrawExcessLamportsAccounts {
    pub source_account: Pubkey,
    pub destination_account: Pubkey,
    pub authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct SetAuthorityData {
    pub authority_type: AuthorityType,
    pub new_authority: Option<Pubkey>,
}

#[cfg(feature = "proto")]
mod proto_parser {
    use token_extension_program_ix_proto::IxOneof;
    use yellowstone_vixen_proto::parser::{
        token_extension_program_ix_proto, CpiGuardIxProto, CreateNativeMintAccountsProto,
        CreateNativeMintIxProto, DefaultAccountStateIxProto, GroupMemberPointerIxProto,
        GroupPointerIxProto, InitializeMintCloseAuthorityAccountsProto,
        InitializeMintCloseAuthorityDataProto, InitializeMintCloseAuthorityIxProto,
        InitializeNonTransferableMintAccountsProto, InitializeNonTransferableMintIxProto,
        InitializePermanentDelegateAccountsProto, InitializePermanentDelegateDataProto,
        InitializePermanentDelegateIxProto, InterestBearingMintIxProto, MemoTransferIxProto,
        MetadataPointerIxProto, ReallocateAccountsProto, ReallocateDataProto, ReallocateIxProto,
        SetAuthorityDataProto, SetAuthorityIxProto, TokenExtensionProgramIxProto,
        TransferHookIxProto, WithdrawExcessLamportsAccountsProto, WithdrawExcessLamportsIxProto,
    };

    use super::{
        CreateNativeMintAccounts, InitializeMintCloseAuthorityAccounts,
        InitializeMintCloseAuthorityData, InitializeNonTransferableMintAccounts,
        InitializePermanentDelegateAccounts, InitializePermanentDelegateData, ReallocateAccounts,
        ReallocateData, SetAuthorityData, TokenExtensionProgramIx, WithdrawExcessLamportsAccounts,
    };
    use crate::helpers::{FromOptPubkeyToOptString, FromVecPubkeyToVecString, IntoProto};

    impl IntoProto<WithdrawExcessLamportsAccountsProto> for WithdrawExcessLamportsAccounts {
        fn into_proto(self) -> WithdrawExcessLamportsAccountsProto {
            WithdrawExcessLamportsAccountsProto {
                source_account: self.source_account.to_string(),
                destination_account: self.destination_account.to_string(),
                authority: self.authority.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<InitializePermanentDelegateAccountsProto> for InitializePermanentDelegateAccounts {
        fn into_proto(self) -> InitializePermanentDelegateAccountsProto {
            InitializePermanentDelegateAccountsProto {
                account: self.account.to_string(),
            }
        }
    }

    impl IntoProto<InitializePermanentDelegateDataProto> for InitializePermanentDelegateData {
        fn into_proto(self) -> InitializePermanentDelegateDataProto {
            InitializePermanentDelegateDataProto {
                delegate: self.delegate.to_string(),
            }
        }
    }

    impl IntoProto<ReallocateAccountsProto> for ReallocateAccounts {
        fn into_proto(self) -> ReallocateAccountsProto {
            ReallocateAccountsProto {
                account: self.account.to_string(),
                payer_account: self.payer.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<ReallocateDataProto> for ReallocateData {
        fn into_proto(self) -> ReallocateDataProto {
            ReallocateDataProto {
                extensions_types: self.extension_types.iter().map(|e| *e as i32).collect(),
            }
        }
    }

    impl IntoProto<InitializeNonTransferableMintAccountsProto>
        for InitializeNonTransferableMintAccounts
    {
        fn into_proto(self) -> InitializeNonTransferableMintAccountsProto {
            InitializeNonTransferableMintAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProto<InitializeMintCloseAuthorityAccountsProto> for InitializeMintCloseAuthorityAccounts {
        fn into_proto(self) -> InitializeMintCloseAuthorityAccountsProto {
            InitializeMintCloseAuthorityAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProto<InitializeMintCloseAuthorityDataProto> for InitializeMintCloseAuthorityData {
        fn into_proto(self) -> InitializeMintCloseAuthorityDataProto {
            InitializeMintCloseAuthorityDataProto {
                close_authority: self.close_authority.to_opt_string(),
            }
        }
    }

    impl IntoProto<CreateNativeMintAccountsProto> for CreateNativeMintAccounts {
        fn into_proto(self) -> CreateNativeMintAccountsProto {
            CreateNativeMintAccountsProto {
                mint: self.mint.to_string(),
                funding_account: self.funding_account.to_string(),
            }
        }
    }

    impl IntoProto<SetAuthorityDataProto> for SetAuthorityData {
        fn into_proto(self) -> SetAuthorityDataProto {
            SetAuthorityDataProto {
                authority_type: self.authority_type as i32,
                new_authority: self.new_authority.to_opt_string(),
            }
        }
    }

    impl IntoProto<TokenExtensionProgramIxProto> for TokenExtensionProgramIx {
        #[allow(clippy::too_many_lines)]
        fn into_proto(self) -> TokenExtensionProgramIxProto {
            match self {
                TokenExtensionProgramIx::TransferFeeIx(acc) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::TransferFeeIx(acc.into_proto())),
                },
                TokenExtensionProgramIx::TokenMetadataIx(acc) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::TokenMetadataIx(acc.into_proto())),
                },
                TokenExtensionProgramIx::TokenGroupIx(acc) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::TokenGroupIx(acc.into_proto())),
                },
                TokenExtensionProgramIx::ConfidentialtransferFeeIx(acc) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::ConfidentialTransferFeeIx(acc.into_proto())),
                    }
                },
                TokenExtensionProgramIx::CpiGuardIx(acc) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::CpiGuardIx(CpiGuardIxProto {
                        ix: Some(acc.ix.into_proto()),
                    })),
                },
                TokenExtensionProgramIx::DefaultAccountStateIx(acc) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::DefaultAccountStateIx(
                            DefaultAccountStateIxProto {
                                ix: Some(acc.ix.into_proto()),
                            },
                        )),
                    }
                },
                TokenExtensionProgramIx::GroupMemberPointerIx(acc) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::GroupMemberPointerIx(GroupMemberPointerIxProto {
                            ix: Some(acc.ix.into_proto()),
                        })),
                    }
                },
                TokenExtensionProgramIx::GroupPointerIx(acc) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::GroupPointerIx(GroupPointerIxProto {
                        ix: Some(acc.ix.into_proto()),
                    })),
                },

                TokenExtensionProgramIx::InterestBearingMintIx(acc) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::InterestBearingMintIx(
                            InterestBearingMintIxProto {
                                ix: Some(acc.ix.into_proto()),
                            },
                        )),
                    }
                },
                TokenExtensionProgramIx::MemoTransferIx(acc) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::MemoTransferIx(MemoTransferIxProto {
                        ix: Some(acc.ix.into_proto()),
                    })),
                },

                TokenExtensionProgramIx::MetadataPointerIx(acc) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::MetadataPointerIx(MetadataPointerIxProto {
                        ix: Some(acc.ix.into_proto()),
                    })),
                },

                TokenExtensionProgramIx::TransferHookIx(acc) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::TransferHookIx(TransferHookIxProto {
                        ix: Some(acc.ix.into_proto()),
                    })),
                },
                TokenExtensionProgramIx::TokenProgramIx(acc) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::TokenProgramIx(acc.into_proto())),
                },

                TokenExtensionProgramIx::CreateNativeMint(acc) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::CreateNativeMintIx(CreateNativeMintIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },
                TokenExtensionProgramIx::InitializeMintCloseAuthority(acc, data) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::InitializeMintCloseAuthorityIx(
                            InitializeMintCloseAuthorityIxProto {
                                accounts: Some(acc.into_proto()),
                                data: Some(data.into_proto()),
                            },
                        )),
                    }
                },
                TokenExtensionProgramIx::InitializeNonTransferableMint(acc) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::InitializeNonTransferableMintIx(
                            InitializeNonTransferableMintIxProto {
                                accounts: Some(acc.into_proto()),
                            },
                        )),
                    }
                },
                TokenExtensionProgramIx::Reallocate(acc, data) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::ReallocateIx(ReallocateIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },
                TokenExtensionProgramIx::InitializePermanentDelegate(acc, data) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::InitializePermanentDelegateIx(
                            InitializePermanentDelegateIxProto {
                                accounts: Some(acc.into_proto()),
                                data: Some(data.into_proto()),
                            },
                        )),
                    }
                },
                TokenExtensionProgramIx::WithdrawExcessLamports(acc) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::WithdrawExcessLamportsIx(
                            WithdrawExcessLamportsIxProto {
                                accounts: Some(acc.into_proto()),
                            },
                        )),
                    }
                },

                TokenExtensionProgramIx::ConfidentialTransferIx(acc) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::ConfidentialTransferIx(acc.into_proto())),
                    }
                },
                TokenExtensionProgramIx::SetAuthority(acc, data) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::SetAuthority(SetAuthorityIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },
            }
        }
    }
}
