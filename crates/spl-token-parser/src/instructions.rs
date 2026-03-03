pub use yellowstone_vixen_core::PublicKey;
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
    pub source: PublicKey,
    pub destination: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
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
    pub mint: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeMintArgs {
    pub decimals: u32, // u8 -> uint32
    pub mint_authority: PublicKey,
    pub freeze_authority: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeAccountAccounts {
    pub account: PublicKey,
    pub mint: PublicKey,
    pub owner: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeAccount2Accounts {
    pub account: PublicKey,
    pub mint: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeAccount2Args {
    pub owner: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeMultisigAccounts {
    pub multisig: PublicKey,
    pub signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeMultisigArgs {
    pub m: u32, // u8 -> uint32
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ApproveAccounts {
    pub source: PublicKey,
    pub delegate: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ApproveArgs {
    pub amount: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct RevokeAccounts {
    pub source: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetAuthorityAccounts {
    pub current_authority: PublicKey,
    pub account: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
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
pub struct MintToAccounts {
    pub mint: PublicKey,
    pub account: PublicKey,
    pub mint_authority: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct MintToArgs {
    pub amount: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct BurnAccounts {
    pub account: PublicKey,
    pub mint: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct BurnArgs {
    pub amount: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct CloseAccountAccounts {
    pub account: PublicKey,
    pub destination: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct FreezeAccountAccounts {
    pub account: PublicKey,
    pub mint: PublicKey,
    pub mint_freeze_authority: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ThawAccountAccounts {
    pub account: PublicKey,
    pub mint: PublicKey,
    pub mint_freeze_authority: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct TransferCheckedAccounts {
    pub source: PublicKey,
    pub mint: PublicKey,
    pub destination: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ApproveCheckedAccounts {
    pub source: PublicKey,
    pub mint: PublicKey,
    pub delegate: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
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
    pub mint: PublicKey,
    pub account: PublicKey,
    pub mint_authority: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
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
    pub account: PublicKey,
    pub mint: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
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
    pub account: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct GetAccountDataSizeAccounts {
    pub mint: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeImmutableOwnerAccounts {
    pub account: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct AmountToUiAmountAccounts {
    pub mint: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct AmountToUiAmountArgs {
    pub amount: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UiAmountToAmountAccounts {
    pub mint: PublicKey,
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
        pub accounts: Option<super::TransferAccounts>,
        pub args: Option<super::TransferArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeMint {
        pub accounts: Option<super::InitializeMintAccounts>,
        pub args: Option<super::InitializeMintArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeAccount {
        pub accounts: Option<super::InitializeAccountAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeAccount2 {
        pub accounts: Option<super::InitializeAccount2Accounts>,
        pub args: Option<super::InitializeAccount2Args>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeAccount3 {
        pub accounts: Option<super::InitializeAccount2Accounts>,
        pub args: Option<super::InitializeAccount2Args>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeMultisig {
        pub accounts: Option<super::InitializeMultisigAccounts>,
        pub args: Option<super::InitializeMultisigArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Approve {
        pub accounts: Option<super::ApproveAccounts>,
        pub args: Option<super::ApproveArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Revoke {
        pub accounts: Option<super::RevokeAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct SetAuthority {
        pub accounts: Option<super::SetAuthorityAccounts>,
        pub args: Option<super::SetAuthorityArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct MintTo {
        pub accounts: Option<super::MintToAccounts>,
        pub args: Option<super::MintToArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Burn {
        pub accounts: Option<super::BurnAccounts>,
        pub args: Option<super::BurnArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct CloseAccount {
        pub accounts: Option<super::CloseAccountAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct FreezeAccount {
        pub accounts: Option<super::FreezeAccountAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct ThawAccount {
        pub accounts: Option<super::ThawAccountAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct TransferChecked {
        pub accounts: Option<super::TransferCheckedAccounts>,
        pub args: Option<super::TransferCheckedArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct ApproveChecked {
        pub accounts: Option<super::ApproveCheckedAccounts>,
        pub args: Option<super::ApproveCheckedArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct MintToChecked {
        pub accounts: Option<super::MintToCheckedAccounts>,
        pub args: Option<super::MintToCheckedArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct BurnChecked {
        pub accounts: Option<super::BurnCheckedAccounts>,
        pub args: Option<super::BurnCheckedArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct SyncNative {
        pub accounts: Option<super::SyncNativeAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct GetAccountDataSize {
        pub accounts: Option<super::GetAccountDataSizeAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeImmutableOwner {
        pub accounts: Option<super::InitializeImmutableOwnerAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct AmountToUiAmount {
        pub accounts: Option<super::AmountToUiAmountAccounts>,
        pub args: Option<super::AmountToUiAmountArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct UiAmountToAmount {
        pub accounts: Option<super::UiAmountToAmountAccounts>,
        pub args: Option<super::UiAmountToAmountArgs>,
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
