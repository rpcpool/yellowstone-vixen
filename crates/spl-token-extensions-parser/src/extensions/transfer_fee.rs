use spl_token_2022::extension::transfer_fee::instruction::TransferFeeInstruction as SplTransferFeeInstruction;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};
use yellowstone_vixen_proc_macro::vixen_proto;

use super::extension::ExtensionInstructionParser;
use crate::PubkeyBytes;

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TransferCheckedWithFeeAccounts {
    pub source: PubkeyBytes,
    pub mint: PubkeyBytes,
    pub destination: PubkeyBytes,
    pub owner: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TransferCheckedWithFeeArgs {
    pub amount: u64,
    pub fee_amount: u64,
    // u8 -> uint32 in proto
    pub decimals: u32,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeTransferFeeConfigAccounts {
    pub mint: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct InitializeTransferFeeConfigArgs {
    pub transfer_fee_config_authority: Option<PubkeyBytes>,
    pub withdraw_withheld_authority: Option<PubkeyBytes>,
    // u16 -> uint32 in proto
    pub transfer_fee_basis_points: u32,
    pub maximum_fee: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawWithheldTokensFromMintAccounts {
    pub mint: PubkeyBytes,
    pub fee_recipient: PubkeyBytes,
    pub withdraw_withheld_authority: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawWithheldTokensFromAccountsAccounts {
    pub mint: PubkeyBytes,
    pub fee_recipient: PubkeyBytes,
    pub withdraw_withheld_authority: PubkeyBytes,
    pub source_accounts: Vec<PubkeyBytes>,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct WithdrawWithheldTokensFromAccountsArgs {
    // u8 -> uint32 in proto
    pub num_token_accounts: u32,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetTransferFeeAccounts {
    pub mint: PubkeyBytes,
    pub mint_fee_acc_owner: PubkeyBytes,
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct SetTransferFeeArgs {
    // u16 -> uint32 in proto
    pub transfer_fee_basis_points: u32,
    pub maximum_fee: u64,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct HarvestWithheldTokensToMintAccounts {
    pub mint: PubkeyBytes,
    pub mint_fee_acc_owner: PubkeyBytes,
}

#[vixen_proto]
#[derive(Clone, PartialEq)]
pub struct TransferFeeIx {
    #[vixen_proto_hint(
        oneof = "transfer_fee_instruction::Instruction",
        tags = "1, 2, 3, 4, 5, 6"
    )]
    pub instruction: Option<transfer_fee_instruction::Instruction>,
}

pub mod transfer_fee_instruction {
    use super::vixen_proto;

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct TransferCheckedWithFee {
        pub accounts: Option<super::TransferCheckedWithFeeAccounts>,
        pub args: Option<super::TransferCheckedWithFeeArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct InitializeTransferFeeConfig {
        pub accounts: Option<super::InitializeTransferFeeConfigAccounts>,
        pub args: Option<super::InitializeTransferFeeConfigArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct WithdrawWithheldTokensFromMint {
        pub accounts: Option<super::WithdrawWithheldTokensFromMintAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct WithdrawWithheldTokensFromAccounts {
        pub accounts: Option<super::WithdrawWithheldTokensFromAccountsAccounts>,
        pub args: Option<super::WithdrawWithheldTokensFromAccountsArgs>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct HarvestWithheldTokensToMint {
        pub accounts: Option<super::HarvestWithheldTokensToMintAccounts>,
    }

    #[vixen_proto]
    #[derive(Clone, PartialEq)]
    pub struct SetTransferFee {
        pub accounts: Option<super::SetTransferFeeAccounts>,
        pub args: Option<super::SetTransferFeeArgs>,
    }

    #[vixen_proto(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Instruction {
        TransferCheckedWithFee(TransferCheckedWithFee),
        InitializeTransferFeeConfig(InitializeTransferFeeConfig),
        WithdrawWithheldTokensFromMint(WithdrawWithheldTokensFromMint),
        WithdrawWithheldTokensFromAccounts(WithdrawWithheldTokensFromAccounts),
        HarvestWithheldTokensToMint(HarvestWithheldTokensToMint),
        SetTransferFee(SetTransferFee),
    }
}

impl ExtensionInstructionParser for TransferFeeIx {
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

                oneof::Instruction::TransferCheckedWithFee(oneof::TransferCheckedWithFee {
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

                oneof::Instruction::InitializeTransferFeeConfig(
                    oneof::InitializeTransferFeeConfig {
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
                    },
                )
            },

            SplTransferFeeInstruction::WithdrawWithheldTokensFromMint => {
                check_min_accounts_req(accounts_len, 3)?;

                oneof::Instruction::WithdrawWithheldTokensFromMint(
                    oneof::WithdrawWithheldTokensFromMint {
                        accounts: Some(WithdrawWithheldTokensFromMintAccounts {
                            mint: ix.accounts[0].to_vec(),
                            fee_recipient: ix.accounts[1].to_vec(),
                            withdraw_withheld_authority: ix.accounts[2].to_vec(),
                            multisig_signers: ix.accounts[3..]
                                .iter()
                                .map(|pk| pk.to_vec())
                                .collect(),
                        }),
                    },
                )
            },

            SplTransferFeeInstruction::WithdrawWithheldTokensFromAccounts {
                num_token_accounts,
            } => {
                let n = num_token_accounts as usize;

                check_min_accounts_req(accounts_len, 3 + n)?;

                oneof::Instruction::WithdrawWithheldTokensFromAccounts(
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

                oneof::Instruction::HarvestWithheldTokensToMint(
                    oneof::HarvestWithheldTokensToMint {
                        accounts: Some(HarvestWithheldTokensToMintAccounts {
                            mint: ix.accounts[0].to_vec(),
                            mint_fee_acc_owner: ix.accounts[1].to_vec(),
                        }),
                    },
                )
            },

            SplTransferFeeInstruction::SetTransferFee {
                transfer_fee_basis_points,
                maximum_fee,
            } => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::SetTransferFee(oneof::SetTransferFee {
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

        Ok(TransferFeeIx {
            instruction: Some(ix_msg),
        })
    }
}
