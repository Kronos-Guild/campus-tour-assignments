use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::Vault;
use crate::errors::VaultError;

// INSTRUCTION: Deposit
// Transfers SOL from user to vault and updates total_deposited

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(amount > 0, VaultError::InvalidAmount);

    // CONCEPT: CPI to System Program
    // Reference: https://www.anchor-lang.com/docs/cross-program-invocations
    let accounts = Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        accounts,
    );

    transfer(cpi_context, amount)?;

    // Update vault state to track total deposited
    let vault = &mut ctx.accounts.vault;
    vault.total_deposited = vault.total_deposited
        .checked_add(amount)
        .ok_or(VaultError::ArithmeticOverflow)?;

    msg!("Deposited {} lamports. Total deposited: {}", amount, vault.total_deposited);
    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// CONCEPT: Checked Math
// Reference: https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add
// checked_add returns Option<u64>:
// - Some(result) if addition succeeds
// - None if overflow would occur
// This prevents silent overflow bugs that could be exploited
