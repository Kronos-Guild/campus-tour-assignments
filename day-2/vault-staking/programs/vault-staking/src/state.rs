use anchor_lang::prelude::*;

// CONCEPT: State Module
// Separating account structs into their own module makes the code cleaner
// and easier to understand, especially as the program grows

// CONCEPT: Extended Vault Account with Staking
// Reference: https://www.anchor-lang.com/docs/the-accounts-struct
// We extend the original vault to track staking information

#[account]
#[derive(InitSpace)]
pub struct Vault {
    // ORIGINAL FIELDS (from Day 1)
    pub owner: Pubkey,           // 32 bytes - who owns this vault
    pub bump: u8,                // 1 byte - PDA bump seed

    // NEW FIELDS (for Day 2 - Staking)
    pub total_deposited: u64,    // 8 bytes - total SOL ever deposited
    pub staked_amount: u64,      // 8 bytes - currently staked amount
    pub stake_timestamp: i64,    // 8 bytes - when staking started (Unix timestamp)
    pub is_staked: bool,         // 1 byte - is currently staked?

    // Total: 32 + 1 + 8 + 8 + 8 + 1 = 58 bytes
    // With discriminator: 58 + 8 = 66 bytes
}

impl Vault {
    // CONCEPT: Helper Methods on Account Structs
    // We can add methods to make the code more readable

    /// Calculates the staking duration in seconds
    pub fn get_stake_duration(&self, current_time: i64) -> i64 {
        if !self.is_staked {
            return 0;
        }
        current_time.saturating_sub(self.stake_timestamp)
    }

    /// Checks if the minimum stake duration has been met
    pub fn has_met_min_duration(&self, current_time: i64, min_duration: i64) -> bool {
        self.get_stake_duration(current_time) >= min_duration
    }

    /// Gets the available (unstaked) balance
    /// Available balance = total deposited - currently staked
    pub fn get_available_balance(&self) -> u64 {
        self.total_deposited.saturating_sub(self.staked_amount)
    }
}

// CONCEPT: Account Space Calculation
// InitSpace automatically calculates: 32 + 1 + 8 + 8 + 8 + 1 = 58 bytes
// Anchor adds 8-byte discriminator automatically
// Total account size = 66 bytes

// CONCEPT: Saturating Math
// Reference: https://doc.rust-lang.org/std/primitive.u64.html#method.saturating_sub
// saturating_sub prevents underflow - if result would be negative, returns 0
// This is safer than regular subtraction which could panic
