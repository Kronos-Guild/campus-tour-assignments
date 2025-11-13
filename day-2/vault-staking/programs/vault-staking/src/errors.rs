use anchor_lang::prelude::*;

// CONCEPT: Custom Error Module
// Reference: https://www.anchor-lang.com/docs/errors
// Separating errors into their own module keeps code organized
// Each error gets a unique error code (6000, 6001, 6002, etc.)

#[error_code]
pub enum VaultError {
    // ORIGINAL ERRORS (from Day 1)
    #[msg("Amount must be greater than 0")]
    InvalidAmount,

    #[msg("Insufficient funds in vault")]
    InsufficientFunds,

    // NEW ERRORS (for Day 2 - Staking)
    #[msg("Cannot stake: funds are already staked")]
    AlreadyStaked,

    #[msg("Cannot unstake: no funds are currently staked")]
    NotStaked,

    #[msg("Insufficient unstaked balance to stake this amount")]
    InsufficientUnstakedBalance,

    #[msg("Minimum stake duration not met - no rewards earned")]
    MinimumStakeDurationNotMet,

    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,

    #[msg("Cannot withdraw: funds are currently staked")]
    FundsStaked,
}

// CONCEPT: Error Messages Best Practices
// 1. Be specific - tell the user exactly what went wrong
// 2. Be actionable - hint at what they need to do differently
// 3. Use present tense - "Cannot stake" not "Could not stake"
// 4. Keep them concise but informative

// CONCEPT: Error Codes
// Anchor automatically assigns error codes starting from 6000:
// - InvalidAmount = 6000
// - InsufficientFunds = 6001
// - AlreadyStaked = 6002
// - etc.
//
// These codes appear in transaction logs and help with debugging
