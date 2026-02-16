use spl_token_2022::{extension::ExtensionType, instruction::AuthorityType as SplAuthorityType};
use yellowstone_vixen_spl_token_parser::{SetAuthorityAccounts, TokenProgramInstruction};
use yellowstone_vixen_proc_macro::vixen_proto;

use super::{
    CommonExtensionInstructions, ConfidentialTransferFeeInstruction,
    ConfidentialTransferInstruction, TokenGroupInstruction, TokenMetadataInstruction,
    TransferFeeInstruction,
};
use crate::PubkeyBytes;

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct CreateNativeMintAccounts {
    pub mint: PubkeyBytes,
    pub funding_account: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeMintCloseAuthorityAccounts {
    pub mint: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
pub struct InitializeMintCloseAuthorityArgs {
    pub close_authority: Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeNonTransferableMintAccounts {
    pub mint: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ReallocateAccounts {
    pub account: PubkeyBytes,
    pub payer: PubkeyBytes,
    pub owner: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ReallocateArgs {
    pub extension_types: Vec<u32>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializePermanentDelegateAccounts {
    pub account: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializePermanentDelegateArgs {
    pub delegate: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawExcessLamportsAccounts {
    pub source_account: PubkeyBytes,
    pub destination_account: PubkeyBytes,
    pub authority: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto(enumeration)]
#[derive(Clone, Copy, PartialEq, Eq)]
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

#[vixen_proto]
#[derive(Clone, PartialEq, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
pub struct SetAuthorityArgs {
    #[vixen_proto_hint(enumeration = "AuthorityTypeProto")]
    pub authority_type: i32,
    pub new_authority: Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TokenExtensionProgramInstruction {
    #[vixen_proto_hint(oneof = "token_extension_program_instruction::Ix", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21")]
    pub ix: Option<token_extension_program_instruction::Ix>,
}

pub mod token_extension_program_instruction {
    use super::*;

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct TokenProgram {
        pub ix: Option<TokenProgramInstruction>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct SetAuthority {
        pub accounts: Option<SetAuthorityAccounts>,
        pub args: Option<super::SetAuthorityArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct CreateNativeMint {
        pub accounts: Option<super::CreateNativeMintAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct InitializeMintCloseAuthority {
        pub accounts: Option<super::InitializeMintCloseAuthorityAccounts>,
        pub args: Option<super::InitializeMintCloseAuthorityArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct InitializeNonTransferableMint {
        pub accounts: Option<super::InitializeNonTransferableMintAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct Reallocate {
        pub accounts: Option<super::ReallocateAccounts>,
        pub args: Option<super::ReallocateArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct InitializePermanentDelegate {
        pub accounts: Option<super::InitializePermanentDelegateAccounts>,
        pub args: Option<super::InitializePermanentDelegateArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct WithdrawExcessLamports {
        pub accounts: Option<super::WithdrawExcessLamportsAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct TransferFee {
        pub ix: Option<TransferFeeInstruction>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct ConfidentialTransfer {
        pub ix: Option<ConfidentialTransferInstruction>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct ConfidentialTransferFee {
        pub ix: Option<ConfidentialTransferFeeInstruction>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct CpiGuard {
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct DefaultAccountState {
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct GroupMemberPointer {
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct GroupPointer {
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct InterestBearingMint {
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct MemoTransfer {
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct MetadataPointer {
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct TransferHook {
        pub ix: Option<CommonExtensionInstructions>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct TokenMetadata {
        pub ix: Option<TokenMetadataInstruction>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct TokenGroup {
        pub ix: Option<TokenGroupInstruction>,
    }

    #[vixen_proto(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Ix {
        TokenProgram(TokenProgram),
        SetAuthority(SetAuthority),
        CreateNativeMint(CreateNativeMint),
        InitializeMintCloseAuthority(InitializeMintCloseAuthority),
        InitializeNonTransferableMint(InitializeNonTransferableMint),
        Reallocate(Reallocate),
        InitializePermanentDelegate(InitializePermanentDelegate),
        WithdrawExcessLamports(WithdrawExcessLamports),
        TransferFee(TransferFee),
        ConfidentialTransfer(ConfidentialTransfer),
        ConfidentialTransferFee(ConfidentialTransferFee),
        CpiGuard(CpiGuard),
        DefaultAccountState(DefaultAccountState),
        GroupMemberPointer(GroupMemberPointer),
        GroupPointer(GroupPointer),
        InterestBearingMint(InterestBearingMint),
        MemoTransfer(MemoTransfer),
        MetadataPointer(MetadataPointer),
        TransferHook(TransferHook),
        TokenMetadata(TokenMetadata),
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
