use spl_token::instruction::TokenInstruction as SplTokenInstruction;
use yellowstone_vixen_core::{
    instruction::InstructionUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};

#[derive(Debug, Clone, Copy)]
pub struct InstructionParser;

impl Parser for InstructionParser {
    type Input = InstructionUpdate;
    type Output = crate::TokenProgramInstruction;

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

    use crate::AuthorityTypeProto as P;

    match a {
        A::MintTokens => P::MintTokens as i32,
        A::FreezeAccount => P::FreezeAccount as i32,
        A::AccountOwner => P::AccountOwner as i32,
        A::CloseAccount => P::CloseAccount as i32,
    }
}

impl InstructionParser {
    #[allow(clippy::too_many_lines)]
    pub fn parse_impl(ix: &InstructionUpdate) -> Result<crate::TokenProgramInstruction> {
        let ix_type = SplTokenInstruction::unpack(&ix.data)
            .parse_err("Error unpacking token instruction data")?;

        let accounts_len = ix.accounts.len();

        use crate::token_program_instruction as oneof;

        let ix_msg = match ix_type {
            SplTokenInstruction::Transfer { amount } => {
                check_min_accounts_req(accounts_len, 3)?;

                oneof::Ix::Transfer(oneof::Transfer {
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

                oneof::Ix::InitializeAccount(oneof::InitializeAccount {
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

                oneof::Ix::InitializeMint(oneof::InitializeMint {
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

                oneof::Ix::InitializeAccount2(oneof::InitializeAccount2 {
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

                oneof::Ix::InitializeAccount3(oneof::InitializeAccount3 {
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

                oneof::Ix::InitializeMultisig(oneof::InitializeMultisig {
                    accounts: Some(crate::InitializeMultisigAccounts {
                        multisig: ix.accounts[0].to_vec(),
                        signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(crate::InitializeMultisigArgs { m: m as u32 }),
                })
            },

            SplTokenInstruction::InitializeMultisig2 { m } => {
                check_min_accounts_req(accounts_len, 2)?;

                oneof::Ix::InitializeMultisig(oneof::InitializeMultisig {
                    accounts: Some(crate::InitializeMultisigAccounts {
                        multisig: ix.accounts[0].to_vec(),
                        signers: ix.accounts[1..].iter().map(|pk| pk.to_vec()).collect(),
                    }),
                    args: Some(crate::InitializeMultisigArgs { m: m as u32 }),
                })
            },

            SplTokenInstruction::Approve { amount } => {
                check_min_accounts_req(accounts_len, 3)?;

                oneof::Ix::Approve(oneof::Approve {
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

                oneof::Ix::Revoke(oneof::Revoke {
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

                oneof::Ix::SetAuthority(oneof::SetAuthority {
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

                oneof::Ix::MintTo(oneof::MintTo {
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

                oneof::Ix::Burn(oneof::Burn {
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

                oneof::Ix::CloseAccount(oneof::CloseAccount {
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

                oneof::Ix::FreezeAccount(oneof::FreezeAccount {
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

                oneof::Ix::ThawAccount(oneof::ThawAccount {
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

                oneof::Ix::TransferChecked(oneof::TransferChecked {
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

                oneof::Ix::ApproveChecked(oneof::ApproveChecked {
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

                oneof::Ix::MintToChecked(oneof::MintToChecked {
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

                oneof::Ix::BurnChecked(oneof::BurnChecked {
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

                oneof::Ix::SyncNative(oneof::SyncNative {
                    accounts: Some(crate::SyncNativeAccounts {
                        account: ix.accounts[0].to_vec(),
                    }),
                })
            },

            SplTokenInstruction::GetAccountDataSize => {
                check_min_accounts_req(accounts_len, 1)?;

                oneof::Ix::GetAccountDataSize(oneof::GetAccountDataSize {
                    accounts: Some(crate::GetAccountDataSizeAccounts {
                        mint: ix.accounts[0].to_vec(),
                    }),
                })
            },

            SplTokenInstruction::InitializeImmutableOwner => {
                check_min_accounts_req(accounts_len, 1)?;

                oneof::Ix::InitializeImmutableOwner(oneof::InitializeImmutableOwner {
                    accounts: Some(crate::InitializeImmutableOwnerAccounts {
                        account: ix.accounts[0].to_vec(),
                    }),
                })
            },

            SplTokenInstruction::AmountToUiAmount { amount } => {
                check_min_accounts_req(accounts_len, 1)?;

                oneof::Ix::AmountToUiAmount(oneof::AmountToUiAmount {
                    accounts: Some(crate::AmountToUiAmountAccounts {
                        mint: ix.accounts[0].to_vec(),
                    }),
                    args: Some(crate::AmountToUiAmountArgs { amount }),
                })
            },

            SplTokenInstruction::UiAmountToAmount { ui_amount } => {
                check_min_accounts_req(accounts_len, 1)?;

                oneof::Ix::UiAmountToAmount(oneof::UiAmountToAmount {
                    accounts: Some(crate::UiAmountToAmountAccounts {
                        mint: ix.accounts[0].to_vec(),
                    }),
                    args: Some(crate::UiAmountToAmountArgs {
                        ui_amount: ui_amount.into(),
                    }),
                })
            },
        };

        Ok(crate::TokenProgramInstruction { ix: Some(ix_msg) })
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

        let Some(crate::token_program_instruction::Ix::MintToChecked(mtc)) =
            ix0.as_ref().and_then(|i| i.ix.as_ref())
        else {
            panic!("Invalid Instruction");
        };

        let args = mtc.args.as_ref().expect("missing args");

        assert_eq!(args.decimals, 10);
        assert_eq!(args.amount, 10.mul(10u64.pow(args.decimals)));
    }
}
