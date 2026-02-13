use prost::alloc::vec::Vec;
use spl_token_2022::{extension::ExtensionType, instruction::AuthorityType as SplAuthorityType};
use yellowstone_vixen_spl_token_parser::{SetAuthorityAccounts, TokenProgramInstruction};

use super::{
    CommonExtensionInstructions, ConfidentialTransferFeeInstruction,
    ConfidentialTransferInstruction, TokenGroupInstruction, TokenMetadataInstruction,
    TransferFeeInstruction,
};
use crate::PubkeyBytes;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNativeMintAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub funding_account: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeMintCloseAuthorityAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
}

#[derive(
    Clone, PartialEq, ::prost::Message, ::borsh::BorshDeserialize, ::borsh::BorshSerialize,
)]
pub struct InitializeMintCloseAuthorityArgs {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub close_authority: Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeNonTransferableMintAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReallocateAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub payer: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReallocateArgs {
    #[prost(uint32, repeated, tag = "1")]
    pub extension_types: Vec<u32>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializePermanentDelegateAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializePermanentDelegateArgs {
    #[prost(bytes = "vec", tag = "1")]
    pub delegate: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawExcessLamportsAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub source_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub destination_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub authority: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthorityTypeProto {
    MintTokens = 0,
    FreezeAccount = 1,
    AccountOwner = 2,
    CloseAccount = 3,
    TransferFeeConfig = 4,
    WithheldWithdraw = 5,
    CloseMint = 6,
    InterestRate = 7,
    PermanentDelegate = 8,
    ConfidentialTransferMint = 9,
    TransferHookProgramId = 10,
    ConfidentialTransferFeeConfig = 11,
    MetadataPointer = 12,
    GroupPointer = 13,
    GroupMemberPointer = 14,
    ScaledUiAmount = 15,
    Pause = 16,
}

impl From<spl_token_2022::instruction::AuthorityType> for AuthorityTypeProto {
    fn from(a: spl_token_2022::instruction::AuthorityType) -> Self {
        use spl_token_2022::instruction::AuthorityType as A;
        match a {
            A::MintTokens => Self::MintTokens,
            A::FreezeAccount => Self::FreezeAccount,
            A::AccountOwner => Self::AccountOwner,
            A::CloseAccount => Self::CloseAccount,
            A::TransferFeeConfig => Self::TransferFeeConfig,
            A::WithheldWithdraw => Self::WithheldWithdraw,
            A::CloseMint => Self::CloseMint,
            A::InterestRate => Self::InterestRate,
            A::PermanentDelegate => Self::PermanentDelegate,
            A::ConfidentialTransferMint => Self::ConfidentialTransferMint,
            A::TransferHookProgramId => Self::TransferHookProgramId,
            A::ConfidentialTransferFeeConfig => Self::ConfidentialTransferFeeConfig,
            A::MetadataPointer => Self::MetadataPointer,
            A::GroupPointer => Self::GroupPointer,
            A::GroupMemberPointer => Self::GroupMemberPointer,
            A::ScaledUiAmount => Self::ScaledUiAmount,
            A::Pause => Self::Pause,
        }
    }
}

#[derive(
    Clone, PartialEq, ::prost::Message, ::borsh::BorshDeserialize, ::borsh::BorshSerialize,
)]
pub struct SetAuthorityArgs {
    #[prost(enumeration = "AuthorityTypeProto", tag = "1")]
    pub authority_type: i32,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub new_authority: Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenExtensionProgramInstruction {
    #[prost(
        oneof = "token_extension_program_instruction::Ix",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21"
    )]
    pub ix: Option<token_extension_program_instruction::Ix>,
}

pub mod token_extension_program_instruction {
    use super::*;

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TokenProgram {
        #[prost(message, optional, tag = "1")]
        pub ix: Option<TokenProgramInstruction>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetAuthority {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<SetAuthorityAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::SetAuthorityArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateNativeMint {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::CreateNativeMintAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitializeMintCloseAuthority {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::InitializeMintCloseAuthorityAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::InitializeMintCloseAuthorityArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitializeNonTransferableMint {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::InitializeNonTransferableMintAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Reallocate {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::ReallocateAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::ReallocateArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitializePermanentDelegate {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::InitializePermanentDelegateAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::InitializePermanentDelegateArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawExcessLamports {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::WithdrawExcessLamportsAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransferFee {
        #[prost(message, optional, tag = "1")]
        pub ix: Option<TransferFeeInstruction>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConfidentialTransfer {
        #[prost(message, optional, tag = "1")]
        pub ix: Option<ConfidentialTransferInstruction>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConfidentialTransferFee {
        #[prost(message, optional, tag = "1")]
        pub ix: Option<ConfidentialTransferFeeInstruction>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CpiGuard {
        #[prost(message, optional, tag = "1")]
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DefaultAccountState {
        #[prost(message, optional, tag = "1")]
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupMemberPointer {
        #[prost(message, optional, tag = "1")]
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupPointer {
        #[prost(message, optional, tag = "1")]
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InterestBearingMint {
        #[prost(message, optional, tag = "1")]
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MemoTransfer {
        #[prost(message, optional, tag = "1")]
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetadataPointer {
        #[prost(message, optional, tag = "1")]
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransferHook {
        #[prost(message, optional, tag = "1")]
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TokenMetadata {
        #[prost(message, optional, tag = "1")]
        pub ix: Option<TokenMetadataInstruction>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TokenGroup {
        #[prost(message, optional, tag = "1")]
        pub ix: Option<TokenGroupInstruction>,
    }

    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ix {
        #[prost(message, tag = "1")]
        TokenProgram(TokenProgram),
        #[prost(message, tag = "2")]
        SetAuthority(SetAuthority),
        #[prost(message, tag = "3")]
        CreateNativeMint(CreateNativeMint),
        #[prost(message, tag = "4")]
        InitializeMintCloseAuthority(InitializeMintCloseAuthority),
        #[prost(message, tag = "5")]
        InitializeNonTransferableMint(InitializeNonTransferableMint),
        #[prost(message, tag = "6")]
        Reallocate(Reallocate),
        #[prost(message, tag = "7")]
        InitializePermanentDelegate(InitializePermanentDelegate),
        #[prost(message, tag = "8")]
        WithdrawExcessLamports(WithdrawExcessLamports),
        #[prost(message, tag = "9")]
        TransferFee(TransferFee),
        #[prost(message, tag = "10")]
        ConfidentialTransfer(ConfidentialTransfer),
        #[prost(message, tag = "11")]
        ConfidentialTransferFee(ConfidentialTransferFee),
        #[prost(message, tag = "12")]
        CpiGuard(CpiGuard),
        #[prost(message, tag = "13")]
        DefaultAccountState(DefaultAccountState),
        #[prost(message, tag = "14")]
        GroupMemberPointer(GroupMemberPointer),
        #[prost(message, tag = "15")]
        GroupPointer(GroupPointer),
        #[prost(message, tag = "16")]
        InterestBearingMint(InterestBearingMint),
        #[prost(message, tag = "17")]
        MemoTransfer(MemoTransfer),
        #[prost(message, tag = "18")]
        MetadataPointer(MetadataPointer),
        #[prost(message, tag = "19")]
        TransferHook(TransferHook),
        #[prost(message, tag = "20")]
        TokenMetadata(TokenMetadata),
        #[prost(message, tag = "21")]
        TokenGroup(TokenGroup),
    }
}

#[inline]
pub fn reallocate_args_from_spl(extension_types: Vec<ExtensionType>) -> ReallocateArgs {
    ReallocateArgs {
        extension_types: extension_types.into_iter().map(|e| e as u32).collect(),
    }
}

#[inline]
pub fn set_authority_args_from_spl(
    authority_type: SplAuthorityType,
    new_authority: Option<yellowstone_vixen_core::Pubkey>,
) -> SetAuthorityArgs {
    SetAuthorityArgs {
        authority_type: authority_type as i32,
        new_authority: new_authority.map(|p| p.to_vec()),
    }
}
