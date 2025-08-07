use bytemuck::from_bytes;
use pinocchio::{account_info::AccountInfo, msg, program_error::ProgramError, ProgramResult};

use crate::instructions::Counter;

pub fn read_counter(accounts: &[AccountInfo], _instruction_data: &[u8]) -> ProgramResult {
    // Expect the first account to be the counter
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let counter_account = &accounts[0];

    // Ensure the counter account has been initialized
    if counter_account.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    // Borrow the data for read access
    let data = &counter_account.try_borrow_data()?;

    // Ensure the buffer is large enough for our Counter struct
    if data.len() < core::mem::size_of::<Counter>() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the account data
    let counter: &Counter = from_bytes(&data[..core::mem::size_of::<Counter>()]);

    // Log the current value
    msg!(&format!("Current counter value: {}", counter.value));

    Ok(())
}
