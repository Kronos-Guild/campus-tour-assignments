// CONCEPT: Module Organization
// Reference: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
// This file declares and re-exports all instruction modules
// It acts as the "public interface" for the instructions folder

// Declare modules (tells Rust these files exist)
pub mod initialize;
pub mod deposit;
pub mod withdraw;
pub mod stake;
pub mod unstake;

// Re-export everything from each module
// This allows: use crate::instructions::*;
// Instead of: use crate::instructions::initialize::Initialize;
pub use initialize::*;
pub use deposit::*;
pub use withdraw::*;
pub use stake::*;
pub use unstake::*;

// CONCEPT: Why This Pattern?
// Benefits of using mod.rs:
// 1. Clean imports in lib.rs: just `use instructions::*;`
// 2. Easy to add new instructions: just add one line here
// 3. Keeps related code together (all instructions in one folder)
// 4. Standard Rust project structure
//
// Without this, you'd need to list every instruction in lib.rs:
// mod instructions {
//     pub mod initialize;
//     pub mod deposit;
//     // ... etc
// }
