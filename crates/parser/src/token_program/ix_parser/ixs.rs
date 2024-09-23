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
        FromCOptionPubkeyToOptString, FromOptVecToDefVec, FromOptionToProtoOption, IntoProtoData,
    };

    impl IntoProtoData<TransferAccountsProto> for TransferAccounts {
        fn into_proto_data(self) -> TransferAccountsProto {
            TransferAccountsProto {
                source: self.source.to_string(),
                destination: self.destination.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<TransferDataProto> for TransferData {
        fn into_proto_data(self) -> TransferDataProto {
            TransferDataProto {
                amount: self.amount,
            }
        }
    }

    impl IntoProtoData<InitializeMintAccountsProto> for InitializeMintAccounts {
        fn into_proto_data(self) -> InitializeMintAccountsProto {
            InitializeMintAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProtoData<InitializeMintDataProto> for InitializeMintData {
        fn into_proto_data(self) -> InitializeMintDataProto {
            InitializeMintDataProto {
                decimals: self.decimals.into(),
                mint_authority: self.mint_authority.to_opt_string(),
                freeze_authority: self.freeze_authority.to_opt_string(),
            }
        }
    }

    impl IntoProtoData<InitializeAccountAccountsProto> for InitializeAccountAccounts {
        fn into_proto_data(self) -> InitializeAccountAccountsProto {
            InitializeAccountAccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
                owner: self.owner.to_string(),
            }
        }
    }

    impl IntoProtoData<InitializeAccount2AccountsProto> for InitializeAccount2Accounts {
        fn into_proto_data(self) -> InitializeAccount2AccountsProto {
            InitializeAccount2AccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProtoData<InitializeAccountData2Proto> for InitializeAccountData2 {
        fn into_proto_data(self) -> InitializeAccountData2Proto {
            InitializeAccountData2Proto {
                owner: self.owner.to_string(),
            }
        }
    }

    impl IntoProtoData<InitializeMultisigAccountsProto> for InitializeMultisigAccounts {
        fn into_proto_data(self) -> InitializeMultisigAccountsProto {
            InitializeMultisigAccountsProto {
                multisig: self.multisig.to_string(),
                signers: self.signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<InitializeMultisigDataProto> for InitializeMultisigData {
        fn into_proto_data(self) -> InitializeMultisigDataProto {
            InitializeMultisigDataProto { m: self.m.into() }
        }
    }

    impl IntoProtoData<ApproveAccountsProto> for ApproveAccounts {
        fn into_proto_data(self) -> ApproveAccountsProto {
            ApproveAccountsProto {
                source: self.source.to_string(),
                delegate: self.delegate.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<ApproveDataProto> for ApproveData {
        fn into_proto_data(self) -> ApproveDataProto {
            ApproveDataProto {
                amount: self.amount,
            }
        }
    }

    impl IntoProtoData<RevokeAccountsProto> for RevokeAccounts {
        fn into_proto_data(self) -> RevokeAccountsProto {
            RevokeAccountsProto {
                source: self.source.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<SetAuthorityAccountsProto> for SetAuthorityAccounts {
        fn into_proto_data(self) -> SetAuthorityAccountsProto {
            SetAuthorityAccountsProto {
                current_authority: self.current_authority.to_string(),
                account: self.account.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<SetAuthorityDataProto> for SetAuthorityData {
        fn into_proto_data(self) -> SetAuthorityDataProto {
            SetAuthorityDataProto {
                authority_type: self.authority_type as i32,
                new_authority: self.new_authority.to_opt_string(),
            }
        }
    }

    impl IntoProtoData<MintToAccountsProto> for MintToAccounts {
        fn into_proto_data(self) -> MintToAccountsProto {
            MintToAccountsProto {
                mint: self.mint.to_string(),
                account: self.account.to_string(),
                mint_authority: self.mint_authority.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<MintToDataProto> for MintToData {
        fn into_proto_data(self) -> MintToDataProto {
            MintToDataProto {
                amount: self.amount,
            }
        }
    }

    impl IntoProtoData<BurnAccountsProto> for BurnAccounts {
        fn into_proto_data(self) -> BurnAccountsProto {
            BurnAccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<BurnDataProto> for BurnData {
        fn into_proto_data(self) -> BurnDataProto {
            BurnDataProto {
                amount: self.amount,
            }
        }
    }

    impl IntoProtoData<CloseAccountAccountsProto> for CloseAccountAccounts {
        fn into_proto_data(self) -> CloseAccountAccountsProto {
            CloseAccountAccountsProto {
                account: self.account.to_string(),
                destination: self.destination.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<FreezeAccountAccountsProto> for FreezeAccountAccounts {
        fn into_proto_data(self) -> FreezeAccountAccountsProto {
            FreezeAccountAccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
                mint_freeze_authority: self.mint_freeze_authority.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<ThawAccountAccountsProto> for ThawAccountAccounts {
        fn into_proto_data(self) -> ThawAccountAccountsProto {
            ThawAccountAccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
                mint_freeze_authority: self.mint_freeze_authority.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<TransferCheckedAccountsProto> for TransferCheckedAccounts {
        fn into_proto_data(self) -> TransferCheckedAccountsProto {
            TransferCheckedAccountsProto {
                source: self.source.to_string(),
                mint: self.mint.to_string(),
                destination: self.destination.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<TransferCheckedDataProto> for TransferCheckedData {
        fn into_proto_data(self) -> TransferCheckedDataProto {
            TransferCheckedDataProto {
                amount: self.amount,
                decimals: self.decimals.into(),
            }
        }
    }

    impl IntoProtoData<ApproveCheckedAccountsProto> for ApproveCheckedAccounts {
        fn into_proto_data(self) -> ApproveCheckedAccountsProto {
            ApproveCheckedAccountsProto {
                source: self.source.to_string(),
                mint: self.mint.to_string(),
                delegate: self.delegate.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<ApproveCheckedDataProto> for ApproveCheckedData {
        fn into_proto_data(self) -> ApproveCheckedDataProto {
            ApproveCheckedDataProto {
                amount: self.amount,
                decimals: self.decimals.into(),
            }
        }
    }

    impl IntoProtoData<MintToCheckedAccountsProto> for MintToCheckedAccounts {
        fn into_proto_data(self) -> MintToCheckedAccountsProto {
            MintToCheckedAccountsProto {
                mint: self.mint.to_string(),
                account: self.account.to_string(),
                mint_authority: self.mint_authority.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<MintToCheckedDataProto> for MintToCheckedData {
        fn into_proto_data(self) -> MintToCheckedDataProto {
            MintToCheckedDataProto {
                amount: self.amount,
                decimals: self.decimals.into(),
            }
        }
    }

    impl IntoProtoData<BurnCheckedAccountsProto> for BurnCheckedAccounts {
        fn into_proto_data(self) -> BurnCheckedAccountsProto {
            BurnCheckedAccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<BurnCheckedDataProto> for BurnCheckedData {
        fn into_proto_data(self) -> BurnCheckedDataProto {
            BurnCheckedDataProto {
                amount: self.amount,
                decimals: self.decimals.into(),
            }
        }
    }

    impl IntoProtoData<SyncNativeAccountsProto> for SyncNativeAccounts {
        fn into_proto_data(self) -> SyncNativeAccountsProto {
            SyncNativeAccountsProto {
                account: self.account.to_string(),
            }
        }
    }

    impl IntoProtoData<GetAccountDataSizeAccountsProto> for GetAccountDataSizeAccounts {
        fn into_proto_data(self) -> GetAccountDataSizeAccountsProto {
            GetAccountDataSizeAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProtoData<InitializeImmutableOwnerAccountsProto> for InitializeImmutableOwnerAccounts {
        fn into_proto_data(self) -> InitializeImmutableOwnerAccountsProto {
            InitializeImmutableOwnerAccountsProto {
                account: self.account.to_string(),
            }
        }
    }

    impl IntoProtoData<AmountToUiAmountAccountsProto> for AmountToUiAmountAccounts {
        fn into_proto_data(self) -> AmountToUiAmountAccountsProto {
            AmountToUiAmountAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProtoData<AmountToUiAmountDataProto> for AmountToUiAmountData {
        fn into_proto_data(self) -> AmountToUiAmountDataProto {
            AmountToUiAmountDataProto {
                amount: self.amount,
            }
        }
    }

    impl IntoProtoData<UiAmountToAmountAccountsProto> for UiAmountToAmountAccounts {
        fn into_proto_data(self) -> UiAmountToAmountAccountsProto {
            UiAmountToAmountAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProtoData<UiAmountToAmountDataProto> for UiAmountToAmountData {
        fn into_proto_data(self) -> UiAmountToAmountDataProto {
            UiAmountToAmountDataProto {
                ui_amount: self.ui_amount,
            }
        }
    }
    impl IntoProtoData<TokenProgramIxProto> for TokenProgramIx {
        #[allow(clippy::too_many_lines)]
        fn into_proto_data(self) -> TokenProgramIxProto {
            match self {
                TokenProgramIx::Transfer(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::Transfer(TransferIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },
                TokenProgramIx::InitializeMint(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::InitializeMint(InitializeMintIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },

                TokenProgramIx::InitializeAccount(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::InitializeAccount(InitializeAccountIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: None,
                    })),
                },

                TokenProgramIx::InitializeAccount2(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::InitializeAccount2(InitializeAccount2IxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },

                TokenProgramIx::InitializeAccount3(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::InitializeAccount3(InitializeAccount3IxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },

                TokenProgramIx::InitializeMultisig(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::InitializeMultisig(InitializeMultisigIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },

                TokenProgramIx::Approve(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::Approve(ApproveIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },

                TokenProgramIx::Revoke(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::Revoke(RevokeIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                    })),
                },

                TokenProgramIx::SetAuthority(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::SetAuthority(SetAuthorityIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },

                TokenProgramIx::MintTo(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::MintTo(MintToIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },

                TokenProgramIx::Burn(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::Burn(BurnIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },

                TokenProgramIx::CloseAccount(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::CloseAccount(CloseAccountIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                    })),
                },

                TokenProgramIx::FreezeAccount(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::FreezeAccount(FreezeAccountIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                    })),
                },

                TokenProgramIx::ThawAccount(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::ThawAccount(ThawAccountIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                    })),
                },

                TokenProgramIx::TransferChecked(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::TransferChecked(TransferCheckedIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },

                TokenProgramIx::ApproveChecked(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::ApproveChecked(ApproveCheckedIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },

                TokenProgramIx::MintToChecked(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::MintToChecked(MintToCheckedIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },

                TokenProgramIx::BurnChecked(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::BurnChecked(BurnCheckedIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },

                TokenProgramIx::SyncNative(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::SyncNative(SyncNativeIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                    })),
                },

                TokenProgramIx::GetAccountDataSize(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::GetAccountDataSize(GetAccountDataSizeIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                    })),
                },

                TokenProgramIx::InitializeImmutableOwner(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::InitializeImmutableOwner(
                        InitializeImmutableOwnerIxProto {
                            accounts: Some(ri.accounts.into_proto_data()),
                        },
                    )),
                },

                TokenProgramIx::AmountToUiAmount(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::AmountToUiAmount(AmountToUiAmountIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },

                TokenProgramIx::UiAmountToAmount(ri) => TokenProgramIxProto {
                    ix_oneof: Some(IxOneof::UiAmountToAmount(UiAmountToAmountIxProto {
                        accounts: Some(ri.accounts.into_proto_data()),
                        data: ri.data.to_proto_option(),
                    })),
                },
            }
        }
    }
}
