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
    use crate::{helpers::IntoProtoData, proto::IntoProto};

    use super::*;

    use cpi_guard_ix_proto::IxOneof;
    use yellowstone_vixen_proto::parser::*;
    impl IntoProtoData<TokenExtensionProgramIxProto> for TokenExtensionProgramIx {
        fn into_proto_data(self) -> TokenExtensionProgramIxProto {
            unimplemented!();
            // match self {
            //     TokenExtensionProgramIx::TransferFeeIx(ix) => TokenExtensionProgramIxProto {
            //         ix_oneof: Some(IxOneof::TransferFeeIx(ix.into_proto_data())),
            //     },
            //     TokenExtensionProgramIx::TokenMetadataIx(ix) => TokenExtensionProgramIxProto {
            //         ix_oneof: Some(IxOneof::TokenMetadataIx(ix.into_proto_data())),
            //     },
            //     TokenExtensionProgramIx::TokenGroupIx(ix) => TokenExtensionProgramIxProto {
            //         ix_oneof: Some(IxOneof::TokenGroupIx(ix.into_proto_data())),
            //     },
            //     TokenExtensionProgramIx::ConfidentialtransferFeeIx(ix) => {
            //         TokenExtensionProgramIxProto {
            //             ix_oneof: Some(IxOneof::ConfidentialTransferFeeIx(ix.into_proto_data())),
            //         }
            //     },
            //     TokenExtensionProgramIx::CpiGuardIx(ix) => TokenExtensionProgramIxProto {
            //         ix_oneof: Some(IxOneof::CpiGuardIx(CpiGuardIxProto {
            //             ix_oneof: Some(IxOneof::EnableIx(ix.into_proto_data())),
            //         })),
            //     },
            //     TokenExtensionProgramIx::DefaultAccountStateIx(ix) => {
            //         TokenExtensionProgramIxProto {
            //             ix_oneof: Some(IxOneof::DefaultAccountStateIx(ix.into_proto_data())),
            //         }
            //     },

            //     TokenExtensionProgramIx::GroupMemberPointerIx(ix) => TokenExtensionProgramIxProto {
            //         ix_oneof: Some(IxOneof::GroupMemberPointerIx(ix.into_proto_data())),
            //     },

            //     TokenExtensionProgramIx::GroupPointerIx(ix) => TokenExtensionProgramIxProto {
            //         ix_oneof: Some(IxOneof::GroupPointerIx(ix.into_proto_data())),
            //     },

            //     TokenExtensionProgramIx::InterestBearingMintIx(ix) => {
            //         TokenExtensionProgramIxProto {
            //             ix_oneof: Some(IxOneof::InterestBearingMintIx(ix.into_proto_data())),
            //         }
            //     },

            //     TokenExtensionProgramIx::MemoTransferIx(ix) => TokenExtensionProgramIxProto {
            //         ix_oneof: Some(IxOneof::MemoTransferIx(ix.into_proto_data())),
            //     },

            //     TokenExtensionProgramIx::MetadataPointerIx(ix) => TokenExtensionProgramIxProto {
            //         ix_oneof: Some(IxOneof::MetadataPointerIx(ix.into_proto_data())),
            //     },

            //     TokenExtensionProgramIx::TransferHookIx(ix) => TokenExtensionProgramIxProto {
            //         ix_oneof: Some(IxOneof::TransferHookIx(ix.into_proto_data())),
            //     },

            //     TokenExtensionProgramIx::TokenProgramIx(ix) => TokenExtensionProgramIxProto {
            //         ix_oneof: Some(IxOneof::TokenProgramIx(ix.into_proto_data())),
            //     },

            //     TokenExtensionProgramIx::SetAuthority(ix) => TokenExtensionProgramIxProto {
            //         ix_oneof: Some(IxOneof::SetAuthority(ix.into_proto_data())),
            //     },

            //     TokenExtensionProgramIx::CreateNativeMint(ix) => TokenExtensionProgramIxProto {
            //         ix_oneof: Some(IxOneof::CreateNativeMint(ix.into_proto_data())),
            //     },

            //     TokenExtensionProgramIx::InitializeMintCloseAuthority(ix) => {
            //         TokenExtensionProgramIxProto {
            //             ix_oneof: Some(IxOneof::InitializeMintCloseAuthority(ix.into_proto_data())),
            //         }
            //     },

            //     TokenExtensionProgramIx::InitializeNonTransferableMint(ix) => {
            //         TokenExtensionProgramIxProto {
            //             ix_oneof: Some(IxOneof::InitializeNonTransferableMint(
            //                 ix.into_proto_data(),
            //             )),
            //         }
            //     },

            //     TokenExtensionProgramIx::Reallocate(ix) => TokenExtensionProgramIxProto {
            //         ix_oneof: Some(IxOneof::Reallocate(ix.into_proto_data())),
            //     },

            //     TokenExtensionProgramIx::InitializePermanentDelegate(ix) => {
            //         TokenExtensionProgramIxProto {
            //             ix_oneof: Some(IxOneof::InitializePermanentDelegate(ix.into_proto_data())),
            //         }
            //     },

            //     TokenExtensionProgramIx::WithdrawExcessLamports(ix) => {
            //         TokenExtensionProgramIxProto {
            //             ix_oneof: Some(IxOneof::WithdrawExcessLamports(ix.into_proto_data())),
            //         }
            //     },
            //     _ => unimplemented!(),
            // }
        }
    }
}
