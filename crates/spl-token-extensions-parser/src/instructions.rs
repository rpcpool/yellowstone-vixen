use spl_token_2022::{extension::ExtensionType, instruction::AuthorityType as SplAuthorityType};
use yellowstone_vixen_proc_macro::vixen;
use yellowstone_vixen_spl_token_parser::{SetAuthorityAccounts, TokenProgram as BaseTokenProgram};

use super::{
    CommonExtensionInstructions, ConfidentialTransferFeeIx, ConfidentialTransferIx, TokenGroupIx,
    TokenMetadataIx, TransferFeeIx,
};
use crate::PublicKey;

#[vixen]
#[derive(Clone, PartialEq)]
pub struct CreateNativeMintAccounts {
    pub mint: PublicKey,
    pub funding_account: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeMintCloseAuthorityAccounts {
    pub mint: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeMintCloseAuthorityArgs {
    pub close_authority: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeNonTransferableMintAccounts {
    pub mint: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ReallocateAccounts {
    pub account: PublicKey,
    pub payer: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ReallocateArgs {
    pub extension_types: Vec<u32>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializePermanentDelegateAccounts {
    pub account: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializePermanentDelegateArgs {
    pub delegate: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WithdrawExcessLamportsAccounts {
    pub source_account: PublicKey,
    pub destination_account: PublicKey,
    pub authority: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen(enumeration)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AuthorityType {
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

impl From<spl_token_2022::instruction::AuthorityType> for AuthorityType {
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

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetAuthorityArgs {
    #[hint(enumeration = "AuthorityType")]
    pub authority_type: i32,
    pub new_authority: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct TokenExtensionProgram {
    #[hint(
        oneof = "instruction::Instruction",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21"
    )]
    pub instruction: Option<instruction::Instruction>,
}

pub mod instruction {
    use super::{
        vixen, BaseTokenProgram, CommonExtensionInstructions, ConfidentialTransferFeeIx,
        ConfidentialTransferIx, SetAuthorityAccounts, TokenGroupIx, TokenMetadataIx, TransferFeeIx,
    };

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct TokenProgram {
        pub instruction: Option<BaseTokenProgram>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct SetAuthority {
        pub accounts: Option<SetAuthorityAccounts>,
        pub args: Option<super::SetAuthorityArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct CreateNativeMint {
        pub accounts: Option<super::CreateNativeMintAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeMintCloseAuthority {
        pub accounts: Option<super::InitializeMintCloseAuthorityAccounts>,
        pub args: Option<super::InitializeMintCloseAuthorityArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeNonTransferableMint {
        pub accounts: Option<super::InitializeNonTransferableMintAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Reallocate {
        pub accounts: Option<super::ReallocateAccounts>,
        pub args: Option<super::ReallocateArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializePermanentDelegate {
        pub accounts: Option<super::InitializePermanentDelegateAccounts>,
        pub args: Option<super::InitializePermanentDelegateArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct WithdrawExcessLamports {
        pub accounts: Option<super::WithdrawExcessLamportsAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct TransferFee {
        pub instruction: Option<TransferFeeIx>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct ConfidentialTransfer {
        pub instruction: Option<ConfidentialTransferIx>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct ConfidentialTransferFee {
        pub instruction: Option<ConfidentialTransferFeeIx>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct CpiGuard {
        pub instruction: Option<CommonExtensionInstructions>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct DefaultAccountState {
        pub instruction: Option<CommonExtensionInstructions>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct GroupMemberPointer {
        pub instruction: Option<CommonExtensionInstructions>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct GroupPointer {
        pub instruction: Option<CommonExtensionInstructions>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InterestBearingMint {
        pub instruction: Option<CommonExtensionInstructions>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct MemoTransfer {
        pub instruction: Option<CommonExtensionInstructions>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct MetadataPointer {
        pub instruction: Option<CommonExtensionInstructions>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct TransferHook {
        pub instruction: Option<CommonExtensionInstructions>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct TokenMetadata {
        pub instruction: Option<TokenMetadataIx>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct TokenGroup {
        pub instruction: Option<TokenGroupIx>,
    }

    #[vixen(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Instruction {
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
    new_authority: Option<yellowstone_vixen_core::KeyBytes<32>>,
) -> SetAuthorityArgs {
    SetAuthorityArgs {
        authority_type: authority_type as i32,
        new_authority: new_authority.map(|p| crate::PublicKey::new(p.0)),
    }
}
