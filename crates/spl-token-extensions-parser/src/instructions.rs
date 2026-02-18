use spl_token_2022::{extension::ExtensionType, instruction::AuthorityType as SplAuthorityType};
use yellowstone_vixen_proc_macro::vixen_proto;
use yellowstone_vixen_spl_token_parser::{SetAuthorityAccounts, TokenProgram as BaseTokenProgram};

use super::{
    CommonExtensionInstructions, ConfidentialTransferFeeIx, ConfidentialTransferIx,
    TokenGroupIx, TokenMetadataIx, TransferFeeIx,
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

#[vixen_proto]
#[derive(Clone, PartialEq, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
pub struct SetAuthorityArgs {
    #[vixen_proto_hint(enumeration = "AuthorityType")]
    pub authority_type: i32,
    pub new_authority: Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TokenExtensionProgram {
    #[vixen_proto_hint(
        oneof = "Instruction",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21"
    )]
    pub instruction: Option<Instruction>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TokenProgramInstruction {
    pub instruction: Option<BaseTokenProgram>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetAuthorityInstruction {
    pub accounts: Option<SetAuthorityAccounts>,
    pub args: Option<super::SetAuthorityArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct CreateNativeMintInstruction {
    pub accounts: Option<super::CreateNativeMintAccounts>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeMintCloseAuthorityInstruction {
    pub accounts: Option<super::InitializeMintCloseAuthorityAccounts>,
    pub args: Option<super::InitializeMintCloseAuthorityArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeNonTransferableMintInstruction {
    pub accounts: Option<super::InitializeNonTransferableMintAccounts>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ReallocateInstruction {
    pub accounts: Option<super::ReallocateAccounts>,
    pub args: Option<super::ReallocateArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializePermanentDelegateInstruction {
    pub accounts: Option<super::InitializePermanentDelegateAccounts>,
    pub args: Option<super::InitializePermanentDelegateArgs>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawExcessLamportsInstruction {
    pub accounts: Option<super::WithdrawExcessLamportsAccounts>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TransferFeeInstruction {
    pub instruction: Option<TransferFeeIx>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ConfidentialTransferInstruction {
    pub instruction: Option<ConfidentialTransferIx>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ConfidentialTransferFeeInstruction {
    pub instruction: Option<ConfidentialTransferFeeIx>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct CpiGuardInstruction {
    pub instruction: Option<CommonExtensionInstructions>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DefaultAccountStateInstruction {
    pub instruction: Option<CommonExtensionInstructions>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct GroupMemberPointerInstruction {
    pub instruction: Option<CommonExtensionInstructions>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct GroupPointerInstruction {
    pub instruction: Option<CommonExtensionInstructions>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InterestBearingMintInstruction {
    pub instruction: Option<CommonExtensionInstructions>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct MemoTransferInstruction {
    pub instruction: Option<CommonExtensionInstructions>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct MetadataPointerInstruction {
    pub instruction: Option<CommonExtensionInstructions>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TransferHookInstruction {
    pub instruction: Option<CommonExtensionInstructions>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TokenMetadataInstruction {
    pub instruction: Option<TokenMetadataIx>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TokenGroupInstruction {
    pub instruction: Option<TokenGroupIx>,
}

#[vixen_proto(oneof)]
#[derive(Clone, PartialEq)]
pub enum Instruction {
    TokenProgram(TokenProgramInstruction),
    SetAuthority(SetAuthorityInstruction),
    CreateNativeMint(CreateNativeMintInstruction),
    InitializeMintCloseAuthority(InitializeMintCloseAuthorityInstruction),
    InitializeNonTransferableMint(InitializeNonTransferableMintInstruction),
    Reallocate(ReallocateInstruction),
    InitializePermanentDelegate(InitializePermanentDelegateInstruction),
    WithdrawExcessLamports(WithdrawExcessLamportsInstruction),
    TransferFee(TransferFeeInstruction),
    ConfidentialTransfer(ConfidentialTransferInstruction),
    ConfidentialTransferFee(ConfidentialTransferFeeInstruction),
    CpiGuard(CpiGuardInstruction),
    DefaultAccountState(DefaultAccountStateInstruction),
    GroupMemberPointer(GroupMemberPointerInstruction),
    GroupPointer(GroupPointerInstruction),
    InterestBearingMint(InterestBearingMintInstruction),
    MemoTransfer(MemoTransferInstruction),
    MetadataPointer(MetadataPointerInstruction),
    TransferHook(TransferHookInstruction),
    TokenMetadata(TokenMetadataInstruction),
    TokenGroup(TokenGroupInstruction),
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
