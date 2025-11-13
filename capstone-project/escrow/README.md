# Escrow Program - Assignment Template

A simple SOL escrow program built with Anchor. This is an **assignment template** with TODOs for students to complete.

## What is an Escrow?

An escrow is a smart contract that holds funds from two parties and swaps them atomically. Both parties get what they want, or nobody does.

**Example:**
- Toby offers 10 SOL for 5 SOL
- Alejandro accepts with 5 SOL
- Atomic swap: Toby gets 5 SOL, Alejandro gets 10 SOL

## Assignment Overview

Complete the TODOs throughout the codebase to build a working escrow program with these features:

1. **Initialize Escrow** - Create an escrow with your offer
2. **Accept Escrow** - Accept someone's offer and complete the swap
3. **Cancel Escrow** - Cancel your escrow and get funds back

## Project Structure

```
programs/escrow/src/
├── lib.rs                    # Entry point (TODOs)
├── state.rs                  # Escrow account (TODOs)
├── errors.rs                 # Custom errors (TODOs)
├── constants.rs              # Config values (TODOs)
└── instructions/
    ├── mod.rs                # Exports (TODOs)
    ├── initialize.rs         # Create escrow (TODOs)
    ├── accept.rs             # Accept offer (TODOs)
    └── cancel.rs             # Cancel escrow (TODOs)
```

## Getting Started

### 1. Complete the State

**File:** `programs/escrow/src/state.rs`

Define the `Escrow` account structure with all necessary fields.

### 2. Define Errors

**File:** `programs/escrow/src/errors.rs`

Create custom error types for different failure scenarios.

### 3. Add Constants

**File:** `programs/escrow/src/constants.rs`

Define seed prefixes and other configuration values.

### 4. Implement Instructions

Complete each instruction file in `programs/escrow/src/instructions/`:
- `initialize.rs` - Create and fund escrow
- `accept.rs` - Execute the atomic swap
- `cancel.rs` - Return funds to maker

### 5. Connect Everything

**File:** `programs/escrow/src/lib.rs`

Uncomment the module imports and instruction handlers.

**File:** `programs/escrow/src/instructions/mod.rs`

Export all instruction modules.

## Building & Testing

```bash
# Build
anchor build

# Get program ID
anchor keys list

# Update program ID in lib.rs and Anchor.toml
# Then rebuild
anchor build

# Test
anchor test
```

## Key Concepts

### PDAs (Program Derived Addresses)
The escrow account is a PDA, allowing the program to sign for it.

### PDA Signing
When transferring from the escrow PDA, use `CpiContext::new_with_signer` with the seeds.

### Atomic Swaps
Both transfers (taker → maker and escrow → taker) happen in one transaction. Both succeed or both fail.

### Account Closure
Use the `close` constraint to close the escrow account and return rent.

## Hints

Check the extensive comments in each file for:
- What each TODO requires
- Code examples and patterns
- Security considerations
- References to documentation

## Resources

- **Day 1 Code:** Review vault program for PDA and CPI patterns
- **Day 2 Code:** Review modular structure and organization
- **Anchor Docs:** https://www.anchor-lang.com/docs

## Expected Flow

### Happy Path
```
1. Maker: initialize_escrow(10 SOL, 5 SOL)
   → Escrow PDA created
   → 10 SOL locked in escrow

2. Taker: accept_escrow()
   → Taker sends 5 SOL to maker
   → Escrow sends 10 SOL to taker
   → Escrow closed
```

### Cancel Path
```
1. Maker: initialize_escrow(10 SOL, 5 SOL)
   → Escrow PDA created
   → 10 SOL locked in escrow

2. Maker: cancel_escrow()
   → 10 SOL returned to maker
   → Escrow closed
```

## Common Issues

### Space Calculation
Remember to account for:
- 8 bytes: discriminator
- 32 bytes: Pubkey
- 8 bytes: u64
- 1 byte: u8
- 8 bytes: i64
- 33 bytes: Option<Pubkey>

### PDA Seeds
Each escrow needs unique seeds. Use maker's pubkey + unique identifier.

### Signer Seeds
When the program signs for a PDA, provide the same seeds + bump used to derive it.

## What to Test

- ✅ Initialize with valid amounts
- ✅ Accept escrow successfully
- ✅ Cancel escrow successfully
- ❌ Initialize with 0 amount (should fail)
- ❌ Wrong taker accepts (should fail)
- ❌ Non-maker cancels (should fail)

## Bonus Challenges

- [ ] Add expiration timestamp
- [ ] Add EscrowStatus enum
- [ ] Support specific or any taker
- [ ] Add protocol fees
- [ ] Support SPL tokens

---

**This is a learning assignment. Take your time, read the comments, and ask questions!**
