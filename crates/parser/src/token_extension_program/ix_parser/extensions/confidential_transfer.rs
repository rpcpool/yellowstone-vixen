use spl_token_2022::extension::confidential_transfer::instruction::ConfidentialTransferInstruction;
use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};

use super::helpers::{decode_extension_ix_type, ExtensionIxParser, Ix};
use crate::{
    helpers::{check_min_accounts_req, get_multisig_signers},
    token_program::ix_parser::InitializeMintAccounts,
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
    pub multisig_signers: Option<Vec<Pubkey>>,
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
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug)]

pub struct DepositAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug)]
pub struct WithdrawAccounts {
    pub source_account: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug)]

pub struct ConfidentialTransferAccounts {
    pub source_account: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub context_account: Pubkey, // Sysvar account or context state account
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug)]
pub struct ApplyPendingBalanceAccounts {
    pub account: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug)]

pub struct CreditsAccounts {
    pub account: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
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

#[derive(Debug)]
pub enum ConfidentaltransferIx {
    InitializeMint(Ix<InitializeMintAccounts>),
    UpdateMint(Ix<UpdateMintAccounts>),
    ConfigureAccount(Ix<ConfigureAccountAccounts>),
    ApproveAccount(Ix<ApproveAccountAccounts>),
    EmptyAccount(Ix<EmptyAccountAccounts>),
    Deposit(Ix<DepositAccounts>),
    Withdraw(Ix<WithdrawAccounts>),
    Transfer(Ix<ConfidentialTransferAccounts>),
    ApplyPendingBalance(Ix<ApplyPendingBalanceAccounts>),
    EnableConfidentialCredits(Ix<CreditsAccounts>),
    DisableConfidentialCredits(Ix<CreditsAccounts>),
    EnableNonConfidentialCredits(Ix<CreditsAccounts>),
    DisableNonConfidentialCredits(Ix<CreditsAccounts>),
    TransferWithSplitProofs(Ix<TransferWithSplitProofsAccounts>),
}

impl ExtensionIxParser for ConfidentaltransferIx {
    fn try_parse_extension_ix(ix: &InstructionUpdate) -> Result<Self, String> {
        let accounts_len = ix.accounts.len();
        let ix_type = decode_extension_ix_type(&ix.data)?;
        match ix_type {
            ConfidentialTransferInstruction::InitializeMint => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(ConfidentaltransferIx::InitializeMint(Ix::from_accounts(
                    InitializeMintAccounts {
                        mint: ix.accounts[0],
                    },
                )))
            },
            ConfidentialTransferInstruction::UpdateMint => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::UpdateMint(Ix::from_accounts(
                    UpdateMintAccounts {
                        mint: ix.accounts[0],
                        authority: ix.accounts[1],
                    },
                )))
            },
            ConfidentialTransferInstruction::ConfigureAccount => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(ConfidentaltransferIx::ConfigureAccount(Ix::from_accounts(
                    ConfigureAccountAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        sysvar: ix.accounts[2],
                        owner: ix.accounts[3],
                        multisig_signers: get_multisig_signers(ix, 4),
                    },
                )))
            },

            ConfidentialTransferInstruction::ApproveAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(ConfidentaltransferIx::ApproveAccount(Ix::from_accounts(
                    ApproveAccountAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        authority: ix.accounts[2],
                    },
                )))
            },

            ConfidentialTransferInstruction::EmptyAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(ConfidentaltransferIx::EmptyAccount(Ix::from_accounts(
                    EmptyAccountAccounts {
                        account: ix.accounts[0],
                        sysvar: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: get_multisig_signers(ix, 3),
                    },
                )))
            },

            ConfidentialTransferInstruction::Deposit => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(ConfidentaltransferIx::Deposit(Ix::from_accounts(
                    DepositAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: get_multisig_signers(ix, 3),
                    },
                )))
            },

            ConfidentialTransferInstruction::Withdraw => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(ConfidentaltransferIx::Withdraw(Ix::from_accounts(
                    WithdrawAccounts {
                        source_account: ix.accounts[0],
                        mint: ix.accounts[1],
                        destination: ix.accounts[2],
                        owner: ix.accounts[3],
                        multisig_signers: get_multisig_signers(ix, 4),
                    },
                )))
            },

            ConfidentialTransferInstruction::Transfer => {
                check_min_accounts_req(accounts_len, 5)?;
                Ok(ConfidentaltransferIx::Transfer(Ix::from_accounts(
                    ConfidentialTransferAccounts {
                        source_account: ix.accounts[0],
                        mint: ix.accounts[1],
                        destination: ix.accounts[2],
                        context_account: ix.accounts[3],
                        owner: ix.accounts[4],
                        multisig_signers: get_multisig_signers(ix, 5),
                    },
                )))
            },

            ConfidentialTransferInstruction::ApplyPendingBalance => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::ApplyPendingBalance(
                    Ix::from_accounts(ApplyPendingBalanceAccounts {
                        account: ix.accounts[0],
                        owner: ix.accounts[1],
                        multisig_signers: get_multisig_signers(ix, 2),
                    }),
                ))
            },

            ConfidentialTransferInstruction::EnableConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::EnableConfidentialCredits(
                    Ix::from_accounts(CreditsAccounts {
                        account: ix.accounts[0],
                        owner: ix.accounts[1],
                        multisig_signers: get_multisig_signers(ix, 2),
                    }),
                ))
            },

            ConfidentialTransferInstruction::DisableConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::DisableConfidentialCredits(
                    Ix::from_accounts(CreditsAccounts {
                        account: ix.accounts[0],
                        owner: ix.accounts[1],
                        multisig_signers: get_multisig_signers(ix, 2),
                    }),
                ))
            },

            ConfidentialTransferInstruction::EnableNonConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::EnableNonConfidentialCredits(
                    Ix::from_accounts(CreditsAccounts {
                        account: ix.accounts[0],
                        owner: ix.accounts[1],
                        multisig_signers: get_multisig_signers(ix, 2),
                    }),
                ))
            },

            ConfidentialTransferInstruction::DisableNonConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::DisableNonConfidentialCredits(
                    Ix::from_accounts(CreditsAccounts {
                        account: ix.accounts[0],
                        owner: ix.accounts[1],
                        multisig_signers: get_multisig_signers(ix, 2),
                    }),
                ))
            },

            ConfidentialTransferInstruction::TransferWithSplitProofs => {
                check_min_accounts_req(accounts_len, 13)?;

                match accounts_len {
                    7 => Ok(ConfidentaltransferIx::TransferWithSplitProofs(
                        Ix::from_accounts(TransferWithSplitProofsAccounts {
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
                        }),
                    )),
                    9 => {
                        let ninth_account = ix.accounts[8];
                        if &ninth_account.to_string() == SOLANA_ZK_PROOF_PROGRAM_ID {
                            Ok(ConfidentaltransferIx::TransferWithSplitProofs(
                                Ix::from_accounts(TransferWithSplitProofsAccounts {
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
                                }),
                            ))
                        } else {
                            Ok(ConfidentaltransferIx::TransferWithSplitProofs(
                                Ix::from_accounts(TransferWithSplitProofsAccounts {
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
                                }),
                            ))
                        }
                    },

                    11 => Ok(ConfidentaltransferIx::TransferWithSplitProofs(
                        Ix::from_accounts(TransferWithSplitProofsAccounts {
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
                        }),
                    )),

                    _ => Err(format!(
                        "Invalid number of accounts for TransferWithSplitProofs: {}",
                        accounts_len
                    )),
                }
            },
        }
    }
}
