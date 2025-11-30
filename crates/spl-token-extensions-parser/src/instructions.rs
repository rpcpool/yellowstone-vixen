use spl_token_2022::{extension::ExtensionType, instruction::AuthorityType};
use yellowstone_vixen_core::Pubkey;
use yellowstone_vixen_spl_token_parser::{SetAuthorityAccounts, TokenProgramInstruction};

use super::{
    CommonExtensionInstructions, ConfidentialTransferFeeInstruction,
    ConfidentialTransferInstruction, TokenGroupInstruction, TokenMetadataInstruction,
    TransferFeeInstruction,
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum TokenExtensionProgramInstruction {
    TokenProgram(TokenProgramInstruction),
    SetAuthority {
        accounts: SetAuthorityAccounts,
        args: SetAuthorityArgs,
    },
    CreateNativeMint {
        accounts: CreateNativeMintAccounts,
    },
    InitializeMintCloseAuthority {
        accounts: InitializeMintCloseAuthorityAccounts,
        args: InitializeMintCloseAuthorityArgs,
    },
    InitializeNonTransferableMint {
        accounts: InitializeNonTransferableMintAccounts,
    },
    Reallocate {
        accounts: ReallocateAccounts,
        args: ReallocateArgs,
    },
    InitializePermanentDelegate {
        accounts: InitializePermanentDelegateAccounts,
        args: InitializePermanentDelegateArgs,
    },
    WithdrawExcessLamports {
        accounts: WithdrawExcessLamportsAccounts,
    },
    TransferFee(TransferFeeInstruction),
    ConfidentialTransfer(ConfidentialTransferInstruction),
    ConfidentialTransferFee(ConfidentialTransferFeeInstruction),
    CpiGuard(CommonExtensionInstructions),
    DefaultAccountState(CommonExtensionInstructions),
    GroupMemberPointer(CommonExtensionInstructions),
    GroupPointer(CommonExtensionInstructions),
    InterestBearingMint(CommonExtensionInstructions),
    MemoTransfer(CommonExtensionInstructions),
    MetadataPointer(CommonExtensionInstructions),
    TransferHook(CommonExtensionInstructions),
    TokenMetadata(TokenMetadataInstruction),
    TokenGroup(TokenGroupInstruction),
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
pub struct InitializeMintCloseAuthorityArgs {
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
pub struct ReallocateArgs {
    pub extension_types: Vec<ExtensionType>,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializePermanentDelegateAccounts {
    pub account: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializePermanentDelegateArgs {
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
pub struct SetAuthorityArgs {
    pub authority_type: AuthorityType,
    pub new_authority: Option<Pubkey>,
}
