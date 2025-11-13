use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::constants::*;

// INSTRUCTION: Unstake
// Unlocks staked funds and distributes rewards if minimum duration was met

pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    // Check that funds are actually staked
    require!(vault.is_staked, VaultError::NotStaked);

    // CONCEPT: Time-based Rewards
    // Get current time from Clock sysvar
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;

    // Calculate how long funds have been staked
    let stake_duration = vault.get_stake_duration(current_time);

    msg!("Unstaking {} lamports. Staked for {} seconds.", vault.staked_amount, stake_duration);

    // CONCEPT: Conditional Rewards
    // Only give rewards if minimum duration was met
    let reward_amount = if vault.has_met_min_duration(current_time, MIN_STAKE_DURATION) {
        // CONCEPT: Reward Calculation with Basis Points
        // Reference: Fixed-point arithmetic
        // Formula: reward = (staked_amount * rate) / divisor
        // Example: (1000 SOL * 1000) / 10000 = 100 SOL (10% reward)
        let reward = (vault.staked_amount as u128)
            .checked_mul(REWARD_RATE_BASIS_POINTS as u128)
            .ok_or(VaultError::ArithmeticOverflow)?
            .checked_div(BASIS_POINTS_DIVISOR as u128)
            .ok_or(VaultError::ArithmeticOverflow)?
            as u64;

        msg!("✅ Minimum duration met! Earned {} lamports reward (10%)", reward);
        reward
    } else {
        msg!("❌ Minimum duration NOT met. No rewards earned. (Staked for {}s, needed {}s)",
            stake_duration,
            MIN_STAKE_DURATION
        );
        0
    };

    // CONCEPT: State Updates
    // Move staked funds back to total_deposited (now unstaked)
    // Add the reward to total_deposited as well
    vault.total_deposited = vault.total_deposited
        .checked_add(reward_amount)
        .ok_or(VaultError::ArithmeticOverflow)?;

    // Reset staking fields
    vault.staked_amount = 0;
    vault.stake_timestamp = 0;
    vault.is_staked = false;

    msg!("Funds unstaked. Total balance now: {} lamports", vault.total_deposited);

    Ok(())
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault.owner.as_ref()],
        bump = vault.bump,
        has_one = owner
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Verified through has_one constraint
    pub owner: UncheckedAccount<'info>,
}

// CONCEPT: Where Do Rewards Come From?
// In this simple version, rewards are "printed" by adding to total_deposited
// This is a simplified educational model.
//
// In production systems, rewards typically come from:
// 1. Pre-funded reward pool (like token emissions)
// 2. Protocol fees collected from users
// 3. Yield from DeFi strategies (e.g., lending)
// 4. Inflation (new tokens minted)
//
// For Day 2, we keep it simple to focus on:
// - Time-based logic
// - Reward calculations
// - State management

// CONCEPT: Why Use u128 for Calculation?
// When multiplying two u64 values, the result could overflow u64
// Example: u64::MAX * 1000 = overflow!
// By casting to u128, we have more room, then cast back to u64
// This is safe because our final result (after division) fits in u64

// CONCEPT: Checked Math is Critical
// Without checked math:
// - Overflow could wrap around (bad!)
// - User might get wrong reward amount
// - Could be exploited by attackers
//
// With checked math:
// - Returns None on overflow
// - Transaction fails safely
// - Prevents exploitation
