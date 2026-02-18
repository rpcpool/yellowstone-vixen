use spl_token::instruction::TokenInstruction as SplTokenInstruction;
use yellowstone_vixen_core::{
    instruction::InstructionUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};

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
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { spl_token::ID.to_bytes().into() }
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

                crate::Instruction::Transfer(crate::TransferInstruction {
                    accounts: Some(crate::TransferAccounts {
                        source: ix.accounts[0].to_vec(),
                        destination: ix.accounts[1].to_vec(),
                        owner: ix.accounts[2].to_vec(),
                        multisig_signers: ix.accounts[3..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(crate::TransferArgs { amount }),
                })
            },

            SplTokenInstruction::InitializeAccount => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::Instruction::InitializeAccount(crate::InitializeAccountInstruction {
                    accounts: Some(crate::InitializeAccountAccounts {
                        account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        owner: ix.accounts[2].to_vec(),
                    }),
                })
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

                crate::Instruction::InitializeMint(crate::InitializeMintInstruction {
                    accounts: Some(crate::InitializeMintAccounts {
                        mint: ix.accounts[0].to_vec(),
                    }),
                    args: Some(crate::InitializeMintArgs {
                        decimals: decimals as u32,
                        mint_authority: mint_authority.to_bytes().to_vec(),
                        freeze_authority: freeze_authority.map(|pk| pk.to_bytes().to_vec()).into(),
                    }),
                })
            },

            SplTokenInstruction::InitializeAccount2 { owner } => {
                check_min_accounts_req(accounts_len, 2)?;

                crate::Instruction::InitializeAccount2(crate::InitializeAccount2Instruction {
                    accounts: Some(crate::InitializeAccount2Accounts {
                        account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                    }),
                    args: Some(crate::InitializeAccount2Args {
                        owner: owner.to_bytes().to_vec(),
                    }),
                })
            },

            SplTokenInstruction::InitializeAccount3 { owner } => {
                check_min_accounts_req(accounts_len, 2)?;

                crate::Instruction::InitializeAccount3(crate::InitializeAccount3Instruction {
                    accounts: Some(crate::InitializeAccount2Accounts {
                        account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                    }),
                    args: Some(crate::InitializeAccount2Args {
                        owner: owner.to_bytes().to_vec(),
                    }),
                })
            },

            SplTokenInstruction::InitializeMultisig { m } => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::Instruction::InitializeMultisig(crate::InitializeMultisigInstruction {
                    accounts: Some(crate::InitializeMultisigAccounts {
                        multisig: ix.accounts[0].to_vec(),
                        signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(crate::InitializeMultisigArgs { m: m as u32 }),
                })
            },

            SplTokenInstruction::InitializeMultisig2 { m } => {
                check_min_accounts_req(accounts_len, 2)?;

                crate::Instruction::InitializeMultisig(crate::InitializeMultisigInstruction {
                    accounts: Some(crate::InitializeMultisigAccounts {
                        multisig: ix.accounts[0].to_vec(),
                        signers: ix.accounts[1..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(crate::InitializeMultisigArgs { m: m as u32 }),
                })
            },

            SplTokenInstruction::Approve { amount } => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::Instruction::Approve(crate::ApproveInstruction {
                    accounts: Some(crate::ApproveAccounts {
                        source: ix.accounts[0].to_vec(),
                        delegate: ix.accounts[1].to_vec(),
                        owner: ix.accounts[2].to_vec(),
                        multisig_signers: ix.accounts[3..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(crate::ApproveArgs { amount }),
                })
            },

            SplTokenInstruction::Revoke => {
                check_min_accounts_req(accounts_len, 2)?;

                crate::Instruction::Revoke(crate::RevokeInstruction {
                    accounts: Some(crate::RevokeAccounts {
                        source: ix.accounts[0].to_vec(),
                        owner: ix.accounts[1].to_vec(),
                        multisig_signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplTokenInstruction::SetAuthority {
                authority_type,
                new_authority,
            } => {
                check_min_accounts_req(accounts_len, 2)?;

                crate::Instruction::SetAuthority(crate::SetAuthorityInstruction {
                    accounts: Some(crate::SetAuthorityAccounts {
                        account: ix.accounts[0].to_vec(),
                        current_authority: ix.accounts[1].to_vec(),
                        multisig_signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(crate::SetAuthorityArgs {
                        authority_type: authority_type_to_proto(authority_type),
                        new_authority: new_authority.map(|pk| pk.to_bytes().to_vec()).into(),
                    }),
                })
            },

            SplTokenInstruction::MintTo { amount } => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::Instruction::MintTo(crate::MintToInstruction {
                    accounts: Some(crate::MintToAccounts {
                        mint: ix.accounts[0].to_vec(),
                        account: ix.accounts[1].to_vec(),
                        mint_authority: ix.accounts[2].to_vec(),
                        multisig_signers: ix.accounts[3..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(crate::MintToArgs { amount }),
                })
            },

            SplTokenInstruction::Burn { amount } => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::Instruction::Burn(crate::BurnInstruction {
                    accounts: Some(crate::BurnAccounts {
                        account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        owner: ix.accounts[2].to_vec(),
                        multisig_signers: ix.accounts[3..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(crate::BurnArgs { amount }),
                })
            },

            SplTokenInstruction::CloseAccount => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::Instruction::CloseAccount(crate::CloseAccountInstruction {
                    accounts: Some(crate::CloseAccountAccounts {
                        account: ix.accounts[0].to_vec(),
                        destination: ix.accounts[1].to_vec(),
                        owner: ix.accounts[2].to_vec(),
                        multisig_signers: ix.accounts[3..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplTokenInstruction::FreezeAccount => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::Instruction::FreezeAccount(crate::FreezeAccountInstruction {
                    accounts: Some(crate::FreezeAccountAccounts {
                        account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        mint_freeze_authority: ix.accounts[2].to_vec(),
                        multisig_signers: ix.accounts[3..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplTokenInstruction::ThawAccount => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::Instruction::ThawAccount(crate::ThawAccountInstruction {
                    accounts: Some(crate::ThawAccountAccounts {
                        account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        mint_freeze_authority: ix.accounts[2].to_vec(),
                        multisig_signers: ix.accounts[3..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                })
            },

            SplTokenInstruction::TransferChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 4)?;

                crate::Instruction::TransferChecked(crate::TransferCheckedInstruction {
                    accounts: Some(crate::TransferCheckedAccounts {
                        source: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        destination: ix.accounts[2].to_vec(),
                        owner: ix.accounts[3].to_vec(),
                        multisig_signers: ix.accounts[4..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(crate::TransferCheckedArgs {
                        amount,
                        decimals: decimals as u32,
                    }),
                })
            },

            SplTokenInstruction::ApproveChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 4)?;

                crate::Instruction::ApproveChecked(crate::ApproveCheckedInstruction {
                    accounts: Some(crate::ApproveCheckedAccounts {
                        source: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        delegate: ix.accounts[2].to_vec(),
                        owner: ix.accounts[3].to_vec(),
                        multisig_signers: ix.accounts[4..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(crate::ApproveCheckedArgs {
                        amount,
                        decimals: decimals as u32,
                    }),
                })
            },

            SplTokenInstruction::MintToChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::Instruction::MintToChecked(crate::MintToCheckedInstruction {
                    accounts: Some(crate::MintToCheckedAccounts {
                        mint: ix.accounts[0].to_vec(),
                        account: ix.accounts[1].to_vec(),
                        mint_authority: ix.accounts[2].to_vec(),
                        multisig_signers: ix.accounts[3..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(crate::MintToCheckedArgs {
                        amount,
                        decimals: decimals as u32,
                    }),
                })
            },

            SplTokenInstruction::BurnChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 3)?;

                crate::Instruction::BurnChecked(crate::BurnCheckedInstruction {
                    accounts: Some(crate::BurnCheckedAccounts {
                        account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                        owner: ix.accounts[2].to_vec(),
                        multisig_signers: ix.accounts[3..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(crate::BurnCheckedArgs {
                        amount,
                        decimals: decimals as u32,
                    }),
                })
            },

            SplTokenInstruction::SyncNative => {
                check_min_accounts_req(accounts_len, 1)?;

                crate::Instruction::SyncNative(crate::SyncNativeInstruction {
                    accounts: Some(crate::SyncNativeAccounts {
                        account: ix.accounts[0].to_vec(),
                    }),
                })
            },

            SplTokenInstruction::GetAccountDataSize => {
                check_min_accounts_req(accounts_len, 1)?;

                crate::Instruction::GetAccountDataSize(crate::GetAccountDataSizeInstruction {
                    accounts: Some(crate::GetAccountDataSizeAccounts {
                        mint: ix.accounts[0].to_vec(),
                    }),
                })
            },

            SplTokenInstruction::InitializeImmutableOwner => {
                check_min_accounts_req(accounts_len, 1)?;

                crate::Instruction::InitializeImmutableOwner(crate::InitializeImmutableOwnerInstruction {
                    accounts: Some(crate::InitializeImmutableOwnerAccounts {
                        account: ix.accounts[0].to_vec(),
                    }),
                })
            },

            SplTokenInstruction::AmountToUiAmount { amount } => {
                check_min_accounts_req(accounts_len, 1)?;

                crate::Instruction::AmountToUiAmount(crate::AmountToUiAmountInstruction {
                    accounts: Some(crate::AmountToUiAmountAccounts {
                        mint: ix.accounts[0].to_vec(),
                    }),
                    args: Some(crate::AmountToUiAmountArgs { amount }),
                })
            },

            SplTokenInstruction::UiAmountToAmount { ui_amount } => {
                check_min_accounts_req(accounts_len, 1)?;

                crate::Instruction::UiAmountToAmount(crate::UiAmountToAmountInstruction {
                    accounts: Some(crate::UiAmountToAmountAccounts {
                        mint: ix.accounts[0].to_vec(),
                    }),
                    args: Some(crate::UiAmountToAmountArgs {
                        ui_amount: ui_amount.into(),
                    }),
                })
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

        let Some(crate::Instruction::MintToChecked(mtc)) =
            ix0.as_ref().and_then(|i| i.instruction.as_ref())
        else {
            panic!("Invalid Instruction");
        };

        let args = mtc.args.as_ref().expect("missing args");

        assert_eq!(args.decimals, 10);
        assert_eq!(args.amount, 10.mul(10u64.pow(args.decimals)));
    }
}
