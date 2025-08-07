use shank::ShankInstruction;

#[allow(dead_code)]
#[derive(ShankInstruction)]
pub enum CounterInstruction {
    /// Creates a new counter account
    #[account(0, writable, signer, name = "owner", desc = "Counter Owner Account")]
    #[account(1, writable, name = "counter", desc = "Counter Account")]
    Create,

    /// Increments the counter
    #[account(0, writable, name = "counter", desc = "Counter Account")]
    Increment,

    /// Reads counter value
    #[account(0, name = "counter", desc = "Counter Account")]
    ReadCounter,

    /// Deletes counter account
    #[account(0, writable, signer, name = "owner", desc = "Counter Owner Account")]
    #[account(1, writable, name = "counter", desc = "Counter Account")]
    Delete,
}
