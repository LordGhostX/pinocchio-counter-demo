use bytemuck::from_bytes_mut;
use pinocchio::{account_info::AccountInfo, msg, program_error::ProgramError, ProgramResult};

use crate::instructions::Counter;

pub fn delete(accounts: &[AccountInfo], _instruction_data: &[u8]) -> ProgramResult {
    // Ensure the required accounts are present
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let owner_account = &accounts[0];
    let counter_account = &accounts[1];

    // Must be a signer to authorize the action
    if !owner_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let data = &mut counter_account.try_borrow_mut_data()?;

    // Ensure the buffer is large enough for our Counter struct
    if data.len() < core::mem::size_of::<Counter>() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the account data
    let counter: &mut Counter = from_bytes_mut(&mut data[..core::mem::size_of::<Counter>()]);

    // Only the original owner can delete the counter
    if counter.owner != *owner_account.key() {
        return Err(ProgramError::IllegalOwner);
    }

    // Zero out the data
    counter.value = 0;
    counter.owner = Default::default();

    msg!("Counter deleted");

    Ok(())
}
