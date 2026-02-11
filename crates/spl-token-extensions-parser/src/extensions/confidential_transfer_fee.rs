use prost::alloc::vec::Vec;
use spl_token_2022::extension::confidential_transfer_fee::instruction::ConfidentialTransferFeeInstruction as SplConfidentialTransferFeeInstruction;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Result};

use super::extension::{decode_extension_ix_type, ExtensionInstructionParser};
use crate::PubkeyBytes;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeConfidentialTransferFeeConfigAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfidentialWithdrawWithheldTokensFromMintAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub fee_recipient: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub sysvar: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub withdraw_withheld_authority: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfidentialWithdrawWithheldTokensFromAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub fee_recipient: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub sysvar: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub withdraw_withheld_authority: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub source_accounts: Vec<PubkeyBytes>,
    #[prost(bytes = "vec", repeated, tag = "6")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfidentialHarvestWithheldTokensToMintAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub source_accounts: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableHarvestToMintAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub confidential_transfer_fee_authority: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableHarvestToMintAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub confidential_transfer_fee_authority: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfidentialTransferFeeInstruction {
    #[prost(
        oneof = "confidential_transfer_fee_instruction::Ix",
        tags = "1, 2, 3, 4, 5, 6"
    )]
    pub ix: Option<confidential_transfer_fee_instruction::Ix>,
}

pub mod confidential_transfer_fee_instruction {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitializeConfidentialTransferFeeConfig {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::InitializeConfidentialTransferFeeConfigAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawWithheldTokensFromMint {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::ConfidentialWithdrawWithheldTokensFromMintAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawWithheldTokensFromAccounts {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::ConfidentialWithdrawWithheldTokensFromAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HarvestWithheldTokensToMint {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::ConfidentialHarvestWithheldTokensToMintAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnableHarvestToMint {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::EnableHarvestToMintAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisableHarvestToMint {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::DisableHarvestToMintAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ix {
        #[prost(message, tag = "1")]
        InitializeConfidentialTransferFeeConfig(InitializeConfidentialTransferFeeConfig),

        #[prost(message, tag = "2")]
        WithdrawWithheldTokensFromMint(WithdrawWithheldTokensFromMint),

        #[prost(message, tag = "3")]
        WithdrawWithheldTokensFromAccounts(WithdrawWithheldTokensFromAccounts),

        #[prost(message, tag = "4")]
        HarvestWithheldTokensToMint(HarvestWithheldTokensToMint),

        #[prost(message, tag = "5")]
        EnableHarvestToMint(EnableHarvestToMint),

        #[prost(message, tag = "6")]
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

                oneof::Ix::InitializeConfidentialTransferFeeConfig(
                    oneof::InitializeConfidentialTransferFeeConfig {
                        accounts: Some(InitializeConfidentialTransferFeeConfigAccounts {
                            mint: ix.accounts[0].to_vec(),
                        }),
                    },
                )
            },

            SplConfidentialTransferFeeInstruction::WithdrawWithheldTokensFromMint => {
                check_min_accounts_req(accounts_len, 4)?;

                oneof::Ix::WithdrawWithheldTokensFromMint(oneof::WithdrawWithheldTokensFromMint {
                    accounts: Some(ConfidentialWithdrawWithheldTokensFromMintAccounts {
                        mint: ix.accounts[0].to_vec(),
                        fee_recipient: ix.accounts[1].to_vec(),
                        sysvar: ix.accounts[2].to_vec(),
                        withdraw_withheld_authority: ix.accounts[3].to_vec(),
                        multisig_signers: ix.accounts[4..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplConfidentialTransferFeeInstruction::WithdrawWithheldTokensFromAccounts => {
                check_min_accounts_req(accounts_len, 5)?;

                oneof::Ix::WithdrawWithheldTokensFromAccounts(
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

                oneof::Ix::HarvestWithheldTokensToMint(oneof::HarvestWithheldTokensToMint {
                    accounts: Some(ConfidentialHarvestWithheldTokensToMintAccounts {
                        mint: ix.accounts[0].to_vec(),
                        source_accounts: ix.accounts[1..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplConfidentialTransferFeeInstruction::EnableHarvestToMint => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Ix::EnableHarvestToMint(oneof::EnableHarvestToMint {
                    accounts: Some(EnableHarvestToMintAccounts {
                        mint: ix.accounts[0].to_vec(),
                        confidential_transfer_fee_authority: ix.accounts[1].to_vec(),
                        multisig_signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplConfidentialTransferFeeInstruction::DisableHarvestToMint => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Ix::DisableHarvestToMint(oneof::DisableHarvestToMint {
                    accounts: Some(DisableHarvestToMintAccounts {
                        account: ix.accounts[0].to_vec(),
                        confidential_transfer_fee_authority: ix.accounts[1].to_vec(),
                        multisig_signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },
        };

        Ok(ConfidentialTransferFeeInstruction { ix: Some(ix_msg) })
    }
}
