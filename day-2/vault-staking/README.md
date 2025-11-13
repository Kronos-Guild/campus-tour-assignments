# Vault Staking Program - Day 2

A modular Solana staking program built with Anchor. Extends the Day 1 vault with time-based staking and rewards.

## Overview

**Day 1:** Simple vault (deposit/withdraw)
**Day 2:** + Staking with time-based rewards + Modular code structure

### Features

- ✅ **Initialize** - Create your personal vault (PDA)
- ✅ **Deposit** - Add SOL to your vault
- ✅ **Withdraw** - Remove unstaked SOL from vault
- ✅ **Stake** - Lock funds to earn rewards
- ✅ **Unstake** - Unlock funds with rewards (if eligible)

## Staking Rules

**Simple & Clear:**
```
Stake for 60+ seconds = Get 10% rewards 🎉
Stake for <60 seconds = Get 0% rewards (no penalty, just no bonus)
```

**Example:**
- Deposit 1000 SOL
- Stake 1000 SOL
- Wait 60 seconds
- Unstake → Receive 1100 SOL (1000 + 100 reward)
- Withdraw 1100 SOL to wallet

## Project Structure

```
programs/vault-staking/src/
├── lib.rs                     # Entry point (~70 lines)
├── state.rs                   # Vault account struct
├── errors.rs                  # Custom error types
├── constants.rs               # Reward config (60s, 10%)
└── instructions/              # Instruction logic
    ├── mod.rs                 # Module exports
    ├── initialize.rs          # Create vault
    ├── deposit.rs             # Add SOL
    ├── withdraw.rs            # Remove SOL
    ├── stake.rs               # Lock for rewards
    └── unstake.rs             # Unlock + distribute rewards
```

### Why Modular?

**Day 1 (Monolithic):** Everything in one file → Hard to navigate
**Day 2 (Modular):** Organized files → Easy to find and maintain

See [CONCEPTS.md](./CONCEPTS.md) for detailed explanation.

## Building & Deploying

### Prerequisites
- Rust + Solana CLI
- Anchor Framework (v0.28+)
- Node.js + Yarn

### Step 1: Build the Program

```bash
cd vault-staking
anchor build
```

This compiles your Rust program to BPF (Berkeley Packet Filter) bytecode that can run on Solana.

### Step 2: Get the Program ID

```bash
anchor keys list
```

Output:
```
vault_staking: mkZJVMPpBYQhUjnhXKWMbuaVsEpGtGdTFa3BbRE1VU1
```

### Step 3: Update Program ID in Code

**Update 2 places:**

1. **programs/vault-staking/src/lib.rs:**
```rust
declare_id!("mkZJVMPpBYQhUjnhXKWMbuaVsEpGtGdTFa3BbRE1VU1");
```

2. **Anchor.toml:**
```toml
[programs.localnet]
vault_staking = "mkZJVMPpBYQhUjnhXKWMbuaVsEpGtGdTFa3BbRE1VU1"

[programs.devnet]
vault_staking = "mkZJVMPpBYQhUjnhXKWMbuaVsEpGtGdTFa3BbRE1VU1"
```

### Step 4: Rebuild

```bash
anchor build
```

**Important:** Always rebuild after updating the program ID!

---

## Deploy to Localnet

### Terminal 1: Start Local Validator
```bash
solana-test-validator
```

Keep this running in a separate terminal.

### Terminal 2: Configure & Deploy

```bash
# Set cluster to localhost
solana config set --url localhost

# Check your wallet balance
solana balance

# If balance is 0, airdrop some SOL
solana airdrop 2

# Deploy the program
anchor deploy

# Or use anchor test (builds, deploys, and runs tests)
anchor test --skip-local-validator
```

**Success Output:**
```
Deploying cluster: http://localhost:8899
Program Id: mkZJVMPpBYQhUjnhXKWMbuaVsEpGtGdTFa3BbRE1VU1
Deploy success
```

---

## Deploy to Devnet

### Step 1: Switch to Devnet

```bash
# Set cluster to devnet
solana config set --url devnet

# Verify configuration
solana config get
```

Output should show:
```
RPC URL: https://api.devnet.solana.com
```

### Step 2: Check Your Wallet

```bash
# Check which wallet you're using
solana address

# Check balance
solana balance
```

### Step 3: Airdrop Devnet SOL

Devnet allows free airdrops for testing:

```bash
# Request 2 SOL (can run multiple times if needed)
solana airdrop 2

# Check balance again
solana balance
```

**Note:** Each airdrop gives 2 SOL. You might need 3-5 SOL for deployment + testing.

**If airdrop fails:**
- Try again in a few minutes (rate limited)
- Use the [Solana Devnet Faucet](https://faucet.solana.com/)

### Step 4: Update Anchor.toml for Devnet

```toml
[provider]
cluster = "devnet"  # Change this to devnet
wallet = "~/.config/solana/id.json"

[programs.devnet]
vault_staking = "mkZJVMPpBYQhUjnhXKWMbuaVsEpGtGdTFa3BbRE1VU1"
```

### Step 5: Deploy to Devnet

```bash
# Deploy
anchor deploy --provider.cluster devnet

# Or shorter (if Anchor.toml cluster is set)
anchor deploy
```

**Success Output:**
```
Deploying cluster: https://api.devnet.solana.com
Program Id: mkZJVMPpBYQhUjnhXKWMbuaVsEpGtGdTFa3BbRE1VU1

Deploy success
```

### Step 6: Verify Deployment

```bash
# Check program account
solana program show mkZJVMPpBYQhUjnhXKWMbuaVsEpGtGdTFa3BbRE1VU1

# View on Solana Explorer
https://explorer.solana.com/address/mkZJVMPpBYQhUjnhXKWMbuaVsEpGtGdTFa3BbRE1VU1?cluster=devnet
```

### Step 7: Run Tests on Devnet

```bash
anchor test --provider.cluster devnet --skip-build --skip-deploy
```

---

## Quick Command Reference

### Localnet
```bash
# Start validator
solana-test-validator

# In another terminal
solana config set --url localhost
solana airdrop 2
anchor build
anchor deploy
anchor test --skip-local-validator
```

### Devnet
```bash
# Switch to devnet
solana config set --url devnet

# Get SOL
solana airdrop 2

# Build and deploy
anchor build
anchor deploy --provider.cluster devnet

# Run tests
anchor test --provider.cluster devnet --skip-build --skip-deploy
```

### Mainnet (WARNING: Real Money!)
```bash
# Switch to mainnet
solana config set --url mainnet-beta

# Deploy (costs real SOL!)
anchor deploy --provider.cluster mainnet-beta
```

**⚠️ Only deploy to mainnet after thorough testing and auditing!**

---

## Instructions Usage

### 1. Initialize
Creates a vault PDA for your wallet.

```typescript
await program.methods
  .initialize()
  .accounts({ vault, user, systemProgram })
  .rpc();
```

### 2. Deposit
Add SOL to your vault (unstaked).

```typescript
await program.methods
  .deposit(new BN(1000000000)) // 1 SOL in lamports
  .accounts({ vault, user, systemProgram })
  .rpc();
```

### 3. Stake
Lock funds to start earning rewards.

```typescript
await program.methods
  .stake(new BN(1000000000)) // Amount to stake
  .accounts({ vault, user, owner })
  .rpc();
```

### 4. Unstake
Unlock funds and receive rewards if 60+ seconds passed.

```typescript
await program.methods
  .unstake()
  .accounts({ vault, user, owner })
  .rpc();
```

### 5. Withdraw
Move unstaked SOL from vault to wallet.

```typescript
await program.methods
  .withdraw(new BN(1000000000))
  .accounts({ vault, user, owner, systemProgram })
  .rpc();
```

---

## Account Structure

### Vault (66 bytes)

```rust
pub struct Vault {
    pub owner: Pubkey,           // 32 bytes - vault owner
    pub bump: u8,                // 1 byte  - PDA bump
    pub total_deposited: u64,    // 8 bytes - total balance
    pub staked_amount: u64,      // 8 bytes - currently staked
    pub stake_timestamp: i64,    // 8 bytes - when staking started
    pub is_staked: bool,         // 1 byte  - staking active?
}
// + 8 bytes discriminator = 66 bytes total
```

---

## Key Concepts

### 1. Modular Architecture
Code organized into logical modules instead of one big file.

**Benefits:**
- Easy to find specific features
- Better for teams
- Easier to test
- Professional structure

### 2. Time-Based Logic
Uses Solana's Clock sysvar to track staking duration.

```rust
let clock = Clock::get()?;
let duration = clock.unix_timestamp - vault.stake_timestamp;

if duration >= 60 {
    // Eligible for rewards!
}
```

### 3. Fixed-Point Math (Basis Points)
No floating point in Solana! Use integers to represent percentages.

```rust
// 10% = 1000 basis points
const REWARD_RATE: u64 = 1000;
const DIVISOR: u64 = 10000;

let reward = (amount * REWARD_RATE) / DIVISOR;
// Example: (1000 * 1000) / 10000 = 100 (10% of 1000)
```

### 4. Checked Math
Prevent overflow vulnerabilities.

```rust
// ❌ Bad: Can overflow
let sum = a + b;

// ✅ Good: Safe
let sum = a.checked_add(b).ok_or(Error::Overflow)?;
```

---

## Configuration

Edit `constants.rs` to change staking parameters:

```rust
// Minimum stake duration (seconds)
pub const MIN_STAKE_DURATION: i64 = 60;

// Reward rate (basis points)
pub const REWARD_RATE_BASIS_POINTS: u64 = 1000; // 10%
```

Want 5% rewards? Change to `500`
Want 1 hour minimum? Change to `3600`

---

## Error Codes

| Error | Code | Description |
|-------|------|-------------|
| InvalidAmount | 6000 | Amount must be > 0 |
| InsufficientFunds | 6001 | Not enough balance |
| AlreadyStaked | 6002 | Funds already staked |
| NotStaked | 6003 | No active stake |
| InsufficientUnstakedBalance | 6004 | Not enough unstaked funds |
| MinimumStakeDurationNotMet | 6005 | Staked for < 60 seconds |
| ArithmeticOverflow | 6006 | Math operation overflowed |
| FundsStaked | 6007 | Can't withdraw staked funds |

---

## Troubleshooting

### "Program account does not exist"
- Make sure you deployed: `anchor deploy`
- Check you're on the right cluster: `solana config get`

### "Insufficient funds"
- Airdrop more SOL: `solana airdrop 2`
- Check balance: `solana balance`

### "Already staked" error
- You can only have one active stake
- Unstake first, then stake again

### No rewards received
- Check that you staked for at least 60 seconds
- The timestamp check is strict

### Can't withdraw
- You can only withdraw unstaked funds
- Unstake first to move funds from "staked" to "unstaked"

### Program ID mismatch
- Update program ID in both `lib.rs` and `Anchor.toml`
- Rebuild: `anchor build`

### Devnet airdrop fails
- Wait a few minutes (rate limited)
- Try multiple times
- Use web faucet: https://faucet.solana.com/

---

## Testing

See the comprehensive test suite in `tests/vault-staking.ts`.

Run tests:
```bash
# Localnet
anchor test

# Devnet
anchor test --provider.cluster devnet --skip-build --skip-deploy
```

---

## Learning Resources

### Comprehensive Guide
See **[CONCEPTS.md](./CONCEPTS.md)** for in-depth explanations of:
- Modular code structure
- Time-based logic
- Reward calculations
- Fixed-point math
- Security considerations
- Complete user flows

### External Resources
- **Anchor Docs:** https://www.anchor-lang.com/docs
- **Clock Sysvar:** https://docs.solana.com/developing/runtime-facilities/sysvars#clock
- **Rust Modules:** https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
- **Devnet Faucet:** https://faucet.solana.com/

---

## Comparison: Day 1 vs Day 2

| | Day 1 | Day 2 |
|-|-------|-------|
| **Files** | 1 (lib.rs) | 7 (modular) |
| **Lines** | ~200 | ~500 |
| **Instructions** | 3 | 5 |
| **Concepts** | PDAs, CPI | + Time, Rewards, Modules |
| **Account Fields** | 2 | 6 |
| **Structure** | Monolithic | Modular |

---

## Security Notes

✅ **Implemented:**
- Checked math (no overflow)
- PDA ownership validation
- Time-based eligibility
- State consistency checks

⚠️ **Not Implemented (Educational Only):**
- Reward pool (rewards are "printed")
- Admin controls
- Pause/emergency functions
- Rate limiting

**This is a learning project, not production-ready!**

For production, you'd need:
- Funded reward pool
- Admin multisig
- Comprehensive tests
- Security audit

---

## Next Steps

Want to extend this program? Try:
- [ ] Add multiple stake accounts per user
- [ ] Implement different lock periods (30d, 60d, 90d)
- [ ] Add a funded reward pool
- [ ] Implement compound staking
- [ ] Add admin controls
- [ ] Deploy to mainnet (after audit!)

---

**Built with Anchor Framework** 🏴‍☠️
**Day 2: Staking + Modular Structure** 🚀
