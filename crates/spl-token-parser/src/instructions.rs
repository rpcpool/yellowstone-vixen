use prost::alloc::{string::String, vec::Vec};

pub type PubkeyBytes = Vec<u8>; // expected len = 32

#[derive(Clone, Copy, Debug, PartialEq, Eq, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthorityTypeProto {
    MintTokens = 0,
    FreezeAccount = 1,
    AccountOwner = 2,
    CloseAccount = 3,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub source: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub destination: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferCheckedArgs {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
    #[prost(uint32, tag = "2")]
    pub decimals: u32, // u8 -> uint32 in proto
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferArgs {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeMintAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeMintArgs {
    #[prost(uint32, tag = "1")]
    pub decimals: u32, // u8 -> uint32
    #[prost(bytes = "vec", tag = "2")]
    pub mint_authority: PubkeyBytes,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub freeze_authority: Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeAccountAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub owner: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeAccount2Accounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeAccount2Args {
    #[prost(bytes = "vec", tag = "1")]
    pub owner: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeMultisigAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub multisig: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeMultisigArgs {
    #[prost(uint32, tag = "1")]
    pub m: u32, // u8 -> uint32
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub source: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub delegate: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveArgs {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub source: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAuthorityAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub current_authority: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAuthorityArgs {
    #[prost(enumeration = "AuthorityTypeProto", tag = "1")]
    pub authority_type: i32,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub new_authority: Option<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintToAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub mint_authority: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintToArgs {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnArgs {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseAccountAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub destination: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreezeAccountAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub mint_freeze_authority: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThawAccountAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub mint_freeze_authority: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferCheckedAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub source: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub destination: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveCheckedAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub source: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub delegate: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveCheckedArgs {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
    #[prost(uint32, tag = "2")]
    pub decimals: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintToCheckedAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub mint_authority: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintToCheckedArgs {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
    #[prost(uint32, tag = "2")]
    pub decimals: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnCheckedAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnCheckedArgs {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
    #[prost(uint32, tag = "2")]
    pub decimals: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncNativeAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountDataSizeAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeImmutableOwnerAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AmountToUiAmountAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AmountToUiAmountArgs {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UiAmountToAmountAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UiAmountToAmountArgs {
    #[prost(string, tag = "1")]
    pub ui_amount: String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenProgramInstruction {
    #[prost(
        oneof = "token_program_instruction::Ix",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20"
    )]
    pub ix: Option<token_program_instruction::Ix>,
}

pub mod token_program_instruction {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Transfer {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::TransferAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::TransferArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitializeMint {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::InitializeMintAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::InitializeMintArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitializeAccount {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::InitializeAccountAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitializeAccount2 {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::InitializeAccount2Accounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::InitializeAccount2Args>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitializeAccount3 {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::InitializeAccount2Accounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::InitializeAccount2Args>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitializeMultisig {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::InitializeMultisigAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::InitializeMultisigArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Approve {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::ApproveAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::ApproveArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Revoke {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::RevokeAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetAuthority {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::SetAuthorityAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::SetAuthorityArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MintTo {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::MintToAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::MintToArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Burn {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::BurnAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::BurnArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CloseAccount {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::CloseAccountAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FreezeAccount {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::FreezeAccountAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThawAccount {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::ThawAccountAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransferChecked {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::TransferCheckedAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::TransferCheckedArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApproveChecked {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::ApproveCheckedAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::ApproveCheckedArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MintToChecked {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::MintToCheckedAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::MintToCheckedArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BurnChecked {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::BurnCheckedAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::BurnCheckedArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SyncNative {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::SyncNativeAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetAccountDataSize {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::GetAccountDataSizeAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitializeImmutableOwner {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::InitializeImmutableOwnerAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AmountToUiAmount {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::AmountToUiAmountAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::AmountToUiAmountArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UiAmountToAmount {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::UiAmountToAmountAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::UiAmountToAmountArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ix {
        #[prost(message, tag = "1")]
        Transfer(Transfer),
        #[prost(message, tag = "2")]
        InitializeMint(InitializeMint),
        #[prost(message, tag = "3")]
        InitializeAccount(InitializeAccount),
        #[prost(message, tag = "4")]
        InitializeAccount2(InitializeAccount2),
        #[prost(message, tag = "5")]
        InitializeAccount3(InitializeAccount3),
        #[prost(message, tag = "6")]
        InitializeMultisig(InitializeMultisig),
        #[prost(message, tag = "7")]
        Approve(Approve),
        #[prost(message, tag = "8")]
        Revoke(Revoke),
        #[prost(message, tag = "9")]
        SetAuthority(SetAuthority),
        #[prost(message, tag = "10")]
        MintTo(MintTo),
        #[prost(message, tag = "11")]
        Burn(Burn),
        #[prost(message, tag = "12")]
        CloseAccount(CloseAccount),
        #[prost(message, tag = "13")]
        FreezeAccount(FreezeAccount),
        #[prost(message, tag = "14")]
        ThawAccount(ThawAccount),
        #[prost(message, tag = "15")]
        TransferChecked(TransferChecked),
        #[prost(message, tag = "16")]
        ApproveChecked(ApproveChecked),
        #[prost(message, tag = "17")]
        MintToChecked(MintToChecked),
        #[prost(message, tag = "18")]
        BurnChecked(BurnChecked),
        #[prost(message, tag = "19")]
        SyncNative(SyncNative),
        #[prost(message, tag = "20")]
        GetAccountDataSize(GetAccountDataSize),
        #[prost(message, tag = "21")]
        InitializeImmutableOwner(InitializeImmutableOwner),
        #[prost(message, tag = "22")]
        AmountToUiAmount(AmountToUiAmount),
        #[prost(message, tag = "23")]
        UiAmountToAmount(UiAmountToAmount),
    }
}
