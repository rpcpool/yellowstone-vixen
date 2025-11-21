use spl_token_2022::extension::confidential_transfer::instruction::ConfidentialTransferInstruction;
use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};

use super::helpers::{decode_extension_ix_type, ExtensionIxParser};
use crate::{helpers::check_min_accounts_req, token_program::InitializeMintAccounts, Result};

#[derive(Debug, Clone, Copy)]
pub struct UpdateMintAccounts {
    pub mint: Pubkey,
    pub authority: Pubkey,
}

#[derive(Debug, Clone)]
pub struct ConfigureAccountAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub sysvar: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]
pub struct ApproveAccountAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub authority: Pubkey,
}

#[derive(Debug)]
pub struct EmptyAccountAccounts {
    pub account: Pubkey,
    pub sysvar: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]

pub struct DepositAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct WithdrawAccounts {
    pub source_account: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]

pub struct ConfidentialTransferAccounts {
    pub source_account: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub context_account: Pubkey, // Sysvar account or context state account
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct ApplyPendingBalanceAccounts {
    pub account: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]

pub struct CreditsAccounts {
    pub account: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]

pub struct TransferWithFeeAccounts {
    pub source_account: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
}

#[derive(Debug, Clone, Copy)]

pub struct ConfigureAccountWithRegistryAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub registry: Pubkey,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum ConfidentaltransferIx {
    InitializeMint(InitializeMintAccounts),
    UpdateMint(UpdateMintAccounts),
    ConfigureAccount(ConfigureAccountAccounts),
    ApproveAccount(ApproveAccountAccounts),
    EmptyAccount(EmptyAccountAccounts),
    Deposit(DepositAccounts),
    Withdraw(WithdrawAccounts),
    Transfer(ConfidentialTransferAccounts),
    ApplyPendingBalance(ApplyPendingBalanceAccounts),
    EnableConfidentialCredits(CreditsAccounts),
    DisableConfidentialCredits(CreditsAccounts),
    EnableNonConfidentialCredits(CreditsAccounts),
    DisableNonConfidentialCredits(CreditsAccounts),
    TransferWithFee(TransferWithFeeAccounts),
    ConfigureAccountWithRegistry(ConfigureAccountWithRegistryAccounts),
}

impl ExtensionIxParser for ConfidentaltransferIx {
    #[allow(clippy::too_many_lines)]
    fn try_parse_extension_ix(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();
        let ix_type = decode_extension_ix_type(&ix.data[1..])?;
        match ix_type {
            ConfidentialTransferInstruction::InitializeMint => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(ConfidentaltransferIx::InitializeMint(
                    InitializeMintAccounts {
                        mint: ix.accounts[0],
                    },
                ))
            },
            ConfidentialTransferInstruction::UpdateMint => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::UpdateMint(UpdateMintAccounts {
                    mint: ix.accounts[0],
                    authority: ix.accounts[1],
                }))
            },
            ConfidentialTransferInstruction::ConfigureAccount => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(ConfidentaltransferIx::ConfigureAccount(
                    ConfigureAccountAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        sysvar: ix.accounts[2],
                        owner: ix.accounts[3],
                        multisig_signers: ix.accounts[4..].to_vec(),
                    },
                ))
            },

            ConfidentialTransferInstruction::ApproveAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(ConfidentaltransferIx::ApproveAccount(
                    ApproveAccountAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        authority: ix.accounts[2],
                    },
                ))
            },

            ConfidentialTransferInstruction::EmptyAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(ConfidentaltransferIx::EmptyAccount(EmptyAccountAccounts {
                    account: ix.accounts[0],
                    sysvar: ix.accounts[1],
                    owner: ix.accounts[2],
                    multisig_signers: ix.accounts[3..].to_vec(),
                }))
            },

            ConfidentialTransferInstruction::Deposit => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(ConfidentaltransferIx::Deposit(DepositAccounts {
                    account: ix.accounts[0],
                    mint: ix.accounts[1],
                    owner: ix.accounts[2],
                    multisig_signers: ix.accounts[3..].to_vec(),
                }))
            },

            ConfidentialTransferInstruction::Withdraw => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(ConfidentaltransferIx::Withdraw(WithdrawAccounts {
                    source_account: ix.accounts[0],
                    mint: ix.accounts[1],
                    destination: ix.accounts[2],
                    owner: ix.accounts[3],
                    multisig_signers: ix.accounts[4..].to_vec(),
                }))
            },

            ConfidentialTransferInstruction::Transfer => {
                check_min_accounts_req(accounts_len, 5)?;
                Ok(ConfidentaltransferIx::Transfer(
                    ConfidentialTransferAccounts {
                        source_account: ix.accounts[0],
                        mint: ix.accounts[1],
                        destination: ix.accounts[2],
                        context_account: ix.accounts[3],
                        owner: ix.accounts[4],
                        multisig_signers: ix.accounts[5..].to_vec(),
                    },
                ))
            },

            ConfidentialTransferInstruction::ApplyPendingBalance => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::ApplyPendingBalance(
                    ApplyPendingBalanceAccounts {
                        account: ix.accounts[0],
                        owner: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                ))
            },

            ConfidentialTransferInstruction::EnableConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::EnableConfidentialCredits(
                    CreditsAccounts {
                        account: ix.accounts[0],
                        owner: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                ))
            },

            ConfidentialTransferInstruction::DisableConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::DisableConfidentialCredits(
                    CreditsAccounts {
                        account: ix.accounts[0],
                        owner: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                ))
            },

            ConfidentialTransferInstruction::EnableNonConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::EnableNonConfidentialCredits(
                    CreditsAccounts {
                        account: ix.accounts[0],
                        owner: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                ))
            },

            ConfidentialTransferInstruction::DisableNonConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::DisableNonConfidentialCredits(
                    CreditsAccounts {
                        account: ix.accounts[0],
                        owner: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                ))
            },
            ConfidentialTransferInstruction::TransferWithFee => {
                check_min_accounts_req(accounts_len, 5)?;
                Ok(ConfidentaltransferIx::TransferWithFee(
                    TransferWithFeeAccounts {
                        source_account: ix.accounts[0],
                        mint: ix.accounts[1],
                        destination: ix.accounts[2],
                    },
                ))
            },
            ConfidentialTransferInstruction::ConfigureAccountWithRegistry => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(ConfidentaltransferIx::ConfigureAccountWithRegistry(
                    ConfigureAccountWithRegistryAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        registry: ix.accounts[2],
                    },
                ))
            },
        }
    }
}

#[cfg(feature = "proto")]
mod proto_parser {

    use confidential_transfer_ext_ix_proto::IxOneof;
    use yellowstone_vixen_proto::parser::token_extensions::{
        confidential_transfer_ext_ix_proto, ApplyPendingBalanceAccountsProto,
        ApplyPendingBalanceIxProto, ApproveAccountAccountsProto, ApproveAccountIxProto,
        ConfidentialTransferAccountsProto, ConfidentialTransferExtIxProto,
        ConfidentialTransferIxProto, ConfigureAccountAccountsProto, ConfigureAccountIxProto,
        ConfigureAccountWithRegistryAccountsProto, ConfigureAccountWithRegistryIxProto,
        CreditsAccountsProto, DepositAccountsProto, DepositIxProto,
        DisableConfidentialCreditsIxProto, DisableNonConfidentialCreditsIxProto,
        EmptyAccountAccountsProto, EmptyAccountIxProto, EnableConfidentialCreditsIxProto,
        EnableNonConfidentialCreditsIxProto, InitializeConfidentialMintAccountsProto,
        InitializeConfidentialMintIxProto, TransferWithFeeAccountsProto, TransferWithFeeIxProto,
        UpdateMintAccountsProto, UpdateMintIxProto, WithdrawAccountsProto, WithdrawIxProto,
    };

    use super::{
        ApplyPendingBalanceAccounts, ApproveAccountAccounts, ConfidentaltransferIx,
        ConfidentialTransferAccounts, ConfigureAccountAccounts, CreditsAccounts, DepositAccounts,
        EmptyAccountAccounts, InitializeMintAccounts, UpdateMintAccounts, WithdrawAccounts,
    };
    use crate::{
        helpers::{proto::FromVecPubkeyToVecString, IntoProto},
        token_extension_program::{ConfigureAccountWithRegistryAccounts, TransferWithFeeAccounts},
    };

    impl IntoProto<InitializeConfidentialMintAccountsProto> for InitializeMintAccounts {
        fn into_proto(self) -> InitializeConfidentialMintAccountsProto {
            InitializeConfidentialMintAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProto<UpdateMintAccountsProto> for UpdateMintAccounts {
        fn into_proto(self) -> UpdateMintAccountsProto {
            UpdateMintAccountsProto {
                mint: self.mint.to_string(),
                authority: self.authority.to_string(),
            }
        }
    }

    impl IntoProto<ConfigureAccountAccountsProto> for ConfigureAccountAccounts {
        fn into_proto(self) -> ConfigureAccountAccountsProto {
            ConfigureAccountAccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
                sysvar: self.sysvar.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<ApproveAccountAccountsProto> for ApproveAccountAccounts {
        fn into_proto(self) -> ApproveAccountAccountsProto {
            ApproveAccountAccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
                authority: self.authority.to_string(),
            }
        }
    }

    impl IntoProto<EmptyAccountAccountsProto> for EmptyAccountAccounts {
        fn into_proto(self) -> EmptyAccountAccountsProto {
            EmptyAccountAccountsProto {
                account: self.account.to_string(),
                sysvar: self.sysvar.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<DepositAccountsProto> for DepositAccounts {
        fn into_proto(self) -> DepositAccountsProto {
            DepositAccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<WithdrawAccountsProto> for WithdrawAccounts {
        fn into_proto(self) -> WithdrawAccountsProto {
            WithdrawAccountsProto {
                source_account: self.source_account.to_string(),
                mint: self.mint.to_string(),
                destination: self.destination.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<ConfidentialTransferAccountsProto> for ConfidentialTransferAccounts {
        fn into_proto(self) -> ConfidentialTransferAccountsProto {
            ConfidentialTransferAccountsProto {
                source_account: self.source_account.to_string(),
                mint: self.mint.to_string(),
                destination: self.destination.to_string(),
                owner: self.owner.to_string(),
                context_account: self.context_account.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<ApplyPendingBalanceAccountsProto> for ApplyPendingBalanceAccounts {
        fn into_proto(self) -> ApplyPendingBalanceAccountsProto {
            ApplyPendingBalanceAccountsProto {
                account: self.account.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<CreditsAccountsProto> for CreditsAccounts {
        fn into_proto(self) -> CreditsAccountsProto {
            CreditsAccountsProto {
                account: self.account.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<TransferWithFeeAccountsProto> for TransferWithFeeAccounts {
        fn into_proto(self) -> TransferWithFeeAccountsProto {
            TransferWithFeeAccountsProto {
                source_account: self.source_account.to_string(),
                mint: self.mint.to_string(),
                destination: self.destination.to_string(),
            }
        }
    }

    impl IntoProto<ConfigureAccountWithRegistryAccountsProto> for ConfigureAccountWithRegistryAccounts {
        fn into_proto(self) -> ConfigureAccountWithRegistryAccountsProto {
            ConfigureAccountWithRegistryAccountsProto {
                account: self.account.to_string(),
                mint: self.mint.to_string(),
                registry: self.registry.to_string(),
            }
        }
    }
    impl IntoProto<ConfidentialTransferExtIxProto> for ConfidentaltransferIx {
        fn into_proto(self) -> ConfidentialTransferExtIxProto {
            match self {
                ConfidentaltransferIx::InitializeMint(acc) => ConfidentialTransferExtIxProto {
                    ix_oneof: Some(IxOneof::InitializeMintIx(
                        InitializeConfidentialMintIxProto {
                            accounts: Some(acc.into_proto()),
                        },
                    )),
                },

                ConfidentaltransferIx::UpdateMint(acc) => ConfidentialTransferExtIxProto {
                    ix_oneof: Some(IxOneof::UpdateMintIx(UpdateMintIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                ConfidentaltransferIx::ConfigureAccount(acc) => ConfidentialTransferExtIxProto {
                    ix_oneof: Some(IxOneof::ConfigureAccountIx(ConfigureAccountIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                ConfidentaltransferIx::ApproveAccount(acc) => ConfidentialTransferExtIxProto {
                    ix_oneof: Some(IxOneof::ApproveAccountIx(ApproveAccountIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                ConfidentaltransferIx::EmptyAccount(acc) => ConfidentialTransferExtIxProto {
                    ix_oneof: Some(IxOneof::EmptyAccountIx(EmptyAccountIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                ConfidentaltransferIx::Deposit(acc) => ConfidentialTransferExtIxProto {
                    ix_oneof: Some(IxOneof::DepositIx(DepositIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                ConfidentaltransferIx::Withdraw(acc) => ConfidentialTransferExtIxProto {
                    ix_oneof: Some(IxOneof::WithdrawIx(WithdrawIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                ConfidentaltransferIx::Transfer(acc) => ConfidentialTransferExtIxProto {
                    ix_oneof: Some(IxOneof::TransferIx(ConfidentialTransferIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                ConfidentaltransferIx::ApplyPendingBalance(acc) => ConfidentialTransferExtIxProto {
                    ix_oneof: Some(IxOneof::ApplyPendingBalanceIx(ApplyPendingBalanceIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                ConfidentaltransferIx::EnableConfidentialCredits(acc) => {
                    ConfidentialTransferExtIxProto {
                        ix_oneof: Some(IxOneof::EnableConfidentialCreditsIx(
                            EnableConfidentialCreditsIxProto {
                                accounts: Some(acc.into_proto()),
                            },
                        )),
                    }
                },

                ConfidentaltransferIx::DisableConfidentialCredits(acc) => {
                    ConfidentialTransferExtIxProto {
                        ix_oneof: Some(IxOneof::DisableConfidentialCreditsIx(
                            DisableConfidentialCreditsIxProto {
                                accounts: Some(acc.into_proto()),
                            },
                        )),
                    }
                },

                ConfidentaltransferIx::EnableNonConfidentialCredits(acc) => {
                    ConfidentialTransferExtIxProto {
                        ix_oneof: Some(IxOneof::EnableNonConfidentialCreditsIx(
                            EnableNonConfidentialCreditsIxProto {
                                accounts: Some(acc.into_proto()),
                            },
                        )),
                    }
                },

                ConfidentaltransferIx::DisableNonConfidentialCredits(acc) => {
                    ConfidentialTransferExtIxProto {
                        ix_oneof: Some(IxOneof::DisableNonConfidentialCreditsIx(
                            DisableNonConfidentialCreditsIxProto {
                                accounts: Some(acc.into_proto()),
                            },
                        )),
                    }
                },

                ConfidentaltransferIx::TransferWithFee(acc) => ConfidentialTransferExtIxProto {
                    ix_oneof: Some(IxOneof::TransferWithFeeIx(TransferWithFeeIxProto {
                        accounts: Some(acc.into_proto()),
                    })),
                },

                ConfidentaltransferIx::ConfigureAccountWithRegistry(acc) => {
                    ConfidentialTransferExtIxProto {
                        ix_oneof: Some(IxOneof::ConfigureAccountWithRegistryIx(
                            ConfigureAccountWithRegistryIxProto {
                                accounts: Some(acc.into_proto()),
                            },
                        )),
                    }
                },
            }
        }
    }
}
