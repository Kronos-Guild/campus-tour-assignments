use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

// CONCEPT: Program ID (declare_id!)
// Reference: https://www.anchor-lang.com/docs/programs
// This macro declares the program's on-chain address. It's generated when you create
// a new Anchor program and is used to verify that the correct program is being called.
declare_id!("4jzb27LPnWxniBfZ78suu7dvfKmmuX5pRHqKcje6NKyY");

// CONCEPT: #[program] macro
// Reference: https://www.anchor-lang.com/docs/programs
// This attribute macro marks the module containing your program's instruction handlers.
// Each public function in this module becomes an instruction that can be called.
#[program]
pub mod vault {
    use super::*;

    // INSTRUCTION 1: Initialize
    // This creates a new vault account for a user
    // CONCEPT: Context<T> - https://www.anchor-lang.com/docs/the-accounts-struct
    // Context provides access to accounts and program metadata
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        // Store the owner's public key
        vault.owner = ctx.accounts.user.key();

        // CONCEPT: Bumps - https://www.anchor-lang.com/docs/pdas
        // ctx.bumps provides the bump seed used to derive the PDA
        // We store it so we can use it later for signing
        vault.bump = ctx.bumps.vault;

        msg!("Vault initialized for owner: {:?}", vault.owner);
        Ok(())
    }

    // INSTRUCTION 2: Deposit
    // Transfers SOL from user to vault
    // CONCEPT: CPI (Cross-Program Invocation) - https://www.anchor-lang.com/docs/cross-program-invocations
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        // Validate amount is greater than 0
        require!(amount > 0, VaultError::InvalidAmount);

        // CONCEPT: CPI to System Program
        // Reference: https://www.anchor-lang.com/docs/cross-program-invocations
        // We use the system program to transfer SOL
        let accounts = Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            accounts,
        );

        // Execute the transfer
        transfer(cpi_context, amount)?;

        msg!("Deposited {} lamports to vault", amount);
        Ok(())
    }

    // INSTRUCTION 3: Withdraw
    // Transfers SOL from vault back to user
    // CONCEPT: PDA Signing - https://www.anchor-lang.com/docs/pdas
    // The vault is a PDA, so only the program can sign for it

    // REPLACE YOUR WITHDRAW FUNCTION WITH THIS
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(amount > 0, VaultError::InvalidAmount);

    let vault = &mut ctx.accounts.vault;
    let user = &mut ctx.accounts.user;

    // 1. Check vault has enough lamports
    // We access the account info directly to see the SOL balance
    let vault_lamports = vault.to_account_info().lamports();
    
    require!(
        vault_lamports >= amount,
        VaultError::InsufficientFunds
    );

    // 2. CONCEPT: Direct Lamport Manipulation
    // Since this program OWNS the vault PDA, we can modify its lamports directly.
    // We do not need a CPI to the System Program.
    
    // A. Subtract from Vault
    **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    
    // B. Add to User
    **user.to_account_info().try_borrow_mut_lamports()? += amount;

    msg!("Withdrawn {} lamports from vault", amount);
    Ok(())
}
}

// ACCOUNTS STRUCT 1: Initialize
// CONCEPT: #[derive(Accounts)] - https://www.anchor-lang.com/docs/the-accounts-struct
// This macro generates code to deserialize and validate accounts
#[derive(Accounts)]
pub struct Initialize<'info> {
    // CONCEPT: #[account(init)] - https://www.anchor-lang.com/docs/the-accounts-struct#init
    // Creates and initializes a new account
    // - init: creates the account
    // - payer: who pays for the account creation
    // - space: how much space to allocate (8 bytes for discriminator + account data)
    // - seeds/bump: PDA derivation
    #[account(
        init,
        payer = user,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    // CONCEPT: Signer - https://www.anchor-lang.com/docs/the-accounts-struct#signer
    // Ensures this account signed the transaction
    #[account(mut)]
    pub user: Signer<'info>,

    // CONCEPT: Program - https://www.anchor-lang.com/docs/the-accounts-struct#program
    // Validates that this is the System Program
    pub system_program: Program<'info, System>,
}

// ACCOUNTS STRUCT 2: Deposit
#[derive(Accounts)]
pub struct Deposit<'info> {
    // CONCEPT: PDA Validation - https://www.anchor-lang.com/docs/pdas
    // seeds and bump constraints verify this is the correct PDA
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


#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault.bump,
        // This ensures the person signing IS the person listed as owner in the vault data
        constraint = vault.owner == user.key() 
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// CONCEPT: #[account] macro - https://www.anchor-lang.com/docs/the-accounts-struct
// Marks a struct as an Anchor account type
// CONCEPT: InitSpace - https://www.anchor-lang.com/docs/space
// Automatically calculates the space needed for the account
#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub owner: Pubkey,  // 32 bytes
    pub bump: u8,       // 1 byte
    // Total: 33 bytes + 8 byte discriminator = 41 bytes
}

// CONCEPT: Custom Errors - https://www.anchor-lang.com/docs/errors
// Define custom error messages for better debugging
#[error_code]
pub enum VaultError {
    #[msg("Amount must be greater than 0")]
    InvalidAmount,
    #[msg("Insufficient funds in vault")]
    InsufficientFunds,
}
