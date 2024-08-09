use spl_pod::solana_program::{
    instruction::CompiledInstruction, message::AccountKeys, pubkey::Pubkey,
};
pub mod account_parser;

pub mod tx_parser;

pub struct InstructionUpdate {
    // Complete list of transaction accounts
    pub account_keys: AccountKeys,
    pub instruction: CompiledInstruction,
    pub program: Pubkey,
    pub stack_height: Option<u32>,
}

pub trait Instruction<A, D> {
    fn accounts() -> A { todo!() }
    fn data() -> D { todo!() }
}
