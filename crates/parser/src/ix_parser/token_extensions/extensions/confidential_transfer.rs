use spl_pod::solana_program::pubkey::Pubkey;
use spl_token_2022::extension::confidential_transfer::instruction::ConfidentialTransferInstruction;

use super::helpers::{decode_extension_ix_type, ExtensionIxParser, Ix};
use crate::ix_parser::{
    token_program::token_ix::InitializeMintAccounts,
    vixen_ix::{
        helpers::{check_min_accounts_req, get_multisig_signers},
        structure::InstructionUpdate,
    },
};

#[derive(Debug)]
pub struct UpdateMintAccounts {
    pub mint: Pubkey,
    pub authority: Pubkey,
}

#[derive(Debug)]
pub struct ConfigureAccountAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub sysvar: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug)]
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
    pub verify_batched_grouped_cipher_text_2_handles_validity_proof: Option<Pubkey>,
    pub verify_batched_range_proof_u128: Option<Pubkey>,
    pub close_split_context_state_on_execution: bool,
    pub verify_ciphertext_commitment_equality_proof: Option<Pubkey>,
    pub verify_batched_grouped_cipher_text_2_handles_validity_proof_1: Option<Pubkey>,
    pub verify_fee_sigma_proof: Option<Pubkey>,
    pub destination_account_for_lamports: Option<Pubkey>,
    pub context_state_account_owner: Option<Pubkey>,
    pub zk_token_proof_program: Option<Pubkey>,
    pub owner: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
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
    fn try_parse_extension_ix(ix_update: &InstructionUpdate) -> Result<Self, String> {
        let accounts_len = ix_update.accounts.len();
        let ix_type = decode_extension_ix_type(&ix_update.data)?;
        match ix_type {
            ConfidentialTransferInstruction::InitializeMint => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(ConfidentaltransferIx::InitializeMint(Ix::from_accounts(
                    InitializeMintAccounts {
                        mint: ix_update.accounts[0],
                    },
                )))
            },
            ConfidentialTransferInstruction::UpdateMint => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::UpdateMint(Ix::from_accounts(
                    UpdateMintAccounts {
                        mint: ix_update.accounts[0],
                        authority: ix_update.accounts[1],
                    },
                )))
            },
            ConfidentialTransferInstruction::ConfigureAccount => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(ConfidentaltransferIx::ConfigureAccount(Ix::from_accounts(
                    ConfigureAccountAccounts {
                        account: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                        sysvar: ix_update.accounts[2],
                        owner: ix_update.accounts[3],
                        multisig_signers: get_multisig_signers(ix_update, 4),
                    },
                )))
            },

            ConfidentialTransferInstruction::ApproveAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(ConfidentaltransferIx::ApproveAccount(Ix::from_accounts(
                    ApproveAccountAccounts {
                        account: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                        authority: ix_update.accounts[2],
                    },
                )))
            },

            ConfidentialTransferInstruction::EmptyAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(ConfidentaltransferIx::EmptyAccount(Ix::from_accounts(
                    EmptyAccountAccounts {
                        account: ix_update.accounts[0],
                        sysvar: ix_update.accounts[1],
                        owner: ix_update.accounts[2],
                        multisig_signers: get_multisig_signers(ix_update, 3),
                    },
                )))
            },

            ConfidentialTransferInstruction::Deposit => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(ConfidentaltransferIx::Deposit(Ix::from_accounts(
                    DepositAccounts {
                        account: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                        owner: ix_update.accounts[2],
                        multisig_signers: get_multisig_signers(ix_update, 3),
                    },
                )))
            },

            ConfidentialTransferInstruction::Withdraw => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(ConfidentaltransferIx::Withdraw(Ix::from_accounts(
                    WithdrawAccounts {
                        source_account: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                        destination: ix_update.accounts[2],
                        owner: ix_update.accounts[3],
                        multisig_signers: get_multisig_signers(ix_update, 4),
                    },
                )))
            },

            ConfidentialTransferInstruction::Transfer => {
                check_min_accounts_req(accounts_len, 5)?;
                Ok(ConfidentaltransferIx::Transfer(Ix::from_accounts(
                    ConfidentialTransferAccounts {
                        source_account: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                        destination: ix_update.accounts[2],
                        context_account: ix_update.accounts[3],
                        owner: ix_update.accounts[4],
                        multisig_signers: get_multisig_signers(ix_update, 5),
                    },
                )))
            },

            ConfidentialTransferInstruction::ApplyPendingBalance => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::ApplyPendingBalance(
                    Ix::from_accounts(ApplyPendingBalanceAccounts {
                        account: ix_update.accounts[0],
                        owner: ix_update.accounts[1],
                        multisig_signers: get_multisig_signers(ix_update, 2),
                    }),
                ))
            },

            ConfidentialTransferInstruction::EnableConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::EnableConfidentialCredits(
                    Ix::from_accounts(CreditsAccounts {
                        account: ix_update.accounts[0],
                        owner: ix_update.accounts[1],
                        multisig_signers: get_multisig_signers(ix_update, 2),
                    }),
                ))
            },

            ConfidentialTransferInstruction::DisableConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::DisableConfidentialCredits(
                    Ix::from_accounts(CreditsAccounts {
                        account: ix_update.accounts[0],
                        owner: ix_update.accounts[1],
                        multisig_signers: get_multisig_signers(ix_update, 2),
                    }),
                ))
            },

            ConfidentialTransferInstruction::EnableNonConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::EnableNonConfidentialCredits(
                    Ix::from_accounts(CreditsAccounts {
                        account: ix_update.accounts[0],
                        owner: ix_update.accounts[1],
                        multisig_signers: get_multisig_signers(ix_update, 2),
                    }),
                ))
            },

            ConfidentialTransferInstruction::DisableNonConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferIx::DisableNonConfidentialCredits(
                    Ix::from_accounts(CreditsAccounts {
                        account: ix_update.accounts[0],
                        owner: ix_update.accounts[1],
                        multisig_signers: get_multisig_signers(ix_update, 2),
                    }),
                ))
            },

            ConfidentialTransferInstruction::TransferWithSplitProofs => {
                check_min_accounts_req(accounts_len, 13)?;
                Ok(ConfidentaltransferIx::TransferWithSplitProofs(
                    Ix::from_accounts(TransferWithSplitProofsAccounts {
                        source_account: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                        destination: ix_update.accounts[2],
                        verify_batched_grouped_cipher_text_2_handles_validity_proof: None,
                        verify_batched_range_proof_u128: None,
                        close_split_context_state_on_execution: false,
                        verify_ciphertext_commitment_equality_proof: None,
                        verify_batched_grouped_cipher_text_2_handles_validity_proof_1: None,
                        verify_fee_sigma_proof: None,
                        destination_account_for_lamports: None,
                        context_state_account_owner: None,
                        zk_token_proof_program: None,
                        owner: ix_update.accounts[12],
                        multisig_signers: None,
                    }),
                ))
            },
        }
    }
}
