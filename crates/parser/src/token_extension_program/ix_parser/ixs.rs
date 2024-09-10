use spl_pod::solana_program::program_option::COption;
use spl_token_2022::{extension::ExtensionType, instruction::AuthorityType};
use yellowstone_vixen_core::Pubkey;

use super::extensions::{
    CommonExtensionIxs, ConfidentaltransferFeeIx, ConfidentaltransferIx, TokenGroupIx,
    TokenMetadataIx, TransferFeeIx,
};
use crate::{
    helpers::ReadableInstruction,
    token_program::ix_parser::{SetAuthorityAccounts, TokenProgramIx},
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum TokenExtensionProgramIx {
    TokenProgramIx(TokenProgramIx),
    SetAuthority(ReadableInstruction<SetAuthorityAccounts, TokenExtSetAutorityData>),
    CreateNativeMint(ReadableInstruction<CreateNativeMintAccounts, ()>),
    InitializeMintCloseAuthority(
        ReadableInstruction<InitializeMintCloseAuthorityAccounts, InitializeMintCloseAuthorityData>,
    ),
    InitializeNonTransferableMint(ReadableInstruction<InitializeNonTransferableMintAccounts, ()>),
    Reallocate(ReadableInstruction<ReallocateAccounts, ReallocateData>),
    InitializePermanentDelegate(
        ReadableInstruction<InitializePermanentDelegateAccounts, InitializePermanentDelegateData>,
    ),
    WithdrawExcessLamports(ReadableInstruction<WithdrawExcessLamportsAccounts, ()>),
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
    pub new_authority: COption<Pubkey>,
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
    pub close_authority: COption<Pubkey>,
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
    pub multisig_signers: Option<Vec<Pubkey>>,
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
    pub multisig_signers: Option<Vec<Pubkey>>,
}
