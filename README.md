# Solana Campus Tour - Programming Assignments

A comprehensive learning path for building Solana programs with Anchor Framework. Progress from basic vault operations to advanced staking mechanisms, culminating in a capstone escrow project.

---

## 📚 Course Structure

This repository contains three progressive learning modules:

```
campus-tour-assignments/
├── day-1/           # Introduction to Solana Programs
├── day-2/           # Intermediate: Staking & Modular Design
└── capstone-project/ # Final Assignment: Escrow Program
```

---

## 📖 Day 1: Simple Vault Program

**Location:** `day-1/vault/`

**Status:** ✅ Complete working example

**Duration:** 2-3 hours

### What You'll Learn

Build a simple vault program that allows users to:
- Create a personal vault (PDA)
- Deposit SOL into the vault
- Withdraw SOL from the vault

### Core Concepts

- **PDAs (Program Derived Addresses)** - Creating deterministic account addresses
- **CPI (Cross-Program Invocations)** - Calling the System Program
- **Account Constraints** - Using Anchor's validation system
- **PDA Signing** - How programs sign for PDAs
- **Custom Errors** - Defining error types

### Project Structure

```
day-1/vault/
├── programs/vault/src/
│   └── lib.rs              # All code in one file (monolithic)
├── CONCEPTS.md             # Detailed concept explanations
└── README.md               # Setup and deployment guide
```

### Key Features

- 3 instructions: `initialize`, `deposit`, `withdraw`
- Monolithic structure (all code in lib.rs)
- Fully documented with inline comments
- Complete working example

### Program Flow Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        DAY 1: VAULT                          │
└─────────────────────────────────────────────────────────────┘

User Actions                    Vault State
─────────────                   ───────────

1. Initialize
   ┌──────┐
   │ User │ ──────────────────> [Vault PDA Created]
   └──────┘                     owner: User
                                bump: 254
                                ↓
2. Deposit 10 SOL
   ┌──────┐     10 SOL
   │ User │ ──────────────────> [Vault PDA]
   └──────┘                     Balance: 10 SOL
                                ↓
3. Withdraw 5 SOL
   ┌──────┐  <────── 5 SOL ──  [Vault PDA]
   │ User │                     Balance: 5 SOL
   └──────┘

Concepts: PDAs, CPI, Account Constraints
```

### Learning Outcomes

By the end of Day 1, you will understand:
- ✅ How to create and use PDAs
- ✅ How to transfer SOL using CPI
- ✅ How Anchor validates accounts
- ✅ Basic Solana program structure

---

## 📖 Day 2: Vault Staking Program

**Location:** `day-2/vault-staking/`

**Status:** ✅ Complete working example with tests

**Duration:** 4-6 hours

### What You'll Build

Extend the Day 1 vault with staking functionality:
- Stake deposited SOL for rewards
- Earn 10% rewards if staked for 60+ seconds
- No rewards if unstaked early
- Modular, production-ready code structure

### New Concepts

- **Modular Architecture** - Organizing code into separate files
- **Time-Based Logic** - Using Clock sysvar for timestamps
- **Fixed-Point Math** - Basis points for percentage calculations
- **Checked Arithmetic** - Safe math to prevent overflows
- **State Management** - Tracking staking state across instructions

### Project Structure

```
day-2/vault-staking/
├── programs/vault-staking/src/
│   ├── lib.rs              # Entry point (~70 lines)
│   ├── state.rs            # Vault account struct
│   ├── errors.rs           # Custom error types
│   ├── constants.rs        # Configuration values
│   └── instructions/       # Each instruction in its own file
│       ├── mod.rs
│       ├── initialize.rs
│       ├── deposit.rs
│       ├── withdraw.rs
│       ├── stake.rs
│       └── unstake.rs
├── tests/
│   └── vault-staking.ts    # Comprehensive test suite (11 tests)
├── CONCEPTS.md             # In-depth explanations
└── README.md               # Build, deploy, and test guide
```

### Key Features

- 5 instructions: `initialize`, `deposit`, `withdraw`, `stake`, `unstake`
- Modular structure (professional organization)
- Time-based reward distribution
- Comprehensive test suite with error cases
- Devnet deployment instructions

### Staking Rules

```
Stake for 60+ seconds = Get 10% rewards ✅
Stake for <60 seconds = Get 0% rewards (no penalty)
```

### Program Flow Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                  DAY 2: VAULT STAKING                        │
└─────────────────────────────────────────────────────────────┘

User Actions                    Vault State
─────────────                   ───────────

1. Initialize + Deposit 10 SOL
   ┌──────┐     10 SOL
   │ User │ ──────────────────> [Vault PDA]
   └──────┘                     total_deposited: 10 SOL
                                staked_amount: 0
                                is_staked: false
                                ↓
2. Stake 10 SOL (Lock funds)
   ┌──────┐
   │ User │ ──────────────────> [Vault PDA - LOCKED]
   └──────┘                     total_deposited: 10 SOL
                                staked_amount: 10 SOL
                                is_staked: true
                                stake_timestamp: T0
                                ↓
                        ⏰ Wait 60+ seconds...
                                ↓
3. Unstake (Claim rewards)
   ┌──────┐
   │ User │ ──────────────────> [Vault PDA - UNLOCKED]
   └──────┘                     total_deposited: 11 SOL ✅ +10%
                                staked_amount: 0
                                is_staked: false
                                ↓
4. Withdraw 11 SOL
   ┌──────┐  <───── 11 SOL ──  [Vault PDA]
   │ User │                     total_deposited: 0
   └──────┘

Concepts: Time Logic, Basis Points, Checked Math, Modular Code
```

### Learning Outcomes

By the end of Day 2, you will understand:
- ✅ How to organize a Solana program professionally
- ✅ How to implement time-based logic
- ✅ How to calculate rewards without floating point
- ✅ How to write comprehensive tests
- ✅ How to deploy to devnet

### Comparison: Day 1 vs Day 2

| Aspect | Day 1 | Day 2 |
|--------|-------|-------|
| **Structure** | Monolithic (1 file) | Modular (7 files) |
| **Lines of Code** | ~200 | ~500 |
| **Instructions** | 3 | 5 |
| **Concepts** | PDAs, CPI | + Time, Rewards, Modules |
| **Tests** | None | 11 comprehensive tests |
| **Complexity** | Beginner | Intermediate |

---

## 📖 Capstone: Escrow Program

**Location:** `capstone-project/escrow/`

**Status:** 🚧 Assignment template (students complete this)

**Duration:** 10-15 hours

### What You'll Build

A simple SOL escrow program for atomic swaps:
- Maker offers X SOL for Y SOL
- Taker accepts the offer
- Atomic swap: both get funds or neither does
- Maker can cancel before acceptance

### Assignment Structure

This is an **incomplete template** with TODOs for students to complete.

```
capstone-project/escrow/
├── programs/escrow/src/
│   ├── lib.rs              # Entry point (with TODOs)
│   ├── state.rs            # Escrow struct (with TODOs)
│   ├── errors.rs           # Custom errors (with TODOs)
│   ├── constants.rs        # Config values (with TODOs)
│   └── instructions/       # Each instruction (with TODOs)
│       ├── mod.rs
│       ├── initialize.rs   # Create escrow
│       ├── accept.rs       # Execute swap
│       └── cancel.rs       # Cancel and refund
└── README.md               # Assignment instructions
```

### What Students Must Implement

1. **Complete State** - Define the `Escrow` account structure
2. **Define Errors** - Create custom error types
3. **Implement Initialize** - Create escrow and lock funds
4. **Implement Accept** - Execute atomic swap (requires PDA signing!)
5. **Implement Cancel** - Return funds to maker
6. **Write Tests** - Verify all functionality works

### Core Challenges

Students will apply everything from Day 1 & 2:
- ✅ PDA creation with unique seeds
- ✅ PDA signing for transfers
- ✅ Account validation and constraints
- ✅ Atomic transactions (both transfers succeed or both fail)
- ✅ Security considerations

### Program Flow Diagram

```
┌─────────────────────────────────────────────────────────────┐
│              CAPSTONE: ESCROW (Atomic Swap)                  │
└─────────────────────────────────────────────────────────────┘

SCENARIO: Toby offers 10 SOL for 5 SOL from Alejandro

══════════════════════════════════════════════════════════════
HAPPY PATH: Atomic Swap
══════════════════════════════════════════════════════════════

1. Toby: Initialize Escrow
   ┌──────┐    10 SOL
   │ Toby │ ─────────────────> [Escrow PDA]
   └──────┘                    maker: Toby
                               taker: Alejandro (optional)
                               maker_amount: 10 SOL
                               taker_amount: 5 SOL
                               ↓
2. Alejandro: Accept Escrow (ATOMIC!)

   ┌───────────┐   5 SOL       ┌──────┐
   │ Alejandro │ ────────────> │ Toby │  (Alejandro pays Toby)
   └───────────┘                └──────┘
        ↓
   [Escrow PDA]  10 SOL
        │────────────────────> ┌───────────┐
                               │ Alejandro │  (Escrow pays Alejandro)
                               └───────────┘
        ↓
   [Escrow Closed ✅]

   RESULT: Toby gets 5 SOL, Alejandro gets 10 SOL
   Both transfers succeed or both fail!

══════════════════════════════════════════════════════════════
ALTERNATIVE PATH: Cancellation
══════════════════════════════════════════════════════════════

1. Toby: Initialize Escrow
   ┌──────┐    10 SOL
   │ Toby │ ─────────────────> [Escrow PDA]
   └──────┘                    (Funds locked)
                               ↓
2. Toby: Cancel Escrow
   ┌──────┐  <───── 10 SOL ── [Escrow PDA]
   │ Toby │                    (Funds returned)
   └──────┘                    ↓
                          [Escrow Closed ✅]

Concepts: Atomic Swaps, PDA Signing, Account Closure, Security
```

### Learning Outcomes

By completing the capstone, students demonstrate mastery of:
- ✅ Modular program architecture
- ✅ Complex PDA patterns
- ✅ Atomic swap logic
- ✅ Security best practices
- ✅ Independent problem-solving

---

## 🎯 Learning Path

### Prerequisites

- Basic Rust knowledge
- Understanding of blockchain concepts
- Solana CLI installed
- Anchor Framework installed
- Node.js and Yarn

### Recommended Order

1. **Start with Day 1** (2-3 hours)
   - Study the complete vault program
   - Understand PDAs and CPI
   - Deploy to localnet

2. **Move to Day 2** (4-6 hours)
   - Study the modular structure
   - Understand time-based logic
   - Run the test suite
   - Deploy to devnet

3. **Tackle the Capstone** (10-15 hours)
   - Complete all TODOs
   - Write comprehensive tests
   - Deploy and verify functionality
   - Present your solution

---

## 📁 Repository Structure

```
campus-tour-assignments/
│
├── day-1/
│   └── vault/
│       ├── programs/vault/src/lib.rs
│       ├── CONCEPTS.md          # ⭐ Start here
│       └── README.md
│
├── day-2/
│   └── vault-staking/
│       ├── programs/vault-staking/src/
│       │   ├── lib.rs
│       │   ├── state.rs
│       │   ├── errors.rs
│       │   ├── constants.rs
│       │   └── instructions/
│       ├── tests/vault-staking.ts
│       ├── CONCEPTS.md          # ⭐ Study this
│       └── README.md
│
└── capstone-project/
    └── escrow/
        ├── programs/escrow/src/
        │   ├── lib.rs           # TODO: Complete this
        │   ├── state.rs         # TODO: Complete this
        │   ├── errors.rs        # TODO: Complete this
        │   ├── constants.rs     # TODO: Complete this
        │   └── instructions/    # TODO: Complete these
        └── README.md            # 📋 Assignment instructions
```

---

## 🚀 Getting Started

### 1. Clone and Explore

```bash
cd campus-tour-assignments

# Explore Day 1
cd day-1/vault
cat CONCEPTS.md  # Read this first!

# Explore Day 2
cd ../../day-2/vault-staking
cat CONCEPTS.md  # Study the new concepts

# View Capstone Assignment
cd ../../capstone-project/escrow
cat README.md    # Read the assignment
```

### 2. Build and Test

```bash
# Day 1
cd day-1/vault
anchor build
anchor test

# Day 2
cd ../../day-2/vault-staking
anchor build
anchor test

# Capstone (will fail until you complete it!)
cd ../../capstone-project/escrow
anchor build  # This will fail - that's expected!
```

---

## 📚 Resources

### Essential Documentation

- **Anchor Framework:** https://www.anchor-lang.com/docs
- **Solana Docs:** https://docs.solana.com/
- **Rust Book:** https://doc.rust-lang.org/book/

### Key Concepts by Day

**Day 1:**
- PDAs: https://www.anchor-lang.com/docs/pdas
- CPIs: https://www.anchor-lang.com/docs/cross-program-invocations
- Accounts: https://www.anchor-lang.com/docs/the-accounts-struct

**Day 2:**
- Rust Modules: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
- Checked Math: https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add
- Clock Sysvar: https://docs.solana.com/developing/runtime-facilities/sysvars#clock

**Capstone:**
- Review Day 1 & Day 2 code
- Anchor documentation for reference

---

## 💡 Tips for Success

### For Day 1 & Day 2 (Learning)

1. **Read CONCEPTS.md first** - Don't skip this!
2. **Follow the inline comments** - They explain everything
3. **Run the code** - See it work before studying it
4. **Experiment** - Modify values and see what happens
5. **Ask questions** - Don't struggle alone

### For Capstone (Building)

1. **Read all TODO comments carefully** - They guide you
2. **Complete files in order** - state → errors → constants → instructions
3. **Test as you go** - Don't wait until everything is done
4. **Review Day 1 & 2 code** - The patterns are there
5. **Use the Anchor docs** - Look up constraints and patterns
6. **Debug systematically** - Read error messages carefully

---

## 🎓 Learning Objectives

### Day 1: Foundation
- Understand Solana account model
- Create and use PDAs
- Make cross-program invocations
- Validate accounts with constraints

### Day 2: Intermediate
- Structure code professionally
- Implement business logic (staking)
- Work with time and state
- Write comprehensive tests

### Capstone: Mastery
- Design a complete program
- Implement atomic swaps
- Handle edge cases and errors
- Demonstrate independent problem-solving

---

## ✅ Progress Checklist

- [ ] Day 1: Read CONCEPTS.md
- [ ] Day 1: Build and deploy vault
- [ ] Day 1: Understand all three instructions
- [ ] Day 2: Read CONCEPTS.md
- [ ] Day 2: Build and test staking vault
- [ ] Day 2: Deploy to devnet
- [ ] Day 2: Understand modular structure
- [ ] Capstone: Complete state.rs
- [ ] Capstone: Complete errors.rs
- [ ] Capstone: Complete all instructions
- [ ] Capstone: Write tests
- [ ] Capstone: Deploy and verify
- [ ] Capstone: Present your solution

---

## 🆘 Getting Help

1. **Check the inline comments** - Extensive explanations in the code
2. **Read CONCEPTS.md** - Detailed concept breakdowns
3. **Review the README** - Setup and troubleshooting guides
4. **Search Anchor docs** - Official documentation
5. **Ask your instructor** - Don't struggle for too long!

---

## 📝 Notes for Instructors

### Teaching Approach

- **Day 1:** Lecture + code walkthrough (2-3 hours)
- **Day 2:** Workshop + live coding session (3-4 hours)
- **Capstone:** Independent work with office hours (1-2 weeks)

### Assessment

Students should be evaluated on:
- Code completeness (all TODOs addressed)
- Code quality (proper structure, error handling)
- Testing (comprehensive test coverage)
- Understanding (ability to explain their code)
- Presentation (demo and explanation)

### Common Issues

See the README.md in each folder for troubleshooting guides.

---

**Good luck on your Solana learning journey!** 🚀

Built with ❤️ by Kronos Guild. ⏳
