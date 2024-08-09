use serde_json::Value;
use solana_transaction_status::parse_instruction::parse;
use spl_pod::solana_program::{program_option::COption, pubkey::Pubkey};
use spl_token::instruction::TokenInstruction;
use yellowstone_vixen_core::{
    Instruction, InstructionUpdate, ParseError, ParseResult, Parser, Prefilter, TransactionUpdate,
};

use crate::tx_parser::vixen_transaction::structure::VixenTransaction;
pub struct TokenProgramInstruction;

struct TransferAccounts {
    mint_info: Pubkey,
    receiver: Pubkey,
}

enum Accounts {
    Transfer(TransferAccounts),
}

impl Instruction<Accounts, TokenInstruction> for TokenProgramInstruction {
    fn new(accounts: &[Pubkey], data: Vec<u8>) -> Self { todo!() }
}

impl TryFrom<&InstructionUpdate> for TokenProgramInstruction {
    type Error = ParseError;

    fn try_from(instruction: &InstructionUpdate) -> Result<Self, Self::Error> {
        let result = parse(
            instruction.program,
            instruction.instruction,
            instruction.account_keys,
            instruction.stack_height,
        )?;

        match result {
            ParsedInstruction::Parsed(parsed) => {
                let ix = parse_token_program_ix(&parsed)
                    .ok_or(ParseError::InstructionKeyMismatch(instruction.program))?;
                Ok(Self::new(&instruction.account_keys, ix))
            },
        }
    }
}
pub struct TokenProgramTransactionParser;

impl Parser for TokenProgramTransactionParser {
    type Input = InstructionUpdate;
    type Output = TokenProgramInstruction;

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .transaction_accounts([spl_token::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, instruction: &InstructionUpdate) -> ParseResult<Self::Output> {
        instruction.try_into()
    }
}

fn get_u64_value(value: &serde_json::Value) -> Option<u64> {
    value
        .as_str()
        .map(|data| data.parse::<u64>().ok())
        .flatten()
}

fn get_u8_value(value: &serde_json::Value) -> Option<u8> {
    value.as_str().map(|data| data.parse::<u8>().ok()).flatten()
}

pub fn parse_token_program_ix<'i>(json_val: &Value) -> Option<TokenInstruction<'i>> {
    let ix_type = json_val
        .get("type")
        .and_then(|val| val.as_str())
        .unwrap_or_default();

    let ix = json_val.get("info").unwrap();
    let ix = match ix_type {
        "transfer" => {
            let amount = get_u64_value(ix.get("amount")?)?;
            TokenInstruction::Transfer { amount }
        },
        "approve" => {
            let amount = get_u64_value(ix.get("amount")?)?;
            TokenInstruction::Approve { amount }
        },
        "initializeAccount" => TokenInstruction::InitializeAccount,
        "initializeMint" => {
            let decimals = get_u8_value(ix.get("decimals")?)?;
            let mint_authority = ix.get("mint_authority")?.as_str()?.parse().ok()?;
            let freeze_authority = if let Some(freeze_authority) = ix.get("freeze_authority") {
                COption::Some(freeze_authority.as_str()?.parse().ok()?)
            } else {
                COption::None
            };
            TokenInstruction::InitializeMint {
                decimals,
                mint_authority,
                freeze_authority,
            }
        },
        "mintTo" => {
            let amount = get_u64_value(ix.get("amount")?)?;
            TokenInstruction::MintTo { amount }
        },
        "burn" => {
            let amount = get_u64_value(ix.get("amount")?)?;
            TokenInstruction::Burn { amount }
        },
        "closeAccount" => TokenInstruction::CloseAccount,
        "freezeAccount" => TokenInstruction::FreezeAccount,
        "thawAccount" => TokenInstruction::ThawAccount,
        "transferChecked" => {
            let amount = get_u64_value(ix.get("amount")?)?;
            let decimals = get_u8_value(ix.get("decimals")?)?;
            TokenInstruction::TransferChecked { amount, decimals }
        },
        "approveChecked" => {
            let amount = get_u64_value(ix.get("amount")?)?;
            let decimals = get_u8_value(ix.get("decimals")?)?;
            TokenInstruction::ApproveChecked { amount, decimals }
        },
        "mintToChecked" => {
            let amount = get_u64_value(ix.get("amount")?)?;
            let decimals = get_u8_value(ix.get("decimals")?)?;
            TokenInstruction::MintToChecked { amount, decimals }
        },
        "burnChecked" => {
            let amount = get_u64_value(ix.get("amount")?)?;
            let decimals = get_u8_value(ix.get("decimals")?)?;
            TokenInstruction::BurnChecked { amount, decimals }
        },
        "initializeAccount2" => {
            let owner = ix.get("owner")?.as_str()?.parse().ok()?;
            TokenInstruction::InitializeAccount2 { owner }
        },
        "syncNative" => TokenInstruction::SyncNative,
        "initializeAccount3" => {
            let owner = ix.get("owner")?.as_str()?.parse().ok()?;
            TokenInstruction::InitializeAccount3 { owner }
        },
        _ => return None,
    };

    Some(ix)
}
