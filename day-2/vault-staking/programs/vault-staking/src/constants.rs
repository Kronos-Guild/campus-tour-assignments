// CONCEPT: Constants Module
// Reference: Rust modules - https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
// Separating constants makes the code more maintainable and easier to modify

// STAKING PARAMETERS

// Minimum time (in seconds) that funds must be staked to earn rewards
// 60 seconds = 1 minute (easy for testing/demo)
// In production, this might be days or weeks
pub const MIN_STAKE_DURATION: i64 = 60;

// Reward rate: 10% of staked amount
// Expressed as basis points (1000 = 10%, 10000 = 100%)
// This avoids floating point math which isn't available in Solana programs
pub const REWARD_RATE_BASIS_POINTS: u64 = 1000; // 10%

// Basis points divisor (10000 = 100%)
// Used to calculate: reward = (staked_amount * REWARD_RATE_BASIS_POINTS) / BASIS_POINTS_DIVISOR
pub const BASIS_POINTS_DIVISOR: u64 = 10000;

// CONCEPT: Fixed-Point Math
// Solana programs can't use floating point (f32, f64) due to non-determinism
// Instead, we use integers with a scale factor (basis points)
// Example: 10% = 1000 basis points, 1% = 100 basis points
// Calculation: reward = (amount * 1000) / 10000 = amount * 0.10
