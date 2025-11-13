use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;

// INSTRUCTION: Stake
// Locks deposited funds for staking to earn rewards

pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
    require!(amount > 0, VaultError::InvalidAmount);

    let vault = &mut ctx.accounts.vault;

    // CONCEPT: Business Logic - Prevent double staking
    // For simplicity, we only allow one active stake at a time
    // In a more complex system, you could have multiple stakes
    require!(!vault.is_staked, VaultError::AlreadyStaked);

    // Check user has enough unstaked balance
    let available_balance = vault.get_available_balance();
    require!(
        available_balance >= amount,
        VaultError::InsufficientUnstakedBalance
    );

    // CONCEPT: Clock Sysvar
    // Reference: https://docs.solana.com/developing/runtime-facilities/sysvars#clock
    // Clock provides current Unix timestamp (seconds since Jan 1, 1970)
    // This is deterministic across all validators
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;

    // Update vault state to reflect staking
    vault.staked_amount = amount;
    vault.stake_timestamp = current_time;
    vault.is_staked = true;

    msg!(
        "Staked {} lamports at timestamp {}. Must wait 60 seconds for rewards.",
        amount,
        current_time
    );

    Ok(())
}

#[derive(Accounts)]
pub struct Stake<'info> {
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

// CONCEPT: Why Track Timestamp?
// We need to know WHEN staking started to:
// 1. Check if minimum duration has been met (60 seconds)
// 2. Calculate reward eligibility
//
// Unix timestamp is deterministic - all validators agree on it
// This prevents "gaming" the system with different times

// CONCEPT: Single Stake Design
// This simple version only allows one stake at a time:
// - Easier to understand for beginners
// - Simpler state management
// - Still demonstrates core concepts
//
// Advanced version could allow:
// - Multiple concurrent stakes
// - Different lock periods with different rates
// - Compounding rewards
