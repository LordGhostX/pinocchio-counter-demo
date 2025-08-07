use bytemuck::{Pod, Zeroable};
use pinocchio::pubkey::Pubkey;
use shank::ShankAccount;

pub mod create;
pub mod delete;
pub mod increment;
pub mod read_counter;

/// A simple counter account
///
/// #[repr(C)] ensures a stable memory layout
/// Pod + Zeroable lets us safely cast from raw bytes
/// ShankAccount marks this struct as an account type for IDL generation
#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, ShankAccount)]
pub struct Counter {
    pub value: u64,    // The current value of the counter
    pub owner: Pubkey, // The public key of the account that created this counter
}
