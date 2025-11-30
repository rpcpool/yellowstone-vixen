use spl_token::instruction::TokenInstruction as SplTokenInstruction;
use yellowstone_vixen_core::{
    instruction::InstructionUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};
use yellowstone_vixen_parser::{check_min_accounts_req, Result, ResultExt};

#[allow(clippy::wildcard_imports)]
use crate::instructions::*;

#[derive(Debug, Clone, Copy)]
pub struct InstructionParser;

impl Parser for InstructionParser {
    type Input = InstructionUpdate;
    type Output = TokenProgramInstruction;

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

impl InstructionParser {
    #[allow(clippy::too_many_lines)]
    pub fn parse_impl(ix: &InstructionUpdate) -> Result<TokenProgramInstruction> {
        let ix_type = SplTokenInstruction::unpack(&ix.data)
            .parse_err("Error unpacking token instruction data")?;
        let accounts_len = ix.accounts.len();
        match ix_type {
            SplTokenInstruction::Transfer { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramInstruction::Transfer {
                    accounts: TransferAccounts {
                        source: ix.accounts[0],
                        destination: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                    args: TransferArgs { amount },
                })
            },
            SplTokenInstruction::InitializeAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramInstruction::InitializeAccount {
                    accounts: InitializeAccountAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        owner: ix.accounts[2],
                    },
                })
            },
            SplTokenInstruction::InitializeMint {
                decimals,
                mint_authority,
                freeze_authority,
            } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramInstruction::InitializeMint {
                    accounts: InitializeMintAccounts {
                        mint: ix.accounts[0],
                    },
                    args: InitializeMintArgs {
                        decimals,
                        mint_authority: mint_authority.to_bytes().into(),
                        freeze_authority: freeze_authority.map(|p| p.to_bytes().into()).into(),
                    },
                })
            },
            SplTokenInstruction::InitializeMint2 {
                decimals,
                mint_authority,
                freeze_authority,
            } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramInstruction::InitializeMint {
                    accounts: InitializeMintAccounts {
                        mint: ix.accounts[0],
                    },
                    args: InitializeMintArgs {
                        decimals,
                        mint_authority: mint_authority.to_bytes().into(),
                        freeze_authority: freeze_authority.map(|p| p.to_bytes().into()).into(),
                    },
                })
            },
            SplTokenInstruction::InitializeAccount2 { owner } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramInstruction::InitializeAccount2 {
                    accounts: InitializeAccount2Accounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                    },
                    args: InitializeAccount2Args {
                        owner: owner.to_bytes().into(),
                    },
                })
            },
            SplTokenInstruction::InitializeAccount3 { owner } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramInstruction::InitializeAccount3 {
                    accounts: InitializeAccount2Accounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                    },
                    args: InitializeAccount2Args {
                        owner: owner.to_bytes().into(),
                    },
                })
            },
            SplTokenInstruction::InitializeMultisig { m } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramInstruction::InitializeMultisig {
                    accounts: InitializeMultisigAccounts {
                        multisig: ix.accounts[0],
                        signers: ix.accounts[2..].to_vec(),
                    },
                    args: InitializeMultisigArgs { m },
                })
            },
            SplTokenInstruction::InitializeMultisig2 { m } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramInstruction::InitializeMultisig {
                    accounts: InitializeMultisigAccounts {
                        multisig: ix.accounts[0],
                        signers: ix.accounts[1..].to_vec(),
                    },
                    args: InitializeMultisigArgs { m },
                })
            },
            SplTokenInstruction::Approve { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramInstruction::Approve {
                    accounts: ApproveAccounts {
                        source: ix.accounts[0],
                        delegate: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                    args: ApproveArgs { amount },
                })
            },
            SplTokenInstruction::Revoke => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramInstruction::Revoke {
                    accounts: RevokeAccounts {
                        source: ix.accounts[0],
                        owner: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                })
            },
            SplTokenInstruction::SetAuthority {
                authority_type,
                new_authority,
            } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramInstruction::SetAuthority {
                    accounts: SetAuthorityAccounts {
                        account: ix.accounts[0],
                        current_authority: ix.accounts[1],
                        multisig_signers: ix.accounts[2..].to_vec(),
                    },
                    args: SetAuthorityArgs {
                        authority_type,
                        new_authority: new_authority.map(|p| p.to_bytes().into()).into(),
                    },
                })
            },
            SplTokenInstruction::MintTo { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramInstruction::MintTo {
                    accounts: MintToAccounts {
                        mint: ix.accounts[0],
                        account: ix.accounts[1],
                        mint_authority: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                    args: MintToArgs { amount },
                })
            },
            SplTokenInstruction::Burn { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramInstruction::Burn {
                    accounts: BurnAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                    args: BurnArgs { amount },
                })
            },
            SplTokenInstruction::CloseAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramInstruction::CloseAccount {
                    accounts: CloseAccountAccounts {
                        account: ix.accounts[0],
                        destination: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                })
            },
            SplTokenInstruction::FreezeAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramInstruction::FreezeAccount {
                    accounts: FreezeAccountAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        mint_freeze_authority: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                })
            },
            SplTokenInstruction::ThawAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramInstruction::ThawAccount {
                    accounts: ThawAccountAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        mint_freeze_authority: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                })
            },
            SplTokenInstruction::TransferChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(TokenProgramInstruction::TransferChecked {
                    accounts: TransferCheckedAccounts {
                        source: ix.accounts[0],
                        mint: ix.accounts[1],
                        destination: ix.accounts[2],
                        owner: ix.accounts[3],
                        multisig_signers: ix.accounts[4..].to_vec(),
                    },
                    args: TransferCheckedArgs { amount, decimals },
                })
            },
            SplTokenInstruction::ApproveChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(TokenProgramInstruction::ApproveChecked {
                    accounts: ApproveCheckedAccounts {
                        source: ix.accounts[0],
                        mint: ix.accounts[1],
                        delegate: ix.accounts[2],
                        owner: ix.accounts[3],
                        multisig_signers: ix.accounts[4..].to_vec(),
                    },
                    args: ApproveCheckedArgs { amount, decimals },
                })
            },
            SplTokenInstruction::MintToChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramInstruction::MintToChecked {
                    accounts: MintToCheckedAccounts {
                        mint: ix.accounts[0],
                        account: ix.accounts[1],
                        mint_authority: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                    args: MintToCheckedArgs { amount, decimals },
                })
            },
            SplTokenInstruction::BurnChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramInstruction::BurnChecked {
                    accounts: BurnCheckedAccounts {
                        account: ix.accounts[0],
                        mint: ix.accounts[1],
                        owner: ix.accounts[2],
                        multisig_signers: ix.accounts[3..].to_vec(),
                    },
                    args: BurnCheckedArgs { amount, decimals },
                })
            },
            SplTokenInstruction::SyncNative => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramInstruction::SyncNative {
                    accounts: SyncNativeAccounts {
                        account: ix.accounts[0],
                    },
                })
            },
            SplTokenInstruction::GetAccountDataSize => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramInstruction::GetAccountDataSize {
                    accounts: GetAccountDataSizeAccounts {
                        mint: ix.accounts[0],
                    },
                })
            },
            SplTokenInstruction::InitializeImmutableOwner => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramInstruction::InitializeImmutableOwner {
                    accounts: InitializeImmutableOwnerAccounts {
                        account: ix.accounts[0],
                    },
                })
            },
            SplTokenInstruction::AmountToUiAmount { amount } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramInstruction::AmountToUiAmount {
                    accounts: AmountToUiAmountAccounts {
                        mint: ix.accounts[0],
                    },
                    args: AmountToUiAmountArgs { amount },
                })
            },
            SplTokenInstruction::UiAmountToAmount { ui_amount } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramInstruction::UiAmountToAmount {
                    accounts: UiAmountToAmountAccounts {
                        mint: ix.accounts[0],
                    },
                    args: UiAmountToAmountArgs {
                        ui_amount: ui_amount.into(),
                    },
                })
            },
        }
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

        let ixs = tx_fixture!("55kpnRufcX9Fo44oRBXtrkxPRww4UWJKxCpgBV39kzAAag8oyJbd9Y3YWdQQUi3TBqtrhjgsMGb9Nw8bUxy7j5rt",&parser);

        let TokenProgramInstruction::MintToChecked { args, .. } = &ixs[0] else {
            panic!("Invalid Instruction");
        };

        assert_eq!(args.decimals, 10);
        assert_eq!(args.amount, 10.mul(10u64.pow(args.decimals.into())));
    }
}
