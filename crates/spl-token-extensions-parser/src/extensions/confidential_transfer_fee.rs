use spl_token_2022::extension::confidential_transfer_fee::instruction::ConfidentialTransferFeeInstruction as SplConfidentialTransferFeeInstruction;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Result};
use yellowstone_vixen_proc_macro::vixen;

use super::extension::{decode_extension_ix_type, ExtensionInstructionParser};
use crate::PublicKey;

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeConfidentialTransferFeeConfigAccounts {
    pub mint: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ConfidentialWithdrawWithheldTokensFromMintAccounts {
    pub mint: PublicKey,
    pub fee_recipient: PublicKey,
    pub sysvar: PublicKey,
    pub withdraw_withheld_authority: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ConfidentialWithdrawWithheldTokensFromAccounts {
    pub mint: PublicKey,
    pub fee_recipient: PublicKey,
    pub sysvar: PublicKey,
    pub withdraw_withheld_authority: PublicKey,
    pub source_accounts: Vec<PublicKey>,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ConfidentialHarvestWithheldTokensToMintAccounts {
    pub mint: PublicKey,
    pub source_accounts: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct EnableHarvestToMintAccounts {
    pub mint: PublicKey,
    pub confidential_transfer_fee_authority: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DisableHarvestToMintAccounts {
    pub account: PublicKey,
    pub confidential_transfer_fee_authority: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ConfidentialTransferFeeIx {
    #[hint(
        oneof = "confidential_transfer_fee_instruction::Instruction",
        tags = "1, 2, 3, 4, 5, 6"
    )]
    pub instruction: Option<confidential_transfer_fee_instruction::Instruction>,
}

pub mod confidential_transfer_fee_instruction {
    use super::vixen;

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeConfidentialTransferFeeConfig {
        pub accounts: Option<super::InitializeConfidentialTransferFeeConfigAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct WithdrawWithheldTokensFromMint {
        pub accounts: Option<super::ConfidentialWithdrawWithheldTokensFromMintAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct WithdrawWithheldTokensFromAccounts {
        pub accounts: Option<super::ConfidentialWithdrawWithheldTokensFromAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct HarvestWithheldTokensToMint {
        pub accounts: Option<super::ConfidentialHarvestWithheldTokensToMintAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct EnableHarvestToMint {
        pub accounts: Option<super::EnableHarvestToMintAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct DisableHarvestToMint {
        pub accounts: Option<super::DisableHarvestToMintAccounts>,
    }

    #[vixen(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Instruction {
        InitializeConfidentialTransferFeeConfig(InitializeConfidentialTransferFeeConfig),

        WithdrawWithheldTokensFromMint(WithdrawWithheldTokensFromMint),

        WithdrawWithheldTokensFromAccounts(WithdrawWithheldTokensFromAccounts),

        HarvestWithheldTokensToMint(HarvestWithheldTokensToMint),

        EnableHarvestToMint(EnableHarvestToMint),

        DisableHarvestToMint(DisableHarvestToMint),
    }
}

impl ExtensionInstructionParser for ConfidentialTransferFeeIx {
    fn try_parse(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();
        let ix_type = decode_extension_ix_type(&ix.data[1..])?;

        use confidential_transfer_fee_instruction as oneof;

        let ix_msg = match ix_type {
            SplConfidentialTransferFeeInstruction::InitializeConfidentialTransferFeeConfig => {
                check_min_accounts_req(accounts_len, 1)?;

                oneof::Instruction::InitializeConfidentialTransferFeeConfig(
                    oneof::InitializeConfidentialTransferFeeConfig {
                        accounts: Some(InitializeConfidentialTransferFeeConfigAccounts {
                            mint: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        }),
                    },
                )
            },

            SplConfidentialTransferFeeInstruction::WithdrawWithheldTokensFromMint => {
                check_min_accounts_req(accounts_len, 4)?;

                oneof::Instruction::WithdrawWithheldTokensFromMint(
                    oneof::WithdrawWithheldTokensFromMint {
                        accounts: Some(ConfidentialWithdrawWithheldTokensFromMintAccounts {
                            mint: crate::PublicKey::new(ix.accounts[0].to_vec()),
                            fee_recipient: crate::PublicKey::new(ix.accounts[1].to_vec()),
                            sysvar: crate::PublicKey::new(ix.accounts[2].to_vec()),
                            withdraw_withheld_authority: crate::PublicKey::new(
                                ix.accounts[3].to_vec(),
                            ),
                            multisig_signers: ix.accounts[4..]
                                .iter()
                                .map(|a| crate::PublicKey::new(a.to_vec()))
                                .collect(),
                        }),
                    },
                )
            },

            SplConfidentialTransferFeeInstruction::WithdrawWithheldTokensFromAccounts => {
                check_min_accounts_req(accounts_len, 5)?;

                oneof::Instruction::WithdrawWithheldTokensFromAccounts(
                    oneof::WithdrawWithheldTokensFromAccounts {
                        accounts: Some(ConfidentialWithdrawWithheldTokensFromAccounts {
                            mint: crate::PublicKey::new(ix.accounts[0].to_vec()),
                            fee_recipient: crate::PublicKey::new(ix.accounts[1].to_vec()),
                            sysvar: crate::PublicKey::new(ix.accounts[2].to_vec()),
                            withdraw_withheld_authority: crate::PublicKey::new(
                                ix.accounts[3].to_vec(),
                            ),
                            source_accounts: ix.accounts[4..]
                                .iter()
                                .map(|a| crate::PublicKey::new(a.to_vec()))
                                .collect(),
                            multisig_signers: Vec::new(),
                        }),
                    },
                )
            },

            SplConfidentialTransferFeeInstruction::HarvestWithheldTokensToMint => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::HarvestWithheldTokensToMint(
                    oneof::HarvestWithheldTokensToMint {
                        accounts: Some(ConfidentialHarvestWithheldTokensToMintAccounts {
                            mint: crate::PublicKey::new(ix.accounts[0].to_vec()),
                            source_accounts: ix.accounts[1..]
                                .iter()
                                .map(|a| crate::PublicKey::new(a.to_vec()))
                                .collect(),
                        }),
                    },
                )
            },

            SplConfidentialTransferFeeInstruction::EnableHarvestToMint => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::EnableHarvestToMint(oneof::EnableHarvestToMint {
                    accounts: Some(EnableHarvestToMintAccounts {
                        mint: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        confidential_transfer_fee_authority: crate::PublicKey::new(
                            ix.accounts[1].to_vec(),
                        ),
                        multisig_signers: ix.accounts[2..]
                            .iter()
                            .map(|a| crate::PublicKey::new(a.to_vec()))
                            .collect(),
                    }),
                })
            },

            SplConfidentialTransferFeeInstruction::DisableHarvestToMint => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::DisableHarvestToMint(oneof::DisableHarvestToMint {
                    accounts: Some(DisableHarvestToMintAccounts {
                        account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        confidential_transfer_fee_authority: crate::PublicKey::new(
                            ix.accounts[1].to_vec(),
                        ),
                        multisig_signers: ix.accounts[2..]
                            .iter()
                            .map(|a| crate::PublicKey::new(a.to_vec()))
                            .collect(),
                    }),
                })
            },
        };

        Ok(ConfidentialTransferFeeIx {
            instruction: Some(ix_msg),
        })
    }
}
