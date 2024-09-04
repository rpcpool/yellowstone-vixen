use spl_pod::solana_program::program_option::COption;
use spl_token::instruction::AuthorityType;
use yellowstone_vixen_core::Pubkey;

use crate::helpers::ReadableInstruction;
#[derive(Debug, Clone)]
pub struct TransferAccounts {
    pub source: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug, Clone, Copy)]
pub struct TransferCheckedData {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct TransferData {
    pub amount: u64,
}
#[derive(Debug, Clone, Copy)]
pub struct InitializeMintAccounts {
    pub mint: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeMintData {
    pub decimals: u8,
    pub mint_authority: COption<Pubkey>,
    pub freeze_authority: COption<Pubkey>,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeAccountAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeAccount2Accounts {
    pub account: Pubkey,
    pub mint: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeAccountData2 {
    pub owner: Pubkey,
}

#[derive(Debug, Clone)]
pub struct InitializeMultisigAccounts {
    pub multisig: Pubkey,
    pub signers: Option<Vec<Pubkey>>,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeMultisigData {
    pub m: u8,
}

#[derive(Debug, Clone)]
pub struct ApproveAccounts {
    pub source: Pubkey,
    pub delegate: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug, Clone, Copy)]
pub struct ApproveData {
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct RevokeAccounts {
    pub source: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug, Clone)]
pub struct SetAuthorityAccounts {
    pub current_authority: Pubkey,
    pub account: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug, Clone)]

pub struct SetAuthorityData {
    pub authority_type: AuthorityType,
    pub new_authority: COption<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct MintToAccounts {
    pub mint: Pubkey,
    pub account: Pubkey,
    pub mint_authority: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug, Clone, Copy)]
pub struct MintToData {
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct BurnAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug, Clone, Copy)]
pub struct BurnData {
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct CloseAccountAccounts {
    pub account: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug, Clone)]
pub struct FreezeAccountAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub mint_freeze_authority: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}
#[derive(Debug, Clone)]
pub struct ThawAccountAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub mint_freeze_authority: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}
#[derive(Debug, Clone)]
pub struct TransferCheckedAccounts {
    pub source: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug, Clone)]
pub struct ApproveCheckedAccounts {
    pub source: Pubkey,
    pub mint: Pubkey,
    pub delegate: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug, Clone, Copy)]
pub struct ApproveCheckedData {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(Debug, Clone)]
pub struct MintToCheckedAccounts {
    pub mint: Pubkey,
    pub account: Pubkey,
    pub mint_authority: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug, Clone, Copy)]
pub struct MintToCheckedData {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(Debug, Clone)]
pub struct BurnCheckedAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug, Clone, Copy)]
pub struct BurnCheckedData {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct SyncNativeAccounts {
    pub account: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct GetAccountDataSizeAccounts {
    pub mint: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeImmutableOwnerAccounts {
    pub account: Pubkey,
}
#[derive(Debug, Clone, Copy)]
pub struct AmountToUiAmountAccounts {
    pub mint: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct AmountToUiAmountData {
    pub amount: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct UiAmountToAmountAccounts {
    pub mint: Pubkey,
}

#[derive(Debug, Clone)]
pub struct UiAmountToAmountData {
    pub ui_amount: String,
}

#[derive(Debug)]
pub enum TokenProgramIx {
    Transfer(ReadableInstruction<TransferAccounts, TransferData>),
    InitializeMint(ReadableInstruction<InitializeMintAccounts, InitializeMintData>),
    InitializeAccount(ReadableInstruction<InitializeAccountAccounts, ()>),
    InitializeAccount2(ReadableInstruction<InitializeAccount2Accounts, InitializeAccountData2>),
    InitializeAccount3(ReadableInstruction<InitializeAccount2Accounts, InitializeAccountData2>),
    InitializeMultisig(ReadableInstruction<InitializeMultisigAccounts, InitializeMultisigData>),
    Approve(ReadableInstruction<ApproveAccounts, ApproveData>),
    Revoke(ReadableInstruction<RevokeAccounts, ()>),
    SetAuthority(ReadableInstruction<SetAuthorityAccounts, SetAuthorityData>),
    MintTo(ReadableInstruction<MintToAccounts, MintToData>),
    Burn(ReadableInstruction<BurnAccounts, BurnData>),
    CloseAccount(ReadableInstruction<CloseAccountAccounts, ()>),
    FreezeAccount(ReadableInstruction<FreezeAccountAccounts, ()>),
    ThawAccount(ReadableInstruction<ThawAccountAccounts, ()>),
    TransferChecked(ReadableInstruction<TransferCheckedAccounts, TransferCheckedData>),
    ApproveChecked(ReadableInstruction<ApproveCheckedAccounts, ApproveCheckedData>),
    MintToChecked(ReadableInstruction<MintToCheckedAccounts, MintToCheckedData>),
    BurnChecked(ReadableInstruction<BurnCheckedAccounts, BurnCheckedData>),
    SyncNative(ReadableInstruction<SyncNativeAccounts, ()>),
    GetAccountDataSize(ReadableInstruction<GetAccountDataSizeAccounts, ()>),
    InitializeImmutableOwner(ReadableInstruction<InitializeImmutableOwnerAccounts, ()>),
    AmountToUiAmount(ReadableInstruction<AmountToUiAmountAccounts, AmountToUiAmountData>),
    UiAmountToAmount(ReadableInstruction<UiAmountToAmountAccounts, UiAmountToAmountData>),
}
