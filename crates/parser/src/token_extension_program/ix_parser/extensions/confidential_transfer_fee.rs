use spl_token_2022::extension::confidential_transfer_fee::instruction::ConfidentialTransferFeeInstruction;
use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};

use super::helpers::{decode_extension_ix_type, ExtensionIxParser};
use crate::{helpers::check_min_accounts_req, Result};
#[derive(Debug, Clone, Copy)]
pub struct InitializeConfidentialTransferFeeConfigAccounts {
    pub mint: Pubkey,
}

#[derive(Debug)]
pub struct WithdrawWithheldTokensFromMintAccounts {
    pub mint: Pubkey,
    pub fee_recipient: Pubkey,
    pub sysvar: Pubkey,
    pub withdraw_withheld_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct WithdrawWithheldTokensFromAccounts {
    pub mint: Pubkey,
    pub fee_recipient: Pubkey,
    pub sysvar: Pubkey,
    pub withdraw_withheld_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
    pub source_accounts: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct HarvestWithheldTokensToMintAccounts {
    pub mint: Pubkey,
    pub source_accounts: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct EnableHarvestToMintAccounts {
    pub mint: Pubkey,
    pub confidential_transfer_fee_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct DisableHarvestToMintAccounts {
    pub account: Pubkey,
    pub confidential_transfer_fee_authority: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]
pub enum ConfidentaltransferFeeIx {
    InitializeConfidentialTransferFeeConfig(InitializeConfidentialTransferFeeConfigAccounts),
    WithdrawWithheldTokensFromMint(WithdrawWithheldTokensFromMint),
    WithdrawWithheldTokensFromAccounts(WithdrawWithheldTokensFromAccounts),
    HarvestWithheldTokensToMint(HarvestWithheldTokensToMint),
    EnableHarvestToMint(EnableHarvestToMint),
    DisableHarvestToMint(DisableHarvestToMint),
}

impl ExtensionIxParser for ConfidentaltransferFeeIx {
    fn try_parse_extension_ix(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();
        let ix_type = decode_extension_ix_type(&ix.data)?;

        match ix_type {
            ConfidentialTransferFeeInstruction::InitializeConfidentialTransferFeeConfig => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(
                    ConfidentaltransferFeeIx::InitializeConfidentialTransferFeeConfig(
                        InitializeConfidentialTransferFeeConfigAccounts {
                            mint: ix.accounts[0],
                        },
                    ),
                )
            },

            ConfidentialTransferFeeInstruction::WithdrawWithheldTokensFromMint => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(ConfidentaltransferFeeIx::WithdrawWithheldTokensFromMint(
                    WithdrawWithheldTokensFromMint {
                        mint: ix.accounts[0],
                        fee_recipient: ix.accounts[1],
                        sysvar: ix.accounts[2],
                        withdraw_withheld_authority: ix.accounts[3],
                        multisig_signers: ix.accounts[4..].to_vec(),
                    },
                ))
            },

            ConfidentialTransferFeeInstruction::WithdrawWithheldTokensFromAccounts => {
                check_min_accounts_req(accounts_len, 5)?;
                Ok(
                    ConfidentaltransferFeeIx::WithdrawWithheldTokensFromAccounts(
                        WithdrawWithheldTokensFromAccounts {
                            mint: ix.accounts[0],
                            fee_recipient: ix.accounts[1],
                            sysvar: ix.accounts[2],
                            withdraw_withheld_authority: ix.accounts[3],
                            source_accounts: ix.accounts[4..].to_vec(),
                            multisig_signers: Vec::new(),
                        },
                    ),
                )
            },

            ConfidentialTransferFeeInstruction::HarvestWithheldTokensToMint => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferFeeIx::HarvestWithheldTokensToMint(
                    HarvestWithheldTokensToMint {
                        mint: ix.accounts[0],
                        source_accounts: ix.accounts[1..].to_vec(),
                    },
                ))
            },

            ConfidentialTransferFeeInstruction::EnableHarvestToMint => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferFeeIx::EnableHarvestToMint(
                    EnableHarvestToMint {
                        mint: ix.accounts[0],
                        confidential_transfer_fee_authority: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                ))
            },

            ConfidentialTransferFeeInstruction::DisableHarvestToMint => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentaltransferFeeIx::DisableHarvestToMint(
                    DisableHarvestToMint {
                        account: ix.accounts[0],
                        confidential_transfer_fee_authority: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                ))
            },
        }
    }
}

#[cfg(feature = "proto")]
mod proto_parser {
    use confidential_transfer_fee_ix_proto::IxOneof;
    use yellowstone_vixen_proto::parser::*;

    use super::*;
    use crate::helpers::{FromOptVecToDefVec, FromVecPubkeyToVecString, IntoProtoData};

    impl IntoProtoData<InitializeConfidentialTransferFeeConfigAccountsProto>
        for InitializeConfidentialTransferFeeConfigAccounts
    {
        fn into_proto_data(self) -> InitializeConfidentialTransferFeeConfigAccountsProto {
            InitializeConfidentialTransferFeeConfigAccountsProto {
                mint: self.mint.to_string(),
            }
        }
    }

    impl IntoProtoData<WithdrawWithheldTokensFromMintAccountsProto>
        for WithdrawWithheldTokensFromMintAccounts
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

    impl IntoProtoData<WithdrawWithheldTokensFromAccountsAccountsProto>
        for WithdrawWithheldTokensFromAccounts
    {
        fn into_proto_data(self) -> WithdrawWithheldTokensFromAccountsAccountsProto {
            WithdrawWithheldTokensFromAccountsAccountsProto {
                mint: self.mint.to_string(),
                fee_recipient: self.fee_recipient.to_string(),
                withdraw_withheld_authority: self.withdraw_withheld_authority.to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
                source_accounts: self.source_accounts.to_string_vec(),
            }
        }
    }

    impl IntoProtoData<HarvestWithheldTokensToMintAccountsProto>
        for HarvestWithheldTokensToMintAccounts
    {
        fn into_proto_data(self) -> HarvestWithheldTokensToMintAccountsProto {
            HarvestWithheldTokensToMintAccountsProto {
                mint: self.mint.to_string(),
                source_accounts: self.source_accounts.to_string_vec(),
            }
        }
    }

    impl IntoProtoData<EnableHarvestToMintAccountsProto> for EnableHarvestToMintAccounts {
        fn into_proto_data(self) -> EnableHarvestToMintAccountsProto {
            EnableHarvestToMintAccountsProto {
                mint: self.mint.to_string(),
                confidential_transfer_fee_authority: self
                    .confidential_transfer_fee_authority
                    .to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<DisableHarvestToMintAccountsProto> for DisableHarvestToMintAccounts {
        fn into_proto_data(self) -> DisableHarvestToMintAccountsProto {
            DisableHarvestToMintAccountsProto {
                account: self.account.to_string(),
                confidential_transfer_fee_authority: self
                    .confidential_transfer_fee_authority
                    .to_string(),
                multisig_signers: self.multisig_signers.to_def_vec(),
            }
        }
    }

    impl IntoProtoData<ConfidentialTransferFeeIxProto> for ConfidentaltransferFeeIx {
        fn into_proto_data(self) -> ConfidentialTransferFeeIxProto {
            match self {
                ConfidentaltransferFeeIx::InitializeConfidentialTransferFeeConfig(ri) => {
                    ConfidentialTransferFeeIxProto {
                        ix_oneof: Some(IxOneof::InitializeConfidentialTransferFeeConfigIx(
                            InitializeConfidentialTransferFeeConfigIxProto {
                                accounts: Some(ri.accounts.into_proto_data()),
                            },
                        )),
                    }
                },

                ConfidentaltransferFeeIx::WithdrawWithheldTokensFromMint(ri) => {
                    ConfidentialTransferFeeIxProto {
                        ix_oneof: Some(IxOneof::WithdrawWithheldTokensFromMintIx(
                            WithdrawWithheldTokensFromMintIxProto {
                                accounts: Some(ri.accounts.into_proto_data()),
                            },
                        )),
                    }
                },

                ConfidentaltransferFeeIx::WithdrawWithheldTokensFromAccounts(ri) => {
                    ConfidentialTransferFeeIxProto {
                        ix_oneof: Some(IxOneof::WithdrawWithheldTokensFromAccountsIx(
                            WithdrawWithheldTokensFromAccountsIxProto {
                                accounts: Some(ri.accounts.into_proto_data()),
                            },
                        )),
                    }
                },

                ConfidentaltransferFeeIx::HarvestWithheldTokensToMint(ri) => {
                    ConfidentialTransferFeeIxProto {
                        ix_oneof: Some(IxOneof::HarvestWithheldTokensToMintIx(
                            HarvestWithheldTokensToMintIxProto {
                                accounts: Some(ri.accounts.into_proto_data()),
                            },
                        )),
                    }
                },

                ConfidentaltransferFeeIx::EnableHarvestToMint(ri) => {
                    ConfidentialTransferFeeIxProto {
                        ix_oneof: Some(IxOneof::EnableHarvestToMintIx(
                            EnableHarvestToMintIxProto {
                                accounts: Some(ri.accounts.into_proto_data()),
                            },
                        )),
                    }
                },

                ConfidentaltransferFeeIx::DisableHarvestToMint(ri) => {
                    ConfidentialTransferFeeIxProto {
                        ix_oneof: Some(IxOneof::DisableHarvestToMintIx(
                            DisableHarvestToMintIxProto {
                                accounts: Some(ri.accounts.into_proto_data()),
                            },
                        )),
                    }
                },
            }
        }
    }
}
