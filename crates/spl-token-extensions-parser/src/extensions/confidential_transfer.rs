use prost::alloc::vec::Vec;
use spl_token_2022::extension::confidential_transfer::instruction::ConfidentialTransferInstruction as SplConfidentialTransferInstruction;
use yellowstone_vixen_core::instruction::InstructionUpdate;
use yellowstone_vixen_parser::{check_min_accounts_req, Result};
use yellowstone_vixen_spl_token_parser::InitializeMintAccounts;

use crate::{decode_extension_ix_type, ExtensionInstructionParser, PubkeyBytes};

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMintAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub authority: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigureAccountAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub sysvar: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveAccountAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub authority: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyAccountAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub sysvar: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub source_account: PubkeyBytes,
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
pub struct ConfidentialTransferAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub source_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub destination: PubkeyBytes,
    #[prost(bytes = "vec", tag = "4")]
    pub context_account: PubkeyBytes, // Sysvar account or context state account
    #[prost(bytes = "vec", tag = "5")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "6")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyPendingBalanceAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditsAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub owner: PubkeyBytes,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub multisig_signers: Vec<PubkeyBytes>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferWithFeeAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub source_account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub destination: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigureAccountWithRegistryAccounts {
    #[prost(bytes = "vec", tag = "1")]
    pub account: PubkeyBytes,
    #[prost(bytes = "vec", tag = "2")]
    pub mint: PubkeyBytes,
    #[prost(bytes = "vec", tag = "3")]
    pub registry: PubkeyBytes,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfidentialTransferInstruction {
    #[prost(
        oneof = "confidential_transfer_instruction::Ix",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15"
    )]
    pub ix: Option<confidential_transfer_instruction::Ix>,
}

pub mod confidential_transfer_instruction {
    use super::*;

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitializeMint {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<InitializeMintAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateMint {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<UpdateMintAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConfigureAccount {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<ConfigureAccountAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApproveAccount {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<ApproveAccountAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EmptyAccount {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<EmptyAccountAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Deposit {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<DepositAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Withdraw {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<WithdrawAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Transfer {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<ConfidentialTransferAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApplyPendingBalance {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<ApplyPendingBalanceAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnableConfidentialCredits {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<CreditsAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisableConfidentialCredits {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<CreditsAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnableNonConfidentialCredits {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<CreditsAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisableNonConfidentialCredits {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<CreditsAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransferWithFee {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<TransferWithFeeAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConfigureAccountWithRegistry {
        #[prost(message, optional, tag = "1")]
        pub accounts: Option<ConfigureAccountWithRegistryAccounts>,
    }

    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ix {
        #[prost(message, tag = "1")]
        InitializeMint(InitializeMint),
        #[prost(message, tag = "2")]
        UpdateMint(UpdateMint),
        #[prost(message, tag = "3")]
        ConfigureAccount(ConfigureAccount),
        #[prost(message, tag = "4")]
        ApproveAccount(ApproveAccount),
        #[prost(message, tag = "5")]
        EmptyAccount(EmptyAccount),
        #[prost(message, tag = "6")]
        Deposit(Deposit),
        #[prost(message, tag = "7")]
        Withdraw(Withdraw),
        #[prost(message, tag = "8")]
        Transfer(Transfer),
        #[prost(message, tag = "9")]
        ApplyPendingBalance(ApplyPendingBalance),
        #[prost(message, tag = "10")]
        EnableConfidentialCredits(EnableConfidentialCredits),
        #[prost(message, tag = "11")]
        DisableConfidentialCredits(DisableConfidentialCredits),
        #[prost(message, tag = "12")]
        EnableNonConfidentialCredits(EnableNonConfidentialCredits),
        #[prost(message, tag = "13")]
        DisableNonConfidentialCredits(DisableNonConfidentialCredits),
        #[prost(message, tag = "14")]
        TransferWithFee(TransferWithFee),
        #[prost(message, tag = "15")]
        ConfigureAccountWithRegistry(ConfigureAccountWithRegistry),
    }
}

impl ExtensionInstructionParser for ConfidentialTransferInstruction {
    #[allow(clippy::too_many_lines)]
    fn try_parse(ix: &InstructionUpdate) -> Result<Self> {
        let accounts_len = ix.accounts.len();
        let ix_type: SplConfidentialTransferInstruction = decode_extension_ix_type(&ix.data[1..])?;

        use crate::confidential_transfer_instruction as oneof;

        let ix_msg = match ix_type {
            SplConfidentialTransferInstruction::InitializeMint => {
                check_min_accounts_req(accounts_len, 1)?;

                oneof::Ix::InitializeMint(oneof::InitializeMint {
                    accounts: Some(InitializeMintAccounts {
                        mint: ix.accounts[0].to_vec(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::UpdateMint => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Ix::UpdateMint(oneof::UpdateMint {
                    accounts: Some(UpdateMintAccounts {
                        mint: ix.accounts[0].to_vec(),
                        authority: ix.accounts[1].to_vec(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::ConfigureAccount => {
                check_min_accounts_req(accounts_len, 4)?;

                oneof::Ix::ConfigureAccount(oneof::ConfigureAccount {
                    accounts: Some(ConfigureAccountAccounts {
                        account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        sysvar: ix.accounts[2].to_vec(),
                        owner: ix.accounts[3].to_vec(),
                        multisig_signers: ix.accounts[4..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::ApproveAccount => {
                check_min_accounts_req(accounts_len, 3)?;

                oneof::Ix::ApproveAccount(oneof::ApproveAccount {
                    accounts: Some(ApproveAccountAccounts {
                        account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        authority: ix.accounts[2].to_vec(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::EmptyAccount => {
                check_min_accounts_req(accounts_len, 3)?;

                oneof::Ix::EmptyAccount(oneof::EmptyAccount {
                    accounts: Some(EmptyAccountAccounts {
                        account: ix.accounts[0].to_vec(),
                        sysvar: ix.accounts[1].to_vec(),
                        owner: ix.accounts[2].to_vec(),
                        multisig_signers: ix.accounts[3..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::Deposit => {
                check_min_accounts_req(accounts_len, 3)?;

                oneof::Ix::Deposit(oneof::Deposit {
                    accounts: Some(DepositAccounts {
                        account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        owner: ix.accounts[2].to_vec(),
                        multisig_signers: ix.accounts[3..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::Withdraw => {
                check_min_accounts_req(accounts_len, 4)?;

                oneof::Ix::Withdraw(oneof::Withdraw {
                    accounts: Some(WithdrawAccounts {
                        source_account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        destination: ix.accounts[2].to_vec(),
                        owner: ix.accounts[3].to_vec(),
                        multisig_signers: ix.accounts[4..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::Transfer => {
                check_min_accounts_req(accounts_len, 5)?;

                oneof::Ix::Transfer(oneof::Transfer {
                    accounts: Some(ConfidentialTransferAccounts {
                        source_account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        destination: ix.accounts[2].to_vec(),
                        context_account: ix.accounts[3].to_vec(),
                        owner: ix.accounts[4].to_vec(),
                        multisig_signers: ix.accounts[5..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::ApplyPendingBalance => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Ix::ApplyPendingBalance(oneof::ApplyPendingBalance {
                    accounts: Some(ApplyPendingBalanceAccounts {
                        account: ix.accounts[0].to_vec(),
                        owner: ix.accounts[1].to_vec(),
                        multisig_signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::EnableConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Ix::EnableConfidentialCredits(oneof::EnableConfidentialCredits {
                    accounts: Some(CreditsAccounts {
                        account: ix.accounts[0].to_vec(),
                        owner: ix.accounts[1].to_vec(),
                        multisig_signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::DisableConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Ix::DisableConfidentialCredits(oneof::DisableConfidentialCredits {
                    accounts: Some(CreditsAccounts {
                        account: ix.accounts[0].to_vec(),
                        owner: ix.accounts[1].to_vec(),
                        multisig_signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::EnableNonConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Ix::EnableNonConfidentialCredits(oneof::EnableNonConfidentialCredits {
                    accounts: Some(CreditsAccounts {
                        account: ix.accounts[0].to_vec(),
                        owner: ix.accounts[1].to_vec(),
                        multisig_signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::DisableNonConfidentialCredits => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Ix::DisableNonConfidentialCredits(oneof::DisableNonConfidentialCredits {
                    accounts: Some(CreditsAccounts {
                        account: ix.accounts[0].to_vec(),
                        owner: ix.accounts[1].to_vec(),
                        multisig_signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::TransferWithFee => {
                check_min_accounts_req(accounts_len, 5)?;

                oneof::Ix::TransferWithFee(oneof::TransferWithFee {
                    accounts: Some(TransferWithFeeAccounts {
                        source_account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        destination: ix.accounts[2].to_vec(),
                    }),
                })
            },

            SplConfidentialTransferInstruction::ConfigureAccountWithRegistry => {
                check_min_accounts_req(accounts_len, 4)?;

                oneof::Ix::ConfigureAccountWithRegistry(oneof::ConfigureAccountWithRegistry {
                    accounts: Some(ConfigureAccountWithRegistryAccounts {
                        account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        registry: ix.accounts[2].to_vec(),
                    }),
                })
            },
        };

        Ok(crate::ConfidentialTransferInstruction { ix: Some(ix_msg) })
    }
}
