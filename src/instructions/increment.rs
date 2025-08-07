use bytemuck::from_bytes_mut;
use pinocchio::{account_info::AccountInfo, msg, program_error::ProgramError, ProgramResult};

use crate::instructions::Counter;

pub fn increment(accounts: &[AccountInfo], _instruction_data: &[u8]) -> ProgramResult {
    // Expect the first account to be the counter
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let counter_account = &accounts[0];

    // Ensure the counter account has been initialized
    if counter_account.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    // Borrow the account's data as a mutable byte slice
    let data = &mut counter_account.try_borrow_mut_data()?;

    // Ensure the buffer is large enough for our Counter struct
    if data.len() < core::mem::size_of::<Counter>() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the account data
    let counter: &mut Counter = from_bytes_mut(&mut data[..core::mem::size_of::<Counter>()]);

    // Increment the counter
    counter.value += 1;

    msg!(&format!("Counter incremented: {}", counter.value));

    Ok(())
}
