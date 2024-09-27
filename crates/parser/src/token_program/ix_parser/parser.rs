use spl_token::instruction::TokenInstruction;
use yellowstone_vixen_core::{
    instruction::InstructionUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};

#[allow(clippy::wildcard_imports)]
use super::ixs::*;
use crate::{
    helpers::{check_min_accounts_req, into_vixen_pubkey},
    Result, ResultExt,
};

#[derive(Debug, Clone, Copy)]
pub struct TokenProgramIxParser;

impl Parser for TokenProgramIxParser {
    type Input = InstructionUpdate;
    type Output = TokenProgramIx;

    fn id(&self) -> std::borrow::Cow<str> {
        "yellowstone_vixen_parser::token_program::TokenProgramIxParser".into()
    }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .transaction_accounts([spl_token::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, ix_update: &InstructionUpdate) -> ParseResult<Self::Output> {
        if ix_update.program.equals_ref(spl_token::ID) {
            TokenProgramIxParser::parse_impl(ix_update).map_err(|e| ParseError::Other(e.into()))
        } else {
            Err(ParseError::Filtered)
        }
    }
}

impl ProgramParser for TokenProgramIxParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { spl_token::ID.to_bytes().into() }
}

impl TokenProgramIxParser {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn parse_impl(ix: &InstructionUpdate) -> Result<TokenProgramIx> {
        let ix_type = TokenInstruction::unpack(&ix.data)
            .parse_err("Error unpacking token instruction data")?;
        let accounts_len = ix.accounts.len();
        match ix_type {
            TokenInstruction::Transfer { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::Transfer(
                    TransferAccounts {
                        source: ix.accounts[0],
                        destination: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                    TransferData { amount },
                ))
            },
            TokenInstruction::InitializeAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::InitializeAccount(
                    InitializeAccountAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        owner: ix.accounts[2],
                    },
                ))
            },
            TokenInstruction::InitializeMint {
                decimals,
                mint_authority,
                freeze_authority,
            } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::InitializeMint(
                    InitializeMintAccounts {
                        mint: ix.accounts[0],
                    },
                    InitializeMintData {
                        decimals,
                        mint_authority: into_vixen_pubkey(mint_authority),
                        freeze_authority: freeze_authority.map(into_vixen_pubkey).into(),
                    },
                ))
            },
            TokenInstruction::InitializeMint2 {
                decimals,
                mint_authority,
                freeze_authority,
            } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::InitializeMint(
                    InitializeMintAccounts {
                        mint: ix.accounts[0],
                    },
                    InitializeMintData {
                        decimals,
                        mint_authority: into_vixen_pubkey(mint_authority),
                        freeze_authority: freeze_authority.map(into_vixen_pubkey).into(),
                    },
                ))
            },

            TokenInstruction::InitializeAccount2 { owner } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramIx::InitializeAccount2(
                    InitializeAccount2Accounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                    },
                    InitializeAccountData2 {
                        owner: into_vixen_pubkey(owner),
                    },
                ))
            },

            TokenInstruction::InitializeAccount3 { owner } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramIx::InitializeAccount3(
                    InitializeAccount2Accounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                    },
                    InitializeAccountData2 {
                        owner: into_vixen_pubkey(owner),
                    },
                ))
            },
            TokenInstruction::InitializeMultisig { m } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::InitializeMultisig(
                    InitializeMultisigAccounts {
                        multisig: ix.accounts[0],
                        signers: ix.accounts[2..].to_vec(),
                    },
                    InitializeMultisigData { m },
                ))
            },

            TokenInstruction::InitializeMultisig2 { m } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramIx::InitializeMultisig(
                    InitializeMultisigAccounts {
                        multisig: ix.accounts[0],
                        signers: ix.accounts[1..].to_vec(),
                    },
                    InitializeMultisigData { m },
                ))
            },

            TokenInstruction::Approve { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::Approve(
                    ApproveAccounts {
                        source: ix.accounts[0],
                        delegate: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                    ApproveData { amount },
                ))
            },

            TokenInstruction::Revoke => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramIx::Revoke(RevokeAccounts {
                    source: ix.accounts[0],
                    owner: ix.accounts[1],
                    multisig_signers: ix.accounts[2..].to_vec(),
                }))
            },

            TokenInstruction::SetAuthority {
                authority_type,
                new_authority,
            } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramIx::SetAuthority(
                    SetAuthorityAccounts {
                        account: ix.accounts[0],
                        current_authority: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                    SetAuthorityData {
                        authority_type,
                        new_authority: new_authority.map(into_vixen_pubkey).into(),
                    },
                ))
            },

            TokenInstruction::MintTo { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::MintTo(
                    MintToAccounts {
                        mint: ix.accounts[0],
                        account: ix.accounts[1],
                        mint_authority: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                    MintToData { amount },
                ))
            },

            TokenInstruction::Burn { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::Burn(
                    BurnAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                    BurnData { amount },
                ))
            },

            TokenInstruction::CloseAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::CloseAccount(CloseAccountAccounts {
                    account: ix.accounts[0],
                    destination: ix.accounts[1],
                    owner: ix.accounts[2],
                    multisig_signers: ix.accounts[3..].to_vec(),
                }))
            },

            TokenInstruction::FreezeAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::FreezeAccount(FreezeAccountAccounts {
                    account: ix.accounts[0],
                    mint: ix.accounts[1],
                    mint_freeze_authority: ix.accounts[2],
                    multisig_signers: ix.accounts[3..].to_vec(),
                }))
            },

            TokenInstruction::ThawAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::ThawAccount(ThawAccountAccounts {
                    account: ix.accounts[0],
                    mint: ix.accounts[1],
                    mint_freeze_authority: ix.accounts[2],
                    multisig_signers: ix.accounts[3..].to_vec(),
                }))
            },

            TokenInstruction::TransferChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(TokenProgramIx::TransferChecked(
                    TransferCheckedAccounts {
                        source: ix.accounts[0],
                        mint: ix.accounts[1],
                        destination: ix.accounts[2],
                        owner: ix.accounts[3],
                        multisig_signers: ix.accounts[4..].to_vec(),
                    },
                    TransferCheckedData { amount, decimals },
                ))
            },

            TokenInstruction::ApproveChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(TokenProgramIx::ApproveChecked(
                    ApproveCheckedAccounts {
                        source: ix.accounts[0],
                        mint: ix.accounts[1],
                        delegate: ix.accounts[2],
                        owner: ix.accounts[3],
                        multisig_signers: ix.accounts[4..].to_vec(),
                    },
                    ApproveCheckedData { amount, decimals },
                ))
            },

            TokenInstruction::MintToChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::MintToChecked(
                    MintToCheckedAccounts {
                        mint: ix.accounts[0],
                        account: ix.accounts[1],
                        mint_authority: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                    MintToCheckedData { amount, decimals },
                ))
            },

            TokenInstruction::BurnChecked { amount, decimals } => {
                //TODO : this ix needs 3 accounts , but only 1 account is available in the instruction
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::BurnChecked(
                    BurnCheckedAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                    BurnCheckedData { amount, decimals },
                ))
            },

            TokenInstruction::SyncNative => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::SyncNative(SyncNativeAccounts {
                    account: ix.accounts[0],
                }))
            },

            TokenInstruction::GetAccountDataSize => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::GetAccountDataSize(
                    GetAccountDataSizeAccounts {
                        mint: ix.accounts[0],
                    },
                ))
            },

            TokenInstruction::InitializeImmutableOwner => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::InitializeImmutableOwner(
                    InitializeImmutableOwnerAccounts {
                        account: ix.accounts[0],
                    },
                ))
            },

            TokenInstruction::AmountToUiAmount { amount } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::AmountToUiAmount(
                    AmountToUiAmountAccounts {
                        mint: ix.accounts[0],
                    },
                    AmountToUiAmountData { amount },
                ))
            },

            TokenInstruction::UiAmountToAmount { ui_amount } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::UiAmountToAmount(
                    UiAmountToAmountAccounts {
                        mint: ix.accounts[0],
                    },
                    UiAmountToAmountData {
                        ui_amount: ui_amount.into(),
                    },
                ))
            },
        }
    }
}

#[cfg(feature = "proto")]
mod proto_parser {
    use yellowstone_vixen_core::proto::ParseProto;
    use yellowstone_vixen_proto::parser::TokenProgramIxProto;

    use super::TokenProgramIxParser;
    use crate::helpers::IntoProto;

    impl ParseProto for TokenProgramIxParser {
        type Message = TokenProgramIxProto;

        fn output_into_message(value: Self::Output) -> Self::Message { value.into_proto() }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use yellowstone_vixen_mock::{run_ix_parse, tx_fixture, FixtureData};

    use super::*;

    #[tokio::test]
    async fn test_mint_to_checked_ix_parsing() {
        let parser = TokenProgramIxParser;

        let ixs = tx_fixture!("55kpnRufcX9Fo44oRBXtrkxPRww4UWJKxCpgBV39kzAAag8oyJbd9Y3YWdQQUi3TBqtrhjgsMGb9Nw8bUxy7j5rt");
        let ix = run_ix_parse!(parser, &ixs[0]);

        let TokenProgramIx::MintToChecked(_accts, data) = ix else {
            panic!("Invalid Instruction");
        };

        assert_eq!(data.decimals, 10);
        assert_eq!(data.amount, 10.mul(10u64.pow(data.decimals.into())));
    }
}
