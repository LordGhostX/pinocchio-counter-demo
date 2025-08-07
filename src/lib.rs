// Suppress warnings caused by Solana-specific target configs
#![allow(unexpected_cfgs)]

use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

mod idl;
mod program_id;

// Import the instructions module
mod instructions;
use instructions::*;

// Declare the program's entrypoint
entrypoint!(process_instruction);

/// Entry function called when the program is invoked
pub fn process_instruction(
    _program_id: &Pubkey,     // Reference to the program ID
    accounts: &[AccountInfo], // List of accounts involved in the transaction
    instruction_data: &[u8],  // Raw instruction data (first byte = selector)
) -> ProgramResult {
    match instruction_data.first() {
        Some(0) => create::create(accounts, instruction_data),
        Some(1) => increment::increment(accounts, instruction_data),
        Some(2) => read_counter::read_counter(accounts, instruction_data),
        Some(3) => delete::delete(accounts, instruction_data),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
