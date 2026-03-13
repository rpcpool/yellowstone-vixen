pub use yellowstone_vixen_core::Pubkey;
use yellowstone_vixen_proc_macro::vixen;

#[vixen(enumeration)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AuthorityType {
    MintTokens = 0,
    FreezeAccount = 1,
    AccountOwner = 2,
    CloseAccount = 3,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct TransferAccounts {
    pub source: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct TransferCheckedArgs {
    pub amount: u64,
    pub decimals: u32, // u8 -> uint32 in proto
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct TransferArgs {
    pub amount: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeMintAccounts {
    pub mint: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeMintArgs {
    pub decimals: u32, // u8 -> uint32
    pub mint_authority: Pubkey,
    pub freeze_authority: Option<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeAccountAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeAccount2Accounts {
    pub account: Pubkey,
    pub mint: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeAccount2Args {
    pub owner: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeMultisigAccounts {
    pub multisig: Pubkey,
    pub signers: Vec<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeMultisigArgs {
    pub m: u32, // u8 -> uint32
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ApproveAccounts {
    pub source: Pubkey,
    pub delegate: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ApproveArgs {
    pub amount: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct RevokeAccounts {
    pub source: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetAuthorityAccounts {
    pub current_authority: Pubkey,
    pub account: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetAuthorityArgs {
    #[hint(enumeration = "AuthorityType")]
    pub authority_type: i32,
    pub new_authority: Option<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct MintToAccounts {
    pub mint: Pubkey,
    pub account: Pubkey,
    pub mint_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct MintToArgs {
    pub amount: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct BurnAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct BurnArgs {
    pub amount: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct CloseAccountAccounts {
    pub account: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct FreezeAccountAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub mint_freeze_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ThawAccountAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub mint_freeze_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct TransferCheckedAccounts {
    pub source: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ApproveCheckedAccounts {
    pub source: Pubkey,
    pub mint: Pubkey,
    pub delegate: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ApproveCheckedArgs {
    pub amount: u64,
    pub decimals: u32,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct MintToCheckedAccounts {
    pub mint: Pubkey,
    pub account: Pubkey,
    pub mint_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct MintToCheckedArgs {
    pub amount: u64,
    pub decimals: u32,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct BurnCheckedAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct BurnCheckedArgs {
    pub amount: u64,
    pub decimals: u32,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SyncNativeAccounts {
    pub account: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct GetAccountDataSizeAccounts {
    pub mint: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeImmutableOwnerAccounts {
    pub account: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct AmountToUiAmountAccounts {
    pub mint: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct AmountToUiAmountArgs {
    pub amount: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UiAmountToAmountAccounts {
    pub mint: Pubkey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UiAmountToAmountArgs {
    pub ui_amount: String,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct TokenProgram {
    #[hint(
        oneof = "instruction::Instruction",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23"
    )]
    pub instruction: Option<instruction::Instruction>,
}

pub mod instruction {
    use super::vixen;

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Transfer {
        pub accounts: super::TransferAccounts,
        pub args: super::TransferArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeMint {
        pub accounts: super::InitializeMintAccounts,
        pub args: super::InitializeMintArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeAccount {
        pub accounts: super::InitializeAccountAccounts,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeAccount2 {
        pub accounts: super::InitializeAccount2Accounts,
        pub args: super::InitializeAccount2Args,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeAccount3 {
        pub accounts: super::InitializeAccount2Accounts,
        pub args: super::InitializeAccount2Args,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeMultisig {
        pub accounts: super::InitializeMultisigAccounts,
        pub args: super::InitializeMultisigArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Approve {
        pub accounts: super::ApproveAccounts,
        pub args: super::ApproveArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Revoke {
        pub accounts: super::RevokeAccounts,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct SetAuthority {
        pub accounts: super::SetAuthorityAccounts,
        pub args: super::SetAuthorityArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct MintTo {
        pub accounts: super::MintToAccounts,
        pub args: super::MintToArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Burn {
        pub accounts: super::BurnAccounts,
        pub args: super::BurnArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct CloseAccount {
        pub accounts: super::CloseAccountAccounts,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct FreezeAccount {
        pub accounts: super::FreezeAccountAccounts,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct ThawAccount {
        pub accounts: super::ThawAccountAccounts,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct TransferChecked {
        pub accounts: super::TransferCheckedAccounts,
        pub args: super::TransferCheckedArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct ApproveChecked {
        pub accounts: super::ApproveCheckedAccounts,
        pub args: super::ApproveCheckedArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct MintToChecked {
        pub accounts: super::MintToCheckedAccounts,
        pub args: super::MintToCheckedArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct BurnChecked {
        pub accounts: super::BurnCheckedAccounts,
        pub args: super::BurnCheckedArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct SyncNative {
        pub accounts: super::SyncNativeAccounts,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct GetAccountDataSize {
        pub accounts: super::GetAccountDataSizeAccounts,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeImmutableOwner {
        pub accounts: super::InitializeImmutableOwnerAccounts,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct AmountToUiAmount {
        pub accounts: super::AmountToUiAmountAccounts,
        pub args: super::AmountToUiAmountArgs,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct UiAmountToAmount {
        pub accounts: super::UiAmountToAmountAccounts,
        pub args: super::UiAmountToAmountArgs,
    }

    #[vixen(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Instruction {
        Transfer(Transfer),
        InitializeMint(InitializeMint),
        InitializeAccount(InitializeAccount),
        InitializeAccount2(InitializeAccount2),
        InitializeAccount3(InitializeAccount3),
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
}
