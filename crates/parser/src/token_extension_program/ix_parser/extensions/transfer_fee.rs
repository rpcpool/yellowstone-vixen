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

#[derive(Debug, Clone, Copy)]
pub struct HarvestWithheldTokensToMintAccounts {
    pub mint: Pubkey,
    pub mint_fee_acc_owner: Pubkey,
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
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TransferFeeIx::HarvestWithheldTokensToMint(
                    HarvestWithheldTokensToMintAccounts {
                        mint: ix.accounts[0],
                        mint_fee_acc_owner: ix.accounts[1],
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
    use yellowstone_vixen_proto::parser::{
        transfer_fee_ix_proto, HarvestWithheldTokensToMintAccountsProto,
        HarvestWithheldTokensToMintIxProto, InitializeTransferFeeConfigAccountsProto,
        InitializeTransferFeeConfigDataProto, InitializeTransferFeeConfigIxProto,
        SetTransferFeeAccountsProto, SetTransferFeeDataProto, SetTransferFeeIxProto,
        TransferCheckedWithFeeAccountsProto, TransferCheckedWithFeeDataProto,
        TransferCheckedWithFeeIxProto, TransferFeeIxProto,
        WithdrawWithheldTokensFromAccountsAccountsProto,
        WithdrawWithheldTokensFromAccountsDataProto, WithdrawWithheldTokensFromAccountsIxProto,
        WithdrawWithheldTokensFromMintAccountsProto, WithdrawWithheldTokensFromMintIxProto,
    };

    use super::{
        InitializeTransferFeeConfigAccounts, InitializeTransferFeeConfigData,
        SetTransferFeeAccounts, SetTransferFeeData, TransferCheckedWithFeeAccounts,
        TransferCheckedWithFeeData, TransferFeeIx, WithdrawWithheldTokensFromAccountsAccounts,
        WithdrawWithheldTokensFromAccountsData, WithdrawWithheldTokensFromMintAccounts,
    };
    use crate::{
        helpers::{FromOptPubkeyToOptString, FromVecPubkeyToVecString, IntoProto},
        token_extension_program::ix_parser::HarvestWithheldTokensToMintAccounts,
    };

    impl IntoProto<InitializeTransferFeeConfigAccountsProto> for InitializeTransferFeeConfigAccounts {
        fn into_proto(self) -> InitializeTransferFeeConfigAccountsProto {
            InitializeTransferFeeConfigAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProto<TransferCheckedWithFeeAccountsProto> for TransferCheckedWithFeeAccounts {
        fn into_proto(self) -> TransferCheckedWithFeeAccountsProto {
            TransferCheckedWithFeeAccountsProto {
                source: self.source.to_string(),
                mint: self.mint.to_string(),
                destination: self.destination.to_string(),
                owner: self.owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<WithdrawWithheldTokensFromMintAccountsProto>
        for WithdrawWithheldTokensFromMintAccounts
    {
        fn into_proto(self) -> WithdrawWithheldTokensFromMintAccountsProto {
            WithdrawWithheldTokensFromMintAccountsProto {
                mint: self.mint.to_string(),
                fee_recipient: self.fee_recipient.to_string(),
                withdraw_withheld_authority: self.withdraw_withheld_authority.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<WithdrawWithheldTokensFromAccountsAccountsProto>
        for WithdrawWithheldTokensFromAccountsAccounts
    {
        fn into_proto(self) -> WithdrawWithheldTokensFromAccountsAccountsProto {
            WithdrawWithheldTokensFromAccountsAccountsProto {
                mint: self.mint.to_string(),
                fee_recipient: self.fee_recipient.to_string(),
                withdraw_withheld_authority: self.withdraw_withheld_authority.to_string(),
                source_accounts: self.source_accounts.to_string_vec(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<SetTransferFeeAccountsProto> for SetTransferFeeAccounts {
        fn into_proto(self) -> SetTransferFeeAccountsProto {
            SetTransferFeeAccountsProto {
                mint: self.mint.to_string(),
                mint_fee_acc_owner: self.mint_fee_acc_owner.to_string(),
                multisig_signers: self.multisig_signers.to_string_vec(),
            }
        }
    }

    impl IntoProto<TransferCheckedWithFeeDataProto> for TransferCheckedWithFeeData {
        fn into_proto(self) -> TransferCheckedWithFeeDataProto {
            TransferCheckedWithFeeDataProto {
                amount: self.amount,
                fee_amount: self.fee_amount,
                decimals: self.decimals.into(),
            }
        }
    }

    impl IntoProto<InitializeTransferFeeConfigDataProto> for InitializeTransferFeeConfigData {
        fn into_proto(self) -> InitializeTransferFeeConfigDataProto {
            InitializeTransferFeeConfigDataProto {
                transfer_fee_config_authority: self.transfer_fee_config_authority.to_opt_string(),
                withdraw_withheld_authority: self.withdraw_withheld_authority.to_opt_string(),
                transfer_fee_basis_points: self.transfer_fee_basis_points.into(),
                maximum_fee: self.maximum_fee,
            }
        }
    }

    impl IntoProto<WithdrawWithheldTokensFromAccountsDataProto>
        for WithdrawWithheldTokensFromAccountsData
    {
        fn into_proto(self) -> WithdrawWithheldTokensFromAccountsDataProto {
            WithdrawWithheldTokensFromAccountsDataProto {
                num_token_accounts: self.num_token_accounts.into(),
            }
        }
    }

    impl IntoProto<SetTransferFeeDataProto> for SetTransferFeeData {
        fn into_proto(self) -> SetTransferFeeDataProto {
            SetTransferFeeDataProto {
                transfer_fee_basis_points: self.transfer_fee_basis_points.into(),
                maximum_fee: self.maximum_fee,
            }
        }
    }

    impl IntoProto<HarvestWithheldTokensToMintAccountsProto> for HarvestWithheldTokensToMintAccounts {
        fn into_proto(self) -> HarvestWithheldTokensToMintAccountsProto {
            HarvestWithheldTokensToMintAccountsProto {
                mint: self.mint.to_string(),
                mint_fee_owner: self.mint_fee_acc_owner.to_string(),
            }
        }
    }

    impl IntoProto<TransferFeeIxProto> for TransferFeeIx {
        fn into_proto(self) -> TransferFeeIxProto {
            match self {
                TransferFeeIx::InitializeTransferFeeConfig(acc, data) => TransferFeeIxProto {
                    ix_oneof: Some(
                        transfer_fee_ix_proto::IxOneof::InitializeTransferFeeConfigIx(
                            InitializeTransferFeeConfigIxProto {
                                accounts: Some(acc.into_proto()),
                                data: Some(data.into_proto()),
                            },
                        ),
                    ),
                },
                TransferFeeIx::TransferCheckedWithFee(acc, data) => TransferFeeIxProto {
                    ix_oneof: Some(transfer_fee_ix_proto::IxOneof::TransferCheckedWithFeeIx(
                        TransferCheckedWithFeeIxProto {
                            accounts: Some(acc.into_proto()),
                            data: Some(data.into_proto()),
                        },
                    )),
                },

                TransferFeeIx::WithdrawWithheldTokensFromMint(acc) => TransferFeeIxProto {
                    ix_oneof: Some(
                        transfer_fee_ix_proto::IxOneof::WithdrawWithheldTokensFromMintIx(
                            WithdrawWithheldTokensFromMintIxProto {
                                accounts: Some(acc.into_proto()),
                            },
                        ),
                    ),
                },

                TransferFeeIx::WithdrawWithheldTokensFromAccounts(acc, data) => {
                    TransferFeeIxProto {
                        ix_oneof: Some(
                            transfer_fee_ix_proto::IxOneof::WithdrawWithheldTokensFromAccountsIx(
                                WithdrawWithheldTokensFromAccountsIxProto {
                                    accounts: Some(acc.into_proto()),
                                    data: Some(data.into_proto()),
                                },
                            ),
                        ),
                    }
                },

                TransferFeeIx::HarvestWithheldTokensToMint(acc) => TransferFeeIxProto {
                    ix_oneof: Some(
                        transfer_fee_ix_proto::IxOneof::HarvestWithheldTokensToMintIx(
                            HarvestWithheldTokensToMintIxProto {
                                accounts: Some(acc.into_proto()),
                            },
                        ),
                    ),
                },

                TransferFeeIx::SetTransferFee(acc, data) => TransferFeeIxProto {
                    ix_oneof: Some(transfer_fee_ix_proto::IxOneof::SetTransferFeeIx(
                        SetTransferFeeIxProto {
                            accounts: Some(acc.into_proto()),
                            data: Some(data.into_proto()),
                        },
                    )),
                },
            }
        }
    }
}
