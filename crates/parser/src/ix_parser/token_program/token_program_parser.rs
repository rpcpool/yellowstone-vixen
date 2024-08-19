use spl_token::instruction::TokenInstruction;
use yellowstone_vixen_core::{ParseError, ParseResult, Parser, Prefilter};

use super::token_ix::*;
use crate::ix_parser::vixen_ix::{
    helpers::{check_min_accounts_req, get_multisig_signers},
    structure::{InstructionParser, InstructionUpdate, ReadableInstruction, ReadableInstructions},
};

#[derive(Debug)]
pub struct TokenProgramIxParser;

impl Parser for TokenProgramIxParser {
    type Input = InstructionUpdate;
    type Output = TokenProgramIx;

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .transaction_accounts([spl_token::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, ix_update: &InstructionUpdate) -> ParseResult<Self::Output> {
        Self::parse_readable_ix(ix_update).map_err(|e| ParseError::Other(e.into()))
    }
}

pub type TokenProgramIxs = Vec<ReadableInstructions<TokenProgramIx>>;

// impl TokenProgramIxParser {
//     pub fn parse(ix_update: &InstructionUpdate) -> ParseResult<TokenProgramIx> {
// let ix_update = InstructionsUpdate::try_from(tx_update)?;
// let mut readable_ixs: TokenProgramIxs = Vec::new();

// for inner_instruction in &ix_update.instructions {
//     let mut inner_ixs: Vec<TokenProgramIx> = Vec::new();
//     for instruction in &inner_instruction.instructions {
//         let mut accounts: Vec<Pubkey> = Vec::new();
//         let account_keys = AccountKeys::new(
//             &ix_update.tx_account_keys.static_keys,
//             ix_update.tx_account_keys.dynamic_keys.as_ref(),
//         );
//         for ix in &instruction.accounts {
//             accounts.push(account_keys[*ix as usize]);
//         }

//         let ix = Instruction {
//             data: instruction.data.clone(),
//             accounts,
//         };
// let ix_context = Self::get_readable_ix(ix_update)?;

//         inner_ixs.push(ix_context);
//     }

//     let readable_ix = ReadableInstructions {
//         index: inner_instruction.index,
//         instructions: inner_ixs,
//     };

//     readable_ixs.push(readable_ix);
// }

// println!("readable_ixs: {:#?}", readable_ixs);
// Ok(readable_ixs)
//         Ok(ix_context)
//     }
// }

impl InstructionParser<TokenProgramIx> for TokenProgramIxParser {
    fn parse_readable_ix(ix_update: &InstructionUpdate) -> Result<TokenProgramIx, String> {
        let ix_type = TokenInstruction::unpack(&ix_update.data).map_err(|e| e.to_string())?;

        let accounts_len = ix_update.accounts.len();
        match ix_type {
            TokenInstruction::Transfer { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::Transfer(ReadableInstruction {
                    accounts: TransferAccounts {
                        source: ix_update.accounts[0],
                        destination: ix_update.accounts[1],
                        owner: ix_update.accounts[2],
                        multisig_signers: get_multisig_signers(ix_update, 3),
                    },
                    data: Some(TransferData { amount }),
                }))
            },
            TokenInstruction::InitializeAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::InitializeAccount(ReadableInstruction {
                    accounts: InitializeAccountAccounts {
                        account: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                        owner: ix_update.accounts[2],
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
                        mint: ix_update.accounts[0],
                    },
                    data: Some(InitializeMintData {
                        decimals,
                        mint_authority: mint_authority.into(),
                        freeze_authority,
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
                        mint: ix_update.accounts[0],
                    },
                    data: Some(InitializeMintData {
                        decimals,
                        mint_authority: mint_authority.into(),
                        freeze_authority,
                    }),
                }))
            },

            TokenInstruction::InitializeAccount2 { owner } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramIx::InitializeAccount2(ReadableInstruction {
                    accounts: InitializeAccount2Accounts {
                        account: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                    },
                    data: Some(InitializeAccountData2 { owner }),
                }))
            },

            TokenInstruction::InitializeAccount3 { owner } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramIx::InitializeAccount3(ReadableInstruction {
                    accounts: InitializeAccount2Accounts {
                        account: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                    },
                    data: Some(InitializeAccountData2 { owner }),
                }))
            },
            TokenInstruction::InitializeMultisig { m } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::InitializeMultisig(ReadableInstruction {
                    accounts: InitializeMultisigAccounts {
                        multisig: ix_update.accounts[0],
                        signers: get_multisig_signers(ix_update, 2),
                    },
                    data: Some(InitializeMultisigData { m }),
                }))
            },

            TokenInstruction::InitializeMultisig2 { m } => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramIx::InitializeMultisig(ReadableInstruction {
                    accounts: InitializeMultisigAccounts {
                        multisig: ix_update.accounts[0],
                        signers: get_multisig_signers(ix_update, 1),
                    },
                    data: Some(InitializeMultisigData { m }),
                }))
            },

            TokenInstruction::Approve { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::Approve(ReadableInstruction {
                    accounts: ApproveAccounts {
                        source: ix_update.accounts[0],
                        delegate: ix_update.accounts[1],
                        owner: ix_update.accounts[2],
                        multisig_signers: get_multisig_signers(ix_update, 3),
                    },
                    data: Some(ApproveData { amount }),
                }))
            },

            TokenInstruction::Revoke => {
                check_min_accounts_req(accounts_len, 2)?;
                Ok(TokenProgramIx::Revoke(ReadableInstruction {
                    accounts: RevokeAccounts {
                        source: ix_update.accounts[0],
                        owner: ix_update.accounts[1],
                        multisig_signers: get_multisig_signers(ix_update, 2),
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
                        current_authority: ix_update.accounts[1],
                        account: ix_update.accounts[0],
                        multisig_signers: get_multisig_signers(ix_update, 2),
                    },
                    data: Some(SetAuthorityData {
                        authority_type,
                        new_authority,
                    }),
                }))
            },

            TokenInstruction::MintTo { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::MintTo(ReadableInstruction {
                    accounts: MintToAccounts {
                        mint: ix_update.accounts[0],
                        account: ix_update.accounts[1],
                        mint_authority: ix_update.accounts[2],
                        multisig_signers: get_multisig_signers(ix_update, 3),
                    },
                    data: Some(MintToData { amount }),
                }))
            },

            TokenInstruction::Burn { amount } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::Burn(ReadableInstruction {
                    accounts: BurnAccounts {
                        account: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                        owner: ix_update.accounts[2],
                        multisig_signers: get_multisig_signers(ix_update, 3),
                    },
                    data: Some(BurnData { amount }),
                }))
            },

            TokenInstruction::CloseAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::CloseAccount(ReadableInstruction {
                    accounts: CloseAccountAccounts {
                        account: ix_update.accounts[0],
                        destination: ix_update.accounts[1],
                        owner: ix_update.accounts[2],
                        multisig_signers: get_multisig_signers(ix_update, 3),
                    },
                    data: None,
                }))
            },

            TokenInstruction::FreezeAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::FreezeAccount(ReadableInstruction {
                    accounts: FreezeAccountAccounts {
                        account: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                        mint_freeze_authority: ix_update.accounts[2],
                        multisig_signers: get_multisig_signers(ix_update, 3),
                    },
                    data: None,
                }))
            },

            TokenInstruction::ThawAccount => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::ThawAccount(ReadableInstruction {
                    accounts: ThawAccountAccounts {
                        account: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                        mint_freeze_authority: ix_update.accounts[2],
                        multisig_signers: get_multisig_signers(ix_update, 3),
                    },
                    data: None,
                }))
            },

            TokenInstruction::TransferChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(TokenProgramIx::TransferChecked(ReadableInstruction {
                    accounts: TransferCheckedAccounts {
                        source: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                        destination: ix_update.accounts[2],
                        owner: ix_update.accounts[3],
                        multisig_signers: get_multisig_signers(ix_update, 4),
                    },
                    data: Some(TransferCheckedData { amount, decimals }),
                }))
            },

            TokenInstruction::ApproveChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 4)?;
                Ok(TokenProgramIx::ApproveChecked(ReadableInstruction {
                    accounts: ApproveCheckedAccounts {
                        source: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                        delegate: ix_update.accounts[2],
                        owner: ix_update.accounts[3],
                        multisig_signers: get_multisig_signers(ix_update, 4),
                    },
                    data: Some(ApproveCheckedData { amount, decimals }),
                }))
            },

            TokenInstruction::MintToChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::MintToChecked(ReadableInstruction {
                    accounts: MintToCheckedAccounts {
                        mint: ix_update.accounts[0],
                        account: ix_update.accounts[1],
                        mint_authority: ix_update.accounts[2],
                        multisig_signers: get_multisig_signers(ix_update, 3),
                    },
                    data: Some(MintToCheckedData { amount, decimals }),
                }))
            },

            TokenInstruction::BurnChecked { amount, decimals } => {
                check_min_accounts_req(accounts_len, 3)?;
                Ok(TokenProgramIx::BurnChecked(ReadableInstruction {
                    accounts: BurnCheckedAccounts {
                        account: ix_update.accounts[0],
                        mint: ix_update.accounts[1],
                        owner: ix_update.accounts[2],
                        multisig_signers: get_multisig_signers(ix_update, 3),
                    },
                    data: Some(BurnCheckedData { amount, decimals }),
                }))
            },

            TokenInstruction::SyncNative => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::SyncNative(ReadableInstruction {
                    accounts: SyncNativeAccounts {
                        account: ix_update.accounts[0],
                    },
                    data: None,
                }))
            },

            TokenInstruction::GetAccountDataSize => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::GetAccountDataSize(ReadableInstruction {
                    accounts: GetAccountDataSizeAccounts {
                        mint: ix_update.accounts[0],
                    },
                    data: None,
                }))
            },

            TokenInstruction::InitializeImmutableOwner => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::InitializeImmutableOwner(
                    ReadableInstruction {
                        accounts: InitializeImmutableOwnerAccounts {
                            account: ix_update.accounts[0],
                        },
                        data: None,
                    },
                ))
            },

            TokenInstruction::AmountToUiAmount { amount } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::AmountToUiAmount(ReadableInstruction {
                    accounts: AmountToUiAmountAccounts {
                        mint: ix_update.accounts[0],
                    },
                    data: Some(AmountToUiAmountData { amount }),
                }))
            },

            TokenInstruction::UiAmountToAmount { ui_amount } => {
                check_min_accounts_req(accounts_len, 1)?;
                Ok(TokenProgramIx::UiAmountToAmount(ReadableInstruction {
                    accounts: UiAmountToAmountAccounts {
                        mint: ix_update.accounts[0],
                    },
                    data: Some(UiAmountToAmountData {
                        ui_amount: ui_amount.into(),
                    }),
                }))
            },
        }
    }
}
