use anchor_lang::prelude::*;
use crate::state::Vault;

// CONCEPT: Modular Instructions
// Each instruction gets its own file for better organization
// This makes the codebase easier to navigate and maintain

// INSTRUCTION: Initialize
// Creates a new vault account for a user

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    // Set the owner
    vault.owner = ctx.accounts.user.key();

    // Store the PDA bump seed
    vault.bump = ctx.bumps.vault;

    // Initialize staking fields to default values
    vault.total_deposited = 0;
    vault.staked_amount = 0;
    vault.stake_timestamp = 0;
    vault.is_staked = false;

    msg!("Vault initialized for owner: {:?}", vault.owner);
    Ok(())
}

// ACCOUNTS STRUCT
#[derive(Accounts)]
pub struct Initialize<'info> {
    // CONCEPT: Account Initialization
    // Reference: https://www.anchor-lang.com/docs/the-accounts-struct#init
    #[account(
        init,
        payer = user,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
