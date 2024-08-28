use spl_token::instruction::TokenInstruction;
use yellowstone_vixen_core::{
    Instruction, InstructionParser, InstructionsUpdate, ParseError, ParseResult, Parser, Prefilter,
    ReadableInstruction,
};

use super::ixs::*;
use crate::helpers::{
    check_min_accounts_req, check_pubkeys_match, get_multisig_signers, to_supported_coption_pubkey,
    to_supported_pubkey,
};

#[derive(Debug)]
pub struct TokenProgramIxParser;

impl Parser for TokenProgramIxParser {
    type Input = InstructionsUpdate;
    type Output = Vec<TokenProgramIx>;

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .transaction_accounts([spl_token::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, ixs_update: &InstructionsUpdate) -> ParseResult<Self::Output> {
        let mut parsed_ixs: Vec<TokenProgramIx> = Vec::new();
        for outer_ixs in ixs_update.instructions.iter() {
            if check_pubkeys_match(&outer_ixs.outer_ix.program_id, &spl_token::ID) {
                let parsed_ix = TokenProgramIxParser::parse_ix(&outer_ixs.outer_ix)
                    .map_err(|e| ParseError::Other(e.into()))?;
                parsed_ixs.push(parsed_ix);
            }
            for inner_ix in outer_ixs.inner_ixs.iter() {
                if check_pubkeys_match(&inner_ix.program_id, &spl_token::ID) {
                    let parsed_ix = TokenProgramIxParser::parse_ix(inner_ix)
                        .map_err(|e| ParseError::Other(e.into()))?;
                    parsed_ixs.push(parsed_ix);
                }
            }
        }
        if parsed_ixs.len() == 0 {
            return Err(ParseError::Other(
                "No token program instructions found to parse"
                    .to_string()
                    .into(),
            ));
        }
        Ok(parsed_ixs)
    }
}

impl InstructionParser<TokenProgramIx> for TokenProgramIxParser {
    fn parse_ix(ix: &Instruction) -> Result<TokenProgramIx, String> {
        let ix_type = TokenInstruction::unpack(&ix.data)
            .map_err(|e| format!("Err while unpacking ix data : {} data :{:?}", e, ix.data))?;
        let accounts_len = ix.accounts.len();
        match ix_type {
            TokenInstruction::Transfer { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::Transfer(ReadableInstruction {
                    accounts: TransferAccounts {
                        source: ix.accounts[0],
                        destination: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: get_multisig_signers(ix, 3),
                    },
                    data: Some(TransferData { amount }),
                }))
            },
            TokenInstruction::InitializeAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::InitializeAccount(ReadableInstruction {
                    accounts: InitializeAccountAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        owner: ix.accounts[2],
                    },
                    data: None,
                }))
            },
            TokenInstruction::InitializeMint {
                decimals,
                mint_authority,
                freeze_authority,
            } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::InitializeMint(ReadableInstruction {
                    accounts: InitializeMintAccounts {
                        mint: ix.accounts[0],
                    },
                    data: Some(InitializeMintData {
                        decimals,
                        mint_authority: to_supported_coption_pubkey(mint_authority.into()),
                        freeze_authority: to_supported_coption_pubkey(freeze_authority),
                    }),
                }))
            },
            TokenInstruction::InitializeMint2 {
                decimals,
                mint_authority,
                freeze_authority,
            } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::InitializeMint(ReadableInstruction {
                    accounts: InitializeMintAccounts {
                        mint: ix.accounts[0],
                    },
                    data: Some(InitializeMintData {
                        decimals,
                        mint_authority: to_supported_coption_pubkey(mint_authority.into()),
                        freeze_authority: to_supported_coption_pubkey(freeze_authority),
                    }),
                }))
            },

            TokenInstruction::InitializeAccount2 { owner } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramIx::InitializeAccount2(ReadableInstruction {
                    accounts: InitializeAccount2Accounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                    },
                    data: Some(InitializeAccountData2 {
                        owner: to_supported_pubkey(owner),
                    }),
                }))
            },

            TokenInstruction::InitializeAccount3 { owner } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramIx::InitializeAccount3(ReadableInstruction {
                    accounts: InitializeAccount2Accounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                    },
                    data: Some(InitializeAccountData2 {
                        owner: to_supported_pubkey(owner),
                    }),
                }))
            },
            TokenInstruction::InitializeMultisig { m } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::InitializeMultisig(ReadableInstruction {
                    accounts: InitializeMultisigAccounts {
                        multisig: ix.accounts[0],
                        signers: get_multisig_signers(ix, 2),
                    },
                    data: Some(InitializeMultisigData { m }),
                }))
            },

            TokenInstruction::InitializeMultisig2 { m } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramIx::InitializeMultisig(ReadableInstruction {
                    accounts: InitializeMultisigAccounts {
                        multisig: ix.accounts[0],
                        signers: get_multisig_signers(ix, 1),
                    },
                    data: Some(InitializeMultisigData { m }),
                }))
            },

            TokenInstruction::Approve { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::Approve(ReadableInstruction {
                    accounts: ApproveAccounts {
                        source: ix.accounts[0],
                        delegate: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: get_multisig_signers(ix, 3),
                    },
                    data: Some(ApproveData { amount }),
                }))
            },

            TokenInstruction::Revoke => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramIx::Revoke(ReadableInstruction {
                    accounts: RevokeAccounts {
                        source: ix.accounts[0],
                        owner: ix.accounts[1],
                        multisig_signers: get_multisig_signers(ix, 2),
                    },
                    data: None,
                }))
            },

            TokenInstruction::SetAuthority {
                authority_type,
                new_authority,
            } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramIx::SetAuthority(ReadableInstruction {
                    accounts: SetAuthorityAccounts {
                        account: ix.accounts[0],
                        current_authority: ix.accounts[1],
                        multisig_signers: get_multisig_signers(ix, 2),
                    },

                    data: Some(SetAuthorityData {
                        authority_type,
                        new_authority: to_supported_coption_pubkey(new_authority),
                    }),
                }))
            },

            TokenInstruction::MintTo { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::MintTo(ReadableInstruction {
                    accounts: MintToAccounts {
                        mint: ix.accounts[0],
                        account: ix.accounts[1],
                        mint_authority: ix.accounts[2],
                        multisig_signers: get_multisig_signers(ix, 3),
                    },
                    data: Some(MintToData { amount }),
                }))
            },

            TokenInstruction::Burn { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::Burn(ReadableInstruction {
                    accounts: BurnAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: get_multisig_signers(ix, 3),
                    },
                    data: Some(BurnData { amount }),
                }))
            },

            TokenInstruction::CloseAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::CloseAccount(ReadableInstruction {
                    accounts: CloseAccountAccounts {
                        account: ix.accounts[0],
                        destination: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: get_multisig_signers(ix, 3),
                    },
                    data: None,
                }))
            },

            TokenInstruction::FreezeAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::FreezeAccount(ReadableInstruction {
                    accounts: FreezeAccountAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        mint_freeze_authority: ix.accounts[2],
                        multisig_signers: get_multisig_signers(ix, 3),
                    },
                    data: None,
                }))
            },

            TokenInstruction::ThawAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::ThawAccount(ReadableInstruction {
                    accounts: ThawAccountAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        mint_freeze_authority: ix.accounts[2],
                        multisig_signers: get_multisig_signers(ix, 3),
                    },
                    data: None,
                }))
            },

            TokenInstruction::TransferChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(TokenProgramIx::TransferChecked(ReadableInstruction {
                    accounts: TransferCheckedAccounts {
                        source: ix.accounts[0],
                        mint: ix.accounts[1],
                        destination: ix.accounts[2],
                        owner: ix.accounts[3],
                        multisig_signers: get_multisig_signers(ix, 4),
                    },
                    data: Some(TransferCheckedData { amount, decimals }),
                }))
            },

            TokenInstruction::ApproveChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(TokenProgramIx::ApproveChecked(ReadableInstruction {
                    accounts: ApproveCheckedAccounts {
                        source: ix.accounts[0],
                        mint: ix.accounts[1],
                        delegate: ix.accounts[2],
                        owner: ix.accounts[3],
                        multisig_signers: get_multisig_signers(ix, 4),
                    },
                    data: Some(ApproveCheckedData { amount, decimals }),
                }))
            },

            TokenInstruction::MintToChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::MintToChecked(ReadableInstruction {
                    accounts: MintToCheckedAccounts {
                        mint: ix.accounts[0],
                        account: ix.accounts[1],
                        mint_authority: ix.accounts[2],
                        multisig_signers: get_multisig_signers(ix, 3),
                    },
                    data: Some(MintToCheckedData { amount, decimals }),
                }))
            },

            TokenInstruction::BurnChecked { amount, decimals } => {
                //TODO : this ix needs 3 accounts , but only 1 account is available in the instruction
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::BurnChecked(ReadableInstruction {
                    accounts: BurnCheckedAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: get_multisig_signers(ix, 3),
                    },
                    data: Some(BurnCheckedData { amount, decimals }),
                }))
            },

            TokenInstruction::SyncNative => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::SyncNative(ReadableInstruction {
                    accounts: SyncNativeAccounts {
                        account: ix.accounts[0],
                    },
                    data: None,
                }))
            },

            TokenInstruction::GetAccountDataSize => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::GetAccountDataSize(ReadableInstruction {
                    accounts: GetAccountDataSizeAccounts {
                        mint: ix.accounts[0],
                    },
                    data: None,
                }))
            },

            TokenInstruction::InitializeImmutableOwner => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::InitializeImmutableOwner(
                    ReadableInstruction {
                        accounts: InitializeImmutableOwnerAccounts {
                            account: ix.accounts[0],
                        },
                        data: None,
                    },
                ))
            },

            TokenInstruction::AmountToUiAmount { amount } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::AmountToUiAmount(ReadableInstruction {
                    accounts: AmountToUiAmountAccounts {
                        mint: ix.accounts[0],
                    },
                    data: Some(AmountToUiAmountData { amount }),
                }))
            },

            TokenInstruction::UiAmountToAmount { ui_amount } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::UiAmountToAmount(ReadableInstruction {
                    accounts: UiAmountToAmountAccounts {
                        mint: ix.accounts[0],
                    },
                    data: Some(UiAmountToAmountData {
                        ui_amount: ui_amount.into(),
                    }),
                }))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use yellowstone_vixen_mock::{run_tx_parse, tx_fixture, FixtureData};

    use super::*;

    #[tokio::test]
    async fn test_mint_to_checked_ix_parsing() {
        let parser = TokenProgramIxParser;

        let fixture_data = tx_fixture!("55kpnRufcX9Fo44oRBXtrkxPRww4UWJKxCpgBV39kzAAag8oyJbd9Y3YWdQQUi3TBqtrhjgsMGb9Nw8bUxy7j5rt");

        if let FixtureData::Instructions(ixs) = fixture_data {
            let ixs = run_tx_parse!(parser, ixs);

            if let TokenProgramIx::MintToChecked(ix) = &ixs[0] {
                assert!(ix.data.is_some());
                let data = ix.data.as_ref().unwrap();
                assert_eq!(data.decimals, 10);
                assert_eq!(data.amount, 10.mul(10u64.pow(data.decimals as u32)));
            } else {
                panic!("Invalid Instruction")
            }
        } else {
            panic!("Invalid Fixture Data")
        }
    }
}
