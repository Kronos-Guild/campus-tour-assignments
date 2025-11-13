use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::Escrow;
// use crate::errors::EscrowError; // Uncomment when you define errors
// use crate::constants::*; // Uncomment when you define constants

// ASSIGNMENT: Complete the initialize_escrow instruction
//
// This instruction creates a new escrow where:
// 1. The maker deposits their offered amount
// 2. The escrow account is created as a PDA
// 3. Escrow details are stored
//
// REFERENCE: https://www.anchor-lang.com/docs/the-accounts-struct

pub fn initialize_escrow(
    ctx: Context<InitializeEscrow>,
    maker_amount: u64,
    taker_amount: u64,
    taker: Option<Pubkey>, // Optional: specific taker or any taker
) -> Result<()> {
    // TODO 1: Validate that maker_amount is greater than 0
    // HINT: Use require! macro with your custom error
    // Example: require!(maker_amount > 0, EscrowError::InvalidAmount);

    // TODO 2: Validate that taker_amount is greater than 0
    // HINT: Same as above

    // TODO 3: Transfer SOL from maker to the escrow PDA
    // HINT: Use the transfer CPI (Cross-Program Invocation)
    // You'll need to:
    // 1. Create a Transfer struct with from and to accounts
    // 2. Create a CpiContext
    // 3. Call transfer() function
    //
    // REFERENCE: https://www.anchor-lang.com/docs/cross-program-invocations
    //
    // let transfer_accounts = Transfer {
    //     from: ???,
    //     to: ???,
    // };
    // let cpi_ctx = CpiContext::new(???, transfer_accounts);
    // transfer(cpi_ctx, maker_amount)?;

    // TODO 4: Initialize the escrow account fields
    // let escrow = &mut ctx.accounts.escrow;
    // escrow.maker = ???;
    // escrow.taker = ???;
    // escrow.maker_amount = ???;
    // escrow.taker_amount = ???;
    // escrow.bump = ???;

    // TODO 5: Set the created_at timestamp
    // HINT: Use Clock::get()?.unix_timestamp
    // escrow.created_at = ???;

    // TODO 6: Log a success message
    // HINT: Use msg! macro
    // msg!("Escrow created: maker offers {} lamports for {} lamports", maker_amount, taker_amount);

    Ok(())
}

// ASSIGNMENT: Complete the InitializeEscrow accounts struct
//
// REFERENCE: https://www.anchor-lang.com/docs/the-accounts-struct

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    // TODO 1: Define the escrow PDA account
    // This account needs to:
    // - Be initialized (init)
    // - Use the maker and a unique seed for PDA derivation
    // - Have proper space allocated
    // - Specify who pays for the account creation (payer)
    //
    // HINT: Use the #[account(init, payer = maker, space = ..., seeds = [...], bump)]
    // The seeds could be: [b"escrow", maker.key().as_ref(), /* maybe a unique id? */]
    //
    // #[account(
    //     init,
    //     payer = ???,
    //     space = 8 + Escrow::INIT_SPACE,
    //     seeds = [???],
    //     bump
    // )]
    // pub escrow: Account<'info, Escrow>,

    // TODO 2: Define the maker account (signer and mut)
    // This account needs to:
    // - Sign the transaction
    // - Be mutable (will transfer SOL)
    //
    // #[account(mut)]
    // pub maker: Signer<'info>,

    // TODO 3: Define the system program
    // HINT: This is needed for account creation and SOL transfers
    // pub system_program: Program<'info, System>,
}

// QUESTIONS FOR STUDENTS:
//
// 1. Why do we need a unique seed for each escrow?
//    Answer: To allow multiple escrows per maker. Each escrow needs a unique address.
//
// 2. What happens if we don't mark the maker as mut?
//    Answer: Transaction will fail because we're transferring SOL from their account.
//
// 3. Why do we store maker_amount in the escrow account?
//    Answer: We need to know how much to return if the escrow is cancelled,
//    and we need to verify amounts during acceptance.
//
// 4. What is the space calculation for the escrow account?
//    Answer: 8 (discriminator) + 32 (maker) + 32 (taker option) + 8 (maker_amount) +
//    8 (taker_amount) + 1 (bump) + 8 (created_at) = 97 bytes (adjust based on your struct)

// DESIGN DECISION:
// Should we allow taker to be None (any taker) or require a specific taker?
//
// Option 1: Specific taker only (more secure, less flexible)
// - Pro: Maker knows exactly who they're trading with
// - Con: Less liquid, requires coordination
//
// Option 2: Any taker allowed (less secure, more flexible)
// - Pro: More liquid, faster trades
// - Con: Maker doesn't control who accepts
//
// Option 3: Optional taker (what we're doing - best of both worlds!)
// - Pro: Maker can choose security vs flexibility
// - Con: More complex logic
