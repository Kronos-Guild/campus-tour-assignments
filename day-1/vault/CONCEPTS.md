# Vault Program - Concept Explanations

This document explains all the Solana and Anchor concepts used in the Vault program. Use this as a reference for your presentation.

---

## Table of Contents
1. [What is Anchor?](#what-is-anchor)
2. [Program Structure](#program-structure)
3. [Key Concepts](#key-concepts)
4. [How the Vault Works](#how-the-vault-works)

---

## What is Anchor?

**Anchor** is a framework for building Solana programs (smart contracts) that makes development easier and more secure.

**Reference:** [https://www.anchor-lang.com/](https://www.anchor-lang.com/)

### Why Use Anchor?
- **Reduces Boilerplate:** Automatically handles account validation and deserialization
- **Type Safety:** Generates TypeScript types from your Rust code
- **Better Security:** Built-in checks prevent common vulnerabilities
- **Easier Testing:** Provides testing utilities for JavaScript/TypeScript

---

## Program Structure

### 1. Program ID (`declare_id!`)

```rust
declare_id!("CGEB5qg98afTaM5g4ax8MbTnRh6xyCSXn738JRksKdJA");
```

**What it is:** The unique address of your program on the Solana blockchain.

**Reference:** [Anchor Programs Documentation](https://www.anchor-lang.com/docs/programs)

**Why it matters:**
- Every Solana program has a unique ID (public key)
- Used to verify the correct program is being called
- Generated automatically when you create a new Anchor project

---

### 2. The `#[program]` Macro

```rust
#[program]
pub mod vault {
    // Instructions go here
}
```

**What it is:** Marks the module containing your program's instructions.

**Reference:** [Anchor Programs Documentation](https://www.anchor-lang.com/docs/programs)

**How it works:**
- Each public function becomes an instruction that users can call
- Anchor automatically generates the instruction handlers
- In our vault: `initialize`, `deposit`, and `withdraw` are instructions

---

### 3. Context<T>

```rust
pub fn initialize(ctx: Context<Initialize>) -> Result<()>
```

**What it is:** A wrapper that provides access to accounts and program metadata.

**Reference:** [The Accounts Struct](https://www.anchor-lang.com/docs/the-accounts-struct)

**What's inside Context:**
- `ctx.accounts` - Access to the accounts passed to the instruction
- `ctx.bumps` - Bump seeds for PDAs (more on this below)
- `ctx.program_id` - The program's ID
- `ctx.remaining_accounts` - Additional accounts not specified in the struct

---

## Key Concepts

### 1. PDAs (Program Derived Addresses)

**What is a PDA?**
A PDA is an account whose address is derived from:
1. A program ID
2. Seeds (arbitrary bytes, like "vault" + user's public key)
3. A bump seed (a number 0-255)

**Reference:** [Anchor PDAs Documentation](https://www.anchor-lang.com/docs/pdas)

**Why PDAs are Important:**
- **Deterministic:** Same seeds always generate the same address
- **No Private Key:** PDAs don't have a private key - only the program can sign for them
- **Program Authority:** Perfect for accounts controlled by the program (like our vault)

**In Our Vault:**
```rust
seeds = [b"vault", user.key().as_ref()]
```
- Each user gets their own vault
- Address is derived from: program ID + "vault" + user's public key
- Only the program can move funds from the vault

**Visual Explanation:**
```
User's Public Key: ABC123...
Seeds: ["vault", ABC123...]
Program ID: CGEB5q...
           ↓
    findProgramAddress()
           ↓
Vault PDA: XYZ789... (deterministic, no private key)
```

---

### 2. Account Constraints

Anchor provides powerful constraints to validate accounts automatically.

**Reference:** [Account Constraints Documentation](https://www.anchor-lang.com/docs/the-accounts-struct#constraints)

#### `#[account(init)]`
```rust
#[account(
    init,
    payer = user,
    space = 8 + Vault::INIT_SPACE,
    seeds = [b"vault", user.key().as_ref()],
    bump
)]
```

**What it does:**
- `init` - Creates a new account
- `payer` - Who pays for the account creation (rent)
- `space` - How many bytes to allocate
  - 8 bytes for Anchor's discriminator
  - `Vault::INIT_SPACE` for the account data (33 bytes)
- `seeds` + `bump` - Derives the PDA address

---

#### `#[account(mut)]`
```rust
#[account(mut)]
pub user: Signer<'info>
```

**What it does:**
- `mut` - Marks the account as mutable (can be modified)
- Required when the account's lamports or data will change

---

#### `has_one` Constraint
```rust
#[account(
    mut,
    seeds = [b"vault", vault.owner.as_ref()],
    bump = vault.bump,
    has_one = owner  // ← This constraint
)]
pub vault: Account<'info, Vault>
```

**What it does:**
- Verifies that `vault.owner` equals the `owner` account's public key
- Prevents users from withdrawing from vaults they don't own
- **Security:** Ensures only the vault owner can withdraw

---

### 3. Account Types

#### `Signer<'info>`
```rust
pub user: Signer<'info>
```
**What it is:** Ensures this account signed the transaction.

**Why it matters:** Proves the user authorized this action.

---

#### `Account<'info, T>`
```rust
pub vault: Account<'info, Vault>
```
**What it is:** A validated, deserialized account of type `T`.

**What Anchor does:**
- Checks the account exists
- Verifies the owner is the program
- Deserializes the data into the `Vault` struct
- Validates all constraints (seeds, bump, has_one, etc.)

---

#### `Program<'info, T>`
```rust
pub system_program: Program<'info, System>
```
**What it is:** Validates that this account is a specific program.

**In our case:** Ensures we're calling the real System Program (not a fake one).

---

#### `UncheckedAccount<'info>`
```rust
/// CHECK: This is the owner of the vault, verified through has_one constraint
pub owner: UncheckedAccount<'info>
```
**What it is:** An account that Anchor doesn't automatically validate.

**When to use:** When you're doing custom validation (like with `has_one`).

**Important:** Always add a `/// CHECK:` comment explaining why it's safe.

---

### 4. CPI (Cross-Program Invocation)

**What is CPI?**
CPI is when your program calls another program.

**Reference:** [Cross-Program Invocations](https://www.anchor-lang.com/docs/cross-program-invocations)

**In our vault:** We call the System Program to transfer SOL.

#### Regular CPI (for Deposit)
```rust
let cpi_context = CpiContext::new(
    ctx.accounts.system_program.to_account_info(),
    Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    },
);

transfer(cpi_context, amount)?;
```

**What happens:**
1. User signs the transaction
2. Our program calls System Program's `transfer` instruction
3. System Program transfers SOL from user to vault

---

#### CPI with Signer Seeds (for Withdraw)
```rust
let seeds = &[
    b"vault",
    vault.owner.as_ref(),
    &[vault.bump],
];
let signer_seeds = &[&seeds[..]];

let cpi_context = CpiContext::new_with_signer(
    ctx.accounts.system_program.to_account_info(),
    Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user.to_account_info(),
    },
    signer_seeds,  // ← Program signs as the vault PDA
);

transfer(cpi_context, amount)?;
```

**What happens:**
1. User signs the transaction
2. Our program provides the PDA seeds to prove it controls the vault
3. System Program transfers SOL from vault to user
4. **The program signs on behalf of the vault PDA**

**Why this works:**
- PDAs don't have private keys
- Only the program that derived the PDA can sign for it
- Must provide the same seeds and bump used to create the PDA

---

### 5. The `#[account]` Macro

```rust
#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub owner: Pubkey,  // 32 bytes
    pub bump: u8,       // 1 byte
}
```

**Reference:** [Account Types](https://www.anchor-lang.com/docs/the-accounts-struct)

**What it does:**
- Marks this struct as an Anchor account
- Adds an 8-byte discriminator (identifies the account type)
- Enables serialization/deserialization

**`InitSpace` derive:**
- Automatically calculates space needed
- `Pubkey` = 32 bytes
- `u8` = 1 byte
- Total: 33 bytes + 8-byte discriminator = 41 bytes

---

### 6. Custom Errors

```rust
#[error_code]
pub enum VaultError {
    #[msg("Amount must be greater than 0")]
    InvalidAmount,
    #[msg("Insufficient funds in vault")]
    InsufficientFunds,
}
```

**Reference:** [Custom Errors](https://www.anchor-lang.com/docs/errors)

**Why use custom errors:**
- Better debugging
- Clear error messages for users
- Each error gets a unique code

**Usage:**
```rust
require!(amount > 0, VaultError::InvalidAmount);
```

---

## How the Vault Works

### Flow Diagram

```
┌─────────────┐
│   User      │
└──────┬──────┘
       │
       │ 1. Initialize
       ▼
┌─────────────────────────────────┐
│  Vault Program                  │
│  - Creates PDA vault            │
│  - Stores owner & bump          │
└─────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────┐
│  Vault PDA Account              │
│  Owner: User's pubkey           │
│  Bump: 254 (example)            │
│  Balance: 0 SOL                 │
└─────────────────────────────────┘

       │
       │ 2. Deposit 0.5 SOL
       ▼
┌─────────────────────────────────┐
│  System Program (CPI)           │
│  Transfer: User → Vault         │
└─────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────┐
│  Vault PDA Account              │
│  Balance: 0.5 SOL               │
└─────────────────────────────────┘

       │
       │ 3. Withdraw 0.2 SOL
       ▼
┌─────────────────────────────────┐
│  Vault Program                  │
│  - Verifies owner               │
│  - Signs with PDA seeds         │
└─────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────┐
│  System Program (CPI)           │
│  Transfer: Vault → User         │
│  Signed by: Program (PDA seeds) │
└─────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────┐
│  Vault PDA Account              │
│  Balance: 0.3 SOL               │
└─────────────────────────────────┘
```

---

### 1. Initialize Instruction

**Purpose:** Creates a new vault for a user.

**What happens:**
1. Anchor derives the vault PDA using: `["vault", user_pubkey]`
2. Creates a new account at that PDA address
3. Stores the owner's public key
4. Stores the bump seed (for later signing)

**Code:**
```rust
pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.owner = ctx.accounts.user.key();
    vault.bump = ctx.bumps.vault;
    Ok(())
}
```

---

### 2. Deposit Instruction

**Purpose:** Transfer SOL from user to vault.

**What happens:**
1. Validates amount > 0
2. Calls System Program via CPI
3. Transfers SOL from user to vault PDA
4. User signs the transaction

**Security:**
- User must sign (they're authorizing the transfer)
- Amount validation prevents 0-value transfers

**Code:**
```rust
pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(amount > 0, VaultError::InvalidAmount);

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}
```

---

### 3. Withdraw Instruction

**Purpose:** Transfer SOL from vault back to user.

**What happens:**
1. Validates amount > 0
2. Checks vault has sufficient balance
3. Verifies user is the vault owner (`has_one = owner`)
4. Program signs for the vault PDA using seeds
5. Calls System Program via CPI
6. Transfers SOL from vault to user

**Security:**
- `has_one` ensures only owner can withdraw
- Balance check prevents overdrafts
- Program signs for PDA (user can't fake this)

**Code:**
```rust
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(amount > 0, VaultError::InvalidAmount);

    let vault_lamports = ctx.accounts.vault.to_account_info().lamports();
    require!(vault_lamports >= amount, VaultError::InsufficientFunds);

    let seeds = &[
        b"vault",
        ctx.accounts.vault.owner.as_ref(),
        &[ctx.accounts.vault.bump],
    ];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.user.to_account_info(),
            },
            &[&seeds[..]],
        ),
        amount,
    )?;

    Ok(())
}
```

---

## Key Takeaways for Your Presentation

### 1. **Anchor Makes Development Easier**
- Automatic account validation
- Clear error messages
- Type-safe client generation

### 2. **PDAs are Powerful**
- Deterministic addresses
- Program-controlled (no private key needed)
- Perfect for escrow/vault scenarios

### 3. **Security Built-In**
- `Signer` ensures authorization
- `has_one` prevents unauthorized access
- `require!` validates inputs
- Seeds verification prevents PDA spoofing

### 4. **CPI Enables Composability**
- Programs can call other programs
- System Program handles SOL transfers
- Signer seeds allow programs to sign for PDAs

### 5. **Our Vault is Secure Because:**
- Each user gets their own PDA vault
- Only the owner can withdraw (`has_one`)
- Program signs for the vault (can't be faked)
- All inputs are validated

---

## Additional Resources

- **Anchor Documentation:** [https://www.anchor-lang.com/docs](https://www.anchor-lang.com/docs)

---

## Questions for Beginners to Consider

1. **Why do we need PDAs?**
   - What's wrong with having the vault be a regular account?

2. **How does the program sign for the vault?**
   - Why can't a user fake this?

3. **What prevents someone from withdrawing from another user's vault?**
   - Follow the `has_one` constraint

4. **Why store the bump seed in the account?**
   - Hint: Look at the withdraw function

5. **What would happen if we didn't check the amount > 0?**
   - Would anything break?

---

**Good luck with your presentation!** 🚀
