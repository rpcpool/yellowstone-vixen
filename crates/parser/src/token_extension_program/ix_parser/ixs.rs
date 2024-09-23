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
    SetAuthority(SetAuthorityAccounts, TokenExtSetAutorityData),
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

#[derive(Debug)]
pub struct TokenExtSetAutorityData {
    pub authority_type: AuthorityType,
    pub new_authority: Option<Pubkey>,
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
        TokenExtensionProgramIxProto, TransferHookIxProto, WithdrawExcessLamportsAccountsProto,
        WithdrawExcessLamportsIxProto,
    };

    use super::{
        CreateNativeMintAccounts, InitializeMintCloseAuthorityAccounts,
        InitializeMintCloseAuthorityData, InitializeNonTransferableMintAccounts,
        InitializePermanentDelegateAccounts, InitializePermanentDelegateData, ReallocateAccounts,
        ReallocateData, TokenExtensionProgramIx, WithdrawExcessLamportsAccounts,
    };
    use crate::helpers::{
        FromCOptionPubkeyToOptString, FromOptVecToDefVec, FromOptionToProtoOption, IntoProtoData,
    };

    impl IntoProtoData<WithdrawExcessLamportsAccountsProto> for WithdrawExcessLamportsAccounts {
        fn into_proto_data(self) -> WithdrawExcessLamportsAccountsProto {
            WithdrawExcessLamportsAccountsProto {
                source_account: self.source_account.to_string(),
                destination_account: self.destination_account.to_string(),
                authority: self.authority.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<InitializePermanentDelegateAccountsProto>
        for InitializePermanentDelegateAccounts
    {
        fn into_proto_data(self) -> InitializePermanentDelegateAccountsProto {
            InitializePermanentDelegateAccountsProto {
                account: self.account.to_string(),
            }
        }
    }

    impl IntoProtoData<InitializePermanentDelegateDataProto> for InitializePermanentDelegateData {
        fn into_proto_data(self) -> InitializePermanentDelegateDataProto {
            InitializePermanentDelegateDataProto {
                delegate: self.delegate.to_string(),
            }
        }
    }

    impl IntoProtoData<ReallocateAccountsProto> for ReallocateAccounts {
        fn into_proto_data(self) -> ReallocateAccountsProto {
            ReallocateAccountsProto {
                account: self.account.to_string(),
                payer_account: self.payer.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<ReallocateDataProto> for ReallocateData {
        fn into_proto_data(self) -> ReallocateDataProto {
            ReallocateDataProto {
                extensions_types: self.extension_types.iter().map(|e| *e as i32).collect(),
            }
        }
    }

    impl IntoProtoData<InitializeNonTransferableMintAccountsProto>
        for InitializeNonTransferableMintAccounts
    {
        fn into_proto_data(self) -> InitializeNonTransferableMintAccountsProto {
            InitializeNonTransferableMintAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProtoData<InitializeMintCloseAuthorityAccountsProto>
        for InitializeMintCloseAuthorityAccounts
    {
        fn into_proto_data(self) -> InitializeMintCloseAuthorityAccountsProto {
            InitializeMintCloseAuthorityAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProtoData<InitializeMintCloseAuthorityDataProto> for InitializeMintCloseAuthorityData {
        fn into_proto_data(self) -> InitializeMintCloseAuthorityDataProto {
            InitializeMintCloseAuthorityDataProto {
                close_authority: self.close_authority.to_opt_string(),
            }
        }
    }

    impl IntoProtoData<CreateNativeMintAccountsProto> for CreateNativeMintAccounts {
        fn into_proto_data(self) -> CreateNativeMintAccountsProto {
            CreateNativeMintAccountsProto {
                mint: self.mint.to_string(),
                funding_account: self.funding_account.to_string(),
            }
        }
    }

    impl IntoProtoData<TokenExtensionProgramIxProto> for TokenExtensionProgramIx {
        #[allow(clippy::too_many_lines)]
        fn into_proto_data(self) -> TokenExtensionProgramIxProto {
            match self {
                TokenExtensionProgramIx::TransferFeeIx(ri) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::TransferFeeIx(ri.into_proto_data())),
                },
                TokenExtensionProgramIx::TokenMetadataIx(ri) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::TokenMetadataIx(ri.into_proto_data())),
                },
                TokenExtensionProgramIx::TokenGroupIx(ri) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::TokenGroupIx(ri.into_proto_data())),
                },
                TokenExtensionProgramIx::ConfidentialtransferFeeIx(ri) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::ConfidentialTransferFeeIx(ri.into_proto_data())),
                    }
                },
                TokenExtensionProgramIx::CpiGuardIx(ri) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::CpiGuardIx(CpiGuardIxProto {
                        ix: Some(ri.ix.into_proto_data()),
                    })),
                },
                TokenExtensionProgramIx::DefaultAccountStateIx(ri) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::DefaultAccountStateIx(
                            DefaultAccountStateIxProto {
                                ix: Some(ri.ix.into_proto_data()),
                            },
                        )),
                    }
                },
                TokenExtensionProgramIx::GroupMemberPointerIx(ri) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::GroupMemberPointerIx(GroupMemberPointerIxProto {
                        ix: Some(ri.ix.into_proto_data()),
                    })),
                },
                TokenExtensionProgramIx::GroupPointerIx(ri) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::GroupPointerIx(GroupPointerIxProto {
                        ix: Some(ri.ix.into_proto_data()),
                    })),
                },

                TokenExtensionProgramIx::InterestBearingMintIx(ri) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::InterestBearingMintIx(
                            InterestBearingMintIxProto {
                                ix: Some(ri.ix.into_proto_data()),
                            },
                        )),
                    }
                },
                TokenExtensionProgramIx::MemoTransferIx(ri) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::MemoTransferIx(MemoTransferIxProto {
                        ix: Some(ri.ix.into_proto_data()),
                    })),
                },

                TokenExtensionProgramIx::MetadataPointerIx(ri) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::MetadataPointerIx(MetadataPointerIxProto {
                        ix: Some(ri.ix.into_proto_data()),
                    })),
                },

                TokenExtensionProgramIx::TransferHookIx(ri) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::TransferHookIx(TransferHookIxProto {
                        ix: Some(ri.ix.into_proto_data()),
                    })),
                },
                TokenExtensionProgramIx::TokenProgramIx(ri) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::TokenProgramIx(ri.into_proto_data())),
                },

                TokenExtensionProgramIx::CreateNativeMint(ri) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::CreateNativeMintIx(CreateNativeMintIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                    })),
                },
                TokenExtensionProgramIx::InitializeMintCloseAuthority(ri) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::InitializeMintCloseAuthorityIx(
                            InitializeMintCloseAuthorityIxProto {
                                accounts: Some(ri.accounts.into_proto_data()),
                                data: ri.data.to_proto_option(),
                            },
                        )),
                    }
                },
                TokenExtensionProgramIx::InitializeNonTransferableMint(ri) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::InitializeNonTransferableMintIx(
                            InitializeNonTransferableMintIxProto {
                                accounts: Some(ri.accounts.into_proto_data()),
                            },
                        )),
                    }
                },
                TokenExtensionProgramIx::Reallocate(ri) => TokenExtensionProgramIxProto {
                    ix_oneof: Some(IxOneof::ReallocateIx(ReallocateIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },
                TokenExtensionProgramIx::InitializePermanentDelegate(ri) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::InitializePermanentDelegateIx(
                            InitializePermanentDelegateIxProto {
                                accounts: Some(ri.accounts.into_proto_data()),
                                data: ri.data.to_proto_option(),
                            },
                        )),
                    }
                },
                TokenExtensionProgramIx::WithdrawExcessLamports(ri) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::WithdrawExcessLamportsIx(
                            WithdrawExcessLamportsIxProto {
                                accounts: Some(ri.accounts.into_proto_data()),
                            },
                        )),
                    }
                },

                TokenExtensionProgramIx::ConfidentialTransferIx(ri) => {
                    TokenExtensionProgramIxProto {
                        ix_oneof: Some(IxOneof::ConfidentialTransferIx(ri.into_proto_data())),
                    }
                },
            }
        }
    }
}
