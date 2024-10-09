use spl_token_2022::extension::confidential_transfer::instruction::ConfidentialTransferInstruction;
use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};

use super::helpers::{decode_extension_ix_type, ExtensionIxParser};
use crate::{
    helpers::check_min_accounts_req, token_program::ix_parser::InitializeMintAccounts, Error,
    Result,
};

const SOLANA_ZK_PROOF_PROGRAM_ID: &str = "ZkTokenProof1111111111111111111111111111111";

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

#[derive(Debug)]
pub struct TransferWithSplitProofsAccounts {
    pub source_account: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub verify_ciphertext_commitment_equality_proof: Pubkey,
    pub verify_batched_grouped_cipher_text_2_handles_validity_proof: Pubkey,
    pub verify_batched_range_proof_u128: Option<Pubkey>,
    pub verify_batched_range_proof_u256: Option<Pubkey>,
    pub verify_batched_grouped_cipher_text_2_handles_validity_proof_next: Option<Pubkey>,
    pub verify_fee_sigma_proof: Option<Pubkey>,
    pub destination_account_for_lamports: Option<Pubkey>,
    pub context_state_account_owner: Option<Pubkey>,
    pub zk_token_proof_program: Option<Pubkey>,
    pub owner: Option<Pubkey>,
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
    TransferWithSplitProofs(TransferWithSplitProofsAccounts),
}

impl ExtensionIxParser for ConfidentaltransferIx {
    #[allow(clippy::too_many_lines)]
    fn try_parse_extension_ix(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();
        let ix_type = decode_extension_ix_type(&ix.data)?;
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

            ConfidentialTransferInstruction::TransferWithSplitProofs => {
                check_min_accounts_req(accounts_len, 13)?;

                match accounts_len {
                    7 => Ok(ConfidentaltransferIx::TransferWithSplitProofs(
                        TransferWithSplitProofsAccounts {
                            source_account: ix.accounts[0],
                            mint: ix.accounts[1],
                            destination: ix.accounts[2],
                            verify_ciphertext_commitment_equality_proof: ix.accounts[3],
                            verify_batched_grouped_cipher_text_2_handles_validity_proof: ix
                                .accounts[4],
                            verify_batched_range_proof_u128: Some(ix.accounts[5]),
                            owner: Some(ix.accounts[6]),
                            // Optional accounts
                            verify_batched_range_proof_u256: None,
                            verify_batched_grouped_cipher_text_2_handles_validity_proof_next: None,
                            verify_fee_sigma_proof: None,
                            destination_account_for_lamports: None,
                            context_state_account_owner: None,
                            zk_token_proof_program: None,
                        },
                    )),
                    9 => {
                        let ninth_account = ix.accounts[8];
                        if ninth_account.to_string() == SOLANA_ZK_PROOF_PROGRAM_ID {
                            Ok(ConfidentaltransferIx::TransferWithSplitProofs(
                                TransferWithSplitProofsAccounts {
                                    source_account: ix.accounts[0],
                                    mint: ix.accounts[1],
                                    destination: ix.accounts[2],
                                    verify_ciphertext_commitment_equality_proof: ix.accounts[3],
                                    verify_batched_grouped_cipher_text_2_handles_validity_proof: ix
                                    .accounts[4],
                                    verify_batched_range_proof_u128: Some(ix.accounts[5]),
                                    destination_account_for_lamports: Some(ix.accounts[6]),
                                    context_state_account_owner: Some(ix.accounts[7]),
                                    zk_token_proof_program: Some(ix.accounts[8]),

                                    // Optional accounts
                                    owner: None,
                                    verify_fee_sigma_proof: None,
                                    verify_batched_range_proof_u256: None,
                                    verify_batched_grouped_cipher_text_2_handles_validity_proof_next:
                                    None,
                                },
                            ))
                        } else {
                            Ok(ConfidentaltransferIx::TransferWithSplitProofs(
                                TransferWithSplitProofsAccounts {
                                    source_account: ix.accounts[0],
                                    mint: ix.accounts[1],
                                    destination: ix.accounts[2],
                                    verify_ciphertext_commitment_equality_proof: ix.accounts[3],
                                    verify_batched_grouped_cipher_text_2_handles_validity_proof: ix
                                    .accounts[4],
                                    verify_fee_sigma_proof: Some(ix.accounts[5]),
                                    verify_batched_range_proof_u256: Some(ix.accounts[6]),
                                    verify_batched_grouped_cipher_text_2_handles_validity_proof_next:
                                    Some(ix.accounts[7]),
                                    owner: Some(ix.accounts[8]),

                                    // Optional accounts
                                    verify_batched_range_proof_u128: None,
                                    destination_account_for_lamports: None,
                                    context_state_account_owner: None,
                                    zk_token_proof_program: None,
                                },
                            ))
                        }
                    },

                    11 => Ok(ConfidentaltransferIx::TransferWithSplitProofs(
                        TransferWithSplitProofsAccounts {
                            source_account: ix.accounts[0],
                            mint: ix.accounts[1],
                            destination: ix.accounts[2],
                            verify_ciphertext_commitment_equality_proof: ix.accounts[3],
                            verify_batched_grouped_cipher_text_2_handles_validity_proof: ix
                                .accounts[4],
                            verify_batched_range_proof_u256: Some(ix.accounts[5]),
                            verify_batched_grouped_cipher_text_2_handles_validity_proof_next: Some(
                                ix.accounts[6],
                            ),
                            verify_fee_sigma_proof: Some(ix.accounts[7]),
                            destination_account_for_lamports: Some(ix.accounts[8]),
                            context_state_account_owner: Some(ix.accounts[9]),
                            zk_token_proof_program: Some(ix.accounts[10]),
                            verify_batched_range_proof_u128: None,
                            owner: None,
                        },
                    )),

                    _ => Err(Error::new(format!(
                        "Invalid number of accounts for TransferWithSplitProofs: {accounts_len}"
                    ))),
                }
            },
        }
    }
}

#[cfg(feature = "proto")]
mod proto_parser {

    use confidential_transfer_ext_ix_proto::IxOneof;
    use yellowstone_vixen_proto::parser::{
        confidential_transfer_ext_ix_proto, ApplyPendingBalanceAccountsProto,
        ApplyPendingBalanceIxProto, ApproveAccountAccountsProto, ApproveAccountIxProto,
        ConfidentialTransferAccountsProto, ConfidentialTransferExtIxProto,
        ConfidentialTransferIxProto, ConfigureAccountAccountsProto, ConfigureAccountIxProto,
        CreditsAccountsProto, DepositAccountsProto, DepositIxProto,
        DisableConfidentialCreditsIxProto, DisableNonConfidentialCreditsIxProto,
        EmptyAccountAccountsProto, EmptyAccountIxProto, EnableConfidentialCreditsIxProto,
        EnableNonConfidentialCreditsIxProto, InitializeConfidentialMintAccountsProto,
        InitializeConfidentialMintIxProto, TransferWithSplitProofsAccountsProto,
        TransferWithSplitProofsIxProto, UpdateMintAccountsProto, UpdateMintIxProto,
        WithdrawAccountsProto, WithdrawIxProto,
    };

    use super::{
        ApplyPendingBalanceAccounts, ApproveAccountAccounts, ConfidentaltransferIx,
        ConfidentialTransferAccounts, ConfigureAccountAccounts, CreditsAccounts, DepositAccounts,
        EmptyAccountAccounts, InitializeMintAccounts, TransferWithSplitProofsAccounts,
        UpdateMintAccounts, WithdrawAccounts,
    };
    use crate::helpers::{FromOptPubkeyToOptString, FromVecPubkeyToVecString, IntoProto};

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

    impl IntoProto<TransferWithSplitProofsAccountsProto> for TransferWithSplitProofsAccounts {
        fn into_proto(self) -> TransferWithSplitProofsAccountsProto {
            TransferWithSplitProofsAccountsProto {
                source_account: self.source_account.to_string(),
                mint: self.mint.to_string(),
                destination: self.destination.to_string(),
                verify_ciphertext_commitment_equality_proof: self
                    .verify_ciphertext_commitment_equality_proof
                    .to_string(),
                verify_batched_grouped_cipher_text_2_handles_validity_proof: self
                    .verify_batched_grouped_cipher_text_2_handles_validity_proof
                    .to_string(),
                verify_batched_range_proof_u128: self
                    .verify_batched_range_proof_u128
                    .to_opt_string(),
                verify_batched_range_proof_u256: self
                    .verify_batched_range_proof_u256
                    .to_opt_string(),
                verify_batched_grouped_cipher_text_2_handles_validity_proof_next: self
                    .verify_batched_grouped_cipher_text_2_handles_validity_proof_next
                    .to_opt_string(),
                verify_fee_sigma_proof: self.verify_fee_sigma_proof.to_opt_string(),
                destination_account_for_lamports: self
                    .destination_account_for_lamports
                    .to_opt_string(),
                context_state_account_owner: self.context_state_account_owner.to_opt_string(),
                zk_token_proof_program: self.zk_token_proof_program.to_opt_string(),
                owner: self.owner.to_opt_string(),
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

                ConfidentaltransferIx::TransferWithSplitProofs(acc) => {
                    ConfidentialTransferExtIxProto {
                        ix_oneof: Some(IxOneof::TransferWithSplitProofsIx(
                            TransferWithSplitProofsIxProto {
                                accounts: Some(acc.into_proto()),
                            },
                        )),
                    }
                },
            }
        }
    }
}
