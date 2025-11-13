use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::Escrow;
// use crate::errors::EscrowError; // Uncomment when you define errors

// ASSIGNMENT: Complete the cancel_escrow instruction
//
// This instruction allows the maker to cancel the escrow and get their funds back
// This should only be allowed BEFORE someone accepts the escrow
//
// REFERENCE: https://www.anchor-lang.com/docs/pdas

pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;

    // TODO 1: Add any additional validation
    // For example, check if escrow has expired (if you implemented expiration)
    // Or check the escrow status if you added that field

    // TODO 2: Transfer the escrowed funds back to the maker
    // The escrow PDA needs to sign this transaction
    //
    // HINT: Similar to accept_escrow, use CpiContext::new_with_signer
    //
    // let seeds = &[
    //     b"escrow",
    //     escrow.maker.as_ref(),
    //     // ... any other seeds ...
    //     &[escrow.bump],
    // ];
    // let signer_seeds = &[&seeds[..]];
    //
    // let transfer_accounts = Transfer {
    //     from: ???,
    //     to: ???,
    // };
    // let cpi_ctx = CpiContext::new_with_signer(???, transfer_accounts, signer_seeds);
    // transfer(cpi_ctx, escrow.maker_amount)?;

    // TODO 3: Log cancellation message
    // msg!("Escrow cancelled by maker. Refunded {} lamports", escrow.maker_amount);

    // NOTE: The escrow account will be closed and rent returned to maker
    // because of the close constraint in CancelEscrow struct

    Ok(())
}

// ASSIGNMENT: Complete the CancelEscrow accounts struct

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    // TODO 1: Define the escrow account
    // This account needs to:
    // - Be mutable (we're transferring from it)
    // - Be a PDA with matching seeds
    // - Be closed after execution (rent to maker)
    // - Verify the maker owns it (has_one constraint)
    //
    // #[account(
    //     mut,
    //     seeds = [???],
    //     bump = escrow.bump,
    //     close = ???,
    //     has_one = maker
    // )]
    // pub escrow: Account<'info, Escrow>,

    // TODO 2: Define the maker account
    // Must be a signer (only maker can cancel)
    // Must be mutable (receives the refund and rent)
    //
    // #[account(mut)]
    // pub maker: Signer<'info>,

    // TODO 3: Define the system program
    // pub system_program: Program<'info, System>,
}

// QUESTIONS FOR STUDENTS:
//
// 1. Why must the maker be a Signer for cancellation?
//    Answer: To prevent anyone else from cancelling the escrow and potentially
//    causing issues. Only the maker should be able to cancel their own escrow.
//
// 2. What prevents the maker from cancelling after the taker has accepted?
//    Answer: Once accept_escrow runs, the escrow account is closed. You can't
//    cancel an account that doesn't exist anymore.
//
// 3. Should we allow cancellation if a specific taker was set?
//    Answer: This is a design decision! Options:
//    - Always allow: Maker can always cancel (more control for maker)
//    - Never allow: Once created, it must be accepted or expire (more trustworthy)
//    - Conditional: Allow if not expired, or if taker hasn't shown interest
//
// 4. What happens to the rent when we close the account?
//    Answer: It's returned to the address specified in the close constraint.
//    We return it to the maker since they paid for the account creation.

// SECURITY CONSIDERATIONS:
//
// 1. AUTHORIZATION: Only maker can cancel (enforced by has_one and Signer)
// 2. RE-ENTRANCY: Not an issue in Solana, but worth noting
// 3. DOUBLE-SPEND: Can't cancel twice (account is closed after first cancel)
// 4. GRIEFING: Maker could cancel right before taker accepts (consider time locks)

// BONUS CHALLENGES:
//
// 1. Add a time lock: Maker can only cancel after X time has passed
//    (prevents immediate cancellation)
//
// 2. Add a penalty: If maker cancels, some funds go to a protocol fee
//    (discourages spam escrows)
//
// 3. Add expiration: Auto-cancel if escrow is past expiration date
//    (implement as a separate instruction or in cancel logic)
//
// 4. Emit a cancellation event for indexing

// DESIGN PATTERN: Why separate cancel from accept?
//
// We could have one "close_escrow" instruction that handles both accepting
// and cancelling based on who calls it. However, separating them:
//
// Pros:
// - Clearer intent and logic
// - Easier to understand and audit
// - Different validation rules for each
// - Better error messages
//
// Cons:
// - More code to maintain
// - Slightly larger program size
//
// For educational purposes and production quality, separate instructions are better.
