// CONCEPT: Modular Program Structure
// Reference: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
//
// Instead of putting all code in lib.rs, we organize into modules:
// - state.rs: Account structs
// - errors.rs: Custom errors
// - constants.rs: Configuration values
// - instructions/: Each instruction in its own file
//
// Benefits:
// 1. Easier to navigate and find code
// 2. Better separation of concerns
// 3. Easier to test individual components
// 4. Scales better as program grows
// 5. Multiple developers can work on different modules

use anchor_lang::prelude::*;

// Import our modules
mod state;
mod errors;
mod constants;
mod instructions;

// Make them available to use in this file
use instructions::*;

// CONCEPT: Program ID
// This will be different from day-1 since it's a new program
declare_id!("mkZJVMPpBYQhUjnhXKWMbuaVsEpGtGdTFa3BbRE1VU1");

// CONCEPT: Program Module
// Each public function becomes an instruction callable from clients
#[program]
pub mod vault_staking {
    use super::*;

    // DAY 1 INSTRUCTIONS (Original Vault)

    /// Creates a new vault account for a user
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::initialize(ctx)
    }

    /// Deposits SOL from user wallet to vault
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::deposit(ctx, amount)
    }

    /// Withdraws unstaked SOL from vault to user wallet
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw::withdraw(ctx, amount)
    }

    // DAY 2 INSTRUCTIONS (New - Staking)

    /// Stakes deposited funds to earn rewards
    /// Must be staked for at least 60 seconds to earn 10% rewards
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake::stake(ctx, amount)
    }

    /// Unstakes funds and distributes rewards if eligible
    /// Rewards: 10% if staked for 60+ seconds, 0% if less
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        instructions::unstake::unstake(ctx)
    }
}

// CONCEPT: Why This Pattern?
//
// Before (Day 1 - Monolithic):
// - All code in one file (lib.rs)
// - ~200 lines becomes ~800 lines with staking
// - Hard to find specific functionality
// - Difficult to maintain
//
// After (Day 2 - Modular):
// - lib.rs is just 70 lines (entry point)
// - Each instruction ~50-80 lines
// - Easy to find and modify specific features
// - Can test modules independently
// - Professional structure
//
// This is how real-world Solana programs are organized!

// CONCEPT: Instruction Delegation Pattern
// Notice how each instruction in #[program] just calls a function from instructions/
// This keeps lib.rs clean and delegates work to specialized modules
//
// Example flow:
// 1. Client calls program.methods.stake()
// 2. Routes to pub fn stake() in #[program] module
// 3. Delegates to instructions::stake::stake()
// 4. That function contains the actual logic
//
// Benefits:
// - lib.rs stays small and readable
// - Business logic is in dedicated files
// - Easier to write unit tests for each instruction
