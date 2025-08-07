# Pinocchio Counter Demo

A minimal Solana program written with [Pinocchio](https://github.com/anza-xyz/pinocchio), showcasing low-level control over accounts, instructions, and memory layout.

This program implements a simple counter with support for:

- `create`: Initializes a counter account with value `0` and stores the owner's public key
- `increment`: Increases the counter and logs the updated value
- `read_counter`: Reads and logs the current counter value
- `delete`: Clears the counter data if called by the original creator

All logic is implemented with manual account validation, zero-copy deserialization via [`bytemuck`](https://docs.rs/bytemuck), and program structure annotations using [`shank`](https://github.com/metaplex-foundation/shank).

## 📁 Project Structure

```bash
.
├── idl/
│   └── pinocchio_demo.json      # Generated IDL from Shank
├── src/
│   ├── instructions/            # Instruction handlers
│   │   ├── create.rs
│   │   ├── delete.rs
│   │   ├── increment.rs
│   │   ├── mod.rs
│   │   └── read_counter.rs
│   ├── idl.rs                   # Instruction enum for IDL generation
│   ├── lib.rs                   # Program entrypoint and routing
│   └── program_id.rs            # Program ID declaration
├── Cargo.toml
└── README.md
```

## 📖 Full Tutorial

Read the full guide here: [How to Build and Deploy a Solana Program Using Pinocchio](https://gist.github.com/LordGhostX/52ccaaf5752e6829b29646620d1a9ce7)
