use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, unique_seed: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = ctx.accounts.user.key();
        vault.bump = ctx.bumps.vault;
        msg!("Vault initialized with seed: {}", unique_seed);
        Ok(())
    }

    // <--- UPDATED: Added unique_seed as an argument here
    pub fn deposit(ctx: Context<Deposit>, unique_seed: u64, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.vault.key(),
            amount,
        );
        
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        msg!("Deposited {} lamports to vault", amount);
        Ok(())
    }

    // <--- UPDATED: Added unique_seed as an argument here
    pub fn withdraw(ctx: Context<Withdraw>, unique_seed: u64, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);

        let vault = &mut ctx.accounts.vault;
        let user = &mut ctx.accounts.user;

        let vault_lamports = vault.to_account_info().lamports();
        require!(
            vault_lamports >= amount,
            VaultError::InsufficientFunds
        );

        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;

        msg!("Withdrawn {} lamports from vault", amount);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(unique_seed: u64)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", user.key().as_ref(), &unique_seed.to_le_bytes()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// <--- UPDATED: Added instruction macro and seed to seeds array
#[derive(Accounts)]
#[instruction(unique_seed: u64)] 
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref(), &unique_seed.to_le_bytes()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// <--- UPDATED: Added instruction macro and seed to seeds array
#[derive(Accounts)]
#[instruction(unique_seed: u64)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref(), &unique_seed.to_le_bytes()],
        bump = vault.bump,
        constraint = vault.owner == user.key()
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub owner: Pubkey,
    pub bump: u8,
}

#[error_code]
pub enum VaultError {
    #[msg("Amount must be greater than 0")]
    InvalidAmount,
    #[msg("Insufficient funds in vault")]
    InsufficientFunds,
}
