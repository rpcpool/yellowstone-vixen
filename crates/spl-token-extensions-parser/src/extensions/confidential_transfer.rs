use spl_token_2022::extension::confidential_transfer::instruction::ConfidentialTransferInstruction as SplConfidentialTransferInstruction;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Result};
use yellowstone_vixen_proc_macro::vixen;
use yellowstone_vixen_spl_token_parser::InitializeMintAccounts;

use crate::{decode_extension_ix_type, ExtensionInstructionParser, PublicKey};

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpdateMintAccounts {
    pub mint: PublicKey,
    pub authority: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ConfigureAccountAccounts {
    pub account: PublicKey,
    pub mint: PublicKey,
    pub sysvar: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ApproveAccountAccounts {
    pub account: PublicKey,
    pub mint: PublicKey,
    pub authority: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct EmptyAccountAccounts {
    pub account: PublicKey,
    pub sysvar: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DepositAccounts {
    pub account: PublicKey,
    pub mint: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WithdrawAccounts {
    pub source_account: PublicKey,
    pub mint: PublicKey,
    pub destination: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ConfidentialTransferAccounts {
    pub source_account: PublicKey,
    pub mint: PublicKey,
    pub destination: PublicKey,
    pub context_account: PublicKey, // Sysvar account or context state account
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ApplyPendingBalanceAccounts {
    pub account: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct CreditsAccounts {
    pub account: PublicKey,
    pub owner: PublicKey,
    pub multisig_signers: Vec<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct TransferWithFeeAccounts {
    pub source_account: PublicKey,
    pub mint: PublicKey,
    pub destination: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ConfigureAccountWithRegistryAccounts {
    pub account: PublicKey,
    pub mint: PublicKey,
    pub registry: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ConfidentialTransferIx {
    #[hint(
        oneof = "confidential_transfer_instruction::Instruction",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15"
    )]
    pub instruction: Option<confidential_transfer_instruction::Instruction>,
}

pub mod confidential_transfer_instruction {
    use super::*;

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeMint {
        pub accounts: Option<InitializeMintAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct UpdateMint {
        pub accounts: Option<UpdateMintAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct ConfigureAccount {
        pub accounts: Option<ConfigureAccountAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct ApproveAccount {
        pub accounts: Option<ApproveAccountAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct EmptyAccount {
        pub accounts: Option<EmptyAccountAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Deposit {
        pub accounts: Option<DepositAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Withdraw {
        pub accounts: Option<WithdrawAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Transfer {
        pub accounts: Option<ConfidentialTransferAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct ApplyPendingBalance {
        pub accounts: Option<ApplyPendingBalanceAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct EnableConfidentialCredits {
        pub accounts: Option<CreditsAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct DisableConfidentialCredits {
        pub accounts: Option<CreditsAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct EnableNonConfidentialCredits {
        pub accounts: Option<CreditsAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct DisableNonConfidentialCredits {
        pub accounts: Option<CreditsAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct TransferWithFee {
        pub accounts: Option<TransferWithFeeAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct ConfigureAccountWithRegistry {
        pub accounts: Option<ConfigureAccountWithRegistryAccounts>,
    }

    #[vixen(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Instruction {
        InitializeMint(InitializeMint),
        UpdateMint(UpdateMint),
        ConfigureAccount(ConfigureAccount),
        ApproveAccount(ApproveAccount),
        EmptyAccount(EmptyAccount),
        Deposit(Deposit),
        Withdraw(Withdraw),
        Transfer(Transfer),
        ApplyPendingBalance(ApplyPendingBalance),
        EnableConfidentialCredits(EnableConfidentialCredits),
        DisableConfidentialCredits(DisableConfidentialCredits),
        EnableNonConfidentialCredits(EnableNonConfidentialCredits),
        DisableNonConfidentialCredits(DisableNonConfidentialCredits),
        TransferWithFee(TransferWithFee),
        ConfigureAccountWithRegistry(ConfigureAccountWithRegistry),
    }
}

impl ExtensionInstructionParser for ConfidentialTransferIx {
    #[allow(clippy::too_many_lines)]
    fn try_parse(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();
        let ix_type: SplConfidentialTransferInstruction = decode_extension_ix_type(&ix.data[1..])?;

        use crate::confidential_transfer_instruction as oneof;

        let ix_msg = match ix_type {
            SplConfidentialTransferInstruction::InitializeMint => {
                check_min_accounts_req(accounts_len, 1)?;

                oneof::Instruction::InitializeMint(oneof::InitializeMint {
                    accounts: Some(InitializeMintAccounts {
                        mint: crate::PublicKey::new(ix.accounts[0].to_vec()),
                    }),
                })
            },

            SplConfidentialTransferInstruction::UpdateMint => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::UpdateMint(oneof::UpdateMint {
                    accounts: Some(UpdateMintAccounts {
                        mint: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        authority: crate::PublicKey::new(ix.accounts[1].to_vec()),
                    }),
                })
            },

            SplConfidentialTransferInstruction::ConfigureAccount => {
                check_min_accounts_req(accounts_len, 4)?;

                oneof::Instruction::ConfigureAccount(oneof::ConfigureAccount {
                    accounts: Some(ConfigureAccountAccounts {
                        account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        mint: crate::PublicKey::new(ix.accounts[1].to_vec()),
                        sysvar: crate::PublicKey::new(ix.accounts[2].to_vec()),
                        owner: crate::PublicKey::new(ix.accounts[3].to_vec()),
                        multisig_signers: ix.accounts[4..]
                            .iter()
                            .map(|a| crate::PublicKey::new(a.to_vec()))
                            .collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::ApproveAccount => {
                check_min_accounts_req(accounts_len, 3)?;

                oneof::Instruction::ApproveAccount(oneof::ApproveAccount {
                    accounts: Some(ApproveAccountAccounts {
                        account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        mint: crate::PublicKey::new(ix.accounts[1].to_vec()),
                        authority: crate::PublicKey::new(ix.accounts[2].to_vec()),
                    }),
                })
            },

            SplConfidentialTransferInstruction::EmptyAccount => {
                check_min_accounts_req(accounts_len, 3)?;

                oneof::Instruction::EmptyAccount(oneof::EmptyAccount {
                    accounts: Some(EmptyAccountAccounts {
                        account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        sysvar: crate::PublicKey::new(ix.accounts[1].to_vec()),
                        owner: crate::PublicKey::new(ix.accounts[2].to_vec()),
                        multisig_signers: ix.accounts[3..]
                            .iter()
                            .map(|a| crate::PublicKey::new(a.to_vec()))
                            .collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::Deposit => {
                check_min_accounts_req(accounts_len, 3)?;

                oneof::Instruction::Deposit(oneof::Deposit {
                    accounts: Some(DepositAccounts {
                        account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        mint: crate::PublicKey::new(ix.accounts[1].to_vec()),
                        owner: crate::PublicKey::new(ix.accounts[2].to_vec()),
                        multisig_signers: ix.accounts[3..]
                            .iter()
                            .map(|a| crate::PublicKey::new(a.to_vec()))
                            .collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::Withdraw => {
                check_min_accounts_req(accounts_len, 4)?;

                oneof::Instruction::Withdraw(oneof::Withdraw {
                    accounts: Some(WithdrawAccounts {
                        source_account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        mint: crate::PublicKey::new(ix.accounts[1].to_vec()),
                        destination: crate::PublicKey::new(ix.accounts[2].to_vec()),
                        owner: crate::PublicKey::new(ix.accounts[3].to_vec()),
                        multisig_signers: ix.accounts[4..]
                            .iter()
                            .map(|a| crate::PublicKey::new(a.to_vec()))
                            .collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::Transfer => {
                check_min_accounts_req(accounts_len, 5)?;

                oneof::Instruction::Transfer(oneof::Transfer {
                    accounts: Some(ConfidentialTransferAccounts {
                        source_account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        mint: crate::PublicKey::new(ix.accounts[1].to_vec()),
                        destination: crate::PublicKey::new(ix.accounts[2].to_vec()),
                        context_account: crate::PublicKey::new(ix.accounts[3].to_vec()),
                        owner: crate::PublicKey::new(ix.accounts[4].to_vec()),
                        multisig_signers: ix.accounts[5..]
                            .iter()
                            .map(|a| crate::PublicKey::new(a.to_vec()))
                            .collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::ApplyPendingBalance => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::ApplyPendingBalance(oneof::ApplyPendingBalance {
                    accounts: Some(ApplyPendingBalanceAccounts {
                        account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        owner: crate::PublicKey::new(ix.accounts[1].to_vec()),
                        multisig_signers: ix.accounts[2..]
                            .iter()
                            .map(|a| crate::PublicKey::new(a.to_vec()))
                            .collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::EnableConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::EnableConfidentialCredits(oneof::EnableConfidentialCredits {
                    accounts: Some(CreditsAccounts {
                        account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        owner: crate::PublicKey::new(ix.accounts[1].to_vec()),
                        multisig_signers: ix.accounts[2..]
                            .iter()
                            .map(|a| crate::PublicKey::new(a.to_vec()))
                            .collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::DisableConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::DisableConfidentialCredits(oneof::DisableConfidentialCredits {
                    accounts: Some(CreditsAccounts {
                        account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        owner: crate::PublicKey::new(ix.accounts[1].to_vec()),
                        multisig_signers: ix.accounts[2..]
                            .iter()
                            .map(|a| crate::PublicKey::new(a.to_vec()))
                            .collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::EnableNonConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::EnableNonConfidentialCredits(
                    oneof::EnableNonConfidentialCredits {
                        accounts: Some(CreditsAccounts {
                            account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                            owner: crate::PublicKey::new(ix.accounts[1].to_vec()),
                            multisig_signers: ix.accounts[2..]
                                .iter()
                                .map(|a| crate::PublicKey::new(a.to_vec()))
                                .collect(),
                        }),
                    },
                )
            },

            SplConfidentialTransferInstruction::DisableNonConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Instruction::DisableNonConfidentialCredits(
                    oneof::DisableNonConfidentialCredits {
                        accounts: Some(CreditsAccounts {
                            account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                            owner: crate::PublicKey::new(ix.accounts[1].to_vec()),
                            multisig_signers: ix.accounts[2..]
                                .iter()
                                .map(|a| crate::PublicKey::new(a.to_vec()))
                                .collect(),
                        }),
                    },
                )
            },

            SplConfidentialTransferInstruction::TransferWithFee => {
                check_min_accounts_req(accounts_len, 5)?;

                oneof::Instruction::TransferWithFee(oneof::TransferWithFee {
                    accounts: Some(TransferWithFeeAccounts {
                        source_account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                        mint: crate::PublicKey::new(ix.accounts[1].to_vec()),
                        destination: crate::PublicKey::new(ix.accounts[2].to_vec()),
                    }),
                })
            },

            SplConfidentialTransferInstruction::ConfigureAccountWithRegistry => {
                check_min_accounts_req(accounts_len, 4)?;

                oneof::Instruction::ConfigureAccountWithRegistry(
                    oneof::ConfigureAccountWithRegistry {
                        accounts: Some(ConfigureAccountWithRegistryAccounts {
                            account: crate::PublicKey::new(ix.accounts[0].to_vec()),
                            mint: crate::PublicKey::new(ix.accounts[1].to_vec()),
                            registry: crate::PublicKey::new(ix.accounts[2].to_vec()),
                        }),
                    },
                )
            },
        };

        Ok(crate::ConfidentialTransferIx {
            instruction: Some(ix_msg),
        })
    }
}
