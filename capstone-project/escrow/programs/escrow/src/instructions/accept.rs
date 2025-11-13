use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::Escrow;
// use crate::errors::EscrowError; // Uncomment when you define errors

// ASSIGNMENT: Complete the accept_escrow instruction
//
// This instruction allows a taker to accept the escrow by:
// 1. Validating the taker is authorized (if specific taker was set)
// 2. Transferring taker's amount to the maker
// 3. Transferring escrow's amount to the taker
// 4. Closing the escrow account
//
// REFERENCE: https://www.anchor-lang.com/docs/cross-program-invocations

pub fn accept_escrow(ctx: Context<AcceptEscrow>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;

    // TODO 1: If a specific taker was set, validate that the signer is that taker
    // HINT: Check if escrow.taker is Some and matches taker.key()
    // if let Some(expected_taker) = escrow.taker {
    //     require!(
    //         expected_taker == ctx.accounts.taker.key(),
    //         EscrowError::UnauthorizedTaker
    //     );
    // }

    // TODO 2: Transfer taker_amount from taker to maker
    // This is the taker's part of the swap
    // HINT: Use transfer CPI similar to initialize
    // let transfer_to_maker = Transfer {
    //     from: ???,
    //     to: ???,
    // };
    // let cpi_ctx = CpiContext::new(???, transfer_to_maker);
    // transfer(cpi_ctx, escrow.taker_amount)?;

    // TODO 3: Transfer maker_amount from escrow PDA to taker
    // This is the maker's part of the swap (escrowed funds)
    // IMPORTANT: The escrow PDA must SIGN this transaction!
    //
    // HINT: Use CpiContext::new_with_signer with the escrow's seeds
    // You'll need to reconstruct the seeds and bump used to create the PDA
    //
    // let seeds = &[
    //     b"escrow",
    //     escrow.maker.as_ref(),
    //     // ... any other seeds you used ...
    //     &[escrow.bump],
    // ];
    // let signer_seeds = &[&seeds[..]];
    //
    // let transfer_to_taker = Transfer {
    //     from: ???,
    //     to: ???,
    // };
    // let cpi_ctx = CpiContext::new_with_signer(???, transfer_to_taker, signer_seeds);
    // transfer(cpi_ctx, escrow.maker_amount)?;

    // TODO 4: Log success message
    // msg!("Escrow accepted! Swapped {} for {} lamports", escrow.maker_amount, escrow.taker_amount);

    // NOTE: The escrow account will be automatically closed because of the
    // close constraint in the AcceptEscrow struct. The remaining lamports
    // will be transferred to the maker.

    Ok(())
}

// ASSIGNMENT: Complete the AcceptEscrow accounts struct

#[derive(Accounts)]
pub struct AcceptEscrow<'info> {
    // TODO 1: Define the escrow account
    // This account needs to:
    // - Be mutable (we're transferring from it)
    // - Be a PDA with the same seeds used in initialization
    // - Be CLOSED after the instruction (return rent to maker)
    // - Verify it belongs to the correct maker (has_one constraint)
    //
    // HINT: Use #[account(mut, seeds = [...], bump = escrow.bump, close = maker, has_one = maker)]
    //
    // #[account(
    //     mut,
    //     seeds = [???],
    //     bump = escrow.bump,
    //     close = ???,  // Who receives the rent back?
    //     has_one = maker
    // )]
    // pub escrow: Account<'info, Escrow>,

    // TODO 2: Define the maker account (receives taker's SOL and rent refund)
    // #[account(mut)]
    // pub maker: ???,

    // TODO 3: Define the taker account (signs and pays)
    // #[account(mut)]
    // pub taker: Signer<'info>,

    // TODO 4: Define the system program (for transfers)
    // pub system_program: Program<'info, System>,
}

// QUESTIONS FOR STUDENTS:
//
// 1. Why do we need to use new_with_signer for the escrow → taker transfer?
//    Answer: The escrow is a PDA with no private key. Only the program can
//    sign for it by providing the seeds that derive its address.
//
// 2. What would happen if we didn't verify the taker when one is specified?
//    Answer: Anyone could accept the escrow, not just the intended taker.
//    This breaks the trust model if the maker wanted a specific counterparty.
//
// 3. Why do we close the escrow account at the end?
//    Answer: To reclaim the rent (storage cost) and prevent the account from
//    being used again. This is a security best practice.
//
// 4. What's the order of operations and why does it matter?
//    Answer:
//    1. Taker → Maker (taker sends their funds)
//    2. Escrow → Taker (escrow releases maker's funds)
//    This ensures both sides of the swap happen atomically. If the first
//    transfer succeeds but the second fails, the whole transaction reverts.

// SECURITY CONSIDERATIONS:
//
// 1. ATOMIC SWAPS: Both transfers must succeed or both fail (transaction atomicity)
// 2. RE-ENTRANCY: Not a concern in Solana (no callbacks), but good to understand
// 3. FRONT-RUNNING: Taker could be front-run by another taker if none specified
// 4. VALIDATION: Always verify the taker is authorized if specified

// BONUS CHALLENGES:
//
// 1. Add support for partial fills (accept only part of the escrow)
// 2. Add an expiration check (escrow can only be accepted before expiry)
// 3. Add a fee that goes to a protocol treasury
// 4. Emit an event when escrow is accepted (for indexing)
