use spl_token_2022::extension::confidential_transfer_fee::instruction::ConfidentialTransferFeeInstruction;
use yellowstone_vixen_core::{Instruction, Pubkey};

use super::helpers::{decode_extension_ix_type, ExtensionIxParser, Ix};
use crate::ix_parser::helpers::{check_min_accounts_req, get_multisig_signers};

#[derive(Debug)]
pub struct InitializeConfidentialTransferFeeConfigAccounts {
    pub mint: Pubkey,
}

#[derive(Debug)]
pub struct WithdrawWithheldTokensFromMint {
    pub mint: Pubkey,
    pub fee_recipient: Pubkey,
    pub sysvar: Pubkey,
    pub withdraw_withheld_authority: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug)]
pub struct WithdrawWithheldTokensFromAccounts {
    pub mint: Pubkey,
    pub fee_recipient: Pubkey,
    pub sysvar: Pubkey,
    pub withdraw_withheld_authority: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
    pub source_accounts: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct HarvestWithheldTokensToMint {
    pub mint: Pubkey,
    pub source_accounts: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct EnableHarvestToMint {
    pub mint: Pubkey,
    pub confidential_transfer_fee_authority: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug)]
pub struct DisableHarvestToMint {
    pub account: Pubkey,
    pub confidential_transfer_fee_authority: Pubkey,
    pub multisig_signers: Option<Vec<Pubkey>>,
}

#[derive(Debug)]
pub enum ConfidentaltransferFeeIx {
    InitializeConfidentialTransferFeeConfig(Ix<InitializeConfidentialTransferFeeConfigAccounts>),
    WithdrawWithheldTokensFromMint(Ix<WithdrawWithheldTokensFromMint>),
    WithdrawWithheldTokensFromAccounts(Ix<WithdrawWithheldTokensFromAccounts>),
    HarvestWithheldTokensToMint(Ix<HarvestWithheldTokensToMint>),
    EnableHarvestToMint(Ix<EnableHarvestToMint>),
    DisableHarvestToMint(Ix<DisableHarvestToMint>),
}

impl ExtensionIxParser for ConfidentaltransferFeeIx {
    fn try_parse_extension_ix(ix: &Instruction) -> Result<Self, String> {
        let accounts_len = ix.accounts.len();
        let ix_type = decode_extension_ix_type(&ix.data)?;

        match ix_type {
            ConfidentialTransferFeeInstruction::InitializeConfidentialTransferFeeConfig => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(
                    ConfidentaltransferFeeIx::InitializeConfidentialTransferFeeConfig(
                        Ix::from_accounts(InitializeConfidentialTransferFeeConfigAccounts {
                            mint: ix.accounts[0],
                        }),
                    ),
                )
            },

            ConfidentialTransferFeeInstruction::WithdrawWithheldTokensFromMint => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(ConfidentaltransferFeeIx::WithdrawWithheldTokensFromMint(
                    Ix::from_accounts(WithdrawWithheldTokensFromMint {
                        mint: ix.accounts[0],
                        fee_recipient: ix.accounts[1],
                        sysvar: ix.accounts[2],
                        withdraw_withheld_authority: ix.accounts[3],
                        multisig_signers: get_multisig_signers(ix, 4),
                    }),
                ))
            },

            ConfidentialTransferFeeInstruction::WithdrawWithheldTokensFromAccounts => {
                check_min_accounts_req(accounts_len, 5)?;
                Ok(
                    ConfidentaltransferFeeIx::WithdrawWithheldTokensFromAccounts(
                        Ix::from_accounts(WithdrawWithheldTokensFromAccounts {
                            mint: ix.accounts[0],
                            fee_recipient: ix.accounts[1],
                            sysvar: ix.accounts[2],
                            withdraw_withheld_authority: ix.accounts[3],
                            source_accounts: ix.accounts[4..].to_vec(),
                            multisig_signers: None,
                        }),
                    ),
                )
            },

            ConfidentialTransferFeeInstruction::HarvestWithheldTokensToMint => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferFeeIx::HarvestWithheldTokensToMint(
                    Ix::from_accounts(HarvestWithheldTokensToMint {
                        mint: ix.accounts[0],
                        source_accounts: ix.accounts[1..].to_vec(),
                    }),
                ))
            },

            ConfidentialTransferFeeInstruction::EnableHarvestToMint => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferFeeIx::EnableHarvestToMint(
                    Ix::from_accounts(EnableHarvestToMint {
                        mint: ix.accounts[0],
                        confidential_transfer_fee_authority: ix.accounts[1],
                        multisig_signers: get_multisig_signers(ix, 2),
                    }),
                ))
            },

            ConfidentialTransferFeeInstruction::DisableHarvestToMint => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferFeeIx::DisableHarvestToMint(
                    Ix::from_accounts(DisableHarvestToMint {
                        account: ix.accounts[0],
                        confidential_transfer_fee_authority: ix.accounts[1],
                        multisig_signers: get_multisig_signers(ix, 2),
                    }),
                ))
            },
        }
    }
}
