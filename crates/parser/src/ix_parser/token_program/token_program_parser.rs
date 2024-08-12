use spl_pod::solana_program::program_error::ProgramError;
use spl_token::instruction::TokenInstruction;
use yellowstone_vixen_core::{ParseResult, Parser, Prefilter};

use super::token_ix_context::*;
use crate::ix_parser::vixen_ix::structure::{Instruction, InstructionData, InstructionUpdate};
pub struct TokenProgramInstruction;

pub type ToeknProgramIxs = Vec<TokenIxContext>;

impl Instruction<TokenIxContext> for TokenProgramInstruction {
    fn get_ix_context(ix_data: &InstructionData) -> Result<TokenIxContext, ProgramError> {
        let ix_type = TokenInstruction::unpack(&ix_data.data)?;

        match ix_type {
            TokenInstruction::Transfer { amount } => {
                let source = ix_data.accounts[0];
                let destination = ix_data.accounts[1];
                Ok(TokenIxContext::Transfer(Transfer {
                    source,
                    destination,
                    amount,
                }))
            },
            TokenInstruction::InitializeAccount => {
                let account = ix_data.accounts[0];
                let mint = ix_data.accounts[1];
                let owner = ix_data.accounts[2];
                Ok(TokenIxContext::InitializeAccount(InitializeAccount {
                    account,
                    mint,
                    owner,
                }))
            },
            TokenInstruction::InitializeMint {
                decimals,
                mint_authority,
                freeze_authority,
            } => {
                let mint = ix_data.accounts[0];
                Ok(TokenIxContext::InitializeMint(InitializeMint {
                    mint,
                    decimals,
                    mint_authority,
                    freeze_authority: freeze_authority.into(),
                }))
            },
            TokenInstruction::InitializeMint2 {
                decimals,
                mint_authority,
                freeze_authority,
            } => {
                let mint = ix_data.accounts[0];
                Ok(TokenIxContext::InitializeMint(InitializeMint {
                    mint,
                    decimals,
                    mint_authority,
                    freeze_authority: freeze_authority.into(),
                }))
            },

            TokenInstruction::InitializeAccount2 { owner } => {
                let account = ix_data.accounts[0];
                let mint = ix_data.accounts[1];
                Ok(TokenIxContext::InitializeAccount(InitializeAccount {
                    account,
                    mint,
                    owner,
                }))
            },

            TokenInstruction::InitializeAccount3 { owner } => {
                let account = ix_data.accounts[0];
                let mint = ix_data.accounts[1];
                Ok(TokenIxContext::InitializeAccount(InitializeAccount {
                    account,
                    mint,
                    owner,
                }))
            },

            TokenInstruction::InitializeMultisig { m } => {
                let multisig = ix_data.accounts[0];
                let signers = ix_data.accounts[2..].to_vec();
                Ok(TokenIxContext::InitializeMultisig(InitializeMultisig {
                    multisig,
                    signers,
                    m,
                }))
            },

            TokenInstruction::InitializeMultisig2 { m } => {
                let multisig = ix_data.accounts[0];
                let signers = ix_data.accounts[1..].to_vec();
                Ok(TokenIxContext::InitializeMultisig(InitializeMultisig {
                    multisig,
                    signers,
                    m,
                }))
            },

            TokenInstruction::Approve { amount } => {
                let source = ix_data.accounts[0];
                let delegate = ix_data.accounts[1];
                Ok(TokenIxContext::Approve(Approve {
                    source,
                    delegate,
                    amount,
                }))
            },

            TokenInstruction::Revoke => {
                let source = ix_data.accounts[0];
                Ok(TokenIxContext::Revoke(Revoke { source }))
            },

            TokenInstruction::SetAuthority {
                authority_type,
                new_authority,
            } => {
                let owned = ix_data.accounts[0];
                Ok(TokenIxContext::SetAuthority(SetAuthority {
                    owned,
                    authority_type,
                    new_authority,
                }))
            },

            TokenInstruction::MintTo { amount } => {
                let mint = ix_data.accounts[0];
                let account = ix_data.accounts[1];
                Ok(TokenIxContext::MintTo(MintTo {
                    mint,
                    account,
                    amount,
                }))
            },

            TokenInstruction::Burn { amount } => {
                let account = ix_data.accounts[0];
                let mint = ix_data.accounts[1];
                Ok(TokenIxContext::Burn(Burn {
                    account,
                    mint,
                    amount,
                }))
            },

            TokenInstruction::CloseAccount => {
                let account = ix_data.accounts[0];
                let destination = ix_data.accounts[1];
                Ok(TokenIxContext::CloseAccount(CloseAccount {
                    account,
                    destination,
                }))
            },

            TokenInstruction::FreezeAccount => {
                let account = ix_data.accounts[0];
                let mint = ix_data.accounts[1];
                Ok(TokenIxContext::FreezeAccount(FreezeAccount {
                    account,
                    mint,
                }))
            },

            TokenInstruction::ThawAccount => {
                let account = ix_data.accounts[0];
                let mint = ix_data.accounts[1];
                Ok(TokenIxContext::ThawAccount(ThawAccount { account, mint }))
            },

            TokenInstruction::TransferChecked { amount, decimals } => {
                let source = ix_data.accounts[0];
                let mint = ix_data.accounts[1];
                let destination = ix_data.accounts[2];
                Ok(TokenIxContext::TransferChecked(TransferChecked {
                    source,
                    mint,
                    destination,
                    amount,
                    decimals,
                }))
            },

            TokenInstruction::ApproveChecked { amount, decimals } => {
                let source = ix_data.accounts[0];
                let mint = ix_data.accounts[1];
                let delegate = ix_data.accounts[2];
                Ok(TokenIxContext::ApproveChecked(ApproveChecked {
                    source,
                    mint,
                    delegate,
                    amount,
                    decimals,
                }))
            },

            TokenInstruction::MintToChecked { amount, decimals } => {
                let mint = ix_data.accounts[0];
                let account = ix_data.accounts[1];
                Ok(TokenIxContext::MintToChecked(MintToChecked {
                    mint,
                    account,
                    amount,
                    decimals,
                }))
            },

            TokenInstruction::BurnChecked { amount, decimals } => {
                let account = ix_data.accounts[0];
                let mint = ix_data.accounts[1];
                Ok(TokenIxContext::BurnChecked(BurnChecked {
                    account,
                    mint,
                    amount,
                    decimals,
                }))
            },

            TokenInstruction::SyncNative => {
                let account = ix_data.accounts[0];
                Ok(TokenIxContext::SyncNative(SyncNative { account }))
            },

            TokenInstruction::GetAccountDataSize => {
                let mint = ix_data.accounts[0];
                Ok(TokenIxContext::GetAccountDataSize(GetAccountDataSize {
                    mint,
                }))
            },

            TokenInstruction::InitializeImmutableOwner => {
                let account = ix_data.accounts[0];
                Ok(TokenIxContext::InitializeImmutableOwner(
                    InitializeImmutableOwner { account },
                ))
            },

            TokenInstruction::AmountToUiAmount { amount } => {
                let mint = ix_data.accounts[0];
                Ok(TokenIxContext::AmountToUiAmount(AmountToUiAmount {
                    mint,
                    amount,
                }))
            },

            TokenInstruction::UiAmountToAmount { ui_amount } => {
                let mint = ix_data.accounts[0];
                Ok(TokenIxContext::UiAmountToAmount(UiAmountToAmount {
                    mint,
                    ui_amount: ui_amount.to_string(),
                }))
            },
        }
    }
}

impl Parser for TokenProgramInstruction {
    type Input = InstructionUpdate;
    type Output = ToeknProgramIxs;

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .transaction_accounts([spl_token::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, instruction: &InstructionUpdate) -> ParseResult<Self::Output> {
        // get ix_data from instruction update
        // call get_ix_context
        todo!()
    }
}
