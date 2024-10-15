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
pub struct InitializeAccountData2 {
    pub owner: Pubkey,
}

#[derive(Debug, Clone)]
pub struct InitializeMultisigAccounts {
    pub multisig: Pubkey,
    pub signers: Vec<Pubkey>,
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
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]
pub struct ApproveData {
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
pub struct SetAuthorityData {
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
pub struct MintToData {
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
pub struct BurnData {
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
pub struct ApproveCheckedData {
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
pub struct MintToCheckedData {
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
    Transfer(TransferAccounts, TransferData),
    InitializeMint(InitializeMintAccounts, InitializeMintData),
    InitializeAccount(InitializeAccountAccounts),
    InitializeAccount2(InitializeAccount2Accounts, InitializeAccountData2),
    InitializeAccount3(InitializeAccount2Accounts, InitializeAccountData2),
    InitializeMultisig(InitializeMultisigAccounts, InitializeMultisigData),
    Approve(ApproveAccounts, ApproveData),
    Revoke(RevokeAccounts),
    SetAuthority(SetAuthorityAccounts, SetAuthorityData),
    MintTo(MintToAccounts, MintToData),
    Burn(BurnAccounts, BurnData),
    CloseAccount(CloseAccountAccounts),
    FreezeAccount(FreezeAccountAccounts),
    ThawAccount(ThawAccountAccounts),
    TransferChecked(TransferCheckedAccounts, TransferCheckedData),
    ApproveChecked(ApproveCheckedAccounts, ApproveCheckedData),
    MintToChecked(MintToCheckedAccounts, MintToCheckedData),
    BurnChecked(BurnCheckedAccounts, BurnCheckedData),
    SyncNative(SyncNativeAccounts),
    GetAccountDataSize(GetAccountDataSizeAccounts),
    InitializeImmutableOwner(InitializeImmutableOwnerAccounts),
    AmountToUiAmount(AmountToUiAmountAccounts, AmountToUiAmountData),
    UiAmountToAmount(UiAmountToAmountAccounts, UiAmountToAmountData),
}

#[cfg(feature = "proto")]
pub mod proto_parser {
    use token_program_ix_proto::IxOneof;
    use yellowstone_vixen_proto::parser::{
        token_program_ix_proto, AmountToUiAmountAccountsProto, AmountToUiAmountDataProto,
        AmountToUiAmountIxProto, ApproveAccountsProto, ApproveCheckedAccountsProto,
        ApproveCheckedDataProto, ApproveCheckedIxProto, ApproveDataProto, ApproveIxProto,
        BurnAccountsProto, BurnCheckedAccountsProto, BurnCheckedDataProto, BurnCheckedIxProto,
        BurnDataProto, BurnIxProto, CloseAccountAccountsProto, CloseAccountIxProto,
        FreezeAccountAccountsProto, FreezeAccountIxProto, GetAccountDataSizeAccountsProto,
        GetAccountDataSizeIxProto, InitializeAccount2AccountsProto, InitializeAccount2IxProto,
        InitializeAccount3IxProto, InitializeAccountAccountsProto, InitializeAccountData2Proto,
        InitializeAccountIxProto, InitializeImmutableOwnerAccountsProto,
        InitializeImmutableOwnerIxProto, InitializeMintAccountsProto, InitializeMintDataProto,
        InitializeMintIxProto, InitializeMultisigAccountsProto, InitializeMultisigDataProto,
        InitializeMultisigIxProto, MintToAccountsProto, MintToCheckedAccountsProto,
        MintToCheckedDataProto, MintToCheckedIxProto, MintToDataProto, MintToIxProto,
        RevokeAccountsProto, RevokeIxProto, SetAuthorityAccountsProto, SetAuthorityDataProto,
        SetAuthorityIxProto, SyncNativeAccountsProto, SyncNativeIxProto, ThawAccountAccountsProto,
        ThawAccountIxProto, TokenProgramIxProto, TransferAccountsProto,
        TransferCheckedAccountsProto, TransferCheckedDataProto, TransferCheckedIxProto,
        TransferDataProto, TransferIxProto, UiAmountToAmountAccountsProto,
        UiAmountToAmountDataProto, UiAmountToAmountIxProto,
    };

    use super::{
        AmountToUiAmountAccounts, AmountToUiAmountData, ApproveAccounts, ApproveCheckedAccounts,
        ApproveCheckedData, ApproveData, BurnAccounts, BurnCheckedAccounts, BurnCheckedData,
        BurnData, CloseAccountAccounts, FreezeAccountAccounts, GetAccountDataSizeAccounts,
        InitializeAccount2Accounts, InitializeAccountAccounts, InitializeAccountData2,
        InitializeImmutableOwnerAccounts, InitializeMintAccounts, InitializeMintData,
        InitializeMultisigAccounts, InitializeMultisigData, MintToAccounts, MintToCheckedAccounts,
        MintToCheckedData, MintToData, RevokeAccounts, SetAuthorityAccounts, SetAuthorityData,
        SyncNativeAccounts, ThawAccountAccounts, TokenProgramIx, TransferAccounts,
        TransferCheckedAccounts, TransferCheckedData, TransferData, UiAmountToAmountAccounts,
        UiAmountToAmountData,
    };
    use crate::helpers::{
        proto::{FromOptPubkeyToOptString, FromVecPubkeyToVecString},
        IntoProto,
    };

    impl IntoProto<TransferAccountsProto> for TransferAccounts {
        fn into_proto(self) -> TransferAccountsProto {
            TransferAccountsProto {
                source: self.source.to_string(),
                destination: self.destination.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<TransferDataProto> for TransferData {
        fn into_proto(self) -> TransferDataProto {
            TransferDataProto {
                amount: self.amount,
            }
        }
    }

    impl IntoProto<InitializeMintAccountsProto> for InitializeMintAccounts {
        fn into_proto(self) -> InitializeMintAccountsProto {
            InitializeMintAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProto<InitializeMintDataProto> for InitializeMintData {
        fn into_proto(self) -> InitializeMintDataProto {
            InitializeMintDataProto {
                decimals: self.decimals.into(),
                mint_authority: self.mint_authority.to_opt_string(),
                freeze_authority: self.freeze_authority.to_opt_string(),
            }
        }
    }

    impl IntoProto<InitializeAccountAccountsProto> for InitializeAccountAccounts {
        fn into_proto(self) -> InitializeAccountAccountsProto {
            InitializeAccountAccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
                owner: self.owner.to_string(),
            }
        }
    }

    impl IntoProto<InitializeAccount2AccountsProto> for InitializeAccount2Accounts {
        fn into_proto(self) -> InitializeAccount2AccountsProto {
            InitializeAccount2AccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProto<InitializeAccountData2Proto> for InitializeAccountData2 {
        fn into_proto(self) -> InitializeAccountData2Proto {
            InitializeAccountData2Proto {
                owner: self.owner.to_string(),
            }
        }
    }

    impl IntoProto<InitializeMultisigAccountsProto> for InitializeMultisigAccounts {
        fn into_proto(self) -> InitializeMultisigAccountsProto {
            InitializeMultisigAccountsProto {
                multisig: self.multisig.to_string(),
                signers: self.signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<InitializeMultisigDataProto> for InitializeMultisigData {
        fn into_proto(self) -> InitializeMultisigDataProto {
            InitializeMultisigDataProto { m: self.m.into() }
        }
    }

    impl IntoProto<ApproveAccountsProto> for ApproveAccounts {
        fn into_proto(self) -> ApproveAccountsProto {
            ApproveAccountsProto {
                source: self.source.to_string(),
                delegate: self.delegate.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<ApproveDataProto> for ApproveData {
        fn into_proto(self) -> ApproveDataProto {
            ApproveDataProto {
                amount: self.amount,
            }
        }
    }

    impl IntoProto<RevokeAccountsProto> for RevokeAccounts {
        fn into_proto(self) -> RevokeAccountsProto {
            RevokeAccountsProto {
                source: self.source.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<SetAuthorityAccountsProto> for SetAuthorityAccounts {
        fn into_proto(self) -> SetAuthorityAccountsProto {
            SetAuthorityAccountsProto {
                current_authority: self.current_authority.to_string(),
                account: self.account.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<SetAuthorityDataProto> for SetAuthorityData {
        fn into_proto(self) -> SetAuthorityDataProto {
            SetAuthorityDataProto {
                authority_type: self.authority_type as i32,
                new_authority: self.new_authority.to_opt_string(),
            }
        }
    }

    impl IntoProto<MintToAccountsProto> for MintToAccounts {
        fn into_proto(self) -> MintToAccountsProto {
            MintToAccountsProto {
                mint: self.mint.to_string(),
                account: self.account.to_string(),
                mint_authority: self.mint_authority.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<MintToDataProto> for MintToData {
        fn into_proto(self) -> MintToDataProto {
            MintToDataProto {
                amount: self.amount,
            }
        }
    }

    impl IntoProto<BurnAccountsProto> for BurnAccounts {
        fn into_proto(self) -> BurnAccountsProto {
            BurnAccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<BurnDataProto> for BurnData {
        fn into_proto(self) -> BurnDataProto {
            BurnDataProto {
                amount: self.amount,
            }
        }
    }

    impl IntoProto<CloseAccountAccountsProto> for CloseAccountAccounts {
        fn into_proto(self) -> CloseAccountAccountsProto {
            CloseAccountAccountsProto {
                account: self.account.to_string(),
                destination: self.destination.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<FreezeAccountAccountsProto> for FreezeAccountAccounts {
        fn into_proto(self) -> FreezeAccountAccountsProto {
            FreezeAccountAccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
                mint_freeze_authority: self.mint_freeze_authority.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<ThawAccountAccountsProto> for ThawAccountAccounts {
        fn into_proto(self) -> ThawAccountAccountsProto {
            ThawAccountAccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
                mint_freeze_authority: self.mint_freeze_authority.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<TransferCheckedAccountsProto> for TransferCheckedAccounts {
        fn into_proto(self) -> TransferCheckedAccountsProto {
            TransferCheckedAccountsProto {
                source: self.source.to_string(),
                mint: self.mint.to_string(),
                destination: self.destination.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<TransferCheckedDataProto> for TransferCheckedData {
        fn into_proto(self) -> TransferCheckedDataProto {
            TransferCheckedDataProto {
                amount: self.amount,
                decimals: self.decimals.into(),
            }
        }
    }

    impl IntoProto<ApproveCheckedAccountsProto> for ApproveCheckedAccounts {
        fn into_proto(self) -> ApproveCheckedAccountsProto {
            ApproveCheckedAccountsProto {
                source: self.source.to_string(),
                mint: self.mint.to_string(),
                delegate: self.delegate.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<ApproveCheckedDataProto> for ApproveCheckedData {
        fn into_proto(self) -> ApproveCheckedDataProto {
            ApproveCheckedDataProto {
                amount: self.amount,
                decimals: self.decimals.into(),
            }
        }
    }

    impl IntoProto<MintToCheckedAccountsProto> for MintToCheckedAccounts {
        fn into_proto(self) -> MintToCheckedAccountsProto {
            MintToCheckedAccountsProto {
                mint: self.mint.to_string(),
                account: self.account.to_string(),
                mint_authority: self.mint_authority.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<MintToCheckedDataProto> for MintToCheckedData {
        fn into_proto(self) -> MintToCheckedDataProto {
            MintToCheckedDataProto {
                amount: self.amount,
                decimals: self.decimals.into(),
            }
        }
    }

    impl IntoProto<BurnCheckedAccountsProto> for BurnCheckedAccounts {
        fn into_proto(self) -> BurnCheckedAccountsProto {
            BurnCheckedAccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<BurnCheckedDataProto> for BurnCheckedData {
        fn into_proto(self) -> BurnCheckedDataProto {
            BurnCheckedDataProto {
                amount: self.amount,
                decimals: self.decimals.into(),
            }
        }
    }

    impl IntoProto<SyncNativeAccountsProto> for SyncNativeAccounts {
        fn into_proto(self) -> SyncNativeAccountsProto {
            SyncNativeAccountsProto {
                account: self.account.to_string(),
            }
        }
    }

    impl IntoProto<GetAccountDataSizeAccountsProto> for GetAccountDataSizeAccounts {
        fn into_proto(self) -> GetAccountDataSizeAccountsProto {
            GetAccountDataSizeAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProto<InitializeImmutableOwnerAccountsProto> for InitializeImmutableOwnerAccounts {
        fn into_proto(self) -> InitializeImmutableOwnerAccountsProto {
            InitializeImmutableOwnerAccountsProto {
                account: self.account.to_string(),
            }
        }
    }

    impl IntoProto<AmountToUiAmountAccountsProto> for AmountToUiAmountAccounts {
        fn into_proto(self) -> AmountToUiAmountAccountsProto {
            AmountToUiAmountAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProto<AmountToUiAmountDataProto> for AmountToUiAmountData {
        fn into_proto(self) -> AmountToUiAmountDataProto {
            AmountToUiAmountDataProto {
                amount: self.amount,
            }
        }
    }

    impl IntoProto<UiAmountToAmountAccountsProto> for UiAmountToAmountAccounts {
        fn into_proto(self) -> UiAmountToAmountAccountsProto {
            UiAmountToAmountAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProto<UiAmountToAmountDataProto> for UiAmountToAmountData {
        fn into_proto(self) -> UiAmountToAmountDataProto {
            UiAmountToAmountDataProto {
                ui_amount: self.ui_amount,
            }
        }
    }
    impl IntoProto<TokenProgramIxProto> for TokenProgramIx {
        #[allow(clippy::too_many_lines)]
        fn into_proto(self) -> TokenProgramIxProto {
            match self {
                TokenProgramIx::Transfer(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::Transfer(TransferIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },
                TokenProgramIx::InitializeMint(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::InitializeMint(InitializeMintIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenProgramIx::InitializeAccount(acc) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::InitializeAccount(InitializeAccountIxProto {
                        accounts: Some(acc.into_proto()),
                        data: None,
                    })),
                },

                TokenProgramIx::InitializeAccount2(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::InitializeAccount2(InitializeAccount2IxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenProgramIx::InitializeAccount3(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::InitializeAccount3(InitializeAccount3IxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenProgramIx::InitializeMultisig(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::InitializeMultisig(InitializeMultisigIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenProgramIx::Approve(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::Approve(ApproveIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenProgramIx::Revoke(acc) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::Revoke(RevokeIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                TokenProgramIx::SetAuthority(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::SetAuthority(SetAuthorityIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenProgramIx::MintTo(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::MintTo(MintToIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenProgramIx::Burn(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::Burn(BurnIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenProgramIx::CloseAccount(acc) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::CloseAccount(CloseAccountIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                TokenProgramIx::FreezeAccount(acc) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::FreezeAccount(FreezeAccountIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                TokenProgramIx::ThawAccount(acc) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::ThawAccount(ThawAccountIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                TokenProgramIx::TransferChecked(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::TransferChecked(TransferCheckedIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenProgramIx::ApproveChecked(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::ApproveChecked(ApproveCheckedIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenProgramIx::MintToChecked(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::MintToChecked(MintToCheckedIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenProgramIx::BurnChecked(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::BurnChecked(BurnCheckedIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenProgramIx::SyncNative(acc) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::SyncNative(SyncNativeIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                TokenProgramIx::GetAccountDataSize(acc) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::GetAccountDataSize(GetAccountDataSizeIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                TokenProgramIx::InitializeImmutableOwner(acc) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::InitializeImmutableOwner(
                        InitializeImmutableOwnerIxProto {
                            accounts: Some(acc.into_proto()),
                        },
                    )),
                },

                TokenProgramIx::AmountToUiAmount(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::AmountToUiAmount(AmountToUiAmountIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },

                TokenProgramIx::UiAmountToAmount(acc, data) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::UiAmountToAmount(UiAmountToAmountIxProto {
                        accounts: Some(acc.into_proto()),
                        data: Some(data.into_proto()),
                    })),
                },
            }
        }
    }
}
