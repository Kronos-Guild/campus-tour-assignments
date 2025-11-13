// ASSIGNMENT: Define constants for the escrow program

// TODO 1: Define a seed prefix for the escrow PDA
// This is used to derive the escrow account address
// Example: pub const ESCROW_SEED: &[u8] = b"escrow";
//
// HINT: Use a descriptive name like "escrow" or "escrow_state"

// TODO 2: (OPTIONAL) Define a minimum escrow amount
// This prevents spam or dust escrows
// Example: pub const MIN_ESCROW_AMOUNT: u64 = 1_000_000; // 0.001 SOL
//
// HINT: 1 SOL = 1,000,000,000 lamports

// TODO 3: (OPTIONAL) Define a maximum escrow duration
// After this time, escrows automatically expire
// Example: pub const MAX_ESCROW_DURATION: i64 = 7 * 24 * 60 * 60; // 7 days in seconds
//
// HINT: Calculate seconds: days * hours * minutes * seconds

// QUESTION FOR STUDENTS:
// Why use constants instead of hardcoding values in the program?
//
// Answer:
// 1. Easy to modify in one place
// 2. Self-documenting (names explain purpose)
// 3. Prevents typos and inconsistencies
// 4. Makes testing easier (can override in tests)

// BONUS: Think about other constants you might need
// - Fee percentages?
// - Maximum number of concurrent escrows per user?
// - Minimum time before cancellation?
