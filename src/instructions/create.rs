use bytemuck::from_bytes_mut;
use pinocchio::{account_info::AccountInfo, msg, program_error::ProgramError, ProgramResult};

use crate::instructions::Counter;

pub fn create(accounts: &[AccountInfo], _instruction_data: &[u8]) -> ProgramResult {
    // Ensure the required accounts are present
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    // The owner (payer and signer)
    let owner_account = &accounts[0];

    // The counter account to initialize
    let counter_account = &accounts[1];

    // Must be a signer to authorize the action
    if !owner_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Ensure you're not reusing an already initialized account
    if !counter_account.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Borrow the account's data as a mutable byte slice
    let data = &mut counter_account.try_borrow_mut_data()?;

    // Ensure the buffer is large enough for our Counter struct
    if data.len() < core::mem::size_of::<Counter>() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the account data
    let counter: &mut Counter = from_bytes_mut(&mut data[..core::mem::size_of::<Counter>()]);

    // Initialize the fields
    counter.value = 0;
    counter.owner = *owner_account.key();

    msg!("Counter created");

    Ok(())
}
