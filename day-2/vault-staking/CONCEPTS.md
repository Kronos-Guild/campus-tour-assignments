# Vault Staking Program - Day 2 Concepts

This document explains the new concepts introduced in Day 2, building on the simple vault from Day 1.

---

## Table of Contents
1. [What's New in Day 2](#whats-new-in-day-2)
2. [Modular Code Structure](#modular-code-structure)
3. [Staking Concepts](#staking-concepts)
4. [Time-Based Logic](#time-based-logic)
5. [Reward Calculation](#reward-calculation)
6. [Complete User Flow](#complete-user-flow)
7. [Code Organization](#code-organization)

---

## What's New in Day 2

### Day 1 Recap (Simple Vault)
- ✅ Initialize a vault (PDA)
- ✅ Deposit SOL
- ✅ Withdraw SOL
- **Total:** 3 instructions, 1 account type, ~200 lines

### Day 2 Additions (Staking + Rewards)
- ✅ **Stake** - Lock funds to earn rewards
- ✅ **Unstake** - Unlock funds with rewards
- ✅ **Time-based rewards** - Must stake for 60 seconds to get 10% rewards
- ✅ **Modular code** - Organized into separate files
- **Total:** 5 instructions, 1 extended account type, ~500 lines (but organized!)

---

## Modular Code Structure

### Why Modular?

In Day 1, everything was in one file (`lib.rs`). This works for small programs but becomes hard to manage as programs grow.

**Day 1 Structure (Monolithic):**
```
programs/vault/src/
└── lib.rs  (200 lines - everything here)
```

**Day 2 Structure (Modular):**
```
programs/vault-staking/src/
├── lib.rs              (Entry point, ~70 lines)
├── state.rs            (Account structs, ~60 lines)
├── errors.rs           (Custom errors, ~45 lines)
├── constants.rs        (Configuration, ~25 lines)
└── instructions/       (Instruction logic)
    ├── mod.rs          (Module exports)
    ├── initialize.rs   (~50 lines)
    ├── deposit.rs      (~50 lines)
    ├── withdraw.rs     (~70 lines)
    ├── stake.rs        (~75 lines)
    └── unstake.rs      (~100 lines)
```

### Benefits of Modular Structure

1. **Easier to Navigate** - Know exactly where to find code
2. **Better Organization** - Related code stays together
3. **Easier to Test** - Can test modules independently
4. **Team-Friendly** - Multiple developers can work on different modules
5. **Scales Better** - Can add features without bloating lib.rs
6. **Professional** - This is how real Solana programs are structured

### How Modules Work

**Reference:** [Rust Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)

```rust
// lib.rs declares modules
mod state;           // Tells Rust that state.rs exists
mod errors;          // Tells Rust that errors.rs exists
mod instructions;    // Tells Rust that instructions/ folder exists

// Then we can use them
use instructions::*; // Import everything from instructions
```

**instructions/mod.rs acts as the folder's entry point:**
```rust
pub mod initialize;  // Declares initialize.rs
pub mod stake;       // Declares stake.rs

pub use initialize::*; // Re-exports everything
pub use stake::*;      // So we can use them elsewhere
```

---

## Staking Concepts

### What is Staking?

**Staking** = Locking funds for a period of time to earn rewards.

Think of it like:
- A bank savings account (lock money, earn interest)
- A certificate of deposit (CD)
- Yield farming in DeFi

### Why Stake?

**For Users:**
- Earn passive rewards
- Support the protocol
- Get additional benefits (voting, etc.)

**For Protocols:**
- Lock up liquidity (reduce volatility)
- Reward loyal users
- Incentivize long-term participation

### Our Simple Staking Model

```
Stake 100 SOL for 60+ seconds = Get back 110 SOL (10% reward)
Stake 100 SOL for <60 seconds  = Get back 100 SOL (no reward)
```

**Rules:**
- Minimum stake duration: **60 seconds**
- Reward rate: **10%** (1000 basis points)
- Only one active stake at a time (simplified for learning)
- Rewards are "printed" (added to balance) on unstake

---

## Time-Based Logic

### The Clock Sysvar

**What is a Sysvar?**
Sysvars (system variables) are special accounts that provide runtime information.

**Reference:** [Solana Sysvars](https://docs.solana.com/developing/runtime-facilities/sysvars)

**Clock Sysvar provides:**
- `unix_timestamp` - Seconds since Jan 1, 1970 (Unix epoch)
- `slot` - Current slot number
- `epoch` - Current epoch

**Using Clock in Anchor:**
```rust
let clock = Clock::get()?;
let current_time = clock.unix_timestamp;  // i64
```

### Why Unix Timestamp?

✅ **Deterministic** - All validators agree on the value
✅ **Familiar** - Standard time format
✅ **Easy to work with** - Simple integer math

❌ **Not perfectly accurate** - Can be off by seconds
❌ **Can't use for high-precision timing**

For staking rewards (measured in minutes/hours/days), this is perfect!

### Calculating Duration

```rust
// When staking starts
vault.stake_timestamp = clock.unix_timestamp;  // Example: 1699900000

// When checking duration
let current_time = clock.unix_timestamp;       // Example: 1699900065
let duration = current_time - vault.stake_timestamp;  // 65 seconds

// Check if minimum met
if duration >= 60 {
    // User gets rewards!
}
```

---

## Reward Calculation

### The Problem: No Floating Point!

Solana programs **cannot use** `f32` or `f64` (floating point numbers).

**Why?**
- Floating point is non-deterministic
- Different CPUs might give slightly different results
- Validators must agree exactly on all calculations

**So how do we calculate 10% rewards?**

### Solution: Basis Points (Fixed-Point Math)

**Basis Points** = Integer representation of percentages

```
1 basis point = 0.01%
100 basis points = 1%
1000 basis points = 10%
10000 basis points = 100%
```

**In our code:**
```rust
// constants.rs
pub const REWARD_RATE_BASIS_POINTS: u64 = 1000;  // 10%
pub const BASIS_POINTS_DIVISOR: u64 = 10000;     // 100%

// Calculation
let reward = (staked_amount * REWARD_RATE_BASIS_POINTS) / BASIS_POINTS_DIVISOR;

// Example: 1000 SOL staked
// reward = (1000 * 1000) / 10000 = 100 SOL  ✅
```

### Preventing Overflow

When multiplying large numbers, we can overflow `u64`.

**Example:**
```rust
let big_number: u64 = u64::MAX;  // 18,446,744,073,709,551,615
let result = big_number * 1000;   // OVERFLOW! 💥
```

**Solution: Use u128 for intermediate calculations**
```rust
let reward = (staked_amount as u128)  // Cast to larger type
    .checked_mul(REWARD_RATE_BASIS_POINTS as u128)  // Safe multiply
    .ok_or(VaultError::ArithmeticOverflow)?
    .checked_div(BASIS_POINTS_DIVISOR as u128)      // Safe divide
    .ok_or(VaultError::ArithmeticOverflow)?
    as u64;  // Cast back to u64
```

**Why this works:**
- `u128` is 2x the size of `u64`
- We have plenty of room for the multiplication
- After division, the result fits back in `u64`
- `checked_mul`/`checked_div` return `None` on overflow
- Transaction fails safely instead of producing wrong results

### Checked Math Methods

**Reference:** [Rust checked arithmetic](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add)

```rust
// Regular math (can panic on overflow)
let result = a + b;  // ❌ Dangerous in smart contracts

// Checked math (returns Option)
let result = a.checked_add(b).ok_or(Error::Overflow)?;  // ✅ Safe

// Saturating math (clamps at min/max)
let result = a.saturating_sub(b);  // If b > a, returns 0

// Available methods:
checked_add       // Returns None on overflow
checked_sub       // Returns None on underflow
checked_mul       // Returns None on overflow
checked_div       // Returns None if dividing by 0
saturating_add    // Clamps at max
saturating_sub    // Clamps at 0 (for unsigned)
```

**Always use checked/saturating math in smart contracts!**

---

## Complete User Flow

### Scenario: Toby Wants to Earn Staking Rewards

**1. Initialize Vault**
```
Action: Toby calls initialize()
Result: Vault PDA created
State:
  - owner: Toby
  - total_deposited: 0
  - staked_amount: 0
  - is_staked: false
```

**2. Deposit SOL**
```
Action: Toby calls deposit(1000 SOL)
Result: 1000 SOL transferred to vault
State:
  - total_deposited: 1000
  - staked_amount: 0
  - is_staked: false
```

**3. Stake Funds**
```
Action: Toby calls stake(1000)
Time: 12:00:00 PM (timestamp: 1699900000)
Result: Funds locked for staking
State:
  - total_deposited: 1000
  - staked_amount: 1000
  - stake_timestamp: 1699900000
  - is_staked: true
```

**4a. Try to Unstake Early (Before 60 seconds)**
```
Action: Toby calls unstake() at 12:00:30 PM (30 seconds later)
Duration: 30 seconds
Result: ❌ No rewards (didn't meet 60 second minimum)
State:
  - total_deposited: 1000 (no reward added)
  - staked_amount: 0
  - is_staked: false
```

**4b. Unstake After Duration (After 60 seconds)**
```
Action: Toby calls unstake() at 12:01:05 PM (65 seconds later)
Duration: 65 seconds
Calculation: reward = (1000 * 1000) / 10000 = 100 SOL
Result: ✅ Earned 100 SOL reward!
State:
  - total_deposited: 1100 (original + reward)
  - staked_amount: 0
  - is_staked: false
```

**5. Withdraw**
```
Action: Toby calls withdraw(1100)
Result: 1100 SOL sent to Toby's wallet
State:
  - total_deposited: 0
  - staked_amount: 0
```

### State Transitions Diagram

```
┌─────────────┐
│ Initialize  │
│ (empty)     │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Deposit    │
│ (unstaked)  │
└──────┬──────┘
       │
       ▼
┌─────────────┐  ◄── Can withdraw
│   Staked    │      unstaked funds
│ (locked)    │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Unstake    │
│ (+ rewards) │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Withdraw   │
│ (to wallet) │
└─────────────┘
```

---

## Code Organization

### File Structure Breakdown

#### 1. **lib.rs** (Entry Point)
- Declares modules
- Defines program instructions
- Delegates to instruction handlers
- **70 lines** - clean and focused

#### 2. **state.rs** (Data Structures)
- `Vault` account struct
- Helper methods (get_stake_duration, etc.)
- **60 lines**

#### 3. **errors.rs** (Error Definitions)
- All custom error types
- Clear error messages
- **45 lines**

#### 4. **constants.rs** (Configuration)
- `MIN_STAKE_DURATION`
- `REWARD_RATE_BASIS_POINTS`
- `BASIS_POINTS_DIVISOR`
- **25 lines**

#### 5. **instructions/** (Business Logic)

**initialize.rs** (~50 lines)
- Creates vault PDA
- Initializes all fields
- Sets up staking state

**deposit.rs** (~50 lines)
- Transfers SOL to vault
- Updates total_deposited
- Uses checked math

**withdraw.rs** (~70 lines)
- Checks unstaked balance
- PDA signing
- Transfers SOL back to user

**stake.rs** (~75 lines)
- Locks funds for staking
- Records timestamp
- Validates available balance

**unstake.rs** (~100 lines)
- Checks duration
- Calculates rewards
- Updates vault state
- Conditional reward distribution

---

## Key Differences: Day 1 vs Day 2

| Aspect | Day 1 | Day 2 |
|--------|-------|-------|
| **Structure** | Monolithic (1 file) | Modular (7 files) |
| **Lines of Code** | ~200 | ~500 |
| **Instructions** | 3 | 5 |
| **Account Fields** | 2 | 6 |
| **New Concepts** | PDAs, CPI | Time, Rewards, Modules |
| **Math** | Simple | Checked, Basis Points |
| **State Tracking** | Basic | Advanced (staking state) |

---

## Advanced Topics (Not Implemented, But Worth Knowing)

### Multiple Stakes
Our simple version only allows one stake at a time. Advanced versions could:
- Allow multiple concurrent stakes
- Different lock periods (30 days, 90 days, etc.)
- Different reward rates based on duration

### Reward Pools
Our rewards are "printed". Production systems use:
- Pre-funded reward pools
- Check pool balance before distributing
- Admin functions to fund the pool

### Compounding
Instead of claiming rewards, automatically re-stake them:
- Compound interest effect
- Better for long-term stakers

### Slashing
Penalize early withdrawal:
- Lose X% if unstake early
- Encourages longer staking

### Governance
Give stakers voting power:
- Staked amount = voting weight
- Participate in protocol decisions

---

## Security Considerations

### 1. Checked Math
✅ Always use `checked_add`, `checked_mul`, etc.
❌ Never use regular `+`, `*` that can overflow

### 2. Time Manipulation
✅ Clock timestamp is controlled by validators (can't fake it)
❌ Don't use slot numbers for precise timing

### 3. State Validation
✅ Check `is_staked` before operations
✅ Verify minimum duration
❌ Don't trust user input

### 4. Account Ownership
✅ Use `has_one` constraint
✅ Verify PDAs with `seeds` and `bump`
❌ Don't let users withdraw from other vaults

### 5. Reward Calculation
✅ Use `u128` for intermediate calculations
✅ Check for overflow/underflow
❌ Don't use floating point

---

## Learning Objectives Achieved

After Day 2, you understand:

✅ **Modular Code Structure** - How to organize a Solana program
✅ **Time-Based Logic** - Using Clock sysvar for duration checks
✅ **Reward Calculations** - Basis points and fixed-point math
✅ **State Management** - Tracking staking state across instructions
✅ **Checked Math** - Preventing overflow/underflow vulnerabilities
✅ **Business Logic** - Implementing conditional rewards
✅ **Code Organization** - Professional program architecture

---

## Resources

- **Anchor Documentation:** [https://www.anchor-lang.com/docs](https://www.anchor-lang.com/docs)
- **Solana Sysvars:** [https://docs.solana.com/developing/runtime-facilities/sysvars](https://docs.solana.com/developing/runtime-facilities/sysvars)
- **Rust Modules:** [https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- **Checked Arithmetic:** [https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add)

---

## Questions for Your Presentation

1. **Why can't we use floating point in Solana programs?**
   - Non-deterministic across different hardware

2. **How do basis points solve the decimal problem?**
   - Integer math to represent percentages

3. **Why do we cast to u128 for calculations?**
   - Prevent overflow in intermediate steps

4. **What happens if someone tries to unstake at 59 seconds?**
   - No rewards, but they get their principal back

5. **Why separate code into modules?**
   - Easier to maintain, test, and collaborate

6. **Could this reward system be exploited?**
   - Not easily - time is controlled by validators
   - But in production, you'd need a funded reward pool

7. **How would you implement a reward pool?**
   - Additional PDA to hold reward funds
   - Check balance before distributing
   - Admin function to fund it

---

**Great work on Day 2!** 🚀

You've learned professional code organization and time-based logic - essential skills for building real Solana programs!
