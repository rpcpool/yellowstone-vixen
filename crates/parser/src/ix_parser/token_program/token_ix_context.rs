use spl_pod::solana_program::{program_option::COption, pubkey::Pubkey};
use spl_token::instruction::AuthorityType;

pub struct Transfer {
    pub source: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
}

pub struct InitializeMint {
    pub mint: Pubkey,
    pub decimals: u8,
    pub mint_authority: Pubkey,
    pub freeze_authority: Option<Pubkey>,
}

pub struct InitializeAccount {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
}

pub struct InitializeMultisig {
    pub multisig: Pubkey,
    pub signers: Vec<Pubkey>,
    pub m: u8,
}

pub struct Approve {
    pub source: Pubkey,
    pub delegate: Pubkey,
    pub amount: u64,
}

pub struct Revoke {
    pub source: Pubkey,
}

pub struct SetAuthority {
    pub owned: Pubkey,
    pub authority_type: AuthorityType,
    pub new_authority: COption<Pubkey>,
}

pub struct MintTo {
    pub mint: Pubkey,
    pub account: Pubkey,
    pub amount: u64,
}

pub struct Burn {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

pub struct CloseAccount {
    pub account: Pubkey,
    pub destination: Pubkey,
}

pub struct FreezeAccount {
    pub account: Pubkey,
    pub mint: Pubkey,
}

pub struct ThawAccount {
    pub account: Pubkey,
    pub mint: Pubkey,
}

pub struct TransferChecked {
    pub source: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
    pub decimals: u8,
}

pub struct ApproveChecked {
    pub source: Pubkey,
    pub mint: Pubkey,
    pub delegate: Pubkey,
    pub amount: u64,
    pub decimals: u8,
}

pub struct MintToChecked {
    pub mint: Pubkey,
    pub account: Pubkey,
    pub amount: u64,
    pub decimals: u8,
}

pub struct BurnChecked {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub decimals: u8,
}

pub struct SyncNative {
    pub account: Pubkey,
}

pub struct GetAccountDataSize {
    pub mint: Pubkey,
}

pub struct InitializeImmutableOwner {
    pub account: Pubkey,
}

pub struct AmountToUiAmount {
    pub mint: Pubkey,
    pub amount: u64,
}

pub struct UiAmountToAmount {
    pub mint: Pubkey,
    pub ui_amount: String,
}

pub enum TokenIxContext {
    Transfer(Transfer),
    InitializeMint(InitializeMint),
    InitializeAccount(InitializeAccount),
    InitializeMultisig(InitializeMultisig),
    Approve(Approve),
    Revoke(Revoke),
    SetAuthority(SetAuthority),
    MintTo(MintTo),
    Burn(Burn),
    CloseAccount(CloseAccount),
    FreezeAccount(FreezeAccount),
    ThawAccount(ThawAccount),
    TransferChecked(TransferChecked),
    ApproveChecked(ApproveChecked),
    MintToChecked(MintToChecked),
    BurnChecked(BurnChecked),
    SyncNative(SyncNative),
    GetAccountDataSize(GetAccountDataSize),
    InitializeImmutableOwner(InitializeImmutableOwner),
    AmountToUiAmount(AmountToUiAmount),
    UiAmountToAmount(UiAmountToAmount),
}
