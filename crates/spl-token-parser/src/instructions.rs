use spl_token::instruction::AuthorityType;
use yellowstone_vixen_core::Pubkey;

#[derive(Debug, Clone)]
pub struct TransferAccounts {
    pub source: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]
pub struct TransferCheckedArgs {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct TransferArgs {
    pub amount: u64,
}
#[derive(Debug, Clone, Copy)]
pub struct InitializeMintAccounts {
    pub mint: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeMintArgs {
    pub decimals: u8,
    pub mint_authority: Pubkey,
    pub freeze_authority: Option<Pubkey>,
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
pub struct InitializeAccount2Args {
    pub owner: Pubkey,
}

#[derive(Debug, Clone)]
pub struct InitializeMultisigAccounts {
    pub multisig: Pubkey,
    pub signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeMultisigArgs {
    pub m: u8,
}

#[derive(Debug, Clone)]
pub struct ApproveAccounts {
    pub source: Pubkey,
    pub delegate: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]
pub struct ApproveArgs {
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct RevokeAccounts {
    pub source: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct SetAuthorityAccounts {
    pub current_authority: Pubkey,
    pub account: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct SetAuthorityArgs {
    pub authority_type: AuthorityType,
    pub new_authority: Option<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct MintToAccounts {
    pub mint: Pubkey,
    pub account: Pubkey,
    pub mint_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]
pub struct MintToArgs {
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct BurnAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]
pub struct BurnArgs {
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct CloseAccountAccounts {
    pub account: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct FreezeAccountAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub mint_freeze_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}
#[derive(Debug, Clone)]
pub struct ThawAccountAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub mint_freeze_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}
#[derive(Debug, Clone)]
pub struct TransferCheckedAccounts {
    pub source: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct ApproveCheckedAccounts {
    pub source: Pubkey,
    pub mint: Pubkey,
    pub delegate: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]
pub struct ApproveCheckedArgs {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(Debug, Clone)]
pub struct MintToCheckedAccounts {
    pub mint: Pubkey,
    pub account: Pubkey,
    pub mint_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]
pub struct MintToCheckedArgs {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(Debug, Clone)]
pub struct BurnCheckedAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]
pub struct BurnCheckedArgs {
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
pub struct AmountToUiAmountArgs {
    pub amount: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct UiAmountToAmountAccounts {
    pub mint: Pubkey,
}

#[derive(Debug, Clone)]
pub struct UiAmountToAmountArgs {
    pub ui_amount: String,
}

#[derive(Debug)]
pub enum TokenProgramInstruction {
    Transfer {
        accounts: TransferAccounts,
        args: TransferArgs,
    },
    InitializeMint {
        accounts: InitializeMintAccounts,
        args: InitializeMintArgs,
    },
    InitializeAccount {
        accounts: InitializeAccountAccounts,
    },
    InitializeAccount2 {
        accounts: InitializeAccount2Accounts,
        args: InitializeAccount2Args,
    },
    InitializeAccount3 {
        accounts: InitializeAccount2Accounts,
        args: InitializeAccount2Args,
    },
    InitializeMultisig {
        accounts: InitializeMultisigAccounts,
        args: InitializeMultisigArgs,
    },
    Approve {
        accounts: ApproveAccounts,
        args: ApproveArgs,
    },
    Revoke {
        accounts: RevokeAccounts,
    },
    SetAuthority {
        accounts: SetAuthorityAccounts,
        args: SetAuthorityArgs,
    },
    MintTo {
        accounts: MintToAccounts,
        args: MintToArgs,
    },
    Burn {
        accounts: BurnAccounts,
        args: BurnArgs,
    },
    CloseAccount {
        accounts: CloseAccountAccounts,
    },
    FreezeAccount {
        accounts: FreezeAccountAccounts,
    },
    ThawAccount {
        accounts: ThawAccountAccounts,
    },
    TransferChecked {
        accounts: TransferCheckedAccounts,
        args: TransferCheckedArgs,
    },
    ApproveChecked {
        accounts: ApproveCheckedAccounts,
        args: ApproveCheckedArgs,
    },
    MintToChecked {
        accounts: MintToCheckedAccounts,
        args: MintToCheckedArgs,
    },
    BurnChecked {
        accounts: BurnCheckedAccounts,
        args: BurnCheckedArgs,
    },
    SyncNative {
        accounts: SyncNativeAccounts,
    },
    GetAccountDataSize {
        accounts: GetAccountDataSizeAccounts,
    },
    InitializeImmutableOwner {
        accounts: InitializeImmutableOwnerAccounts,
    },
    AmountToUiAmount {
        accounts: AmountToUiAmountAccounts,
        args: AmountToUiAmountArgs,
    },
    UiAmountToAmount {
        accounts: UiAmountToAmountAccounts,
        args: UiAmountToAmountArgs,
    },
}
