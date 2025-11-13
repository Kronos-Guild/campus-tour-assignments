use anchor_lang::prelude::*;

// ASSIGNMENT: Complete the Escrow account structure
//
// An escrow is a smart contract that holds funds from two parties
// and swaps them atomically (both get funds or neither does)
//
// REFERENCE: https://www.anchor-lang.com/docs/the-accounts-struct

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    // TODO 1: Add the maker's public key (the person who creates the escrow)
    // Hint: Use Pubkey type (32 bytes)
    // pub maker: ???,

    // TODO 2: Add the taker's public key (the person who accepts the escrow)
    // This might be None initially if any taker is allowed
    // Hint: Use Option<Pubkey> if optional, or Pubkey if required
    // pub taker: ???,

    // TODO 3: Add the amount the maker is offering (in lamports)
    // Hint: Use u64 for lamport amounts
    // pub maker_amount: ???,

    // TODO 4: Add the amount the maker wants in return (in lamports)
    // Hint: Use u64 for lamport amounts
    // pub taker_amount: ???,

    // TODO 5: Add a bump seed for the PDA
    // Hint: Use u8 type
    // pub bump: ???,

    // TODO 6: Add a timestamp for when the escrow was created
    // Hint: Use i64 for Unix timestamps
    // pub created_at: ???,

    // BONUS TODO: Add an optional expiration timestamp
    // After this time, the maker can cancel even if a taker is specified
    // pub expires_at: Option<i64>,

    // BONUS TODO: Add a status field to track escrow state
    // Could be: Pending, Accepted, Cancelled
    // Hint: Create an enum for this
    // pub status: ???,
}

// BONUS: Create an enum for escrow status
// REFERENCE: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
//
// #[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
// pub enum EscrowStatus {
//     Pending,
//     Completed,
//     Cancelled,
// }

// QUESTION FOR STUDENTS:
// Why do we use a PDA for the escrow account instead of a regular account?
//
// Answer: PDAs allow the program to sign for the escrow, which is necessary
// for the program to transfer funds from the escrow to the taker when the
// trade is executed. Regular accounts require a private key to sign.

// SPACE CALCULATION PRACTICE:
// Calculate the total space needed for your Escrow account:
// - Discriminator: 8 bytes (Anchor adds this automatically)
// - Your fields: ??? bytes
// Total: ??? bytes
//
// Hint: Pubkey = 32 bytes, u64 = 8 bytes, u8 = 1 byte, i64 = 8 bytes
