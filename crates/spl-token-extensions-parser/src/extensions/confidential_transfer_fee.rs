use spl_token_2022::extension::confidential_transfer_fee::instruction::ConfidentialTransferFeeInstruction as SplConfidentialTransferFeeInstruction;
use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};
use yellowstone_vixen_parser::{check_min_accounts_req, Result};

use super::extension::{decode_extension_ix_type, ExtensionInstructionParser};

#[derive(Debug, Clone, Copy)]
pub struct InitializeConfidentialTransferFeeConfigAccounts {
    pub mint: Pubkey,
}

#[derive(Debug, Clone)]
pub struct ConfidentialWithdrawWithheldTokensFromMintAccounts {
    pub mint: Pubkey,
    pub fee_recipient: Pubkey,
    pub sysvar: Pubkey,
    pub withdraw_withheld_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct ConfidentialWithdrawWithheldTokensFromAccounts {
    pub mint: Pubkey,
    pub fee_recipient: Pubkey,
    pub sysvar: Pubkey,
    pub withdraw_withheld_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
    pub source_accounts: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct ConfidentialHarvestWithheldTokensToMintAccounts {
    pub mint: Pubkey,
    pub source_accounts: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct EnableHarvestToMintAccounts {
    pub mint: Pubkey,
    pub confidential_transfer_fee_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct DisableHarvestToMintAccounts {
    pub account: Pubkey,
    pub confidential_transfer_fee_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub enum ConfidentialTransferFeeInstruction {
    InitializeConfidentialTransferFeeConfig {
        accounts: InitializeConfidentialTransferFeeConfigAccounts,
    },
    WithdrawWithheldTokensFromMint {
        accounts: ConfidentialWithdrawWithheldTokensFromMintAccounts,
    },
    WithdrawWithheldTokensFromAccounts {
        accounts: ConfidentialWithdrawWithheldTokensFromAccounts,
    },
    HarvestWithheldTokensToMint {
        accounts: ConfidentialHarvestWithheldTokensToMintAccounts,
    },
    EnableHarvestToMint {
        accounts: EnableHarvestToMintAccounts,
    },
    DisableHarvestToMint {
        accounts: DisableHarvestToMintAccounts,
    },
}

impl ExtensionInstructionParser for ConfidentialTransferFeeInstruction {
    fn try_parse(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();
        let ix_type = decode_extension_ix_type(&ix.data[1..])?;

        match ix_type {
            SplConfidentialTransferFeeInstruction::InitializeConfidentialTransferFeeConfig => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(
                    ConfidentialTransferFeeInstruction::InitializeConfidentialTransferFeeConfig {
                        accounts: InitializeConfidentialTransferFeeConfigAccounts {
                            mint: ix.accounts[0],
                        },
                    },
                )
            },

            SplConfidentialTransferFeeInstruction::WithdrawWithheldTokensFromMint => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(
                    ConfidentialTransferFeeInstruction::WithdrawWithheldTokensFromMint {
                        accounts: ConfidentialWithdrawWithheldTokensFromMintAccounts {
                            mint: ix.accounts[0],
                            fee_recipient: ix.accounts[1],
                            sysvar: ix.accounts[2],
                            withdraw_withheld_authority: ix.accounts[3],
                            multisig_signers: ix.accounts[4..].to_vec(),
                        },
                    },
                )
            },

            SplConfidentialTransferFeeInstruction::WithdrawWithheldTokensFromAccounts => {
                check_min_accounts_req(accounts_len, 5)?;
                Ok(
                    ConfidentialTransferFeeInstruction::WithdrawWithheldTokensFromAccounts {
                        accounts: ConfidentialWithdrawWithheldTokensFromAccounts {
                            mint: ix.accounts[0],
                            fee_recipient: ix.accounts[1],
                            sysvar: ix.accounts[2],
                            withdraw_withheld_authority: ix.accounts[3],
                            source_accounts: ix.accounts[4..].to_vec(),
                            multisig_signers: Vec::new(),
                        },
                    },
                )
            },

            SplConfidentialTransferFeeInstruction::HarvestWithheldTokensToMint => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(
                    ConfidentialTransferFeeInstruction::HarvestWithheldTokensToMint {
                        accounts: ConfidentialHarvestWithheldTokensToMintAccounts {
                            mint: ix.accounts[0],
                            source_accounts: ix.accounts[1..].to_vec(),
                        },
                    },
                )
            },

            SplConfidentialTransferFeeInstruction::EnableHarvestToMint => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentialTransferFeeInstruction::EnableHarvestToMint {
                    accounts: EnableHarvestToMintAccounts {
                        mint: ix.accounts[0],
                        confidential_transfer_fee_authority: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                })
            },

            SplConfidentialTransferFeeInstruction::DisableHarvestToMint => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentialTransferFeeInstruction::DisableHarvestToMint {
                    accounts: DisableHarvestToMintAccounts {
                        account: ix.accounts[0],
                        confidential_transfer_fee_authority: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                })
            },
        }
    }
}
