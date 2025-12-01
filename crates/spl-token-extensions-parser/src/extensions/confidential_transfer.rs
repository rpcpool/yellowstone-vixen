use spl_token_2022::extension::confidential_transfer::instruction::ConfidentialTransferInstruction as SplConfidentialTransferInstruction;
use yellowstone_vixen_core::{instruction::InstructionUpdate, Pubkey};
use yellowstone_vixen_parser::{check_min_accounts_req, Result};
use yellowstone_vixen_spl_token_parser::InitializeMintAccounts;

use crate::{decode_extension_ix_type, ExtensionInstructionParser};

#[derive(Debug, Clone, Copy)]
pub struct UpdateMintAccounts {
    pub mint: Pubkey,
    pub authority: Pubkey,
}

#[derive(Debug, Clone)]
pub struct ConfigureAccountAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub sysvar: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]
pub struct ApproveAccountAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub authority: Pubkey,
}

#[derive(Debug)]
pub struct EmptyAccountAccounts {
    pub account: Pubkey,
    pub sysvar: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]

pub struct DepositAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct WithdrawAccounts {
    pub source_account: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]

pub struct ConfidentialTransferAccounts {
    pub source_account: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub owner: Pubkey,
    pub context_account: Pubkey, // Sysvar account or context state account
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]
pub struct ApplyPendingBalanceAccounts {
    pub account: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug)]

pub struct CreditsAccounts {
    pub account: Pubkey,
    pub owner: Pubkey,
    pub multisig_signers: Vec<Pubkey>,
}

#[derive(Debug, Clone, Copy)]

pub struct TransferWithFeeAccounts {
    pub source_account: Pubkey,
    pub mint: Pubkey,
    pub destination: Pubkey,
}

#[derive(Debug, Clone, Copy)]

pub struct ConfigureAccountWithRegistryAccounts {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub registry: Pubkey,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum ConfidentialTransferInstruction {
    InitializeMint {
        accounts: InitializeMintAccounts,
    },
    UpdateMint {
        accounts: UpdateMintAccounts,
    },
    ConfigureAccount {
        accounts: ConfigureAccountAccounts,
    },
    ApproveAccount {
        accounts: ApproveAccountAccounts,
    },
    EmptyAccount {
        accounts: EmptyAccountAccounts,
    },
    Deposit {
        accounts: DepositAccounts,
    },
    Withdraw {
        accounts: WithdrawAccounts,
    },
    Transfer {
        accounts: ConfidentialTransferAccounts,
    },
    ApplyPendingBalance {
        accounts: ApplyPendingBalanceAccounts,
    },
    EnableConfidentialCredits {
        accounts: CreditsAccounts,
    },
    DisableConfidentialCredits {
        accounts: CreditsAccounts,
    },
    EnableNonConfidentialCredits {
        accounts: CreditsAccounts,
    },
    DisableNonConfidentialCredits {
        accounts: CreditsAccounts,
    },
    TransferWithFee {
        accounts: TransferWithFeeAccounts,
    },
    ConfigureAccountWithRegistry {
        accounts: ConfigureAccountWithRegistryAccounts,
    },
}

impl ExtensionInstructionParser for ConfidentialTransferInstruction {
    #[allow(clippy::too_many_lines)]
    fn try_parse(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();
        let ix_type: SplConfidentialTransferInstruction = decode_extension_ix_type(&ix.data[1..])?;
        match ix_type {
            SplConfidentialTransferInstruction::InitializeMint => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(ConfidentialTransferInstruction::InitializeMint {
                    accounts: InitializeMintAccounts {
                        mint: ix.accounts[0],
                    },
                })
            },
            SplConfidentialTransferInstruction::UpdateMint => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentialTransferInstruction::UpdateMint {
                    accounts: UpdateMintAccounts {
                        mint: ix.accounts[0],
                        authority: ix.accounts[1],
                    },
                })
            },
            SplConfidentialTransferInstruction::ConfigureAccount => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(ConfidentialTransferInstruction::ConfigureAccount {
                    accounts: ConfigureAccountAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        sysvar: ix.accounts[2],
                        owner: ix.accounts[3],
                        multisig_signers: ix.accounts[4..].to_vec(),
                    },
                })
            },

            SplConfidentialTransferInstruction::ApproveAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(ConfidentialTransferInstruction::ApproveAccount {
                    accounts: ApproveAccountAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        authority: ix.accounts[2],
                    },
                })
            },

            SplConfidentialTransferInstruction::EmptyAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(ConfidentialTransferInstruction::EmptyAccount {
                    accounts: EmptyAccountAccounts {
                        account: ix.accounts[0],
                        sysvar: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                })
            },

            SplConfidentialTransferInstruction::Deposit => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(ConfidentialTransferInstruction::Deposit {
                    accounts: DepositAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                })
            },

            SplConfidentialTransferInstruction::Withdraw => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(ConfidentialTransferInstruction::Withdraw {
                    accounts: WithdrawAccounts {
                        source_account: ix.accounts[0],
                        mint: ix.accounts[1],
                        destination: ix.accounts[2],
                        owner: ix.accounts[3],
                        multisig_signers: ix.accounts[4..].to_vec(),
                    },
                })
            },

            SplConfidentialTransferInstruction::Transfer => {
                check_min_accounts_req(accounts_len, 5)?;
                Ok(ConfidentialTransferInstruction::Transfer {
                    accounts: ConfidentialTransferAccounts {
                        source_account: ix.accounts[0],
                        mint: ix.accounts[1],
                        destination: ix.accounts[2],
                        context_account: ix.accounts[3],
                        owner: ix.accounts[4],
                        multisig_signers: ix.accounts[5..].to_vec(),
                    },
                })
            },

            SplConfidentialTransferInstruction::ApplyPendingBalance => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentialTransferInstruction::ApplyPendingBalance {
                    accounts: ApplyPendingBalanceAccounts {
                        account: ix.accounts[0],
                        owner: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                })
            },

            SplConfidentialTransferInstruction::EnableConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(ConfidentialTransferInstruction::EnableConfidentialCredits {
                    accounts: CreditsAccounts {
                        account: ix.accounts[0],
                        owner: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                })
            },

            SplConfidentialTransferInstruction::DisableConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(
                    ConfidentialTransferInstruction::DisableConfidentialCredits {
                        accounts: CreditsAccounts {
                            account: ix.accounts[0],
                            owner: ix.accounts[1],
                            multisig_signers: ix.accounts[2..].to_vec(),
                        },
                    },
                )
            },

            SplConfidentialTransferInstruction::EnableNonConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(
                    ConfidentialTransferInstruction::EnableNonConfidentialCredits {
                        accounts: CreditsAccounts {
                            account: ix.accounts[0],
                            owner: ix.accounts[1],
                            multisig_signers: ix.accounts[2..].to_vec(),
                        },
                    },
                )
            },

            SplConfidentialTransferInstruction::DisableNonConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(
                    ConfidentialTransferInstruction::DisableNonConfidentialCredits {
                        accounts: CreditsAccounts {
                            account: ix.accounts[0],
                            owner: ix.accounts[1],
                            multisig_signers: ix.accounts[2..].to_vec(),
                        },
                    },
                )
            },
            SplConfidentialTransferInstruction::TransferWithFee => {
                check_min_accounts_req(accounts_len, 5)?;
                Ok(ConfidentialTransferInstruction::TransferWithFee {
                    accounts: TransferWithFeeAccounts {
                        source_account: ix.accounts[0],
                        mint: ix.accounts[1],
                        destination: ix.accounts[2],
                    },
                })
            },
            SplConfidentialTransferInstruction::ConfigureAccountWithRegistry => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(
                    ConfidentialTransferInstruction::ConfigureAccountWithRegistry {
                        accounts: ConfigureAccountWithRegistryAccounts {
                            account: ix.accounts[0],
                            mint: ix.accounts[1],
                            registry: ix.accounts[2],
                        },
                    },
                )
            },
        }
    }
}
