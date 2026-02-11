use prost::alloc::vec::Vec;
use spl_token_2022::extension::transfer_fee::instruction::TransferFeeInstruction as SplTransferFeeInstruction;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};

use super::extension::ExtensionInstructionParser;
use crate::PubkeyBytes;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferCheckedWithFeeAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub source: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub destination: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferCheckedWithFeeArgs {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
    #[prost(uint64, tag = "2")]
    pub fee_amount: u64,
    // u8 -> uint32 in proto
    #[prost(uint32, tag = "3")]
    pub decimals: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeTransferFeeConfigAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeTransferFeeConfigArgs {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub transfer_fee_config_authority: Option<PubkeyBytes>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub withdraw_withheld_authority: Option<PubkeyBytes>,
    // u16 -> uint32 in proto
    #[prost(uint32, tag = "3")]
    pub transfer_fee_basis_points: u32,
    #[prost(uint64, tag = "4")]
    pub maximum_fee: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawWithheldTokensFromMintAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub fee_recipient: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub withdraw_withheld_authority: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawWithheldTokensFromAccountsAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub fee_recipient: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub withdraw_withheld_authority: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub source_accounts: Vec<PubkeyBytes>,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawWithheldTokensFromAccountsArgs {
    // u8 -> uint32 in proto
    #[prost(uint32, tag = "1")]
    pub num_token_accounts: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTransferFeeAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint_fee_acc_owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTransferFeeArgs {
    // u16 -> uint32 in proto
    #[prost(uint32, tag = "1")]
    pub transfer_fee_basis_points: u32,
    #[prost(uint64, tag = "2")]
    pub maximum_fee: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HarvestWithheldTokensToMintAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint_fee_acc_owner: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferFeeInstruction {
    #[prost(oneof = "transfer_fee_instruction::Ix", tags = "1, 2, 3, 4, 5, 6")]
    pub ix: Option<transfer_fee_instruction::Ix>,
}

pub mod transfer_fee_instruction {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransferCheckedWithFee {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::TransferCheckedWithFeeAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::TransferCheckedWithFeeArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitializeTransferFeeConfig {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::InitializeTransferFeeConfigAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::InitializeTransferFeeConfigArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawWithheldTokensFromMint {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::WithdrawWithheldTokensFromMintAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawWithheldTokensFromAccounts {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::WithdrawWithheldTokensFromAccountsAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::WithdrawWithheldTokensFromAccountsArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HarvestWithheldTokensToMint {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::HarvestWithheldTokensToMintAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetTransferFee {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<super::SetTransferFeeAccounts>,
        #[prost(message, optional, tag = "2")]
        pub args: Option<super::SetTransferFeeArgs>,
    }

    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ix {
        #[prost(message, tag = "1")]
        TransferCheckedWithFee(TransferCheckedWithFee),
        #[prost(message, tag = "2")]
        InitializeTransferFeeConfig(InitializeTransferFeeConfig),
        #[prost(message, tag = "3")]
        WithdrawWithheldTokensFromMint(WithdrawWithheldTokensFromMint),
        #[prost(message, tag = "4")]
        WithdrawWithheldTokensFromAccounts(WithdrawWithheldTokensFromAccounts),
        #[prost(message, tag = "5")]
        HarvestWithheldTokensToMint(HarvestWithheldTokensToMint),
        #[prost(message, tag = "6")]
        SetTransferFee(SetTransferFee),
    }
}

impl ExtensionInstructionParser for TransferFeeInstruction {
    #[allow(clippy::too_many_lines)]
    fn try_parse(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();

        let ix_type = SplTransferFeeInstruction::unpack(&ix.data[1..])
            .parse_err("Error unpacking transfer fee instruction data")?;

        use transfer_fee_instruction as oneof;

        let ix_msg = match ix_type {
            SplTransferFeeInstruction::TransferCheckedWithFee {
                amount,
                decimals,
                fee,
            } => {
                check_min_accounts_req(accounts_len, 4)?;

                oneof::Ix::TransferCheckedWithFee(oneof::TransferCheckedWithFee {
                    accounts: Some(TransferCheckedWithFeeAccounts {
                        source: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        destination: ix.accounts[2].to_vec(),
                        owner: ix.accounts[3].to_vec(),
                        multisig_signers: ix.accounts[4..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(TransferCheckedWithFeeArgs {
                        amount,
                        fee_amount: fee,
                        decimals: decimals as u32,
                    }),
                })
            },

            SplTransferFeeInstruction::InitializeTransferFeeConfig {
                transfer_fee_config_authority,
                withdraw_withheld_authority,
                transfer_fee_basis_points,
                maximum_fee,
            } => {
                check_min_accounts_req(accounts_len, 1)?;

                oneof::Ix::InitializeTransferFeeConfig(oneof::InitializeTransferFeeConfig {
                    accounts: Some(InitializeTransferFeeConfigAccounts {
                        mint: ix.accounts[0].to_vec(),
                    }),
                    args: Some(InitializeTransferFeeConfigArgs {
                        transfer_fee_config_authority: transfer_fee_config_authority
                            .map(|p| p.to_bytes().to_vec())
                            .into(),
                        withdraw_withheld_authority: withdraw_withheld_authority
                            .map(|p| p.to_bytes().to_vec())
                            .into(),
                        transfer_fee_basis_points: transfer_fee_basis_points as u32,
                        maximum_fee,
                    }),
                })
            },

            SplTransferFeeInstruction::WithdrawWithheldTokensFromMint => {
                check_min_accounts_req(accounts_len, 3)?;

                oneof::Ix::WithdrawWithheldTokensFromMint(oneof::WithdrawWithheldTokensFromMint {
                    accounts: Some(WithdrawWithheldTokensFromMintAccounts {
                        mint: ix.accounts[0].to_vec(),
                        fee_recipient: ix.accounts[1].to_vec(),
                        withdraw_withheld_authority: ix.accounts[2].to_vec(),
                        multisig_signers: ix.accounts[3..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplTransferFeeInstruction::WithdrawWithheldTokensFromAccounts {
                num_token_accounts,
            } => {
                let n = num_token_accounts as usize;

                check_min_accounts_req(accounts_len, 3 + n)?;

                oneof::Ix::WithdrawWithheldTokensFromAccounts(
                    oneof::WithdrawWithheldTokensFromAccounts {
                        accounts: Some(WithdrawWithheldTokensFromAccountsAccounts {
                            mint: ix.accounts[0].to_vec(),
                            fee_recipient: ix.accounts[1].to_vec(),
                            withdraw_withheld_authority: ix.accounts[2].to_vec(),
                            source_accounts: ix.accounts[3..(3 + n)]
                                .iter()
                                .map(|pk| pk.to_vec())
                                .collect(),
                            multisig_signers: ix.accounts[(3 + n)..]
                                .iter()
                                .map(|pk| pk.to_vec())
                                .collect(),
                        }),
                        args: Some(WithdrawWithheldTokensFromAccountsArgs {
                            num_token_accounts: num_token_accounts as u32,
                        }),
                    },
                )
            },

            SplTransferFeeInstruction::HarvestWithheldTokensToMint => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Ix::HarvestWithheldTokensToMint(oneof::HarvestWithheldTokensToMint {
                    accounts: Some(HarvestWithheldTokensToMintAccounts {
                        mint: ix.accounts[0].to_vec(),
                        mint_fee_acc_owner: ix.accounts[1].to_vec(),
                    }),
                })
            },

            SplTransferFeeInstruction::SetTransferFee {
                transfer_fee_basis_points,
                maximum_fee,
            } => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Ix::SetTransferFee(oneof::SetTransferFee {
                    accounts: Some(SetTransferFeeAccounts {
                        mint: ix.accounts[0].to_vec(),
                        mint_fee_acc_owner: ix.accounts[1].to_vec(),
                        multisig_signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(SetTransferFeeArgs {
                        transfer_fee_basis_points: transfer_fee_basis_points as u32,
                        maximum_fee,
                    }),
                })
            },
        };

        Ok(TransferFeeInstruction { ix: Some(ix_msg) })
    }
}
