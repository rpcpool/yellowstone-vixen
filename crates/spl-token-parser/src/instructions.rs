use yellowstone_vixen_proc_macro::vixen_proto;

pub type PubkeyBytes = Vec<u8>; // expected len = 32

#[vixen_proto(enumeration)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AuthorityTypeProto {
    MintTokens = 0,
    FreezeAccount = 1,
    AccountOwner = 2,
    CloseAccount = 3,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TransferAccounts {
    pub source: PubkeyBytes,
    pub destination: PubkeyBytes,
    pub owner: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TransferCheckedArgs {
    pub amount: u64,
    pub decimals: u32, // u8 -> uint32 in proto
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TransferArgs {
    pub amount: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeMintAccounts {
    pub mint: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeMintArgs {
    pub decimals: u32, // u8 -> uint32
    pub mint_authority: PubkeyBytes,
    pub freeze_authority: Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeAccountAccounts {
    pub account: PubkeyBytes,
    pub mint: PubkeyBytes,
    pub owner: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeAccount2Accounts {
    pub account: PubkeyBytes,
    pub mint: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeAccount2Args {
    pub owner: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeMultisigAccounts {
    pub multisig: PubkeyBytes,
    pub signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeMultisigArgs {
    pub m: u32, // u8 -> uint32
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ApproveAccounts {
    pub source: PubkeyBytes,
    pub delegate: PubkeyBytes,
    pub owner: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ApproveArgs {
    pub amount: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct RevokeAccounts {
    pub source: PubkeyBytes,
    pub owner: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetAuthorityAccounts {
    pub current_authority: PubkeyBytes,
    pub account: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetAuthorityArgs {
    #[vixen_proto_hint(enumeration = "AuthorityTypeProto")]
    pub authority_type: i32,
    pub new_authority: Option<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct MintToAccounts {
    pub mint: PubkeyBytes,
    pub account: PubkeyBytes,
    pub mint_authority: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct MintToArgs {
    pub amount: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct BurnAccounts {
    pub account: PubkeyBytes,
    pub mint: PubkeyBytes,
    pub owner: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct BurnArgs {
    pub amount: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct CloseAccountAccounts {
    pub account: PubkeyBytes,
    pub destination: PubkeyBytes,
    pub owner: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct FreezeAccountAccounts {
    pub account: PubkeyBytes,
    pub mint: PubkeyBytes,
    pub mint_freeze_authority: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ThawAccountAccounts {
    pub account: PubkeyBytes,
    pub mint: PubkeyBytes,
    pub mint_freeze_authority: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TransferCheckedAccounts {
    pub source: PubkeyBytes,
    pub mint: PubkeyBytes,
    pub destination: PubkeyBytes,
    pub owner: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ApproveCheckedAccounts {
    pub source: PubkeyBytes,
    pub mint: PubkeyBytes,
    pub delegate: PubkeyBytes,
    pub owner: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ApproveCheckedArgs {
    pub amount: u64,
    pub decimals: u32,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct MintToCheckedAccounts {
    pub mint: PubkeyBytes,
    pub account: PubkeyBytes,
    pub mint_authority: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct MintToCheckedArgs {
    pub amount: u64,
    pub decimals: u32,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct BurnCheckedAccounts {
    pub account: PubkeyBytes,
    pub mint: PubkeyBytes,
    pub owner: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct BurnCheckedArgs {
    pub amount: u64,
    pub decimals: u32,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SyncNativeAccounts {
    pub account: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct GetAccountDataSizeAccounts {
    pub mint: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeImmutableOwnerAccounts {
    pub account: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct AmountToUiAmountAccounts {
    pub mint: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct AmountToUiAmountArgs {
    pub amount: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UiAmountToAmountAccounts {
    pub mint: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct UiAmountToAmountArgs {
    pub ui_amount: String,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TokenProgramInstruction {
    #[vixen_proto_hint(
        oneof = "token_program_instruction::Ix",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23"
    )]
    pub ix: Option<token_program_instruction::Ix>,
}

pub mod token_program_instruction {
    use super::*;

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct Transfer {
        pub accounts: Option<super::TransferAccounts>,
        pub args: Option<super::TransferArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct InitializeMint {
        pub accounts: Option<super::InitializeMintAccounts>,
        pub args: Option<super::InitializeMintArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct InitializeAccount {
        pub accounts: Option<super::InitializeAccountAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct InitializeAccount2 {
        pub accounts: Option<super::InitializeAccount2Accounts>,
        pub args: Option<super::InitializeAccount2Args>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct InitializeAccount3 {
        pub accounts: Option<super::InitializeAccount2Accounts>,
        pub args: Option<super::InitializeAccount2Args>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct InitializeMultisig {
        pub accounts: Option<super::InitializeMultisigAccounts>,
        pub args: Option<super::InitializeMultisigArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct Approve {
        pub accounts: Option<super::ApproveAccounts>,
        pub args: Option<super::ApproveArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct Revoke {
        pub accounts: Option<super::RevokeAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct SetAuthority {
        pub accounts: Option<super::SetAuthorityAccounts>,
        pub args: Option<super::SetAuthorityArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct MintTo {
        pub accounts: Option<super::MintToAccounts>,
        pub args: Option<super::MintToArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct Burn {
        pub accounts: Option<super::BurnAccounts>,
        pub args: Option<super::BurnArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct CloseAccount {
        pub accounts: Option<super::CloseAccountAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct FreezeAccount {
        pub accounts: Option<super::FreezeAccountAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct ThawAccount {
        pub accounts: Option<super::ThawAccountAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct TransferChecked {
        pub accounts: Option<super::TransferCheckedAccounts>,
        pub args: Option<super::TransferCheckedArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct ApproveChecked {
        pub accounts: Option<super::ApproveCheckedAccounts>,
        pub args: Option<super::ApproveCheckedArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct MintToChecked {
        pub accounts: Option<super::MintToCheckedAccounts>,
        pub args: Option<super::MintToCheckedArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct BurnChecked {
        pub accounts: Option<super::BurnCheckedAccounts>,
        pub args: Option<super::BurnCheckedArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct SyncNative {
        pub accounts: Option<super::SyncNativeAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct GetAccountDataSize {
        pub accounts: Option<super::GetAccountDataSizeAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct InitializeImmutableOwner {
        pub accounts: Option<super::InitializeImmutableOwnerAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct AmountToUiAmount {
        pub accounts: Option<super::AmountToUiAmountAccounts>,
        pub args: Option<super::AmountToUiAmountArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct UiAmountToAmount {
        pub accounts: Option<super::UiAmountToAmountAccounts>,
        pub args: Option<super::UiAmountToAmountArgs>,
    }

    #[vixen_proto(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Ix {
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
