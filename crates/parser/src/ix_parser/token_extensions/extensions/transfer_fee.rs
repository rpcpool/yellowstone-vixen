use spl_pod::solana_program::{program_option::COption, pubkey::Pubkey};
use spl_token_2022::extension::transfer_fee::instruction::TransferFeeInstruction;

use super::helpers::ExtensionIxParser;
use crate::ix_parser::vixen_ix::{
    helpers::check_min_accounts_req,
    structure::{InstructionUpdate, ReadableInstruction},
};

#[derive(Debug)]
pub struct TransferCheckedWithFeeAccounts {
    pub source: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
}
#[derive(Debug)]
pub struct TransferCheckedWithFeeData {
    pub amount: u64,
    pub fee_amount: u64,
    pub decimals: u8,
}

#[derive(Debug)]
pub struct InitializeTransferFeeConfigAccounts {
    pub mint: Pubkey,
}

#[derive(Debug)]
pub struct InitializeTransferFeeConfigData {
    pub transfer_fee_config_authority: COption<Pubkey>,
    pub withdraw_withheld_authority: COption<Pubkey>,
    pub transfer_fee_basis_points: u16,
    pub maximum_fee: u64,
}

#[derive(Debug)]

pub struct WithdrawWithheldTokensFromMintAccounts {
    pub mint: Pubkey,
    pub fee_recipient: Pubkey,
    pub withdraw_withheld_authority: Pubkey,
}

#[derive(Debug)]
pub struct WithdrawWithheldTokensFromAccountsAccounts {
    pub mint: Pubkey,
    pub fee_recipient: Pubkey,
    pub withdraw_withheld_authority: Pubkey,
    pub source_accounts: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct WithdrawWithheldTokensFromAccountsData {
    pub num_token_accounts: u8,
}

#[derive(Debug)]

pub struct HarvestWithheldTokensToMintAccounts {
    pub mint: Pubkey,
    pub source_accounts: Vec<Pubkey>,
}

#[derive(Debug)]

pub struct SetTransferFeeAccounts {
    pub mint: Pubkey,
    pub mint_fee_acc_owner: Pubkey,
}

#[derive(Debug)]

pub struct SetTransferFeeData {
    pub transfer_fee_basis_points: u16,
    pub maximum_fee: u64,
}

#[derive(Debug)]
pub enum TransferFeeIx {
    TransferCheckedWithFee(
        ReadableInstruction<TransferCheckedWithFeeAccounts, TransferCheckedWithFeeData>,
    ),
    InitializeTransferFeeConfig(
        ReadableInstruction<InitializeTransferFeeConfigAccounts, InitializeTransferFeeConfigData>,
    ),
    WithdrawWithheldTokensFromMint(ReadableInstruction<WithdrawWithheldTokensFromMintAccounts, ()>),

    WithdrawWithheldTokensFromAccounts(
        ReadableInstruction<
            WithdrawWithheldTokensFromAccountsAccounts,
            WithdrawWithheldTokensFromAccountsData,
        >,
    ),

    HarvestWithheldTokensToMint(ReadableInstruction<HarvestWithheldTokensToMintAccounts, ()>),

    SetTransferFee(ReadableInstruction<SetTransferFeeAccounts, SetTransferFeeData>),
}

impl ExtensionIxParser for TransferFeeIx {
    fn try_parse_extension_ix(ix_data: &InstructionUpdate) -> Result<Self, String> {
        let accounts_len = ix_data.accounts.len();
        let ix_type = TransferFeeInstruction::unpack(&ix_data.data).map_err(|e| e.to_string())?;
        match ix_type {
            TransferFeeInstruction::TransferCheckedWithFee {
                amount,
                decimals,
                fee,
            } => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(TransferFeeIx::TransferCheckedWithFee(ReadableInstruction {
                    accounts: TransferCheckedWithFeeAccounts {
                        source: ix_data.accounts[0],
                        mint: ix_data.accounts[1],
                        destination: ix_data.accounts[2],
                    },
                    data: Some(TransferCheckedWithFeeData {
                        amount,
                        fee_amount: fee,
                        decimals,
                    }),
                }))
            },

            TransferFeeInstruction::InitializeTransferFeeConfig {
                transfer_fee_config_authority,
                withdraw_withheld_authority,
                transfer_fee_basis_points,
                maximum_fee,
            } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TransferFeeIx::InitializeTransferFeeConfig(
                    ReadableInstruction {
                        accounts: InitializeTransferFeeConfigAccounts {
                            mint: ix_data.accounts[0],
                        },
                        data: Some(InitializeTransferFeeConfigData {
                            transfer_fee_config_authority,
                            withdraw_withheld_authority,
                            transfer_fee_basis_points,
                            maximum_fee,
                        }),
                    },
                ))
            },

            TransferFeeInstruction::WithdrawWithheldTokensFromMint => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TransferFeeIx::WithdrawWithheldTokensFromMint(
                    ReadableInstruction::from_accounts(WithdrawWithheldTokensFromMintAccounts {
                        mint: ix_data.accounts[0],
                        fee_recipient: ix_data.accounts[1],
                        withdraw_withheld_authority: ix_data.accounts[2],
                    }),
                ))
            },

            TransferFeeInstruction::WithdrawWithheldTokensFromAccounts { num_token_accounts } => {
                check_min_accounts_req(accounts_len, 3 + num_token_accounts as usize)?;
                Ok(TransferFeeIx::WithdrawWithheldTokensFromAccounts(
                    ReadableInstruction {
                        accounts: WithdrawWithheldTokensFromAccountsAccounts {
                            mint: ix_data.accounts[0],
                            fee_recipient: ix_data.accounts[1],
                            withdraw_withheld_authority: ix_data.accounts[2],
                            source_accounts: ix_data.accounts[3..].to_vec(),
                        },
                        data: Some(WithdrawWithheldTokensFromAccountsData { num_token_accounts }),
                    },
                ))
            },

            TransferFeeInstruction::HarvestWithheldTokensToMint => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TransferFeeIx::HarvestWithheldTokensToMint(
                    ReadableInstruction::from_accounts(HarvestWithheldTokensToMintAccounts {
                        mint: ix_data.accounts[0],
                        source_accounts: ix_data.accounts[1..].to_vec(),
                    }),
                ))
            },

            TransferFeeInstruction::SetTransferFee {
                transfer_fee_basis_points,
                maximum_fee,
            } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TransferFeeIx::SetTransferFee(ReadableInstruction {
                    accounts: SetTransferFeeAccounts {
                        mint: ix_data.accounts[0],
                        mint_fee_acc_owner: ix_data.accounts[1],
                    },
                    data: Some(SetTransferFeeData {
                        transfer_fee_basis_points,
                        maximum_fee,
                    }),
                }))
            },
        }
    }
}
