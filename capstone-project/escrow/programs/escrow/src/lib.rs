// CAPSTONE PROJECT: Simple Escrow Program
//
// This is an assignment template. You need to complete the TODOs throughout
// the codebase to create a working escrow program.
//
// WHAT IS AN ESCROW?
// An escrow is a smart contract that holds funds from two parties and
// atomically swaps them. Both parties get what they want, or nobody does.
//
// EXAMPLE USE CASE:
// - Toby has 10 SOL and wants to trade it for something
// - Alejandro wants to pay for Toby's 10 SOL
// - Toby creates an escrow offering 10 SOL for 5 SOL
// - Alejandro accepts the escrow with his 5 SOL
// - The swap happens atomically: Toby gets 5 SOL, Alejandro gets 10 SOL
//
// LEARNING OBJECTIVES:
// - Modular program structure
// - PDAs and signer seeds
// - Cross-program invocations (CPI)
// - Account validation and constraints
// - Error handling
// - Security best practices

use anchor_lang::prelude::*;

// TODO: Uncomment these when you create the files
// mod state;
// mod errors;
// mod constants;
// mod instructions;

// TODO: Uncomment this when you implement instructions
// use instructions::*;

// Program ID - this will be different when you build
declare_id!("DpyyAnbMVMR5rwBuRWntUGq9RMM9gmyEKDjrUtc6pTvw");

#[program]
pub mod escrow {
    use super::*;

    // INSTRUCTION 1: Initialize Escrow
    // Creates a new escrow with maker's offer
    //
    // TODO: Implement this instruction
    // pub fn initialize_escrow(
    //     ctx: Context<InitializeEscrow>,
    //     maker_amount: u64,
    //     taker_amount: u64,
    //     taker: Option<Pubkey>,
    // ) -> Result<()> {
    //     instructions::initialize::initialize_escrow(ctx, maker_amount, taker_amount, taker)
    // }

    // INSTRUCTION 2: Accept Escrow
    // Allows a taker to accept the escrow offer
    //
    // TODO: Implement this instruction
    // pub fn accept_escrow(ctx: Context<AcceptEscrow>) -> Result<()> {
    //     instructions::accept::accept_escrow(ctx)
    // }

    // INSTRUCTION 3: Cancel Escrow
    // Allows maker to cancel and get funds back
    //
    // TODO: Implement this instruction
    // pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
    //     instructions::cancel::cancel_escrow(ctx)
    // }

    // BONUS INSTRUCTION IDEAS:
    //
    // 1. update_escrow: Allow maker to update terms before acceptance
    // 2. extend_expiration: Allow maker to extend the expiration date
    // 3. partial_accept: Allow partial fills of the escrow
}

// PROGRAM FLOW:
//
// Happy Path:
// 1. Maker calls initialize_escrow(10 SOL, 5 SOL)
//    → Escrow PDA created, 10 SOL deposited
// 2. Taker calls accept_escrow()
//    → Taker sends 5 SOL to maker
//    → Escrow sends 10 SOL to taker
//    → Escrow account closed
//
// Cancellation Path:
// 1. Maker calls initialize_escrow(10 SOL, 5 SOL)
//    → Escrow PDA created, 10 SOL deposited
// 2. Maker calls cancel_escrow()
//    → 10 SOL returned to maker
//    → Escrow account closed
//
// SECURITY CONSIDERATIONS:
//
// 1. Atomic Swaps: Both transfers must succeed or both fail
// 2. PDA Signing: Only the program can sign for the escrow PDA
// 3. Authorization: Verify maker/taker identities
// 4. Validation: Check all amounts and addresses
// 5. Account Closure: Always close accounts after use to reclaim rent
//
// TESTING STRATEGY:
//
// You should test:
// 1. Happy path: create → accept
// 2. Cancellation: create → cancel
// 3. Errors:
//    - Invalid amounts (0 or too large)
//    - Wrong taker trying to accept
//    - Non-maker trying to cancel
//    - Double accept/cancel
// 4. Edge cases:
//    - Expired escrows
//    - Minimum/maximum amounts
//
// RESOURCES:
//
// - Anchor Book: https://www.anchor-lang.com/docs
