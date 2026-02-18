use spl_token_2022::extension::confidential_transfer_fee::instruction::ConfidentialTransferFeeInstruction as SplConfidentialTransferFeeInstruction;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Result};
use yellowstone_vixen_proc_macro::vixen_proto;

use super::extension::{decode_extension_ix_type, ExtensionInstructionParser};
use crate::PubkeyBytes;

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeConfidentialTransferFeeConfigAccounts {
    pub mint: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ConfidentialWithdrawWithheldTokensFromMintAccounts {
    pub mint: PubkeyBytes,
    pub fee_recipient: PubkeyBytes,
    pub sysvar: PubkeyBytes,
    pub withdraw_withheld_authority: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ConfidentialWithdrawWithheldTokensFromAccounts {
    pub mint: PubkeyBytes,
    pub fee_recipient: PubkeyBytes,
    pub sysvar: PubkeyBytes,
    pub withdraw_withheld_authority: PubkeyBytes,
    pub source_accounts: Vec<PubkeyBytes>,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ConfidentialHarvestWithheldTokensToMintAccounts {
    pub mint: PubkeyBytes,
    pub source_accounts: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct EnableHarvestToMintAccounts {
    pub mint: PubkeyBytes,
    pub confidential_transfer_fee_authority: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct DisableHarvestToMintAccounts {
    pub account: PubkeyBytes,
    pub confidential_transfer_fee_authority: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct ConfidentialTransferFeeInstruction {
    #[vixen_proto_hint(
        oneof = "confidential_transfer_fee_instruction::Instruction",
        tags = "1, 2, 3, 4, 5, 6"
    )]
    pub instruction: Option<confidential_transfer_fee_instruction::Instruction>,
}

pub mod confidential_transfer_fee_instruction {
    use super::vixen_proto;

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct InitializeConfidentialTransferFeeConfig {
        pub accounts: Option<super::InitializeConfidentialTransferFeeConfigAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct WithdrawWithheldTokensFromMint {
        pub accounts: Option<super::ConfidentialWithdrawWithheldTokensFromMintAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct WithdrawWithheldTokensFromAccounts {
        pub accounts: Option<super::ConfidentialWithdrawWithheldTokensFromAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct HarvestWithheldTokensToMint {
        pub accounts: Option<super::ConfidentialHarvestWithheldTokensToMintAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct EnableHarvestToMint {
        pub accounts: Option<super::EnableHarvestToMintAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct DisableHarvestToMint {
        pub accounts: Option<super::DisableHarvestToMintAccounts>,
    }

    #[vixen_proto(oneof)]
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

impl ExtensionInstructionParser for ConfidentialTransferFeeInstruction {
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
                            mint: ix.accounts[0].to_vec(),
                        }),
                    },
                )
            },

            SplConfidentialTransferFeeInstruction::WithdrawWithheldTokensFromMint => {
                check_min_accounts_req(accounts_len, 4)?;

                oneof::Instruction::WithdrawWithheldTokensFromMint(
                    oneof::WithdrawWithheldTokensFromMint {
                        accounts: Some(ConfidentialWithdrawWithheldTokensFromMintAccounts {
                            mint: ix.accounts[0].to_vec(),
                            fee_recipient: ix.accounts[1].to_vec(),
                            sysvar: ix.accounts[2].to_vec(),
                            withdraw_withheld_authority: ix.accounts[3].to_vec(),
                            multisig_signers: ix.accounts[4..]
                                .iter()
                                .map(|pk| pk.to_vec())
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
                            mint: ix.accounts[0].to_vec(),
                            fee_recipient: ix.accounts[1].to_vec(),
                            sysvar: ix.accounts[2].to_vec(),
                            withdraw_withheld_authority: ix.accounts[3].to_vec(),
                            source_accounts: ix.accounts[4..]
                                .iter()
                                .map(|pk| pk.to_vec())
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
                            mint: ix.accounts[0].to_vec(),
                            source_accounts: ix.accounts[1..]
                                .iter()
                                .map(|pk| pk.to_vec())
                                .collect(),
                        }),
                    },
                )
            },

            SplConfidentialTransferFeeInstruction::EnableHarvestToMint => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::EnableHarvestToMint(oneof::EnableHarvestToMint {
                    accounts: Some(EnableHarvestToMintAccounts {
                        mint: ix.accounts[0].to_vec(),
                        confidential_transfer_fee_authority: ix.accounts[1].to_vec(),
                        multisig_signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplConfidentialTransferFeeInstruction::DisableHarvestToMint => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::DisableHarvestToMint(oneof::DisableHarvestToMint {
                    accounts: Some(DisableHarvestToMintAccounts {
                        account: ix.accounts[0].to_vec(),
                        confidential_transfer_fee_authority: ix.accounts[1].to_vec(),
                        multisig_signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },
        };

        Ok(ConfidentialTransferFeeInstruction {
            instruction: Some(ix_msg),
        })
    }
}
