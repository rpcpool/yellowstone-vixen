use spl_token::instruction::TokenInstruction as SplTokenInstruction;
use yellowstone_vixen_core::{
    instruction::InstructionUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};

use crate::PublicKey;

fn pk(key: &yellowstone_vixen_core::KeyBytes<32>) -> PublicKey { PublicKey::new(key.0) }

fn pk_from_key(key: &spl_token::solana_program::pubkey::Pubkey) -> PublicKey {
    PublicKey::new(key.to_bytes())
}

#[derive(Debug, Clone, Copy)]
pub struct InstructionParser;

impl Parser for InstructionParser {
    type Input = InstructionUpdate;
    type Output = crate::TokenProgram;

    fn id(&self) -> std::borrow::Cow<'static, str> { "token_program::InstructionParser".into() }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .transaction_accounts([spl_token::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, ix_update: &InstructionUpdate) -> ParseResult<Self::Output> {
        if ix_update.program.equals_ref(spl_token::ID) {
            InstructionParser::parse_impl(ix_update).map_err(|e| ParseError::Other(e.into()))
        } else {
            Err(ParseError::Filtered)
        }
    }
}

impl ProgramParser for InstructionParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::KeyBytes<32> { spl_token::ID.to_bytes().into() }
}

#[inline]
fn authority_type_to_proto(a: spl_token::instruction::AuthorityType) -> i32 {
    use spl_token::instruction::AuthorityType as A;

    use crate::AuthorityType as P;

    match a {
        A::MintTokens => P::MintTokens as i32,
        A::FreezeAccount => P::FreezeAccount as i32,
        A::AccountOwner => P::AccountOwner as i32,
        A::CloseAccount => P::CloseAccount as i32,
    }
}

impl InstructionParser {
    #[allow(clippy::too_many_lines)]
    pub fn parse_impl(ix: &InstructionUpdate) -> Result<crate::TokenProgram> {
        let ix_type = SplTokenInstruction::unpack(&ix.data)
            .parse_err("Error unpacking token instruction data")?;

        let accounts_len = ix.accounts.len();

        let ix_msg = match ix_type {
            SplTokenInstruction::Transfer { amount } => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::instruction::Instruction::Transfer(crate::instruction::Transfer {
                    accounts: Some(crate::TransferAccounts {
                        source: pk(&ix.accounts[0]),
                        destination: pk(&ix.accounts[1]),
                        owner: pk(&ix.accounts[2]),
                        multisig_signers: ix.accounts[3..]
                            .iter()
                            .map(|a| PublicKey::new(a.0))
                            .collect(),
                    }),
                    args: Some(crate::TransferArgs { amount }),
                })
            },

            SplTokenInstruction::InitializeAccount => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::instruction::Instruction::InitializeAccount(
                    crate::instruction::InitializeAccount {
                        accounts: Some(crate::InitializeAccountAccounts {
                            account: pk(&ix.accounts[0]),
                            mint: pk(&ix.accounts[1]),
                            owner: pk(&ix.accounts[2]),
                        }),
                    },
                )
            },

            SplTokenInstruction::InitializeMint {
                decimals,
                mint_authority,
                freeze_authority,
            }
            | SplTokenInstruction::InitializeMint2 {
                decimals,
                mint_authority,
                freeze_authority,
            } => {
                check_min_accounts_req(accounts_len, 1)?;

                crate::instruction::Instruction::InitializeMint(
                    crate::instruction::InitializeMint {
                        accounts: Some(crate::InitializeMintAccounts {
                            mint: pk(&ix.accounts[0]),
                        }),
                        args: Some(crate::InitializeMintArgs {
                            decimals: decimals as u32,
                            mint_authority: pk_from_key(&mint_authority),
                            freeze_authority: freeze_authority
                                .map(|k| PublicKey::new(k.to_bytes()))
                                .into(),
                        }),
                    },
                )
            },

            SplTokenInstruction::InitializeAccount2 { owner } => {
                check_min_accounts_req(accounts_len, 2)?;

                crate::instruction::Instruction::InitializeAccount2(
                    crate::instruction::InitializeAccount2 {
                        accounts: Some(crate::InitializeAccount2Accounts {
                            account: pk(&ix.accounts[0]),
                            mint: pk(&ix.accounts[1]),
                        }),
                        args: Some(crate::InitializeAccount2Args {
                            owner: pk_from_key(&owner),
                        }),
                    },
                )
            },

            SplTokenInstruction::InitializeAccount3 { owner } => {
                check_min_accounts_req(accounts_len, 2)?;

                crate::instruction::Instruction::InitializeAccount3(
                    crate::instruction::InitializeAccount3 {
                        accounts: Some(crate::InitializeAccount2Accounts {
                            account: pk(&ix.accounts[0]),
                            mint: pk(&ix.accounts[1]),
                        }),
                        args: Some(crate::InitializeAccount2Args {
                            owner: pk_from_key(&owner),
                        }),
                    },
                )
            },

            SplTokenInstruction::InitializeMultisig { m } => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::instruction::Instruction::InitializeMultisig(
                    crate::instruction::InitializeMultisig {
                        accounts: Some(crate::InitializeMultisigAccounts {
                            multisig: pk(&ix.accounts[0]),
                            signers: ix.accounts[2..]
                                .iter()
                                .map(|a| PublicKey::new(a.0))
                                .collect(),
                        }),
                        args: Some(crate::InitializeMultisigArgs { m: m as u32 }),
                    },
                )
            },

            SplTokenInstruction::InitializeMultisig2 { m } => {
                check_min_accounts_req(accounts_len, 2)?;

                crate::instruction::Instruction::InitializeMultisig(
                    crate::instruction::InitializeMultisig {
                        accounts: Some(crate::InitializeMultisigAccounts {
                            multisig: pk(&ix.accounts[0]),
                            signers: ix.accounts[1..]
                                .iter()
                                .map(|a| PublicKey::new(a.0))
                                .collect(),
                        }),
                        args: Some(crate::InitializeMultisigArgs { m: m as u32 }),
                    },
                )
            },

            SplTokenInstruction::Approve { amount } => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::instruction::Instruction::Approve(crate::instruction::Approve {
                    accounts: Some(crate::ApproveAccounts {
                        source: pk(&ix.accounts[0]),
                        delegate: pk(&ix.accounts[1]),
                        owner: pk(&ix.accounts[2]),
                        multisig_signers: ix.accounts[3..]
                            .iter()
                            .map(|a| PublicKey::new(a.0))
                            .collect(),
                    }),
                    args: Some(crate::ApproveArgs { amount }),
                })
            },

            SplTokenInstruction::Revoke => {
                check_min_accounts_req(accounts_len, 2)?;

                crate::instruction::Instruction::Revoke(crate::instruction::Revoke {
                    accounts: Some(crate::RevokeAccounts {
                        source: pk(&ix.accounts[0]),
                        owner: pk(&ix.accounts[1]),
                        multisig_signers: ix.accounts[2..]
                            .iter()
                            .map(|a| PublicKey::new(a.0))
                            .collect(),
                    }),
                })
            },

            SplTokenInstruction::SetAuthority {
                authority_type,
                new_authority,
            } => {
                check_min_accounts_req(accounts_len, 2)?;

                crate::instruction::Instruction::SetAuthority(crate::instruction::SetAuthority {
                    accounts: Some(crate::SetAuthorityAccounts {
                        account: pk(&ix.accounts[0]),
                        current_authority: pk(&ix.accounts[1]),
                        multisig_signers: ix.accounts[2..]
                            .iter()
                            .map(|a| PublicKey::new(a.0))
                            .collect(),
                    }),
                    args: Some(crate::SetAuthorityArgs {
                        authority_type: authority_type_to_proto(authority_type),
                        new_authority: new_authority.map(|k| PublicKey::new(k.to_bytes())).into(),
                    }),
                })
            },

            SplTokenInstruction::MintTo { amount } => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::instruction::Instruction::MintTo(crate::instruction::MintTo {
                    accounts: Some(crate::MintToAccounts {
                        mint: pk(&ix.accounts[0]),
                        account: pk(&ix.accounts[1]),
                        mint_authority: pk(&ix.accounts[2]),
                        multisig_signers: ix.accounts[3..]
                            .iter()
                            .map(|a| PublicKey::new(a.0))
                            .collect(),
                    }),
                    args: Some(crate::MintToArgs { amount }),
                })
            },

            SplTokenInstruction::Burn { amount } => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::instruction::Instruction::Burn(crate::instruction::Burn {
                    accounts: Some(crate::BurnAccounts {
                        account: pk(&ix.accounts[0]),
                        mint: pk(&ix.accounts[1]),
                        owner: pk(&ix.accounts[2]),
                        multisig_signers: ix.accounts[3..]
                            .iter()
                            .map(|a| PublicKey::new(a.0))
                            .collect(),
                    }),
                    args: Some(crate::BurnArgs { amount }),
                })
            },

            SplTokenInstruction::CloseAccount => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::instruction::Instruction::CloseAccount(crate::instruction::CloseAccount {
                    accounts: Some(crate::CloseAccountAccounts {
                        account: pk(&ix.accounts[0]),
                        destination: pk(&ix.accounts[1]),
                        owner: pk(&ix.accounts[2]),
                        multisig_signers: ix.accounts[3..]
                            .iter()
                            .map(|a| PublicKey::new(a.0))
                            .collect(),
                    }),
                })
            },

            SplTokenInstruction::FreezeAccount => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::instruction::Instruction::FreezeAccount(crate::instruction::FreezeAccount {
                    accounts: Some(crate::FreezeAccountAccounts {
                        account: pk(&ix.accounts[0]),
                        mint: pk(&ix.accounts[1]),
                        mint_freeze_authority: pk(&ix.accounts[2]),
                        multisig_signers: ix.accounts[3..]
                            .iter()
                            .map(|a| PublicKey::new(a.0))
                            .collect(),
                    }),
                })
            },

            SplTokenInstruction::ThawAccount => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::instruction::Instruction::ThawAccount(crate::instruction::ThawAccount {
                    accounts: Some(crate::ThawAccountAccounts {
                        account: pk(&ix.accounts[0]),
                        mint: pk(&ix.accounts[1]),
                        mint_freeze_authority: pk(&ix.accounts[2]),
                        multisig_signers: ix.accounts[3..]
                            .iter()
                            .map(|a| PublicKey::new(a.0))
                            .collect(),
                    }),
                })
            },

            SplTokenInstruction::TransferChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 4)?;

                crate::instruction::Instruction::TransferChecked(
                    crate::instruction::TransferChecked {
                        accounts: Some(crate::TransferCheckedAccounts {
                            source: pk(&ix.accounts[0]),
                            mint: pk(&ix.accounts[1]),
                            destination: pk(&ix.accounts[2]),
                            owner: pk(&ix.accounts[3]),
                            multisig_signers: ix.accounts[4..]
                                .iter()
                                .map(|a| PublicKey::new(a.0))
                                .collect(),
                        }),
                        args: Some(crate::TransferCheckedArgs {
                            amount,
                            decimals: decimals as u32,
                        }),
                    },
                )
            },

            SplTokenInstruction::ApproveChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 4)?;

                crate::instruction::Instruction::ApproveChecked(
                    crate::instruction::ApproveChecked {
                        accounts: Some(crate::ApproveCheckedAccounts {
                            source: pk(&ix.accounts[0]),
                            mint: pk(&ix.accounts[1]),
                            delegate: pk(&ix.accounts[2]),
                            owner: pk(&ix.accounts[3]),
                            multisig_signers: ix.accounts[4..]
                                .iter()
                                .map(|a| PublicKey::new(a.0))
                                .collect(),
                        }),
                        args: Some(crate::ApproveCheckedArgs {
                            amount,
                            decimals: decimals as u32,
                        }),
                    },
                )
            },

            SplTokenInstruction::MintToChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::instruction::Instruction::MintToChecked(crate::instruction::MintToChecked {
                    accounts: Some(crate::MintToCheckedAccounts {
                        mint: pk(&ix.accounts[0]),
                        account: pk(&ix.accounts[1]),
                        mint_authority: pk(&ix.accounts[2]),
                        multisig_signers: ix.accounts[3..]
                            .iter()
                            .map(|a| PublicKey::new(a.0))
                            .collect(),
                    }),
                    args: Some(crate::MintToCheckedArgs {
                        amount,
                        decimals: decimals as u32,
                    }),
                })
            },

            SplTokenInstruction::BurnChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::instruction::Instruction::BurnChecked(crate::instruction::BurnChecked {
                    accounts: Some(crate::BurnCheckedAccounts {
                        account: pk(&ix.accounts[0]),
                        mint: pk(&ix.accounts[1]),
                        owner: pk(&ix.accounts[2]),
                        multisig_signers: ix.accounts[3..]
                            .iter()
                            .map(|a| PublicKey::new(a.0))
                            .collect(),
                    }),
                    args: Some(crate::BurnCheckedArgs {
                        amount,
                        decimals: decimals as u32,
                    }),
                })
            },

            SplTokenInstruction::SyncNative => {
                check_min_accounts_req(accounts_len, 1)?;

                crate::instruction::Instruction::SyncNative(crate::instruction::SyncNative {
                    accounts: Some(crate::SyncNativeAccounts {
                        account: pk(&ix.accounts[0]),
                    }),
                })
            },

            SplTokenInstruction::GetAccountDataSize => {
                check_min_accounts_req(accounts_len, 1)?;

                crate::instruction::Instruction::GetAccountDataSize(
                    crate::instruction::GetAccountDataSize {
                        accounts: Some(crate::GetAccountDataSizeAccounts {
                            mint: pk(&ix.accounts[0]),
                        }),
                    },
                )
            },

            SplTokenInstruction::InitializeImmutableOwner => {
                check_min_accounts_req(accounts_len, 1)?;

                crate::instruction::Instruction::InitializeImmutableOwner(
                    crate::instruction::InitializeImmutableOwner {
                        accounts: Some(crate::InitializeImmutableOwnerAccounts {
                            account: pk(&ix.accounts[0]),
                        }),
                    },
                )
            },

            SplTokenInstruction::AmountToUiAmount { amount } => {
                check_min_accounts_req(accounts_len, 1)?;

                crate::instruction::Instruction::AmountToUiAmount(
                    crate::instruction::AmountToUiAmount {
                        accounts: Some(crate::AmountToUiAmountAccounts {
                            mint: pk(&ix.accounts[0]),
                        }),
                        args: Some(crate::AmountToUiAmountArgs { amount }),
                    },
                )
            },

            SplTokenInstruction::UiAmountToAmount { ui_amount } => {
                check_min_accounts_req(accounts_len, 1)?;

                crate::instruction::Instruction::UiAmountToAmount(
                    crate::instruction::UiAmountToAmount {
                        accounts: Some(crate::UiAmountToAmountAccounts {
                            mint: pk(&ix.accounts[0]),
                        }),
                        args: Some(crate::UiAmountToAmountArgs {
                            ui_amount: ui_amount.into(),
                        }),
                    },
                )
            },
        };

        Ok(crate::TokenProgram {
            instruction: Some(ix_msg),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use yellowstone_vixen_mock::tx_fixture;

    use super::*;

    #[tokio::test]
    async fn test_mint_to_checked_ix_parsing() {
        let parser = InstructionParser;

        let ixs =
            tx_fixture!("55kpnRufcX9Fo44oRBXtrkxPRww4UWJKxCpgBV39kzAAag8oyJbd9Y3YWdQQUi3TBqtrhjgsMGb9Nw8bUxy7j5rt", &parser);

        let ix0 = &ixs[0];

        let Some(crate::instruction::Instruction::MintToChecked(mtc)) =
            ix0.as_ref().and_then(|i| i.instruction.as_ref())
        else {
            panic!("Invalid Instruction");
        };

        let args = mtc.args.as_ref().expect("missing args");

        assert_eq!(args.decimals, 10);
        assert_eq!(args.amount, 10.mul(10u64.pow(args.decimals)));
    }
}
