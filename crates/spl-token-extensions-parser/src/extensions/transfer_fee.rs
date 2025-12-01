use spl_token_2022::extension::transfer_fee::instruction::TransferFeeInstruction as SplTransferFeeInstruction;
use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};

use super::extension::ExtensionInstructionParser;

#[derive(Debug)]
pub struct TransferCheckedWithFeeAccounts {
    pub source: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}
#[derive(Debug, Clone, Copy)]
pub struct TransferCheckedWithFeeArgs {
    pub amount: u64,
    pub fee_amount: u64,
    pub decimals: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeTransferFeeConfigAccounts {
    pub mint: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeTransferFeeConfigArgs {
    pub transfer_fee_config_authority: Option<Pubkey>,
    pub withdraw_withheld_authority: Option<Pubkey>,
    pub transfer_fee_basis_points: u16,
    pub maximum_fee: u64,
}

#[derive(Debug)]
pub struct WithdrawWithheldTokensFromMintAccounts {
    pub mint: Pubkey,
    pub fee_recipient: Pubkey,
    pub withdraw_withheld_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct WithdrawWithheldTokensFromAccountsAccounts {
    pub mint: Pubkey,
    pub fee_recipient: Pubkey,
    pub withdraw_withheld_authority: Pubkey,
    pub source_accounts: Vec<Pubkey>,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]
pub struct WithdrawWithheldTokensFromAccountsArgs {
    pub num_token_accounts: u8,
}

#[derive(Debug)]

pub struct SetTransferFeeAccounts {
    pub mint: Pubkey,
    pub mint_fee_acc_owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]

pub struct SetTransferFeeArgs {
    pub transfer_fee_basis_points: u16,
    pub maximum_fee: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct HarvestWithheldTokensToMintAccounts {
    pub mint: Pubkey,
    pub mint_fee_acc_owner: Pubkey,
}

#[derive(Debug)]
pub enum TransferFeeInstruction {
    TransferCheckedWithFee {
        accounts: TransferCheckedWithFeeAccounts,
        args: TransferCheckedWithFeeArgs,
    },
    InitializeTransferFeeConfig {
        accounts: InitializeTransferFeeConfigAccounts,
        args: InitializeTransferFeeConfigArgs,
    },
    WithdrawWithheldTokensFromMint {
        accounts: WithdrawWithheldTokensFromMintAccounts,
    },
    WithdrawWithheldTokensFromAccounts {
        accounts: WithdrawWithheldTokensFromAccountsAccounts,
        args: WithdrawWithheldTokensFromAccountsArgs,
    },
    HarvestWithheldTokensToMint {
        accounts: HarvestWithheldTokensToMintAccounts,
    },
    SetTransferFee {
        accounts: SetTransferFeeAccounts,
        args: SetTransferFeeArgs,
    },
}

impl ExtensionInstructionParser for TransferFeeInstruction {
    #[allow(clippy::too_many_lines)]
    fn try_parse(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();
        let ix_type = SplTransferFeeInstruction::unpack(&ix.data[1..])
            .parse_err("Error unpacking transfer fee instruction data")?;
        match ix_type {
            SplTransferFeeInstruction::TransferCheckedWithFee {
                amount,
                decimals,
                fee,
            } => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(TransferFeeInstruction::TransferCheckedWithFee {
                    accounts: TransferCheckedWithFeeAccounts {
                        source: ix.accounts[0],
                        mint: ix.accounts[1],
                        destination: ix.accounts[2],
                        owner: ix.accounts[3],
                        multisig_signers: ix.accounts[4..].to_vec(),
                    },
                    args: TransferCheckedWithFeeArgs {
                        amount,
                        fee_amount: fee,
                        decimals,
                    },
                })
            },

            SplTransferFeeInstruction::InitializeTransferFeeConfig {
                transfer_fee_config_authority,
                withdraw_withheld_authority,
                transfer_fee_basis_points,
                maximum_fee,
            } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TransferFeeInstruction::InitializeTransferFeeConfig {
                    accounts: InitializeTransferFeeConfigAccounts {
                        mint: ix.accounts[0],
                    },
                    args: InitializeTransferFeeConfigArgs {
                        transfer_fee_config_authority: transfer_fee_config_authority
                            .map(|p| p.to_bytes().into())
                            .into(),
                        withdraw_withheld_authority: withdraw_withheld_authority
                            .map(|p| p.to_bytes().into())
                            .into(),
                        transfer_fee_basis_points,
                        maximum_fee,
                    },
                })
            },

            SplTransferFeeInstruction::WithdrawWithheldTokensFromMint => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TransferFeeInstruction::WithdrawWithheldTokensFromMint {
                    accounts: WithdrawWithheldTokensFromMintAccounts {
                        mint: ix.accounts[0],
                        fee_recipient: ix.accounts[1],
                        withdraw_withheld_authority: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                })
            },

            SplTransferFeeInstruction::WithdrawWithheldTokensFromAccounts {
                num_token_accounts,
            } => {
                check_min_accounts_req(accounts_len, 3 + num_token_accounts as usize)?;
                Ok(TransferFeeInstruction::WithdrawWithheldTokensFromAccounts {
                    accounts: WithdrawWithheldTokensFromAccountsAccounts {
                        mint: ix.accounts[0],
                        fee_recipient: ix.accounts[1],
                        withdraw_withheld_authority: ix.accounts[2],
                        source_accounts: ix.accounts[3..(3 + num_token_accounts) as usize].to_vec(),
                        multisig_signers: ix.accounts[(3 + num_token_accounts as usize)..].to_vec(),
                    },
                    args: WithdrawWithheldTokensFromAccountsArgs { num_token_accounts },
                })
            },

            SplTransferFeeInstruction::HarvestWithheldTokensToMint => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TransferFeeInstruction::HarvestWithheldTokensToMint {
                    accounts: HarvestWithheldTokensToMintAccounts {
                        mint: ix.accounts[0],
                        mint_fee_acc_owner: ix.accounts[1],
                    },
                })
            },

            SplTransferFeeInstruction::SetTransferFee {
                transfer_fee_basis_points,
                maximum_fee,
            } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TransferFeeInstruction::SetTransferFee {
                    accounts: SetTransferFeeAccounts {
                        mint: ix.accounts[0],
                        mint_fee_acc_owner: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                    args: SetTransferFeeArgs {
                        transfer_fee_basis_points,
                        maximum_fee,
                    },
                })
            },
        }
    }
}
