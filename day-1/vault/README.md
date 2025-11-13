# Solana Vault Program

A simple vault program built with Anchor that allows users to deposit and withdraw SOL.

## Overview

This program demonstrates core Solana and Anchor concepts:
- **PDAs (Program Derived Addresses)** - Each user gets their own vault
- **CPI (Cross-Program Invocations)** - Calling the System Program to transfer SOL
- **Account Validation** - Using Anchor constraints for security
- **PDA Signing** - Program signs for vaults to authorize withdrawals

## Features

- ✅ **Initialize:** Create a personal vault (PDA)
- ✅ **Deposit:** Transfer SOL from your wallet to your vault
- ✅ **Withdraw:** Transfer SOL from your vault back to your wallet

## Program Structure

```
vault/
├── programs/vault/src/lib.rs    # Main program code (heavily commented)
├── CONCEPTS.md                  # Detailed concept explanations
└── Anchor.toml                  # Anchor configuration
```

## Instructions

### 1. Initialize
Creates a new vault account for a user using a PDA.

**Accounts:**
- `vault` - The PDA vault account (created)
- `user` - The user creating the vault (signer)
- `system_program` - System program for account creation

### 2. Deposit
Transfers SOL from user to their vault.

**Accounts:**
- `vault` - The user's vault PDA (mut)
- `user` - The user depositing (signer, mut)
- `system_program` - System program for transfer

**Parameters:**
- `amount: u64` - Amount in lamports to deposit

### 3. Withdraw
Transfers SOL from vault back to user. Only the vault owner can withdraw.

**Accounts:**
- `vault` - The vault PDA (mut)
- `user` - The user withdrawing (signer, mut)
- `owner` - The vault owner (verified via `has_one`)
- `system_program` - System program for transfer

**Parameters:**
- `amount: u64` - Amount in lamports to withdraw

## How It Works

### PDA Vault
Each user's vault is a PDA derived from:
```
seeds = ["vault", user_pubkey]
```

This ensures:
- Deterministic addresses (same user = same vault address)
- No private key needed
- Only the program can sign for the vault

### Security Features

1. **Owner Verification:** `has_one = owner` constraint ensures only vault owner can withdraw
2. **Input Validation:** `require!` checks prevent invalid amounts
3. **Balance Checks:** Prevents withdrawing more than available
4. **PDA Signing:** Program signs for vault using seeds - can't be faked

## Building and Testing

### Build the program
```bash
cd vault
anchor build
```

### Get the program ID
```bash
anchor keys list
```

### Update the program ID
1. Copy the program ID from above
2. Update `declare_id!()` in `programs/vault/src/lib.rs`
3. Update `[programs.vault]` in `Anchor.toml`

### Deploy to localnet
```bash
# Start local validator (in separate terminal)
solana-test-validator

# Deploy
anchor deploy
```

### Run tests (when ready)
```bash
anchor test
```

## Code Documentation

The main program code (`programs/vault/src/lib.rs`) is heavily commented with:
- **CONCEPT** tags explaining each Anchor/Solana concept
- **Reference** links to official documentation
- Inline explanations of what each section does

For detailed explanations of all concepts, see **[CONCEPTS.md](./CONCEPTS.md)**.

## Key Concepts Explained

### PDAs (Program Derived Addresses)
- Addresses derived from seeds + program ID
- No private key - program controls them
- Perfect for program-controlled accounts like vaults

### CPI (Cross-Program Invocation)
- One program calling another program
- Used here to call System Program for transfers
- Requires signer seeds when PDA needs to sign

### Account Constraints
- `#[account(init)]` - Create new account
- `#[account(mut)]` - Account can be modified
- `has_one` - Verify account relationships
- `seeds` + `bump` - PDA derivation and validation

## Learning Resources

- **Full Concept Guide:** See [CONCEPTS.md](./CONCEPTS.md) for comprehensive explanations
- **Anchor Docs:** [https://www.anchor-lang.com/docs](https://www.anchor-lang.com/docs)

## Common Issues

### Program ID mismatch
After building, make sure to:
1. Get the program ID: `anchor keys list`
2. Update `declare_id!()` in the code
3. Update `Anchor.toml`
4. Rebuild: `anchor build`

### Insufficient balance
Make sure your wallet has enough SOL for:
- Transaction fees
- Rent for account creation
- Deposit amounts

### PDA derivation errors
The seeds must match exactly:
- In the program: `seeds = [b"vault", user.key().as_ref()]`
- In the client: `["vault", userPublicKey]`

## Questions?

Review the [CONCEPTS.md](./CONCEPTS.md) file for detailed explanations of:
- How PDAs work
- Why we need signer seeds
- Account validation and security
- And much more!

---

**Built with Anchor Framework** 🏴‍☠️
