use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::Vault;
use crate::errors::VaultError;

// INSTRUCTION: Withdraw
// Transfers SOL from vault back to user (only unstaked funds)

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(amount > 0, VaultError::InvalidAmount);

    let vault = &mut ctx.accounts.vault;

    // CONCEPT: Business Logic - Can only withdraw unstaked funds
    // Available balance = total_deposited - staked_amount
    let available_balance = vault.get_available_balance();

    require!(
        available_balance >= amount,
        VaultError::InsufficientFunds
    );

    // Check vault PDA has enough lamports
    let vault_lamports = vault.to_account_info().lamports();
    require!(
        vault_lamports >= amount,
        VaultError::InsufficientFunds
    );

    // CONCEPT: PDA Signing
    // Reference: https://www.anchor-lang.com/docs/pdas
    let seeds = &[
        b"vault",
        vault.owner.as_ref(),
        &[vault.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        accounts,
        signer_seeds,
    );

    transfer(cpi_context, amount)?;

    // Update vault state
    vault.total_deposited = vault.total_deposited
        .checked_sub(amount)
        .ok_or(VaultError::ArithmeticOverflow)?;

    msg!("Withdrawn {} lamports. Total deposited: {}", amount, vault.total_deposited);
    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
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

    pub system_program: Program<'info, System>,
}

// CONCEPT: Withdraw vs Unstake
// - Withdraw: removes unstaked funds from vault to wallet
// - Unstake: moves funds from "staked" to "unstaked" (with potential rewards)
//
// User flow:
// 1. Deposit SOL → vault (unstaked)
// 2. Stake → locks funds (staked)
// 3. Unstake → unlocks funds + rewards (unstaked)
// 4. Withdraw → removes funds from vault to wallet
