use anchor_lang::prelude::*;

// ASSIGNMENT: Define custom errors for the escrow program
//
// REFERENCE: https://www.anchor-lang.com/docs/errors

#[error_code]
pub enum EscrowError {
    // TODO 1: Add an error for when amounts are invalid (e.g., 0 or too large)
    // Example: #[msg("Amount must be greater than 0")]
    // InvalidAmount,

    // TODO 2: Add an error for when the wrong taker tries to accept
    // (if escrow specifies a specific taker)
    // UnauthorizedTaker,

    // TODO 3: Add an error for when someone tries to cancel an escrow
    // that has already been accepted
    // EscrowAlreadyCompleted,

    // TODO 4: Add an error for when the escrow has expired
    // EscrowExpired,

    // TODO 5: Add an error for when someone other than the maker
    // tries to cancel the escrow
    // NotMaker,

    // BONUS TODO: Add more specific error messages for different scenarios
    // Think about what could go wrong in an escrow:
    // - Insufficient funds
    // - Arithmetic overflow
    // - Invalid state transitions
    // - Etc.
}

// ERROR MESSAGE BEST PRACTICES:
// 1. Be specific - tell the user exactly what went wrong
// 2. Be actionable - hint at how to fix it
// 3. Use present tense
// 4. Keep them under 100 characters
//
// Examples:
// ✅ Good: "Amount must be greater than 0"
// ❌ Bad: "Invalid"
//
// ✅ Good: "Only the maker can cancel this escrow"
// ❌ Bad: "Unauthorized"

// QUESTION FOR STUDENTS:
// Why do we need custom errors instead of just using panic! or assert!?
//
// Answer: Custom errors provide:
// 1. Better debugging (unique error codes)
// 2. Clear error messages for users
// 3. Proper error handling in tests
// 4. Professional code quality
