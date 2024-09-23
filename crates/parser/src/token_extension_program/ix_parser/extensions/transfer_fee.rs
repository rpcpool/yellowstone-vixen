use spl_token_2022::extension::transfer_fee::instruction::TransferFeeInstruction;
use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};

use super::helpers::ExtensionIxParser;
use crate::{
    helpers::{check_min_accounts_req, into_vixen_pubkey},
    Result, ResultExt,
};

#[derive(Debug)]
pub struct TransferCheckedWithFeeAccounts {
    pub source: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}
#[derive(Debug, Clone, Copy)]
pub struct TransferCheckedWithFeeData {
    pub amount: u64,
    pub fee_amount: u64,
    pub decimals: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeTransferFeeConfigAccounts {
    pub mint: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct InitializeTransferFeeConfigData {
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
pub struct WithdrawWithheldTokensFromAccountsData {
    pub num_token_accounts: u8,
}

#[derive(Debug)]

pub struct HarvestWithheldTokensToMintAccountsTransferFee {
    pub mint: Pubkey,
    pub source_accounts: Vec<Pubkey>,
}

#[derive(Debug)]

pub struct SetTransferFeeAccounts {
    pub mint: Pubkey,
    pub mint_fee_acc_owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]

pub struct SetTransferFeeData {
    pub transfer_fee_basis_points: u16,
    pub maximum_fee: u64,
}

#[derive(Debug)]
pub enum TransferFeeIx {
    TransferCheckedWithFee(TransferCheckedWithFeeAccounts, TransferCheckedWithFeeData),
    InitializeTransferFeeConfig(
        InitializeTransferFeeConfigAccounts,
        InitializeTransferFeeConfigData,
    ),
    WithdrawWithheldTokensFromMint(WithdrawWithheldTokensFromMintAccounts),

    WithdrawWithheldTokensFromAccounts(
        WithdrawWithheldTokensFromAccountsAccounts,
        WithdrawWithheldTokensFromAccountsData,
    ),

    HarvestWithheldTokensToMint(HarvestWithheldTokensToMintAccounts),

    SetTransferFee(SetTransferFeeAccounts, SetTransferFeeData),
}

impl ExtensionIxParser for TransferFeeIx {
    #[allow(clippy::too_many_lines)]
    fn try_parse_extension_ix(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();
        let ix_type = TransferFeeInstruction::unpack(&ix.data)
            .parse_err("Error unpacking transfer fee instruction data")?;
        match ix_type {
            TransferFeeInstruction::TransferCheckedWithFee {
                amount,
                decimals,
                fee,
            } => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(TransferFeeIx::TransferCheckedWithFee(
                    TransferCheckedWithFeeAccounts {
                        source: ix.accounts[0],
                        mint: ix.accounts[1],
                        destination: ix.accounts[2],
                        owner: ix.accounts[3],
                        multisig_signers: ix.accounts[4..].to_vec(),
                    },
                    TransferCheckedWithFeeData {
                        amount,
                        fee_amount: fee,
                        decimals,
                    },
                ))
            },

            TransferFeeInstruction::InitializeTransferFeeConfig {
                transfer_fee_config_authority,
                withdraw_withheld_authority,
                transfer_fee_basis_points,
                maximum_fee,
            } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TransferFeeIx::InitializeTransferFeeConfig(
                    InitializeTransferFeeConfigAccounts {
                        mint: ix.accounts[0],
                    },
                    InitializeTransferFeeConfigData {
                        transfer_fee_config_authority: transfer_fee_config_authority
                            .map(into_vixen_pubkey)
                            .into(),
                        withdraw_withheld_authority: withdraw_withheld_authority
                            .map(into_vixen_pubkey)
                            .into(),
                        transfer_fee_basis_points,
                        maximum_fee,
                    },
                ))
            },

            TransferFeeInstruction::WithdrawWithheldTokensFromMint => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TransferFeeIx::WithdrawWithheldTokensFromMint(
                    WithdrawWithheldTokensFromMintAccounts {
                        mint: ix.accounts[0],
                        fee_recipient: ix.accounts[1],
                        withdraw_withheld_authority: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                ))
            },

            TransferFeeInstruction::WithdrawWithheldTokensFromAccounts { num_token_accounts } => {
                check_min_accounts_req(accounts_len, 3 + num_token_accounts as usize)?;
                Ok(TransferFeeIx::WithdrawWithheldTokensFromAccounts(
                    WithdrawWithheldTokensFromAccountsAccounts {
                        mint: ix.accounts[0],
                        fee_recipient: ix.accounts[1],
                        withdraw_withheld_authority: ix.accounts[2],
                        source_accounts: ix.accounts[3..(3 + num_token_accounts) as usize].to_vec(),
                        multisig_signers: ix.accounts[(3 + num_token_accounts as usize)..].to_vec(),
                    },
                    WithdrawWithheldTokensFromAccountsData { num_token_accounts },
                ))
            },

            TransferFeeInstruction::HarvestWithheldTokensToMint => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TransferFeeIx::HarvestWithheldTokensToMint(
                    HarvestWithheldTokensToMintAccounts {
                        mint: ix.accounts[0],
                        source_accounts: ix.accounts[1..].to_vec(),
                    },
                ))
            },

            TransferFeeInstruction::SetTransferFee {
                transfer_fee_basis_points,
                maximum_fee,
            } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TransferFeeIx::SetTransferFee(
                    SetTransferFeeAccounts {
                        mint: ix.accounts[0],
                        mint_fee_acc_owner: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                    SetTransferFeeData {
                        transfer_fee_basis_points,
                        maximum_fee,
                    },
                ))
            },
        }
    }
}

#[cfg(feature = "proto")]
mod proto_parser {
    use yellowstone_vixen_proto::parser::*;

    use super::*;
    use crate::helpers::{
        FromCOptionPubkeyToOptString, FromOptVecToDefVec, FromOptionToProtoOption,
        FromVecPubkeyToVecString, IntoProtoData,
    };

    impl IntoProtoData<InitializeTransferFeeConfigAccountsProto>
        for InitializeTransferFeeConfigAccounts
    {
        fn into_proto_data(self) -> InitializeTransferFeeConfigAccountsProto {
            InitializeTransferFeeConfigAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProtoData<TransferCheckedWithFeeAccountsProto> for TransferCheckedWithFeeAccounts {
        fn into_proto_data(self) -> TransferCheckedWithFeeAccountsProto {
            TransferCheckedWithFeeAccountsProto {
                source: self.source.to_string(),
                mint: self.mint.to_string(),
                destination: self.destination.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<WithdrawWithheldTokensFromMintAccountsProto>
        for WithdrawWithheldTokensFromMintAccountsTransferFee
    {
        fn into_proto_data(self) -> WithdrawWithheldTokensFromMintAccountsProto {
            WithdrawWithheldTokensFromMintAccountsProto {
                mint: self.mint.to_string(),
                fee_recipient: self.fee_recipient.to_string(),
                withdraw_withheld_authority: self.withdraw_withheld_authority.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<WithdrawWithheldTokensFromAccountsAccountsProto2>
        for WithdrawWithheldTokensFromAccountsAccounts
    {
        fn into_proto_data(self) -> WithdrawWithheldTokensFromAccountsAccountsProto2 {
            WithdrawWithheldTokensFromAccountsAccountsProto2 {
                mint: self.mint.to_string(),
                fee_recipient: self.fee_recipient.to_string(),
                withdraw_withheld_authority: self.withdraw_withheld_authority.to_string(),
                source_accounts: self.source_accounts.to_string_vec(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<HarvestWithheldTokensToMintAccountsProto>
        for HarvestWithheldTokensToMintAccountsTransferFee
    {
        fn into_proto_data(self) -> HarvestWithheldTokensToMintAccountsProto {
            HarvestWithheldTokensToMintAccountsProto {
                mint: self.mint.to_string(),
                source_accounts: self.source_accounts.to_string_vec(),
            }
        }
    }

    impl IntoProtoData<SetTransferFeeAccountsProto> for SetTransferFeeAccounts {
        fn into_proto_data(self) -> SetTransferFeeAccountsProto {
            SetTransferFeeAccountsProto {
                mint: self.mint.to_string(),
                mint_fee_acc_owner: self.mint_fee_acc_owner.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<TransferCheckedWithFeeDataProto> for TransferCheckedWithFeeData {
        fn into_proto_data(self) -> TransferCheckedWithFeeDataProto {
            TransferCheckedWithFeeDataProto {
                amount: self.amount,
                fee_amount: self.fee_amount,
                decimals: self.decimals.into(),
            }
        }
    }

    impl IntoProtoData<InitializeTransferFeeConfigDataProto> for InitializeTransferFeeConfigData {
        fn into_proto_data(self) -> InitializeTransferFeeConfigDataProto {
            InitializeTransferFeeConfigDataProto {
                transfer_fee_config_authority: self.transfer_fee_config_authority.to_opt_string(),
                withdraw_withheld_authority: self.withdraw_withheld_authority.to_opt_string(),
                transfer_fee_basis_points: self.transfer_fee_basis_points.into(),
                maximum_fee: self.maximum_fee,
            }
        }
    }

    impl IntoProtoData<WithdrawWithheldTokensFromAccountsDataProto2>
        for WithdrawWithheldTokensFromAccountsData
    {
        fn into_proto_data(self) -> WithdrawWithheldTokensFromAccountsDataProto2 {
            WithdrawWithheldTokensFromAccountsDataProto2 {
                num_token_accounts: self.num_token_accounts.into(),
            }
        }
    }

    impl IntoProtoData<SetTransferFeeDataProto> for SetTransferFeeData {
        fn into_proto_data(self) -> SetTransferFeeDataProto {
            SetTransferFeeDataProto {
                transfer_fee_basis_points: self.transfer_fee_basis_points.into(),
                maximum_fee: self.maximum_fee.into(),
            }
        }
    }

    impl IntoProtoData<TransferFeeIxProto> for TransferFeeIx {
        fn into_proto_data(self) -> TransferFeeIxProto {
            match self {
                TransferFeeIx::InitializeTransferFeeConfig(data) => TransferFeeIxProto {
                    ix_oneof: Some(
                        transfer_fee_ix_proto::IxOneof::InitializeTransferFeeConfigIx(
                            InitializeTransferFeeConfigIxProto {
                                accounts: Some(data.accounts.into_proto_data()),
                                data: data.data.to_proto_option(),
                            },
                        ),
                    ),
                },
                TransferFeeIx::TransferCheckedWithFee(data) => TransferFeeIxProto {
                    ix_oneof: Some(transfer_fee_ix_proto::IxOneof::TransferCheckedWithFeeIx(
                        TransferCheckedWithFeeIxProto {
                            accounts: Some(data.accounts.into_proto_data()),
                            data: data.data.to_proto_option(),
                        },
                    )),
                },

                TransferFeeIx::WithdrawWithheldTokensFromMint(data) => TransferFeeIxProto {
                    ix_oneof: Some(
                        transfer_fee_ix_proto::IxOneof::WithdrawWithheldTokensFromMintIx(
                            WithdrawWithheldTokensFromMintIxProto {
                                accounts: Some(data.accounts.into_proto_data()),
                            },
                        ),
                    ),
                },

                TransferFeeIx::WithdrawWithheldTokensFromAccounts(data) => TransferFeeIxProto {
                    ix_oneof: Some(
                        transfer_fee_ix_proto::IxOneof::WithdrawWithheldTokensFromAccountsIx(
                            WithdrawWithheldTokensFromAccountsIxProto2 {
                                accounts: Some(data.accounts.into_proto_data()),
                                data: data.data.to_proto_option(),
                            },
                        ),
                    ),
                },

                TransferFeeIx::HarvestWithheldTokensToMint(data) => TransferFeeIxProto {
                    ix_oneof: Some(
                        transfer_fee_ix_proto::IxOneof::HarvestWithheldTokensToMintIx(
                            HarvestWithheldTokensToMintIxProto {
                                accounts: Some(data.accounts.into_proto_data()),
                            },
                        ),
                    ),
                },

                TransferFeeIx::SetTransferFee(data) => TransferFeeIxProto {
                    ix_oneof: Some(transfer_fee_ix_proto::IxOneof::SetTransferFeeIx(
                        SetTransferFeeIxProto {
                            accounts: Some(data.accounts.into_proto_data()),
                            data: data.data.to_proto_option(),
                        },
                    )),
                },
            }
        }
    }
}
